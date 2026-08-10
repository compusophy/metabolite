//! FNV-1a-64. The world hash is the receipt: same seed, same history, one
//! integer. `metabolite run --seed S` twice must print the same number.

pub const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;

pub fn fold(mut h: u64, bytes: &[u8]) -> u64 {
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }
    h
}

pub fn fold_u64(h: u64, v: u64) -> u64 {
    fold(h, &v.to_le_bytes())
}

pub fn hash_str(s: &str) -> u64 {
    fold(FNV_OFFSET, s.as_bytes())
}
