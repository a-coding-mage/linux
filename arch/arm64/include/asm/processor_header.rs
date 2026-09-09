/* SPDX-License-Identifier: GPL-2.0-only */
/* Translation of arm64/include/asm/processor.h. */

pub const NET_IP_ALIGN: usize = 0;
pub const MTE_CTRL_GCR_USER_EXCL_SHIFT: usize = 0;
pub const MTE_CTRL_GCR_USER_EXCL_MASK: usize = 0xffff;
pub const MTE_CTRL_TCF_SYNC: usize = 1usize << 16;
pub const MTE_CTRL_TCF_ASYNC: usize = 1usize << 17;
pub const MTE_CTRL_TCF_ASYMM: usize = 1usize << 18;
pub const MTE_CTRL_STORE_ONLY: usize = 1usize << 19;

/* Build-time configuration and symbols supplied by the kernel are external dependencies. */
pub const DEFAULT_MAP_WINDOW_64: usize = 1usize << VA_BITS_MIN;
pub const TASK_SIZE_64: usize = 1usize << vabits_actual;
pub const TASK_SIZE_MAX: usize = 1usize << VA_BITS;

#[cfg(all(feature = "compat", feature = "arm64_64k_pages", feature = "kuser_helpers"))]
pub const TASK_SIZE_32: usize = 0x100000000usize;
#[cfg(all(feature = "compat", not(all(feature = "arm64_64k_pages", feature = "kuser_helpers"))))]
pub const TASK_SIZE_32: usize = 0x100000000usize - PAGE_SIZE;

#[repr(C)]
pub struct debug_info {
    #[cfg(feature = "have_hw_breakpoint")]
    pub suspended_step: core::ffi::c_int,
    #[cfg(feature = "have_hw_breakpoint")]
    pub bps_disabled: core::ffi::c_int,
    #[cfg(feature = "have_hw_breakpoint")]
    pub wps_disabled: core::ffi::c_int,
    #[cfg(feature = "have_hw_breakpoint")]
    pub hbp_break: [*mut perf_event; ARM_MAX_BRP],
    #[cfg(feature = "have_hw_breakpoint")]
    pub hbp_watch: [*mut perf_event; ARM_MAX_WRP],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum vec_type { ARM64_VEC_SVE = 0, ARM64_VEC_SME, ARM64_VEC_MAX }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum fp_type { FP_STATE_CURRENT, FP_STATE_FPSIMD, FP_STATE_SVE }

pub enum arm64_sve_state {}
pub enum arm64_sme_state {}

#[repr(C)]
pub struct cpu_context {
    pub x19: usize, pub x20: usize, pub x21: usize, pub x22: usize,
    pub x23: usize, pub x24: usize, pub x25: usize, pub x26: usize,
    pub x27: usize, pub x28: usize, pub fp: usize, pub sp: usize, pub pc: usize,
}

#[repr(C)]
pub struct thread_struct {
    pub cpu_context: cpu_context,
    pub uw: thread_user_whitelist,
    pub fp_type: fp_type,
    pub fpsimd_cpu: u32,
    pub sve_state: *mut arm64_sve_state,
    pub sme_state: *mut arm64_sme_state,
    pub vl: [u32; ARM64_VEC_MAX as usize],
    pub vl_onexec: [u32; ARM64_VEC_MAX as usize],
    pub fault_address: usize,
    pub fault_code: usize,
    pub debug: debug_info,
    pub kernel_fpsimd_state: *mut user_fpsimd_state,
    pub kernel_fpsimd_cpu: u32,
    #[cfg(feature = "arm64_ptr_auth")]
    pub keys_user: ptrauth_keys_user,
    #[cfg(all(feature = "arm64_ptr_auth", feature = "arm64_ptr_auth_kernel"))]
    pub keys_kernel: ptrauth_keys_kernel,
    #[cfg(feature = "arm64_mte")]
    pub mte_ctrl: u64,
    pub sctlr_user: u64, pub svcr: u64, pub tpidr2_el0: u64, pub por_el0: u64,
    #[cfg(feature = "arm64_gcs")]
    pub gcs_el0_mode: u32,
    #[cfg(feature = "arm64_gcs")]
    pub gcs_el0_locked: u32,
    #[cfg(feature = "arm64_gcs")]
    pub gcspr_el0: u64,
    #[cfg(feature = "arm64_gcs")]
    pub gcs_base: u64,
    #[cfg(feature = "arm64_gcs")]
    pub gcs_size: u64,
}

#[repr(C)]
pub struct thread_user_whitelist {
    pub tp_value: usize, pub tp2_value: usize, pub fpmr: u64, pub pad: usize,
    pub fpsimd_state: user_fpsimd_state,
}

#[inline]
pub unsafe fn thread_get_vl(thread: *mut thread_struct, ty: vec_type) -> u32 { (*thread).vl[ty as usize] }
#[inline] pub unsafe fn thread_get_sve_vl(t: *mut thread_struct) -> u32 { thread_get_vl(t, vec_type::ARM64_VEC_SVE) }
#[inline] pub unsafe fn thread_get_sme_vl(t: *mut thread_struct) -> u32 { thread_get_vl(t, vec_type::ARM64_VEC_SME) }
#[inline] pub unsafe fn thread_get_cur_vl(t: *mut thread_struct) -> u32 {
    if system_supports_sme() && ((*t).svcr & SVCR_SM_MASK) != 0 { thread_get_sme_vl(t) } else { thread_get_sve_vl(t) }
}

extern "C" {
    pub fn task_get_vl(task: *const task_struct, ty: vec_type) -> u32;
    pub fn task_set_vl(task: *mut task_struct, ty: vec_type, vl: usize);
    pub fn task_set_vl_onexec(task: *mut task_struct, ty: vec_type, vl: usize);
    pub fn task_get_vl_onexec(task: *const task_struct, ty: vec_type) -> u32;
    pub fn tls_preserve_current_state();
    pub fn __get_wchan(p: *mut task_struct) -> usize;
    pub fn update_sctlr_el1(sctlr: u64);
    pub fn cpu_switch_to(prev: *mut task_struct, next: *mut task_struct) -> *mut task_struct;
    pub static mut arm64_dma_phys_limit: usize;
    pub static mut signal_minsigstksz: usize;
    pub fn minsigstksz_setup();
}

#[inline] pub unsafe fn task_get_sve_vl(t: *const task_struct) -> u32 { task_get_vl(t, vec_type::ARM64_VEC_SVE) }
#[inline] pub unsafe fn task_get_sme_vl(t: *const task_struct) -> u32 { task_get_vl(t, vec_type::ARM64_VEC_SME) }
#[inline] pub unsafe fn task_set_sve_vl(t: *mut task_struct, vl: usize) { task_set_vl(t, vec_type::ARM64_VEC_SVE, vl) }
#[inline] pub unsafe fn task_get_sve_vl_onexec(t: *const task_struct) -> u32 { task_get_vl_onexec(t, vec_type::ARM64_VEC_SVE) }
#[inline] pub unsafe fn task_set_sve_vl_onexec(t: *mut task_struct, vl: usize) { task_set_vl_onexec(t, vec_type::ARM64_VEC_SVE, vl) }

#[inline] pub unsafe fn arch_thread_struct_whitelist(offset: *mut usize, size: *mut usize) {
    *offset = core::mem::offset_of!(thread_struct, uw);
    *size = core::mem::size_of::<thread_user_whitelist>();
}

#[repr(C)] pub struct task_struct;

#[inline] pub unsafe fn prefetch(ptr: *const core::ffi::c_void) { core::arch::asm!("prfm pldl1keep, [{0}]", in(reg) ptr); }
#[inline] pub unsafe fn prefetchw(ptr: *const core::ffi::c_void) { core::arch::asm!("prfm pstl1keep, [{0}]", in(reg) ptr); }

extern "C" {
    pub fn set_tagged_addr_ctrl(task: *mut task_struct, arg: usize) -> isize;
    pub fn get_tagged_addr_ctrl(task: *mut task_struct) -> isize;
    pub fn get_tsc_mode(adr: usize) -> core::ffi::c_int;
    pub fn set_tsc_mode(val: u32) -> core::ffi::c_int;
    pub fn system_supports_sme() -> bool;
    pub fn spectre_v4_enable_task_mitigation(task: *mut task_struct);
    pub fn system_uses_irq_prio_masking() -> bool;
    pub fn current_task() -> *mut task_struct;
    pub fn sve_set_current_vl(arg: usize) -> usize;
    pub fn sve_get_current_vl() -> usize;
    pub fn sme_set_current_vl(arg: usize) -> usize;
    pub fn sme_get_current_vl() -> usize;
    pub fn ptrauth_prctl_reset_keys(task: *mut task_struct, arg: usize) -> isize;
    pub fn ptrauth_set_enabled_keys(task: *mut task_struct, keys: usize, enabled: usize) -> isize;
    pub fn ptrauth_get_enabled_keys(task: *mut task_struct) -> usize;
}

#[inline] pub unsafe fn start_thread_common(regs: *mut pt_regs, pc: usize, pstate: usize) {
    (*regs).user_regs.pc = pc;
    (*regs).user_regs.pstate = pstate;
    (*regs).orig_x0 = 0;
    if system_uses_irq_prio_masking() { (*regs).pmr = GIC_PRIO_IRQON; }
    WARN_ON_ONCE((*regs).stackframe.record.fp != 0);
    WARN_ON_ONCE((*regs).stackframe.record.lr != 0);
    WARN_ON_ONCE((*regs).stackframe.type_ != FRAME_META_TYPE_FINAL);
}

#[inline] pub unsafe fn start_thread(regs: *mut pt_regs, pc: usize, sp: usize) {
    start_thread_common(regs, pc, PSR_MODE_EL0t);
    spectre_v4_enable_task_mitigation(current_task());
    (*regs).sp = sp;
}

#[cfg(feature = "compat")]
#[inline] pub unsafe fn compat_start_thread(regs: *mut pt_regs, pc: usize, sp: usize) {
    let mut pstate = PSR_AA32_MODE_USR;
    if pc & 1 != 0 { pstate |= PSR_AA32_T_BIT; }
    if cfg!(feature = "cpu_big_endian") { pstate |= PSR_AA32_E_BIT; }
    start_thread_common(regs, pc, pstate);
    spectre_v4_enable_task_mitigation(current_task());
    (*regs).compat_sp = sp;
}

#[inline] pub fn is_ttbr0_addr(addr: usize) -> bool { addr < TASK_SIZE_64 }
#[inline] pub unsafe fn is_ttbr1_addr(addr: usize) -> bool { arch_kasan_reset_tag(addr) >= PAGE_OFFSET }

macro_rules! SVE_SET_VL { ($arg:expr) => { unsafe { sve_set_current_vl($arg) } }; }
macro_rules! SVE_GET_VL { () => { unsafe { sve_get_current_vl() } }; }
macro_rules! SME_SET_VL { ($arg:expr) => { unsafe { sme_set_current_vl($arg) } }; }
macro_rules! SME_GET_VL { () => { unsafe { sme_get_current_vl() } }; }
macro_rules! PAC_RESET_KEYS { ($tsk:expr, $arg:expr) => { unsafe { ptrauth_prctl_reset_keys($tsk, $arg) } }; }
macro_rules! PAC_SET_ENABLED_KEYS { ($tsk:expr, $keys:expr, $enabled:expr) => { unsafe { ptrauth_set_enabled_keys($tsk, $keys, $enabled) } }; }
macro_rules! PAC_GET_ENABLED_KEYS { ($tsk:expr) => { unsafe { ptrauth_get_enabled_keys($tsk) } }; }
macro_rules! GET_TSC_CTL { ($adr:expr) => { unsafe { get_tsc_mode($adr) } }; }
macro_rules! SET_TSC_CTL { ($val:expr) => { unsafe { set_tsc_mode($val) } }; }

/* External kernel types, constants, and helper functions referenced above are intentionally undeclared here. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
