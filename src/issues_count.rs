use chrono::{DateTime, FixedOffset, NaiveDate};
use std::{collections::HashMap, error::Error};

use crate::{
    errors::CmdError,
    github::{
        issues_query::issues_query::IssuesQueryOrganizationProjectV2ItemsNodesContent, GhClient,
        IssuesClient,
    },
    Args,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Issue {
    title: String,
    created_at: DateTime<FixedOffset>,
    assignees: Vec<String>,
}

pub async fn issues_count(
    args: &Args,
    gh_cl: &GhClient,
) -> Result<HashMap<String, i64>, Box<dyn Error>> {
    let mut iss_count: HashMap<String, i64> = HashMap::new();
    let mut cursor: Option<String> = None;

    let start = NaiveDate::parse_from_str(&args.start_date.as_ref().unwrap(), "%Y-%m-%d")?;
    let end = NaiveDate::parse_from_str(&args.end_date.as_ref().unwrap(), "%Y-%m-%d")?;
    loop {
        let (issues, next_cursor) = get_issues(gh_cl, args, cursor).await?;
        let issue = filter_issues(issues, &start, &end);
        cursor = next_cursor;

        if cursor.is_none() {
            break;
        }

        count_from_issues(&mut iss_count, issue);
    }

    Ok(iss_count)
}

fn count_from_issues(count_map: &mut HashMap<String, i64>, issues: Vec<Issue>) {
    issues.into_iter().for_each(|issue| {
        issue.assignees.into_iter().for_each(|assignee| {
            let count = count_map.entry(assignee).or_insert(0);
            *count += 1;
        });
    });
}

fn filter_issues(
    issues: Vec<Issue>,
    start: &NaiveDate,
    end: &NaiveDate,
) -> Vec<Issue> {
    issues
        .into_iter()
        .filter(|issue| issue.created_at.date_naive() >= *start && issue.created_at.date_naive() <= *end)
        .collect::<Vec<Issue>>()
}

// get issues and transform from api, return next cursor
async fn get_issues(
    gh_cl: &GhClient,
    arg: &Args,
    cursor: Option<String>,
) -> Result<(Vec<Issue>, Option<String>), Box<dyn Error>> {
    let organziation = arg.org.as_ref().unwrap();
    let resp = gh_cl
        .get_issues(organziation, arg.project_num.unwrap(), cursor.as_deref())
        .await?;

    let data = resp
        .data
        .ok_or(CmdError::NotFound)?
        .organization
        .ok_or(CmdError::NotFound)?
        .project_v2
        .ok_or(CmdError::NotFound)?
        .items;

    let next_cursor = data.page_info.end_cursor;

    let issues_data = data.nodes.ok_or(CmdError::NotFound)?;

    let issues_data = issues_data.iter().filter_map(|issue| {
        let issue_unw = issue.as_ref()?;
        let content: &crate::github::issues_query::issues_query::IssuesQueryOrganizationProjectV2ItemsNodesContent = issue_unw.content.as_ref()?;
        let issue = match content {
            IssuesQueryOrganizationProjectV2ItemsNodesContent::Issue(issue) => {
                let title = issue.title.clone();
                let created_at = DateTime::parse_from_rfc3339(&issue.created_at).ok()?;
                let assignees = issue.assignees.nodes.as_ref()?.iter()
                    .filter_map(|a| Some(a.as_ref()?.login.clone()))
                    .collect::<Vec<String>>();
                Some(Issue {
                    assignees: assignees,
                    created_at: created_at,
                    title: title,
                })
            },
            _ => None,
        };
        issue
    }).collect::<Vec<Issue>>();

    Ok((issues_data, next_cursor))
}
