use std::i32;

// ids of the coefficients with simple values in any cs.coeffs slice.
pub const COEFF_ID_ZERO: i32 = 0;
pub const COEFF_ID_ONE: i32 = 1;
pub const COEFF_ID_TWO: i32 = 2;
pub const COEFF_ID_MINUS_ONE: i32 = -1;
pub const COEFF_ID_MINUS_TWO: i32 = -2;

// Term represents a coeff * variable in a constraint system
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Term {
    pub cid: i32,
    pub vid: i32,
}

impl Term {
    pub fn mark_constant(&mut self) {
        self.vid = i32::MAX;
    }
    
    pub fn is_constant(&self) -> bool {
        self.vid == i32::MAX
    }

    pub fn wire_id(&self) -> i32 {
        self.vid as i32
    }

    pub fn coeff_id(&self) -> i32 {
        self.cid as i32
    }

    pub fn new(cid: i32, vid: i32) -> Term {
        Term { cid, vid }
    }

    /*
    TODO: Fix this function:

    fn string(&self, r: &dyn Resolver) -> String {
        let mut sbb = StringBuilder::new(r);
        sbb.write_term(*self);
        sbb.string()
    }
    */
}

pub trait Compressible {
    fn compress(&self, to: &mut Vec<i32>);
}

impl Compressible for Term {
    fn compress(&self, to: &mut Vec<i32>) {
        to.push(1);
        to.push(self.cid);
        to.push(self.vid);
    }
}
