/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2022 Google Corporation
 */

// DEVCOREDUMP_TIMEOUT = msecs_to_jiffies(10000) /* 10 sec */
pub const HCI_DEVCD_HDR_SIZE_MAX: usize = 512;
pub const HCI_DEVCD_HDR_END_MARKER: &str = "--- Start dump ---\n";

pub type CoredumpT = unsafe extern "C" fn(hdev: *mut HciDev);
pub type DmpHdrT = unsafe extern "C" fn(hdev: *mut HciDev, skb: *mut SkBuff);
pub type NotifyChangeT = unsafe extern "C" fn(hdev: *mut HciDev, state: ::core::ffi::c_int);

/* struct hci_devcoredump - Devcoredump state
 *
 * @supported: Indicates if FW dump collection is supported by driver
 * @state: Current state of dump collection
 * @timeout: Indicates a timeout for collecting the devcoredump
 *
 * @alloc_size: Total size of the dump
 * @head: Start of the dump
 * @tail: Pointer to current end of dump
 * @end: head + alloc_size for easy comparisons
 *
 * @dump_q: Dump queue for state machine to process
 * @dump_rx: Devcoredump state machine work
 * @dump_timeout: Devcoredump timeout work
 *
 * @coredump: Called from the driver's .coredump() function.
 * @dmp_hdr: Create a dump header to identify controller/fw/driver info
 * @notify_change: Notify driver when devcoredump state has changed
 */
#[repr(C)]
pub struct HciDevcoredump {
    pub supported: bool,
    pub state: DevcoredumpState,
    pub timeout: ::core::ffi::c_ulong,
    pub alloc_size: usize,
    pub head: *mut ::core::ffi::c_char,
    pub tail: *mut ::core::ffi::c_char,
    pub end: *mut ::core::ffi::c_char,
    pub dump_q: SkBuffHead,
    pub dump_rx: WorkStruct,
    pub dump_timeout: DelayedWork,
    pub coredump: Option<CoredumpT>,
    pub dmp_hdr: Option<DmpHdrT>,
    pub notify_change: Option<NotifyChangeT>,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum DevcoredumpState {
    HciDevcoredumpIdle,
    HciDevcoredumpActive,
    HciDevcoredumpDone,
    HciDevcoredumpAbort,
    HciDevcoredumpTimeout,
}

#[cfg(CONFIG_DEV_COREDUMP)]
extern "C" {
    pub fn hci_devcd_state_name(state: DevcoredumpState) -> *const ::core::ffi::c_char;
    pub fn hci_devcd_reset(hdev: *mut HciDev);
    pub fn hci_devcd_rx(work: *mut WorkStruct);
    pub fn hci_devcd_timeout(work: *mut WorkStruct);
    pub fn hci_devcd_register(
        hdev: *mut HciDev,
        coredump: Option<CoredumpT>,
        dmp_hdr: Option<DmpHdrT>,
        notify_change: Option<NotifyChangeT>,
    ) -> ::core::ffi::c_int;
    pub fn hci_devcd_init(hdev: *mut HciDev, dump_size: u32) -> ::core::ffi::c_int;
    pub fn hci_devcd_append(hdev: *mut HciDev, skb: *mut SkBuff) -> ::core::ffi::c_int;
    pub fn hci_devcd_append_pattern(
        hdev: *mut HciDev,
        pattern: u8,
        len: u32,
    ) -> ::core::ffi::c_int;
    pub fn hci_devcd_complete(hdev: *mut HciDev) -> ::core::ffi::c_int;
    pub fn hci_devcd_abort(hdev: *mut HciDev) -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn hci_devcd_state_name(_state: DevcoredumpState) -> *const ::core::ffi::c_char {
    b"\0".as_ptr() as *const ::core::ffi::c_char
}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn hci_devcd_reset(_hdev: *mut HciDev) {}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn hci_devcd_rx(_work: *mut WorkStruct) {}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn hci_devcd_timeout(_work: *mut WorkStruct) {}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn hci_devcd_register(
    _hdev: *mut HciDev,
    _coredump: Option<CoredumpT>,
    _dmp_hdr: Option<DmpHdrT>,
    _notify_change: Option<NotifyChangeT>,
) -> ::core::ffi::c_int {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn hci_devcd_init(_hdev: *mut HciDev, _dump_size: u32) -> ::core::ffi::c_int {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn hci_devcd_append(_hdev: *mut HciDev, _skb: *mut SkBuff) -> ::core::ffi::c_int {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn hci_devcd_append_pattern(
    _hdev: *mut HciDev,
    _pattern: u8,
    _len: u32,
) -> ::core::ffi::c_int {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn hci_devcd_complete(_hdev: *mut HciDev) -> ::core::ffi::c_int {
    -EOPNOTSUPP
}

#[cfg(not(CONFIG_DEV_COREDUMP))]
pub unsafe fn hci_devcd_abort(_hdev: *mut HciDev) -> ::core::ffi::c_int {
    -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
