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

    let s: Polynomial<FieldElement<U64PrimeField<97>>> = Polynomial::new(&[FE::new(1), FE::new(3), FE::new(35), FE::new(9), FE::new(27), FE::new(30)]);

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

    
    let a_1_s = a_1.clone() * s.clone();
    let a_2_s = a_2.clone() * s.clone();
    let a_3_s = a_3.clone() * s.clone();
    let a_4_s = a_4.clone() * s.clone();
    let a_5_s = a_5.clone() * s.clone();
    let a_6_s = a_6.clone() * s.clone();

    let AS_poly = a_1_s.clone() + a_2_s.clone() + a_3_s.clone() + a_4_s.clone() + a_5_s.clone() + a_6_s.clone();

    let b_1_s = b_1.clone() * s.clone();
    let b_2_s = b_2.clone() * s.clone();
    let b_3_s = b_3.clone() * s.clone();
    let b_4_s = b_4.clone() * s.clone();
    let b_5_s = b_5.clone() * s.clone();
    let b_6_s = b_6.clone() * s.clone();

    let BS_poly = b_1_s.clone() + b_2_s.clone() + b_3_s.clone() + b_4_s.clone() + b_5_s.clone() + b_6_s.clone();

    let c_1_s = c_1.clone() * s.clone();
    let c_2_s = c_2.clone() * s.clone();
    let c_3_s = c_3.clone() * s.clone();
    let c_4_s = c_4.clone() * s.clone();
    let c_5_s = c_5.clone() * s.clone();
    let c_6_s = c_6.clone() * s.clone();

    let CS_poly = c_1_s.clone() + c_2_s.clone() + c_3_s.clone() + c_4_s.clone() + c_5_s.clone() + c_6_s.clone();

    let t_poly = (AS_poly.clone() * BS_poly.clone()) - CS_poly.clone();

    // ZD_poly is a precalculated value (It must be verified!)

    let ZD_poly = Polynomial::new(&[FE::new(24), FE::new(47), FE::new(35), FE::new(87), FE::new(1)]);

    let h_poly = t_poly.clone() / ZD_poly.clone();

    // Generators:

    let g_1 = BLS12381Curve::generator();
    let g_2 = BLS12381TwistCurve::generator();

    // Random values:

    let t = FE::new(2);
    let alpha = FE::new(8);
    let beta = FE::new(4);
    let y = FE::new(9);
    let d = FE::new(10);

    // Proving key:

    let alphag_1 = g_1.operate_with_self(alpha.representative()); // 1
    let betag_1 = g_1.operate_with_self(beta.representative()); // 2
    let betag_2 = g_2.operate_with_self(beta.representative()); // 3
    let dg_1 = g_1.operate_with_self(d.representative()); // 4
    let dg_2 = g_2.operate_with_self(d.representative()); // 5

    let al = // 6
        [g_1.operate_with_self(a_1.evaluate(&FE::new(t.representative())).representative()), 
        g_1.operate_with_self(a_2.evaluate(&FE::new(t.representative())).representative()), 
        g_1.operate_with_self(a_3.evaluate(&FE::new(t.representative())).representative()), 
        g_1.operate_with_self(a_4.evaluate(&FE::new(t.representative())).representative()), 
        g_1.operate_with_self(a_5.evaluate(&FE::new(t.representative())).representative()),
        g_1.operate_with_self(a_6.evaluate(&FE::new(t.representative())).representative())];
            
    let bl_1 = // 7
        [g_1.operate_with_self(b_1.evaluate(&FE::new(t.representative())).representative()),
        g_1.operate_with_self(b_2.evaluate(&FE::new(t.representative())).representative()),
        g_1.operate_with_self(b_3.evaluate(&FE::new(t.representative())).representative()),
        g_1.operate_with_self(b_4.evaluate(&FE::new(t.representative())).representative()),
        g_1.operate_with_self(b_5.evaluate(&FE::new(t.representative())).representative()),
        g_1.operate_with_self(b_6.evaluate(&FE::new(t.representative())).representative())];

    let bl_2 = // 8
        [g_2.operate_with_self(b_1.evaluate(&FE::new(t.representative())).representative()),
        g_2.operate_with_self(b_2.evaluate(&FE::new(t.representative())).representative()),
        g_2.operate_with_self(b_3.evaluate(&FE::new(t.representative())).representative()),
        g_2.operate_with_self(b_4.evaluate(&FE::new(t.representative())).representative()),
        g_2.operate_with_self(b_5.evaluate(&FE::new(t.representative())).representative()),
        g_2.operate_with_self(b_6.evaluate(&FE::new(t.representative())).representative())];
        
    // 9: TODO: Verify the amount of this elements.

    let inv_d = d.pow(95_u64);

    let Kp_1 = inv_d * (beta * a_1.evaluate(&FE::new(t.representative())) + alpha * b_1.evaluate(&FE::new(t.representative())) + c_1.evaluate(&FE::new(t.representative())));
    let Kp_2 = inv_d * (beta * a_2.evaluate(&FE::new(t.representative())) + alpha * b_2.evaluate(&FE::new(t.representative())) + c_2.evaluate(&FE::new(t.representative())));
    let Kp_3 = inv_d * (beta * a_3.evaluate(&FE::new(t.representative())) + alpha * b_3.evaluate(&FE::new(t.representative())) + c_3.evaluate(&FE::new(t.representative())));
    let Kp_4 = inv_d * (beta * a_4.evaluate(&FE::new(t.representative())) + alpha * b_4.evaluate(&FE::new(t.representative())) + c_4.evaluate(&FE::new(t.representative())));
    let Kp_5 = inv_d * (beta * a_5.evaluate(&FE::new(t.representative())) + alpha * b_5.evaluate(&FE::new(t.representative())) + c_5.evaluate(&FE::new(t.representative())));
    let Kp_6 = inv_d * (beta * a_6.evaluate(&FE::new(t.representative())) + alpha * b_6.evaluate(&FE::new(t.representative())) + c_6.evaluate(&FE::new(t.representative())));
    
    let Kp_f = [Kp_1, Kp_2, Kp_3, Kp_4, Kp_5, Kp_6];

    // 10: TODO: Verify the amount of this elements.

    let Z_0 = inv_d * t.pow(0_u64) * ZD_poly.evaluate(&FE::new(t.representative()));
    let Z_1 = inv_d * t.pow(1_u64) * ZD_poly.evaluate(&FE::new(t.representative()));
    let Z_2 = inv_d * t.pow(2_u64) * ZD_poly.evaluate(&FE::new(t.representative()));
    //let Z_3 = inv_d * t.pow(3_u64) * ZD_poly.evaluate(&FE::new(t.representative()));
    //let Z_4 = inv_d * t.pow(4_u64) * ZD_poly.evaluate(&FE::new(t.representative()));
    //let Z_5 = inv_d * t.pow(5_u64) * ZD_poly.evaluate(&FE::new(t.representative()));
    //let Z_6 = inv_d * t.pow(6_u64) * ZD_poly.evaluate(&FE::new(t.representative()));
    //let Z_f = [Z_0, Z_1, Z_2, Z_3, Z_4, Z_5, Z_6];

    let Z_f = [Z_0, Z_1, Z_2]; // 10

    // Verifying key:

    let alphag_1_betag_2 = BLS12381AtePairing::compute_batch(&[(&alphag_1, &betag_2)]); // 1
    let yg_2 = g_2.operate_with_self(y.representative()); // 2
    let dg_2 = g_2.operate_with_self(d.representative()); // 3

    // 4: TODO: Verify the amount of this elements.

    let inv_y = y.pow(95_u64);

    let Ku_1 = inv_y * (beta * a_1.evaluate(&FE::new(t.representative())) + alpha * b_1.evaluate(&FE::new(t.representative())) + c_1.evaluate(&FE::new(t.representative())));
    let Ku_2 = inv_y * (beta * a_2.evaluate(&FE::new(t.representative())) + alpha * b_2.evaluate(&FE::new(t.representative())) + c_2.evaluate(&FE::new(t.representative())));
    let Ku_3 = inv_y * (beta * a_3.evaluate(&FE::new(t.representative())) + alpha * b_3.evaluate(&FE::new(t.representative())) + c_3.evaluate(&FE::new(t.representative())));
    let Ku_4 = inv_y * (beta * a_4.evaluate(&FE::new(t.representative())) + alpha * b_4.evaluate(&FE::new(t.representative())) + c_4.evaluate(&FE::new(t.representative())));
    let Ku_5 = inv_y * (beta * a_5.evaluate(&FE::new(t.representative())) + alpha * b_5.evaluate(&FE::new(t.representative())) + c_5.evaluate(&FE::new(t.representative())));
    let Ku_6 = inv_y * (beta * a_6.evaluate(&FE::new(t.representative())) + alpha * b_6.evaluate(&FE::new(t.representative())) + c_6.evaluate(&FE::new(t.representative())));
    
    let Ku_f = [Ku_1, Ku_2, Ku_3, Ku_4, Ku_5, Ku_6]; // 4

    // Proof:

    let r_ran = FE::new(2);
    let s_ran = FE::new(8);

    // pi_1_1:

    let sum_1 = 
        (g_1.operate_with_self(a_1.evaluate(&FE::new(t.representative())).representative())).operate_with_self(1_u64);

    let sum_2 = 
        (g_1.operate_with_self(a_2.evaluate(&FE::new(t.representative())).representative())).operate_with_self(3_u64);

    let sum_3 = 
        (g_1.operate_with_self(a_3.evaluate(&FE::new(t.representative())).representative())).operate_with_self(35_u64);

    let sum_4 = 
        (g_1.operate_with_self(a_4.evaluate(&FE::new(t.representative())).representative())).operate_with_self(9_u64);

    let sum_5 = 
        (g_1.operate_with_self(a_5.evaluate(&FE::new(t.representative())).representative())).operate_with_self(27_u64);

    let sum_6 = 
        (g_1.operate_with_self(a_6.evaluate(&FE::new(t.representative())).representative())).operate_with_self(30_u64);

    
    let sum_d_1 = ((((sum_1.operate_with(&sum_2)).operate_with(&sum_3)).operate_with(&sum_4)).operate_with(&sum_5)).operate_with(&sum_6);

    let r_dg_1 = dg_1.operate_with_self(r_ran.representative());

    let pi_1_1 = (alphag_1.operate_with(&sum_d_1)).operate_with(&r_dg_1);

    // pi_2_2:

    let sum_1_2 = 
        (g_2.operate_with_self(b_1.evaluate(&FE::new(t.representative())).representative())).operate_with_self(1_u64);

    let sum_2_2 = 
        (g_2.operate_with_self(b_2.evaluate(&FE::new(t.representative())).representative())).operate_with_self(3_u64);

    let sum_3_2 = 
        (g_2.operate_with_self(b_3.evaluate(&FE::new(t.representative())).representative())).operate_with_self(35_u64);

    let sum_4_2 = 
        (g_2.operate_with_self(b_4.evaluate(&FE::new(t.representative())).representative())).operate_with_self(9_u64);

    let sum_5_2 = 
        (g_2.operate_with_self(b_5.evaluate(&FE::new(t.representative())).representative())).operate_with_self(27_u64);

    let sum_6_2 = 
        (g_2.operate_with_self(b_6.evaluate(&FE::new(t.representative())).representative())).operate_with_self(30_u64);

    let sum_d_2 = ((((sum_1_2.operate_with(&sum_2_2)).operate_with(&sum_3_2)).operate_with(&sum_4_2)).operate_with(&sum_5_2)).operate_with(&sum_6_2);

    let s_dg_2 = dg_2.operate_with_self(s_ran.representative());

    let pi_2_2 = (betag_2.operate_with(&sum_d_2)).operate_with(&s_dg_2);

    // pi_2_1:

    let sum_2_1_1 = 
        (g_1.operate_with_self(b_1.evaluate(&FE::new(t.representative())).representative())).operate_with_self(1_u64);

    let sum_2_1_2 = 
        (g_1.operate_with_self(b_2.evaluate(&FE::new(t.representative())).representative())).operate_with_self(3_u64);

    let sum_2_2_3 = 
        (g_1.operate_with_self(b_3.evaluate(&FE::new(t.representative())).representative())).operate_with_self(35_u64);

    let sum_2_2_4 = 
        (g_1.operate_with_self(b_4.evaluate(&FE::new(t.representative())).representative())).operate_with_self(9_u64);

    let sum_2_2_5 = 
        (g_1.operate_with_self(b_5.evaluate(&FE::new(t.representative())).representative())).operate_with_self(27_u64);

    let sum_2_2_6 = 
        (g_1.operate_with_self(b_6.evaluate(&FE::new(t.representative())).representative())).operate_with_self(30_u64);

    let sum_d_2_1 = ((((sum_2_1_1.operate_with(&sum_2_1_2)).operate_with(&sum_2_2_3)).operate_with(&sum_2_2_4)).operate_with(&sum_2_2_5)).operate_with(&sum_2_2_6);

    let s_dg_1 = dg_1.operate_with_self(s_ran.representative());

    let pi_2_1 = (betag_1.operate_with(&sum_d_2_1)).operate_with(&s_dg_1);

    // Verification:

    let P_1 = BLS12381AtePairing::compute_batch(&[(&pi_1_1, &pi_2_2)]);

    // TODO: P_2
}
