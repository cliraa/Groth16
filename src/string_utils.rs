use crate::term::Term;
use crate::linear_expression::LinearExpression;

// ids of the coefficients with simple values in any cs.coeffs slice.
const COEFF_ID_ZERO: i32 = 0;
const COEFF_ID_ONE: i32 = 1;
const COEFF_ID_TWO: i32 = 2;

pub trait Resolver {
    fn coeff_to_string(&self, coeff_id: i32) -> String;
    fn variable_to_string(&self, variable_id: i32) -> String;
}

pub struct StringBuilder<'a> {
    builder: std::string::String,
    resolver: &'a dyn Resolver,
}

impl<'a> StringBuilder<'a> {
    pub fn new(resolver: &'a dyn Resolver) -> Self {
        Self {
            builder: std::string::String::new(),
            resolver,
        }
    }

    pub fn write_linear_expression(&mut self, l: &LinearExpression) {
        let mut first = true;
    
        for term in &l.0 {
            if !first {
                self.string().push_str(" + ");
            } else {
                first = false;
            }
    
            self.write_term(term);
        }
    }

    pub fn write_term(&mut self, t: &Term) {
        if t.coeff_id() == COEFF_ID_ZERO {
            self.builder.push('0');
            return;
        }
        let vs = self.resolver.variable_to_string(t.wire_id());
        if t.coeff_id() == COEFF_ID_ONE {
            self.builder.push_str(&vs);
            return;
        }
        self.builder.push_str(&self.resolver.coeff_to_string(t.coeff_id()));
        if t.wire_id() == 0 && vs == "1" {
            return;
        }
        self.builder.push('⋅');
        self.builder.push_str(&vs);
    }

    pub fn string(&self) -> String {
        self.builder.clone()
    }
}
