#![allow(clippy::too_many_arguments)]
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackageData {
    pub result: String,
    pub code: u32,
    pub data: Package,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PackagesData {
    pub result: String,
    pub code: u32,
    pub data: Vec<Package>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Package {
    pub mbpkgid: u32,
    pub package_status: String,
    pub fqdn: String,
    pub name: String,
    pub gid: u32,
    #[serde(rename = "domU_package")]
    pub domu_package: u32,
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

import "strconv"

// Package struct stores the purchaced package values
type Package struct {
    ID        int    `json:"mbpkgid,string"`
    Status    string `json:"package_status"`
    Locked    string `json:"locked"`
    PlanName  string `json:"name"`
    Installed int    `json:"installed,string"`
}

// GetPackages external method on Client that returns a
// list of Package object from the API
func (c *Client) GetPackages() ([]Package, error) {

    var packageList []Package

    if err := c.get("cloud/packages", &packageList); err != nil {
        return nil, err
    }

    return packageList, nil
}

// GetPackage external method on Client that takes an id (int) as it's sole
// argument and returns a single Package object
func (c *Client) GetPackage(id int) (pkg Package, err error) {
    if err := c.get("/cloud/package/"+strconv.Itoa(id), &pkg); err != nil {
        return Package{Installed: 0}, err
    }
    return pkg, nil
}
*/
