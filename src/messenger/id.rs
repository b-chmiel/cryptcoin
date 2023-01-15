use rand::Rng;

pub struct Id {
    value: u32,
}

impl Id {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        Self {
            value: rng.gen_range(0..u32::MAX),
        }
    }

    pub fn to_string(&self) -> String {
        self.value.to_string()
    }
}
