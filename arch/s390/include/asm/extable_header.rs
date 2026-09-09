/* SPDX-License-Identifier: GPL-2.0 */

// Dependency declarations supplied by the included architecture headers.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

/*
 * The exception table consists of three addresses:
 *
 * - Address of an instruction that is allowed to fault.
 * - Address at which the program should continue.
 * - Optional address of handler that takes pt_regs * argument and runs in
 *   interrupt context.
 *
 * No registers are modified, so it is entirely up to the continuation code
 * to figure out what to do.
 *
 * All the routines below use bits of fixup code that are out of line
 * with the main instruction path.  This means when everything is well,
 * we don't even have to jump over them.  Further, they do not intrude
 * on our cache or tlb entries.
 */
#[repr(C)]
pub struct exception_table_entry {
    pub insn: i32,
    pub fixup: i32,
    pub type_: i16,
    pub data: i16,
}

extern "C" {
    pub static mut __start_amode31_ex_table: exception_table_entry;
    pub static mut __stop_amode31_ex_table: exception_table_entry;

    pub fn s390_search_extables(addr: ::core::ffi::c_ulong) -> *const exception_table_entry;
}

#[inline]
pub unsafe fn extable_fixup(x: *const exception_table_entry) -> ::core::ffi::c_ulong {
    (core::ptr::addr_of!((*x).fixup) as usize)
        .wrapping_add((*x).fixup as isize as usize) as ::core::ffi::c_ulong
}

// ARCH_HAS_RELATIVE_EXTABLE

#[inline]
pub unsafe fn swap_ex_entry_fixup(
    a: *mut exception_table_entry,
    b: *mut exception_table_entry,
    tmp: exception_table_entry,
    delta: i32,
) {
    (*a).fixup = (*b).fixup.wrapping_add(delta);
    (*b).fixup = tmp.fixup.wrapping_sub(delta);
    (*a).type_ = (*b).type_;
    (*b).type_ = tmp.type_;
    (*a).data = (*b).data;
    (*b).data = tmp.data;
}

// CONFIG_BPF_JIT selects the external implementation; otherwise this inline
// handler always reports that it did not handle the exception.
#[cfg(feature = "CONFIG_BPF_JIT")]
extern "C" {
    pub fn ex_handler_bpf(
        ex: *const exception_table_entry,
        regs: *mut pt_regs,
    ) -> bool;
}

#[cfg(not(feature = "CONFIG_BPF_JIT"))]
#[inline]
pub unsafe fn ex_handler_bpf(
    _ex: *const exception_table_entry,
    _regs: *mut pt_regs,
) -> bool {
    false
}

extern "C" {
    pub fn fixup_exception(regs: *mut pt_regs) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
