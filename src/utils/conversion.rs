use {
    primitive_types::{U256},
    num::{BigRational, BigInt, FromPrimitive},
    bigdecimal::num_bigint::Sign,
};

pub fn u256_to_big_rational(value: &U256) -> BigRational {
    let mut value_bytes: Vec<u8> = Vec::with_capacity(32);
    value_bytes.resize(32, 0);
    value.to_little_endian(value_bytes.as_mut_slice());
    BigRational::new(
        BigInt::from_bytes_le(Sign::Plus, &value_bytes),
        BigInt::from_u32(1).unwrap()
    )
}
