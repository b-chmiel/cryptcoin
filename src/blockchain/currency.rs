use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Currency {
    pub amount: f32,
}

impl std::ops::AddAssign for Currency {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            amount: self.amount + other.amount,
        };
    }
}

impl std::ops::SubAssign for Currency {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            amount: self.amount - other.amount,
        };
    }
}
