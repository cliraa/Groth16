mod term;
mod linear_expression;

#[cfg(test)]
mod tests {

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
}
