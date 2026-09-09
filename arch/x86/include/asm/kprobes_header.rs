/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Kernel Probes (KProbes)
 *
 * Copyright (C) IBM Corporation, 2002, 2004
 *
 * See arch/x86/kernel/kprobes.c for x86 kprobes history.
 *
 * The declarations below depend on symbols supplied by asm-generic/kprobes.h,
 * linux/types.h, linux/ptrace.h, linux/percpu.h, asm/text-patching.h, and
 * asm/insn.h.
 */

/* __ARCH_WANT_KPROBES_INSN_SLOT */

#[cfg(CONFIG_KPROBES)]
pub type KprobeOpcodeT = u8;

#[cfg(CONFIG_KPROBES)]
pub const MAX_STACK_SIZE: usize = 64;

#[cfg(CONFIG_KPROBES)]
#[inline]
pub unsafe fn CUR_STACK_SIZE(addr: *const core::ffi::c_void) -> usize {
    current_top_of_stack() - (addr as usize)
}

#[cfg(CONFIG_KPROBES)]
#[inline]
pub unsafe fn MIN_STACK_SIZE(addr: *const core::ffi::c_void) -> usize {
    if MAX_STACK_SIZE < CUR_STACK_SIZE(addr) {
        MAX_STACK_SIZE
    } else {
        CUR_STACK_SIZE(addr)
    }
}

#[cfg(CONFIG_KPROBES)]
#[inline]
pub fn flush_insn_slot<T>(_p: *mut T) {}

#[cfg(CONFIG_KPROBES)]
unsafe extern "C" {
    pub static mut optprobe_template_entry: KprobeOpcodeT;
    pub static mut optprobe_template_clac: KprobeOpcodeT;
    pub static mut optprobe_template_val: KprobeOpcodeT;
    pub static mut optprobe_template_call: KprobeOpcodeT;
    pub static mut optprobe_template_end: KprobeOpcodeT;
    pub static kretprobe_blacklist_size: core::ffi::c_int;
    pub fn arch_remove_kprobe(p: *mut Kprobe);
    pub fn kprobe_fault_handler(regs: *mut PtRegs, trapnr: core::ffi::c_int) -> core::ffi::c_int;
    pub fn kprobe_int3_handler(regs: *mut PtRegs) -> core::ffi::c_int;
}

#[cfg(CONFIG_KPROBES)]
pub const MAX_OPTIMIZED_LENGTH: usize = MAX_INSN_SIZE + DISP32_SIZE;

#[cfg(CONFIG_KPROBES)]
pub const MAX_OPTINSN_SIZE: usize =
    ((unsafe { &optprobe_template_end as *const _ as usize }
        - unsafe { &optprobe_template_entry as *const _ as usize })
        + MAX_OPTIMIZED_LENGTH
        + JMP32_INSN_SIZE);

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct ArchSpecificInsn {
    pub insn: *mut KprobeOpcodeT,
    pub boostable: u8,
    pub size: u8,
    pub data: ArchSpecificInsnData,
    pub rel32: i32,
    pub emulate_op: Option<unsafe extern "C" fn(*mut Kprobe, *mut PtRegs)>,
    pub tp_len: core::ffi::c_int,
}

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub union ArchSpecificInsnData {
    pub opcode: u8,
    pub jcc: ArchSpecificInsnJcc,
    pub loop_: ArchSpecificInsnLoop,
    pub indirect: ArchSpecificInsnIndirect,
}

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct ArchSpecificInsnJcc {
    pub type_: u8,
}

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct ArchSpecificInsnLoop {
    pub type_: u8,
    pub asize: u8,
}

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct ArchSpecificInsnIndirect {
    pub reg: u8,
}

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct ArchOptimizedInsn {
    pub copied_insn: [KprobeOpcodeT; DISP32_SIZE],
    pub insn: *mut KprobeOpcodeT,
    pub size: usize,
}

#[cfg(CONFIG_KPROBES)]
#[inline]
pub unsafe fn arch_prepared_optinsn(optinsn: *mut ArchOptimizedInsn) -> core::ffi::c_int {
    (*optinsn).size as core::ffi::c_int
}

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct PrevKprobe {
    pub kp: *mut Kprobe,
    pub status: usize,
    pub old_flags: usize,
    pub saved_flags: usize,
}

#[cfg(CONFIG_KPROBES)]
#[repr(C)]
pub struct KprobeCtlblk {
    pub kprobe_status: usize,
    pub kprobe_old_flags: usize,
    pub kprobe_saved_flags: usize,
    pub prev_kprobe: PrevKprobe,
}

#[cfg(not(CONFIG_KPROBES))]
#[inline]
pub unsafe fn kprobe_debug_handler(_regs: *mut PtRegs) -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
