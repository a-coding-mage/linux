/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependencies supplied by the surrounding PowerPC translation.

#[macro_export]
macro_rules! ___get_user_instr {
    ($gu_op:ident, $dest:expr, $ptr:expr) => {{
        let __gui_ptr = $ptr as *mut u32;
        let mut __prefix: u32 = 0;
        let mut __suffix: u32 = 0;
        let mut __gui_ret = $gu_op(&mut __prefix, __gui_ptr);
        if __gui_ret == 0 {
            if cfg!(any(target_pointer_width = "64", feature = "ppc64")) && (__prefix >> 26) == OP_PREFIX {
                __gui_ret = $gu_op(&mut __suffix, unsafe { __gui_ptr.add(1) });
                if __gui_ret == 0 { $dest = ppc_inst_prefix(__prefix, __suffix); }
            } else { $dest = ppc_inst(__prefix); }
        }
        __gui_ret
    }};
}

#[macro_export]
macro_rules! get_user_instr { ($x:expr, $ptr:expr) => { $crate->___get_user_instr!(get_user, $x, $ptr) }; }
#[macro_export]
macro_rules! __get_user_instr { ($x:expr, $ptr:expr) => { $crate->___get_user_instr!(__get_user, $x, $ptr) }; }

/*
 * Instruction data type for POWER
 */

#[cfg(any(target_pointer_width = "64", feature = "ppc64"))]
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ppc_inst_t {
    pub val: u32,
    pub suffix: u32,
}

#[cfg(not(any(target_pointer_width = "64", feature = "ppc64")))]
pub type ppc_inst_t = u32;

#[cfg(any(target_pointer_width = "64", feature = "ppc64"))]
#[inline]
pub fn ppc_inst_val(x: ppc_inst_t) -> u32 { x.val }

#[cfg(not(any(target_pointer_width = "64", feature = "ppc64")))]
#[inline]
pub fn ppc_inst_val(x: ppc_inst_t) -> u32 { x }

#[cfg(any(target_pointer_width = "64", feature = "ppc64"))]
#[inline]
pub fn ppc_inst(x: u32) -> ppc_inst_t { ppc_inst_t { val: x, suffix: 0 } }

#[cfg(not(any(target_pointer_width = "64", feature = "ppc64")))]
#[inline]
pub fn ppc_inst(x: u32) -> ppc_inst_t { x }

#[inline]
pub fn ppc_inst_primary_opcode(x: ppc_inst_t) -> i32 {
    (ppc_inst_val(x) >> 26) as i32
}

#[cfg(any(target_pointer_width = "64", feature = "ppc64"))]
#[inline]
pub fn ppc_inst_prefix(x: u32, y: u32) -> ppc_inst_t { ppc_inst_t { val: x, suffix: y } }

#[cfg(not(any(target_pointer_width = "64", feature = "ppc64")))]
#[inline]
pub fn ppc_inst_prefix(x: u32, _y: u32) -> ppc_inst_t { ppc_inst(x) }

#[cfg(any(target_pointer_width = "64", feature = "ppc64"))]
#[inline]
pub fn ppc_inst_suffix(x: ppc_inst_t) -> u32 { x.suffix }

#[cfg(not(any(target_pointer_width = "64", feature = "ppc64")))]
#[inline]
pub fn ppc_inst_suffix(_x: ppc_inst_t) -> u32 { 0 }

#[inline]
pub unsafe fn ppc_inst_read(ptr: *const u32) -> ppc_inst_t {
    let val = *ptr;
    if cfg!(any(target_pointer_width = "64", feature = "ppc64")) && (val >> 26) == OP_PREFIX {
        ppc_inst_prefix(val, *ptr.add(1))
    } else { ppc_inst(val) }
}

#[inline]
pub fn ppc_inst_prefixed(x: ppc_inst_t) -> bool {
    cfg!(any(target_pointer_width = "64", feature = "ppc64")) && ppc_inst_primary_opcode(x) == OP_PREFIX as i32
}

#[inline]
pub fn ppc_inst_swab(x: ppc_inst_t) -> ppc_inst_t {
    ppc_inst_prefix(ppc_inst_val(x).swap_bytes(), ppc_inst_suffix(x).swap_bytes())
}

#[inline]
pub fn ppc_inst_equal(x: ppc_inst_t, y: ppc_inst_t) -> bool {
    if ppc_inst_val(x) != ppc_inst_val(y) { return false; }
    if !ppc_inst_prefixed(x) { return true; }
    ppc_inst_suffix(x) == ppc_inst_suffix(y)
}

#[inline]
pub fn ppc_inst_len(x: ppc_inst_t) -> i32 { if ppc_inst_prefixed(x) { 8 } else { 4 } }

/* Return the address of the next instruction, if @value was located at @location. */
#[inline]
pub unsafe fn ppc_inst_next(location: *mut u32, value: *const u32) -> *mut u32 {
    let tmp = ppc_inst_read(value);
    (location as *mut u8).add(ppc_inst_len(tmp) as usize) as *mut u32
}

#[inline]
pub fn ppc_inst_as_ulong(x: ppc_inst_t) -> u64 {
    if cfg!(feature = "ppc32") { ppc_inst_val(x) as u64 }
    else if cfg!(target_endian = "little") { ((ppc_inst_suffix(x) as u64) << 32) | ppc_inst_val(x) as u64 }
    else { ((ppc_inst_val(x) as u64) << 32) | ppc_inst_suffix(x) as u64 }
}

#[inline]
pub unsafe fn ppc_inst_write(ptr: *mut u32, x: ppc_inst_t) {
    if !ppc_inst_prefixed(x) { *ptr = ppc_inst_val(x); }
    else { *(ptr as *mut u64) = ppc_inst_as_ulong(x); }
}

#[inline]
pub unsafe fn __copy_inst_from_kernel_nofault(inst: *mut ppc_inst_t, src: *mut u32) -> i32 {
    let val: u32 = __get_kernel_nofault(src);
    if cfg!(any(target_pointer_width = "64", feature = "ppc64")) && get_op(val) == OP_PREFIX {
        let suffix: u32 = __get_kernel_nofault(src.add(1));
        *inst = ppc_inst_prefix(val, suffix);
    } else { *inst = ppc_inst(val); }
    0
}

#[inline]
pub unsafe fn copy_inst_from_kernel_nofault(inst: *mut ppc_inst_t, src: *mut u32) -> i32 {
    if !is_kernel_addr(src as usize) { return -ERANGE; }
    __copy_inst_from_kernel_nofault(inst, src)
}

// The C get_user_instr and __get_user_instr statement-expression macros are
// represented by the corresponding caller-provided user-access operation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
