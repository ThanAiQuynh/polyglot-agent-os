use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum Modality {
    Unspecified = 0,
    Read = 1,
    Listen = 2,
    Write = 3,
    Speak = 4,
}

impl From<i32> for Modality {
    fn from(value: i32) -> Self {
        match value {
            1 => Modality::Read,
            2 => Modality::Listen,
            3 => Modality::Write,
            4 => Modality::Speak,
            _ => Modality::Unspecified,
        }
    }
}

pub struct MemoryObject {
    pub id: String,
    pub user_id: String,
    pub linguistic_unit: String,
    pub han_viet: Option<String>,
    pub stability: f64,
    pub difficulty: f64,
    pub last_modality: Modality,
    pub last_review: Option<DateTime<Utc>>,
}

// Logic cho SRS Kernel có thể đặt ở đây
impl MemoryObject {
    pub fn update_stability(&mut self, quality: i32) {
        // Thuật toán SRS cơ bản: S_new = S_old * (1 + factor * quality)
        let factor = 0.1;
        self.stability *= 1.0 + (factor * quality as f64);
    }
}
