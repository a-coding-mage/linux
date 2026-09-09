// SPDX-License-Identifier: GPL-2.0

// Dependency declarations corresponding to <linux/kprobes.h>.
use core::ffi::c_void;

#[repr(C)]
pub struct ftrace_ops {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct ftrace_regs {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _opaque: [u8; 0],
}

pub type kprobe_opcode_t = u8;

pub type KprobePreHandler = unsafe extern "C" fn(*mut kprobe, *mut pt_regs) -> i32;
pub type KprobePostHandler = unsafe extern "C" fn(*mut kprobe, *mut pt_regs, i32);

#[repr(C)]
pub struct kprobe_ainsn_api {
    pub insn: *mut c_void,
}

#[repr(C)]
pub struct kprobe_ainsn {
    pub api: kprobe_ainsn_api,
}

#[repr(C)]
pub struct kprobe {
    pub addr: *mut c_void,
    pub ainsn: kprobe_ainsn,
    pub pre_handler: Option<KprobePreHandler>,
    pub post_handler: Option<KprobePostHandler>,
}

#[repr(C)]
pub struct kprobe_ctlblk {
    pub kprobe_status: i32,
}

extern "C" {
    static mut kprobe_ftrace_disabled: bool;

    fn ftrace_test_recursion_trylock(ip: usize, parent_ip: usize) -> i32;
    fn ftrace_test_recursion_unlock(bit: i32);
    fn ftrace_get_regs(fregs: *mut ftrace_regs) -> *mut pt_regs;
    fn get_kprobe(addr: *mut kprobe_opcode_t) -> *mut kprobe;
    fn get_kprobe_ctlblk() -> *mut kprobe_ctlblk;
    fn kprobe_running() -> bool;
    fn kprobes_inc_nmissed_count(p: *mut kprobe);
    fn instruction_pointer(regs: *mut pt_regs) -> usize;
    fn instruction_pointer_set(regs: *mut pt_regs, value: usize);
    fn __this_cpu_write_current_kprobe(p: *mut kprobe);
    fn kprobe_disabled(p: *mut kprobe) -> bool;
}

pub const MCOUNT_INSN_SIZE: usize = 4;
pub const KPROBE_HIT_ACTIVE: i32 = 0;
pub const KPROBE_HIT_SSDONE: i32 = 1;

/* Ftrace callback handler for kprobes -- called under preepmt disabled */
pub unsafe extern "C" fn kprobe_ftrace_handler(
    mut ip: usize,
    parent_ip: usize,
    _ops: *mut ftrace_ops,
    fregs: *mut ftrace_regs,
) {
    let bit: i32;
    let mut lr_saver = false;
    let mut p: *mut kprobe;
    let kcb: *mut kprobe_ctlblk;
    let regs: *mut pt_regs;

    if kprobe_ftrace_disabled {
        return;
    }

    bit = ftrace_test_recursion_trylock(ip, parent_ip);
    if bit < 0 {
        return;
    }

    regs = ftrace_get_regs(fregs);
    p = get_kprobe(ip as *mut kprobe_opcode_t);
    if p.is_null() {
        p = get_kprobe(ip.wrapping_sub(MCOUNT_INSN_SIZE) as *mut kprobe_opcode_t);
        if p.is_null() || kprobe_disabled(p) {
            ftrace_test_recursion_unlock(bit);
            return;
        }
        lr_saver = true;
    }

    kcb = get_kprobe_ctlblk();
    if kprobe_running() {
        kprobes_inc_nmissed_count(p);
    } else {
        let orig_ip = instruction_pointer(regs);

        if lr_saver {
            ip = ip.wrapping_sub(MCOUNT_INSN_SIZE);
        }
        instruction_pointer_set(regs, ip);
        __this_cpu_write_current_kprobe(p);
        (*kcb).kprobe_status = KPROBE_HIT_ACTIVE;
        let pre_handler = (*p).pre_handler;
        if pre_handler.is_none() || pre_handler.unwrap()(p, regs) == 0 {
            /*
             * Emulate singlestep (and also recover regs->pc)
             * as if there is a nop
             */
            instruction_pointer_set(regs, (*p).addr as usize + MCOUNT_INSN_SIZE);
            if let Some(post_handler) = (*p).post_handler {
                (*kcb).kprobe_status = KPROBE_HIT_SSDONE;
                post_handler(p, regs, 0);
            }
            instruction_pointer_set(regs, orig_ip);
        }
        /*
         * If pre_handler returns !0, it changes regs->pc. We have to
         * skip emulating post_handler.
         */
        __this_cpu_write_current_kprobe(core::ptr::null_mut());
    }

    ftrace_test_recursion_unlock(bit);
}

// NOKPROBE_SYMBOL(kprobe_ftrace_handler);

pub unsafe extern "C" fn arch_prepare_kprobe_ftrace(p: *mut kprobe) -> i32 {
    (*p).ainsn.api.insn = core::ptr::null_mut();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
