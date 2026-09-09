/* SPDX-License-Identifier: GPL-2.0-only */

// C dependencies: <asm/ptrace.h>, <linux/kprobes.h>

use core::ffi::c_void;

// `struct pt_regs` is supplied by the translated ptrace dependency.
type PtRegs = crate::pt_regs;

unsafe extern "C" {
    pub static mut exception_table: [*mut c_void; 0];

    pub fn show_registers(regs: *mut PtRegs);

    // `asmlinkage` and `noinstr` are calling/instrumentation attributes in C.
    pub fn cache_parity_error();
    pub fn do_ade(regs: *mut PtRegs);
    pub fn do_ale(regs: *mut PtRegs);
    pub fn do_bce(regs: *mut PtRegs);
    pub fn do_bp(regs: *mut PtRegs);
    pub fn do_ri(regs: *mut PtRegs);
    pub fn do_fpu(regs: *mut PtRegs);
    pub fn do_fpe(regs: *mut PtRegs, fcsr: c_ulong);
    pub fn do_lsx(regs: *mut PtRegs);
    pub fn do_lasx(regs: *mut PtRegs);
    pub fn do_lbt(regs: *mut PtRegs);
    pub fn do_watch(regs: *mut PtRegs);
    pub fn do_syscall(regs: *mut PtRegs);
    pub fn do_reserved(regs: *mut PtRegs);
    pub fn do_vint(regs: *mut PtRegs, sp: c_ulong);
    // `__kprobes` is a C instrumentation attribute.
    pub fn do_page_fault(regs: *mut PtRegs, write: c_ulong, address: c_ulong);

    pub fn handle_ade();
    pub fn handle_ale();
    pub fn handle_bce();
    pub fn handle_sys();
    pub fn handle_bp();
    pub fn handle_ri();
    pub fn handle_fpu();
    pub fn handle_fpe();
    pub fn handle_lsx();
    pub fn handle_lasx();
    pub fn handle_lbt();
    pub fn handle_watch();
    pub fn handle_reserved();
    pub fn handle_vint();
    pub fn handle_loongarch_irq(regs: *mut PtRegs);
}

type c_ulong = core::ffi::c_ulong;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
