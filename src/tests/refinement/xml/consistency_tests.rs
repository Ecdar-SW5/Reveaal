#[cfg(test)]
mod test {
    use crate::tests::refinement::Helper::xml_run_query;
    use crate::System::executable_query::QueryResult;
    use crate::System::local_consistency::ConsistencyResult;

    static PATH: &str = "samples/xml/ConsTests.xml";

    fn convert_to_bool(a: QueryResult) -> bool {
        if let QueryResult::Consistency(ConsistencyResult::Success) = a {
            return true;
        }
        return false;
    }

    #[test]
    fn testG1() {
        let result = xml_run_query(PATH, "consistency: G1");

        if let QueryResult::Consistency(_) = &result {
            assert!(convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }
    #[test]
    fn testG2() {
        let result = xml_run_query(PATH, "consistency: G2");

        if let QueryResult::Consistency(_) = &result {
            assert!(convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG3() {
        let result = xml_run_query(PATH, "consistency: G3");

        if let QueryResult::Consistency(_) = &result {
            assert!(!convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG4() {
        let result = xml_run_query(PATH, "consistency: G4");

        if let QueryResult::Consistency(_) = &result {
            assert!(!convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG5() {
        let result = xml_run_query(PATH, "consistency: G5");

        if let QueryResult::Consistency(_) = &result {
            assert!(!convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG6() {
        let result = xml_run_query(PATH, "consistency: G6");

        if let QueryResult::Consistency(_) = &result {
            assert!(convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG7() {
        let result = xml_run_query(PATH, "consistency: G7");

        if let QueryResult::Consistency(_) = &result {
            assert!(!convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG8() {
        let result = xml_run_query(PATH, "consistency: G8");

        if let QueryResult::Consistency(_) = &result {
            assert!(convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG9() {
        let result = xml_run_query(PATH, "consistency: G9");

        if let QueryResult::Consistency(_) = &result {
            assert!(!convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG10() {
        let result = xml_run_query(PATH, "consistency: G10");

        if let QueryResult::Consistency(_) = &result {
            assert!(!convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG11() {
        let result = xml_run_query(PATH, "consistency: G11");

        if let QueryResult::Consistency(_) = &result {
            assert!(!convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG12() {
        let result = xml_run_query(PATH, "consistency: G12");

        if let QueryResult::Consistency(_) = &result {
            assert!(!convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG13() {
        let result = xml_run_query(PATH, "consistency: G13");

        if let QueryResult::Consistency(_) = &result {
            assert!(convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG14() {
        let result = xml_run_query(PATH, "consistency: G14");

        if let QueryResult::Consistency(_) = &result {
            assert!(!convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG15() {
        let result = xml_run_query(PATH, "consistency: G15");

        if let QueryResult::Consistency(_) = &result {
            assert!(convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG16() {
        let result = xml_run_query(PATH, "consistency: G16");

        if let QueryResult::Consistency(_) = &result {
            assert!(!convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG17() {
        let result = xml_run_query(PATH, "consistency: G17");

        if let QueryResult::Consistency(_) = &result {
            assert!(convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG18() {
        let result = xml_run_query(PATH, "consistency: G18");

        if let QueryResult::Consistency(_) = &result {
            assert!(convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG19() {
        let result = xml_run_query(PATH, "consistency: G19");

        if let QueryResult::Consistency(_) = &result {
            assert!(!convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG20() {
        let result = xml_run_query(PATH, "consistency: G20");

        if let QueryResult::Consistency(_) = &result {
            assert!(convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }

    #[test]
    fn testG21() {
        let result = xml_run_query(PATH, "consistency: G21");

        if let QueryResult::Consistency(_) = &result {
            assert!(convert_to_bool(result));
        } else {
            panic!("Not consistency check");
        }
    }
}
