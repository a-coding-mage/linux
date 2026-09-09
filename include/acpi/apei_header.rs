/* SPDX-License-Identifier: GPL-2.0 */
/*
 * apei.h - ACPI Platform Error Interface
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than redefined.

pub const APEI_ERST_INVALID_RECORD_ID: u64 = 0xffff_ffff_ffff_ffff;

// _IOW('E', 1, u64) and _IOR('E', 2, u32), respectively.  These ioctl values
// are architecture/build dependent in the original header.
pub const APEI_ERST_CLEAR_RECORD: u64 = crate::_IOW(b'E' as u32, 1, core::mem::size_of::<u64>());
pub const APEI_ERST_GET_RECORD_COUNT: u64 = crate::_IOR(b'E' as u32, 2, core::mem::size_of::<u32>());

#[cfg(feature = "kernel")]
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum hest_status {
    HEST_ENABLED,
    HEST_DISABLED,
    HEST_NOT_FOUND,
}

#[cfg(feature = "kernel")]
extern "C" {
    pub static mut hest_disable: core::ffi::c_int;
    pub static mut erst_disable: core::ffi::c_int;

    #[cfg(feature = "CONFIG_ACPI_APEI_GHES")]
    pub static mut ghes_disable: bool;

    #[cfg(feature = "CONFIG_ACPI_APEI_GHES")]
    pub fn acpi_ghes_init();

    #[cfg(feature = "CONFIG_ACPI_APEI")]
    pub fn acpi_hest_init();

    pub fn erst_write(record: *const crate::cper_record_header) -> core::ffi::c_int;
    pub fn erst_get_record_count() -> isize;
    pub fn erst_get_record_id_begin(pos: *mut core::ffi::c_int) -> core::ffi::c_int;
    pub fn erst_get_record_id_next(
        pos: *mut core::ffi::c_int,
        record_id: *mut u64,
    ) -> core::ffi::c_int;
    pub fn erst_get_record_id_end();
    pub fn erst_read(
        record_id: u64,
        record: *mut crate::cper_record_header,
        buflen: usize,
    ) -> isize;
    pub fn erst_read_record(
        record_id: u64,
        record: *mut crate::cper_record_header,
        buflen: usize,
        recordlen: usize,
        creatorid: *const crate::guid_t,
    ) -> isize;
    pub fn erst_clear(record_id: u64) -> core::ffi::c_int;

    pub fn arch_apei_enable_cmcff(
        hest_hdr: *mut crate::acpi_hest_header,
        data: *mut core::ffi::c_void,
    ) -> core::ffi::c_int;
    pub fn arch_apei_report_mem_error(
        sev: core::ffi::c_int,
        mem_err: *mut crate::cper_sec_mem_err,
    );
}

#[cfg(all(feature = "kernel", not(feature = "CONFIG_ACPI_APEI_GHES")))]
pub const ghes_disable: bool = true;

#[cfg(all(feature = "kernel", not(feature = "CONFIG_ACPI_APEI_GHES")))]
#[inline]
pub fn acpi_ghes_init() {}

#[cfg(all(feature = "kernel", not(feature = "CONFIG_ACPI_APEI")))]
#[inline]
pub fn acpi_hest_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
