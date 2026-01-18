pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

pub mod learning {
    tonic::include_proto!("core_linguistic.learning.v1");
}
