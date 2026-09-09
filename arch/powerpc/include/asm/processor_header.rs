/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Translated from asm/processor.h. C preprocessor conditions are retained as cfg intent. */

#[cfg(feature = "CONFIG_VSX")]
pub const TS_FPRWIDTH: usize = 2;
#[cfg(not(feature = "CONFIG_VSX"))]
pub const TS_FPRWIDTH: usize = 1;
#[cfg(all(feature = "CONFIG_VSX", target_endian = "big"))]
pub const TS_FPROFFSET: usize = 0;
#[cfg(all(feature = "CONFIG_VSX", target_endian = "big"))]
pub const TS_VSRLOWOFFSET: usize = 1;
#[cfg(all(feature = "CONFIG_VSX", target_endian = "little"))]
pub const TS_FPROFFSET: usize = 1;
#[cfg(all(feature = "CONFIG_VSX", target_endian = "little"))]
pub const TS_VSRLOWOFFSET: usize = 0;
#[cfg(not(feature = "CONFIG_VSX"))]
pub const TS_FPROFFSET: usize = 0;

#[cfg(feature = "CONFIG_PPC64")]
pub const PPR_PRIORITY: u64 = 3;
#[cfg(feature = "CONFIG_PPC64")]
pub const DEFAULT_PPR: u64 = PPR_PRIORITY << 50;

pub const _PREP_Motorola: u32 = 0x01;
pub const _PREP_Firm: u32 = 0x02;
pub const _PREP_IBM: u32 = 0x00;
pub const _PREP_Bull: u32 = 0x03;
pub const _CHRP_Motorola: u32 = 0x04;
pub const _CHRP_IBM: u32 = 0x05;
pub const _CHRP_Pegasos: u32 = 0x06;
pub const _CHRP_briq: u32 = 0x07;

#[cfg(all(feature = "__KERNEL__", feature = "CONFIG_PPC32"))]
extern "C" { pub static mut _chrp_type: core::ffi::c_int; }

#[repr(C)]
pub struct thread_fp_state {
    pub fpr: [[u64; TS_FPRWIDTH]; 32],
    pub fpscr: u64,
}

#[repr(C)]
pub struct thread_vr_state {
    pub vr: [vector128; 32],
    pub vscr: vector128,
}

#[repr(C)]
pub struct debug_reg {
    #[cfg(feature = "CONFIG_PPC_ADV_DEBUG_REGS")]
    pub dbcr0: u32,
    #[cfg(feature = "CONFIG_PPC_ADV_DEBUG_REGS")]
    pub dbcr1: u32,
    #[cfg(all(feature = "CONFIG_PPC_ADV_DEBUG_REGS", feature = "CONFIG_BOOKE"))]
    pub dbcr2: u32,
    #[cfg(feature = "CONFIG_PPC_ADV_DEBUG_REGS")]
    pub dbsr: u32,
    #[cfg(feature = "CONFIG_PPC_ADV_DEBUG_REGS")]
    pub iac1: c_ulong,
    #[cfg(feature = "CONFIG_PPC_ADV_DEBUG_REGS")]
    pub iac2: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC_ADV_DEBUG_REGS", feature = "CONFIG_PPC_ADV_DEBUG_IACS_GT_2"))]
    pub iac3: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC_ADV_DEBUG_REGS", feature = "CONFIG_PPC_ADV_DEBUG_IACS_GT_2"))]
    pub iac4: c_ulong,
    #[cfg(feature = "CONFIG_PPC_ADV_DEBUG_REGS")]
    pub dac1: c_ulong,
    #[cfg(feature = "CONFIG_PPC_ADV_DEBUG_REGS")]
    pub dac2: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC_ADV_DEBUG_REGS", feature = "CONFIG_PPC_ADV_DEBUG_DVCS"))]
    pub dvc1: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC_ADV_DEBUG_REGS", feature = "CONFIG_PPC_ADV_DEBUG_DVCS"))]
    pub dvc2: c_ulong,
}

#[repr(C)]
pub struct thread_struct {
    pub ksp: c_ulong,
    #[cfg(feature = "CONFIG_PPC64")] pub ksp_vsid: c_ulong,
    pub regs: *mut pt_regs,
    #[cfg(feature = "CONFIG_BOOKE")] pub normsave: [c_ulong; 8],
    #[cfg(feature = "CONFIG_PPC32")] pub pgdir: *mut core::ffi::c_void,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_RTAS"))] pub rtas_sp: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_BOOK3S_32", feature = "CONFIG_PPC_KUAP"))] pub kuap: c_ulong,
    #[cfg(feature = "CONFIG_PPC32")] pub srr0: c_ulong,
    #[cfg(feature = "CONFIG_PPC32")] pub srr1: c_ulong,
    #[cfg(feature = "CONFIG_PPC32")] pub dar: c_ulong,
    #[cfg(feature = "CONFIG_PPC32")] pub dsisr: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_BOOK3S_32"))] pub r0: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_BOOK3S_32"))] pub r3: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_BOOK3S_32"))] pub r4: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_BOOK3S_32"))] pub r5: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_BOOK3S_32"))] pub r6: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_BOOK3S_32"))] pub r8: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_BOOK3S_32"))] pub r9: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_BOOK3S_32"))] pub r11: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_BOOK3S_32"))] pub lr: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_BOOK3S_32"))] pub ctr: c_ulong,
    #[cfg(all(feature = "CONFIG_PPC32", feature = "CONFIG_PPC_BOOK3S_32"))] pub sr0: c_ulong,
    #[cfg(all(feature = "CONFIG_BOOKE", feature = "CONFIG_PPC_KUAP"))] pub pid: c_ulong,
    pub debug: debug_reg,
    #[cfg(feature = "CONFIG_PPC_FPU_REGS")] pub fp_state: thread_fp_state,
    #[cfg(feature = "CONFIG_PPC_FPU_REGS")] pub fp_save_area: *mut thread_fp_state,
    pub fpexc_mode: core::ffi::c_int,
    pub align_ctl: u32,
    #[cfg(feature = "CONFIG_HAVE_HW_BREAKPOINT")] pub ptrace_bps: [*mut perf_event; HBP_NUM_MAX],
    pub hw_brk: [arch_hw_breakpoint; HBP_NUM_MAX],
    pub trap_nr: c_ulong,
    pub load_slb: u8,
    pub load_fp: u8,
    #[cfg(feature = "CONFIG_ALTIVEC")] pub load_vec: u8,
    #[cfg(feature = "CONFIG_ALTIVEC")] pub vr_state: thread_vr_state,
    #[cfg(feature = "CONFIG_ALTIVEC")] pub vr_save_area: *mut thread_vr_state,
    #[cfg(feature = "CONFIG_ALTIVEC")] pub vrsave: c_ulong,
    #[cfg(feature = "CONFIG_ALTIVEC")] pub used_vr: core::ffi::c_int,
    #[cfg(feature = "CONFIG_VSX")] pub used_vsr: core::ffi::c_int,
    #[cfg(feature = "CONFIG_SPE")] pub evr: [c_ulong; 32],
    #[cfg(feature = "CONFIG_SPE")] pub acc: u64,
    #[cfg(feature = "CONFIG_SPE")] pub spefscr: c_ulong,
    #[cfg(feature = "CONFIG_SPE")] pub spefscr_last: c_ulong,
    #[cfg(feature = "CONFIG_SPE")] pub used_spe: core::ffi::c_int,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")] pub load_tm: u8,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")] pub tm_tfhar: u64,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")] pub tm_texasr: u64,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")] pub tm_tfiar: u64,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")] pub ckpt_regs: pt_regs,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")] pub tm_tar: c_ulong,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")] pub tm_ppr: c_ulong,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")] pub tm_dscr: c_ulong,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")] pub tm_amr: c_ulong,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")] pub ckfp_state: thread_fp_state,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")] pub ckvr_state: thread_vr_state,
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")] pub ckvrsave: c_ulong,
    #[cfg(feature = "CONFIG_KVM_BOOK3S_32_HANDLER")] pub kvm_shadow_vcpu: *mut core::ffi::c_void,
    #[cfg(all(feature = "CONFIG_KVM", feature = "CONFIG_BOOKE"))] pub kvm_vcpu: *mut kvm_vcpu,
    #[cfg(feature = "CONFIG_PPC64")] pub dscr: c_ulong,
    #[cfg(feature = "CONFIG_PPC64")] pub fscr: c_ulong,
    #[cfg(feature = "CONFIG_PPC64")] pub dscr_inherit: core::ffi::c_int,
    #[cfg(feature = "CONFIG_PPC64")] pub tidr: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub tar: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub ebbrr: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub ebbhr: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub bescr: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub siar: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub sdar: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub sier: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub mmcr2: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub mmcr0: u32,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub used_ebb: u32,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub mmcr3: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub sier2: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub sier3: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub hashkeyr: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub dexcr: c_ulong,
    #[cfg(feature = "CONFIG_PPC_BOOK3S_64")] pub dexcr_onexec: c_ulong,
}

pub const ARCH_MIN_TASKALIGN: usize = 16;
pub const NET_IP_ALIGN: usize = 0;

#[repr(i32)]
pub enum idle_boot_override { IDLE_NO_OVERRIDE = 0, IDLE_POWERSAVE_OFF }

extern "C" {
    pub fn start_thread(regs: *mut pt_regs, fdptr: c_ulong, sp: c_ulong);
    pub fn __get_wchan(p: *mut task_struct) -> c_ulong;
    pub fn get_fpexc_mode(tsk: *mut task_struct, adr: c_ulong) -> core::ffi::c_int;
    pub fn set_fpexc_mode(tsk: *mut task_struct, val: u32) -> core::ffi::c_int;
    pub fn get_endian(tsk: *mut task_struct, adr: c_ulong) -> core::ffi::c_int;
    pub fn set_endian(tsk: *mut task_struct, val: u32) -> core::ffi::c_int;
    pub fn get_unalign_ctl(tsk: *mut task_struct, adr: c_ulong) -> core::ffi::c_int;
    pub fn set_unalign_ctl(tsk: *mut task_struct, val: u32) -> core::ffi::c_int;
    pub fn load_fp_state(fp: *mut thread_fp_state);
    pub fn store_fp_state(fp: *mut thread_fp_state);
    pub fn load_vr_state(vr: *mut thread_vr_state);
    pub fn store_vr_state(vr: *mut thread_vr_state);
    pub fn validate_sp(sp: c_ulong, p: *mut task_struct) -> core::ffi::c_int;
    pub fn validate_sp_size(sp: c_ulong, p: *mut task_struct, nbytes: c_ulong) -> core::ffi::c_int;
    pub fn isa300_idle_stop_noloss(psscr_val: c_ulong) -> c_ulong;
    pub fn isa300_idle_stop_mayloss(psscr_val: c_ulong) -> c_ulong;
    pub fn isa206_idle_insn_mayloss(kind: c_ulong) -> c_ulong;
    pub fn power4_idle_nap();
    pub fn power4_idle_nap_return();
    pub fn power7_idle_type(kind: c_ulong);
    pub fn arch300_idle_type(stop_psscr_val: c_ulong, stop_psscr_mask: c_ulong);
    pub fn pnv_power9_force_smt4_catch();
    pub fn pnv_power9_force_smt4_release();
    pub fn fix_alignment(regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn do_mathemu(regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn do_spe_mathemu(regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn speround_handler(regs: *mut pt_regs) -> core::ffi::c_int;
    pub fn enter_vmx_usercopy() -> core::ffi::c_int;
    pub fn exit_vmx_usercopy() -> core::ffi::c_int;
    pub fn enter_vmx_ops() -> core::ffi::c_int;
    pub fn exit_vmx_ops(dest: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
}

#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
extern "C" {
    pub fn get_dexcr_prctl(tsk: *mut task_struct, asp: c_ulong) -> core::ffi::c_int;
    pub fn set_dexcr_prctl(tsk: *mut task_struct, asp: c_ulong, val: c_ulong) -> core::ffi::c_int;
}

pub static mut cpuidle_disable: c_ulong = 0;
pub static mut powersave_nap: core::ffi::c_int = 0;

#[inline]
pub const unsafe fn __unpack_fe01(msr_bits: c_ulong) -> u32 {
    (((msr_bits & MSR_FE0) >> 10) | ((msr_bits & MSR_FE1) >> 8)) as u32
}

#[inline]
pub const unsafe fn __pack_fe01(fpmode: u32) -> c_ulong {
    (((fpmode as c_ulong) << 10) & MSR_FE0) | (((fpmode as c_ulong) << 8) & MSR_FE1)
}

pub type c_ulong = usize;
extern "C" { pub type vector128; pub type pt_regs; pub type task_struct; pub type perf_event; pub type arch_hw_breakpoint; pub type kvm_vcpu; }
extern "C" { pub static mut init_stack: u8; pub static mut swapper_pg_dir: *mut core::ffi::c_void; }
extern "C" { pub static MSR_FE0: c_ulong; pub static MSR_FE1: c_ulong; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
