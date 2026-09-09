/* SPDX-License-Identifier: GPL-2.0 */

// The C header's include dependencies are supplied by the surrounding kernel
// translation: asm/cacheflush.h, asm/text-patching.h, and asm/byteorder.h.
// The MODULE build-time error is intentionally retained as a condition note.

/* Cannot use runtime-const infrastructure from modules. */

/*
 * These macros use inline assembly and linker sections to arrange runtime
 * fixups.  Rust has no direct equivalent for C's token-pasted section names;
 * the instruction sequence and its observable result are retained here.
 */
#[macro_export]
macro_rules! runtime_const_ptr {
    ($sym:ident) => {{
        let mut ret: usize;
        unsafe {
            core::arch::asm!(
                "movz {0}, #0xcdef",
                "movk {0}, #0x89ab, lsl #16",
                "movk {0}, #0x4567, lsl #32",
                "movk {0}, #0x0123, lsl #48",
                out(reg) ret,
                options(nostack, preserves_flags)
            );
        }
        // C emits a runtime_ptr_$sym linker section containing the fixup site.
        ret
    }};
}

#[macro_export]
macro_rules! runtime_const_shift_right_32 {
    ($val:expr, $sym:ident) => {{
        let mut ret: usize;
        let value: u32 = 0u32.wrapping_add($val as u32);
        unsafe {
            core::arch::asm!("lsr {0:w}, {1:w}, #12", out(reg) ret, in(reg) value,
                options(nostack, preserves_flags));
        }
        // C emits a runtime_shift_$sym linker section containing the fixup site.
        ret
    }};
}

#[macro_export]
macro_rules! runtime_const_mask_32 {
    ($val:expr, $sym:ident) => {{
        let mut ret: usize;
        let value: u32 = 0u32.wrapping_add($val as u32);
        unsafe {
            core::arch::asm!("ubfx {0:w}, {1:w}, #0, #32", out(reg) ret, in(reg) value,
                options(nostack, preserves_flags));
        }
        // C emits a runtime_mask_$sym linker section containing the fixup site.
        ret
    }};
}

#[macro_export]
macro_rules! runtime_const_init {
    ($kind:ident, $sym:ident) => {{
        extern "C" {
            static __start_runtime_$kind: i32;
            static __stop_runtime_$kind: i32;
        }
        unsafe {
            runtime_const_fixup(__runtime_fixup_$kind,
                $sym as usize,
                &__start_runtime_$kind as *const i32 as *mut i32,
                &__stop_runtime_$kind as *const i32 as *mut i32);
        }
    }};
}

// 16-bit immediate for wide move (movz and movk) in bits 5..20
#[inline]
pub unsafe fn __runtime_fixup_16(p: *mut __le32, val: u32) {
    let mut insn: u32 = le32_to_cpu(*p);
    insn &= 0xffe0001f;
    insn |= (val & 0xffff) << 5;
    aarch64_insn_patch_text_nosync(p as *mut core::ffi::c_void, insn);
}

#[inline]
pub unsafe fn __runtime_fixup_ptr(where_: *mut core::ffi::c_void, val: usize) {
    let p = where_ as *mut __le32;
    __runtime_fixup_16(p, val as u32);
    __runtime_fixup_16(p.add(1), (val >> 16) as u32);
    __runtime_fixup_16(p.add(2), (val >> 32) as u32);
    __runtime_fixup_16(p.add(3), (val >> 48) as u32);
}

// Immediate value is 6 bits starting at bit #16
#[inline]
pub unsafe fn __runtime_fixup_shift(where_: *mut core::ffi::c_void, val: usize) {
    let p = where_ as *mut __le32;
    let mut insn: u32 = le32_to_cpu(*p);
    insn &= 0xffc0ffff;
    insn |= ((val & 63) as u32) << 16;
    aarch64_insn_patch_text_nosync(p as *mut core::ffi::c_void, insn);
}

#[inline]
pub unsafe fn __runtime_fixup_mask(where_: *mut core::ffi::c_void, val: usize) {
    let width: u32 = if val != 0 { (usize::BITS - val.leading_zeros()) } else { 0 };
    let p = where_ as *mut __le32;
    // Supports only GENMASK(width - 1, 0), as in the original implementation.
    BUG_ON(val == 0 || width > 32 || (((1usize << width) - 1) != val));
    let mut insn: u32 = le32_to_cpu(*p);
    insn &= 0xffff03ff;
    insn |= ((width - 1) & 0x1f) << 10;
    aarch64_insn_patch_text_nosync(p as *mut core::ffi::c_void, insn);
}

#[inline]
pub unsafe fn runtime_const_fixup(
    fn_: unsafe fn(*mut core::ffi::c_void, usize),
    val: usize,
    mut start: *mut i32,
    end: *mut i32,
) {
    while start < end {
        fn_((*start as isize as usize + start as usize) as *mut core::ffi::c_void, val);
        start = start.add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
