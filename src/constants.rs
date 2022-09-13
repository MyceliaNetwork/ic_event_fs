pub const WASM_PAGE_SIZE: u64 = 65536;

pub const BLOCK_SIZE: u64 = 512;
pub const IDX_BLOCK_SIZE: u64 = 32;

pub const MAGIC_NUMBER_IDX: u64 = 0;
pub const TOPIC_BLOCK_SIZE_IDX: u64 = 8;
pub const TOPIC_BLOCK_DATA_START_IDX: u64 = 16;

pub const TOPIC_BLOCK_MAX_SIZE: usize = 512;

pub const FREE_MEMORY_BLOCK_SIZE_IDX: u64 = TOPIC_BLOCK_DATA_START_IDX + TOPIC_BLOCK_MAX_SIZE as u64;
pub const FREE_MEMORY_BLOCK_START_IDX: u64 = TOPIC_BLOCK_DATA_START_IDX + TOPIC_BLOCK_MAX_SIZE as u64 + 8;
pub const FREE_MEMORY_BLOCK_SIZE: u64 = 120 * 1024 * 1024;

// Move 256MiB into memory
pub const IDX_ZONE_OFFSET: u64 = 256 * 1024 * 1024;
pub const IDX_ZONE_END: u64 = IDX_ZONE_OFFSET + (8192 * WASM_PAGE_SIZE);

pub const TOPIC_HEIGHT_IDX: u64 = 512 + 16;
