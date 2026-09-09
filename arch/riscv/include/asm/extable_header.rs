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

#[macro_export]
macro_rules! swap_ex_entry_fixup {
    ($a:expr, $b:expr, $tmp:expr, $delta:expr) => {{
        unsafe {
            (*$a).fixup = (*$b).fixup.wrapping_add($delta);
            (*$b).fixup = ($tmp).fixup.wrapping_sub($delta);
            (*$a).type_ = (*$b).type_;
            (*$b).type_ = ($tmp).type_;
            (*$a).data = (*$b).data;
            (*$b).data = ($tmp).data;
        }
    }};
}

/* CONFIG_MMU selects the external implementation; otherwise this is inline. */
#[cfg(CONFIG_MMU)]
extern "C" {
    pub fn fixup_exception(regs: *mut crate::pt_regs) -> bool;
}

#[cfg(not(CONFIG_MMU))]
#[inline]
pub fn fixup_exception(_regs: *mut crate::pt_regs) -> bool {
    false
}

/* CONFIG_BPF_JIT && CONFIG_ARCH_RV64I selects the external implementation. */
#[cfg(all(CONFIG_BPF_JIT, CONFIG_ARCH_RV64I))]
extern "C" {
    pub fn ex_handler_bpf(
        ex: *const exception_table_entry,
        regs: *mut crate::pt_regs,
    ) -> bool;
}

#[cfg(not(all(CONFIG_BPF_JIT, CONFIG_ARCH_RV64I)))]
#[inline]
pub fn ex_handler_bpf(
    _ex: *const exception_table_entry,
    _regs: *mut crate::pt_regs,
) -> bool {
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
