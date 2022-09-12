#![allow(non_snake_case)]
use clap::{load_yaml, App};
mod logging;
use logging::setup_logger;

use reveaal::{
    extract_system_rep, parse_queries, queries, start_grpc_server_with_tokio, xml_parser,
    ComponentLoader, JsonProjectLoader, ProjectLoader, Query, QueryResult, XmlProjectLoader,
};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "logging")]
    setup_logger().unwrap();
    println!("issue-test")
    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    if let Some(ip_endpoint) = matches.value_of("endpoint") {
        start_grpc_server_with_tokio(ip_endpoint)?;
    } else {
        start_using_cli(&matches);
    }

    Ok(())
}

fn start_using_cli(matches: &clap::ArgMatches) {
    let (mut comp_loader, queries) = parse_args(matches);

    let mut results = vec![];
    for query in &queries {
        let executable_query = Box::new(
            extract_system_rep::create_executable_query(query, &mut *comp_loader).unwrap(),
        );

        let result = executable_query.execute();

        if let QueryResult::Error(err) = result {
            panic!("{}", err);
        }

        results.push(result);
    }

    println!("\nQuery results:");
    for index in 0..queries.len() {
        results[index].print_result(&queries[index].query.as_ref().unwrap().pretty_string())
    }
}

fn parse_args(matches: &clap::ArgMatches) -> (Box<dyn ComponentLoader>, Vec<queries::Query>) {
    let mut folder_path: String = "".to_string();
    let mut query = "".to_string();

    if let Some(folder_arg) = matches.value_of("folder") {
        folder_path = folder_arg.to_string();
    }

    if let Some(query_arg) = matches.value_of("query") {
        query = query_arg.to_string();
    }

    let project_loader = get_project_loader(folder_path);

    if query.is_empty() {
        let queries: Vec<Query> = project_loader.get_queries().clone();

        (project_loader.to_comp_loader(), queries)
    } else {
        let queries = parse_queries::parse_to_query(&query);

        (project_loader.to_comp_loader(), queries)
    }
}

fn get_project_loader(project_path: String) -> Box<dyn ProjectLoader> {
    if xml_parser::is_xml_project(&project_path) {
        XmlProjectLoader::new(project_path)
    } else {
        JsonProjectLoader::new(project_path)
    }
}

pub fn set_working_directory(folder_path: &str) {
    let mut path = std::path::Path::new(folder_path);
    if path.is_file() {
        path = path
            .parent()
            .expect("Failed to find parent directory of input file");
    };
    env::set_current_dir(path).expect("Failed to set working directory to input folder");
}
