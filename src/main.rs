use lambdaworks_math::{
    polynomial::Polynomial,
    elliptic_curve::
        traits::{
            IsPairing,
            IsEllipticCurve},
    cyclic_group::IsGroup,
    field::{
        element::FieldElement,
        fields::u64_prime_field::U64PrimeField},
    elliptic_curve::
        short_weierstrass::curves::bls12_381::{
            pairing::BLS12381AtePairing,
            curve::BLS12381Curve,
            twist::BLS12381TwistCurve},
};

fn main() {

    const ORDER: u64 = 97;
    type F = U64PrimeField<ORDER>;
    type FE = FieldElement<F>;

    let Z = Polynomial::new(&[FE::new(1), FE::new(3), FE::new(35), FE::new(9), FE::new(27), FE::new(30)]);

    //QAP:

    let a_1 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(0), FE::new(5)]).unwrap();

    let a_2 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(1), FE::new(0), FE::new(1), FE::new(0)]).unwrap();

    let a_3 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(0), FE::new(0)]).unwrap();
        
    let a_4 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(1), FE::new(0), FE::new(0)]).unwrap();

    let a_5 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(1), FE::new(0)]).unwrap();

    let a_6 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(0), FE::new(1)]).unwrap();

    let b_1 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(1), FE::new(1)]).unwrap();

    let b_2 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(1), FE::new(1), FE::new(0), FE::new(0)]).unwrap();

    let b_3 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(0), FE::new(0)]).unwrap();

    let b_4 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(0), FE::new(0)]).unwrap();

    let b_5 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(0), FE::new(0)]).unwrap();

    let b_6 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(0), FE::new(0)]).unwrap();

    let c_1 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(0), FE::new(0)]).unwrap();

    let c_2 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(0), FE::new(0)]).unwrap();

    let c_3 = 
            Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(0), FE::new(1)]).unwrap();

    let c_4 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(1), FE::new(0), FE::new(0), FE::new(0)]).unwrap();

    let c_5 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(1), FE::new(0), FE::new(0)]).unwrap();

    let c_6 = 
        Polynomial::interpolate(&[FE::new(1), FE::new(2), FE::new(3), FE::new(4)], &[FE::new(0), FE::new(0), FE::new(1), FE::new(0)]).unwrap();

    // Generators:

    let g_1 = BLS12381Curve::generator();
    let g_2 = BLS12381TwistCurve::generator();

    // Random values:

    let t: u64 = 2;
    let alpha: u64 = 8;
    let beta: u64 = 4;
    let y: u64 = 9;
    let d: u64 = 10;

    // Proving key:

    let alphag_1 = g_1.operate_with_self(alpha); // 1
    let betag_1 = g_1.operate_with_self(beta); // 2
    let betag_2 = g_2.operate_with_self(beta); // 3
    let dg_1 = g_1.operate_with_self(d); // 4
    let dg_2 = g_2.operate_with_self(d); // 5

    let a_1_r = a_1.evaluate(&FE::new(t));
    let a_2_r = a_2.evaluate(&FE::new(t));
    let a_3_r = a_3.evaluate(&FE::new(t));
    let a_4_r = a_4.evaluate(&FE::new(t));
    let a_5_r = a_5.evaluate(&FE::new(t));
    let a_6_r = a_6.evaluate(&FE::new(t));

    let al = // 6
        [g_1.operate_with_self(a_1_r.representative()), 
        g_1.operate_with_self(a_2_r.representative()), 
        g_1.operate_with_self(a_3_r.representative()), 
        g_1.operate_with_self(a_4_r.representative()), 
        g_1.operate_with_self(a_5_r.representative()), 
        g_1.operate_with_self(a_6_r.representative())];

    let b_1_r = b_1.evaluate(&FE::new(t));
    let b_2_r = b_2.evaluate(&FE::new(t));
    let b_3_r = b_3.evaluate(&FE::new(t));
    let b_4_r = b_4.evaluate(&FE::new(t));
    let b_5_r = b_5.evaluate(&FE::new(t));
    let b_6_r = b_6.evaluate(&FE::new(t));
            
    let bl_1 = // 7
        [g_1.operate_with_self(b_1_r.representative()), 
        g_1.operate_with_self(b_2_r.representative()), 
        g_1.operate_with_self(b_3_r.representative()), 
        g_1.operate_with_self(b_4_r.representative()), 
        g_1.operate_with_self(b_5_r.representative()), 
        g_1.operate_with_self(b_6_r.representative())];

    let bl_2 = // 8
        [g_2.operate_with_self(b_1_r.representative()), 
        g_2.operate_with_self(b_2_r.representative()), 
        g_2.operate_with_self(b_3_r.representative()), 
        g_2.operate_with_self(b_4_r.representative()), 
        g_2.operate_with_self(b_5_r.representative()), 
        g_2.operate_with_self(b_6_r.representative())];
        
    // 9: TODO: Kpk+1(t)],[Kpk+2(t)],...,[Kpn(t)] TODO.
    // 10: TODO: [Z0(t)],[Z1(t)],...,[Zm−1(t)] TODO.        

    // Verifying key:

    let alphag_1_betag_2 = BLS12381AtePairing::compute_batch(&[(&alphag_1, &betag_2)]); // 1
    let yg_2 = g_2.operate_with_self(y); // 2
    let dg_2 = g_2.operate_with_self(d); // 3
    // 4: TODO: [Kv0(t)]1,[Kv1(t)]1,...,[Kvk(t)]

}
