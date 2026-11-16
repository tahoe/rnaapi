//! Derive `get`, `list`, and write methods for NetActuate endpoint structs.
//!
//! ```ignore
//! #[derive(EndpointGetOne, EndpointGetAll)]
//! #[getone(path = "cloud/server?mbpkgid={}", mbpkgid: u32)]
//! #[getall(path = "cloud/servers")]
//! struct Server { /* ... */ }
//! ```
//!
//! Each `name: Type` becomes a function parameter, in order, and must match
//! one `{}` placeholder in the path.
use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{DeriveInput, Ident, LitStr, Token, Type};

struct EndpointAttr {
    path: LitStr,
    args: Vec<(Ident, Type)>,
}

impl Parse for EndpointAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let key: Ident = input.parse()?;
        if key != "path" {
            return Err(syn::Error::new(
                key.span(),
                "expected `path = \"...\"` as the first argument",
            ));
        }
        input.parse::<Token![=]>()?;
        let path: LitStr = input.parse()?;
        let mut args = Vec::new();
        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
            let name: Ident = input.parse()?;
            if name == "args" && input.peek(Token![=]) {
                return Err(syn::Error::new(
                    name.span(),
                    "write parameters as `name: Type`, for example `mbpkgid: u32`",
                ));
            }
            input.parse::<Token![:]>()?;
            let ty: Type = input.parse()?;
            args.push((name, ty));
        }
        if !input.is_empty() {
            return Err(input.error("unexpected tokens in endpoint attribute"));
        }
        Ok(Self { path, args })
    }
}

fn validate_placeholders(path: &LitStr, argc: usize) -> syn::Result<()> {
    let value = path.value();
    let mut count = 0usize;
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '{' {
            match chars.next() {
                Some('{') => {}
                Some('}') => count += 1,
                _ => {
                    return Err(syn::Error::new(
                        path.span(),
                        "path placeholders must be `{}`",
                    ));
                }
            }
        } else if ch == '}' {
            match chars.next() {
                Some('}') => {}
                _ => {
                    return Err(syn::Error::new(
                        path.span(),
                        "unmatched `}` in path",
                    ));
                }
            }
        }
    }
    if count != argc {
        return Err(syn::Error::new(
            path.span(),
            format!(
                "path has {count} `{{}}` placeholders but {argc} parameters"
            ),
        ));
    }
    Ok(())
}

fn expand(item: TokenStream, attr_name: &str, list: bool) -> TokenStream {
    let mut ast = match syn::parse::<DeriveInput>(item) {
        Ok(ast) => ast,
        Err(err) => return err.into_compile_error().into(),
    };
    let outcome: syn::Result<proc_macro2::TokenStream> = (|| {
        let pos = ast
            .attrs
            .iter()
            .position(|attr| attr.path().is_ident(attr_name))
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    &ast.ident,
                    format!("missing #[{attr_name}(path = \"...\")] attribute"),
                )
            })?;
        let attr = ast.attrs.remove(pos);
        let parsed: EndpointAttr = attr.parse_args()?;
        validate_placeholders(&parsed.path, parsed.args.len())?;
        Ok(emit(&ast.ident, &parsed, list))
    })();
    match outcome {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn emit(
    ident: &Ident, attr: &EndpointAttr, list: bool,
) -> proc_macro2::TokenStream {
    let path = &attr.path;
    let params = attr.args.iter().map(|(name, ty)| quote!(#name: #ty));
    let names: Vec<_> = attr.args.iter().map(|(name, _)| name).collect();
    let fetch = if names.is_empty() {
        quote!(client.get_data(#path).await?)
    } else {
        quote!(client.get_data(&format!(#path, #(#names),*)).await?)
    };
    let method = Ident::new(if list { "list" } else { "get" }, ident.span());
    let output = if list {
        quote!(Vec<#ident>)
    } else {
        quote!(#ident)
    };
    let parsed = if list {
        quote! {
            let results: Vec<#ident> = ::serde_json::from_value(data).map_err(|err| {
                crate::errors::NaApiError::Deserialize(err.to_string())
            })?;
            Ok(results)
        }
    } else {
        quote! {
            let result: #ident = ::serde_json::from_value(data).map_err(|err| {
                crate::errors::NaApiError::Deserialize(err.to_string())
            })?;
            Ok(result)
        }
    };

    quote! {
        impl #ident {
            pub async fn #method(
                client: &crate::NaClient,
                #(#params),*
            ) -> ::core::result::Result<#output, crate::errors::NaApiError> {
                let data = #fetch;
                #parsed
            }
        }
    }
}

#[proc_macro_derive(EndpointGetOne, attributes(getone))]
pub fn endpointgetone_derive_macro(item: TokenStream) -> TokenStream {
    expand(item, "getone", false)
}

#[proc_macro_derive(EndpointGetAll, attributes(getall))]
pub fn endpointgetall_derive_macro(item: TokenStream) -> TokenStream {
    expand(item, "getall", true)
}

struct WriteAttr {
    verb: String,
    method_name: Ident,
    path: LitStr,
    form: bool,
    json: bool,
    body: Option<Type>,
    returns: Option<Type>,
    args: Vec<(Ident, Type)>,
}

impl Parse for WriteAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let verb: Ident = input.parse()?;
        let verb_name = verb.to_string();
        if !matches!(verb_name.as_str(), "post" | "put" | "patch" | "delete") {
            return Err(syn::Error::new(
                verb.span(),
                "expected `post`, `put`, `patch`, or `delete`",
            ));
        }
        let mut method_name = None;
        let mut path = None;
        let mut form = false;
        let mut json = false;
        let mut body = None;
        let mut returns = None;
        let mut args = Vec::new();
        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
            if input.peek(Ident) && input.peek2(Token![=]) {
                let key: Ident = input.parse()?;
                input.parse::<Token![=]>()?;
                match key.to_string().as_str() {
                    "name" => {
                        let lit: LitStr = input.parse()?;
                        method_name = Some(
                            syn::parse_str::<Ident>(&lit.value()).map_err(
                                |_| {
                                    syn::Error::new(
                                        lit.span(),
                                        "method name must be an identifier",
                                    )
                                },
                            )?,
                        );
                    }
                    "path" => path = Some(input.parse()?),
                    "returns" => returns = Some(input.parse()?),
                    _ => {
                        return Err(syn::Error::new(
                            key.span(),
                            "expected `name`, `path`, or `returns`",
                        ));
                    }
                }
            } else if input.peek(Ident) && input.peek2(Token![:]) {
                let name: Ident = input.parse()?;
                input.parse::<Token![:]>()?;
                let ty: Type = input.parse()?;
                if name == "body" {
                    body = Some(ty);
                } else {
                    args.push((name, ty));
                }
            } else {
                let flag: Ident = input.parse()?;
                match flag.to_string().as_str() {
                    "form" => form = true,
                    "json" => json = true,
                    _ => {
                        return Err(syn::Error::new(
                            flag.span(),
                            "expected `form`, `json`, `name: Type`, or `body: Type`",
                        ));
                    }
                }
            }
        }
        if !input.is_empty() {
            return Err(input.error("unexpected tokens in write attribute"));
        }
        if form && json {
            return Err(syn::Error::new(
                verb.span(),
                "choose either `form` or `json`",
            ));
        }
        let method_name = method_name.ok_or_else(|| {
            syn::Error::new(verb.span(), "missing `name = \"...\"`")
        })?;
        let path = path.ok_or_else(|| {
            syn::Error::new(verb.span(), "missing `path = \"...\"`")
        })?;
        Ok(Self {
            verb: verb_name,
            method_name,
            path,
            form,
            json,
            body,
            returns,
            args,
        })
    }
}

fn expand_writes(item: TokenStream) -> TokenStream {
    let ast = match syn::parse::<DeriveInput>(item) {
        Ok(ast) => ast,
        Err(err) => return err.into_compile_error().into(),
    };
    let outcome: syn::Result<proc_macro2::TokenStream> = (|| {
        let mut writes = Vec::new();
        for attr in &ast.attrs {
            if attr.path().is_ident("write") {
                let parsed: WriteAttr = attr.parse_args()?;
                validate_placeholders(&parsed.path, parsed.args.len())?;
                writes.push(parsed);
            }
        }
        if writes.is_empty() {
            return Err(syn::Error::new_spanned(
                &ast.ident,
                "missing #[write(post, name = \"...\", path = \"...\")] attribute",
            ));
        }
        let ident = &ast.ident;
        let methods = writes.iter().map(|attr| emit_write(ident, attr));
        Ok(quote! {
            impl #ident {
                #(#methods)*
            }
        })
    })();
    match outcome {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn emit_write(ident: &Ident, attr: &WriteAttr) -> proc_macro2::TokenStream {
    let path = &attr.path;
    let method_name = &attr.method_name;
    let http = match attr.verb.as_str() {
        "post" => "POST",
        "put" => "PUT",
        "patch" => "PATCH",
        "delete" => "DELETE",
        _ => "POST",
    };
    let params = attr.args.iter().map(|(name, ty)| quote!(#name: #ty));
    let names: Vec<_> = attr.args.iter().map(|(name, _)| name).collect();
    let formatted = if names.is_empty() {
        quote!(#path)
    } else {
        quote!(&format!(#path, #(#names),*))
    };
    let (body_param, payload) = match &attr.body {
        Some(ty) if attr.form => {
            (quote!(body: &#ty,), quote!(Some(crate::form_body(body)?)))
        }
        Some(ty) => (
            quote!(body: &#ty,),
            quote! {
                Some(::serde_json::to_vec(body).map_err(|err| {
                    crate::errors::NaApiError::Deserialize(err.to_string())
                })?)
            },
        ),
        None if attr.json => (quote!(), quote!(Some(b"null".to_vec()))),
        None if attr.form => (quote!(), quote!(Some(Vec::new()))),
        None => (quote!(), quote!(None)),
    };
    let content_type = if attr.form {
        quote!(Some("application/x-www-form-urlencoded"))
    } else if attr.json || attr.body.is_some() {
        quote!(Some("application/json"))
    } else {
        quote!(None)
    };
    let output = match &attr.returns {
        Some(ty) => quote!(#ty),
        None => quote!(#ident),
    };
    let finish = if matches!(&attr.returns, Some(ty) if is_unit(ty)) {
        quote! {
            let _data = data;
            Ok(())
        }
    } else {
        quote! {
            let result: #output = ::serde_json::from_value(data).map_err(|err| {
                crate::errors::NaApiError::Deserialize(err.to_string())
            })?;
            Ok(result)
        }
    };
    quote! {
        pub async fn #method_name(
            client: &crate::NaClient,
            #(#params,)*
            #body_param
        ) -> ::core::result::Result<#output, crate::errors::NaApiError> {
            let data = client.send_data(
                #http,
                #formatted,
                #content_type,
                #payload,
            ).await?;
            #finish
        }
    }
}

fn is_unit(ty: &Type) -> bool {
    matches!(ty, Type::Tuple(tuple) if tuple.elems.is_empty())
}

#[proc_macro_derive(EndpointWrite, attributes(write))]
pub fn endpoint_write_derive_macro(item: TokenStream) -> TokenStream {
    expand_writes(item)
}
