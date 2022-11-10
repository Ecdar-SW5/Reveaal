use crate::logging::setup_logger;
use crate::DataReader::component_loader::{JsonProjectLoader, XmlProjectLoader};
use crate::DataReader::parse_queries;
use crate::ModelObjects::queries::Query;
use crate::System::executable_query::QueryResult;
use crate::System::extract_system_rep::create_executable_query;
use crate::System::refine::RefinementResult;

fn try_setup_logging() {
    #[cfg(feature = "logging")]
    let _ = setup_logger();
}

pub fn xml_refinement_check(PATH: &str, QUERY: &str) -> bool {
    try_setup_logging();
    match xml_run_query(PATH, QUERY) {
        QueryResult::Refinement(RefinementResult::Success) => true,
        QueryResult::Refinement(RefinementResult::Failure(_)) => false,
        QueryResult::Error(err) => panic!("{}", err),
        _ => panic!("Not a refinement check"),
    }
}

pub fn json_refinement_check(PATH: &str, QUERY: &str) -> bool {
    try_setup_logging();

    match json_run_query(PATH, QUERY) {
        QueryResult::Refinement(RefinementResult::Success) => true,
        QueryResult::Refinement(RefinementResult::Failure(_)) => false,
        QueryResult::Error(err) => panic!("{}", err),
        _ => panic!("Not a refinement check"),
    }
}

pub fn xml_run_query(PATH: &str, QUERY: &str) -> QueryResult {
    let project_path = String::from(PATH);
    let project_loader = XmlProjectLoader::new(project_path);
    let query = parse_queries::parse_to_expression_tree(QUERY)
        .unwrap()
        .remove(0);
    let q = Query {
        query: Option::from(query),
        comment: "".to_string(),
    };

    let mut comp_loader = project_loader.to_comp_loader();
    let query = create_executable_query(&q, &mut *comp_loader).unwrap();

    query.execute()
}

pub fn json_run_query(PATH: &str, QUERY: &str) -> QueryResult {
    let project_loader = JsonProjectLoader::new(String::from(PATH));
    let query = parse_queries::parse_to_expression_tree(QUERY)
        .unwrap()
        .remove(0);
    let q = Query {
        query: Option::from(query),
        comment: "".to_string(),
    };

    let mut comp_loader = project_loader.to_comp_loader();
    let query = create_executable_query(&q, &mut *comp_loader).unwrap();

    query.execute()
}
