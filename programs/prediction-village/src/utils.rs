pub fn generate_random_number(seed: u64) -> f32 {
    ((seed % 71) as f32 / 100.0) + 0.3
} 