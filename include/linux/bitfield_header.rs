/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of linux/bitfield.h. */

#![allow(non_snake_case, non_camel_case_types, dead_code)]

#[inline(always)]
pub const fn __bf_shf(mask: u64) -> u32 { mask.trailing_zeros() }

#[inline(always)]
pub const fn field_multiplier(field: u64) -> u64 {
    let n = field | field.wrapping_sub(1);
    if (n | n.wrapping_add(1)) != u64::MAX { unsafe { __bad_mask() } }
    field & field.wrapping_neg()
}

#[inline(always)]
pub const fn field_mask(field: u64) -> u64 { field / field_multiplier(field) }

#[inline(always)]
pub const fn field_max(field: u64) -> u64 { field_mask(field) }

extern "C" {
    pub fn __field_overflow() -> !;
    pub fn __bad_mask() -> !;
}

#[inline(always)]
pub fn field_prep<T>(mask: T, val: T) -> T
where T: Copy + std::ops::BitAnd<Output=T> + std::ops::Shl<u32, Output=T>
{
    let shift = match std::mem::size_of::<T>() { 1..=4 => ((mask_to_u64(mask)) as u32).trailing_zeros(), _ => mask_to_u64(mask).trailing_zeros() };
    (val << shift) & mask
}

#[inline(always)]
pub fn field_get<T>(mask: T, reg: T) -> T
where T: Copy + std::ops::BitAnd<Output=T> + std::ops::Shr<u32, Output=T>
{
    let shift = mask_to_u64(mask).trailing_zeros();
    (reg & mask) >> shift
}

#[inline(always)]
fn mask_to_u64<T: Copy>(value: T) -> u64 {
    unsafe {
        match std::mem::size_of::<T>() {
            1 => *(std::ptr::from_ref(&value) as *const u8) as u64,
            2 => *(std::ptr::from_ref(&value) as *const u16) as u64,
            4 => *(std::ptr::from_ref(&value) as *const u32) as u64,
            8 => *(std::ptr::from_ref(&value) as *const u64),
            _ => 0,
        }
    }
}

#[inline(always)]
pub fn field_prep_const<T>(mask: T, val: T) -> T
where T: Copy + std::ops::BitAnd<Output=T> + std::ops::Shl<u32, Output=T>
{ field_prep(mask, val) }

#[inline(always)]
pub fn field_max_u64(mask: u64) -> u64 { field_mask(mask) }

#[inline(always)]
pub fn field_fit(mask: u64, val: u64) -> bool {
    (val << __bf_shf(mask)) & !mask == 0
}

#[inline(always)]
pub unsafe fn field_modify<T>(mask: T, reg: *mut T, val: T)
where T: Copy + std::ops::BitAnd<Output=T> + std::ops::BitOr<Output=T> +
    std::ops::Not<Output=T> + std::ops::Shl<u32, Output=T>
{
    *reg = (*reg & !mask) | field_prep(mask, val);
}

#[inline(always)]
pub fn u8_encode_bits(v: u8, field: u8) -> u8 { ((v & field_mask(field as u64) as u8).wrapping_mul(field_multiplier(field as u64) as u8)) }
#[inline(always)]
pub fn u8_replace_bits(old: u8, val: u8, field: u8) -> u8 { (old & !field) | u8_encode_bits(val, field) }
#[inline(always)]
pub unsafe fn u8p_replace_bits(p: *mut u8, val: u8, field: u8) { *p = u8_replace_bits(*p, val, field) }
#[inline(always)]
pub fn u8_get_bits(v: u8, field: u8) -> u8 { (v & field) / field_multiplier(field as u64) as u8 }

macro_rules! make_ops {
    ($size:ty, $name:ident) => {
        #[inline(always)] pub fn $name##_encode_bits(v: $size, field: $size) -> $size { ((v & field_mask(field as u64) as $size).wrapping_mul(field_multiplier(field as u64) as $size)) }
    }
}

/* The C header also emits endian-specific u16/u32/u64 operation families. */
#[inline(always)] pub fn u16_encode_bits(v:u16,f:u16)->u16{((v&field_mask(f as u64)as u16).wrapping_mul(field_multiplier(f as u64)as u16))}
#[inline(always)] pub fn u32_encode_bits(v:u32,f:u32)->u32{((v&field_mask(f as u64)as u32).wrapping_mul(field_multiplier(f as u64)as u32))}
#[inline(always)] pub fn u64_encode_bits(v:u64,f:u64)->u64{(v&field_mask(f).wrapping_mul(field_multiplier(f)))}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
