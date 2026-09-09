// SPDX-License-Identifier: Zlib
// C dependencies: dfltcc.h, linux/kmsan-checks.h, and linux/zutil.h.

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum dfltcc_cc {
    DFLTCC_CC_OK = 0,
    DFLTCC_CC_OP1_TOO_SHORT = 1,
    DFLTCC_CC_OP2_TOO_SHORT = 2,
    DFLTCC_CC_AGAIN = 3,
}
pub const DFLTCC_CC_OP2_CORRUPT: dfltcc_cc = dfltcc_cc::DFLTCC_CC_OP2_TOO_SHORT;

pub const DFLTCC_QAF: i32 = 0;
pub const DFLTCC_GDHT: i32 = 1;
pub const DFLTCC_CMPR: i32 = 2;
pub const DFLTCC_XPND: i32 = 4;
pub const HBT_CIRCULAR: i32 = 1 << 7;
pub const DFLTCC_FN_MASK: i32 = (1 << 7) - 1;
pub const HB_BITS: u32 = 15;
pub const HB_SIZE: usize = 1usize << HB_BITS;

// External declarations supplied by the translated dependency headers.
extern "C" {
    fn kmsan_unpoison_memory(address: *mut core::ffi::c_void, size: usize);
    fn oesc_msg(buf: *mut core::ffi::c_char, oesc: i32) -> *mut core::ffi::c_char;
}

// The following parameter layouts and Byte/uInt/uLong types are supplied by dfltcc.h/zutil.h.
#[allow(improper_ctypes)]
extern "C" {
    type dfltcc_qaf_param;
    type dfltcc_param_v0;
}

#[inline]
pub unsafe fn dfltcc(
    fn_: i32,
    param: *mut core::ffi::c_void,
    op1: *mut *mut u8,
    len1: *mut usize,
    op2: *mut *const u8,
    len2: *mut usize,
    hist: *mut core::ffi::c_void,
) -> dfltcc_cc {
    let mut t2 = if !op1.is_null() { *op1 } else { core::ptr::null_mut() };
    let orig_t2 = t2;
    let mut t3 = if !len1.is_null() { *len1 } else { 0 };
    let mut t4 = if !op2.is_null() { *op2 } else { core::ptr::null() };
    let mut t5 = if !len2.is_null() { *len2 } else { 0 };
    let mut cc: i32;

    // The DEFLATE CONVERSION CALL instruction is architecture-specific (s390x).
    // This preserves the C inline-assembly operation and its register effects.
    core::arch::asm!(
        ".insn rrf,0xb9390000,{r2},{r4},{hist},0",
        "ipm {cc}",
        r2 = inout(reg) t2,
        r3 = inout(reg) t3,
        r4 = inout(reg) t4,
        r5 = inout(reg) t5,
        r0 = in(reg) fn_,
        r1 = in(reg) param,
        hist = in(reg) hist,
        cc = lateout(reg) cc,
        options(nostack)
    );

    match fn_ & DFLTCC_FN_MASK {
        DFLTCC_QAF => kmsan_unpoison_memory(param, core::mem::size_of::<dfltcc_qaf_param>()),
        DFLTCC_GDHT => kmsan_unpoison_memory(param, core::mem::offset_of!(dfltcc_param_v0, csb)),
        DFLTCC_CMPR => {
            kmsan_unpoison_memory(param, core::mem::size_of::<dfltcc_param_v0>());
            let sbb = *((param as *const dfltcc_param_v0).cast::<u8>()); // field supplied by dfltcc.h
            kmsan_unpoison_memory(orig_t2.cast(), t2.offset_from(orig_t2) as usize + if sbb == 0 { 0 } else { 1 });
        }
        DFLTCC_XPND => {
            kmsan_unpoison_memory(param, core::mem::size_of::<dfltcc_param_v0>());
            kmsan_unpoison_memory(orig_t2.cast(), t2.offset_from(orig_t2) as usize);
        }
        _ => {}
    }
    if !op1.is_null() { *op1 = t2; }
    if !len1.is_null() { *len1 = t3; }
    if !op2.is_null() { *op2 = t4; }
    if !len2.is_null() { *len2 = t5; }
    match (cc >> 28) & 3 { 0 => dfltcc_cc::DFLTCC_CC_OK, 1 => dfltcc_cc::DFLTCC_CC_OP1_TOO_SHORT, 2 => dfltcc_cc::DFLTCC_CC_OP2_TOO_SHORT, _ => dfltcc_cc::DFLTCC_CC_AGAIN }
}

#[inline]
pub unsafe fn is_bit_set(bits: *const i8, n: i32) -> i32 {
    (*bits.offset((n / 8) as isize) as i32) & (1 << (7 - (n % 8)))
}

#[inline]
pub unsafe fn turn_bit_off(bits: *mut i8, n: i32) {
    *bits.offset((n / 8) as isize) &= !(1 << (7 - (n % 8))) as i8;
}

#[inline]
pub fn dfltcc_are_params_ok(level: i32, window_bits: u32, strategy: i32, level_mask: u64) -> i32 {
    ((level_mask & (1u64 << level)) != 0 && window_bits == HB_BITS && strategy == 0) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
