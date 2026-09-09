// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/hexagon/kernel/kgdb.c - Hexagon KGDB Support
 *
 * Copyright (c) 2011-2012, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the Linux kernel headers:
// linux/irq.h, linux/sched.h, linux/sched/task_stack.h, linux/kdebug.h,
// linux/kgdb.h

/* All registers are 4 bytes, for now */
const GDB_SIZEOF_REG: u32 = 4;

/* The register names are used during printing of the regs;
 * Keep these at three letters to pretty-print. */
#[repr(C)]
pub struct dbg_reg_def_t {
    pub name: *const core::ffi::c_char,
    pub size: u32,
    pub offset: usize,
}

extern "C" {
    static mut dbg_reg_def: [dbg_reg_def_t; DBG_MAX_REG_NUM];
    static mut arch_kgdb_ops: kgdb_arch;
    static mut kgdb_active: core::ffi::c_int;
}

extern "C" {
    fn kgdb_nmicallback(cpu: core::ffi::c_int, regs: *mut pt_regs);
    fn smp_processor_id() -> core::ffi::c_int;
    fn user_mode(regs: *mut pt_regs) -> bool;
    fn kgdb_handle_exception(
        trapnr: core::ffi::c_int,
        signr: core::ffi::c_int,
        err: core::ffi::c_int,
        regs: *mut pt_regs,
    ) -> core::ffi::c_int;
    fn local_irq_save(flags: *mut core::ffi::c_ulong);
    fn local_irq_restore(flags: core::ffi::c_ulong);
    fn register_die_notifier(block: *mut notifier_block) -> core::ffi::c_int;
    fn unregister_die_notifier(block: *mut notifier_block);
    fn memset(s: *mut core::ffi::c_void, c: core::ffi::c_int, n: usize) -> *mut core::ffi::c_void;
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
}

// Kernel-provided types and constants.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}
#[repr(C)]
pub struct task_struct { _private: [u8; 0] }
#[repr(C)]
pub struct die_args {
    pub regs: *mut pt_regs,
    pub trapnr: core::ffi::c_int,
    pub signr: core::ffi::c_int,
    pub err: core::ffi::c_int,
}
#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, core::ffi::c_ulong, *mut core::ffi::c_void) -> core::ffi::c_int>,
    pub priority: core::ffi::c_int,
}
#[repr(C)]
pub struct kgdb_arch { pub gdb_bpt_instr: [u8; 4] }

extern "C" {
    static mut NUMREGBYTES: usize;
}

const DBG_MAX_REG_NUM: usize = 51;
const EINVAL: core::ffi::c_int = 22;
const NOTIFY_STOP: core::ffi::c_int = 0x8000;
const NOTIFY_DONE: core::ffi::c_int = 0;
const INT_MAX: core::ffi::c_int = 0x7fffffff;

pub unsafe fn dbg_get_reg(regno: core::ffi::c_int, mem: *mut core::ffi::c_void, regs: *mut pt_regs) -> *mut core::ffi::c_char {
    if regno >= DBG_MAX_REG_NUM as core::ffi::c_int || regno < 0 { return core::ptr::null_mut(); }
    let def = &dbg_reg_def[regno as usize];
    let src = (regs as *mut u8).add(def.offset) as *const core::ffi::c_ulong;
    *(mem as *mut core::ffi::c_ulong) = *src;
    def.name as *mut core::ffi::c_char
}

pub unsafe fn dbg_set_reg(regno: core::ffi::c_int, mem: *mut core::ffi::c_void, regs: *mut pt_regs) -> core::ffi::c_int {
    if regno >= DBG_MAX_REG_NUM as core::ffi::c_int || regno < 0 { return -EINVAL; }
    let def = &dbg_reg_def[regno as usize];
    let dst = (regs as *mut u8).add(def.offset) as *mut core::ffi::c_ulong;
    *dst = *(mem as *const core::ffi::c_ulong);
    0
}

pub unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, pc: core::ffi::c_ulong) {
    // instruction_pointer(regs) = pc;
    *(regs as *mut u8) = pc as u8;
}

/*  Not yet working  */
pub unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut core::ffi::c_ulong, task: *mut task_struct) {
    if task.is_null() { return; }
    /* Initialize to zero */
    memset(gdb_regs as *mut core::ffi::c_void, 0, NUMREGBYTES);
    /* Otherwise, we have only some registers from switch_to() */
    let thread_regs = task_pt_regs(task);
    // gdb_regs[0] = thread_regs->r00;
    *gdb_regs = *(thread_regs as *const core::ffi::c_ulong);
}

/**
 * kgdb_arch_handle_exception - Handle architecture specific GDB packets.
 * @vector: The error vector of the exception that happened.
 * @signo: The signal number of the exception that happened.
 * @err_code: The error code of the exception that happened.
 * @remcom_in_buffer: The buffer of the packet we have read.
 * @remcom_out_buffer: The buffer of %BUFMAX bytes to write a packet into.
 * @regs: The &struct pt_regs of the current process.
 *
 * This function MUST handle the 'c' and 's' command packets,
 * as well packets to set / remove a hardware breakpoint, if used.
 * If there are additional packets which the hardware needs to handle,
 * they are handled here.  The code should return -1 if it wants to
 * process more packets, and a %0 or %1 if it wants to exit from the
 * kgdb callback.
 *
 * Not yet working.
 */
pub unsafe fn kgdb_arch_handle_exception(_vector: core::ffi::c_int, _signo: core::ffi::c_int, _err_code: core::ffi::c_int, remcom_in_buffer: *mut core::ffi::c_char, _remcom_out_buffer: *mut core::ffi::c_char, _linux_regs: *mut pt_regs) -> core::ffi::c_int {
    match *remcom_in_buffer as u8 {
        b's' | b'c' => 0,
        _ => -1,
    }
}

unsafe fn __kgdb_notify(args: *mut die_args, _cmd: core::ffi::c_ulong) -> core::ffi::c_int {
    /* cpu roundup */
    if kgdb_active != -1 {
        kgdb_nmicallback(smp_processor_id(), (*args).regs);
        return NOTIFY_STOP;
    }
    if user_mode((*args).regs) { return NOTIFY_DONE; }
    if kgdb_handle_exception((*args).trapnr & 0xff, (*args).signr, (*args).err, (*args).regs) != 0 { return NOTIFY_DONE; }
    NOTIFY_STOP
}

unsafe extern "C" fn kgdb_notify(_self: *mut notifier_block, cmd: core::ffi::c_ulong, ptr: *mut core::ffi::c_void) -> core::ffi::c_int {
    let mut flags = 0;
    local_irq_save(&mut flags);
    let ret = __kgdb_notify(ptr as *mut die_args, cmd);
    local_irq_restore(flags);
    ret
}

static mut kgdb_notifier: notifier_block = notifier_block {
    notifier_call: Some(kgdb_notify),
    /* Lowest-prio notifier priority, we want to be notified last: */
    priority: -INT_MAX,
};

/** kgdb_arch_init - Perform any architecture specific initialization. */
pub unsafe fn kgdb_arch_init() -> core::ffi::c_int { register_die_notifier(&mut kgdb_notifier) }

/** kgdb_arch_exit - Perform any architecture specific uninitalization. */
pub unsafe fn kgdb_arch_exit() { unregister_die_notifier(&mut kgdb_notifier); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
