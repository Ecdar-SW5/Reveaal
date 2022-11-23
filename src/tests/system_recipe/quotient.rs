#[cfg(test)]

mod test {
    use crate::tests::refinement::Helper::json_run_query;

    const PATH: &str = "samples/json/SystemRecipe/Quotient";

    #[test]
    fn quotient1_fails_correctly() {
        let actual = json_run_query(PATH, "consistency: LeftQuotient1 / RightQuotient1");
        //TODO: Assertion
    }

    #[test]
    fn left_quotient_fails_correctly() {
        let actual = json_run_query(
            PATH,
            "consistency: NotDeterministicQuotientComp / DeterministicQuotientComp",
        );
        //TODO: Assertion
    }

    #[test]
    fn right_quotient_fails_correctly() {
        let actual = json_run_query(
            PATH,
            "consistency: DeterministicQuotientComp / NotDeterministicQuotientComp",
        );
        //TODO: Assertion
    }
}
