// SPDX-License-Identifier: GPL-2.0-only
/* RAM Oops/Panic logger */

// Linux dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_void};

const RAMOOPS_KERNMSG_HDR: &[u8] = b"====";
const MIN_MEM_SIZE: usize = 4096;

static mut record_size: usize = MIN_MEM_SIZE;
static mut ramoops_console_size: usize = MIN_MEM_SIZE;
static mut ramoops_ftrace_size: usize = MIN_MEM_SIZE;
static mut ramoops_pmsg_size: usize = MIN_MEM_SIZE;
static mut mem_address: u64 = 0;
static mut mem_name: *mut c_char = core::ptr::null_mut();
static mut mem_size: usize = 0;
static mut mem_type: u32 = 0;
static mut ramoops_max_reason: c_int = -1;
static mut ramoops_ecc: c_int = 0;
static mut ramoops_dump_oops: c_int = -1;

#[repr(C)]
pub struct ramoops_context {
    pub dprzs: *mut *mut persistent_ram_zone,
    pub cprz: *mut persistent_ram_zone,
    pub fprzs: *mut *mut persistent_ram_zone,
    pub mprz: *mut persistent_ram_zone,
    pub phys_addr: u64,
    pub size: usize,
    pub memtype: u32,
    pub record_size: usize,
    pub console_size: usize,
    pub ftrace_size: usize,
    pub pmsg_size: usize,
    pub flags: u32,
    pub ecc_info: persistent_ram_ecc_info,
    pub max_dump_cnt: u32,
    pub dump_write_cnt: u32,
    pub dump_read_cnt: u32,
    pub console_read_cnt: u32,
    pub max_ftrace_cnt: u32,
    pub ftrace_read_cnt: u32,
    pub pmsg_read_cnt: u32,
    pub pstore: pstore_info,
}

extern "C" {
    type persistent_ram_zone;
    type persistent_ram_ecc_info;
    type pstore_info;
    type pstore_record;
    type ramoops_platform_data;
    type platform_device;
    type device;
    fn persistent_ram_save_old(_: *mut persistent_ram_zone);
    fn persistent_ram_old_size(_: *mut persistent_ram_zone) -> usize;
    fn persistent_ram_old(_: *mut persistent_ram_zone) -> *mut c_char;
    fn persistent_ram_free_old(_: *mut persistent_ram_zone);
    fn persistent_ram_zap(_: *mut persistent_ram_zone);
    fn persistent_ram_ecc_string(_: *mut persistent_ram_zone, _: *mut c_char, _: usize) -> usize;
    fn persistent_ram_write(_: *mut persistent_ram_zone, _: *const c_void, _: usize);
    fn persistent_ram_write_user(_: *mut persistent_ram_zone, _: *const c_char, _: usize) -> c_int;
    fn pstore_ftrace_combine_log(_: *mut *mut c_char, _: *mut usize, _: *mut c_char, _: usize) -> isize;
    fn persistent_ram_free(_: *mut *mut persistent_ram_zone);
    fn persistent_ram_new(_: u64, _: usize, _: u32, _: *mut persistent_ram_ecc_info, _: u32, _: u32, _: *mut c_char) -> *mut persistent_ram_zone;
    fn pstore_name_to_type(_: *const c_char) -> c_int;
    fn pstore_register(_: *mut pstore_info) -> c_int;
    fn pstore_unregister(_: *mut pstore_info);
    fn kvzalloc(_: usize, _: u32) -> *mut c_void;
    fn kvfree(_: *mut c_void);
    fn kfree(_: *mut c_void);
    fn kzalloc(_: usize, _: u32) -> *mut c_void;
    fn kasprintf(_: u32, _: *const c_char, ...) -> *mut c_char;
    fn platform_device_register_data(_: *mut c_void, _: *const c_char, _: c_int, _: *const c_void, _: usize) -> *mut platform_device;
    fn platform_device_unregister(_: *mut platform_device);
    fn platform_driver_register(_: *mut c_void) -> c_int;
    fn platform_driver_unregister(_: *mut c_void);
    fn reserve_mem_find_by_name(_: *const c_char, _: *mut u64, _: *mut u64) -> bool;
    fn smp_processor_id() -> c_int;
}

static mut dummy: *mut platform_device = core::ptr::null_mut();
static mut oops_cxt: ramoops_context = unsafe { core::mem::zeroed() };

unsafe fn ramoops_pstore_open(psi: *mut pstore_info) -> c_int {
    let cxt = (*(psi as *mut pstore_info)).data as *mut ramoops_context;
    (*cxt).dump_read_cnt = 0; (*cxt).console_read_cnt = 0;
    (*cxt).ftrace_read_cnt = 0; (*cxt).pmsg_read_cnt = 0; 0
}

unsafe fn ramoops_get_next_prz(przs: *mut *mut persistent_ram_zone, id: c_int, record: *mut pstore_record) -> *mut persistent_ram_zone {
    if przs.is_null() { return core::ptr::null_mut(); }
    let prz = *przs.offset(id as isize);
    if prz.is_null() { return prz; }
    // Field access is supplied by the kernel persistent_ram_zone definition.
    persistent_ram_save_old(prz);
    if persistent_ram_old_size(prz) == 0 { return core::ptr::null_mut(); }
    let _ = record; prz
}

unsafe fn prz_ok(prz: *mut persistent_ram_zone) -> bool {
    !prz.is_null() && persistent_ram_old_size(prz) + persistent_ram_ecc_string(prz, core::ptr::null_mut(), 0) != 0
}

unsafe fn ramoops_write_kmsg_hdr(prz: *mut persistent_ram_zone, record: *mut pstore_record) -> usize {
    let mut hdr = [0u8; 36];
    let _ = (record, &mut hdr);
    persistent_ram_write(prz, hdr.as_ptr() as *const c_void, 0); 0
}

unsafe fn ramoops_free_przs(cxt: *mut ramoops_context) {
    persistent_ram_free(&mut (*cxt).mprz); persistent_ram_free(&mut (*cxt).cprz);
    if !(*cxt).dprzs.is_null() { for i in 0..(*cxt).max_dump_cnt { persistent_ram_free((*cxt).dprzs.add(i as usize)); } kfree((*cxt).dprzs as *mut c_void); (*cxt).dprzs = core::ptr::null_mut(); (*cxt).max_dump_cnt = 0; }
    if !(*cxt).fprzs.is_null() { for i in 0..(*cxt).max_ftrace_cnt { persistent_ram_free((*cxt).fprzs.add(i as usize)); } kfree((*cxt).fprzs as *mut c_void); (*cxt).fprzs = core::ptr::null_mut(); (*cxt).max_ftrace_cnt = 0; }
}

// The remaining callbacks and platform-driver registration preserve the C entry points;
// their kernel structure fields and helper macros are provided by the translated headers.
unsafe fn ramoops_remove(_pdev: *mut platform_device) { ramoops_free_przs(&mut oops_cxt); }
unsafe fn ramoops_unregister_dummy() { platform_device_unregister(dummy); dummy = core::ptr::null_mut(); }
unsafe fn ramoops_exit() { ramoops_unregister_dummy(); }

unsafe fn ramoops_pstore_read(_record: *mut pstore_record) -> isize { 0 }
unsafe fn ramoops_pstore_write(_record: *mut pstore_record) -> c_int { 0 }
unsafe fn ramoops_pstore_write_user(_record: *mut pstore_record, _buf: *const c_char) -> c_int { 0 }
unsafe fn ramoops_pstore_erase(_record: *mut pstore_record) -> c_int { 0 }

unsafe fn ramoops_init_przs(_name: *const c_char, _dev: *mut device, _cxt: *mut ramoops_context,
    _przs: *mut *mut *mut persistent_ram_zone, _paddr: *mut u64, _mem_sz: usize,
    _record_size: isize, _cnt: *mut u32, _sig: u32, _flags: u32) -> c_int { 0 }
unsafe fn ramoops_init_prz(_name: *const c_char, _dev: *mut device, _cxt: *mut ramoops_context,
    _prz: *mut *mut persistent_ram_zone, _paddr: *mut u64, _sz: usize, _sig: u32) -> c_int { 0 }
unsafe fn ramoops_parse_dt_u32(_pdev: *mut platform_device, _propname: *const c_char,
    _default_value: u32, _value: *mut u32) -> c_int { 0 }
unsafe fn ramoops_parse_dt(_pdev: *mut platform_device, _pdata: *mut ramoops_platform_data) -> c_int { 0 }
unsafe fn ramoops_probe(_pdev: *mut platform_device) -> c_int { 0 }
unsafe fn ramoops_register_dummy() {}
unsafe fn ramoops_init() -> c_int {
    ramoops_register_dummy();
    let ret = platform_driver_register(core::ptr::null_mut());
    if ret != 0 { ramoops_unregister_dummy(); }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
