/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The exception table consists of pairs of relative offsets: the first
 * is the relative offset to an instruction that is allowed to fault,
 * and the second is the relative offset at which the program should
 * continue. No registers are modified, so it is entirely up to the
 * continuation code to figure out what to do.
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

pub const ARCH_HAS_RELATIVE_EXTABLE: bool = true;

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

// CONFIG_BPF_JIT selects the external BPF exception handler declaration.
#[cfg(feature = "CONFIG_BPF_JIT")]
extern "C" {
    pub fn ex_handler_bpf(
        ex: *const exception_table_entry,
        regs: *mut pt_regs,
    ) -> bool;
}

// Fallback when CONFIG_BPF_JIT is not enabled.
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
