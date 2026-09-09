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
    pub r#type: i16,
    pub data: i16,
}

pub const ARCH_HAS_RELATIVE_EXTABLE: bool = true;

macro_rules! swap_ex_entry_fixup {
    ($a:expr, $b:expr, $tmp:expr, $delta:expr) => {{
        ($a).fixup = ($b).fixup + ($delta);
        ($b).fixup = ($tmp).fixup - ($delta);
        ($a).r#type = ($b).r#type;
        ($b).r#type = ($tmp).r#type;
        ($a).data = ($b).data;
        ($b).data = ($tmp).data;
    }};
}

extern "C" {
    pub fn insn_may_access_user(addr: c_ulong, esr: c_ulong) -> bool;
}

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
    pub fn fixup_exception(regs: *mut pt_regs, esr: c_ulong) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
