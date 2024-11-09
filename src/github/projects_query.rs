#![allow(clippy::all, warnings)]
pub struct ProjectsQuery;
pub mod projects_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "ProjectsQuery";
    pub const QUERY : & str = "query ProjectsQuery($owner: String!, $cursor:String!) {\n  organization(login: $owner) {\n    projectsV2(first: 10, after: $cursor) {\n      __typename\n      nodes {\n        __typename\n        number\n        title\n      }\n      pageInfo {\n        endCursor\n        startCursor\n      }\n      totalCount\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    #[derive(Serialize, Debug)]
    pub struct Variables {
        pub owner: String,
        pub cursor: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub organization: Option<ProjectsQueryOrganization>,
    }
    #[derive(Deserialize, Debug)]
    pub struct ProjectsQueryOrganization {
        #[serde(rename = "projectsV2")]
        pub projects_v2: ProjectsQueryOrganizationProjectsV2,
    }
    #[derive(Deserialize, Debug)]
    pub struct ProjectsQueryOrganizationProjectsV2 {
        pub nodes: Option<Vec<Option<ProjectsQueryOrganizationProjectsV2Nodes>>>,
        #[serde(rename = "pageInfo")]
        pub page_info: ProjectsQueryOrganizationProjectsV2PageInfo,
        #[serde(rename = "totalCount")]
        pub total_count: Int,
    }
    #[derive(Deserialize, Debug)]
    pub struct ProjectsQueryOrganizationProjectsV2Nodes {
        pub number: Int,
        pub title: String,
    }
    #[derive(Deserialize, Debug)]
    pub struct ProjectsQueryOrganizationProjectsV2PageInfo {
        #[serde(rename = "endCursor")]
        pub end_cursor: Option<String>,
        #[serde(rename = "startCursor")]
        pub start_cursor: Option<String>,
    }
}
impl graphql_client::GraphQLQuery for ProjectsQuery {
    type Variables = projects_query::Variables;
    type ResponseData = projects_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: projects_query::QUERY,
            operation_name: projects_query::OPERATION_NAME,
        }
    }
}
