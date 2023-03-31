use serde::Serialize;

/// Type of Number.
#[derive(Serialize)]
pub enum ToN {}

/// Data Coding Scheme.
#[derive(Serialize)]
pub enum DCS {}

#[derive(Serialize)]
pub enum Priority {
    High,
    Normal,
    Low,
}

#[derive(Serialize)]
pub struct Date {}

#[derive(Serialize)]
pub struct Currency {}

#[derive(Serialize)]
pub struct KeyValue {}

pub type Long = u64;
pub type Integer = u32;
pub type Boolean = bool;
pub type String = std::string::String;
