#[cfg(test)]

mod test {
    use crate::tests::refinement::Helper::json_run_query;

    const PATH: &str = "samples/json/SystemRecipe/Conjunction";

    #[test]
    fn conjunction1_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftConjunction1 || RightConjunction1");
        //TODO: Assertion
    }

    #[test]
    fn conjunction2_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftConjunction2 || RightConjunction2");
        //TODO: Assertion
    }

    #[test]
    fn conjunction3_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftConjunction3 || RightConjunction3");
        //TODO: Assertion
    }

    #[test]
    fn empty_conjunction_fails_correctly() {
        let actual = json_run_query(
            PATH,
            "consistency: LeftConjunctionEmpty || RightConjunctionEmpty",
        );
        //TODO: Assertion
    }
}
