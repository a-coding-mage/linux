/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from the SPARC64 kprobes header. */

// Dependency supplied by asm-generic/kprobes.h.

pub const BREAKPOINT_INSTRUCTION: u32 = 0x91d0_2070; // ta 0x70
pub const BREAKPOINT_INSTRUCTION_2: u32 = 0x91d0_2071; // ta 0x71

/* The following declarations are present when CONFIG_KPROBES is enabled. */

pub type KprobeOpcodeT = u32;

pub const MAX_INSN_SIZE: usize = 2;

pub const KRETPROBE_BLACKLIST_SIZE: usize = 0;

#[inline]
pub unsafe fn arch_remove_kprobe<T>(_p: *mut T) {
}

/* The argument is expected to provide a mutable `ainsn.insn` array. */
#[macro_export]
macro_rules! flush_insn_slot {
    ($p:expr) => {{
        unsafe {
            flusi(&($p).ainsn.insn[0] as *const _);
            flusi(&($p).ainsn.insn[1] as *const _);
        }
    }};
}

extern "C" {
    pub fn flusi(address: *const KprobeOpcodeT);
    pub fn __kretprobe_trampoline();
}

/* Architecture specific copy of original instruction */
#[repr(C)]
pub struct ArchSpecificInsn {
    /* copy of the original instruction */
    pub insn: [KprobeOpcodeT; MAX_INSN_SIZE],
}

#[repr(C)]
pub struct PrevKprobe {
    pub kp: *mut Kprobe,
    pub status: libc::c_ulong,
    pub orig_tnpc: libc::c_ulong,
    pub orig_tstate_pil: libc::c_ulong,
}

/* per-cpu kprobe control block */
#[repr(C)]
pub struct KprobeCtlblk {
    pub kprobe_status: libc::c_ulong,
    pub kprobe_orig_tnpc: libc::c_ulong,
    pub kprobe_orig_tstate_pil: libc::c_ulong,
    pub prev_kprobe: PrevKprobe,
}

/* Opaque types supplied by the kernel headers. */
pub enum Kprobe {}
pub enum PtRegs {}

extern "C" {
    pub fn kprobe_fault_handler(regs: *mut PtRegs, trapnr: libc::c_int) -> libc::c_int;
    /* asmlinkage and __kprobes are kernel declaration attributes. */
    pub fn kprobe_trap(trap_level: libc::c_ulong, regs: *mut PtRegs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
