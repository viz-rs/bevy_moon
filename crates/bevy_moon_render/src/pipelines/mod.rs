pub mod primitive;
pub mod shadow;

pub mod quad {
    pub const INDEXES_COUNT: usize = 6;

    // pub const INDEXES: [u32; INDEXES_COUNT] = [0, 1, 2, 3, 4, 5];
    pub const INDEXES: [u32; INDEXES_COUNT] = [0, 2, 3, 0, 2, 1];

    /// 0..6
    pub const INDEXES_RANGE: std::ops::Range<u32> = 0..INDEXES_COUNT as u32;
}
