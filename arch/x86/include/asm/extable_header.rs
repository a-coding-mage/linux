/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by the surrounding translation unit:
 * asm/extable_fixup_types.h
 */

/*
 * The exception table consists of two addresses relative to the
 * exception table entry itself and a type selector field.
 *
 * The first address is of an instruction that is allowed to fault, the
 * second is the target at which the program should continue.
 *
 * The type entry is used by fixup_exception() to select the handler to
 * deal with the fault caused by the instruction in the first field.
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
    pub data: i32,
}

pub struct pt_regs;

/* ARCH_HAS_RELATIVE_EXTABLE */

#[inline]
pub unsafe fn swap_ex_entry_fixup(
    a: *mut exception_table_entry,
    b: *mut exception_table_entry,
    tmp: exception_table_entry,
    delta: i32,
) {
    (*a).fixup = (*b).fixup.wrapping_add(delta);
    (*b).fixup = tmp.fixup.wrapping_sub(delta);
    (*a).data = (*b).data;
    (*b).data = tmp.data;
}

unsafe extern "C" {
    pub fn fixup_exception(
        regs: *mut pt_regs,
        trapnr: i32,
        error_code: usize,
        fault_addr: usize,
    ) -> i32;
    pub fn ex_get_fixup_type(ip: usize) -> i32;
    pub fn early_fixup_exception(regs: *mut pt_regs, trapnr: i32);
}

/* The following conditional preserves CONFIG_X86_MCE. */
#[cfg(CONFIG_X86_MCE)]
unsafe extern "C" {
    pub fn ex_handler_msr_mce(regs: *mut pt_regs, wrmsr: bool) -> !;
}

#[cfg(not(CONFIG_X86_MCE))]
#[inline]
pub unsafe fn ex_handler_msr_mce(_regs: *mut pt_regs, _wrmsr: bool) -> ! {
    loop {
        cpu_relax();
    }
}

/* The following conditional preserves CONFIG_BPF_JIT && CONFIG_X86_64. */
#[cfg(all(CONFIG_BPF_JIT, CONFIG_X86_64))]
unsafe extern "C" {
    pub fn ex_handler_bpf(
        x: *const exception_table_entry,
        regs: *mut pt_regs,
    ) -> bool;
}

#[cfg(not(all(CONFIG_BPF_JIT, CONFIG_X86_64)))]
#[inline]
pub unsafe fn ex_handler_bpf(
    _x: *const exception_table_entry,
    _regs: *mut pt_regs,
) -> bool {
    false
}

/* Supplied by the surrounding translation unit. */
unsafe extern "C" {
    fn cpu_relax();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
