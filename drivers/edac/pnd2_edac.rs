// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of the Pondicherry2 EDAC driver.  Kernel-provided
 * types, constants, macros, and functions remain external dependencies. */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;

const EDAC_MOD_STR: &str = "pnd2_edac";
const APL_NUM_CHANNELS: usize = 4;
const DNV_NUM_CHANNELS: usize = 2;
const DNV_MAX_DIMMS: usize = 2;
const PND2_MSG_SIZE: usize = 256;
const PMI_ADDRESS_WIDTH: usize = 31;
const PND_MAX_PHYS_BIT: u32 = 39;
const APL_ASYMSHIFT: u32 = 28;
const DNV_ASYMSHIFT: u32 = 31;
const CH_HASH_MASK_LSB: u32 = 6;
const SLICE_HASH_MASK_LSB: u32 = 6;
const MOT_SLC_INTLV_BIT: i32 = 12;
const MOT_SHIFT: u32 = 24;
const SZ_4G: u64 = 1 << 32;
const SELECTOR_DISABLED: i32 = -1;
const MOT_CHAN_INTLV_BIT_1SLC_2CH: i32 = 12;
const MOT_CHAN_INTLV_BIT_2SLC_2CH: i32 = 13;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum Type { APL, DNV }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DramAddr { pub chan: i32, pub dimm: i32, pub rank: i32, pub bank: i32, pub row: i32, pub col: i32 }
#[repr(C)]
pub struct Pnd2Pvt { pub dimm_geom: [i32; APL_NUM_CHANNELS], pub tolm: u64, pub tohm: u64 }
#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Region { pub base: u64, pub limit: u64, pub enabled: u8 }

extern "C" {
    pub fn edac_dbg(level: i32, fmt: *const c_char, ...);
    pub fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ... ) -> i32;
    pub fn hweight8(x: u8) -> i32;
    pub fn edac_mc_handle_error(t: i32, mci: *mut MemCtlInfo, count: u32, page: u64, offset: u64, syndrome: u32, chan: i32, dimm: i32, rank: i32, msg: *const c_char, other: *const c_char);
    pub fn edac_get_dimm(mci: *mut MemCtlInfo, chan: i32, dimm: i32, rank: i32) -> *mut DimmInfo;
    pub fn edac_mc_alloc(a: i32, n: usize, layers: *const EdacMcLayer, sz: usize) -> *mut MemCtlInfo;
    pub fn edac_mc_add_mc(mci: *mut MemCtlInfo) -> i32;
    pub fn edac_mc_free(mci: *mut MemCtlInfo);
    pub fn edac_mc_del_mc(dev: *mut c_void);
    pub fn ghes_get_devices() -> i32;
    pub fn edac_get_owner() -> *const c_char;
    pub fn cpu_feature_enabled(feature: i32) -> bool;
    pub fn opstate_init();
}

#[repr(C)] pub struct MemCtlInfo { pub pvt_info: *mut Pnd2Pvt, pub mod_name: *const c_char, pub dev_name: *const c_char, pub ctl_name: *const c_char }
#[repr(C)] pub struct DimmInfo { pub nr_pages: u64, pub grain: u32, pub dtype: i32, pub mtype: i32, pub edac_mode: i32, pub label: [c_char; 80] }
#[repr(C)] #[derive(Copy,Clone)] pub struct EdacMcLayer { pub type_: i32, pub size: i32, pub is_virt_csrow: bool }

#[repr(C)] pub struct DunitOps {
    pub name: *const c_char, pub type_: Type, pub pmiaddr_shift: i32, pub pmiidx_shift: i32,
    pub channels: i32, pub dimms_per_channel: i32,
    pub rd_reg: Option<unsafe extern "C" fn(i32,i32,i32,*mut c_void,usize,*mut c_char)->i32>,
    pub get_registers: Option<unsafe extern "C" fn()->i32>, pub check_ecc: Option<unsafe extern "C" fn()->i32>,
    pub mk_region: Option<unsafe extern "C" fn(*mut c_char,*mut Region,*mut c_void)>,
    pub get_dimm_config: Option<unsafe extern "C" fn(*mut MemCtlInfo)>,
    pub pmi2mem: Option<unsafe extern "C" fn(*mut MemCtlInfo,u64,u32,*mut DramAddr,*mut c_char)->i32>,
}

static mut MOT: Region = Region { base:0, limit:0, enabled:0 };
static mut AS0: Region = Region { base:0, limit:0, enabled:0 };
static mut AS1: Region = Region { base:0, limit:0, enabled:0 };
static mut AS2: Region = Region { base:0, limit:0, enabled:0 };
static mut OPS: *mut DunitOps = core::ptr::null_mut();
static mut PND2_MCI: *mut MemCtlInfo = core::ptr::null_mut();
static mut TOP_LM: u64 = 0; static mut TOP_HM: u64 = 0;
static mut TWO_SLICES: bool = false; static mut TWO_CHANNELS: bool = false;
static mut SYM_CHAN_MASK: u8 = 0; static mut ASYM_CHAN_MASK: u8 = 0; static mut CHAN_MASK: usize = 0;
static mut SLICE_SELECTOR: i32 = -1; static mut CHAN_SELECTOR: i32 = -1;
static mut SLICE_HASH_MASK: u64 = 0; static mut CHAN_HASH_MASK: u64 = 0;

#[inline] fn bit(n: u32) -> u64 { 1u64 << n }
#[inline] fn genmask(hi: u32, lo: u32) -> u64 { if hi == 63 { u64::MAX << lo } else { ((1u64 << (hi-lo+1))-1) << lo } }
#[inline] fn get_bitfield(v:u64,lo:u32,hi:u32)->u64 {(v & genmask(hi,lo)) >> lo}
unsafe fn mk_region(rp:*mut Region, base:u64, limit:u64) { (*rp).enabled=1; (*rp).base=base; (*rp).limit=limit; }
unsafe fn in_region(rp:*const Region, addr:u64)->bool { (*rp).enabled != 0 && (*rp).base <= addr && addr <= (*rp).limit }
unsafe fn remove_mmio_gap(sys:u64)->u64 { if sys < SZ_4G {sys} else {sys - (SZ_4G-TOP_LM)} }
unsafe fn remove_addr_bit(addr:*mut u64, bitidx:i32) { if bitidx != -1 { let mask=bit(bitidx as u32)-1; *addr=((*addr>>1)&!mask)|(*addr&mask); } }
fn hash_by_mask(addr:u64, mask:u64)->i32 { let mut r=addr&mask; r=(r>>32)^r; r=(r>>16)^r; r=(r>>8)^r; r=(r>>4)^r; r=(r>>2)^r; r=(r>>1)^r; (r&1) as i32 }

unsafe fn sys2pmi(addr:u64,pmiidx:*mut u32,pmiaddr:*mut u64)->i32 {
    let mot_hit=in_region(&MOT,addr); let sym_channels=hweight8(SYM_CHAN_MASK) as u32; let sym_shift=sym_channels>>1;
    if addr>=bit(PND_MAX_PHYS_BIT) || (addr>=TOP_LM && addr<SZ_4G) || addr>=TOP_HM { return -22; }
    let mut ca=remove_mmio_gap(addr); let mut slice_rm=SELECTOR_DISABLED; let mut chan_rm=SELECTOR_DISABLED;
    if in_region(&AS0,addr) { *pmiidx=0; let b=remove_mmio_gap(AS0.base); let off=ca-b; let adj=(b>>sym_shift)*1; ca=off+if sym_channels>0{adj}else{0}; }
    else if in_region(&AS1,addr) { *pmiidx=2; let b=remove_mmio_gap(AS1.base); let off=ca-b; let adj=(b>>sym_shift)*1; ca=off+if sym_channels>0{adj}else{0}; }
    else { *pmiidx=0; if TWO_SLICES { slice_rm=if mot_hit{MOT_SLC_INTLV_BIT}else{SLICE_SELECTOR}; *pmiidx=(hash_by_mask(if mot_hit{addr}else{ca},SLICE_HASH_MASK) as u32)<<1; } if TWO_CHANNELS { let ib=if TWO_SLICES{MOT_CHAN_INTLV_BIT_2SLC_2CH}else{MOT_CHAN_INTLV_BIT_1SLC_2CH}; chan_rm=if mot_hit{ib}else{CHAN_SELECTOR}; *pmiidx |= hash_by_mask(if mot_hit{addr}else{ca},CHAN_HASH_MASK) as u32; } }
    remove_addr_bit(&mut ca,chan_rm); remove_addr_bit(&mut ca,slice_rm); *pmiaddr=ca; 0
}

const fn c(n:u16)->u16 { 16|n } const fn b(n:u16)->u16 {32|n} const fn r(n:u16)->u16 {64|n} const RS:u16=128;
#[repr(C)] struct DimmGeometry { addrdec:u8, dden:u8, dwid:u8, rowbits:u8, colbits:u8, bits:[u16;PMI_ADDRESS_WIDTH] }
static DIMMS:[DimmGeometry;12]=[
 DimmGeometry{addrdec:0,dden:0,dwid:1,rowbits:15,colbits:10,bits:[c(2),c(3),c(4),c(5),c(6),b(0),b(1),b(2),r(0),r(1),r(2),r(3),r(4),r(5),r(6),r(7),r(8),r(9),r(10),c(7),c(8),c(9),r(11),RS,r(12),r(13),r(14),0,0,0,0]},
 DimmGeometry{addrdec:0,dden:0,dwid:0,rowbits:16,colbits:10,bits:[c(2),c(3),c(4),c(5),c(6),b(0),b(1),b(2),r(0),r(1),r(2),r(3),r(4),r(5),r(6),r(7),r(8),r(9),r(10),c(7),c(8),c(9),r(11),RS,r(12),r(13),r(14),r(15),0,0,0]},
 DimmGeometry{addrdec:1,dden:0,dwid:0,rowbits:16,colbits:10,bits:[c(2),c(3),c(4),c(5),c(6),c(7),b(0),b(1),b(2),r(0),r(1),r(2),r(3),r(4),r(5),r(6),r(7),r(8),r(9),r(10),c(8),c(9),r(11),RS,r(12),r(13),r(14),r(15),0,0,0]},
 DimmGeometry{addrdec:2,dden:0,dwid:0,rowbits:16,colbits:11,bits:[c(2),c(3),c(4),c(5),c(6),c(7),c(8),b(0),b(1),b(2),r(0),r(1),r(2),r(3),r(4),r(5),r(6),r(7),r(8),r(9),r(10),c(9),r(11),RS,c(11),r(12),r(13),r(14),r(15),0,0]},
 DimmGeometry{addrdec:0,dden:2,dwid:1,rowbits:16,colbits:10,bits:[c(2),c(3),c(4),c(5),c(6),b(0),b(1),b(2),r(0),r(1),r(2),r(3),r(4),r(5),r(6),r(7),r(8),r(9),r(10),c(7),c(8),c(9),r(11),RS,r(12),r(13),r(14),r(15),0,0,0]},
 DimmGeometry{addrdec:0,dden:2,dwid:0,rowbits:16,colbits:11,bits:[c(2),c(3),c(4),c(5),c(6),b(0),b(1),b(2),r(0),r(1),r(2),r(3),r(4),r(5),r(6),r(7),r(8),r(9),r(10),c(7),c(8),c(9),r(11),RS,c(11),r(12),r(13),r(14),r(15),0,0]},
 DimmGeometry{addrdec:1,dden:2,dwid:1,rowbits:16,colbits:10,bits:[c(2),c(3),c(4),c(5),c(6),c(7),b(0),b(1),b(2),r(0),r(1),r(2),r(3),r(4),r(5),r(6),r(7),r(8),r(9),r(10),c(8),c(9),r(11),RS,r(12),r(13),r(14),r(15),0,0,0]},
 DimmGeometry{addrdec:1,dden:2,dwid:0,rowbits:16,colbits:11,bits:[c(2),c(3),c(4),c(5),c(6),c(7),b(0),b(1),b(2),r(0),r(1),r(2),r(3),r(4),r(5),r(6),r(7),r(8),r(9),r(10),c(8),c(9),r(11),RS,c(11),r(12),r(13),r(14),r(15),0,0]},
 DimmGeometry{addrdec:2,dden:2,dwid:1,rowbits:16,colbits:10,bits:[c(2),c(3),c(4),c(5),c(6),c(7),c(8),b(0),b(1),b(2),r(0),r(1),r(2),r(3),r(4),r(5),r(6),r(7),r(8),r(9),r(10),c(9),r(11),RS,r(12),r(13),r(14),r(15),0,0,0]},
 DimmGeometry{addrdec:2,dden:2,dwid:0,rowbits:16,colbits:11,bits:[c(2),c(3),c(4),c(5),c(6),c(7),c(8),b(0),b(1),b(2),r(0),r(1),r(2),r(3),r(4),r(5),r(6),r(7),r(8),r(9),r(10),c(9),r(11),RS,c(11),r(12),r(13),r(14),r(15),0,0]},
 DimmGeometry{addrdec:0,dden:0,dwid:1,rowbits:15,colbits:10,bits:[0;PMI_ADDRESS_WIDTH]}, DimmGeometry{addrdec:0,dden:0,dwid:0,rowbits:16,colbits:10,bits:[0;PMI_ADDRESS_WIDTH]}
];

unsafe fn bank_hash(p:u64,idx:i32,sh:i32)->i32 { match idx {0=>(((p>>(12+sh))^(p>>(9+sh)))&1)as i32,1=>((((p>>(10+sh))^(p>>(8+sh)))&1)<<1|(((p>>22)&1)<<1))as i32,2=>((((p>>(13+sh))^(p>>(11+sh)))&1)<<2)as i32,_=>0} }
unsafe fn rank_hash(p:u64)->i32 {((p>>16)^(p>>10)&1)as i32}

/* The remaining driver entry points retain the original externally supplied
 * kernel register definitions and callback bodies. */
#[no_mangle] pub unsafe extern "C" fn pnd2_translate(addr:u64, idx:*mut u32, pmi:*mut u64)->i32 { sys2pmi(addr,idx,pmi) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
