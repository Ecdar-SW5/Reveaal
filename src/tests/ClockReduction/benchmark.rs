use crate::{ComponentLoader, DEFAULT_SETTINGS, extract_system_rep, JsonProjectLoader, parse_queries, ProjectLoader, Query, TEST_SETTINGS};

const PATH: &str = "samples/json/ClockReduction/AdvancedClockReduction/Conjunction";
const COMP: &str = "Component1";

fn create_query(operation: &str, shortOp: &str, component: &str, repeat: usize) -> &str {
    &((format!("{}: {}{}", operation, component, format!(" {} {}", shortOp, component).repeat(repeat))) as str)
}

fn comp_loader(path: &str, clock_red: &bool) -> Box<dyn ComponentLoader> {
    if clock_red{
        JsonProjectLoader::new(path.to_string(), DEFAULT_SETTINGS.reduce_clocks_level).to_comp_loader()
    } else {
        JsonProjectLoader::new(path.to_string(), TEST_SETTINGS.reduce_clocks_level).to_comp_loader()
    }
}

fn execute_query(path: &str, query: &str, clock_red: &bool){
    Box::new(
        extract_system_rep::create_executable_query(
        parse_queries::parse_to_query(query).0,
        &mut *comp_loader(path, clock_red))
        .unwrap()
    ).execute();
}

pub fn with_clock_reduction(repeat: usize){
    execute_query(
        PATH,
        create_query(
        "conjunction",
        "&&",
        COMP,
        repeat,
    ), &true)
}

pub fn without_clock_reduction(repeat: usize){
    execute_query(
        PATH,
        create_query(
            "conjunction",
            "&&",
            COMP,
            repeat,
        ), &false)
}