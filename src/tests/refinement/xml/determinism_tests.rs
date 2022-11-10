#[cfg(test)]
mod test {
    use crate::tests::refinement::Helper::xml_run_query;
    use crate::System::executable_query::QueryResult;
    use crate::System::local_consistency::DeterminismResult;

    static PATH: &str = "samples/xml/ConsTests.xml";

    fn convert_to_bool(result: DeterminismResult) -> bool {
        matches!(result, DeterminismResult::Success)
    }

    #[test]
    fn testG1() {
        let result = xml_run_query(PATH, "determinism: G1");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }
    #[test]
    fn testG2() {
        let result = xml_run_query(PATH, "determinism: G2");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG3() {
        let result = xml_run_query(PATH, "determinism: G3");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }
    #[test]
    fn testG4() {
        let result = xml_run_query(PATH, "determinism: G4");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG5() {
        let result = xml_run_query(PATH, "determinism: G5");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }
    #[test]
    fn testG6() {
        let result = xml_run_query(PATH, "determinism: G6");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG7() {
        let result = xml_run_query(PATH, "determinism: G7");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG8() {
        let result = xml_run_query(PATH, "determinism: G8");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG9() {
        // shouldn't be deterministic
        let result = xml_run_query(PATH, "determinism: G9");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(!convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG10() {
        let result = xml_run_query(PATH, "determinism: G10");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG11() {
        let result = xml_run_query(PATH, "determinism: G11");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG12() {
        let result = xml_run_query(PATH, "determinism: G12");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG13() {
        let result = xml_run_query(PATH, "determinism: G13");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG14() {
        let result = xml_run_query(PATH, "determinism: G14");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(!convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG15() {
        let result = xml_run_query(PATH, "determinism: G15");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG16() {
        let result = xml_run_query(PATH, "determinism: G16");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(!convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG17() {
        let result = xml_run_query(PATH, "determinism: G17");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG22() {
        let result = xml_run_query(PATH, "determinism: G22");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }

    #[test]
    fn testG23() {
        // shouldn't be deterministic
        let result = xml_run_query(PATH, "determinism: G23");

        if let QueryResult::Determinism(is_deterministic) = result {
            assert!(!convert_to_bool(is_deterministic));
        } else {
            panic!("Not determinism query");
        }
    }
}
