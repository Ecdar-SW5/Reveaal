#[cfg(test)]

mod test {
    use crate::tests::refinement::Helper::json_run_query;

    const PATH: &str = "samples/json/SystemRecipe/CompiledComponent";

    #[test]
    fn compiled_component1_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: CompiledComponent1");
        //TODO: Assertion
    }

    #[test]
    fn compiled_component2_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: CompiledComponent2");
        //TODO: Assertion
    }

    #[test]
    fn compiled_component3_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: CompiledComponent3");
        //TODO: Assertion
    }
}
