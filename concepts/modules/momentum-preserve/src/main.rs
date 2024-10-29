fn main() {
    let m1 = 4.0; // kg
    let m2 = 11.0;

    let v1_initial = 25.0; //m/s
    let v2_initial = -14.0;

    let m_total = m1 + m2;

    let momentum_initial = (m1 * v1_initial) + (m2 * v2_initial);
    let v_final = momentum_initial / m_total;
    println!("Final velocity: {:.2} m/s", &v_final);
}
