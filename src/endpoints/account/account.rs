// Copyright (C) 2025 Dennis Durling
// This file is part of RNAAPI Rust API Client Library, licensed
// under the GNU General Public License v3.0
#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

use crate::NaApiError;
use crate::{EndpointGet, EndpointGetArgs, NaClient};
use async_trait::async_trait;

///
/// Account Details #[derive(Debug)]
///
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Details {
    pub result: String,
    pub userid: String,
    pub client_id: String,
    pub id: String,
    pub owner_user_id: String,
    pub uuid: String,
    pub firstname: String,
    pub lastname: String,
    pub fullname: String,
    pub companyname: String,
    pub email: String,
    pub address1: String,
    pub address2: String,
    pub city: String,
    pub fullstate: String,
    pub state: String,
    pub postcode: String,
    pub countrycode: String,
    pub country: String,
    pub phonenumber: String,
    pub tax_id: String,
    pub email_preferences: String,
    pub statecode: String,
    pub countryname: String,
    pub phonecc: String,
    pub phonenumberformatted: String,
    #[serde(rename = "telephoneNumber")]
    pub telephonenumber: String,
    pub billingcid: String,
    pub notes: String,
    pub currency: String,
    pub cclastfour: String,
    pub groupid: String,
    pub status: String,
    pub credit: String,
    #[serde(rename = "allowSingleSignOn")]
    pub allowsinglesignon: String,
    pub lastlogin: String,
}

// Get Details
#[async_trait]
impl EndpointGet for Details {
    type Endpoint = Details;
    /// Get your account details
    async fn get_one(
        na_client: &NaClient, args: EndpointGetArgs,
    ) -> Result<Details, NaApiError> {
        match args {
            EndpointGetArgs::NoArgs => {
                let data = na_client.get_data("account/details").await?;
                let deets: Details = serde_json::from_value(data).unwrap();
                Ok(deets)
            }
            _ => {
                Err(NaApiError::UnknownError("No arguments allowed".to_owned()))
            }
        }
    }
}
