pub const MASK_6BIT: u64 = 0b111111;

pub fn extend_sign(value: u64, nbits: u32) -> u64 {
    let target = value as i64;
    let shamt = 64 - nbits;
    target.wrapping_shl(shamt).wrapping_shr(shamt) as u64
}
