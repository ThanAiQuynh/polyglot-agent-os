pub mod tokenizer;
use crate::domain::MemoryObject;

// Trait cho Persistence Layer (Infrastructure)
pub trait MemoryRepository: Send + Sync {
    fn save(&self, object: &MemoryObject) -> Result<(), String>;
    fn find_by_user(&self, user_id: &str, limit: i32) -> Vec<MemoryObject>;
}

pub struct LearningService {
    // Repository sẽ được inject vào đây
    // repo: Arc<dyn MemoryRepository>,
}

impl LearningService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_review(&self, object_id: &str, quality: i32) -> Result<(), String> {
        println!("Processing review for {} with quality {}", object_id, quality);
        // Logic điều phối: Lấy từ repo -> Cập nhật domain -> Lưu lại
        Ok(())
    }
}
