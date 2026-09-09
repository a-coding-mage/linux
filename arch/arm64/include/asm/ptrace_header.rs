/* SPDX-License-Identifier: GPL-2.0-only */
/* Based on arch/arm/include/asm/ptrace.h */

/* Dependencies supplied by the corresponding kernel headers. */

pub const CurrentEL_EL1: u32 = 1 << 2;
pub const CurrentEL_EL2: u32 = 2 << 2;

pub const INIT_PSTATE_EL1: u64 = PSR_D_BIT | PSR_A_BIT | PSR_I_BIT | PSR_F_BIT | PSR_MODE_EL1h;

pub const GIC_PRIO_IRQON: u8 = GICV3_PRIO_UNMASKED;
pub const GIC_PRIO_IRQOFF: u8 = GICV3_PRIO_IRQ;
pub const GIC_PRIO_PSR_I_SET: u8 = GICV3_PRIO_PSR_I_SET;

pub const PSR_MODE_THREAD_BIT: u32 = 1 << 0;
pub const PSR_IL_BIT: u32 = 1 << 20;

pub const COMPAT_PTRACE_GETREGS: u32 = 12;
pub const COMPAT_PTRACE_SETREGS: u32 = 13;
pub const COMPAT_PTRACE_GET_THREAD_AREA: u32 = 22;
pub const COMPAT_PTRACE_SET_SYSCALL: u32 = 23;
pub const COMPAT_PTRACE_GETVFPREGS: u32 = 27;
pub const COMPAT_PTRACE_SETVFPREGS: u32 = 28;
pub const COMPAT_PTRACE_GETHBPREGS: u32 = 29;
pub const COMPAT_PTRACE_SETHBPREGS: u32 = 30;

pub const PSR_AA32_MODE_MASK: u32 = 0x0000001f;
pub const PSR_AA32_MODE_USR: u32 = 0x00000010;
pub const PSR_AA32_MODE_FIQ: u32 = 0x00000011;
pub const PSR_AA32_MODE_IRQ: u32 = 0x00000012;
pub const PSR_AA32_MODE_SVC: u32 = 0x00000013;
pub const PSR_AA32_MODE_ABT: u32 = 0x00000017;
pub const PSR_AA32_MODE_HYP: u32 = 0x0000001a;
pub const PSR_AA32_MODE_UND: u32 = 0x0000001b;
pub const PSR_AA32_MODE_SYS: u32 = 0x0000001f;
pub const PSR_AA32_T_BIT: u32 = 0x00000020;
pub const PSR_AA32_F_BIT: u32 = 0x00000040;
pub const PSR_AA32_I_BIT: u32 = 0x00000080;
pub const PSR_AA32_A_BIT: u32 = 0x00000100;
pub const PSR_AA32_E_BIT: u32 = 0x00000200;
pub const PSR_AA32_PAN_BIT: u32 = 0x00400000;
pub const PSR_AA32_SSBS_BIT: u32 = 0x00800000;
pub const PSR_AA32_DIT_BIT: u32 = 0x01000000;
pub const PSR_AA32_Q_BIT: u32 = 0x08000000;
pub const PSR_AA32_V_BIT: u32 = 0x10000000;
pub const PSR_AA32_C_BIT: u32 = 0x20000000;
pub const PSR_AA32_Z_BIT: u32 = 0x40000000;
pub const PSR_AA32_N_BIT: u32 = 0x80000000;
pub const PSR_AA32_IT_MASK: u32 = 0x0600fc00;
pub const PSR_AA32_GE_MASK: u32 = 0x000f0000;

#[cfg(feature = "CONFIG_CPU_BIG_ENDIAN")]
pub const PSR_AA32_ENDSTATE: u32 = PSR_AA32_E_BIT;
#[cfg(not(feature = "CONFIG_CPU_BIG_ENDIAN"))]
pub const PSR_AA32_ENDSTATE: u32 = 0;

pub const COMPAT_PSR_DIT_BIT: u64 = 0x00200000;
pub const COMPAT_PT_TEXT_ADDR: usize = 0x10000;
pub const COMPAT_PT_DATA_ADDR: usize = 0x10004;
pub const COMPAT_PT_TEXT_END_ADDR: usize = 0x10008;
pub const NO_SYSCALL: i32 = -1;
pub const COMPAT_USER_SZ: usize = 296;

#[inline]
pub unsafe fn compat_psr_to_pstate(psr: u64) -> u64 {
    let mut pstate = psr & !COMPAT_PSR_DIT_BIT;
    if psr & COMPAT_PSR_DIT_BIT != 0 { pstate |= PSR_AA32_DIT_BIT as u64; }
    pstate
}

#[inline]
pub unsafe fn pstate_to_compat_psr(pstate: u64) -> u64 {
    let mut psr = pstate & !(PSR_AA32_DIT_BIT as u64);
    if pstate & (PSR_AA32_DIT_BIT as u64) != 0 { psr |= COMPAT_PSR_DIT_BIT; }
    psr
}

#[repr(C)]
pub union PtRegsUnion {
    pub user_regs: user_pt_regs,
    pub raw: PtRegsRaw,
}
#[repr(C)] pub struct PtRegsRaw { pub regs: [u64; 31] }

#[repr(C)]
pub struct pt_regs {
    pub user: PtRegsUnion,
    pub sp: u64,
    pub pc: u64,
    pub pstate: u64,
    pub orig_x0: u64,
    pub syscallno: i32,
    pub pmr: u32,
    pub sdei_ttbr1: u64,
    pub stackframe: frame_record_meta,
}

#[inline]
pub unsafe fn in_syscall(regs: *const pt_regs) -> bool { (*regs).syscallno != NO_SYSCALL }
#[inline]
pub unsafe fn forget_syscall(regs: *mut pt_regs) { (*regs).syscallno = NO_SYSCALL; }

pub const MAX_REG_OFFSET: usize = core::mem::offset_of!(pt_regs, pstate);
#[inline] pub const fn arch_has_single_step() -> i32 { 1 }

#[inline]
pub unsafe fn user_mode(regs: *const pt_regs) -> bool { ((*regs).pstate & PSR_MODE_MASK) == PSR_MODE_EL0t }
#[inline]
pub unsafe fn compat_user_mode(regs: *const pt_regs) -> bool { ((*regs).pstate & (PSR_MODE32_BIT | PSR_MODE_MASK)) == (PSR_MODE32_BIT | PSR_MODE_EL0t) }
#[inline]
pub unsafe fn processor_mode(regs: *const pt_regs) -> u64 { (*regs).pstate & PSR_MODE_MASK }
#[inline]
pub unsafe fn irqs_priority_unmasked(regs: *const pt_regs) -> bool { system_uses_irq_prio_masking() && (*regs).pmr == GIC_PRIO_IRQON as u32 || !system_uses_irq_prio_masking() }
#[inline]
pub unsafe fn regs_irqs_disabled(regs: *const pt_regs) -> bool { (*regs).pstate & PSR_I_BIT != 0 || !irqs_priority_unmasked(regs) }
#[inline]
pub unsafe fn interrupts_enabled(regs: *const pt_regs) -> bool { !regs_irqs_disabled(regs) }
#[inline]
pub unsafe fn user_stack_pointer(regs: *mut pt_regs) -> u64 { if compat_user_mode(regs) { (*regs).sp } else { (*regs).sp } }

#[inline]
pub unsafe fn regs_get_register(regs: *mut pt_regs, mut offset: u32) -> u64 {
    let mut val = 0;
    offset >>= 3;
    match offset {
        0..=30 => val = (*regs).user.raw.regs[offset as usize],
        x if x == core::mem::offset_of!(pt_regs, sp) as u32 >> 3 => val = (*regs).sp,
        x if x == core::mem::offset_of!(pt_regs, pc) as u32 >> 3 => val = (*regs).pc,
        x if x == core::mem::offset_of!(pt_regs, pstate) as u32 >> 3 => val = (*regs).pstate,
        _ => {}
    }
    val
}

#[inline]
pub unsafe fn pt_regs_read_reg(regs: *const pt_regs, r: i32) -> u64 {
    if r == 31 { 0 } else { (*regs).user.raw.regs[r as usize] }
}
#[inline]
pub unsafe fn pt_regs_write_reg(regs: *mut pt_regs, r: i32, val: u64) {
    if r != 31 { (*regs).user.raw.regs[r as usize] = val; }
}
#[inline] pub unsafe fn kernel_stack_pointer(regs: *mut pt_regs) -> u64 { (*regs).sp }

#[inline]
pub unsafe fn regs_return_value(regs: *mut pt_regs) -> u64 {
    let mut val = (*regs).user.raw.regs[0];
    if compat_user_mode(regs) { val = (val as i64 as i32 as i64) as u64; }
    val
}
#[inline] pub unsafe fn regs_set_return_value(regs: *mut pt_regs, rc: u64) { (*regs).user.raw.regs[0] = rc; }

#[inline]
pub unsafe fn regs_get_kernel_argument(regs: *mut pt_regs, n: u32) -> u64 {
    if n < 8 { pt_regs_read_reg(regs, n as i32) } else { 0 }
}
#[inline] pub unsafe fn instruction_pointer(regs: *mut pt_regs) -> u64 { (*regs).pc }
#[inline] pub unsafe fn instruction_pointer_set(regs: *mut pt_regs, val: u64) { (*regs).pc = val; }
#[inline] pub unsafe fn frame_pointer(regs: *mut pt_regs) -> u64 { (*regs).user.raw.regs[29] }
#[inline] pub unsafe fn procedure_link_pointer(regs: *mut pt_regs) -> u64 { (*regs).user.raw.regs[30] }
#[inline] pub unsafe fn procedure_link_pointer_set(regs: *mut pt_regs, val: u64) { (*regs).user.raw.regs[30] = val; }

extern "C" {
    pub fn regs_query_register_offset(name: *const i8) -> i32;
    pub fn regs_get_kernel_stack_nth(regs: *mut pt_regs, n: u32) -> u64;
    pub fn valid_user_regs(regs: *mut user_pt_regs, task: *mut task_struct) -> i32;
    pub fn profile_pc(regs: *mut pt_regs) -> u64;
}

#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct user_pt_regs;
#[repr(C)] pub struct frame_record_meta;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
