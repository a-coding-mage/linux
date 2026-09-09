// SPDX-License-Identifier: GPL-2.0-only
/*
 * Here's a sample kernel module showing the use of kprobes to dump a
 * stack trace and selected registers when kernel_clone() is called.
 *
 * For more information on theory of operation of kprobes, see
 * Documentation/trace/kprobes.rst
 *
 * You will see the trace data in /var/log/messages and on the console
 * whenever kernel_clone() is invoked to create a new process.
 */

// Dependency headers: linux/kernel.h, linux/module.h, linux/kprobes.h

use core::ffi::{c_char, c_int, c_ulong};

const KSYM_NAME_LEN: usize = 256;

#[repr(C)]
pub struct kprobe {
    pub symbol_name: *mut c_char,
    pub addr: *mut core::ffi::c_void,
    pub pre_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs) -> c_int>,
    pub post_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs, c_ulong)>,
}

#[repr(C)]
pub struct pt_regs {
    pub ip: c_ulong,
    pub flags: c_ulong,
    pub nip: c_ulong,
    pub msr: c_ulong,
    pub cp0_epc: c_ulong,
    pub cp0_status: c_ulong,
    pub pc: c_ulong,
    pub pstate: c_ulong,
    pub ARM_pc: c_ulong,
    pub ARM_cpsr: c_ulong,
    pub epc: c_ulong,
    pub status: c_ulong,
    pub psw: pt_regs_psw,
    pub csr_era: c_ulong,
    pub csr_estat: c_ulong,
}

#[repr(C)]
pub struct pt_regs_psw { pub addr: c_ulong }

static mut symbol: [c_char; KSYM_NAME_LEN] = {
    let mut value = [0; KSYM_NAME_LEN];
    value[0] = b'k' as c_char; value[1] = b'e' as c_char; value[2] = b'r' as c_char;
    value[3] = b'n' as c_char; value[4] = b'e' as c_char; value[5] = b'l' as c_char;
    value[6] = b'_' as c_char; value[7] = b'c' as c_char; value[8] = b'l' as c_char;
    value[9] = b'o' as c_char; value[10] = b'n' as c_char; value
};

// module_param_string(symbol, symbol, KSYM_NAME_LEN, 0644);

/* For each probe you need to allocate a kprobe structure */
static mut kp: kprobe = kprobe {
    symbol_name: unsafe { symbol.as_ptr() as *mut c_char },
    addr: core::ptr::null_mut(),
    pre_handler: None,
    post_handler: None,
};

extern "C" {
    fn register_kprobe(p: *mut kprobe) -> c_int;
    fn unregister_kprobe(p: *mut kprobe);
}

/* kprobe pre_handler: called just before the probed instruction is executed */
unsafe extern "C" fn handler_pre(p: *mut kprobe, regs: *mut pt_regs) -> c_int {
    // Architecture-specific pr_info calls retained as conditional intent:
    // CONFIG_X86: p->symbol_name, p->addr, regs->ip, regs->flags
    // CONFIG_PPC: p->symbol_name, p->addr, regs->nip, regs->msr
    // CONFIG_MIPS: p->symbol_name, p->addr, regs->cp0_epc, regs->cp0_status
    // CONFIG_ARM64: p->symbol_name, p->addr, (long)regs->pc, (long)regs->pstate
    // CONFIG_ARM: p->symbol_name, p->addr, (long)regs->ARM_pc, (long)regs->ARM_cpsr
    // CONFIG_RISCV: p->symbol_name, p->addr, regs->epc, regs->status
    // CONFIG_S390: p->symbol_name, p->addr, regs->psw.addr, regs->flags
    // CONFIG_LOONGARCH: p->symbol_name, p->addr, regs->csr_era, regs->csr_estat
    let _ = (p, regs);
    /* A dump_stack() here will give a stack backtrace */
    0
}

/* kprobe post_handler: called after the probed instruction is executed */
unsafe extern "C" fn handler_post(p: *mut kprobe, regs: *mut pt_regs, flags: c_ulong) {
    // Architecture-specific pr_info calls retained as conditional intent:
    // CONFIG_X86: p->symbol_name, p->addr, regs->flags
    // CONFIG_PPC: p->symbol_name, p->addr, regs->msr
    // CONFIG_MIPS: p->symbol_name, p->addr, regs->cp0_status
    // CONFIG_ARM64: p->symbol_name, p->addr, (long)regs->pstate
    // CONFIG_ARM: p->symbol_name, p->addr, (long)regs->ARM_cpsr
    // CONFIG_RISCV: p->symbol_name, p->addr, regs->status
    // CONFIG_S390: p->symbol_name, p->addr, regs->flags
    // CONFIG_LOONGARCH: p->symbol_name, p->addr, regs->csr_estat
    let _ = (p, regs, flags);
}

unsafe extern "C" fn kprobe_init() -> c_int {
    kp.pre_handler = Some(handler_pre);
    kp.post_handler = Some(handler_post);

    let ret = register_kprobe(&mut kp);
    if ret < 0 {
        // pr_err("register_kprobe failed, returned %d\\n", ret);
        return ret;
    }
    // pr_info("Planted kprobe at %p\\n", kp.addr);
    0
}

unsafe extern "C" fn kprobe_exit() {
    unregister_kprobe(&mut kp);
    // pr_info("kprobe at %p unregistered\\n", kp.addr);
}

// module_init(kprobe_init)
// module_exit(kprobe_exit)
// MODULE_DESCRIPTION("sample kernel module showing the use of kprobes");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
