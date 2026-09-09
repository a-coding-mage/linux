/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(i32)]
pub enum FwOpt {
    FW_OPT_UEVENT = 1 << 0,
    FW_OPT_NOWAIT = 1 << 1,
    FW_OPT_USERHELPER = 1 << 2,
    FW_OPT_NO_WARN = 1 << 3,
    FW_OPT_NOCACHE = 1 << 4,
    FW_OPT_NOFALLBACK_SYSFS = 1 << 5,
    FW_OPT_FALLBACK_PLATFORM = 1 << 6,
    FW_OPT_PARTIAL = 1 << 7,
}

#[repr(i32)]
pub enum FwStatus {
    FW_STATUS_UNKNOWN,
    FW_STATUS_LOADING,
    FW_STATUS_DONE,
    FW_STATUS_ABORTED,
}

#[repr(C)]
pub struct FwState {
    pub completion: Completion,
    pub status: FwStatus,
}

#[repr(C)]
pub struct FwPriv {
    pub ref_: Kref,
    pub list: ListHead,
    pub fwc: *mut FirmwareCache,
    pub fw_st: FwState,
    pub data: *mut core::ffi::c_void,
    pub size: usize,
    pub allocated_size: usize,
    pub offset: usize,
    pub opt_flags: u32,
    #[cfg(CONFIG_FW_LOADER_PAGED_BUF)]
    pub is_paged_buf: bool,
    #[cfg(CONFIG_FW_LOADER_PAGED_BUF)]
    pub pages: *mut *mut Page,
    #[cfg(CONFIG_FW_LOADER_PAGED_BUF)]
    pub nr_pages: i32,
    #[cfg(CONFIG_FW_LOADER_PAGED_BUF)]
    pub page_array_size: i32,
    #[cfg(CONFIG_FW_LOADER_USER_HELPER)]
    pub need_uevent: bool,
    #[cfg(CONFIG_FW_LOADER_USER_HELPER)]
    pub pending_list: ListHead,
    pub fw_name: *const core::ffi::c_char,
}

extern "C" {
    pub static mut fw_lock: Mutex;
    pub static mut fw_cache: FirmwareCache;
    pub static mut fw_load_abort_all: bool;

    pub fn wait_for_completion_killable_timeout(completion: *mut Completion, timeout: i64) -> i64;
    pub fn complete_all(completion: *mut Completion);
    pub fn list_del_init(list: *mut ListHead);
    pub fn write_once<T>(ptr: *mut T, value: T);

    pub fn alloc_lookup_fw_priv(
        fw_name: *const core::ffi::c_char,
        fwc: *mut FirmwareCache,
        fw_priv: *mut *mut FwPriv,
        dbuf: *mut core::ffi::c_void,
        size: usize,
        offset: usize,
        opt_flags: u32,
    ) -> i32;
    pub fn assign_fw(fw: *mut Firmware, device: *mut Device) -> i32;
    pub fn free_fw_priv(fw_priv: *mut FwPriv);
    pub fn fw_state_init(fw_priv: *mut FwPriv);
}

#[inline]
pub unsafe fn __fw_state_check(fw_priv: *mut FwPriv, status: FwStatus) -> bool {
    (*fw_priv).fw_st.status == status
}

#[inline]
pub unsafe fn __fw_state_wait_common(fw_priv: *mut FwPriv, timeout: i64) -> i32 {
    let fw_st = &mut (*fw_priv).fw_st;
    let ret = wait_for_completion_killable_timeout(&mut fw_st.completion, timeout);
    if ret != 0 && fw_st.status == FwStatus::FW_STATUS_ABORTED {
        return -2; // -ENOENT
    }
    if ret == 0 {
        return -110; // -ETIMEDOUT
    }
    if ret < 0 { ret as i32 } else { 0 }
}

#[inline]
pub unsafe fn __fw_state_set(fw_priv: *mut FwPriv, status: FwStatus) {
    let fw_st = &mut (*fw_priv).fw_st;
    write_once(&mut fw_st.status, status);
    if status == FwStatus::FW_STATUS_DONE || status == FwStatus::FW_STATUS_ABORTED {
        #[cfg(CONFIG_FW_LOADER_USER_HELPER)]
        list_del_init(&mut (*fw_priv).pending_list);
        complete_all(&mut fw_st.completion);
    }
}

#[inline]
pub unsafe fn fw_state_aborted(fw_priv: *mut FwPriv) { __fw_state_set(fw_priv, FwStatus::FW_STATUS_ABORTED); }
#[inline]
pub unsafe fn fw_state_is_aborted(fw_priv: *mut FwPriv) -> bool { __fw_state_check(fw_priv, FwStatus::FW_STATUS_ABORTED) }
#[inline]
pub unsafe fn fw_state_start(fw_priv: *mut FwPriv) { __fw_state_set(fw_priv, FwStatus::FW_STATUS_LOADING); }
#[inline]
pub unsafe fn fw_state_done(fw_priv: *mut FwPriv) { __fw_state_set(fw_priv, FwStatus::FW_STATUS_DONE); }
#[inline]
pub unsafe fn fw_state_is_done(fw_priv: *mut FwPriv) -> bool { __fw_state_check(fw_priv, FwStatus::FW_STATUS_DONE) }
#[inline]
pub unsafe fn fw_state_is_loading(fw_priv: *mut FwPriv) -> bool { __fw_state_check(fw_priv, FwStatus::FW_STATUS_LOADING) }

#[cfg(CONFIG_FW_LOADER)]
extern "C" {
    pub fn firmware_is_builtin(fw: *const Firmware) -> bool;
    pub fn firmware_request_builtin_buf(fw: *mut Firmware, name: *const core::ffi::c_char, buf: *mut core::ffi::c_void, size: usize) -> bool;
}

#[cfg(not(CONFIG_FW_LOADER))]
#[inline]
pub unsafe fn firmware_is_builtin(_fw: *const Firmware) -> bool { false }
#[cfg(not(CONFIG_FW_LOADER))]
#[inline]
pub unsafe fn firmware_request_builtin_buf(_fw: *mut Firmware, _name: *const core::ffi::c_char, _buf: *mut core::ffi::c_void, _size: usize) -> bool { false }

#[cfg(CONFIG_FW_LOADER_PAGED_BUF)]
extern "C" {
    pub fn fw_free_paged_buf(fw_priv: *mut FwPriv);
    pub fn fw_grow_paged_buf(fw_priv: *mut FwPriv, pages_needed: i32) -> i32;
    pub fn fw_map_paged_buf(fw_priv: *mut FwPriv) -> i32;
    pub fn fw_is_paged_buf(fw_priv: *mut FwPriv) -> bool;
}

#[cfg(not(CONFIG_FW_LOADER_PAGED_BUF))]
#[inline]
pub unsafe fn fw_free_paged_buf(_fw_priv: *mut FwPriv) {}
#[cfg(not(CONFIG_FW_LOADER_PAGED_BUF))]
#[inline]
pub unsafe fn fw_grow_paged_buf(_fw_priv: *mut FwPriv, _pages_needed: i32) -> i32 { -6 /* -ENXIO */ }
#[cfg(not(CONFIG_FW_LOADER_PAGED_BUF))]
#[inline]
pub unsafe fn fw_map_paged_buf(_fw_priv: *mut FwPriv) -> i32 { -6 /* -ENXIO */ }
#[cfg(not(CONFIG_FW_LOADER_PAGED_BUF))]
#[inline]
pub unsafe fn fw_is_paged_buf(_fw_priv: *mut FwPriv) -> bool { false }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
