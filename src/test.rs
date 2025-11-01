    // Get "data" from result
    pub async fn get_data(&self, mbpkgid: u32) -> Result<Value, reqwest::Error> {
        let server_data = self
            .http_client
            .get(format!(
                "{}server/?key={}&mbpkgid={mbpkgid}",
                self.address, self.api_key
            ))
            .send()
            .await?
            .json::<Value>()
            .await;

        match server_data {
            Ok(json_result) => Ok(json_result["data"].clone()),
            Err(error) => Err(error),
        }
    }

