/* SPDX-License-Identifier: GPL-2.0 */
/* S390 version; translated from the C header. */

pub const PIF_SYSCALL: u32 = 0;
pub const PIF_PSW_ADDR_ADJUSTED: u32 = 1;
pub const PIF_SYSCALL_RET_SET: u32 = 2;
pub const PIF_FTRACE_FULL_REGS: u32 = 3;

pub const _PIF_SYSCALL: usize = 1usize << PIF_SYSCALL;
pub const _PIF_ADDR_PSW_ADJUSTED: usize = 1usize << PIF_PSW_ADDR_ADJUSTED;
pub const _PIF_SYSCALL_RET_SET: usize = 1usize << PIF_SYSCALL_RET_SET;
pub const _PIF_FTRACE_FULL_REGS: usize = 1usize << PIF_FTRACE_FULL_REGS;

pub const PSW32_MASK_PER: usize = 0x40000000;
pub const PSW32_MASK_DAT: usize = 0x04000000;
pub const PSW32_MASK_IO: usize = 0x02000000;
pub const PSW32_MASK_EXT: usize = 0x01000000;
pub const PSW32_MASK_KEY: usize = 0x00F00000;
pub const PSW32_MASK_BASE: usize = 0x00080000;
pub const PSW32_MASK_MCHECK: usize = 0x00040000;
pub const PSW32_MASK_WAIT: usize = 0x00020000;
pub const PSW32_MASK_PSTATE: usize = 0x00010000;
pub const PSW32_MASK_ASC: usize = 0x0000C000;
pub const PSW32_MASK_CC: usize = 0x00003000;
pub const PSW32_MASK_PM: usize = 0x00000f00;
pub const PSW32_MASK_RI: usize = 0x00000080;
pub const PSW32_ADDR_AMODE: usize = 0x80000000;
pub const PSW32_ADDR_INSN: usize = 0x7FFFFFFF;
pub const PSW32_DEFAULT_KEY: usize = (PAGE_DEFAULT_ACC as usize) << 20;
pub const PSW32_ASC_PRIMARY: usize = 0x00000000;
pub const PSW32_ASC_ACCREG: usize = 0x00004000;
pub const PSW32_ASC_SECONDARY: usize = 0x00008000;
pub const PSW32_ASC_HOME: usize = 0x0000C000;
pub const PSW_DEFAULT_KEY: usize = (PAGE_DEFAULT_ACC as usize) << 52;
pub const PSW_KERNEL_BITS: usize = PSW_DEFAULT_KEY | PSW_MASK_BASE | PSW_ASC_HOME | PSW_MASK_EA | PSW_MASK_BA | PSW_MASK_DAT;
pub const PSW_USER_BITS: usize = PSW_MASK_DAT | PSW_MASK_IO | PSW_MASK_EXT | PSW_DEFAULT_KEY | PSW_MASK_BASE | PSW_MASK_MCHECK | PSW_MASK_PSTATE | PSW_ASC_PRIMARY;

#[repr(C)]
pub struct psw_bits {
    pub raw: [u64; 2],
}

pub const PSW_BITS_AMODE_24BIT: u32 = 0;
pub const PSW_BITS_AMODE_31BIT: u32 = 1;
pub const PSW_BITS_AMODE_64BIT: u32 = 3;
pub const PSW_BITS_AS_PRIMARY: u32 = 0;
pub const PSW_BITS_AS_ACCREG: u32 = 1;
pub const PSW_BITS_AS_SECONDARY: u32 = 2;
pub const PSW_BITS_AS_HOME: u32 = 3;

#[repr(C, align(8))]
pub struct psw32_t { pub mask: u32, pub addr: u32 }

pub const PGM_INT_CODE_MASK: u32 = 0x7f;
pub const PGM_INT_CODE_PER: u32 = 0x80;

#[repr(C)]
pub union pt_regs_first { pub user_regs: user_pt_regs, pub frame: pt_regs_frame }
#[repr(C)]
pub struct pt_regs_frame { pub args: [u64; 1], pub psw: psw_t, pub gprs: [u64; NUM_GPRS as usize] }
#[repr(C)]
pub union pt_regs_orig { pub orig_gpr2: u64, pub monitor_code: u64 }
#[repr(C)]
pub struct pt_regs_int { pub int_code: u32, pub int_parm: u32, pub int_parm_long: u64 }
#[repr(C)]
pub union pt_regs_event { pub regs: pt_regs_int, pub tpi_info: tpi_info }
#[repr(C)]
pub struct pt_regs {
    pub first: pt_regs_first,
    pub orig: pt_regs_orig,
    pub event: pt_regs_event,
    pub flags: u64,
    pub last_break: u64,
    pub cpu: u32,
    pub percpu_register: u8,
}

#[repr(C)] pub struct per_regs { pub control: u64, pub start: u64, pub end: u64 }
#[repr(C)] pub struct per_event { pub cause: u16, pub address: u64, pub paid: u8 }
#[repr(C)] pub struct per_struct_kernel { pub cr9: u64, pub cr10: u64, pub cr11: u64, pub bits: u64, pub starting_addr: u64, pub ending_addr: u64, pub perc_atmid: u16, pub address: u64, pub access_id: u8 }

pub const PER_EVENT_MASK: usize = 0xEB000000;
pub const PER_EVENT_BRANCH: usize = 0x80000000;
pub const PER_EVENT_IFETCH: usize = 0x40000000;
pub const PER_EVENT_STORE: usize = 0x20000000;
pub const PER_EVENT_STORE_REAL: usize = 0x08000000;
pub const PER_EVENT_TRANSACTION_END: usize = 0x02000000;
pub const PER_EVENT_NULLIFICATION: usize = 0x01000000;
pub const PER_CONTROL_MASK: usize = 0x00e00000;
pub const PER_CONTROL_BRANCH_ADDRESS: usize = 0x00800000;
pub const PER_CONTROL_SUSPENSION: usize = 0x00400000;
pub const PER_CONTROL_ALTERATION: usize = 0x00200000;

#[inline] pub unsafe fn set_pt_regs_flag(regs: *mut pt_regs, flag: i32) { (*regs).flags |= 1u64 << flag; }
#[inline] pub unsafe fn clear_pt_regs_flag(regs: *mut pt_regs, flag: i32) { (*regs).flags &= !(1u64 << flag); }
#[inline] pub unsafe fn test_pt_regs_flag(regs: *mut pt_regs, flag: i32) -> i32 { (((*regs).flags & (1u64 << flag)) != 0) as i32 }
#[inline] pub unsafe fn test_and_clear_pt_regs_flag(regs: *mut pt_regs, flag: i32) -> i32 { let ret = test_pt_regs_flag(regs, flag); clear_pt_regs_flag(regs, flag); ret }

#[repr(C)] pub struct task_struct;
extern "C" { pub fn update_cr_regs(task: *mut task_struct); pub fn regs_query_register_offset(name: *const core::ffi::c_char) -> i32; pub fn regs_query_register_name(offset: u32) -> *const core::ffi::c_char; }
pub const NR_REG_ARGUMENTS: u32 = 5;

#[inline] pub const fn arch_has_single_step() -> bool { true }
#[inline] pub const fn arch_has_block_step() -> bool { true }

#[inline]
pub unsafe fn user_mode(regs: *const pt_regs) -> bool {
    ((*regs).first.frame.psw /* pstate is represented by the PSW bitfield */.mask & PSW_MASK_PSTATE) != 0
}

#[inline]
pub unsafe fn regs_return_value(regs: *const pt_regs) -> u64 {
    (*regs).first.frame.gprs[2]
}

#[inline]
pub unsafe fn instruction_pointer(regs: *const pt_regs) -> u64 {
    (*regs).first.frame.psw.addr
}

#[inline]
pub unsafe fn instruction_pointer_set(regs: *mut pt_regs, val: u64) {
    (*regs).first.frame.psw.addr = val;
}

#[inline]
pub unsafe fn kernel_stack_pointer(regs: *const pt_regs) -> u64 {
    (*regs).first.frame.gprs[15]
}

#[inline]
pub unsafe fn user_stack_pointer(regs: *const pt_regs) -> u64 {
    (*regs).first.frame.gprs[15]
}

#[inline]
pub unsafe fn regs_get_register(regs: *const pt_regs, offset: u32) -> u64 {
    if offset >= NUM_GPRS as u32 { 0 } else { (*regs).first.frame.gprs[offset as usize] }
}

#[inline]
pub unsafe fn regs_within_kernel_stack(regs: *const pt_regs, addr: u64) -> bool {
    let ksp = kernel_stack_pointer(regs);
    (addr & !((THREAD_SIZE as u64) - 1)) == (ksp & !((THREAD_SIZE as u64) - 1))
}

#[inline]
pub unsafe fn regs_get_kernel_stack_nth(regs: *const pt_regs, n: u32) -> u64 {
    let addr = kernel_stack_pointer(regs).wrapping_add((n as u64).wrapping_mul(core::mem::size_of::<u64>() as u64));
    if !regs_within_kernel_stack(regs, addr) { return 0; }
    core::ptr::read_volatile(addr as *const u64)
}

#[inline]
pub unsafe fn regs_get_kernel_argument(regs: *const pt_regs, mut n: u32) -> u64 {
    let argoffset = (STACK_FRAME_OVERHEAD as u64) / core::mem::size_of::<u64>() as u64;
    if n < NR_REG_ARGUMENTS { return regs_get_register(regs, 2 + n); }
    n -= NR_REG_ARGUMENTS;
    regs_get_kernel_stack_nth(regs, (argoffset + n as u64) as u32)
}

#[inline]
pub unsafe fn regs_set_return_value(regs: *mut pt_regs, rc: u64) {
    (*regs).first.frame.gprs[2] = rc;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
