use rand::Rng;

pub fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let charset: Vec<u8> = (b'a'..=b'z').chain(b'A'..=b'Z').collect();
    let string: String = (0..length)
        .map(|_| {
            let index = rng.gen_range(0..charset.len());
            charset[index] as char
        })
        .collect();
    string
}
