/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994, 95, 96, 97, 98, 99, 2000 by Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

/*
 * This struct defines the way the registers are stored on the stack during a
 * system call/exception. As usual the registers k0/k1 aren't being saved.
 *
 * If you add a register here, also add it to regoffset_table[] in
 * arch/mips/kernel/ptrace.c.
 */
#[repr(C, align(8))]
pub struct pt_regs {
    #[cfg(CONFIG_32BIT)]
    /* Saved syscall stack arguments; entries 0-3 unused. */
    pub args: [usize; 8],

    /* Saved main processor registers. */
    pub regs: [usize; 32],

    /* Saved special registers. */
    pub cp0_status: usize,
    pub hi: usize,
    pub lo: usize,
    #[cfg(CONFIG_CPU_HAS_SMARTMIPS)]
    pub acx: usize,
    pub cp0_badvaddr: usize,
    pub cp0_cause: usize,
    pub cp0_epc: usize,
    #[cfg(CONFIG_CPU_CAVIUM_OCTEON)]
    pub mpl: [u64; 6], // MTM{0-5}
    #[cfg(CONFIG_CPU_CAVIUM_OCTEON)]
    pub mtp: [u64; 6], // MTP{0-5}
    pub __last: [usize; 0],
}

#[inline]
pub unsafe fn kernel_stack_pointer(regs: *mut pt_regs) -> usize {
    (*regs).regs[29]
}

#[inline]
pub unsafe fn instruction_pointer_set(regs: *mut pt_regs, val: usize) {
    (*regs).cp0_epc = val;
    (*regs).cp0_cause &= !CAUSEF_BD;
}

/* Query offset/name of register from its name/offset */
extern "C" {
    pub fn regs_query_register_offset(name: *const core::ffi::c_char) -> i32;
}

pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(pt_regs, __last) - core::mem::size_of::<usize>();

/**
 * regs_get_register() - get register value from its offset
 * @regs:       pt_regs from which register value is gotten.
 * @offset:     offset number of the register.
 */
#[inline]
pub unsafe fn regs_get_register(regs: *mut pt_regs, offset: u32) -> usize {
    if offset as usize > MAX_REG_OFFSET { return 0; }
    *((regs as *mut u8).add(offset as usize) as *const usize)
}

#[inline]
pub unsafe fn regs_within_kernel_stack(regs: *mut pt_regs, addr: usize) -> i32 {
    ((addr & !(THREAD_SIZE - 1)) == (kernel_stack_pointer(regs) & !(THREAD_SIZE - 1))) as i32
}

#[inline]
pub unsafe fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: u32) -> usize {
    let addr = (kernel_stack_pointer(regs) as *mut usize).add(n as usize);
    if regs_within_kernel_stack(regs, addr as usize) != 0 { *addr } else { 0 }
}

pub struct task_struct;

extern "C" {
    pub fn ptrace_getregs(child: *mut task_struct, data: *mut user_pt_regs) -> i32;
    pub fn ptrace_setregs(child: *mut task_struct, data: *mut user_pt_regs) -> i32;
    pub fn ptrace_getfpregs(child: *mut task_struct, data: *mut u32) -> i32;
    pub fn ptrace_setfpregs(child: *mut task_struct, data: *mut u32) -> i32;
    pub fn ptrace_get_watch_regs(child: *mut task_struct, addr: *mut pt_watch_regs) -> i32;
    pub fn ptrace_set_watch_regs(child: *mut task_struct, addr: *mut pt_watch_regs) -> i32;
}

/* Does the process account for user or for system time? */
#[inline]
pub unsafe fn user_mode(regs: *const pt_regs) -> bool {
    ((*regs).cp0_status & KU_MASK) == KU_USER
}

#[inline]
pub unsafe fn is_syscall_success(regs: *const pt_regs) -> bool { (*regs).regs[7] == 0 }

#[inline]
pub unsafe fn regs_return_value(regs: *const pt_regs) -> isize {
    if is_syscall_success(regs) || !user_mode(regs) { (*regs).regs[2] as isize } else { -((*regs).regs[2] as isize) }
}

#[inline]
pub unsafe fn instruction_pointer(regs: *const pt_regs) -> usize { (*regs).cp0_epc }
extern "C" { pub fn exception_ip(regs: *mut pt_regs) -> usize; }

extern "C" { pub fn syscall_trace_enter(regs: *mut pt_regs) -> isize; pub fn syscall_trace_leave(regs: *mut pt_regs); }
extern "C" { pub fn die(str_: *const core::ffi::c_char, regs: *mut pt_regs) -> !; }

#[inline]
pub unsafe fn die_if_kernel(str_: *const core::ffi::c_char, regs: *mut pt_regs) {
    if !user_mode(regs) { die(str_, regs); }
}

// Equivalent of the C current_pt_regs() expression; the frame-address
// primitive is supplied by the target kernel/toolchain.
#[inline]
pub unsafe fn current_pt_regs() -> *mut pt_regs {
    let sp: usize;
    core::arch::asm!("move {}, $sp", out(reg) sp);
    (((sp | (THREAD_SIZE - 1)) + 1 - 32) as *mut pt_regs).sub(1)
}

#[inline]
pub unsafe fn user_stack_pointer(regs: *const pt_regs) -> usize { (*regs).regs[29] }

#[inline]
pub unsafe fn user_stack_pointer_set(regs: *mut pt_regs, val: usize) { (*regs).regs[29] = val; }

#[macro_export]
macro_rules! instruction_pointer_value { ($regs:expr) => { unsafe { (*($regs)).cp0_epc } }; }

#[macro_export]
macro_rules! exception_ip_value { ($regs:expr) => { unsafe { $crate::exception_ip($regs) } }; }

#[macro_export]
macro_rules! profile_pc { ($regs:expr) => { $crate::instruction_pointer_value!($regs) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
