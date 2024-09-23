#[test]
fn methods_on_primitive_types() {
    let number = 10_i32;
    let larger_number = number.pow(2);
    dbg!(larger_number);
    let number = 10.0_f32;
    let larger_number = number.powf(2.5);
    dbg!(larger_number);

    let pi = std::f32::consts::PI;
    dbg!(pi);
    let e = std::f32::consts::E;
    dbg!(e);
    let pi = std::f64::consts::PI;
    dbg!(pi);
    let e = std::f64::consts::E;
    dbg!(e);
}
