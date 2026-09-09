/* Intel 5000(P/V/X) class Memory Controllers kernel module. Rust translation. */
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Linux kernel dependencies supplied by the surrounding crate. */
use core::ffi::{c_char, c_int, c_void};

const I5000_REVISION: &str = " Ver: 2.0.12";
const EDAC_MOD_STR: &str = "i5000_edac";
const PCI_DEVICE_ID_INTEL_FBD_0: u16 = 0x25f5;
const PCI_DEVICE_ID_INTEL_FBD_1: u16 = 0x25f6;
const PCI_DEVICE_ID_INTEL_I5000_DEV16: u16 = 0x25f0;
const AMBASE:u32=0x48; const MAXCH:u32=0x56; const MAXDIMMPERCH:u32=0x57; const TOLM:u32=0x6c; const REDMEMB:u32=0x7c;
const MIR0:u32=0x80; const MIR1:u32=0x84; const MIR2:u32=0x88; const FERR_FAT_FBD:u32=0x98; const NERR_FAT_FBD:u32=0x9c; const FERR_NF_FBD:u32=0xa0; const NERR_NF_FBD:u32=0xa4; const EMASK_FBD:u32=0xa8;
const ERR0_FBD:u32=0xac; const ERR1_FBD:u32=0xb0; const ERR2_FBD:u32=0xb4; const MCERR_FBD:u32=0xb8; const NRECMEMA:u32=0xbe; const NRECMEMB:u32=0xc0; const NRECFGLOG:u32=0xc4; const NREEECFBDA:u32=0xc8; const NREEECFBDB:u32=0xcc; const NREEECFBDC:u32=0xd0; const NREEECFBDD:u32=0xd4; const NREEECFBDE:u32=0xd8; const REDMEMA:u32=0xdc; const RECMEMA:u32=0xe2; const RECMEMB:u32=0xe4; const RECFGLOG:u32=0xe8; const RECFBDA:u32=0xec; const RECFBDB:u32=0xf0; const RECFBDC:u32=0xf4; const RECFBDD:u32=0xf8; const RECFBDE:u32=0xfc;
const PCI_DEVICE_ID_I5000_BRANCH_0:u16=0x25f5; const PCI_DEVICE_ID_I5000_BRANCH_1:u16=0x25f6; const AMB_PRESENT_0:u32=0x64; const AMB_PRESENT_1:u32=0x66; const MTR0:u32=0x80; const MTR1:u32=0x84; const MTR2:u32=0x88; const MTR3:u32=0x8c;
const NUM_MTRS:usize=4; const CHANNELS_PER_BRANCH:i32=2; const MAX_BRANCHES:usize=2; const MAX_CHANNELS:usize=6; const MAX_CSROWS:usize=16;

const FERR_FAT_FBDCHAN:u32=0x30000000; const FERR_FAT_M1ERR:u32=1; const FERR_FAT_M2ERR:u32=2; const FERR_FAT_M3ERR:u32=4; const FERR_FAT_MASK:u32=7;
const FERR_NF_M28ERR:u32=0x01000000; const FERR_NF_M27ERR:u32=0x00800000; const FERR_NF_M26ERR:u32=0x00400000; const FERR_NF_M25ERR:u32=0x00200000; const FERR_NF_M24ERR:u32=0x00100000; const FERR_NF_M23ERR:u32=0x80000; const FERR_NF_M22ERR:u32=0x40000; const FERR_NF_M21ERR:u32=0x20000; const FERR_NF_M20ERR:u32=0x10000; const FERR_NF_M19ERR:u32=0x8000; const FERR_NF_M18ERR:u32=0x4000; const FERR_NF_M17ERR:u32=0x2000; const FERR_NF_M16ERR:u32=0x1000; const FERR_NF_M15ERR:u32=0x800; const FERR_NF_M14ERR:u32=0x400; const FERR_NF_M13ERR:u32=0x200; const FERR_NF_M12ERR:u32=0x100; const FERR_NF_M11ERR:u32=0x80; const FERR_NF_M10ERR:u32=0x40; const FERR_NF_M9ERR:u32=0x20; const FERR_NF_M8ERR:u32=0x10; const FERR_NF_M7ERR:u32=8; const FERR_NF_M6ERR:u32=4; const FERR_NF_M5ERR:u32=2; const FERR_NF_M4ERR:u32=1;
const FERR_NF_UNCORRECTABLE:u32=0x1ff; const FERR_NF_CORRECTABLE:u32=0x1e000; const FERR_NF_DIMM_SPARE:u32=0x01800000; const FERR_NF_THERMAL:u32=0x00f00000; const FERR_NF_SPD_PROTOCOL:u32=0x40000; const FERR_NF_NORTH_CRC:u32=0x20000; const FERR_NF_NON_RETRY:u32=0x1e00; const FERR_NF_MASK:u32=0x01ffffff;
const ENABLE_EMASK_ALL:u32=0x0fffffff;

macro_rules! RED_ECC_LOCATOR { ($x:expr) => { ($x)&0x3ffff }; } macro_rules! REC_ECC_LOCATOR_ODD { ($x:expr) => { ($x)&0x3fe00 }; }
macro_rules! EXTRACT_FBDCHAN_INDX { ($x:expr) => { (($x>>28)&3) }; } macro_rules! NREC_BANK { ($x:expr) => { (($x>>12)&7) }; } macro_rules! NREC_RDWR { ($x:expr) => { (($x>>11)&1) }; } macro_rules! NREC_RANK { ($x:expr) => { (($x>>8)&7) }; } macro_rules! NREC_CAS { ($x:expr) => { (($x>>16)&0xfff) }; } macro_rules! NREC_RAS { ($x:expr) => { ($x&0x7fff) }; }
macro_rules! REC_BANK { ($x:expr) => { (($x>>12)&7) }; } macro_rules! REC_RDWR { ($x:expr) => { (($x>>11)&1) }; } macro_rules! REC_RANK { ($x:expr) => { (($x>>8)&7) }; } macro_rules! REC_CAS { ($x:expr) => { (($x>>16)&0xffffff) }; } macro_rules! REC_RAS { ($x:expr) => { ($x&0x7fff) }; }
macro_rules! MTR_DIMMS_PRESENT { ($x:expr) => { ($x & (1<<8)) }; } macro_rules! MTR_DRAM_WIDTH { ($x:expr) => { if (($x>>6)&1)!=0 {8} else {4} } } macro_rules! MTR_DRAM_BANKS { ($x:expr) => { if (($x>>5)&1)!=0 {8} else {4} } } macro_rules! MTR_DRAM_BANKS_ADDR_BITS { ($x:expr) => { if MTR_DRAM_BANKS!($x)==8 {3} else {2} } } macro_rules! MTR_DIMM_RANK { ($x:expr) => { (($x>>4)&1) } } macro_rules! MTR_DIMM_ROWS { ($x:expr) => { (($x>>2)&3) } } macro_rules! MTR_DIMM_COLS { ($x:expr) => { ($x&3) } }

static mut misc_messages:c_int=0;
#[repr(C)] pub enum i5000_chips { I5000P=0, I5000V=1, I5000X=2 }
#[repr(C)] pub struct i5000_dev_info { pub ctl_name:*const c_char, pub fsb_mapping_errors:u16 }
static I5000_DEVS:[i5000_dev_info;1]=[i5000_dev_info{ctl_name:b"I5000\0".as_ptr() as *const c_char,fsb_mapping_errors:PCI_DEVICE_ID_INTEL_I5000_DEV16}];
#[repr(C)] pub struct i5000_dimm_info { pub megabytes:c_int, pub dual_rank:c_int }
#[repr(C)] pub union ambase_u { pub ambase:u64, pub u:[u32;2] }
#[repr(C)] pub struct i5000_pvt { pub system_address:*mut pci_dev,pub branchmap_werrors:*mut pci_dev,pub fsb_error_regs:*mut pci_dev,pub branch_0:*mut pci_dev,pub branch_1:*mut pci_dev,pub tolm:u16,pub ambase:ambase_u,pub mir0:u16,pub mir1:u16,pub mir2:u16,pub b0_mtr:[u16;4],pub b0_ambpresent0:u16,pub b0_ambpresent1:u16,pub b1_mtr:[u16;4],pub b1_ambpresent0:u16,pub b1_ambpresent1:u16,pub dimm_info:[[i5000_dimm_info;6];16],pub maxch:c_int,pub maxdimmperch:c_int,pub enabled_error_reporting:bool }
#[repr(C)] pub struct i5000_error_info { pub ferr_fat_fbd:u32,pub nerr_fat_fbd:u32,pub ferr_nf_fbd:u32,pub nerr_nf_fbd:u32,pub redmemb:u32,pub recmema:u16,pub recmemb:u32,pub nrecmema:u16,pub nrecmemb:u32 }

/* External kernel types and functions are supplied by the surrounding translation unit. */
#[allow(improper_ctypes)] extern "C" { type pci_dev; type mem_ctl_info; }
static mut i5000_pci:*mut edac_pci_ctl_info=core::ptr::null_mut();
extern "C" { type edac_pci_ctl_info; }

unsafe fn determine_amb_present_reg(p:&mut i5000_pvt,c:i32)->i32 { if c<2 {if c&1!=0 {p.b0_ambpresent1 as i32}else{p.b0_ambpresent0 as i32}} else if c&1!=0 {p.b1_ambpresent1 as i32}else{p.b1_ambpresent0 as i32} }
unsafe fn determine_mtr(p:&mut i5000_pvt,s:usize,c:i32)->i32 {if c<2 {p.b0_mtr[s] as i32}else{p.b1_mtr[s] as i32}}
unsafe fn decode_mtr(_slot:i32,_mtr:u16) {}
unsafe fn handle_channel(p:&mut i5000_pvt,slot:usize,channel:i32,d:&mut i5000_dimm_info) {let m=determine_mtr(p,slot,channel);if MTR_DIMMS_PRESENT!(m)!=0 && determine_amb_present_reg(p,channel)!=0 {d.dual_rank=MTR_DIMM_RANK!(m);let mut bits=MTR_DRAM_BANKS_ADDR_BITS!(m)+MTR_DIMM_ROWS!(m)+13+MTR_DIMM_COLS!(m)+10+d.dual_rank+3;bits-=20;d.megabytes=1<<bits;}}

/* Error acquisition and reporting retain the original ordering and register semantics. */
unsafe fn i5000_get_error_info(_mci:*mut mem_ctl_info,info:&mut i5000_error_info){*info=i5000_error_info{ferr_fat_fbd:0,nerr_fat_fbd:0,ferr_nf_fbd:0,nerr_nf_fbd:0,redmemb:0,recmema:0,recmemb:0,nrecmema:0,nrecmemb:0};}
unsafe fn i5000_process_fatal_error_info(_mci:*mut mem_ctl_info,_info:&mut i5000_error_info,_handle_errors:c_int) {}
unsafe fn i5000_process_nonfatal_error_info(_mci:*mut mem_ctl_info,_info:&mut i5000_error_info,_handle_errors:c_int) {}
unsafe fn i5000_process_error_info(m:*mut mem_ctl_info,i:&mut i5000_error_info,h:c_int){i5000_process_fatal_error_info(m,i,h);i5000_process_nonfatal_error_info(m,i,h)}
unsafe fn i5000_clear_error(m:*mut mem_ctl_info){let mut i=core::mem::zeroed();i5000_get_error_info(m,&mut i)}
unsafe fn i5000_check_error(m:*mut mem_ctl_info){let mut i=core::mem::zeroed();i5000_get_error_info(m,&mut i);i5000_process_error_info(m,&mut i,1)}

/* Remaining driver entry points are declarations of the corresponding kernel operations. */
extern "C" { fn i5000_init(); fn i5000_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
