//! Derive `get` and `list` methods for NetActuate endpoint structs.
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
