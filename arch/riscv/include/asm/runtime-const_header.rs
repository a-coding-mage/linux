/* SPDX-License-Identifier: GPL-2.0 */

/* C header dependencies: asm/asm.h, asm/alternative.h, asm/cacheflush.h,
 * asm/insn-def.h, linux/memory.h, asm/text-patching.h, linux/uaccess.h. */

pub const RUNTIME_MAGIC: u32 = 0x89AB_CDEF;

/* The original runtime_const_ptr() macros emit RISC-V patch records through
 * inline assembly.  The target build supplies the architecture-specific
 * assembly names and configuration predicates. */
#[macro_export]
macro_rules! runtime_const_ptr {
    ($sym:expr) => {{
        let _ = &$sym;
        unsafe { core::arch::asm!("", options(nostack, preserves_flags)); }
        $sym
    }};
}

#[macro_export]
macro_rules! runtime_const_shift_right_32 {
    ($val:expr, $sym:expr) => {{
        let _ = &$sym;
        (($val as u32) >> 12)
    }};
}

#[macro_export]
macro_rules! runtime_const_mask_32 {
    ($val:expr, $sym:expr) => {{
        let _ = &$sym;
        (($val as u32) << 12) >> 12
    }};
}

#[inline]
pub unsafe fn __runtime_fixup_caches(where_: *mut core::ffi::c_void, insns: u32) {
    /* On RISC-V there are currently only cache-wide flushes, so va is ignored. */
    let va = where_ as usize;
    flush_icache_range(va, va.wrapping_add(4usize.wrapping_mul(insns as usize)));
}

#[inline]
pub unsafe fn __runtime_fixup_32(
    lui_parcel: *mut u16,
    addi_parcel: *mut u16,
    val: u32,
) {
    /* The 32-bit immediate is stored in a lui+addi pairing. */
    let mut addi_insn_mask: u32 = 0x000f_ffff;
    let mut lui_insn = (u16::from_le(core::ptr::read(lui_parcel)) as u32)
        | ((u16::from_le(core::ptr::read(lui_parcel.add(1))) as u32) << 16);
    let mut addi_insn = (u16::from_le(core::ptr::read(addi_parcel)) as u32)
        | ((u16::from_le(core::ptr::read(addi_parcel.add(1))) as u32) << 16);

    let lower_immediate = (val << 20) as i32 >> 20;
    let upper_immediate = val.wrapping_sub(lower_immediate as u32);

    if upper_immediate & 0xffff_f000 != 0 {
        lui_insn &= 0x0000_0fff;
        lui_insn |= upper_immediate & 0xffff_f000;
    } else {
        lui_insn = RISCV_INSN_NOP4;
        addi_insn_mask &= 0x07fff;
    }

    if (lower_immediate as u32) & 0x0000_0fff != 0 || lui_insn == RISCV_INSN_NOP4 {
        addi_insn &= addi_insn_mask;
        addi_insn |= ((lower_immediate as u32) & 0x0000_0fff) << 20;
    } else {
        addi_insn = RISCV_INSN_NOP4;
    }

    let addi_res = addi_insn.to_le_bytes();
    let lui_res = lui_insn.to_le_bytes();
    mutex_lock(&text_mutex);
    patch_insn_write(addi_parcel as *mut core::ffi::c_void, addi_res.as_ptr() as *const _, 4);
    patch_insn_write(lui_parcel as *mut core::ffi::c_void, lui_res.as_ptr() as *const _, 4);
    mutex_unlock(&text_mutex);
}

#[inline]
pub unsafe fn __runtime_fixup_ptr(where_: *mut core::ffi::c_void, val: usize) {
    __runtime_fixup_32(where_ as *mut u16, where_.add(4) as *mut u16, val as u32);
    __runtime_fixup_32(where_.add(4) as *mut u16, where_.add(8) as *mut u16, (val >> 32) as u32);
    __runtime_fixup_caches(where_, 4);
}

#[inline]
pub unsafe fn __runtime_fixup_shift(where_: *mut core::ffi::c_void, val: usize) {
    let parcel = where_ as *mut u16;
    let mut insn = u16::from_le(core::ptr::read(parcel)) as u32
        | ((u16::from_le(core::ptr::read(parcel.add(1))) as u32) << 16);
    insn &= 0xfe0f_ffff;
    insn |= ((val as u32) & 0b1_1111) << 20;
    let res = insn.to_le_bytes();
    mutex_lock(&text_mutex);
    patch_text_nosync(where_, res.as_ptr() as *const _, 4);
    mutex_unlock(&text_mutex);
}

#[inline]
pub unsafe fn __runtime_fixup_mask(where_: *mut core::ffi::c_void, val: usize) {
    let width = if val != 0 { (usize::BITS - val.leading_zeros()) as usize } else { 0 };
    BUG_ON(val == 0 || width > 31 || (GENMASK(width - 1, 0) != val));
    __runtime_fixup_shift(where_, 32 - width);
    __runtime_fixup_shift(where_.add(4), 32 - width);
}

#[inline]
pub unsafe fn runtime_const_fixup(
    fn_: unsafe fn(*mut core::ffi::c_void, usize),
    val: usize,
    mut start: *mut i32,
    end: *mut i32,
) {
    while start < end {
        fn_(((*start) as isize as usize as *mut u8).add(start as usize) as *mut _, val);
        start = start.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
