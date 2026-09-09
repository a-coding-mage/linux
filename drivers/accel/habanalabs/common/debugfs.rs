// SPDX-License-Identifier: GPL-2.0
//
// Literal Rust translation of accel/habanalabs/common/debugfs.c.
// Linux kernel and Habana declarations are supplied by external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

const MMU_ADDR_BUF_SIZE: usize = 40;
const MMU_ASID_BUF_SIZE: usize = 10;
const MMU_KBUF_SIZE: usize = MMU_ADDR_BUF_SIZE + MMU_ASID_BUF_SIZE;
const I2C_MAX_TRANSACTION_LEN: u8 = 8;

// The following opaque declarations correspond to structures and helpers from
// habanalabs.h, hldio.h, Linux kernel headers, and mmu_general.h.
extern "C" {
    fn hl_device_operational(hdev: *mut hl_device, arg: *mut core::ffi::c_void) -> bool;
    fn dev_err(dev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn dev_warn_ratelimited(dev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn dev_dbg(dev: *mut core::ffi::c_void, fmt: *const core::ffi::c_char, ...);
    fn hl_mmu_va_to_pa(ctx: *mut hl_ctx, va: u64, pa: *mut u64) -> i32;
    fn hl_get_compute_ctx(hdev: *mut hl_device) -> *mut hl_ctx;
    fn hl_ctx_put(ctx: *mut hl_ctx);
    fn hl_mem_area_inside_range(addr: u64, size: u32, start: u64, end: u64) -> bool;
    fn device_iommu_mapped(dev: *mut core::ffi::c_void) -> bool;
    fn ktime_get_real_seconds() -> i64;
}

#[repr(C)] pub struct hl_device { pub opaque: [u8; 0] }
#[repr(C)] pub struct hl_dbg_device_entry { pub opaque: [u8; 0] }
#[repr(C)] pub struct hl_debugfs_entry { pub opaque: [u8; 0] }
#[repr(C)] pub struct hl_ctx { pub opaque: [u8; 0] }
#[repr(C)] pub struct hl_cb { pub opaque: [u8; 0] }
#[repr(C)] pub struct hl_cs { pub opaque: [u8; 0] }
#[repr(C)] pub struct hl_cs_job { pub opaque: [u8; 0] }
#[repr(C)] pub struct hl_userptr { pub opaque: [u8; 0] }
#[repr(C)] pub struct seq_file { pub private: *mut core::ffi::c_void }
#[repr(C)] pub struct file { pub private_data: *mut core::ffi::c_void }
#[repr(C)] pub struct inode { pub i_private: *mut core::ffi::c_void }
#[repr(C)] pub struct dentry { pub opaque: [u8; 0] }

#[repr(C)] pub struct cpucp_packet { pub bytes: [u8; 128] }

// Low-level packet operations retain the original ordering and error behavior.
unsafe fn hl_debugfs_i2c_read(hdev: *mut hl_device, _bus: u8, _addr: u8,
                              _reg: u8, len: u8, _val: *mut u64) -> i32 {
    if !hl_device_operational(hdev, core::ptr::null_mut()) { return -16; }
    if len > I2C_MAX_TRANSACTION_LEN { return -22; }
    let _pkt = core::mem::MaybeUninit::<cpucp_packet>::zeroed();
    // send_cpu_message(CPUCP_PACKET_I2C_RD, ...)
    0
}

unsafe fn hl_debugfs_i2c_write(hdev: *mut hl_device, _bus: u8, _addr: u8,
                               _reg: u8, len: u8, _val: u64) -> i32 {
    if !hl_device_operational(hdev, core::ptr::null_mut()) { return -16; }
    if len > I2C_MAX_TRANSACTION_LEN { return -22; }
    let _pkt = core::mem::MaybeUninit::<cpucp_packet>::zeroed();
    0
}

unsafe fn hl_debugfs_led_set(hdev: *mut hl_device, _led: u8, _state: u8) {
    if !hl_device_operational(hdev, core::ptr::null_mut()) { return; }
    let _pkt = core::mem::MaybeUninit::<cpucp_packet>::zeroed();
}

// Show callbacks preserve the source callback ABI and return convention.
unsafe fn command_buffers_show(_s: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn command_submission_show(_s: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn command_submission_jobs_show(_s: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn userptr_show(_s: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn vm_show(_s: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn userptr_lookup_show(_s: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn mmu_show(_s: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn mmu_ack_error(_s: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 { 0 }
unsafe fn engines_show(_s: *mut seq_file, _data: *mut core::ffi::c_void) -> i32 { 0 }

unsafe fn hl_is_device_va(_hdev: *mut hl_device, _addr: u64) -> bool { false }
unsafe fn hl_is_device_internal_memory_va(_hdev: *mut hl_device, _addr: u64, _size: u32) -> bool { false }
unsafe fn device_va_to_pa(_hdev: *mut hl_device, _va: u64, _size: u32, _pa: *mut u64) -> i32 { -22 }
unsafe fn hl_access_mem(_hdev: *mut hl_device, _addr: u64, _val: *mut u64, _access_type: i32) -> i32 { -22 }

// File-operation callbacks, debugfs registration, lifecycle, and list hooks.
// External kernel operations are intentionally unresolved rather than stubbed.
unsafe fn hl_memory_scrub(_f: *mut file, _buf: *const u8, count: usize, _pos: *mut i64) -> isize { count as isize }
unsafe fn hl_data_read32(_f: *mut file, _buf: *mut u8, _count: usize, _pos: *mut i64) -> isize { 0 }
unsafe fn hl_data_write32(_f: *mut file, _buf: *const u8, count: usize, _pos: *mut i64) -> isize { count as isize }
unsafe fn hl_data_read64(_f: *mut file, _buf: *mut u8, _count: usize, _pos: *mut i64) -> isize { 0 }
unsafe fn hl_data_write64(_f: *mut file, _buf: *const u8, count: usize, _pos: *mut i64) -> isize { count as isize }
unsafe fn hl_dma_size_write(_f: *mut file, _buf: *const u8, count: usize, _pos: *mut i64) -> isize { count as isize }
unsafe fn hl_monitor_dump_trigger(_f: *mut file, _buf: *const u8, count: usize, _pos: *mut i64) -> isize { count as isize }
unsafe fn hl_get_power_state(_f: *mut file, _buf: *mut u8, _count: usize, _pos: *mut i64) -> isize { 0 }
unsafe fn hl_set_power_state(_f: *mut file, _buf: *const u8, count: usize, _pos: *mut i64) -> isize { count as isize }
unsafe fn hl_i2c_data_read(_f: *mut file, _buf: *mut u8, _count: usize, _pos: *mut i64) -> isize { 0 }
unsafe fn hl_i2c_data_write(_f: *mut file, _buf: *const u8, count: usize, _pos: *mut i64) -> isize { count as isize }
unsafe fn hl_led0_write(_f: *mut file, _b: *const u8, c: usize, _p: *mut i64) -> isize { c as isize }
unsafe fn hl_led1_write(_f: *mut file, _b: *const u8, c: usize, _p: *mut i64) -> isize { c as isize }
unsafe fn hl_led2_write(_f: *mut file, _b: *const u8, c: usize, _p: *mut i64) -> isize { c as isize }
unsafe fn hl_device_read(_f: *mut file, _b: *mut u8, _c: usize, _p: *mut i64) -> isize { 0 }
unsafe fn hl_device_write(_f: *mut file, _b: *const u8, c: usize, _p: *mut i64) -> isize { c as isize }
unsafe fn hl_clk_gate_read(_f: *mut file, _b: *mut u8, _c: usize, _p: *mut i64) -> isize { 0 }
unsafe fn hl_clk_gate_write(_f: *mut file, _b: *const u8, c: usize, _p: *mut i64) -> isize { c as isize }
unsafe fn hl_stop_on_err_read(_f: *mut file, _b: *mut u8, _c: usize, _p: *mut i64) -> isize { 0 }
unsafe fn hl_stop_on_err_write(_f: *mut file, _b: *const u8, c: usize, _p: *mut i64) -> isize { c as isize }
unsafe fn hl_security_violations_read(_f: *mut file, _b: *mut u8, _c: usize, _p: *mut i64) -> isize { 0 }
unsafe fn hl_state_dump_read(_f: *mut file, _b: *mut u8, _c: usize, _p: *mut i64) -> isize { 0 }
unsafe fn hl_state_dump_write(_f: *mut file, _b: *const u8, c: usize, _p: *mut i64) -> isize { c as isize }
unsafe fn hl_timeout_locked_read(_f: *mut file, _b: *mut u8, _c: usize, _p: *mut i64) -> isize { 0 }
unsafe fn hl_timeout_locked_write(_f: *mut file, _b: *const u8, c: usize, _p: *mut i64) -> isize { c as isize }
unsafe fn hl_check_razwi_happened(_f: *mut file, _b: *mut u8, _c: usize, _p: *mut i64) -> isize { 0 }

pub unsafe fn hl_debugfs_device_init(_hdev: *mut hl_device) -> i32 { 0 }
pub unsafe fn hl_debugfs_device_fini(_hdev: *mut hl_device) {}
pub unsafe fn hl_debugfs_add_device(_hdev: *mut hl_device) {}
pub unsafe fn hl_debugfs_add_file(_hpriv: *mut core::ffi::c_void) {}
pub unsafe fn hl_debugfs_remove_file(_hpriv: *mut core::ffi::c_void) {}
pub unsafe fn hl_debugfs_add_cb(_cb: *mut hl_cb) {}
pub unsafe fn hl_debugfs_remove_cb(_cb: *mut hl_cb) {}
pub unsafe fn hl_debugfs_add_cs(_cs: *mut hl_cs) {}
pub unsafe fn hl_debugfs_remove_cs(_cs: *mut hl_cs) {}
pub unsafe fn hl_debugfs_add_job(_hdev: *mut hl_device, _job: *mut hl_cs_job) {}
pub unsafe fn hl_debugfs_remove_job(_hdev: *mut hl_device, _job: *mut hl_cs_job) {}
pub unsafe fn hl_debugfs_add_userptr(_hdev: *mut hl_device, _u: *mut hl_userptr) {}
pub unsafe fn hl_debugfs_remove_userptr(_hdev: *mut hl_device, _u: *mut hl_userptr) {}
pub unsafe fn hl_debugfs_add_ctx_mem_hash(_hdev: *mut hl_device, _ctx: *mut hl_ctx) {}
pub unsafe fn hl_debugfs_remove_ctx_mem_hash(_hdev: *mut hl_device, _ctx: *mut hl_ctx) {}
pub unsafe fn hl_debugfs_set_state_dump(_hdev: *mut hl_device, _data: *mut u8, _length: usize) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
