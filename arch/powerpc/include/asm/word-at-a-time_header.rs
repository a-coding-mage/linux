/*
 * Word-at-a-time interfaces for PowerPC.
 *
 * C dependencies: linux/bitops.h, linux/wordpart.h, asm/asm-compat.h,
 * and asm/extable.h provide REPEAT_BYTE, PPC_* assembly names, and EX_TABLE.
 */

#[cfg(target_endian = "big")]
#[repr(C)]
pub struct word_at_a_time {
    pub high_bits: ::core::ffi::c_ulong,
    pub low_bits: ::core::ffi::c_ulong,
}

#[cfg(target_endian = "big")]
pub const WORD_AT_A_TIME_CONSTANTS: (u64, u64) = (
    (0xfefefefefefefefe_u64).wrapping_add(1),
    0x7f7f7f7f7f7f7f7f_u64,
);

#[cfg(target_endian = "big")]
#[inline]
pub unsafe fn prep_zero_mask(
    val: ::core::ffi::c_ulong,
    rhs: ::core::ffi::c_ulong,
    c: *const word_at_a_time,
) -> isize {
    let mask = (val & (*c).low_bits).wrapping_add((*c).low_bits);
    (!(mask | rhs)) as isize
}

#[cfg(target_endian = "big")]
#[inline]
pub const fn create_zero_mask(mask: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong { mask }

#[cfg(target_endian = "big")]
#[inline]
pub fn find_zero(mask: ::core::ffi::c_ulong) -> isize {
    mask.leading_zeros() as isize >> 3
}

#[cfg(target_endian = "big")]
#[inline]
pub unsafe fn has_zero(
    val: ::core::ffi::c_ulong,
    data: *mut ::core::ffi::c_ulong,
    c: *const word_at_a_time,
) -> ::core::ffi::c_ulong {
    let rhs = val | (*c).low_bits;
    *data = rhs;
    (val.wrapping_add((*c).high_bits)) & !rhs
}

#[cfg(target_endian = "big")]
#[inline]
pub fn zero_bytemask(mask: ::core::ffi::c_ulong) -> ::core::ffi::c_ulong {
    !1usize.wrapping_shl(mask.trailing_zeros()) as ::core::ffi::c_ulong
}

/* The following little-endian definitions correspond to the C build-time branches. */
#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
#[repr(C)]
pub struct word_at_a_time {}

#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
pub const WORD_AT_A_TIME_CONSTANTS: () = ();

#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
#[inline]
pub unsafe fn has_zero(a: usize, bits: *mut usize, _c: *const word_at_a_time) -> usize {
    let ret: usize;
    let zero: usize = 0;
    ::core::arch::asm!("cmpb {ret}, {a}, {zero}", ret = out(reg) ret, a = in(reg) a, zero = in(reg) zero);
    *bits = ret;
    ret
}

#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
#[inline]
pub unsafe fn prep_zero_mask(_a: usize, bits: usize, _c: *const word_at_a_time) -> usize { bits }

#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
#[inline]
pub fn create_zero_mask(bits: usize) -> usize {
    let mut leading_zero_bits: usize;
    let trailing_zero_bit_mask: usize;
    unsafe {
        ::core::arch::asm!(
            "addi {tmp}, {bits}, -1\n\tandc {tmp}, {tmp}, {bits}\n\tpopcntd {out}, {tmp}",
            out = lateout(reg) leading_zero_bits, tmp = lateout(reg) trailing_zero_bit_mask,
            bits = in(reg) bits
        );
    }
    leading_zero_bits
}

#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
#[inline]
pub const fn find_zero(mask: usize) -> usize { mask >> 3 }

#[cfg(all(target_endian = "little", target_pointer_width = "64"))]
#[inline]
pub fn zero_bytemask(mask: usize) -> usize { (1usize << mask).wrapping_sub(1) }

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
#[repr(C)]
pub struct word_at_a_time {
    pub one_bits: u32,
    pub high_bits: u32,
}

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
pub const WORD_AT_A_TIME_CONSTANTS: (u32, u32) = (0x01010101, 0x80808080);

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
#[inline]
pub fn count_masked_bytes(mask: i32) -> i32 {
    let a = (0x0ff0001_i32.wrapping_add(mask)) >> 23;
    a & mask
}

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
#[inline]
pub fn create_zero_mask(mut bits: u32) -> u32 {
    bits = bits.wrapping_sub(1) & !bits;
    bits >> 7
}

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
#[inline]
pub fn find_zero(mask: u32) -> i32 { count_masked_bytes(mask as i32) }

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
#[inline]
pub unsafe fn has_zero(a: u32, bits: *mut u32, c: *const word_at_a_time) -> u32 {
    let mask = (a.wrapping_sub((*c).one_bits) & !a) & (*c).high_bits;
    *bits = mask;
    mask
}

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
#[inline]
pub unsafe fn prep_zero_mask(_a: u32, bits: u32, _c: *const word_at_a_time) -> u32 { bits }

#[cfg(all(target_endian = "little", target_pointer_width = "32"))]
#[inline]
pub const fn zero_bytemask(mask: u32) -> u32 { mask }

/* load_unaligned_zeropad uses PowerPC exception-table fixup assembly in C. */
#[inline]
pub unsafe fn load_unaligned_zeropad(addr: *const ::core::ffi::c_void) -> usize {
    let mut ret: usize;
    ::core::ptr::copy_nonoverlapping(addr as *const u8, &mut ret as *mut usize as *mut u8, core::mem::size_of::<usize>());
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
