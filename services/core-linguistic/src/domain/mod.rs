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

use std::collections::HashMap;

pub struct MemoryObject {
    pub id: String,
    pub user_id: String,
    pub linguistic_unit: String,
    pub sino_vietnamese: Option<String>,
    pub stability: f64,
    pub difficulty: f64,
    pub decay_curve: f64,
    pub modality_weights: HashMap<i32, f64>,
    pub last_modality: Modality,
    pub last_review: Option<DateTime<Utc>>,
}

// Logic cho SRS Kernel
impl MemoryObject {
    pub fn update_memory_state(&mut self, quality: i32, now: DateTime<Utc>) {
        let q = quality as f64;
        
        // Cập nhật Difficulty (D)
        // D_new = D_old + (3 - q) * 0.1, giới hạn trong [0.1, 1.0]
        self.difficulty = (self.difficulty + (3.0 - q) * 0.1).clamp(0.1, 1.0);
        
        // Cập nhật Stability (S)
        // S_new = S_old * exp(K * q / D)
        // Đơn giản hóa cho MVP: S_new = S_old * (1.0 + 0.1 * q / self.difficulty)
        let modifier = 0.1 * q / self.difficulty;
        self.stability *= 1.0 + modifier;
        
        // Cập nhật thời gian review cuối
        self.last_review = Some(now);
    }

    pub fn get_retrievability(&self, now: DateTime<Utc>) -> f64 {
        if let Some(last) = self.last_review {
            let elapsed_days = (now - last).num_seconds() as f64 / 86400.0;
            // R = exp(ln(0.9) * t / S)
            (-0.10536 * elapsed_days / self.stability).exp()
        } else {
            0.0 // Chưa từng học
        }
    }
}
