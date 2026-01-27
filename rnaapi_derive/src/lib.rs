//use proc_macro::TokenStream;
//use rnahelpers::EndpointGetArgs;
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

fn impl_endpointgetall_trait(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    // get struct identifier
    let mut ast: DeriveInput = syn::parse2(item)?;

    // get the attributes (required)
    let EndpointGetAllAttrs { path, args } =
        deluxe::extract_attributes(&mut ast)?;
    let ident = ast.ident;

    if args == 0 {
        Ok(quote::quote! {
            #[async_trait]
            impl EndpointGetAll for #ident {
                type Endpoint = #ident;
                async fn get_all(
                    na_client: &NaClient, args: EndpointGetArgs
                ) -> Result<Vec<#ident>, NaApiError> {
                    match args {
                        EndpointGetArgs::NoArgs => {
                            let data = na_client.get_data(&format!(#path)).await?;
                            let results: Vec<#ident> = serde_json::from_value(data).unwrap();
                            Ok(results)
                        }
                        _ => Err(NaApiError::UnknownError(
                                "No args Allowed".to_owned(),
                        ))
                    }
                }
            }
        })
    } else if args == 1 {
        Ok(quote::quote! {
            #[async_trait]
            impl EndpointGetAll for #ident {
                type Endpoint = #ident;
                async fn get_all(
                    na_client: &NaClient, args: EndpointGetArgs
                ) -> Result<Vec<#ident>, NaApiError> {
                    match args {
                        EndpointGetArgs::OneInt(arg1) => {
                            let data = na_client.get_data(&format!(#path, arg1)).await?;
                            let results: Vec<#ident> = serde_json::from_value(data).unwrap();
                            Ok(results)
                        }
                        _ => Err(NaApiError::UnknownError(
                                "Only one arg Allowed".to_owned(),
                        ))
                    }
                }
            }
        })
    } else {
        Ok(quote::quote! {
            #[async_trait]
            impl EndpointGetAll for #ident {
                type Endpoint = #ident;
                async fn get_all(
                    na_client: &NaClient, args: EndpointGetArgs
                ) -> Result<Vec<#ident>, NaApiError> {
                    match args {
                        EndpointGetArgs::TwoInt(arg1, arg2) => {
                            let data = na_client.get_data(&format!(#path, arg1, arg2)).await?;
                            let results: Vec<#ident> = serde_json::from_value(data).unwrap();
                            Ok(results)
                        }
                        _ => Err(NaApiError::UnknownError(
                                "Only two args Allowed".to_owned(),
                        ))
                    }
                }
            }
        })
    }
}

fn impl_endpointgetone_trait(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    // get struct identifier
    let mut ast: DeriveInput = syn::parse2(item)?;

    // get the attributes (required)
    let EndpointGetOneAttrs { path, args } =
        deluxe::extract_attributes(&mut ast)?;
    let ident = ast.ident;

    // generate impl

    if args == 0 {
        Ok(quote::quote! {
            #[async_trait]
            impl EndpointGetOne for #ident {
                type Endpoint = #ident;
                async fn get_one(
                    na_client: &NaClient, args: EndpointGetArgs
                ) -> Result<#ident, NaApiError> {
                    match args {
                        EndpointGetArgs::NoArgs => {
                            let data = na_client.get_data(&format!(#path)).await?;
                            let result: #ident = serde_json::from_value(data).unwrap();
                            Ok(result)
                        }
                        _ => Err(NaApiError::UnknownError(
                                "No args Allowed".to_owned(),
                        ))
                    }
                }
            }
        })
    } else if args == 1 {
        Ok(quote::quote! {
            #[async_trait]
            impl EndpointGetOne for #ident {
                type Endpoint = #ident;
                async fn get_one(
                    na_client: &NaClient, args: EndpointGetArgs
                ) -> Result<#ident, NaApiError> {
                    match args {
                        EndpointGetArgs::OneInt(arg1) => {
                            let data = na_client.get_data(&format!(#path, arg1)).await?;
                            let result: #ident = serde_json::from_value(data).unwrap();
                            Ok(result)
                        }
                        _ => Err(NaApiError::UnknownError(
                                "Only one arg Allowed".to_owned(),
                        ))
                    }
                }
            }
        })
    } else {
        Ok(quote::quote! {
            #[async_trait]
            impl EndpointGetOne for #ident {
                type Endpoint = #ident;
                async fn get_one(
                    na_client: &NaClient, args: EndpointGetArgs
                ) -> Result<#ident, NaApiError> {
                    match args {
                        EndpointGetArgs::TwoInt(arg1, arg2) => {
                            let data = na_client.get_data(&format!(#path, arg1, arg2)).await?;
                            let result: #ident = serde_json::from_value(data).unwrap();
                            Ok(result)
                        }
                        _ => Err(NaApiError::UnknownError(
                                "Only two args Allowed".to_owned(),
                        ))
                    }
                }
            }
        })
    }
}

#[proc_macro_derive(EndpointGetOne, attributes(getone))]
pub fn endpointgetone_derive_macro(
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // generate
    impl_endpointgetone_trait(item.into()).unwrap().into()
}

#[proc_macro_derive(EndpointGetAll, attributes(getall))]
pub fn endpointgetall_derive_macro(
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // generate
    impl_endpointgetall_trait(item.into()).unwrap().into()
}
