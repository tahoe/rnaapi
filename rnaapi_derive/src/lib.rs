use syn::DeriveInput;

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(getone))]
struct EndpointGetOneAttrs {
    path: String,
    args: u32,
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(getall))]
struct EndpointGetAllAttrs {
    path: String,
    args: u32,
}

fn placeholder_count(path: &str) -> usize {
    path.matches("{}").count()
}

fn check_path(
    path: &str, args: u32, span: proc_macro2::Span,
) -> deluxe::Result<()> {
    if args > 2 {
        return Err(syn::Error::new(span, "args must be 0, 1, or 2"));
    }
    let placeholders = placeholder_count(path);
    if placeholders != args as usize {
        return Err(syn::Error::new(
            span,
            format!(
                "path `{path}` has {placeholders} `{{}}` placeholders but args = {args}"
            ),
        ));
    }
    Ok(())
}

fn parse_body(ident: &syn::Ident, list: bool) -> proc_macro2::TokenStream {
    if list {
        quote::quote! {
            let results: Vec<#ident> = ::serde_json::from_value(data).map_err(|err| {
                NaApiError::Deserialize(err.to_string())
            })?;
            Ok(results)
        }
    } else {
        quote::quote! {
            let result: #ident = ::serde_json::from_value(data).map_err(|err| {
                NaApiError::Deserialize(err.to_string())
            })?;
            Ok(result)
        }
    }
}

fn fetch_match(
    args: u32, path: &str, body: proc_macro2::TokenStream,
) -> proc_macro2::TokenStream {
    match args {
        0 => quote::quote! {
            match args {
                EndpointGetArgs::NoArgs => {
                    let data = na_client.get_data(&format!(#path)).await?;
                    #body
                }
                _ => Err(NaApiError::UnknownError(
                    "no arguments allowed".to_owned(),
                )),
            }
        },
        1 => quote::quote! {
            match args {
                EndpointGetArgs::OneInt(arg1) => {
                    let data = na_client.get_data(&format!(#path, arg1)).await?;
                    #body
                }
                _ => Err(NaApiError::UnknownError(
                    "only one argument allowed".to_owned(),
                )),
            }
        },
        _ => quote::quote! {
            match args {
                EndpointGetArgs::TwoInt(arg1, arg2) => {
                    let data = na_client.get_data(&format!(#path, arg1, arg2)).await?;
                    #body
                }
                _ => Err(NaApiError::UnknownError(
                    "only two arguments allowed".to_owned(),
                )),
            }
        },
    }
}

fn impl_endpoint(
    item: proc_macro2::TokenStream, list: bool,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(item)?;
    let (path, args) = if list {
        let EndpointGetAllAttrs { path, args } =
            deluxe::extract_attributes(&mut ast)?;
        (path, args)
    } else {
        let EndpointGetOneAttrs { path, args } =
            deluxe::extract_attributes(&mut ast)?;
        (path, args)
    };
    check_path(&path, args, ast.ident.span())?;
    let ident = &ast.ident;
    let fetch = fetch_match(args, &path, parse_body(ident, list));

    if list {
        Ok(quote::quote! {
            #[async_trait]
            impl EndpointGetAll for #ident {
                type Endpoint = #ident;
                async fn get_all(
                    na_client: &NaClient,
                    args: EndpointGetArgs,
                ) -> Result<Vec<#ident>, NaApiError> {
                    #fetch
                }
            }
        })
    } else {
        Ok(quote::quote! {
            #[async_trait]
            impl EndpointGetOne for #ident {
                type Endpoint = #ident;
                async fn get_one(
                    na_client: &NaClient,
                    args: EndpointGetArgs,
                ) -> Result<#ident, NaApiError> {
                    #fetch
                }
            }
        })
    }
}

fn expand(
    item: proc_macro::TokenStream, list: bool,
) -> proc_macro::TokenStream {
    match impl_endpoint(item.into(), list) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro_derive(EndpointGetOne, attributes(getone))]
pub fn endpointgetone_derive_macro(
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    expand(item, false)
}

#[proc_macro_derive(EndpointGetAll, attributes(getall))]
pub fn endpointgetall_derive_macro(
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    expand(item, true)
}
