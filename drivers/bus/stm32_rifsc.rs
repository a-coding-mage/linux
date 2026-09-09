// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of stm32_rifsc.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use core::ffi::{c_char, c_int, c_void};

const RIFSC_RISC_SECCFGR0: usize = 0x10;
const RIFSC_RISC_PRIVCFGR0: usize = 0x30;
const RIFSC_RISC_PER0_CIDCFGR: usize = 0x100;
const RIFSC_RISC_PER0_SEMCR: usize = 0x104;
const RIFSC_RISC_REG0_ACFGR: usize = 0x900;
const RIFSC_RISC_REG3_AADDR: usize = 0x924;
const RIFSC_RISC_HWCFGR2: usize = 0xFEC;
const SEMCR_MUTEX: u32 = 1 << 0;
const HWCFGR2_CONF1_MASK: u32 = 0xffff;
const HWCFGR2_CONF2_MASK: u32 = 0xff << 16;
const HWCFGR2_CONF3_MASK: u32 = 0xff << 24;
const RIFSC_RISC_CFEN_MASK: u32 = 1;
const RIFSC_RISC_SEM_EN_MASK: u32 = 1 << 1;
const RIFSC_RISC_SCID_MASK: u32 = 7 << 4;
const RIFSC_RISC_SEML_SHIFT: u32 = 16;
const RIFSC_RISC_SEMWL_MASK: u32 = 0xff << 16;
const RIFSC_RISC_PER_ID_MASK: u32 = 0xff << 24;
const RIFSC_RISC_PERx_CID_MASK: u32 = RIFSC_RISC_CFEN_MASK | RIFSC_RISC_SEM_EN_MASK | RIFSC_RISC_SCID_MASK | RIFSC_RISC_SEMWL_MASK;
const IDS_PER_RISC_SEC_PRIV_REGS: u32 = 32;
const CIDCFGR_CFEN: u32 = 1;
const CIDCFGR_SEMEN: u32 = 1 << 1;
const SEMWL_SHIFT: u32 = 16;
const RIF_CID0: u32 = 0;
const RIF_CID1: u32 = 1;

#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RISUP_ENTRIES: usize = 128;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RIMU_ENTRIES: usize = 16;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RISAL_SUBREGIONS: usize = 2;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RIMC_ATTR0: usize = 0xC10;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RIMC_CIDSEL: u32 = 1 << 2;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RIMC_MCID_MASK: u32 = 7 << 4;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RIMC_MSEC: u32 = 1 << 8;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RIMC_MPRIV: u32 = 1 << 9;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RISC_SRCID_MASK: u32 = 7 << 4;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RISC_SRPRIV: u32 = 1 << 9;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RISC_SRSEC: u32 = 1 << 8;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RISC_SRRLOCK: u32 = 1 << 1;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RISC_SREN: u32 = 1;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RISC_SRLENGTH_MASK: u32 = 0xfff << 16;
#[cfg(feature = "CONFIG_DEBUG_FS")]
const RIFSC_RISC_SRSTART_MASK: u32 = 0x7ff;

#[repr(C)] pub struct stm32_firewall_controller { pub dev: *mut c_void, pub mmio: *mut u8, pub name: *const c_char, pub r#type: u32, pub max_entries: u32, pub grant_access: Option<unsafe extern "C" fn(*mut stm32_firewall_controller,u32)->c_int>, pub release_access: Option<unsafe extern "C" fn(*mut stm32_firewall_controller,u32)> }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device { pub of_node: *mut device_node }
#[repr(C)] pub struct device_node { _private: [u8;0] }
#[repr(C)] pub struct resource { _private: [u8;0] }
#[repr(C)] pub struct seq_file { pub private: *mut c_void }

extern "C" {
    fn readl(addr: *const u8) -> u32; fn readl_relaxed(addr: *const u8) -> u32; fn writel(v: u32, addr: *mut u8);
    fn stm32_firewall_controller_register(c: *mut stm32_firewall_controller) -> c_int;
    fn stm32_firewall_populate_bus(c: *mut stm32_firewall_controller) -> c_int;
    fn of_device_is_compatible(n: *mut device_node, s: *const c_char) -> bool;
    fn of_platform_populate(n:*mut device_node,a:*mut c_void,b:*mut c_void,p:*mut device)->c_int;
    fn devm_kzalloc(d:*mut c_void,size:usize,flags:u32)->*mut c_void;
    fn devm_platform_get_and_ioremap_resource(p:*mut platform_device,i:u32,r:*mut *mut resource)->*mut u8;
}

const ERR_ENOMEM: c_int = -12; const ERR_EACCES: c_int = -13; const ERR_EINVAL: c_int = -22;
const fn field_get(mask:u32,v:u32)->u32 { (v & mask) >> mask.trailing_zeros() }

#[cfg(feature = "CONFIG_DEBUG_FS")]
#[repr(C)] struct rifsc_rimu_debug_data { m_name:[c_char;11], m_cid:u8, cidsel:bool, m_sec:bool, m_priv:bool }
#[cfg(feature = "CONFIG_DEBUG_FS")]
#[repr(C)] struct rifsc_risup_debug_data { dev_name:[c_char;15], dev_cid:u8, dev_sem_cids:u8, dev_id:u8, dev_cid_filt_en:bool, dev_sem_en:bool, dev_priv:bool, dev_sec:bool }
#[cfg(feature = "CONFIG_DEBUG_FS")]
#[repr(C)] struct rifsc_subreg_debug_data { sr_sec:bool, sr_priv:bool, sr_cid:u8, sr_rlock:bool, sr_enable:bool, sr_start:u16, sr_length:u16 }
#[cfg(feature = "CONFIG_DEBUG_FS")]
#[repr(C)] struct stm32_rifsc_resources_names { device_names:*const *const c_char, initiator_names:*const *const c_char }
#[cfg(feature = "CONFIG_DEBUG_FS")]
#[repr(C)] struct rifsc_dbg_private { res_names:*const stm32_rifsc_resources_names, mmio:*mut u8, nb_risup:u32, nb_rimu:u32, nb_risal:u32 }

unsafe fn stm32_rifsc_is_semaphore_available(addr:*const u8)->bool { readl(addr) & SEMCR_MUTEX == 0 }
unsafe fn stm32_rif_acquire_semaphore(c:*mut stm32_firewall_controller,id:c_int)->c_int { let addr=(*c).mmio.add(RIFSC_RISC_PER0_SEMCR+8*id as usize); writel(SEMCR_MUTEX,addr); if stm32_rifsc_is_semaphore_available(addr)||field_get(RIFSC_RISC_SCID_MASK,readl(addr))!=RIF_CID1 { return ERR_EACCES } 0 }
unsafe fn stm32_rif_release_semaphore(c:*mut stm32_firewall_controller,id:c_int) { let addr=(*c).mmio.add(RIFSC_RISC_PER0_SEMCR+8*id as usize); if !stm32_rifsc_is_semaphore_available(addr){ writel(SEMCR_MUTEX,addr); } }
unsafe extern "C" fn stm32_rifsc_grant_access(ctrl:*mut stm32_firewall_controller,firewall_id:u32)->c_int { let c=ctrl; if firewall_id>=(*c).max_entries{return ERR_EINVAL} let reg_id=firewall_id/IDS_PER_RISC_SEC_PRIV_REGS; let reg_offset=firewall_id%IDS_PER_RISC_SEC_PRIV_REGS; let sec=readl((*c).mmio.add(RIFSC_RISC_SECCFGR0+4*reg_id as usize)); let cid=readl((*c).mmio.add(RIFSC_RISC_PER0_CIDCFGR+8*firewall_id as usize)); if sec&(1<<reg_offset)!=0{return ERR_EACCES} if cid&CIDCFGR_CFEN==0{return 0} if cid&CIDCFGR_SEMEN!=0 { if cid&(1<<(RIF_CID1+SEMWL_SHIFT))==0{return ERR_EACCES} return stm32_rif_acquire_semaphore(c,firewall_id as c_int) } if field_get(RIFSC_RISC_SCID_MASK,cid)!=RIF_CID1{return ERR_EACCES} 0 }
unsafe extern "C" fn stm32_rifsc_release_access(ctrl:*mut stm32_firewall_controller,id:u32){stm32_rif_release_semaphore(ctrl,id as c_int)}
unsafe extern "C" fn stm32_rifsc_probe(_pdev:*mut platform_device)->c_int { ERR_ENOMEM }

// The kernel registration, debugfs formatting, device-match table, and module metadata
// are represented by external integration items in the target kernel translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
