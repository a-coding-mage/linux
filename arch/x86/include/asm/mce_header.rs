/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency: <uapi/asm/mce.h> */
/* Machine Check support for x86. */

pub const MCG_BANKCNT_MASK: u64 = 0xff;
pub const MCG_CTL_P: u64 = 1u64 << 8;
pub const MCG_EXT_P: u64 = 1u64 << 9;
pub const MCG_CMCI_P: u64 = 1u64 << 10;
pub const MCG_SEAM_NR: u64 = 1u64 << 12;
pub const MCG_EXT_CNT_MASK: u64 = 0xff0000;
pub const MCG_EXT_CNT_SHIFT: u32 = 16;
#[inline] pub const fn MCG_EXT_CNT(c: u64) -> u64 { (c & MCG_EXT_CNT_MASK) >> MCG_EXT_CNT_SHIFT }
pub const MCG_SER_P: u64 = 1u64 << 24;
pub const MCG_ELOG_P: u64 = 1u64 << 26;
pub const MCG_LMCE_P: u64 = 1u64 << 27;

pub const MCG_STATUS_RIPV: u64 = 1u64 << 0;
pub const MCG_STATUS_EIPV: u64 = 1u64 << 1;
pub const MCG_STATUS_MCIP: u64 = 1u64 << 2;
pub const MCG_STATUS_LMCES: u64 = 1u64 << 3;
pub const MCG_STATUS_SEAM_NR: u64 = 1u64 << 12;
pub const MCG_EXT_CTL_LMCE_EN: u64 = 1u64 << 0;

pub const MCI_STATUS_VAL: u64 = 1u64 << 63;
pub const MCI_STATUS_OVER: u64 = 1u64 << 62;
pub const MCI_STATUS_UC: u64 = 1u64 << 61;
pub const MCI_STATUS_EN: u64 = 1u64 << 60;
pub const MCI_STATUS_MISCV: u64 = 1u64 << 59;
pub const MCI_STATUS_ADDRV: u64 = 1u64 << 58;
pub const MCI_STATUS_PCC: u64 = 1u64 << 57;
pub const MCI_STATUS_S: u64 = 1u64 << 56;
pub const MCI_STATUS_AR: u64 = 1u64 << 55;
pub const MCI_STATUS_CEC_SHIFT: u32 = 38;
pub const MCI_STATUS_CEC_MASK: u64 = 0x1ffffc000000000;
#[inline] pub const fn MCI_STATUS_CEC(c: u64) -> u64 { (c & MCI_STATUS_CEC_MASK) >> MCI_STATUS_CEC_SHIFT }
#[inline] pub const fn MCI_STATUS_MSCOD(m: u64) -> u64 { (m >> 16) & 0xffff }

pub const MCI_STATUS_TCC: u64 = 1u64 << 55;
pub const MCI_STATUS_PADDRV: u64 = 1u64 << 54;
pub const MCI_STATUS_SYNDV: u64 = 1u64 << 53;
pub const MCI_STATUS_DEFERRED: u64 = 1u64 << 44;
pub const MCI_STATUS_POISON: u64 = 1u64 << 43;
pub const MCI_STATUS_SCRUB: u64 = 1u64 << 40;
pub const MCI_CONFIG_MCAX: u32 = 0x1;
pub const MCI_CONFIG_FRUTEXT: u64 = 1u64 << 9;
pub const MCI_CONFIG_PADDRV: u64 = 1u64 << 11;
pub const MCI_IPID_MCATYPE: u32 = 0xffff0000;
pub const MCI_IPID_HWID: u32 = 0xfff;
pub const MCACOD: u32 = 0xefff;
pub const MCACOD_SCRUB: u32 = 0x00c0;
pub const MCACOD_SCRUBMSK: u32 = 0xeff0;
pub const MCACOD_L3WB: u32 = 0x017a;
pub const MCACOD_DATA: u32 = 0x0134;
pub const MCACOD_INSTR: u32 = 0x0150;

#[inline] pub const fn MCI_MISC_ADDR_LSB(m: u64) -> u64 { m & 0x3f }
#[inline] pub const fn MCI_MISC_ADDR_MODE(m: u64) -> u64 { (m >> 6) & 7 }
pub const MCI_MISC_ADDR_SEGOFF: u32 = 0;
pub const MCI_MISC_ADDR_LINEAR: u32 = 1;
pub const MCI_MISC_ADDR_PHYS: u32 = 2;
pub const MCI_MISC_ADDR_MEM: u32 = 3;
pub const MCI_MISC_ADDR_GENERIC: u32 = 7;
/* MCI_ADDR_PHYSADDR uses GENMASK_ULL(boot_cpu_data.x86_phys_bits - 1, 0). */
pub const MCI_CTL2_CMCI_EN: u64 = 1u64 << 30;
pub const MCI_CTL2_CMCI_THRESHOLD_MASK: u64 = 0x7fff;
pub const MCJ_CTX_MASK: u32 = 3;
#[inline] pub const fn MCJ_CTX(flags: u32) -> u32 { flags & MCJ_CTX_MASK }
pub const MCJ_CTX_RANDOM: u32 = 0;
pub const MCJ_CTX_PROCESS: u32 = 1;
pub const MCJ_CTX_IRQ: u32 = 2;
pub const MCJ_NMI_BROADCAST: u32 = 4;
pub const MCJ_EXCEPTION: u32 = 8;
pub const MCJ_IRQ_BROADCAST: u32 = 0x10;
pub const MCE_OVERFLOW: u32 = 0;
pub const MCE_LOG_MIN_LEN: u32 = 32;
pub const MCE_LOG_SIGNATURE: &[u8; 12] = b"MACHINECHECK";

pub const MSR_AMD64_SMCA_MC0_CTL: u32 = 0xc0002000;
pub const MSR_AMD64_SMCA_MC0_STATUS: u32 = 0xc0002001;
pub const MSR_AMD64_SMCA_MC0_ADDR: u32 = 0xc0002002;
pub const MSR_AMD64_SMCA_MC0_MISC0: u32 = 0xc0002003;
pub const MSR_AMD64_SMCA_MC0_CONFIG: u32 = 0xc0002004;
pub const MSR_AMD64_SMCA_MC0_IPID: u32 = 0xc0002005;
pub const MSR_AMD64_SMCA_MC0_SYND: u32 = 0xc0002006;
pub const MSR_AMD64_SMCA_MC0_DESTAT: u32 = 0xc0002008;
pub const MSR_AMD64_SMCA_MC0_DEADDR: u32 = 0xc0002009;
pub const MSR_AMD64_SMCA_MC0_MISC1: u32 = 0xc000200a;
pub const MSR_AMD64_SMCA_MC0_SYND1: u32 = 0xc000200e;
pub const MSR_AMD64_SMCA_MC0_SYND2: u32 = 0xc000200f;
macro_rules! smca_reg { ($name:ident, $base:ident, $x:ident) => { #[inline] pub const fn $name($x: u32) -> u32 { $base + 0x10 * $x } }; }
smca_reg!(MSR_AMD64_SMCA_MCx_CTL, MSR_AMD64_SMCA_MC0_CTL, x);
smca_reg!(MSR_AMD64_SMCA_MCx_STATUS, MSR_AMD64_SMCA_MC0_STATUS, x);
smca_reg!(MSR_AMD64_SMCA_MCx_ADDR, MSR_AMD64_SMCA_MC0_ADDR, x);
smca_reg!(MSR_AMD64_SMCA_MCx_MISC, MSR_AMD64_SMCA_MC0_MISC0, x);
smca_reg!(MSR_AMD64_SMCA_MCx_CONFIG, MSR_AMD64_SMCA_MC0_CONFIG, x);
smca_reg!(MSR_AMD64_SMCA_MCx_IPID, MSR_AMD64_SMCA_MC0_IPID, x);
smca_reg!(MSR_AMD64_SMCA_MCx_SYND, MSR_AMD64_SMCA_MC0_SYND, x);
smca_reg!(MSR_AMD64_SMCA_MCx_DESTAT, MSR_AMD64_SMCA_MC0_DESTAT, x);
smca_reg!(MSR_AMD64_SMCA_MCx_DEADDR, MSR_AMD64_SMCA_MC0_DEADDR, x);
#[inline] pub const fn MSR_AMD64_SMCA_MCx_MISCy(x: u32, y: u32) -> u32 { (MSR_AMD64_SMCA_MC0_MISC1 + y) + 0x10 * x }
smca_reg!(MSR_AMD64_SMCA_MCx_SYND1, MSR_AMD64_SMCA_MC0_SYND1, x);
smca_reg!(MSR_AMD64_SMCA_MCx_SYND2, MSR_AMD64_SMCA_MC0_SYND2, x);
#[inline] pub const fn XEC(x: u64, mask: u64) -> u64 { (x >> 16) & mask }

pub const MCE_HANDLED_CEC: u64 = 1u64 << 0;
pub const MCE_HANDLED_UC: u64 = 1u64 << 1;
pub const MCE_HANDLED_EXTLOG: u64 = 1u64 << 2;
pub const MCE_HANDLED_NFIT: u64 = 1u64 << 3;
pub const MCE_HANDLED_EDAC: u64 = 1u64 << 4;
pub const MCE_HANDLED_MCELOG: u64 = 1u64 << 5;
pub const MCE_IN_KERNEL_RECOV: u64 = 1u64 << 6;
pub const MCE_IN_KERNEL_COPYIN: u64 = 1u64 << 7;
pub const MCE_CHECK_DFR_REGS: u64 = 1u64 << 8;

#[repr(C)] pub struct mce_log_buffer { pub signature: [i8; 12], pub len: u32, pub next: u32, pub flags: u32, pub recordlen: u32, pub entry: [mce; 0] }
#[repr(C)] pub struct mce_hw_err { pub m: mce, pub vendor: vendor_info }
#[repr(C)] pub union vendor_info { pub amd: amd_vendor_info }
#[repr(C)] pub struct amd_vendor_info { pub synd1: u64, pub synd2: u64 }
#[inline] pub unsafe fn to_mce_hw_err(mce: *mut mce) -> *mut mce_hw_err { (mce as *mut u8).sub(core::mem::offset_of!(mce_hw_err, m)) as *mut mce_hw_err }

#[repr(C)] pub struct notifier_block { _private: [u8; 0] }
extern "C" { pub fn mce_register_decode_chain(nb: *mut notifier_block); pub fn mce_unregister_decode_chain(nb: *mut notifier_block); }
extern "C" { pub static mut mce_p5_enabled: i32; }

/* Conditional declarations below retain the source CONFIG_* conditions. */
extern "C" { pub fn enable_copy_mc_fragile(); pub fn copy_mc_fragile(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, cnt: usize) -> usize; }
#[repr(C)] pub struct cper_ia_proc_ctx { _private: [u8; 0] }
extern "C" { pub fn mcheck_init() -> i32; pub fn mca_bsp_init(c: *mut cpuinfo_x86); pub fn mcheck_cpu_init(c: *mut cpuinfo_x86); pub fn mcheck_cpu_clear(c: *mut cpuinfo_x86); pub fn apei_smca_report_x86_error(ctx_info: *mut cper_ia_proc_ctx, lapic_id: u64) -> i32; }
extern "C" { pub fn mce_prep_record(err: *mut mce_hw_err); pub fn mce_log(err: *mut mce_hw_err); }
pub const MAX_NR_BANKS: usize = 64;
extern "C" { pub fn mce_intel_feature_init(c: *mut cpuinfo_x86); pub fn mce_intel_feature_clear(c: *mut cpuinfo_x86); pub fn cmci_clear(); pub fn cmci_reenable(); pub fn cmci_rediscover(); pub fn cmci_recheck(); pub fn mce_available(c: *mut cpuinfo_x86) -> bool; pub fn mce_is_memory_error(m: *mut mce) -> bool; pub fn mce_is_correctable(m: *mut mce) -> bool; pub fn mce_usable_address(m: *mut mce) -> bool; }
pub type mce_banks_t = [u64; (MAX_NR_BANKS + 63) / 64];
#[repr(u32)] pub enum mcp_flags { MCP_TIMESTAMP = 1, MCP_UC = 2, MCP_QUEUE_LOG = 4 }
extern "C" { pub fn machine_check_poll(flags: mcp_flags, b: *mut mce_banks_t); pub fn mce_disable_bank(bank: i32); pub fn mce_save_apei_thr_limit(thr_limit: u32); pub fn do_machine_check(pt_regs: *mut pt_regs); pub static mut mce_threshold_vector: Option<unsafe extern "C" fn()>; pub static mut deferred_error_int_vector: Option<unsafe extern "C" fn()>; }
#[repr(C)] pub struct cper_sec_mem_err { _private: [u8; 0] }
extern "C" { pub fn apei_mce_report_mem_error(corrected: i32, mem_err: *mut cper_sec_mem_err); }

#[repr(u32)] pub enum smca_bank_types { SMCA_CS, SMCA_CS_V2, SMCA_DACC_BE, SMCA_DACC_FE, SMCA_DE, SMCA_EDDR5CMN, SMCA_EX, SMCA_FP, SMCA_GMI_PCS, SMCA_GMI_PHY, SMCA_IF, SMCA_L2_CACHE, SMCA_L3_CACHE, SMCA_LS, SMCA_LS_V2, SMCA_MA_LLC, SMCA_MP5, SMCA_MPART, SMCA_MPASP, SMCA_MPASP_V2, SMCA_MPDACC, SMCA_MPDMA, SMCA_MPM, SMCA_MPRAS, SMCA_NBIF, SMCA_NBIO, SMCA_PB, SMCA_PCIE, SMCA_PCIE_V2, SMCA_PCIE_PL, SMCA_PIE, SMCA_PSP, SMCA_PSP_V2, SMCA_RESERVED, SMCA_SATA, SMCA_SHUB, SMCA_SMU, SMCA_SMU_V2, SMCA_SSBDCI, SMCA_UMC, SMCA_UMC_V2, SMCA_USB, SMCA_USR_CP, SMCA_USR_DP, SMCA_WAFL_PHY, SMCA_XGMI_PCS, SMCA_XGMI_PHY, N_SMCA_BANK_TYPES }
extern "C" { pub fn amd_mce_is_memory_error(m: *mut mce) -> bool; pub fn mce_amd_feature_init(c: *mut cpuinfo_x86); pub fn smca_get_bank_type(cpu: u32, bank: u32) -> smca_bank_types; pub fn copy_mc_fragile_handle_tail(to: *mut i8, from: *mut i8, len: u32) -> usize; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
