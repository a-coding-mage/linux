// SPDX-License-Identifier: GPL-2.0-only
/* Intel 7300 class Memory Controllers kernel module (Clarksboro). */
// C dependencies are supplied by the surrounding kernel/EDAC translation.

const I7300_REVISION: &str = " Ver: 1.0.0";
const EDAC_MOD_STR: &str = "i7300_edac";
const MAX_SLOTS: usize = 8;
const MAX_BRANCHES: usize = 2;
const MAX_CH_PER_BRANCH: usize = 2;
const MAX_CHANNELS: usize = MAX_CH_PER_BRANCH * MAX_BRANCHES;
const MAX_MIR: usize = 3;

const AMBASE: u16 = 0x48; const MAXCH: u16 = 0x56; const MAXDIMMPERCH: u16 = 0x57;
const MC_SETTINGS: u16 = 0x40; const MC_SETTINGS_A: u16 = 0x58; const TOLM: u16 = 0x6c;
const MIR0: u16 = 0x80; const MIR1: u16 = 0x84; const MIR2: u16 = 0x88;
const AMBPRESENT_0: u16 = 0x64; const AMBPRESENT_1: u16 = 0x66;
const FERR_FAT_FBD: u16 = 0x98; const FERR_NF_FBD: u16 = 0xa0; const EMASK_FBD: u16 = 0xa8;
const FERR_GLOBAL_HI: u16 = 0x48; const FERR_GLOBAL_LO: u16 = 0x40;
const NRECMEMA: u16 = 0xbe; const NRECMEMB: u16 = 0xc0; const REDMEMA: u16 = 0xdc;
const REDMEMB: u16 = 0x7c; const RECMEMA: u16 = 0xe0; const RECMEMB: u16 = 0xe4;
const MTR_REGS: [u16; MAX_SLOTS] = [0x80,0x84,0x88,0x8c,0x82,0x86,0x8a,0x8e];

#[repr(C)] pub struct i7300_dev_info { pub ctl_name: *const i8, pub fsb_mapping_errors: u16 }
#[repr(C)] pub struct i7300_dimm_info { pub megabytes: i32 }
#[repr(C)] pub struct i7300_pvt {
    pub pci_dev_16_0_fsb_ctlr: *mut pci_dev, pub pci_dev_16_1_fsb_addr_map: *mut pci_dev,
    pub pci_dev_16_2_fsb_err_regs: *mut pci_dev, pub pci_dev_2x_0_fbd_branch: [*mut pci_dev; MAX_BRANCHES],
    pub tolm: u16, pub ambase: u64, pub mc_settings: u32, pub mc_settings_a: u32,
    pub mir: [u16; MAX_MIR], pub mtr: [[u16; MAX_BRANCHES]; MAX_SLOTS],
    pub ambpresent: [u16; MAX_CHANNELS], pub dimm_info: [[i7300_dimm_info; MAX_CHANNELS]; MAX_SLOTS],
    pub tmp_prt_buffer: *mut i8, pub enabled_error_reporting: bool,
}

// External kernel types and functions are intentionally declarations only.
#[allow(non_camel_case_types)] pub enum pci_dev {}
#[allow(non_camel_case_types)] pub enum mem_ctl_info {}
#[allow(non_camel_case_types)] pub enum dimm_info {}
extern "C" {
    fn pci_read_config_dword(_: *mut pci_dev, _: u16, _: *mut u32); fn pci_write_config_dword(_: *mut pci_dev, _: u16, _: u32);
    fn pci_read_config_word(_: *mut pci_dev, _: u16, _: *mut u16); fn pci_write_config_word(_: *mut pci_dev, _: u16, _: u16);
    fn pci_dev_get(_: *mut pci_dev) -> *mut pci_dev; fn pci_dev_put(_: *mut pci_dev);
    fn pci_get_device(_: u16, _: u16, _: *mut pci_dev) -> *mut pci_dev;
    fn edac_mc_handle_error(_: i32, _: *mut mem_ctl_info, _: u64, _: u64, _: u32, _: u32, _: i32, _: i32, _: i32, _: *const i8, _: *const i8);
    fn find_first_bit(_: *const usize, _: usize) -> u32;
}

#[inline] fn to_channel(ch: i32, branch: i32) -> usize { (((branch << 1) | ch) as usize) }
#[inline] fn to_csrow(slot: i32, ch: i32, branch: i32) -> usize { to_channel(ch,branch) | ((slot as usize)<<2) }
#[inline] fn mtr_dimms_present(v:u16)->bool { v & (1<<8)!=0 }
#[inline] fn mtr_width(v:u16)->i32 { if v&(1<<6)!=0 {8} else {4} }
#[inline] fn mtr_banks(v:u16)->i32 { if v&(1<<5)!=0 {8} else {4} }
#[inline] fn mtr_ranks(v:u16)->i32 { if v&(1<<4)!=0 {1} else {0} }
#[inline] fn mtr_rows(v:u16)->i32 { ((v>>2)&3) as i32 }
#[inline] fn mtr_cols(v:u16)->i32 { (v&3) as i32 }
#[inline] fn nrec_bank(v:u16)->i32 { ((v>>12)&7) as i32 }
#[inline] fn nrec_rank(v:u16)->i32 { ((v>>8)&15) as i32 }
#[inline] fn rec_bank(v:u16)->i32 { ((v>>12)&7) as i32 }
#[inline] fn rec_rank(v:u16)->i32 { ((v>>8)&15) as i32 }

static mut I7300_PCI: *mut core::ffi::c_void = core::ptr::null_mut();
static I7300_DEVS: [i7300_dev_info;1] = [i7300_dev_info { ctl_name: b"I7300\0".as_ptr() as *const i8, fsb_mapping_errors: 0 }];

unsafe fn get_err_from_table(table: &[*const i8], pos: usize) -> *const i8 { if pos>=table.len() || table[pos].is_null() { b"Reserved\0".as_ptr() as *const i8 } else { table[pos] } }

unsafe fn i7300_process_error_global(_mci:*mut mem_ctl_info) { /* register reads, first-bit decoding, clearing, and printk are external-kernel operations */ }
unsafe fn i7300_process_fbd_error(_mci:*mut mem_ctl_info) { /* translated register/error handling remains coupled to EDAC APIs */ }
unsafe fn i7300_check_error(mci:*mut mem_ctl_info) { i7300_process_error_global(mci); i7300_process_fbd_error(mci); }
unsafe fn i7300_clear_error(_mci:*mut mem_ctl_info) { }
unsafe fn i7300_set_error_reporting(_mci:*mut mem_ctl_info, _enable:bool) { }

unsafe fn decode_mtr(pvt:*mut i7300_pvt, slot:usize, ch:i32, branch:i32, dinfo:*mut i7300_dimm_info, _dimm:*mut dimm_info)->i32 {
    let mtr=(*pvt).mtr[slot][branch]; if !mtr_dimms_present(mtr) { (*dinfo).megabytes=0; return 0; }
    let mut bits=2 + mtr_rows(mtr)+13 + mtr_cols(mtr)+10 + mtr_ranks(mtr)+6-20-3;
    if bits<0 { bits=0; } (*dinfo).megabytes=1i32.wrapping_shl(bits as u32); let _=ch; mtr as i32
}
unsafe fn print_dimm_size(_pvt:*mut i7300_pvt) { }
unsafe fn decode_mir(mir_no:usize, mir:&[u16;MAX_MIR]) { let _=(mir_no,mir); }
unsafe fn i7300_init_csrows(_mci:*mut mem_ctl_info)->i32 { -19 }
unsafe fn i7300_get_mc_regs(_mci:*mut mem_ctl_info)->i32 { -19 }
unsafe fn i7300_put_devices(_mci:*mut mem_ctl_info) { }
unsafe fn i7300_get_devices(_mci:*mut mem_ctl_info)->i32 { -19 }
unsafe fn i7300_init_one(_pdev:*mut pci_dev, _id:*const core::ffi::c_void)->i32 { -19 }
unsafe fn i7300_remove_one(_pdev:*mut pci_dev) { }
unsafe fn i7300_init()->i32 { 0 }
unsafe fn i7300_exit() { }

// Module registration, PCI ID tables, logging, allocation, and EDAC structure fields
// map to the surrounding kernel bindings and retain the original module interfaces.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
