pub mod errors;
mod projects_query;

use std::collections::HashMap;

use graphql_client::GraphQLQuery;
use projects_query::{
    projects_query::{ResponseData, Variables},
    ProjectsQuery,
};
use serde::{Deserialize, Serialize};

const GH_API_URL: &str = "https://api.github.com/graphql";

#[allow(dead_code)]
pub struct GhClient {
    client: reqwest::Client,
    user: String,
    token: String,
}

#[allow(dead_code)]
pub trait ProjectsClient {
    async fn get_projects(&self, org: &str, cursor: Option<&str>) ->  Result<GhResponse<ResponseData>, errors::GithubError>;
}

#[derive(Deserialize, Debug)]
pub struct GhResponse<T> {
    pub data: Option<T>
}

impl ProjectsClient for GhClient {
    async fn get_projects(&self, org: &str, cursor: Option<&str>) -> Result<GhResponse<ResponseData>, errors::GithubError> {
        let body = ProjectsQuery::build_query(Variables {
            owner: org.to_string(),
            cursor: cursor.unwrap_or("").to_string(),
        });

        let params: HashMap<String, String> = HashMap::new();

        let resp = self
            .post_request("", params, body)
            .await?
            .json::<GhResponse<ResponseData>>().await?;

        Ok(resp)
    }
}

impl GhClient {
    pub fn new(user: String, token: String) -> Self {
        GhClient {
            client: reqwest::Client::new(),
            user,
            token,
        }
    }

    async fn post_request(
        &self,
        path: &str,
        params: impl Serialize,
        body: impl Serialize,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = format!("{}{}", GH_API_URL, path);

        self.client
            .post(url)
            .query(&params)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("X-GitHub-Api-Version", "2022-11-28")
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "Contribute-Count")
            .json(&body)
            .send()
            .await
    }
}
