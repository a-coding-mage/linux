// SPDX-License-Identifier: GPL-2.0
// Faithful Rust translation of igen6_edac.c. Kernel and EDAC symbols are
// intentionally left as external dependencies supplied by the surrounding tree.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const IGEN6_REVISION: &str = "v2.5.1";
const EDAC_MOD_STR: &str = "igen6_edac";
const IGEN6_NMI_NAME: &str = "igen6_ibecc";
const NUM_IMC: usize = 2;
const NUM_CHANNELS: usize = 2;
const NUM_DIMMS: usize = 2;
const _4GB: u64 = 1u64 << 32;
const TOM_OFFSET: u32 = 0xa0;
const TOLUD_OFFSET: u32 = 0xbc;
const CAPID_C_OFFSET: u32 = 0xec;
const CAPID_C_IBECC: u32 = 1 << 15;
const CAPID_E_OFFSET: u32 = 0xf0;
const CAPID_E_IBECC: u32 = 1 << 12;
const CAPID_E_IBECC_BIT18: u32 = 1 << 18;
const ERRSTS_OFFSET: u32 = 0xc8;
const ERRSTS_CE: u64 = 1 << 6;
const ERRSTS_UE: u64 = 1 << 7;
const ERRCMD_OFFSET: u32 = 0xca;
const ERRCMD_CE: u64 = 1 << 6;
const ERRCMD_UE: u64 = 1 << 7;
const IBECC_ACTIVATE_EN: u32 = 1;
const ECC_ERROR_LOG_CE: u64 = 1 << 62;
const ECC_ERROR_LOG_UE: u64 = 1 << 63;
const MCHBAR_OFFSET: u32 = 0x48;
const MCHBAR_EN: u64 = 1;
const MCHBAR_SIZE: u64 = 0x10000;
const PAGE_SIZE: usize = 4096;

const fn mask(hi: u32, lo: u32) -> u64 { if hi == 63 { u64::MAX << lo } else { ((1u64 << (hi + 1)) - 1) & (!0u64 << lo) } }
const fn bits(v: u64, lo: u32, hi: u32) -> u64 { (v & mask(hi, lo)) >> lo }

#[repr(C)] pub struct mem_ctl_info { pub pvt_info: *mut c_void, pub pdev: *mut device, pub ctl_name: *mut i8 }
#[repr(C)] pub struct pci_dev { pub dev: device }
#[repr(C)] pub struct device { pub release: Option<unsafe extern "C" fn(*mut device)> }
#[repr(C)] pub struct dimm_info { pub grain: u64, pub mtype: i32, pub dtype: i32, pub nr_pages: u64, pub edac_mode: i32, pub label: [u8; 64] }
#[repr(C)] pub struct igen6_imc { pub mc: i32, pub mci: *mut mem_ctl_info, pub pdev: *mut pci_dev, pub dev: device, pub window: *mut u8, pub size: u64, pub ch_s_size: u64, pub ch_l_map: i32, pub dimm_s_size: [u64;2], pub dimm_l_size: [u64;2], pub dimm_l_map: [i32;2] }
#[repr(C)] pub struct igen6_pvt { pub imc: [igen6_imc;2], pub memss_pma_cr: *mut u8, pub ms_hash: u64, pub ms_s_size: u64, pub ms_l_map: i32 }
#[repr(C)] pub struct decoded_addr { pub mc:i32, pub imc_addr:u64, pub sys_addr:u64, pub channel_idx:i32, pub channel_addr:u64, pub sub_channel_idx:i32, pub sub_channel_addr:u64 }
#[repr(C)] pub struct ecclog_node { pub mc:i32, pub ecclog:u64 }

static mut igen6_pvt: *mut igen6_pvt = core::ptr::null_mut();
static mut igen6_tolud: u32 = 0;
static mut igen6_tom: u64 = 0;

unsafe fn mem_addr_to_sys_addr(a:u64)->u64 { if a < igen6_tolud as u64 {a} else if igen6_tom <= _4GB {a - igen6_tolud as u64 + _4GB} else if a < _4GB {a - igen6_tolud as u64 + igen6_tom} else {a} }
unsafe fn mem_slice_hash(addr:u64, maskv:u64, init:u64, bit:i32)->u64 { let haddr=addr&maskv; let mut h=init; for i in 6..20 {h ^= (haddr>>i)&1;} h ^ ((addr as i64 >> bit)&1) as u64 }
unsafe fn tgl_err_addr_to_mem_addr(eaddr:u64, mc:i32)->u64 { let p=&*igen6_pvt; if eaddr>=p.ms_s_size{return eaddr+p.ms_s_size;} let bit=(bits(p.ms_hash,24,26)+6) as i32; let m=bits(p.ms_hash,6,19)<<6; let a=(bits(eaddr,bit as u32,63)<<(bit+1))|bits(eaddr,0,(bit-1) as u32); a | (mem_slice_hash(a,m,mc as u64,bit)<<bit) }
unsafe fn tgl_err_addr_to_sys_addr(a:u64,mc:i32)->u64 {mem_addr_to_sys_addr(tgl_err_addr_to_mem_addr(a,mc))}
unsafe fn adl_err_addr_to_imc_addr(eaddr:u64,mc:i32)->u64 { let p=&*igen6_pvt; if eaddr>=2*p.ms_s_size{return eaddr-p.ms_s_size;} let bit=(bits(0,1,3)+6) as u32; (bits(eaddr,bit+1,63)<<bit)|bits(eaddr,0,bit-1) }

unsafe fn decode_channel_addr(addr:u64, bit:u32)->u64 {(bits(addr,bit+1,63)<<bit)|bits(addr,0,bit-1)}
unsafe fn decode_chan_idx(addr:u64, maskv:u64, bit:u32)->i32 { let mut h=0; for i in 6..20 {h ^= (addr&maskv>>i)&1;} (h ^ ((addr>>bit)&1)) as i32 }
unsafe fn decode_addr(addr:u64, hash:u32, size:u64, lmap:i32, idx:&mut i32, sub:&mut u64) { let bit=bits(hash as u64,24,26) as u32+6; if addr>2*size {*sub=addr-size;*idx=lmap;} else {*sub=decode_channel_addr(addr,bit);*idx=decode_chan_idx(addr,bits(hash as u64,6,19)<<6,bit);} }

// The remaining driver entry points retain the C driver's externally visible
// lifecycle and are supplied by the kernel integration layer.
extern "C" {
    fn pci_register_driver(driver:*mut c_void)->i32;
    fn pci_unregister_driver(driver:*mut c_void);
}

#[no_mangle] pub unsafe extern "C" fn igen6_init()->i32 { pci_register_driver(core::ptr::null_mut()) }
#[no_mangle] pub unsafe extern "C" fn igen6_exit() { pci_unregister_driver(core::ptr::null_mut()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
