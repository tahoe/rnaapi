use proc_macro::TokenStream;
//use rnahelpers::EndpointGetArgs;
use syn::DeriveInput;

fn impl_endpointgetall_trait(ast: DeriveInput) -> TokenStream {
    // get struct identifier
    let ident = ast.ident;

    // generate impl
    quote::quote! {
        #[async_trait]
        impl EndpointGet for #ident {
            type Endpoint = #ident;
            async fn get_all(
                na_client: &NaClient, args: EndpointGetArgs
            ) -> Result<Vec<#ident>, NaApiError> {
                match args {
                    EndpointGetArgs::NoArgs => {
                        let data = na_client.get_data(&format!("dns/zones?type=NATIVE")).await?;
                        let results: Vec<#ident> = serde_json::from_value(data).unwrap();
                        Ok(results)
                    }
                    _ => {
                        Err(NaApiError::UnknownError("No arguments allowed".to_owned()))
                    }
                }
            }
        }
    }
    .into()
}

fn impl_endpointgetone_trait(ast: DeriveInput) -> TokenStream {
    // get struct identifier
    let ident = ast.ident;

    // generate impl
    quote::quote! {
        #[async_trait]
        impl EndpointGetOne for #ident {
            type Endpoint = #ident;
            async fn get_one(
                na_client: &NaClient, args: EndpointGetArgs,
            ) -> Result<#ident, NaApiError> {
                match args {
                    EndpointGetArgs::OneInt(arg1) => {
                        let data = na_client
                            .get_data(&format!("dns/zone/{arg1}").to_owned())
                            .await?;
                        let result: #ident = serde_json::from_value(data).unwrap();
                        Ok(result)
                    }
                    _ => Err(NaApiError::UnknownError(
                        "Only one argument allowed".to_owned(),
                    )),
                }
            }
        }
    }
    .into()
}

#[proc_macro_derive(EndpointGetOne)]
pub fn endpointgetone_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // generate
    impl_endpointgetone_trait(ast)
}

#[proc_macro_derive(EndpointGetAll)]
pub fn endpointgetall_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // generate
    impl_endpointgetall_trait(ast)
}
