#[cfg(test)]
mod samples {
    use crate::DataReader::component_loader::JsonProjectLoader;

    static CONJUNCTION_SAMPLE: &str = "samples/json/Conjunction";

    #[test]
    fn test_locations_T1() {
        let mut project_loader = JsonProjectLoader::new(CONJUNCTION_SAMPLE.to_string(), false);
        let t1 = project_loader.get_component("Test1");

        assert_eq!(t1.get_name(), "Test1");
        assert_eq!(t1.get_locations().len(), 2);
    }

    #[test]
    fn test_locations_T2() {
        let mut project_loader = JsonProjectLoader::new(CONJUNCTION_SAMPLE.to_string(), false);
        let t2 = project_loader.get_component("Test2");

        assert_eq!(t2.get_name(), "Test2");
        assert_eq!(t2.get_locations().len(), 2);
    }

    #[test]
    fn test_locations_T3() {
        let mut project_loader = JsonProjectLoader::new(CONJUNCTION_SAMPLE.to_string(), false);
        let t3 = project_loader.get_component("Test3");

        assert_eq!(t3.get_name(), "Test3");
        assert_eq!(t3.get_locations().len(), 3);
    }

    #[test]
    fn test_names_T1_through_T12() {
        let mut project_loader = JsonProjectLoader::new(CONJUNCTION_SAMPLE.to_string(), false);

        for i in 1..12 {
            let t = project_loader.get_component(&format!("Test{}", i).to_string());

            assert_eq!(t.name, format!("Test{}", i));
        }
    }
}
