use crate::errors::CmdError;

use crate::{
    github::{GhClient, ProjectsClient},
    Args,
};
use std::{error::Error, io::{self, Write}, pin::Pin};

pub async fn run_interactive(arg: &mut Args, client: &mut GhClient) -> Result<(), Box<dyn Error>> {
    println!("Start in interactive mode");
    if arg.username.is_none() {
        print!("Enter github username: ");
        io::stdout().flush()?;
        let mut username = String::new();
        io::stdin().read_line(&mut username).unwrap();
        arg.username = Some(username.trim().to_string());
    }

    if arg.token.is_none() {
        print!("Enter github token: ");
        io::stdout().flush()?;
        let mut token = String::new();
        io::stdin().read_line(&mut token).unwrap();
        arg.token = Some(token.trim().to_string());
    }

    if arg.org.is_none() {
        print!("Enter github organization name: ");
        io::stdout().flush()?;
        let mut org = String::new();
        io::stdin().read_line(&mut org).unwrap();
        arg.org = Some(org.trim().to_string());
    }

    *client = GhClient::new(arg.username.clone().unwrap(), arg.token.clone().unwrap());

    if arg.project_num.is_none() {
        print!("Enter github project number (leave empty to get list of projects): ");
        io::stdout().flush()?;
        let mut num = String::new();
        io::stdin().read_line(&mut num).unwrap();
        if num.trim().is_empty() {
            let project = pick_project(arg, client, None).await?;
            arg.project_num = Some(project.number);
        } else {
            arg.project_num = Some(
                num.trim()
                    .parse::<i64>()
                    .map_err(|_| CmdError::InvalidInput)?,
            );
        }
    }

    if arg.start_date.is_none() {
        print!("Enter start date (yyyy-mm-dd): ");
        io::stdout().flush()?;
        let mut start_date = String::new();
        io::stdin().read_line(&mut start_date).unwrap();
        arg.start_date = Some(start_date.trim().to_string());
    }

    if arg.end_date.is_none() {
        print!("Enter end date (yyyy-mm-dd): ");
        io::stdout().flush()?;
        let mut end_date = String::new();
        io::stdin().read_line(&mut end_date).unwrap();
        arg.end_date = Some(end_date.trim().to_string());
    }

    Ok(())
}

#[derive(Clone)]
struct Project {
    title: String,
    number: i64,
}

fn pick_project<'a>(arg: &'a mut Args, client: &'a GhClient, cursor: Option<&'a str>) -> Pin<Box<dyn std::future::Future<Output = Result<Project, Box<dyn Error>>> + 'a>> {
    Box::pin(async move {
    let resp = client.get_projects(arg.org.as_ref().unwrap(), cursor).await?;
    let data = resp
        .data
        .ok_or(CmdError::NotFound)?
        .organization
        .ok_or(CmdError::NotFound)?
        .projects_v2;
    let projects = data.nodes.ok_or(CmdError::NotFound)?;
    let projects_dat = projects
        .iter()
        .filter_map(|p| {
            Some(Project {
                title: p.as_ref()?.title.clone(),
                number: p.as_ref()?.number,
            })
        })
        .collect::<Vec<Project>>();
    let next_cursor = data.page_info.end_cursor;

    if data.page_info.start_cursor.is_none() {
        return Err(CmdError::NotFound.into());
    }

    println!("Select a project:");
    for (i, proj) in projects_dat.iter().enumerate() {
        println!("[{}]: {}", i, proj.title);
    }
    print!("Enter project number (or n to get next page): ");
    io::stdout().flush()?;
    let mut project_num = String::new();
    io::stdin().read_line(&mut project_num).unwrap();

    // if user wants to get next page
    if project_num == "n\n" {
        return pick_project(arg, client, next_cursor.as_deref()).await;
    }

    let project_num = project_num
        .trim()
        .parse::<usize>()
        .map_err(|_| CmdError::InvalidInput)?;
    Ok(projects_dat[project_num].clone())
    })
}