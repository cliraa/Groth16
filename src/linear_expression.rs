use crate::term::Term;
pub struct LinearExpression(pub Vec<Term>);

impl LinearExpression {
    // Clone returns a copy of the underlying vector
    pub fn clone(&self) -> LinearExpression {
        let res = self.0.clone();
        LinearExpression(res)
    }

    /*

    TODO: Fix this function:
    
    pub fn string(&self, r: &Resolver) -> String {
        let mut sbb = StringBuilder::new(r);
        sbb.write_linear_expression(&self.0);
        sbb.string()
    }
    */

    pub fn compress(&self, to: &mut Vec<i32>) {
        to.push(self.0.len() as i32);
        for i in 0..self.0.len() {
            to.push(self.0[i].cid);
            to.push(self.0[i].vid);
        }
    }
}
