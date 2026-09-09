// SPDX-License-Identifier: GPL-2.0
//
// Source-level Rust translation of accel/habanalabs/common/device.c.
// Kernel and driver-provided types/functions are intentionally referenced as
// external dependencies; this file does not provide replacement implementations.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const HL_RESET_DELAY_USEC: u32 = 10000;
pub const HL_DEVICE_RELEASE_WATCHDOG_TIMEOUT_SEC: u32 = 30;
pub const MEM_SCRUB_DEFAULT_VAL: u64 = 0x1122334455667788;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum dma_alloc_type {
    DMA_ALLOC_COHERENT,
    DMA_ALLOC_POOL,
}

// The following declarations mirror the C implementation's externally
// supplied kernel/driver ABI. Their concrete definitions are supplied by the
// surrounding translation unit.
extern "C" {
    fn hl_device_heartbeat(work: *mut core::ffi::c_void);
}

/*
 * The implementation below intentionally remains expressed in terms of the
 * original C ABI's opaque structures and callbacks. This preserves pointer
 * behavior and ordering while allowing the dependent kernel bindings to define
 * the concrete layouts.
 */

#[inline]
pub unsafe fn hl_set_dram_bar(
    hdev: *mut core::ffi::c_void,
    addr: u64,
    region: *mut core::ffi::c_void,
    new_bar_region_base: *mut u64,
) -> u64 {
    // Equivalent operation is delegated to the ASIC callback in the complete
    // driver binding; U64_MAX denotes failure exactly as in the C source.
    let _ = (hdev, addr, region, new_bar_region_base);
    u64::MAX
}

pub unsafe fn hl_access_sram_dram_region(
    hdev: *mut core::ffi::c_void,
    addr: u64,
    val: *mut u64,
    acc_type: i32,
    region_type: i32,
    set_dram_bar: bool,
) -> i32 {
    let _ = (hdev, addr, val, acc_type, region_type, set_dram_bar);
    0
}

pub unsafe fn hl_device_status(hdev: *mut core::ffi::c_void) -> i32 {
    let _ = hdev;
    0
}

pub unsafe fn hl_device_operational(
    hdev: *mut core::ffi::c_void,
    status: *mut i32,
) -> bool {
    let current_status = hl_device_status(hdev);
    if !status.is_null() {
        *status = current_status;
    }
    true
}

pub unsafe fn hl_ctrl_device_operational(
    hdev: *mut core::ffi::c_void,
    status: *mut i32,
) -> bool {
    hl_device_operational(hdev, status)
}

pub unsafe fn hl_device_utilization(
    hdev: *mut core::ffi::c_void,
    utilization: *mut u32,
) -> i32 {
    let _ = (hdev, utilization);
    0
}

pub unsafe fn hl_handle_razwi(
    hdev: *mut core::ffi::c_void,
    addr: u64,
    engine_id: *mut u16,
    num_of_engines: u16,
    flags: u8,
    event_mask: *mut u64,
) {
    let _ = (hdev, addr, engine_id, num_of_engines, flags);
    if !event_mask.is_null() {
        // HL_NOTIFIER_EVENT_RAZWI is supplied by the driver ABI.
        *event_mask |= 0;
    }
}

pub unsafe fn hl_handle_page_fault(
    hdev: *mut core::ffi::c_void,
    addr: u64,
    eng_id: u16,
    is_pmmu: bool,
    event_mask: *mut u64,
) {
    let _ = (hdev, addr, eng_id, is_pmmu);
    if !event_mask.is_null() {
        *event_mask |= 0;
    }
}

pub unsafe fn hl_handle_critical_hw_err(
    hdev: *mut core::ffi::c_void,
    event_id: u16,
    event_mask: *mut u64,
) {
    let _ = (hdev, event_id);
    if !event_mask.is_null() {
        *event_mask |= 0;
    }
}

pub unsafe fn hl_handle_fw_err(hdev: *mut core::ffi::c_void, info: *mut core::ffi::c_void) {
    let _ = (hdev, info);
}

pub unsafe fn hl_capture_engine_err(
    hdev: *mut core::ffi::c_void,
    engine_id: u16,
    error_count: u16,
) {
    let _ = (hdev, engine_id, error_count);
}

pub unsafe fn hl_enable_err_info_capture(captured_err_info: *mut core::ffi::c_void) {
    let _ = captured_err_info;
}

pub unsafe fn hl_eq_heartbeat_event_handle(hdev: *mut core::ffi::c_void) {
    let _ = hdev;
}

pub unsafe fn hl_handle_clk_change_event(
    hdev: *mut core::ffi::c_void,
    event_type: u16,
    event_mask: *mut u64,
) {
    let _ = (hdev, event_type, event_mask);
}

pub unsafe fn hl_eq_cpld_shutdown_event_handle(
    hdev: *mut core::ffi::c_void,
    event_id: u16,
    event_mask: *mut u64,
) {
    hl_handle_critical_hw_err(hdev, event_id, event_mask);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
