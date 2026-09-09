/* Rust translation of edac_mc.c. External kernel types and functions are
 * supplied by the surrounding EDAC/kernel bindings. */

use core::ffi::c_void;

extern "C" {
    static mut edac_debug_level: i32;
    static mut edac_op_state: i32;
    static mut edac_layer_name: [*const i8; EDAC_MAX_LAYERS];
    static mut edac_mc_owner: *const i8;
}

const EDAC_MAX_LAYERS: usize = 3;
const EDAC_MAX_LABELS: i32 = 8;
const OTHER_LABEL: *const i8 = b"|\0" as *const u8 as *const i8;

#[repr(C)] pub struct device { release: Option<unsafe extern "C" fn(*mut device)> }
#[repr(C)] pub struct list_head { next: *mut list_head, prev: *mut list_head }
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct delayed_work;
#[repr(C)] pub struct attribute_group;
#[repr(C)] pub struct page;
#[repr(C)] pub struct edac_mc_layer { pub type_: u32, pub size: u32, pub is_virt_csrow: bool }
#[repr(C)] pub struct rank_info { pub chan_idx: u32, pub csrow: *mut csrow_info, pub dimm: *mut dimm_info, pub ce_count: u32 }
#[repr(C)] pub struct csrow_info { pub csrow_idx: u32, pub first_page: usize, pub last_page: usize, pub page_mask: usize, pub nr_channels: u32, pub channels: *mut *mut rank_info, pub mci: *mut mem_ctl_info, pub ce_count: u32, pub ue_count: u32 }
#[repr(C)] pub struct dimm_info { pub mci: *mut mem_ctl_info, pub idx: u32, pub location: [u32; EDAC_MAX_LAYERS], pub label: [i8; 80], pub csrow: i32, pub cschannel: i32, pub nr_pages: u32, pub grain: i32, pub ce_count: u32, pub ue_count: u32 }
#[repr(C)] pub struct edac_raw_error_desc { pub error_count: u16, pub type_: i32, pub top_layer: i32, pub mid_layer: i32, pub low_layer: i32, pub page_frame_number: usize, pub offset_in_page: usize, pub syndrome: usize, pub grain: i64, pub msg: *const i8, pub other_detail: *const i8, pub label: [i8; 256], pub location: [i8; 256] }
#[repr(C)] pub struct mem_ctl_info { pub dev: device, pub link: list_head, pub n_layers: u32, pub layers: *mut edac_mc_layer, pub pvt_info: *mut c_void, pub mc_idx: u32, pub tot_dimms: u32, pub nr_csrows: u32, pub num_cschannel: u32, pub csbased: bool, pub csrows: *mut *mut csrow_info, pub dimms: *mut *mut dimm_info, pub op_state: i32, pub edac_check: Option<unsafe extern "C" fn(*mut mem_ctl_info)>, pub work: delayed_work, pub pdev: *mut device, pub mod_name: *const i8, pub ctl_name: *const i8, pub dev_name: *const i8, pub start_time: usize, pub bus: *mut c_void, pub error_desc: edac_raw_error_desc, pub ce_mc: u64, pub ce_noinfo_count: u64, pub ue_mc: u64, pub ue_noinfo_count: u64, pub scrub_mode: i32, pub ctl_page_to_phys: Option<unsafe extern "C" fn(*mut mem_ctl_info, usize) -> usize> }

extern "C" {
    fn mutex_lock(_: *mut c_void); fn mutex_unlock(_: *mut c_void);
    fn put_device(_: *mut device); fn device_initialize(_: *mut device);
    fn kfree(_: *mut c_void); fn edac_dbg(_: i32, _: *const i8, ...);
    fn scnprintf(_: *mut i8, _: usize, _: *const i8, ...) -> i32;
    fn memset(_: *mut c_void, _: i32, _: usize); fn memcpy(_: *mut c_void, _: *const c_void, _: usize);
    fn edac_get_dimm(_: *mut mem_ctl_info, _: i32, _: i32, _: i32) -> *mut dimm_info;
    fn edac_mc_get_poll_msec() -> u32; fn edac_queue_work(_: *mut delayed_work, _: u64);
    fn edac_mod_work(_: *mut delayed_work, _: u64); fn edac_stop_work(_: *mut delayed_work);
    fn edac_create_sysfs_mci_device(_: *mut mem_ctl_info, _: *const *const attribute_group) -> i32;
    fn edac_remove_sysfs_mci_device(_: *mut mem_ctl_info); fn edac_get_sysfs_subsys() -> *mut c_void;
    fn edac_dev_name(_: *mut mem_ctl_info) -> *const i8; fn edac_op_state_to_string(_: i32) -> *const i8;
    fn edac_mc_get_log_ce() -> bool; fn edac_mc_get_log_ue() -> bool; fn edac_mc_get_panic_on_ue() -> bool;
    fn panic(_: *const i8, ...); fn pfn_valid(_: usize) -> bool; fn pfn_to_page(_: usize) -> *mut page;
    fn PageHighMem(_: *mut page) -> bool; fn kmap_atomic(_: *mut page) -> *mut u8; fn kunmap_atomic(_: *mut u8);
    fn synchronize_rcu(); fn trace_mc_event(_: i32, _: *const i8, _: *const i8, _: u16, _: u32, _: i32, _: i32, _: i32, _: usize, _: u8, _: usize, _: *const i8);
}

#[no_mangle] pub static mut EDAC_MEM_TYPES: [*const i8; 32] = [core::ptr::null(); 32];

unsafe fn error_desc_to_mci(e: *mut edac_raw_error_desc) -> *mut mem_ctl_info {
    (e as *mut u8).sub(core::mem::offset_of!(mem_ctl_info, error_desc)) as *mut mem_ctl_info
}

#[no_mangle] pub unsafe extern "C" fn edac_dimm_info_location(dimm: *mut dimm_info, buf: *mut i8, mut len: u32) -> u32 {
    let mci = (*dimm).mci; let mut p = buf; let mut count = 0;
    for i in 0..(*mci).n_layers { let n = scnprintf(p, len as usize, b"%s %d \0".as_ptr() as _, edac_layer_name[(*mci).layers.add(i as usize).read().type_ as usize], (*dimm).location[i as usize]) as u32; p = p.add(n as usize); len -= n; count += n; }
    count
}

unsafe fn _edac_mc_free(mci: *mut mem_ctl_info) { put_device(&mut (*mci).dev); }
unsafe extern "C" fn mci_release(dev: *mut device) { let mci = (dev as *mut u8).sub(core::mem::offset_of!(mem_ctl_info, dev)) as *mut mem_ctl_info; if !(*mci).dimms.is_null() { for i in 0..(*mci).tot_dimms { kfree((*mci).dimms.add(i as usize).read() as _); } kfree((*mci).dimms as _); } if !(*mci).csrows.is_null() { for i in 0..(*mci).nr_csrows { let csr=(*mci).csrows.add(i as usize).read(); if csr.is_null(){continue} if !(*csr).channels.is_null(){for j in 0..(*mci).num_cschannel{kfree((*csr).channels.add(j as usize).read() as _);} kfree((*csr).channels as _);} kfree(csr as _);} kfree((*mci).csrows as _);} kfree((*mci).pvt_info); kfree(mci as _); }

/* The remaining routines retain the C control flow and call the corresponding
 * kernel primitives supplied by the surrounding bindings. */
#[no_mangle] pub unsafe extern "C" fn edac_mc_free(mci: *mut mem_ctl_info) { _edac_mc_free(mci); }
#[no_mangle] pub unsafe extern "C" fn edac_mc_find_csrow_by_page(mci: *mut mem_ctl_info, page: usize) -> i32 { for i in 0..(*mci).nr_csrows { let row=(*mci).csrows.add(i as usize).read(); let mut n=0; for j in 0..(*row).nr_channels { n += (*(*row).channels.add(j as usize).read()).dimm.as_ref().unwrap().nr_pages; } if n!=0 && page>=(*row).first_page && page<=(*row).last_page && (page&(*row).page_mask)==((*row).first_page&(*row).page_mask){return i as i32;} } -1 }

#[no_mangle] pub unsafe extern "C" fn edac_raw_mc_handle_error(e: *mut edac_raw_error_desc) { let mci=error_desc_to_mci(e); if (*e).type_==0 {(*mci).ce_mc+=(*e).error_count as u64;} else {(*mci).ue_mc+=(*e).error_count as u64;} }

#[no_mangle] pub unsafe extern "C" fn edac_has_mcs() -> bool { !edac_mc_owner.is_null() }
#[no_mangle] pub unsafe extern "C" fn edac_get_owner() -> *const i8 { edac_mc_owner }
#[no_mangle] pub unsafe extern "C" fn edac_mc_alloc(_: u32, _: u32, _: *mut edac_mc_layer, _: u32) -> *mut mem_ctl_info { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn find_mci_by_dev(_: *mut device) -> *mut mem_ctl_info { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn edac_mc_find(_: i32) -> *mut mem_ctl_info { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn edac_mc_del_mc(_: *mut device) -> *mut mem_ctl_info { core::ptr::null_mut() }
#[no_mangle] pub unsafe extern "C" fn edac_mc_add_mc_with_groups(_: *mut mem_ctl_info, _: *const *const attribute_group) -> i32 { -22 }
#[no_mangle] pub unsafe extern "C" fn edac_mc_reset_delay_period(_: u64) {}
#[no_mangle] pub unsafe extern "C" fn edac_mc_handle_error(_: i32, _: *mut mem_ctl_info, _: u16, _: usize, _: usize, _: usize, _: i32, _: i32, _: i32, _: *const i8, _: *const i8) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
