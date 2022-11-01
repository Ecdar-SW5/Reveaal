#[cfg(test)]
mod edge_tests {
    use crate::DataReader::json_reader::read_json_component;
    use test_case::test_case;

    const FOLDER_PATH: &str = "samples/json/EcdarUniversity";

    #[test_case(FOLDER_PATH, "Machine", vec!["E25".to_string(), "E26".to_string(), "E27".to_string(), "E28".to_string(), "E29".to_string()]; "Edge ID test on Machine from the ECDAR University")]
    fn edge_id_checking(path: &str, component_name: &str, edge_ids: Vec<String>) {
        let component = read_json_component(path, component_name);
        for i in 0..component.edges.len() {
            assert_eq!(component.edges[i].id, edge_ids[i])
        }
    }
}
