// SPDX-License-Identifier: GPL-2.0
/*
 * PA-RISC KGDB support
 *
 * Copyright (c) 2019 Sven Schnelle <svens@stackframe.org>
 * Copyright (c) 2022 Helge Deller <deller@gmx.de>
 *
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct KgdbArch {
    pub gdb_bpt_instr: [u8; 4],
}

pub static arch_kgdb_ops: KgdbArch = KgdbArch {
    gdb_bpt_instr: [0x03, 0xff, 0xa0, 0x1f],
};

extern "C" {
    fn kgdb_handle_exception(
        exception: i32,
        signo: i32,
        cmd: usize,
        regs: *mut pt_regs,
    ) -> i32;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn register_die_notifier(nb: *mut notifier_block) -> i32;
    fn unregister_die_notifier(nb: *mut notifier_block);
    fn task_pt_regs(task: *mut task_struct) -> *mut pt_regs;
    fn copy_from_kernel_nofault(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, size: usize) -> i32;
    fn __patch_text(addr: *mut core::ffi::c_void, insn: u32);
    fn kgdb_hex2long(ptr: *mut *mut u8, value: *mut usize) -> i32;
    fn mtctl(value: i32, cr: i32);
}

extern "C" {
    static mut kgdb_contthread: *mut task_struct;
    static mut kgdb_single_step: i32;
}

#[repr(C)]
pub struct die_args {
    pub regs: *mut pt_regs,
    pub signr: i32,
}

#[repr(C)]
pub struct notifier_block {
    pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut core::ffi::c_void) -> i32>,
    pub priority: i32,
}

#[repr(C)]
pub struct pt_regs {
    pub gr: [usize; 32],
    pub fr: [usize; 32],
    pub sr: [usize; 8],
    pub sar: usize,
    pub iir: usize,
    pub isr: usize,
    pub ior: usize,
    pub ipsw: usize,
    pub cr27: usize,
    pub iaoq: [usize; 2],
    pub iasq: [usize; 2],
    pub ksp: usize,
    pub kpc: usize,
}

#[repr(C)]
pub struct parisc_gdb_regs {
    pub gpr: [usize; 32],
    pub fr: [usize; 32],
    pub sr0: usize, pub sr1: usize, pub sr2: usize, pub sr3: usize,
    pub sr4: usize, pub sr5: usize, pub sr6: usize, pub sr7: usize,
    pub sar: usize, pub iir: usize, pub isr: usize, pub ior: usize,
    pub ipsw: usize, pub cr27: usize,
    pub iaoq_f: usize, pub iasq_f: usize,
    pub iaoq_b: usize, pub iasq_b: usize,
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kgdb_bkpt {
    pub saved_instr: *mut u8,
    pub bpt_addr: usize,
}

const NOTIFY_DONE: i32 = 0;
const NOTIFY_STOP: i32 = 0x8000;
const INT_MAX: i32 = 0x7fff_ffff;
const BREAK_INSTR_SIZE: usize = 4;
const PARISC_KGDB_COMPILED_BREAK_INSN: usize = 0x03ffa01f;
const PSW_R: usize = 1 << 19;

unsafe fn __kgdb_notify(args: *mut die_args, cmd: usize) -> i32 {
    let regs = (*args).regs;

    if kgdb_handle_exception(1, (*args).signr, cmd, regs) != 0 {
        return NOTIFY_DONE;
    }
    NOTIFY_STOP
}

unsafe extern "C" fn kgdb_notify(_self: *mut notifier_block, cmd: usize, ptr: *mut core::ffi::c_void) -> i32 {
    let mut flags: usize = 0;
    local_irq_save(&mut flags);
    let ret = __kgdb_notify(ptr as *mut die_args, cmd);
    local_irq_restore(flags);
    ret
}

static mut kgdb_notifier: notifier_block = notifier_block {
    notifier_call: Some(kgdb_notify),
    priority: -INT_MAX,
};

pub unsafe fn kgdb_arch_init() -> i32 {
    register_die_notifier(&mut kgdb_notifier)
}

pub unsafe fn kgdb_arch_exit() {
    unregister_die_notifier(&mut kgdb_notifier);
}

pub unsafe fn pt_regs_to_gdb_regs(gdb_regs: *mut usize, regs: *mut pt_regs) {
    let gr = gdb_regs as *mut parisc_gdb_regs;
    core::ptr::write_bytes(gr, 0, 1);
    core::ptr::copy_nonoverlapping((*regs).gr.as_ptr(), (*gr).gpr.as_mut_ptr(), (*gr).gpr.len());
    core::ptr::copy_nonoverlapping((*regs).fr.as_ptr(), (*gr).fr.as_mut_ptr(), (*gr).fr.len());
    (*gr).sr0 = (*regs).sr[0]; (*gr).sr1 = (*regs).sr[1]; (*gr).sr2 = (*regs).sr[2]; (*gr).sr3 = (*regs).sr[3];
    (*gr).sr4 = (*regs).sr[4]; (*gr).sr5 = (*regs).sr[5]; (*gr).sr6 = (*regs).sr[6]; (*gr).sr7 = (*regs).sr[7];
    (*gr).sar = (*regs).sar; (*gr).iir = (*regs).iir; (*gr).isr = (*regs).isr; (*gr).ior = (*regs).ior;
    (*gr).ipsw = (*regs).ipsw; (*gr).cr27 = (*regs).cr27;
    (*gr).iaoq_f = (*regs).iaoq[0]; (*gr).iasq_f = (*regs).iasq[0];
    (*gr).iaoq_b = (*regs).iaoq[1]; (*gr).iasq_b = (*regs).iasq[1];
}

pub unsafe fn gdb_regs_to_pt_regs(gdb_regs: *mut usize, regs: *mut pt_regs) {
    let gr = gdb_regs as *mut parisc_gdb_regs;
    core::ptr::copy_nonoverlapping((*gr).gpr.as_ptr(), (*regs).gr.as_mut_ptr(), (*regs).gr.len());
    core::ptr::copy_nonoverlapping((*gr).fr.as_ptr(), (*regs).fr.as_mut_ptr(), (*regs).fr.len());
    (*regs).sr[0] = (*gr).sr0; (*regs).sr[1] = (*gr).sr1; (*regs).sr[2] = (*gr).sr2; (*regs).sr[3] = (*gr).sr3;
    (*regs).sr[4] = (*gr).sr4; (*regs).sr[5] = (*gr).sr5; (*regs).sr[6] = (*gr).sr6; (*regs).sr[7] = (*gr).sr7;
    (*regs).sar = (*gr).sar; (*regs).iir = (*gr).iir; (*regs).isr = (*gr).isr; (*regs).ior = (*gr).ior;
    (*regs).ipsw = (*gr).ipsw; (*regs).cr27 = (*gr).cr27;
    (*regs).iaoq[0] = (*gr).iaoq_f; (*regs).iasq[0] = (*gr).iasq_f;
    (*regs).iaoq[1] = (*gr).iaoq_b; (*regs).iasq[1] = (*gr).iasq_b;
}

pub unsafe fn sleeping_thread_to_gdb_regs(gdb_regs: *mut usize, task: *mut task_struct) {
    let regs = task_pt_regs(task);
    let gr30 = (*regs).gr[30];
    let iaoq = (*regs).iaoq[0];
    (*regs).gr[30] = (*regs).ksp;
    (*regs).iaoq[0] = (*regs).kpc;
    pt_regs_to_gdb_regs(gdb_regs, regs);
    (*regs).gr[30] = gr30;
    (*regs).iaoq[0] = iaoq;
}

unsafe fn step_instruction_queue(regs: *mut pt_regs) {
    (*regs).iaoq[0] = (*regs).iaoq[1];
    (*regs).iaoq[1] = (*regs).iaoq[1].wrapping_add(4);
}

pub unsafe fn kgdb_arch_set_pc(regs: *mut pt_regs, ip: usize) {
    (*regs).iaoq[0] = ip;
    (*regs).iaoq[1] = ip.wrapping_add(4);
}

pub unsafe fn kgdb_arch_set_breakpoint(bpt: *mut kgdb_bkpt) -> i32 {
    let ret = copy_from_kernel_nofault((*bpt).saved_instr as *mut core::ffi::c_void, (*bpt).bpt_addr as *const core::ffi::c_void, BREAK_INSTR_SIZE);
    if ret != 0 { return ret; }
    let insn = u32::from_ne_bytes(arch_kgdb_ops.gdb_bpt_instr);
    __patch_text((*bpt).bpt_addr as *mut core::ffi::c_void, insn);
    ret
}

pub unsafe fn kgdb_arch_remove_breakpoint(bpt: *mut kgdb_bkpt) -> i32 {
    let insn = core::ptr::read_unaligned((*bpt).saved_instr as *const u32);
    __patch_text((*bpt).bpt_addr as *mut core::ffi::c_void, insn);
    0
}

pub unsafe fn kgdb_arch_handle_exception(trap: i32, _signo: i32, _err_code: i32, inbuf: *mut u8, _outbuf: *mut u8, regs: *mut pt_regs) -> i32 {
    let mut addr: usize = 0;
    let mut p = inbuf.add(1);
    match *inbuf {
        b'D' | b'c' | b'k' => {
            kgdb_contthread = core::ptr::null_mut();
            kgdb_single_step = 0;
            if kgdb_hex2long(&mut p, &mut addr) != 0 {
                kgdb_arch_set_pc(regs, addr);
            } else if trap == 9 && (*regs).iir == PARISC_KGDB_COMPILED_BREAK_INSN {
                step_instruction_queue(regs);
            }
            0
        }
        b's' => {
            kgdb_single_step = 1;
            if kgdb_hex2long(&mut p, &mut addr) != 0 {
                kgdb_arch_set_pc(regs, addr);
            } else if trap == 9 && (*regs).iir == PARISC_KGDB_COMPILED_BREAK_INSN {
                step_instruction_queue(regs);
                mtctl(-1, 0);
            } else {
                mtctl(0, 0);
            }
            (*regs).gr[0] |= PSW_R;
            0
        }
        _ => -1,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
