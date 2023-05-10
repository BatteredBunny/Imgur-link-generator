use std::str;

pub struct CodeGenerator {
    rng: fastrand::Rng,
    output: Vec<u8>,
    code_length: usize,
}

impl CodeGenerator {
    pub fn new(code_length: usize) -> Self {
        CodeGenerator {
            rng: fastrand::Rng::new(),
            output: format!("https://i.imgur.com/{:code_length$}.jpg", 0).into_bytes(),
            code_length,
        }
    }

    pub fn generate(&mut self) -> &str {
        for c in self.output.iter_mut().skip(20).take(self.code_length) {
            *c = self.rng.alphanumeric() as u8;
        }

        str::from_utf8(&self.output).unwrap()
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use test::Bencher;

    use super::*;

    #[bench]
    fn bench_generation(b: &mut Bencher) {
        let mut generator = CodeGenerator::new(7);
        b.iter(|| {
            let _ = generator.generate();
        });
    }
}
