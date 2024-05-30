use rand::Rng;
pub fn generate_random_16_digit_code() -> String {
    let mut rng = rand::thread_rng();
    let mut code = String::with_capacity(16);

    for _ in 0..16 {
        let digit: u8 = rng.gen_range(0..10);
        code.push_str(&digit.to_string());
    }

    code
}
