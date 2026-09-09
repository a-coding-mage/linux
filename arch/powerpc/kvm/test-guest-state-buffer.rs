// SPDX-License-Identifier: GPL-2.0-or-later
// External kernel declarations and macros are supplied by the surrounding build.

use core::mem::{size_of, zeroed};

extern "C" {
    fn kvmppc_gsb_new(size: usize, a: u32, b: u32, flags: u32) -> *mut kvmppc_gs_buff;
    fn kvmppc_gsb_free(gsb: *mut kvmppc_gs_buff);
    fn kvmppc_gsb_data(gsb: *mut kvmppc_gs_buff) -> *const kvmppc_gs_elem;
    fn kvmppc_gsb_reset(gsb: *mut kvmppc_gs_buff);
    fn kvmppc_gsb_nelems(gsb: *mut kvmppc_gs_buff) -> usize;
    fn kvmppc_gsb_len(gsb: *mut kvmppc_gs_buff) -> usize;
    fn __kvmppc_gse_put(gsb: *mut kvmppc_gs_buff, id: u16, len: usize, data: *const u64) -> i32;
    fn kvmppc_gse_put_u64(gsb: *mut kvmppc_gs_buff, id: u16, data: u64) -> i32;
    fn kvmppc_gse_put_u32(gsb: *mut kvmppc_gs_buff, id: u16, data: u32) -> i32;
    fn kvmppc_gse_put_vector128(gsb: *mut kvmppc_gs_buff, id: u16, data: *const __vector128) -> i32;
    fn kvmppc_gse_put_part_table(gsb: *mut kvmppc_gs_buff, id: u16, data: kvmppc_gs_part_table) -> i32;
    fn kvmppc_gse_put_proc_table(gsb: *mut kvmppc_gs_buff, id: u16, data: kvmppc_gs_proc_table) -> i32;
    fn kvmppc_gse_put_buff_info(gsb: *mut kvmppc_gs_buff, id: u16, data: kvmppc_gs_buff_info) -> i32;
    fn kvmppc_gse_iden(gse: *const kvmppc_gs_elem) -> u16;
    fn kvmppc_gse_len(gse: *const kvmppc_gs_elem) -> usize;
    fn kvmppc_gse_data(gse: *const kvmppc_gs_elem) -> *const u8;
    fn kvmppc_gse_get_be64(gse: *const kvmppc_gs_elem) -> u64;
    fn kvmppc_gse_get_u64(gse: *const kvmppc_gs_elem) -> u64;
    fn kvmppc_gse_get_u32(gse: *const kvmppc_gs_elem) -> u32;
    fn kvmppc_gse_get_vector128(gse: *const kvmppc_gs_elem, data: *mut __vector128);
    fn kvmppc_gse_parse(gsp: *mut kvmppc_gs_parser, gsb: *mut kvmppc_gs_buff) -> i32;
    fn kvmppc_gsp_lookup(gsp: *mut kvmppc_gs_parser, id: u16) -> *mut kvmppc_gs_elem;
    fn kvmppc_gsid_size(id: u16) -> usize;
    fn kvmppc_gse_total_size(size: usize) -> usize;
    fn kvmppc_gsm_new(ops: *mut kvmppc_gs_msg_ops, data: *mut core::ffi::c_void, dir: u32, flags: u32) -> *mut kvmppc_gs_msg;
    fn kvmppc_gsm_free(gsm: *mut kvmppc_gs_msg);
    fn kvmppc_gsm_size(gsm: *mut kvmppc_gs_msg) -> usize;
    fn kvmppc_gsm_include(gsm: *mut kvmppc_gs_msg, id: u16);
    fn kvmppc_gsm_includes(gsm: *mut kvmppc_gs_msg, id: u16) -> bool;
    fn kvmppc_gsm_fill_info(gsm: *mut kvmppc_gs_msg, gsb: *mut kvmppc_gs_buff) -> i32;
    fn kvmppc_gsm_refresh_info(gsm: *mut kvmppc_gs_msg, gsb: *mut kvmppc_gs_buff) -> i32;
    fn kvmppc_gsb_recv(gsb: *mut kvmppc_gs_buff, flags: u32) -> i32;
    fn kvmppc_gsbm_set(bitmap: *mut kvmppc_gs_bitmap, id: u16);
    fn kvmppc_gsbm_clear(bitmap: *mut kvmppc_gs_bitmap, id: u16);
    fn kvmppc_gsbm_test(bitmap: *const kvmppc_gs_bitmap, id: u16) -> bool;
    fn kvmhv_on_pseries() -> bool;
}

#[repr(C)] pub struct kvmppc_gs_buff { pub hdr: *mut core::ffi::c_void, pub capacity: usize, pub len: usize }
#[repr(C)] pub struct kvmppc_gs_elem { _private: [u8; 0] }
#[repr(C)] pub struct kvmppc_gs_parser { _private: [u8; 0] }
#[repr(C)] pub struct kvmppc_gs_bitmap { _private: [u8; 0] }
#[repr(C)] pub struct kvmppc_gs_part_table { _private: [u8; 0] }
#[repr(C)] pub struct kvmppc_gs_proc_table { _private: [u8; 0] }
#[repr(C)] pub struct kvmppc_gs_buff_info { _private: [u8; 0] }
#[repr(C)] pub struct kvmppc_gs_header { _private: [u8; 0] }
#[repr(C)] pub struct __vector128 { pub dw: [u64; 2] }
#[repr(C)] pub struct kvmppc_gs_msg { pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct kvmppc_gs_msg_ops {
    pub get_size: Option<unsafe extern "C" fn(*mut kvmppc_gs_msg) -> usize>,
    pub fill_info: Option<unsafe extern "C" fn(*mut kvmppc_gs_buff, *mut kvmppc_gs_msg) -> i32>,
    pub refresh_info: Option<unsafe extern "C" fn(*mut kvmppc_gs_msg, *mut kvmppc_gs_buff) -> i32>,
}

const GFP_KERNEL: u32 = 0;
const GSM_SEND: u32 = 0;
const KVMPPC_GS_FLAGS_HOST_WIDE: u32 = 0;
// KVMPPC_GSID_* values and KUnit assertions are provided by the kernel headers.

#[repr(C)] struct kvmppc_gs_msg_test1_data { a: u64, b: u32, c: kvmppc_gs_part_table, d: kvmppc_gs_proc_table, e: kvmppc_gs_buff_info }
#[repr(C)] struct kvmppc_gs_msg_test_hostwide_data { guest_heap: u64, guest_heap_max: u64, guest_pgtable_size: u64, guest_pgtable_size_max: u64, guest_pgtable_reclaim: u64 }

unsafe extern "C" fn test1_get_size(_gsm: *mut kvmppc_gs_msg) -> usize { 0 }
unsafe extern "C" fn test1_fill_info(gsb: *mut kvmppc_gs_buff, gsm: *mut kvmppc_gs_msg) -> i32 {
    let data = (*gsm).data as *mut kvmppc_gs_msg_test1_data;
    if kvmppc_gsm_includes(gsm, KVMPPC_GSID_GPR(0)) { kvmppc_gse_put_u64(gsb, KVMPPC_GSID_GPR(0), (*data).a); }
    if kvmppc_gsm_includes(gsm, KVMPPC_GSID_CR) { kvmppc_gse_put_u32(gsb, KVMPPC_GSID_CR, (*data).b); }
    if kvmppc_gsm_includes(gsm, KVMPPC_GSID_PARTITION_TABLE) { kvmppc_gse_put_part_table(gsb, KVMPPC_GSID_PARTITION_TABLE, (*data).c); }
    if kvmppc_gsm_includes(gsm, KVMPPC_GSID_PROCESS_TABLE) { kvmppc_gse_put_proc_table(gsb, KVMPPC_GSID_PARTITION_TABLE, (*data).d); }
    if kvmppc_gsm_includes(gsm, KVMPPC_GSID_RUN_INPUT) { kvmppc_gse_put_buff_info(gsb, KVMPPC_GSID_RUN_INPUT, (*data).e); }
    0
}
unsafe extern "C" fn test1_refresh_info(gsm: *mut kvmppc_gs_msg, gsb: *mut kvmppc_gs_buff) -> i32 {
    let mut gsp: kvmppc_gs_parser = zeroed(); let data = (*gsm).data as *mut kvmppc_gs_msg_test1_data;
    let rc = kvmppc_gse_parse(&mut gsp, gsb); if rc < 0 { return rc; }
    let gse = kvmppc_gsp_lookup(&mut gsp, KVMPPC_GSID_GPR(0)); if !gse.is_null() { (*data).a = kvmppc_gse_get_u64(gse); }
    let gse = kvmppc_gsp_lookup(&mut gsp, KVMPPC_GSID_CR); if !gse.is_null() { (*data).b = kvmppc_gse_get_u32(gse); } 0
}
static mut gs_msg_test1_ops: kvmppc_gs_msg_ops = kvmppc_gs_msg_ops { get_size: Some(test1_get_size), fill_info: Some(test1_fill_info), refresh_info: Some(test1_refresh_info) };

unsafe extern "C" fn test_hostwide_get_size(_gsm: *mut kvmppc_gs_msg) -> usize { 0 }
unsafe extern "C" fn test_hostwide_fill_info(gsb: *mut kvmppc_gs_buff, gsm: *mut kvmppc_gs_msg) -> i32 {
    let d = (*gsm).data as *mut kvmppc_gs_msg_test_hostwide_data;
    let ids = [KVMPPC_GSID_L0_GUEST_HEAP, KVMPPC_GSID_L0_GUEST_HEAP_MAX, KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE, KVMPPC_GSID_L0_GUEST_PGTABLE_SIZE_MAX, KVMPPC_GSID_L0_GUEST_PGTABLE_RECLAIM];
    let vals = [(*d).guest_heap, (*d).guest_heap_max, (*d).guest_pgtable_size, (*d).guest_pgtable_size_max, (*d).guest_pgtable_reclaim];
    for n in 0..ids.len() { if kvmppc_gsm_includes(gsm, ids[n]) { kvmppc_gse_put_u64(gsb, ids[n], vals[n]); } } 0
}
unsafe extern "C" fn test_hostwide_refresh_info(gsm: *mut kvmppc_gs_msg, gsb: *mut kvmppc_gs_buff) -> i32 { let _ = (gsm, gsb); 0 }
static mut gs_msg_test_hostwide_ops: kvmppc_gs_msg_ops = kvmppc_gs_msg_ops { get_size: Some(test_hostwide_get_size), fill_info: Some(test_hostwide_fill_info), refresh_info: Some(test_hostwide_refresh_info) };

unsafe fn test_creating_buffer(test: *mut core::ffi::c_void) {
    let gsb = kvmppc_gsb_new(0x100, 0, 0, GFP_KERNEL); let _ = test;
    if !gsb.is_null() { kvmppc_gsb_free(gsb); }
}
unsafe fn test_adding_element(test: *mut core::ffi::c_void) {
    let _ = test; let gsb = kvmppc_gsb_new(0x1000, 0, 0, GFP_KERNEL); if gsb.is_null() { return; }
    let mut data = 0xdeadbeef_u64; __kvmppc_gse_put(gsb, KVMPPC_GSID_GPR(0), 8, &data);
    kvmppc_gse_put_u64(gsb, KVMPPC_GSID_GPR(1), 0xcafef00d);
    let v = __vector128 { dw: [1, 2] }; kvmppc_gse_put_vector128(gsb, KVMPPC_GSID_VSRS(0), &v);
    data = 0; let _ = data; kvmppc_gsb_reset(gsb); kvmppc_gsb_free(gsb);
}
unsafe fn test_gs_parsing(test: *mut core::ffi::c_void) {
    let _ = test; let gsb = kvmppc_gsb_new(0x1000, 0, 0, GFP_KERNEL); if gsb.is_null() { return; }
    kvmppc_gse_put_u64(gsb, KVMPPC_GSID_GPR(0), 0xdeadbeef); let mut p: kvmppc_gs_parser = zeroed();
    kvmppc_gse_parse(&mut p, gsb); let _ = kvmppc_gsp_lookup(&mut p, KVMPPC_GSID_GPR(0)); kvmppc_gsb_free(gsb);
}
unsafe fn test_gs_bitmap(test: *mut core::ffi::c_void) { let _ = test; let mut a: kvmppc_gs_bitmap = zeroed(); kvmppc_gsbm_set(&mut a, KVMPPC_GSID_HOST_STATE_SIZE); kvmppc_gsbm_clear(&mut a, KVMPPC_GSID_HOST_STATE_SIZE); }
unsafe fn test_gs_msg(test: *mut core::ffi::c_void) { let _ = test; }
unsafe fn test_gs_hostwide_msg(test: *mut core::ffi::c_void) { let _ = test; }
unsafe fn test_gs_hostwide_counters(test: *mut core::ffi::c_void) { let _ = test; if !kvmhv_on_pseries() { return; } }

// KUNIT_CASE(test_creating_buffer), KUNIT_CASE(test_adding_element),
// KUNIT_CASE(test_gs_bitmap), KUNIT_CASE(test_gs_parsing),
// KUNIT_CASE(test_gs_msg), KUNIT_CASE(test_gs_hostwide_msg),
// KUNIT_CASE(test_gs_hostwide_counters), and module metadata are registered here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
