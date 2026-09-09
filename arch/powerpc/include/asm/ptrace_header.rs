/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of the PowerPC ptrace header. */

#[repr(C)]
pub union PtRegs {
    pub user_regs: UserPtRegs,
    pub regs: PtRegsFields,
}

#[repr(C)]
pub struct PtRegsFields {
    pub gpr: [usize; 32],
    pub nip: usize,
    pub msr: usize,
    pub orig_gpr3: usize,
    pub ctr: usize,
    pub link: usize,
    pub xer: usize,
    pub ccr: usize,
    #[cfg(target_pointer_width = "64")]
    pub softe: usize,
    #[cfg(not(target_pointer_width = "64"))]
    pub mq: usize,
    pub trap: usize,
    pub dar: usize,
    pub dsisr: usize,
    pub result: usize,
}

#[repr(C)]
pub struct UserPtRegs {
    pub _opaque: [usize; 0],
}

pub const STACK_FRAME_REGS_MARKER: usize = if cfg!(target_endian = "big") {
    0x5245_4753
} else {
    0x5347_4552
};

pub const USER_REDZONE_SIZE: usize = if cfg!(target_pointer_width = "64") { 512 } else { 0 };
pub const KERNEL_REDZONE_SIZE: usize = if cfg!(target_pointer_width = "64") { 288 } else { 0 };
pub const STACK_FRAME_LR_SAVE: usize = if cfg!(target_pointer_width = "64") { 2 } else { 1 };
pub const STACK_FRAME_MIN_SIZE: usize = if cfg!(target_pointer_width = "64") { 112 } else { 16 };
pub const __SIGNAL_FRAMESIZE: usize = if cfg!(target_pointer_width = "64") { 128 } else { 64 };
#[cfg(target_pointer_width = "64")]
pub const __SIGNAL_FRAMESIZE32: usize = 64;

// CONFIG_PPC64_ELF_ABI_V2 selects the alternate 32-byte frame layout.
pub const STACK_USER_INT_FRAME_SIZE: usize = core::mem::size_of::<PtRegs>() + STACK_FRAME_MIN_SIZE;
pub const STACK_INT_FRAME_REGS: usize = STACK_FRAME_MIN_SIZE;
pub const STACK_INT_FRAME_MARKER: usize = STACK_FRAME_MIN_SIZE - if cfg!(target_pointer_width = "64") { 16 } else { 8 };
pub const STACK_SWITCH_FRAME_SIZE: usize = STACK_USER_INT_FRAME_SIZE;
pub const STACK_SWITCH_FRAME_REGS: usize = STACK_FRAME_MIN_SIZE;
pub const STACK_INT_FRAME_SIZE: usize = KERNEL_REDZONE_SIZE + STACK_USER_INT_FRAME_SIZE;
pub const STACK_INT_FRAME_MARKER_LONGS: usize = STACK_INT_FRAME_MARKER / core::mem::size_of::<usize>();

// External dependencies supplied by the surrounding kernel translation.
extern "C" {
    pub fn set_thread_flag(flag: usize);
    pub fn task_stack_page(task: *mut core::ffi::c_void) -> usize;
    pub static mut current: *mut core::ffi::c_void;
}

#[inline]
pub unsafe fn set_return_regs_changed() {
    // CONFIG_PPC_BOOK3S_64: WRITE_ONCE(local_paca->hsrr_valid, 0) and srr_valid.
}

#[inline]
pub unsafe fn regs_set_return_ip(regs: *mut PtRegsFields, ip: usize) {
    (*regs).nip = ip;
    set_return_regs_changed();
}

#[inline]
pub unsafe fn regs_set_return_msr(regs: *mut PtRegsFields, msr: usize) {
    (*regs).msr = msr;
    set_return_regs_changed();
}

#[inline]
pub unsafe fn regs_add_return_ip(regs: *mut PtRegsFields, offset: isize) {
    regs_set_return_ip(regs, ((*regs).nip as isize).wrapping_add(offset) as usize);
}

#[inline] pub unsafe fn instruction_pointer(regs: *mut PtRegsFields) -> usize { (*regs).nip }
#[inline] pub unsafe fn instruction_pointer_set(regs: *mut PtRegsFields, val: usize) { regs_set_return_ip(regs, val); }
#[inline] pub unsafe fn user_stack_pointer(regs: *mut PtRegsFields) -> usize { (*regs).gpr[1] }
#[inline] pub unsafe fn frame_pointer(_regs: *mut PtRegsFields) -> usize { 0 }

#[inline] pub unsafe fn user_mode(regs: *mut PtRegsFields) -> bool { ((*regs).msr & MSR_PR) != 0 }
#[inline] pub unsafe fn kernel_stack_pointer(regs: *mut PtRegsFields) -> usize { (*regs).gpr[1] }

pub const TRAP_FLAGS_MASK: usize = if cfg!(target_pointer_width = "64") { 0x1 } else { 0xf };
#[inline] pub unsafe fn trap(regs: *mut PtRegsFields) -> usize { (*regs).trap & !TRAP_FLAGS_MASK }
#[inline] pub unsafe fn set_trap(regs: *mut PtRegsFields, val: usize) { (*regs).trap = ((*regs).trap & TRAP_FLAGS_MASK) | (val & !TRAP_FLAGS_MASK); }
#[inline] pub unsafe fn trap_is_scv(regs: *mut PtRegsFields) -> bool { cfg!(target_pointer_width = "64") && trap(regs) == 0x3000 }
#[inline] pub unsafe fn trap_is_unsupported_scv(regs: *mut PtRegsFields) -> bool { cfg!(target_pointer_width = "64") && trap(regs) == 0x7ff0 }
#[inline] pub unsafe fn trap_is_syscall(regs: *mut PtRegsFields) -> bool { trap_is_scv(regs) || trap(regs) == 0xc00 }
#[inline] pub unsafe fn trap_norestart(regs: *mut PtRegsFields) -> bool { (*regs).trap & 1 != 0 }
#[inline] pub unsafe fn set_trap_norestart(regs: *mut PtRegsFields) { (*regs).trap |= 1; }

#[inline] pub unsafe fn is_syscall_success(regs: *mut PtRegsFields) -> bool { if trap_is_scv(regs) { !is_err_value((*regs).gpr[3]) } else { (*regs).ccr & 0x1000_0000 == 0 } }
#[inline] pub unsafe fn regs_return_value(regs: *mut PtRegsFields) -> isize { if trap_is_scv(regs) || is_syscall_success(regs) { (*regs).gpr[3] as isize } else { -((*regs).gpr[3] as isize) } }
#[inline] pub unsafe fn regs_set_return_value(regs: *mut PtRegsFields, rc: usize) { (*regs).gpr[3] = rc; }
#[inline] pub fn cpu_has_msr_ri() -> bool { !cfg!(any()) }
#[inline] pub unsafe fn regs_is_unrecoverable(regs: *mut PtRegsFields) -> bool { cpu_has_msr_ri() && (*regs).msr & MSR_RI == 0 }
#[inline] pub unsafe fn regs_set_recoverable(regs: *mut PtRegsFields) { if cpu_has_msr_ri() { regs_set_return_msr(regs, (*regs).msr | MSR_RI); } }
#[inline] pub unsafe fn regs_set_unrecoverable(regs: *mut PtRegsFields) { if cpu_has_msr_ri() { regs_set_return_msr(regs, (*regs).msr & !MSR_RI); } }

pub const NR_REG_ARGUMENTS: usize = 8;
pub const ARCH_HAS_USER_SINGLE_STEP_REPORT: bool = true;
pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(PtRegsFields, dsisr);

extern "C" { pub fn regs_query_register_offset(name: *const core::ffi::c_char) -> i32; pub fn regs_query_register_name(offset: u32) -> *const core::ffi::c_char; }
#[inline] pub unsafe fn regs_get_register(regs: *mut PtRegsFields, offset: usize) -> usize { if offset > MAX_REG_OFFSET { 0 } else { *((regs as *mut u8).add(offset) as *const usize) } }
#[inline] pub unsafe fn regs_within_kernel_stack(regs: *mut PtRegsFields, addr: usize) -> bool { (addr & !(THREAD_SIZE - 1)) == ((*regs).gpr[1] & !(THREAD_SIZE - 1)) }
#[inline] pub unsafe fn regs_get_kernel_stack_nth(regs: *mut PtRegsFields, n: usize) -> usize { let addr = ((*regs).gpr[1] as *mut usize).add(n); if regs_within_kernel_stack(regs, addr as usize) { *addr } else { 0 } }
#[inline] pub unsafe fn regs_get_kernel_argument(regs: *mut PtRegsFields, n: usize) -> usize { if n < NR_REG_ARGUMENTS { regs_get_register(regs, core::mem::offset_of!(PtRegsFields, gpr) + (3 + n) * core::mem::size_of::<usize>()) } else { 0 } }

#[cfg(not(target_pointer_width = "64"))] pub const PT_SOFTE: usize = PT_MQ;
#[cfg(target_pointer_width = "64")] pub const PT_FPSCR32: usize = PT_FPR0 + 2 * 32 + 1;
#[cfg(target_pointer_width = "64")] pub const PT_VR0_32: usize = 164;
#[cfg(target_pointer_width = "64")] pub const PT_VSCR_32: usize = PT_VR0 + 32 * 4 + 3;
#[cfg(target_pointer_width = "64")] pub const PT_VRSAVE_32: usize = PT_VR0 + 33 * 4;
#[cfg(target_pointer_width = "64")] pub const PT_VSR0_32: usize = 300;

// Symbols supplied by included kernel headers.
extern "C" {
    fn is_err_value(value: usize) -> bool;
    static MSR_PR: usize;
    static MSR_RI: usize;
    static THREAD_SIZE: usize;
    static PT_MQ: usize;
    static PT_FPR0: usize;
    static PT_VR0: usize;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
