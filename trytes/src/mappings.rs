#![allow(dead_code)]

use constants::*;

/// Hardcoded mappings from `u8` to 5 `Trit`s
pub static BYTE_TO_TRITS_MAPPINGS: [[Trit; TRITS_PER_BYTE]; HASH_LENGTH] = [
    [0, 0, 0, 0, 0],
    [1, 0, 0, 0, 0],
    [-1, 1, 0, 0, 0],
    [0, 1, 0, 0, 0],
    [1, 1, 0, 0, 0],
    [-1, -1, 1, 0, 0],
    [0, -1, 1, 0, 0],
    [1, -1, 1, 0, 0],
    [-1, 0, 1, 0, 0],
    [0, 0, 1, 0, 0],
    [1, 0, 1, 0, 0],
    [-1, 1, 1, 0, 0],
    [0, 1, 1, 0, 0],
    [1, 1, 1, 0, 0],
    [-1, -1, -1, 1, 0],
    [0, -1, -1, 1, 0],
    [1, -1, -1, 1, 0],
    [-1, 0, -1, 1, 0],
    [0, 0, -1, 1, 0],
    [1, 0, -1, 1, 0],
    [-1, 1, -1, 1, 0],
    [0, 1, -1, 1, 0],
    [1, 1, -1, 1, 0],
    [-1, -1, 0, 1, 0],
    [0, -1, 0, 1, 0],
    [1, -1, 0, 1, 0],
    [-1, 0, 0, 1, 0],
    [0, 0, 0, 1, 0],
    [1, 0, 0, 1, 0],
    [-1, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [1, 1, 0, 1, 0],
    [-1, -1, 1, 1, 0],
    [0, -1, 1, 1, 0],
    [1, -1, 1, 1, 0],
    [-1, 0, 1, 1, 0],
    [0, 0, 1, 1, 0],
    [1, 0, 1, 1, 0],
    [-1, 1, 1, 1, 0],
    [0, 1, 1, 1, 0],
    [1, 1, 1, 1, 0],
    [-1, -1, -1, -1, 1],
    [0, -1, -1, -1, 1],
    [1, -1, -1, -1, 1],
    [-1, 0, -1, -1, 1],
    [0, 0, -1, -1, 1],
    [1, 0, -1, -1, 1],
    [-1, 1, -1, -1, 1],
    [0, 1, -1, -1, 1],
    [1, 1, -1, -1, 1],
    [-1, -1, 0, -1, 1],
    [0, -1, 0, -1, 1],
    [1, -1, 0, -1, 1],
    [-1, 0, 0, -1, 1],
    [0, 0, 0, -1, 1],
    [1, 0, 0, -1, 1],
    [-1, 1, 0, -1, 1],
    [0, 1, 0, -1, 1],
    [1, 1, 0, -1, 1],
    [-1, -1, 1, -1, 1],
    [0, -1, 1, -1, 1],
    [1, -1, 1, -1, 1],
    [-1, 0, 1, -1, 1],
    [0, 0, 1, -1, 1],
    [1, 0, 1, -1, 1],
    [-1, 1, 1, -1, 1],
    [0, 1, 1, -1, 1],
    [1, 1, 1, -1, 1],
    [-1, -1, -1, 0, 1],
    [0, -1, -1, 0, 1],
    [1, -1, -1, 0, 1],
    [-1, 0, -1, 0, 1],
    [0, 0, -1, 0, 1],
    [1, 0, -1, 0, 1],
    [-1, 1, -1, 0, 1],
    [0, 1, -1, 0, 1],
    [1, 1, -1, 0, 1],
    [-1, -1, 0, 0, 1],
    [0, -1, 0, 0, 1],
    [1, -1, 0, 0, 1],
    [-1, 0, 0, 0, 1],
    [0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [-1, 1, 0, 0, 1],
    [0, 1, 0, 0, 1],
    [1, 1, 0, 0, 1],
    [-1, -1, 1, 0, 1],
    [0, -1, 1, 0, 1],
    [1, -1, 1, 0, 1],
    [-1, 0, 1, 0, 1],
    [0, 0, 1, 0, 1],
    [1, 0, 1, 0, 1],
    [-1, 1, 1, 0, 1],
    [0, 1, 1, 0, 1],
    [1, 1, 1, 0, 1],
    [-1, -1, -1, 1, 1],
    [0, -1, -1, 1, 1],
    [1, -1, -1, 1, 1],
    [-1, 0, -1, 1, 1],
    [0, 0, -1, 1, 1],
    [1, 0, -1, 1, 1],
    [-1, 1, -1, 1, 1],
    [0, 1, -1, 1, 1],
    [1, 1, -1, 1, 1],
    [-1, -1, 0, 1, 1],
    [0, -1, 0, 1, 1],
    [1, -1, 0, 1, 1],
    [-1, 0, 0, 1, 1],
    [0, 0, 0, 1, 1],
    [1, 0, 0, 1, 1],
    [-1, 1, 0, 1, 1],
    [0, 1, 0, 1, 1],
    [1, 1, 0, 1, 1],
    [-1, -1, 1, 1, 1],
    [0, -1, 1, 1, 1],
    [1, -1, 1, 1, 1],
    [-1, 0, 1, 1, 1],
    [0, 0, 1, 1, 1],
    [1, 0, 1, 1, 1],
    [-1, 1, 1, 1, 1],
    [0, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
    [-1, -1, -1, -1, -1],
    [0, -1, -1, -1, -1],
    [1, -1, -1, -1, -1],
    [-1, 0, -1, -1, -1],
    [0, 0, -1, -1, -1],
    [1, 0, -1, -1, -1],
    [-1, 1, -1, -1, -1],
    [0, 1, -1, -1, -1],
    [1, 1, -1, -1, -1],
    [-1, -1, 0, -1, -1],
    [0, -1, 0, -1, -1],
    [1, -1, 0, -1, -1],
    [-1, 0, 0, -1, -1],
    [0, 0, 0, -1, -1],
    [1, 0, 0, -1, -1],
    [-1, 1, 0, -1, -1],
    [0, 1, 0, -1, -1],
    [1, 1, 0, -1, -1],
    [-1, -1, 1, -1, -1],
    [0, -1, 1, -1, -1],
    [1, -1, 1, -1, -1],
    [-1, 0, 1, -1, -1],
    [0, 0, 1, -1, -1],
    [1, 0, 1, -1, -1],
    [-1, 1, 1, -1, -1],
    [0, 1, 1, -1, -1],
    [1, 1, 1, -1, -1],
    [-1, -1, -1, 0, -1],
    [0, -1, -1, 0, -1],
    [1, -1, -1, 0, -1],
    [-1, 0, -1, 0, -1],
    [0, 0, -1, 0, -1],
    [1, 0, -1, 0, -1],
    [-1, 1, -1, 0, -1],
    [0, 1, -1, 0, -1],
    [1, 1, -1, 0, -1],
    [-1, -1, 0, 0, -1],
    [0, -1, 0, 0, -1],
    [1, -1, 0, 0, -1],
    [-1, 0, 0, 0, -1],
    [0, 0, 0, 0, -1],
    [1, 0, 0, 0, -1],
    [-1, 1, 0, 0, -1],
    [0, 1, 0, 0, -1],
    [1, 1, 0, 0, -1],
    [-1, -1, 1, 0, -1],
    [0, -1, 1, 0, -1],
    [1, -1, 1, 0, -1],
    [-1, 0, 1, 0, -1],
    [0, 0, 1, 0, -1],
    [1, 0, 1, 0, -1],
    [-1, 1, 1, 0, -1],
    [0, 1, 1, 0, -1],
    [1, 1, 1, 0, -1],
    [-1, -1, -1, 1, -1],
    [0, -1, -1, 1, -1],
    [1, -1, -1, 1, -1],
    [-1, 0, -1, 1, -1],
    [0, 0, -1, 1, -1],
    [1, 0, -1, 1, -1],
    [-1, 1, -1, 1, -1],
    [0, 1, -1, 1, -1],
    [1, 1, -1, 1, -1],
    [-1, -1, 0, 1, -1],
    [0, -1, 0, 1, -1],
    [1, -1, 0, 1, -1],
    [-1, 0, 0, 1, -1],
    [0, 0, 0, 1, -1],
    [1, 0, 0, 1, -1],
    [-1, 1, 0, 1, -1],
    [0, 1, 0, 1, -1],
    [1, 1, 0, 1, -1],
    [-1, -1, 1, 1, -1],
    [0, -1, 1, 1, -1],
    [1, -1, 1, 1, -1],
    [-1, 0, 1, 1, -1],
    [0, 0, 1, 1, -1],
    [1, 0, 1, 1, -1],
    [-1, 1, 1, 1, -1],
    [0, 1, 1, 1, -1],
    [1, 1, 1, 1, -1],
    [-1, -1, -1, -1, 0],
    [0, -1, -1, -1, 0],
    [1, -1, -1, -1, 0],
    [-1, 0, -1, -1, 0],
    [0, 0, -1, -1, 0],
    [1, 0, -1, -1, 0],
    [-1, 1, -1, -1, 0],
    [0, 1, -1, -1, 0],
    [1, 1, -1, -1, 0],
    [-1, -1, 0, -1, 0],
    [0, -1, 0, -1, 0],
    [1, -1, 0, -1, 0],
    [-1, 0, 0, -1, 0],
    [0, 0, 0, -1, 0],
    [1, 0, 0, -1, 0],
    [-1, 1, 0, -1, 0],
    [0, 1, 0, -1, 0],
    [1, 1, 0, -1, 0],
    [-1, -1, 1, -1, 0],
    [0, -1, 1, -1, 0],
    [1, -1, 1, -1, 0],
    [-1, 0, 1, -1, 0],
    [0, 0, 1, -1, 0],
    [1, 0, 1, -1, 0],
    [-1, 1, 1, -1, 0],
    [0, 1, 1, -1, 0],
    [1, 1, 1, -1, 0],
    [-1, -1, -1, 0, 0],
    [0, -1, -1, 0, 0],
    [1, -1, -1, 0, 0],
    [-1, 0, -1, 0, 0],
    [0, 0, -1, 0, 0],
    [1, 0, -1, 0, 0],
    [-1, 1, -1, 0, 0],
    [0, 1, -1, 0, 0],
    [1, 1, -1, 0, 0],
    [-1, -1, 0, 0, 0],
    [0, -1, 0, 0, 0],
    [1, -1, 0, 0, 0],
    [-1, 0, 0, 0, 0],
];

/// Hardcoded tryte to `Trit` mappings
pub static TRYTE_TO_TRITS_MAPPINGS: [[Trit; TRITS_PER_TRYTE]; TRYTE_SPACE] = [
    [0, 0, 0],
    [1, 0, 0],
    [-1, 1, 0],
    [0, 1, 0],
    [1, 1, 0],
    [-1, -1, 1],
    [0, -1, 1],
    [1, -1, 1],
    [-1, 0, 1],
    [0, 0, 1],
    [1, 0, 1],
    [-1, 1, 1],
    [0, 1, 1],
    [1, 1, 1],
    [-1, -1, -1],
    [0, -1, -1],
    [1, -1, -1],
    [-1, 0, -1],
    [0, 0, -1],
    [1, 0, -1],
    [-1, 1, -1],
    [0, 1, -1],
    [1, 1, -1],
    [-1, -1, 0],
    [0, -1, 0],
    [1, -1, 0],
    [-1, 0, 0],
];