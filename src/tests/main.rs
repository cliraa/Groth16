mod term;
mod linear_expression;
mod string_utils;

#[cfg(test)]
mod tests {

    use crate::string_utils::Resolver;
    use crate::string_utils::StringBuilder;

    // term.rs test:

    use crate::term::Term;
    use crate::term::COEFF_ID_ZERO;
    use crate::term::COEFF_ID_ONE;
    use crate::term::COEFF_ID_TWO;
    use crate::term::COEFF_ID_MINUS_ONE;
    use crate::term::Compressible;

    #[test]
    fn test_mark_constant() {
        // Create a test Term
        let mut term = Term {
            cid: COEFF_ID_ONE,
            vid: 123,
        };

        // Mark the term as constant
        term.mark_constant();

        // Check if the term is constant
        assert_eq!(term.is_constant(), true);
    }

    #[test]
    fn test_wire_id() {
        // Create a test Term
        let term = Term {
            cid: COEFF_ID_TWO,
            vid: 456,
        };

        // Get the wire ID
        let wire_id = term.wire_id();

        // Check if the wire ID matches the expected value
        assert_eq!(wire_id, 456);
    }

    #[test]
    fn test_coeff_id() {
        // Create a test Term
        let term = Term {
            cid: COEFF_ID_MINUS_ONE,
            vid: 789,
        };

        // Get the coefficient ID
        let coeff_id = term.coeff_id();

        // Check if the coefficient ID matches the expected value
        assert_eq!(coeff_id, -1);
    }

    // TODO: String function test

    #[test]
    fn test_compress() {
        // Create a test Term
        let term = Term {
            cid: COEFF_ID_ZERO,
            vid: COEFF_ID_ONE,
        };

        // Compress the term into a vector
        let mut compressed = Vec::new();
        term.compress(&mut compressed);

        // Check if the compressed vector contains the expected values
        assert_eq!(compressed, vec![1, 0, 1]);
    }

    // linear_expression.rs test:

    use crate::linear_expression::LinearExpression;

    #[test]
    fn test_linear_expression_clone() {
        let linear_exp = LinearExpression(vec![
            Term { cid: 1, vid: 2 },
            Term { cid: 3, vid: 4 },
        ]);

        let cloned_exp = linear_exp.clone();
        assert_eq!(cloned_exp.0, linear_exp.0);
    }

    // TODO: string function test

    #[test]
    fn test_linear_expression_compress() {
        let linear_exp = LinearExpression(vec![
            Term { cid: 1, vid: 2 },
            Term { cid: 3, vid: 4 },
        ]);

        let mut compressed: Vec<i32> = Vec::new();
        linear_exp.compress(&mut compressed);

        // Perform assertions on the compressed vector
        assert_eq!(compressed, vec![2, 1, 2, 3, 4]);
    }

    // string_utils.rs test:

    struct TestResolver;

    impl Resolver for TestResolver {
        fn coeff_to_string(&self, coeff_id: i32) -> String {
            match coeff_id {
                COEFF_ID_ZERO => String::from("0"),
                COEFF_ID_ONE => String::from("1"),
                COEFF_ID_TWO => String::from("2"),
                _ => panic!("Unknown coefficient id"),
            }
        }

        fn variable_to_string(&self, variable_id: i32) -> String {
            format!("x{}", variable_id)
        }
    }

    #[test]
    fn test_string_builder() {
        let resolver = TestResolver;
        let mut builder = StringBuilder::new(&resolver);

        let linear_expression = LinearExpression(vec![
            Term::new(COEFF_ID_ONE, 1),
            Term::new(COEFF_ID_TWO, 2),
        ]);

        builder.write_linear_expression(&linear_expression);

        assert_eq!(builder.string(), "x12⋅x2");

    }

}
