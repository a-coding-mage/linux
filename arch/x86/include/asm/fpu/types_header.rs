/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from asm/fpu/types.h. */

#[repr(C)]
pub struct fregs_state { pub cwd: u32, pub swd: u32, pub twd: u32, pub fip: u32, pub fcs: u32, pub foo: u32, pub fos: u32, pub st_space: [u32; 20], pub status: u32 }

#[repr(C, align(16))]
pub struct fxregs_state {
    pub cwd: u16, pub swd: u16, pub twd: u16, pub fop: u16,
    pub pointers: fxregs_state_pointers,
    pub mxcsr: u32, pub mxcsr_mask: u32,
    pub st_space: [u32; 32], pub xmm_space: [u32; 64], pub padding: [u32; 12],
    pub padding_or_reserved: fxregs_state_reserved,
}
#[repr(C)] pub union fxregs_state_pointers { pub rip_rdp: fxregs_state_rip_rdp, pub legacy: fxregs_state_legacy }
#[repr(C)] pub struct fxregs_state_rip_rdp { pub rip: u64, pub rdp: u64 }
#[repr(C)] pub struct fxregs_state_legacy { pub fip: u32, pub fcs: u32, pub foo: u32, pub fos: u32 }
#[repr(C)] pub union fxregs_state_reserved { pub padding1: [u32; 12], pub sw_reserved: [u32; 12] }

pub const MXCSR_DEFAULT: u32 = 0x1f80;
pub const MXCSR_AND_FLAGS_SIZE: usize = core::mem::size_of::<u64>();

#[repr(C)] pub struct swregs_state {
    pub cwd: u32, pub swd: u32, pub twd: u32, pub fip: u32, pub fcs: u32, pub foo: u32, pub fos: u32,
    pub st_space: [u32; 20], pub ftop: u8, pub changed: u8, pub lookahead: u8, pub no_update: u8,
    pub rm: u8, pub alimit: u8, pub info: *mut math_emu_info, pub entry_eip: u32,
}
#[repr(C)] pub struct math_emu_info;

#[repr(u32)] pub enum xfeature { XFEATURE_FP, XFEATURE_SSE, XFEATURE_YMM, XFEATURE_BNDREGS, XFEATURE_BNDCSR, XFEATURE_OPMASK, XFEATURE_ZMM_Hi256, XFEATURE_Hi16_ZMM, XFEATURE_PT_UNIMPLEMENTED_SO_FAR, XFEATURE_PKRU, XFEATURE_PASID, XFEATURE_CET_USER, XFEATURE_CET_KERNEL, XFEATURE_RSRVD_COMP_13, XFEATURE_RSRVD_COMP_14, XFEATURE_LBR, XFEATURE_RSRVD_COMP_16, XFEATURE_XTILE_CFG, XFEATURE_XTILE_DATA, XFEATURE_APX, XFEATURE_MAX }
pub const XFEATURE_MASK_FP: u64 = 1 << xfeature::XFEATURE_FP as u32;
pub const XFEATURE_MASK_SSE: u64 = 1 << xfeature::XFEATURE_SSE as u32;
pub const XFEATURE_MASK_YMM: u64 = 1 << xfeature::XFEATURE_YMM as u32;
pub const XFEATURE_MASK_BNDREGS: u64 = 1 << xfeature::XFEATURE_BNDREGS as u32;
pub const XFEATURE_MASK_BNDCSR: u64 = 1 << xfeature::XFEATURE_BNDCSR as u32;
pub const XFEATURE_MASK_OPMASK: u64 = 1 << xfeature::XFEATURE_OPMASK as u32;
pub const XFEATURE_MASK_ZMM_Hi256: u64 = 1 << xfeature::XFEATURE_ZMM_Hi256 as u32;
pub const XFEATURE_MASK_Hi16_ZMM: u64 = 1 << xfeature::XFEATURE_Hi16_ZMM as u32;
pub const XFEATURE_MASK_PT: u64 = 1 << xfeature::XFEATURE_PT_UNIMPLEMENTED_SO_FAR as u32;
pub const XFEATURE_MASK_PKRU: u64 = 1 << xfeature::XFEATURE_PKRU as u32;
pub const XFEATURE_MASK_PASID: u64 = 1 << xfeature::XFEATURE_PASID as u32;
pub const XFEATURE_MASK_CET_USER: u64 = 1 << xfeature::XFEATURE_CET_USER as u32;
pub const XFEATURE_MASK_CET_KERNEL: u64 = 1 << xfeature::XFEATURE_CET_KERNEL as u32;
pub const XFEATURE_MASK_LBR: u64 = 1 << xfeature::XFEATURE_LBR as u32;
pub const XFEATURE_MASK_XTILE_CFG: u64 = 1 << xfeature::XFEATURE_XTILE_CFG as u32;
pub const XFEATURE_MASK_XTILE_DATA: u64 = 1 << xfeature::XFEATURE_XTILE_DATA as u32;
pub const XFEATURE_MASK_APX: u64 = 1 << xfeature::XFEATURE_APX as u32;
pub const XFEATURE_MASK_FPSSE: u64 = XFEATURE_MASK_FP | XFEATURE_MASK_SSE;
pub const XFEATURE_MASK_AVX512: u64 = XFEATURE_MASK_OPMASK | XFEATURE_MASK_ZMM_Hi256 | XFEATURE_MASK_Hi16_ZMM;
pub const XFEATURE_MASK_XTILE: u64 = XFEATURE_MASK_XTILE_DATA | XFEATURE_MASK_XTILE_CFG;
pub const FIRST_EXTENDED_XFEATURE: xfeature = xfeature::XFEATURE_YMM;

#[repr(C)] pub struct reg_128_bit { pub regbytes: [u8; 16] }
#[repr(C)] pub struct reg_256_bit { pub regbytes: [u8; 32] }
#[repr(C)] pub struct reg_512_bit { pub regbytes: [u8; 64] }
#[repr(C)] pub struct reg_1024_byte { pub regbytes: [u8; 1024] }
#[repr(C, packed)] pub struct ymmh_struct { pub hi_ymm: [reg_128_bit; 16] }
#[repr(C, packed)] pub struct mpx_bndreg { pub lower_bound: u64, pub upper_bound: u64 }
#[repr(C, packed)] pub struct mpx_bndreg_state { pub bndreg: [mpx_bndreg; 4] }
#[repr(C, packed)] pub struct mpx_bndcsr { pub bndcfgu: u64, pub bndstatus: u64 }
#[repr(C, packed)] pub struct mpx_bndcsr_state { pub bndcsr: mpx_bndcsr, pub pad_to_64_bytes: [u8; 64] }
#[repr(C, packed)] pub struct avx_512_opmask_state { pub opmask_reg: [u64; 8] }
#[repr(C, packed)] pub struct avx_512_zmm_uppers_state { pub zmm_upper: [reg_256_bit; 16] }
#[repr(C, packed)] pub struct avx_512_hi16_state { pub hi16_zmm: [reg_512_bit; 16] }
#[repr(C, packed)] pub struct pkru_state { pub pkru: u32, pub pad: u32 }
#[repr(C)] pub struct cet_user_state { pub user_cet: u64, pub user_ssp: u64 }
#[repr(C, packed)] pub struct cet_supervisor_state { pub pl0_ssp: u64, pub pl1_ssp: u64, pub pl2_ssp: u64 }
#[repr(C)] pub struct lbr_entry { pub from: u64, pub to: u64, pub info: u64 }
#[repr(C)] pub struct arch_lbr_state { pub lbr_ctl: u64, pub lbr_depth: u64, pub ler_from: u64, pub ler_to: u64, pub ler_info: u64, pub entries: [lbr_entry; 0] }
#[repr(C, packed)] pub struct xtile_cfg { pub tcfg: [u64; 8] }
#[repr(C, packed)] pub struct xtile_data { pub tmm: reg_1024_byte }
#[repr(C, packed)] pub struct apx_state { pub egpr: [u64; 16] }
#[repr(C, packed)] pub struct ia32_pasid_state { pub pasid: u64 }
#[repr(C, packed)] pub struct xstate_header { pub xfeatures: u64, pub xcomp_bv: u64, pub reserved: [u64; 6] }
pub const XCOMP_BV_COMPACTED_FORMAT: u64 = 1u64 << 63;

#[repr(C, packed, align(64))] pub struct xregs_state { pub i387: fxregs_state, pub header: xstate_header, pub extended_state_area: [u8; 0] }
#[repr(C)] pub union fpregs_state { pub fsave: fregs_state, pub fxsave: fxregs_state, pub soft: swregs_state, pub xsave: xregs_state, pub __padding: [u8; PAGE_SIZE] }
#[repr(C, align(64))] pub struct fpstate { pub size: u32, pub user_size: u32, pub xfeatures: u64, pub user_xfeatures: u64, pub xfd: u64, pub is_valloc: u32, pub is_guest: u32, pub is_confidential: u32, pub in_use: u32, pub regs: fpregs_state }
pub const FPU_GUEST_PERM_LOCKED: u64 = 1u64 << 63;
#[repr(C)] pub struct fpu_state_perm { pub __state_perm: u64, pub __state_size: u32, pub __user_state_size: u32 }
#[repr(C)] pub struct fpu { pub last_cpu: u32, pub avx512_timestamp: usize, pub fpstate: *mut fpstate, pub __task_fpstate: *mut fpstate, pub perm: fpu_state_perm, pub guest_perm: fpu_state_perm, pub __fpstate: fpstate }
#[repr(C)] pub struct fpu_guest { pub xfeatures: u64, pub xfd_err: u64, pub uabi_size: u32, pub fpstate: *mut fpstate }
#[repr(C)] pub struct vcpu_fpu_config { pub size: u32, pub features: u64 }
#[repr(C)] pub struct fpu_state_config { pub max_size: u32, pub default_size: u32, pub max_features: u64, pub default_features: u64, pub legacy_features: u64, pub independent_features: u64 }
extern "C" { pub static mut fpu_kernel_cfg: fpu_state_config; pub static mut fpu_user_cfg: fpu_state_config; pub static mut guest_default_cfg: vcpu_fpu_config; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
