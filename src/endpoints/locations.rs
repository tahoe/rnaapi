#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

use crate::NaClient;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Location {
    pub id: u32,
    pub name: String,
    pub iata_code: String,
    pub continent: String,
    pub flag: String,
    pub latitude: String,
    pub longitude: String,
    pub disabled: u32,
}

//
// Locations
//
impl NaClient {
    pub async fn get_locations(&self) -> Result<Vec<Location>, reqwest::Error> {
        let data = self.get_data("locations").await?;
        let location_list: Vec<Location> = serde_json::from_value(data).unwrap();
        Ok(location_list)
    }
}
/*

// Location is an API response message of available deployment locations
type Location struct {
    ID        int    `json:"id"`
    Name      string `json:"name"`
    IATACode  string `json:"iata_code"`
    Continent string `json:"continent"`
    Flag      string `json:"flat"`
    Disabled  int    `json:"disabled"`
}

// GetLocations public method on Client to get a list of locations
func (c *Client) GetLocations() ([]Location, error) {
    r := make([]Location, 0)
    if err := c.get("cloud/locations", &r); err != nil {
        return nil, err
    }
    return r, nil
}
*/
