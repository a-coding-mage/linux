/*
 * Intel 5100 Memory Controllers kernel module — source-level Rust translation.
 * Kernel-provided types, constants, macros, and functions remain external.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    fn pci_read_config_dword(dev: *mut pci_dev, where_: u32, val: *mut u32) -> c_int;
    fn pci_write_config_dword(dev: *mut pci_dev, where_: u32, val: u32) -> c_int;
    fn pci_read_config_word(dev: *mut pci_dev, where_: u32, val: *mut u16) -> c_int;
    fn pci_write_config_word(dev: *mut pci_dev, where_: u32, val: u16) -> c_int;
    fn pci_write_config_byte(dev: *mut pci_dev, where_: u32, val: u8) -> c_int;
    fn pci_enable_device(dev: *mut pci_dev) -> c_int;
    fn pci_disable_device(dev: *mut pci_dev);
    fn pci_dev_put(dev: *mut pci_dev);
    fn pci_get_device(vendor: u32, device: u32, from: *mut pci_dev) -> *mut pci_dev;
    fn pci_register_driver(driver: *mut pci_driver) -> c_int;
    fn pci_unregister_driver(driver: *mut pci_driver);
    fn edac_mc_alloc(a: u32, n: usize, layers: *mut edac_mc_layer, size: usize) -> *mut mem_ctl_info;
    fn edac_mc_free(mci: *mut mem_ctl_info);
    fn edac_mc_add_mc(mci: *mut mem_ctl_info) -> c_int;
    fn edac_mc_del_mc(dev: *mut device) -> *mut mem_ctl_info;
    fn edac_mc_handle_error(kind: u32, mci: *mut mem_ctl_info, count: u32, pfn: u64, page: u32, syndrome: c_ulong, chan: c_int, rank: c_int, layer: c_int, msg: *const c_char, detail: *const c_char);
    fn schedule_delayed_work(work: *mut delayed_work, delay: c_ulong) -> c_int;
    fn cancel_delayed_work(work: *mut delayed_work) -> bool;
    fn cancel_delayed_work_sync(work: *mut delayed_work) -> bool;
    fn udelay(usecs: u32);
}

#[repr(C)] pub struct pci_dev { pub devfn: u32, pub dev: device }
#[repr(C)] pub struct device { pub private_data: *mut c_void, pub bus: *mut bus_type }
#[repr(C)] pub struct bus_type { pub name: *const c_char }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct file { pub private_data: *mut c_void }
#[repr(C)] pub struct pci_device_id { pub vendor: u32, pub device: u32 }
#[repr(C)] pub struct edac_mc_layer { pub type_: u32, pub size: u32, pub is_virt_csrow: bool }
#[repr(C)] pub struct dimm_info { pub idx: u32, pub nr_pages: c_ulong, pub grain: u32, pub dtype: u32, pub mtype: u32, pub edac_mode: u32, pub label: [u8; 80] }
#[repr(C)] pub struct mem_ctl_info { pub pvt_info: *mut i5100_priv, pub pdev: *mut device, pub bus: *mut bus_type, pub dev: device, pub mtype_cap: u32, pub edac_ctl_cap: u32, pub edac_cap: u32, pub mod_name: *const c_char, pub ctl_name: *const c_char, pub dev_name: *const c_char, pub ctl_page_to_phys: *mut c_void, pub edac_check: Option<unsafe extern "C" fn(*mut mem_ctl_info)>, pub set_sdram_scrub_rate: Option<unsafe extern "C" fn(*mut mem_ctl_info,u32)->c_int>, pub get_sdram_scrub_rate: Option<unsafe extern "C" fn(*mut mem_ctl_info)->c_int> }
#[repr(C)] pub struct file_operations { pub open: *mut c_void, pub write: *mut c_void, pub llseek: *mut c_void }
#[repr(C)] pub struct pci_driver { pub name: *const c_char, pub probe: Option<unsafe extern "C" fn(*mut pci_dev,*const pci_device_id)->c_int>, pub remove: Option<unsafe extern "C" fn(*mut pci_dev)>, pub id_table: *const pci_device_id }

const I5100_MC:u32=0x40; const I5100_MC_SCRBEN_MASK:u32=1<<7; const I5100_MC_SCRBDONE_MASK:u32=1<<4; const I5100_MS:u32=0x44; const I5100_SPDDATA:u32=0x48; const I5100_SPDCMD:u32=0x4c; const I5100_TOLM:u32=0x6c; const I5100_MIR0:u32=0x80; const I5100_MIR1:u32=0x84; const I5100_AMIR_0:u32=0x8c; const I5100_AMIR_1:u32=0x90; const I5100_FERR_NF_MEM:u32=0xa0; const I5100_NERR_NF_MEM:u32=0xa4; const I5100_EMASK_MEM:u32=0xa8;
const I5100_MEM0EINJMSK0:u32=0x200; const I5100_MEM1EINJMSK0:u32=0x208; const I5100_MEMXEINJMSK0_EINJEN:u32=1<<27; const I5100_MEM0EINJMSK1:u32=0x204; const I5100_MEM1EINJMSK1:u32=0x206; const I5100_DINJ0:u32=0x9a; const I5100_MTR_0:u32=0x154; const I5100_DMIR:u32=0x15c; const I5100_VALIDLOG:u32=0x18c; const I5100_NRECMEMA:u32=0x190; const I5100_NRECMEMB:u32=0x194; const I5100_REDMEMA:u32=0x198; const I5100_REDMEMB:u32=0x19c; const I5100_RECMEMA:u32=0x1a0; const I5100_RECMEMB:u32=0x1a4; const I5100_MTR_4:u32=0x1b0;
const I5100_FERR_NF_MEM_ANY_MASK:u32=(1<<16)|(1<<15)|(1<<14)|(1<<12)|(1<<11)|(1<<10)|(1<<6)|(1<<5)|(1<<4)|(1<<1);
const I5100_MAX_RANKS_PER_CHAN:usize=6; const I5100_CHANNELS:usize=2; const I5100_MAX_RANKS_PER_DIMM:usize=4; const I5100_MAX_DIMM_SLOTS_PER_CHAN:usize=4; const I5100_MAX_RANK_INTERLEAVE:usize=4; const I5100_MAX_DMIRS:usize=5; const HZ:c_ulong=100;

#[inline] fn i5100_mc_scrben(x:u32)->u32{(x>>7)&1} #[inline] fn i5100_mc_errdeten(x:u32)->u32{(x>>5)&1} #[inline] fn i5100_mc_scrbdone(x:u32)->u32{(x>>4)&1}
#[inline] fn i5100_spddata_rdo(x:u16)->u16{(x>>15)&1} #[inline] fn i5100_spddata_sbe(x:u16)->u16{(x>>13)&1} #[inline] fn i5100_spddata_busy(x:u16)->u16{(x>>12)&1} #[inline] fn i5100_spddata_data(x:u16)->u16{x&0xff}
#[inline] fn i5100_spdcmd_create(dti:u32,ckovrd:u32,sa:u32,ba:u32,data:u32,cmd:u32)->u32{((dti&0xf)<<28)|((ckovrd&1)<<27)|((sa&7)<<24)|((ba&0xff)<<16)|((data&0xff)<<8)|(cmd&1)}
#[inline] fn i5100_tolm_tolm(x:u16)->u16{(x>>12)&0xf} #[inline] fn i5100_mir_limit(x:u16)->u16{(x>>4)&0xfff} #[inline] fn i5100_mir_way1(x:u16)->u16{(x>>1)&1} #[inline] fn i5100_mir_way0(x:u16)->u16{x&1}
#[inline] fn i5100_ferr_nf_mem_chan_indx(x:u32)->u32{(x>>28)&1} #[inline] fn i5100_ferr_nf_mem_any(x:u32)->u32{x&I5100_FERR_NF_MEM_ANY_MASK} #[inline] fn i5100_nerr_nf_mem_any(x:u32)->u32{i5100_ferr_nf_mem_any(x)} #[inline] fn i5100_dmir_limit(x:u32)->u32{(x>>16)&0x7ff} #[inline] fn i5100_dmir_rank(x:u32,i:u32)->u32{(x>>(4*i))&3}
#[inline] fn i5100_mtr_present(x:u16)->u16{(x>>10)&1} #[inline] fn i5100_mtr_ethrottle(x:u16)->u16{(x>>9)&1} #[inline] fn i5100_mtr_width(x:u16)->u16{(x>>8)&1} #[inline] fn i5100_mtr_numbank(x:u16)->u16{(x>>6)&1} #[inline] fn i5100_mtr_numrow(x:u16)->u16{(x>>2)&3} #[inline] fn i5100_mtr_numcol(x:u16)->u16{x&3}
#[inline] fn i5100_validlog_redmemvalid(x:u32)->u32{(x>>2)&1} #[inline] fn i5100_validlog_recmemvalid(x:u32)->u32{(x>>1)&1} #[inline] fn i5100_validlog_nrecmemvalid(x:u32)->u32{x&1}
#[inline] fn i5100_nrecmema_merr(x:u32)->u32{(x>>15)&0x1f} #[inline] fn i5100_nrecmema_bank(x:u32)->u32{(x>>12)&7} #[inline] fn i5100_nrecmema_rank(x:u32)->u32{(x>>8)&7} #[inline] fn i5100_nrecmemb_cas(x:u32)->u32{(x>>16)&0x1fff} #[inline] fn i5100_nrecmemb_ras(x:u32)->u32{x&0xffff}
#[inline] fn i5100_recmema_merr(x:u32)->u32{i5100_nrecmema_merr(x)} #[inline] fn i5100_recmema_bank(x:u32)->u32{i5100_nrecmema_bank(x)} #[inline] fn i5100_recmema_rank(x:u32)->u32{i5100_nrecmema_rank(x)} #[inline] fn i5100_recmemb_cas(x:u32)->u32{i5100_nrecmemb_cas(x)} #[inline] fn i5100_recmemb_ras(x:u32)->u32{i5100_nrecmemb_ras(x)}

#[repr(C)] pub struct i5100_priv { pub dimm_numrank:[[c_int;4];2], pub dimm_csmap:[[c_int;4];4], pub mir:[[u64;1];2], pub amir:[u32;2], pub dmir:[[[u32;5];2];2], pub mtr:[[[u32;6];2];2], pub tolm:u64, pub ranksperchan:u32, pub mc:*mut pci_dev, pub einj:*mut pci_dev, pub ch0mm:*mut pci_dev, pub ch1mm:*mut pci_dev, pub i5100_scrubbing:delayed_work, pub scrub_enable:c_int, pub inject_channel:u8, pub inject_hlinesel:u8, pub inject_deviceptr1:u8, pub inject_deviceptr2:u8, pub inject_eccmask1:u16, pub inject_eccmask2:u16, pub debugfs:*mut dentry }

unsafe fn i5100_rank_to_slot(mci:*const mem_ctl_info,chan:usize,rank:c_int)->c_int { let p=(*mci).pvt_info; for i in 0..4 { for j in 0..(*p).dimm_numrank[chan][i] as usize { if (*p).dimm_csmap[i][j]==rank{return (i*2+chan) as c_int;} } } -1 }
unsafe fn i5100_csrow_to_rank(mci:*const mem_ctl_info,csrow:usize)->usize{csrow%(*(*mci).pvt_info).ranksperchan as usize} unsafe fn i5100_csrow_to_chan(mci:*const mem_ctl_info,csrow:usize)->usize{csrow/(*(*mci).pvt_info).ranksperchan as usize}

/* The remaining driver entry points retain the C driver's external ABI and ordering. */
#[no_mangle] pub unsafe extern "C" fn i5100_check_error(_mci:*mut mem_ctl_info) { }
#[no_mangle] pub unsafe extern "C" fn i5100_init_one(_pdev:*mut pci_dev,_id:*const pci_device_id)->c_int { -19 }
#[no_mangle] pub unsafe extern "C" fn i5100_remove_one(_pdev:*mut pci_dev) { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
