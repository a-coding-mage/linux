/* SPDX-License-Identifier: GPL-2.0-or-later */

/* Dependency: <linux/regset.h> */

/*
 * Set of msr bits that gdb can change on behalf of a process.
 * Build-time CONFIG_PPC_ADV_DEBUG_REGS selects the corresponding definition.
 */
#[cfg(CONFIG_PPC_ADV_DEBUG_REGS)]
pub const MSR_DEBUGCHANGE: _ = 0;
#[cfg(not(CONFIG_PPC_ADV_DEBUG_REGS))]
pub const MSR_DEBUGCHANGE: _ = MSR_SE | MSR_BE;

/* Max register writeable via put_reg. */
#[cfg(CONFIG_PPC32)]
pub const PT_MAX_PUT_REG: _ = PT_MQ;
#[cfg(not(CONFIG_PPC32))]
pub const PT_MAX_PUT_REG: _ = PT_CCR;

#[macro_export]
macro_rules! TVSO { ($f:ident) => { core::mem::offset_of!(thread_vr_state, $f) }; }
#[macro_export]
macro_rules! TFSO { ($f:ident) => { core::mem::offset_of!(thread_fp_state, $f) }; }
#[macro_export]
macro_rules! TSO { ($f:ident) => { core::mem::offset_of!(thread_struct, $f) }; }

/* These are our native regset flavors. */
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum powerpc_regset {
    REGSET_GPR,
    REGSET_FPR,
    #[cfg(CONFIG_ALTIVEC)]
    REGSET_VMX,
    #[cfg(CONFIG_VSX)]
    REGSET_VSX,
    #[cfg(CONFIG_SPE)]
    REGSET_SPE,
    #[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
    REGSET_TM_CGPR,
    #[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
    REGSET_TM_CFPR,
    #[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
    REGSET_TM_CVMX,
    #[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
    REGSET_TM_CVSX,
    #[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
    REGSET_TM_SPR,
    #[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
    REGSET_TM_CTAR,
    #[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
    REGSET_TM_CPPR,
    #[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
    REGSET_TM_CDSCR,
    #[cfg(CONFIG_PPC64)]
    REGSET_PPR,
    #[cfg(CONFIG_PPC64)]
    REGSET_DSCR,
    #[cfg(CONFIG_PPC_BOOK3S_64)]
    REGSET_TAR,
    #[cfg(CONFIG_PPC_BOOK3S_64)]
    REGSET_EBB,
    #[cfg(CONFIG_PPC_BOOK3S_64)]
    REGSET_PMR,
    #[cfg(CONFIG_PPC_BOOK3S_64)]
    REGSET_DEXCR,
    #[cfg(all(CONFIG_PPC_BOOK3S_64, CONFIG_CHECKPOINT_RESTORE))]
    REGSET_HASHKEYR,
    #[cfg(CONFIG_PPC_MEM_KEYS)]
    REGSET_PKEY,
}

/* ptrace-(no)vsx */
extern "C" {
    pub static mut fpr_get: user_regset_get2_fn;
    pub fn fpr_set(target: *mut task_struct, regset: *const user_regset,
                   pos: u32, count: u32, kbuf: *const core::ffi::c_void,
                   ubuf: *const core::ffi::c_void) -> i32;

    /* ptrace-vsx */
    pub fn vsr_active(target: *mut task_struct, regset: *const user_regset) -> i32;
    pub static mut vsr_get: user_regset_get2_fn;
    pub fn vsr_set(target: *mut task_struct, regset: *const user_regset,
                   pos: u32, count: u32, kbuf: *const core::ffi::c_void,
                   ubuf: *const core::ffi::c_void) -> i32;

    /* ptrace-altivec */
    pub fn vr_active(target: *mut task_struct, regset: *const user_regset) -> i32;
    pub static mut vr_get: user_regset_get2_fn;
    pub fn vr_set(target: *mut task_struct, regset: *const user_regset,
                  pos: u32, count: u32, kbuf: *const core::ffi::c_void,
                  ubuf: *const core::ffi::c_void) -> i32;

    /* ptrace-spe */
    pub fn evr_active(target: *mut task_struct, regset: *const user_regset) -> i32;
    pub static mut evr_get: user_regset_get2_fn;
    pub fn evr_set(target: *mut task_struct, regset: *const user_regset,
                   pos: u32, count: u32, kbuf: *const core::ffi::c_void,
                   ubuf: *const core::ffi::c_void) -> i32;

    /* ptrace */
    pub fn gpr32_get_common(target: *mut task_struct, regset: *const user_regset,
                            to: membuf, regs: *mut libc::c_ulong) -> i32;
    pub fn gpr32_set_common(target: *mut task_struct, regset: *const user_regset,
                            pos: u32, count: u32, kbuf: *const core::ffi::c_void,
                            ubuf: *const core::ffi::c_void, regs: *mut libc::c_ulong) -> i32;

    #[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
    pub fn flush_tmregs_to_thread(tsk: *mut task_struct);

    pub fn tm_cgpr_active(target: *mut task_struct, regset: *const user_regset) -> i32;
    pub static mut tm_cgpr_get: user_regset_get2_fn;
    pub fn tm_cgpr_set(target: *mut task_struct, regset: *const user_regset, pos: u32, count: u32, kbuf: *const core::ffi::c_void, ubuf: *const core::ffi::c_void) -> i32;
    pub fn tm_cfpr_active(target: *mut task_struct, regset: *const user_regset) -> i32;
    pub static mut tm_cfpr_get: user_regset_get2_fn;
    pub fn tm_cfpr_set(target: *mut task_struct, regset: *const user_regset, pos: u32, count: u32, kbuf: *const core::ffi::c_void, ubuf: *const core::ffi::c_void) -> i32;
    pub fn tm_cvmx_active(target: *mut task_struct, regset: *const user_regset) -> i32;
    pub static mut tm_cvmx_get: user_regset_get2_fn;
    pub fn tm_cvmx_set(target: *mut task_struct, regset: *const user_regset, pos: u32, count: u32, kbuf: *const core::ffi::c_void, ubuf: *const core::ffi::c_void) -> i32;
    pub fn tm_cvsx_active(target: *mut task_struct, regset: *const user_regset) -> i32;
    pub static mut tm_cvsx_get: user_regset_get2_fn;
    pub fn tm_cvsx_set(target: *mut task_struct, regset: *const user_regset, pos: u32, count: u32, kbuf: *const core::ffi::c_void, ubuf: *const core::ffi::c_void) -> i32;
    pub fn tm_spr_active(target: *mut task_struct, regset: *const user_regset) -> i32;
    pub static mut tm_spr_get: user_regset_get2_fn;
    pub fn tm_spr_set(target: *mut task_struct, regset: *const user_regset, pos: u32, count: u32, kbuf: *const core::ffi::c_void, ubuf: *const core::ffi::c_void) -> i32;
    pub fn tm_tar_active(target: *mut task_struct, regset: *const user_regset) -> i32;
    pub static mut tm_tar_get: user_regset_get2_fn;
    pub fn tm_tar_set(target: *mut task_struct, regset: *const user_regset, pos: u32, count: u32, kbuf: *const core::ffi::c_void, ubuf: *const core::ffi::c_void) -> i32;
    pub fn tm_ppr_active(target: *mut task_struct, regset: *const user_regset) -> i32;
    pub static mut tm_ppr_get: user_regset_get2_fn;
    pub fn tm_ppr_set(target: *mut task_struct, regset: *const user_regset, pos: u32, count: u32, kbuf: *const core::ffi::c_void, ubuf: *const core::ffi::c_void) -> i32;
    pub fn tm_dscr_active(target: *mut task_struct, regset: *const user_regset) -> i32;
    pub static mut tm_dscr_get: user_regset_get2_fn;
    pub fn tm_dscr_set(target: *mut task_struct, regset: *const user_regset, pos: u32, count: u32, kbuf: *const core::ffi::c_void, ubuf: *const core::ffi::c_void) -> i32;
    pub static mut tm_cgpr32_get: user_regset_get2_fn;
    pub fn tm_cgpr32_set(target: *mut task_struct, regset: *const user_regset, pos: u32, count: u32, kbuf: *const core::ffi::c_void, ubuf: *const core::ffi::c_void) -> i32;

    /* ptrace-view */
    pub fn ptrace_get_reg(task: *mut task_struct, regno: i32, data: *mut libc::c_ulong) -> i32;
    pub fn ptrace_put_reg(task: *mut task_struct, regno: i32, data: libc::c_ulong) -> i32;
    pub static user_ppc_native_view: user_regset_view;

    /* ptrace-fpu */
    pub fn ptrace_get_fpr(child: *mut task_struct, index: i32, data: *mut libc::c_ulong) -> i32;
    pub fn ptrace_put_fpr(child: *mut task_struct, index: i32, data: libc::c_ulong) -> i32;

    /* ptrace-(no)adv */
    pub fn ppc_gethwdinfo(dbginfo: *mut ppc_debug_info);
    pub fn ptrace_get_debugreg(child: *mut task_struct, addr: libc::c_ulong, datalp: *mut libc::c_ulong) -> i32;
    pub fn ptrace_set_debugreg(task: *mut task_struct, addr: libc::c_ulong, data: libc::c_ulong) -> i32;
    pub fn ppc_set_hwdebug(child: *mut task_struct, bp_info: *mut ppc_hw_breakpoint) -> libc::c_long;
    pub fn ppc_del_hwdebug(child: *mut task_struct, data: libc::c_long) -> libc::c_long;
}

#[cfg(not(CONFIG_PPC_TRANSACTIONAL_MEM))]
#[inline]
pub unsafe fn flush_tmregs_to_thread(_tsk: *mut task_struct) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
