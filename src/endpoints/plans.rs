#![allow(clippy::too_many_arguments, non_snake_case)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageData {
    pub result: String,
    pub data: Package,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackagesData {
    pub result: String,
    pub data: Vec<Package>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Package {
    pub mbpkgid: u32,
    pub package_status: String,
    pub fqdn: String,
    pub name: String,
    pub gid: u32,
    pub domU_package: u32,
    pub rescue: u32,
    pub locked: u32,
    pub package: String,
    pub ipv6: String,
    pub city: String,
    pub ip: String,
    pub installed: u32,
    pub state: String,
    pub uptime: String,
    pub os: String,
    pub is_building: u32,
}
/*

// Plan struct defines the purchaceable plans/packages
type Plan struct {
    ID        int    `json:"plan_id,string"`
    Name      string `json:"plan"`
    RAM       string `json:"ram"`
    Disk      string `json:"disk"`
    Transfer  string `json:"transfer"`
    Price     string `json:"price"`
    Available string `json:"available"`
}

// GetPlans external method on Client to list available Plans
func (c *Client) GetPlans() ([]Plan, error) {

    var planList []Plan

    if err := c.get("cloud/sizes", &planList); err != nil {
        return nil, err
    }

    return planList, nil
}
*/
