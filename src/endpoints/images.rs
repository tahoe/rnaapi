use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImageData {
    pub result: String,
    pub code: u32,
    pub data: Image,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ImagesData {
    pub result: String,
    pub code: u32,
    pub data: Vec<Image>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Image {
    pub id: u32,
    pub os: Option<String>,
    pub description: Option<String>,
    pub size: Option<String>,
    pub subtype: Option<String>,
    pub created: Option<String>,
    pub category: Option<String>,
    pub updated: Option<String>,
    pub iso: Option<String>,
    pub bits: Option<String>,
    pub tech: Option<String>,
    pub icon: Option<String>,
    pub private: Option<u32>,
}
/*
// OS is a struct for storing the attributes of an OS
type OS struct {
    ID      int    `json:"id"`
    Os      string `json:"os"`
    Type    string `json:"type"`
    Subtype string `json:"subtype"`
    Size    string `json:"size"`
    Bits    string `json:"bits"`
    Tech    string `json:"tech"`
}

// GetOSs returns a list of OS objects from the api
func (c *Client) GetOSs() ([]OS, error) {
    var osList []OS
    if err := c.get("cloud/images", &osList); err != nil {
        return nil, err
    }
    return osList, nil
}
*/
