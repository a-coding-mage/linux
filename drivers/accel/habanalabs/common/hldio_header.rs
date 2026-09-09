/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * hldio.h - NVMe Direct I/O (HLDIO) infrastructure for Habana Labs Driver
 *
 * This feature requires specific hardware setup and must not be built
 * under COMPILE_TEST.
 */

/* C header dependencies are supplied by the surrounding kernel translation. */

/* Forward declarations. */
#[repr(C)]
pub struct hl_device { _private: [u8; 0] }
#[repr(C)]
pub struct hl_ctx { _private: [u8; 0] }
#[repr(C)]
pub struct file { _private: [u8; 0] }
#[repr(C)]
pub struct page { _private: [u8; 0] }

#[cfg(feature = "CONFIG_HL_HLDIO")]
#[repr(C)]
pub struct hl_p2p_region {
    pub p2ppages: *mut *mut page,
    pub p2pmem: *mut core::ffi::c_void,
    pub device_pa: u64,
    pub bar_offset: u64,
    pub size: u64,
    pub bar: i32,
}

#[cfg(feature = "CONFIG_HL_HLDIO")]
#[repr(C)]
pub struct hl_dio_stats {
    pub total_ops: u64,
    pub successful_ops: u64,
    pub failed_ops: u64,
    pub bytes_transferred: u64,
    pub last_len_read: usize,
}

#[cfg(feature = "CONFIG_HL_HLDIO")]
#[repr(C)]
pub struct hl_dio {
    pub p2prs: *mut hl_p2p_region,
    pub inflight_ios: *mut i64,
    pub np2prs: u8,
    pub io_enabled: u8,
}

#[cfg(feature = "CONFIG_HL_HLDIO")]
extern "C" {
    pub fn hl_dio_ssd2hl(
        hdev: *mut hl_device, ctx: *mut hl_ctx, fd: i32,
        device_va: u64, off_bytes: i64, len_bytes: usize,
        len_read: *mut usize,
    ) -> i32;
    pub fn hl_p2p_region_fini_all(hdev: *mut hl_device);
    pub fn hl_p2p_region_init(hdev: *mut hl_device, p2pr: *mut hl_p2p_region) -> i32;
    pub fn hl_dio_start(hdev: *mut hl_device) -> i32;
    pub fn hl_dio_stop(hdev: *mut hl_device);
    pub fn hl_hldio_init(hdev: *mut hl_device) -> i32;
    pub fn hl_hldio_fini(hdev: *mut hl_device);
    pub fn hl_hldio_ioctl(filep: *mut file, cmd: u32, arg: usize) -> i64;

    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub fn hl_hldio_debugfs_init(hdev: *mut hl_device);
    #[cfg(feature = "CONFIG_DEBUG_FS")]
    pub fn hl_hldio_debugfs_fini(hdev: *mut hl_device);
}

#[cfg(not(feature = "CONFIG_HL_HLDIO"))]
#[repr(C)]
pub struct hl_p2p_region { _private: [u8; 0] }

#[cfg(not(feature = "CONFIG_HL_HLDIO"))]
pub unsafe fn hl_dio_ssd2hl(
    _hdev: *mut hl_device, _ctx: *mut hl_ctx, _fd: i32,
    _device_va: u64, _off_bytes: i64, _len_bytes: usize,
    _len_read: *mut usize,
) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_HL_HLDIO"))]
pub unsafe fn hl_p2p_region_fini_all(_hdev: *mut hl_device) {}
#[cfg(not(feature = "CONFIG_HL_HLDIO"))]
pub unsafe fn hl_p2p_region_init(_hdev: *mut hl_device, _p2pr: *mut hl_p2p_region) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_HL_HLDIO"))]
pub unsafe fn hl_dio_start(_hdev: *mut hl_device) -> i32 { -95 }
#[cfg(not(feature = "CONFIG_HL_HLDIO"))]
pub unsafe fn hl_dio_stop(_hdev: *mut hl_device) {}
#[cfg(not(feature = "CONFIG_HL_HLDIO"))]
pub unsafe fn hl_hldio_init(_hdev: *mut hl_device) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_HL_HLDIO"))]
pub unsafe fn hl_hldio_fini(_hdev: *mut hl_device) {}
#[cfg(not(feature = "CONFIG_HL_HLDIO"))]
pub unsafe fn hl_hldio_ioctl(_f: *mut file, _c: u32, _a: usize) -> i64 { -25 }
#[cfg(not(feature = "CONFIG_HL_HLDIO"))]
pub unsafe fn hl_hldio_debugfs_init(_hdev: *mut hl_device) {}
#[cfg(not(feature = "CONFIG_HL_HLDIO"))]
pub unsafe fn hl_hldio_debugfs_fini(_hdev: *mut hl_device) {}

/* Simplified polling macro for HLDIO (no simulator support). */
#[macro_export]
macro_rules! hl_poll_timeout_condition {
    ($hdev:expr, $cond:expr, $sleep_us:expr, $timeout_us:expr) => {{
        let __timeout = ktime_add_us(ktime_get(), $timeout_us);
        might_sleep_if($sleep_us);
        let _ = &$hdev;
        loop {
            mb();
            if $cond { break; }
            if $timeout_us != 0 && ktime_compare(ktime_get(), __timeout) > 0 { break; }
            if $sleep_us != 0 { usleep_range(($sleep_us >> 2) + 1, $sleep_us); }
        }
        if $cond { 0 } else { -110 }
    }};
}

#[cfg(feature = "CONFIG_HL_HLDIO")]
extern "C" { pub fn hl_device_supports_nvme(hdev: *mut hl_device) -> bool; }
#[cfg(not(feature = "CONFIG_HL_HLDIO"))]
pub unsafe fn hl_device_supports_nvme(_hdev: *mut hl_device) -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
