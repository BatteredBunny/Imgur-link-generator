use rand::prelude::IteratorRandom;
use rand::rngs::ThreadRng;

pub struct CodeGenerator {
    rng: ThreadRng,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            rng: rand::thread_rng(),
        }
    }

    pub fn generate(&mut self, code_length: usize) -> String {
        let char_pool = (b'A'..=b'Z').chain(b'a'..=b'z').chain(b'0'..=b'9');
        let random_bytes = char_pool.choose_multiple(&mut self.rng, code_length);
        let code = String::from_utf8(random_bytes).unwrap();

        format!("https://i.imgur.com/{}.jpg", code)
    }
}