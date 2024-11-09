#![allow(clippy::all, warnings)]
pub struct IssuesQuery;
pub mod issues_query {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "IssuesQuery";
    pub const QUERY : & str = "query IssuesQuery($owner: String!, $proj_num: Int!, $cursor: String!) {\n  organization(login: $owner) {\n    projectV2(number: $proj_num) {\n      id\n      title\n      items(first: 100, after: $cursor) {\n \t\t\t\t__typename       \n        totalCount\n        pageInfo {\n          endCursor\n        }\n        nodes {\n          id\n          __typename\n          content {\n            __typename\n            ... on Issue {\n              __typename\n              createdAt\n              closedAt\n              title\n              assignees(first: 20) {\n                __typename\n                totalCount\n                nodes {\n                  login\n                }\n              }\n            }\n          }\n        }\n      }\n    }\n  }\n}" ;
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
    type DateTime = super::DateTime;
    #[derive(Serialize, Debug)]
    pub struct Variables {
        pub owner: String,
        pub proj_num: Int,
        pub cursor: String,
    }
    impl Variables {}
    #[derive(Deserialize, Debug)]
    pub struct ResponseData {
        pub organization: Option<IssuesQueryOrganization>,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesQueryOrganization {
        #[serde(rename = "projectV2")]
        pub project_v2: Option<IssuesQueryOrganizationProjectV2>,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesQueryOrganizationProjectV2 {
        pub id: ID,
        pub title: String,
        pub items: IssuesQueryOrganizationProjectV2Items,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesQueryOrganizationProjectV2Items {
        #[serde(rename = "totalCount")]
        pub total_count: Int,
        #[serde(rename = "pageInfo")]
        pub page_info: IssuesQueryOrganizationProjectV2ItemsPageInfo,
        pub nodes: Option<Vec<Option<IssuesQueryOrganizationProjectV2ItemsNodes>>>,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesQueryOrganizationProjectV2ItemsPageInfo {
        #[serde(rename = "endCursor")]
        pub end_cursor: Option<String>,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesQueryOrganizationProjectV2ItemsNodes {
        pub id: ID,
        pub content: Option<IssuesQueryOrganizationProjectV2ItemsNodesContent>,
    }
    #[derive(Deserialize, Debug)]
    #[serde(tag = "__typename")]
    pub enum IssuesQueryOrganizationProjectV2ItemsNodesContent {
        DraftIssue,
        Issue(IssuesQueryOrganizationProjectV2ItemsNodesContentOnIssue),
        PullRequest,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesQueryOrganizationProjectV2ItemsNodesContentOnIssue {
        #[serde(rename = "createdAt")]
        pub created_at: DateTime,
        #[serde(rename = "closedAt")]
        pub closed_at: Option<DateTime>,
        pub title: String,
        pub assignees: IssuesQueryOrganizationProjectV2ItemsNodesContentOnIssueAssignees,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesQueryOrganizationProjectV2ItemsNodesContentOnIssueAssignees {
        #[serde(rename = "totalCount")]
        pub total_count: Int,
        pub nodes: Option<
            Vec<Option<IssuesQueryOrganizationProjectV2ItemsNodesContentOnIssueAssigneesNodes>>,
        >,
    }
    #[derive(Deserialize, Debug)]
    pub struct IssuesQueryOrganizationProjectV2ItemsNodesContentOnIssueAssigneesNodes {
        pub login: String,
    }
}
type DateTime = String;
impl graphql_client::GraphQLQuery for IssuesQuery {
    type Variables = issues_query::Variables;
    type ResponseData = issues_query::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: issues_query::QUERY,
            operation_name: issues_query::OPERATION_NAME,
        }
    }
}
