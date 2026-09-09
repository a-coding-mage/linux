/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// `UM_FRAME_SIZE`, `HOST_*`, and `faultinfo`.

pub const MAX_REG_OFFSET: usize = UM_FRAME_SIZE;
pub const MAX_REG_NR: usize = MAX_REG_OFFSET / core::mem::size_of::<usize>();

macro_rules! REGS_IP { ($r:expr) => { ($r)[HOST_IP] }; }
macro_rules! REGS_SP { ($r:expr) => { ($r)[HOST_SP] }; }
macro_rules! REGS_EFLAGS { ($r:expr) => { ($r)[HOST_EFLAGS] }; }
macro_rules! REGS_AX { ($r:expr) => { ($r)[HOST_AX] }; }
macro_rules! REGS_BX { ($r:expr) => { ($r)[HOST_BX] }; }
macro_rules! REGS_CX { ($r:expr) => { ($r)[HOST_CX] }; }
macro_rules! REGS_DX { ($r:expr) => { ($r)[HOST_DX] }; }
macro_rules! REGS_SI { ($r:expr) => { ($r)[HOST_SI] }; }
macro_rules! REGS_DI { ($r:expr) => { ($r)[HOST_DI] }; }
macro_rules! REGS_BP { ($r:expr) => { ($r)[HOST_BP] }; }
macro_rules! REGS_CS { ($r:expr) => { ($r)[HOST_CS] }; }
macro_rules! REGS_SS { ($r:expr) => { ($r)[HOST_SS] }; }
macro_rules! REGS_DS { ($r:expr) => { ($r)[HOST_DS] }; }
macro_rules! REGS_ES { ($r:expr) => { ($r)[HOST_ES] }; }

macro_rules! UPT_IP { ($r:expr) => { REGS_IP!((&$r).gp) }; }
macro_rules! UPT_SP { ($r:expr) => { REGS_SP!((&$r).gp) }; }
macro_rules! UPT_EFLAGS { ($r:expr) => { REGS_EFLAGS!((&$r).gp) }; }
macro_rules! UPT_AX { ($r:expr) => { REGS_AX!((&$r).gp) }; }
macro_rules! UPT_BX { ($r:expr) => { REGS_BX!((&$r).gp) }; }
macro_rules! UPT_CX { ($r:expr) => { REGS_CX!((&$r).gp) }; }
macro_rules! UPT_DX { ($r:expr) => { REGS_DX!((&$r).gp) }; }
macro_rules! UPT_SI { ($r:expr) => { REGS_SI!((&$r).gp) }; }
macro_rules! UPT_DI { ($r:expr) => { REGS_DI!((&$r).gp) }; }
macro_rules! UPT_BP { ($r:expr) => { REGS_BP!((&$r).gp) }; }
macro_rules! UPT_CS { ($r:expr) => { REGS_CS!((&$r).gp) }; }
macro_rules! UPT_SS { ($r:expr) => { REGS_SS!((&$r).gp) }; }
macro_rules! UPT_DS { ($r:expr) => { REGS_DS!((&$r).gp) }; }
macro_rules! UPT_ES { ($r:expr) => { REGS_ES!((&$r).gp) }; }

// On i386, the declarations from `ptrace_32.h` are selected; otherwise those
// from `ptrace_64.h` are selected.  The corresponding target-specific items
// are supplied by the surrounding translation.

extern "C" {
    pub static mut host_fp_size: usize;
}

#[repr(C)]
pub struct uml_pt_regs {
    pub gp: [usize; MAX_REG_NR],
    pub faultinfo: faultinfo,
    pub syscall: isize,
    pub is_user: i32,

    /* Dynamically sized FP registers (holds an XSTATE) */
    pub fp: [usize; 0],
}

macro_rules! EMPTY_UML_PT_REGS { () => { {} }; }

macro_rules! UPT_SYSCALL_NR { ($r:expr) => { (&$r).syscall }; }
macro_rules! UPT_FAULTINFO { ($r:expr) => { &(&$r).faultinfo }; }
macro_rules! UPT_IS_USER { ($r:expr) => { (&$r).is_user }; }

extern "C" {
    pub fn arch_init_registers(pid: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
