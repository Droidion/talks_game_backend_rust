use uuid::Uuid;

/// Generates new UUID
pub fn generate_uuid() -> Uuid {
    return Uuid::new_v4();
}