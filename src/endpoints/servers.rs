#![allow(clippy::too_many_arguments)]
use std::fmt::format;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::NaClient;

// Define a module to hold the custom serialization/deserialization logic.
// This is kind of BS to have to do...
mod custom_datetime_format {
    use chrono::{NaiveDateTime, ParseResult};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    // The signature for a `serialize_with` function must take the value being
    // serialized and a serializer.
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature for a `deserialize_with` function must take a deserializer.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

//
// Server struct
//
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Server {
    pub city: String,
    pub fqdn: String,
    #[serde(rename = "domU_package")]
    pub domu_package: u32,
    pub mbpkgid: u32,
    pub os_id: u32,
    pub location_id: u32,
    pub ip: String,
    pub ipv6: String,
    pub plan_id: u32,
    pub pkg_id: u32,
    pub state: String,
    pub status: String,
    pub uptime: String,
    pub installed: u32,
    pub nic1_mac: String,
    pub nic2_mac: String,
}

// Get Server
impl NaClient {
    pub async fn get_server(&self, mbpkgid: u32) -> Result<Server, reqwest::Error> {
        let data = self
            .get_data(&format!("server?mbpkgid={mbpkgid}").to_owned())
            .await?;
        let server: Server = serde_json::from_value(data).unwrap();
        Ok(server)
    }
}

// Get Servers
impl NaClient {
    pub async fn get_servers(&self) -> Result<Vec<Server>, reqwest::Error> {
        let data = self.get_data("servers").await?;
        let servers: Vec<Server> = serde_json::from_value(data).unwrap();
        Ok(servers)
    }
}

//
// Job structs
// URL: https://vapi2.netactuate.com/api/cloud/server/{mbpkgid}/jobs/{jobid}
// URL: https://vapi2.netactuate.com/api/cloud/server/{mbpkgid}/jobs
//

// JobData struct
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SrvJobData {
    pub code: u32,
    pub result: String,
    pub data: SrvJob,
}

// SrvJobsData struct (plural)
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SrvJobsData {
    pub code: u32,
    pub result: String,
    pub data: Vec<SrvJob>,
}

// SrvJob struct
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SrvJob {
    pub id: u32,
    #[serde(with = "custom_datetime_format")]
    pub ts_insert: NaiveDateTime,
    pub command: String,
    pub status: u32,
}

// Get Job
impl NaClient {
    pub async fn get_job(&self, mbpkgid: u32, jobid: u32) -> Result<SrvJob, reqwest::Error> {
        let server_job_data = self
            .http_client
            .get(format!(
                "{}server/{mbpkgid}/jobs/{jobid}?key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<SrvJobData>()
            .await?;
        Ok(server_job_data.data)
    }

    // Get Jobs
    pub async fn get_jobs(&self, mbpkgid: u32) -> Result<Vec<SrvJob>, reqwest::Error> {
        let server_jobs_data = self
            .http_client
            .get(format!(
                "{}server/{mbpkgid}/jobs?key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<SrvJobsData>()
            .await?;
        Ok(server_jobs_data.data)
    }
}

//
// Status structs
// URL: https://vapi2.netactuate.com/api/cloud/status/{mbpkgid}
//

// SrvStatusData struct
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SrvStatusData {
    pub code: u32,
    pub result: String,
    pub data: SrvStatus,
}

// SrvStatus struct
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SrvStatus {
    pub status: String,
}

// Get Server Status
impl NaClient {
    pub async fn get_status(&self, mbpkgid: u32) -> Result<SrvStatus, reqwest::Error> {
        let srv_status_data = self
            .http_client
            .get(format!(
                "{}status/{mbpkgid}?key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<SrvStatusData>()
            .await?;
        Ok(srv_status_data.data)
    }
}

//
// IPv4IP structs
// URL: https://vapi2.netactuate.com/api/cloud/ipv4?mbpkgid=<mbpkgid>&key=<api_key>
//

// IPv4Data struct
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPv4Data {
    pub code: u32,
    pub result: String,
    pub data: Vec<IPv4>,
}

// IPv4 struct
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IPv4 {
    pub id: u32,
    pub primary: u32,
    pub reverse: String,
    pub ip: String,
    pub netmask: String,
    pub gateway: String,
    pub broadcast: String,
}

// Get IPv4 Data
impl NaClient {
    pub async fn get_ipv4(&self, mbpkgid: u32) -> Result<Vec<IPv4>, reqwest::Error> {
        let ipv4_data = self
            .http_client
            .get(format!(
                "{}ipv4?mbpkgid={mbpkgid}&key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<IPv4Data>()
            .await?;
        Ok(ipv4_data.data)
    }
}

//
// IPv6IP structs
// URL: https://vapi2.netactuate.com/api/cloud/ipv6?mbpkgid=<mbpkgid>&key=<api_key>
//

// IPv6Data struct
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct IPv6Data {
    pub code: u32,
    pub result: String,
    pub data: Vec<IPv6>,
}

// IPv6 struct
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct IPv6 {
    pub id: u32,
    pub primary: u32,
    pub reverse: String,
    pub ip: String,
    pub netmask: String,
    pub gateway: String,
    pub broadcast: String,
}

// Get IPv6 Data
impl NaClient {
    pub async fn get_ipv6(&self, mbpkgid: u32) -> Result<Vec<IPv6>, reqwest::Error> {
        let ipv6_data = self
            .http_client
            .get(format!(
                "{}ipv6?mbpkgid={mbpkgid}&key={}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<IPv6Data>()
            .await?;
        Ok(ipv6_data.data)
    }
}

//
// SrvSummary structs
// URL: https://vapi2.netactuate.com/api/cloud/serversummary/{mbpkgid}
// TODO: start here and keep going

// impl Server {
//     pub fn new(
//         city: String,
//         domU_package: u32,
//         fqdn: String,
//         mbpkgid: u32,
//         os_id: u32,
//         location_id: u32,
//         ip: String,
//         ipv6: String,
//         plan_id: u32,
//         pkg_id: u32,
//         state: String,
//         status: String,
//         uptime: String,
//         installed: u32,
//         nic1_mac: String,
//         nic2_mac: String,
//     ) -> Self {
//         Self {
//             city,
//             domU_package,
//             fqdn,
//             mbpkgid,
//             os_id,
//             location_id,
//             ip,
//             ipv6,
//             plan_id,
//             pkg_id,
//             state,
//             status,
//             uptime,
//             installed,
//             nic1_mac,
//             nic2_mac,
//         }
//     }
// }

/*
// Server struct defines what a VPS looks like
type Server struct {
    Name                     string `json:"fqdn"`
    ID                       int    `json:"mbpkgid"`
    OS                       string `json:"os"`
    OSID                     int    `json:"os_id"`
    PrimaryIPv4              string `json:"ip"`
    PrimaryIPv6              string `json:"ipv6"`
    PlanID                   int    `json:"plan_id"`
    Package                  string `json:"package"`
    PackageBilling           string `json:"package_billing"`
    PackageBillingContractId string `json:"package_billing_contract_id"`
    Location                 string `json:"city"`
    LocationID               int    `json:"location_id"`
    ServerStatus             string `json:"status"`
    PowerStatus              string `json:"state"`
    Installed                int    `json:"installed"`
}

// GetServers external method on Client to list your instances
func (c *Client) GetServers() ([]Server, error) {
    var serverList []Server
    if err := c.get("cloud/servers", &serverList); err != nil {
        return nil, err
    }
    return serverList, nil
}

// GetServer external method on Client to get an instance
func (c *Client) GetServer(id int) (server Server, err error) {
    if err := c.get("cloud/server?mbpkgid="+strconv.Itoa(id), &server); err != nil {
        return server, err
    }
    return server, nil
}

// CreateServerRequest is as set of parameters for a server creation call.
type CreateServerRequest struct {
    Plan                     string `url:"plan,omitempty"`
    Location                 int    `url:"location,omitempty"`
    Image                    int    `url:"image,omitempty"`
    FQDN                     string `url:"fqdn,omitempty"`
    SSHKey                   string `url:"ssh_key,omitempty"`
    SSHKeyID                 int    `url:"ssh_key_id,omitempty"`
    Password                 string `url:"password,omitempty"`
    PackageBilling           string `url:"package_billing,omitempty"`
    PackageBillingContractId string `url:"package_billing_contract_id,omitempty"`
    CloudConfig              string `url:"cloud_config,omitempty"`
    ScriptContent            string `url:"script_content,omitempty"`
    Params     				 string `url:"params,omitempty"`
}

// ServerBuild is a server creation response message.
type ServerBuild struct {
    ServerID int    `json:"mbpkgid"`
    Status   string `json:"status"`
    Build    int    `json:"build"`
}

// CreateServer external method on Client to buy and build a new instance.
func (c *Client) CreateServer(r *CreateServerRequest) (b ServerBuild, err error) {
    values, err := query.Values(r)
    if err != nil {
        return b, err
    }
    if values.Has("script_content") {
        values.Add("script_type", "user-data")
    }

    if err := c.post("cloud/server/buy_build", []byte(values.Encode()), &b); err != nil {
        return b, err
    }

    return b, nil
}

// BuildServerRequest is a set of parameters for a server re-building call.
type BuildServerRequest struct {
    Plan                     string `url:"plan,omitempty"`
    Location                 int    `url:"location,omitempty"`
    Image                    int    `url:"image,omitempty"`
    FQDN                     string `url:"fqdn,omitempty"`
    SSHKey                   string `url:"ssh_key,omitempty"`
    SSHKeyID                 int    `url:"ssh_key_id,omitempty"`
    Password                 string `url:"password,omitempty"`
    PackageBilling           string `url:"package_billing,omitempty"`
    PackageBillingContractId string `url:"package_billing_contract_id,omitempty"`
    CloudConfig              string `url:"cloud_config,omitempty"`
    ScriptContent            string `url:"script_content,omitempty"`
    Params     				 string `url:"params,omitempty"`
}

// BuildServer external method on Client to re-build an instance
func (c *Client) BuildServer(id int, r *BuildServerRequest) (b ServerBuild, err error) {
    values, err := query.Values(r)
    if err != nil {
        return b, err
    }
    if values.Has("script_content") {
        values.Add("script_type", "user-data")
    }

    // if r.Params != "" {
    //     values.Add("params", r.Params)
    // }

    if err := c.post("cloud/server/build/"+strconv.Itoa(id), []byte(values.Encode()), &b); err != nil {
        return b, err
    }

    return b, nil
}

// DeleteServer external method on Client to destroy an instance.
func (c *Client) DeleteServer(id int, cancelBilling bool) error {
    values := url.Values{}
    if cancelBilling {
        values.Add("cancel_billing", "1")
    }
    return c.post("cloud/server/delete?mbpkgid="+strconv.Itoa(id), []byte(values.Encode()), nil)
}

// UnlinkServer external method on Client to unlink a billing package from a location
func (c *Client) UnlinkServer(id int) error {
    return c.post("cloud/server/unlink/"+strconv.Itoa(id), nil, nil)
}

// StartServer external method on Client to boot up an instance
func (c *Client) StartServer(id int) error {

    if err := c.post("cloud/server/start/"+strconv.Itoa(id), nil, nil); err != nil {
        return err
    }

    return nil
}

// StopServer external method on Client to shut down an instance
func (c *Client) StopServer(id int) error {

    if err := c.post("cloud/server/shutdown/"+strconv.Itoa(id), nil, nil); err != nil {
        return err
    }

    return nil
}
*/
