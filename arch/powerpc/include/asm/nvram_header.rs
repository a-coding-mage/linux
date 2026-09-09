/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * NVRAM definitions and access functions.
 *
 * C header dependencies are supplied by other translated units.
 */

/// Set oops header version to distinguish between old and new format header.
/// lnx,oops-log partition max size is 4000, header version > 4000 will
/// help in identifying new header.
pub const OOPS_HDR_VERSION: i32 = 5000;

#[repr(C)]
pub struct err_log_info {
    pub error_type: __be32,
    pub seq_num: __be32,
}

#[repr(C)]
pub struct nvram_os_partition {
    pub name: *const ::core::ffi::c_char,
    pub req_size: ::core::ffi::c_int, // desired size, in bytes
    pub min_size: ::core::ffi::c_int, // minimum acceptable size (0 means req_size)
    pub size: ::core::ffi::c_long, // size of data portion (excluding err_log_info)
    pub index: ::core::ffi::c_long, // offset of data portion of partition
    pub os_partition: bool, // partition initialized by OS, not FW
}

#[repr(C, packed)]
pub struct oops_log_info {
    pub version: __be16,
    pub report_length: __be16,
    pub timestamp: __be64,
}

pub static mut oops_log_partition: nvram_os_partition = unsafe { ::core::mem::zeroed() };

#[cfg(CONFIG_PPC_PSERIES)]
extern "C" {
    pub static mut rtas_log_partition: nvram_os_partition;

    pub fn nvram_write_error_log(
        buff: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_int,
        err_type: ::core::ffi::c_uint,
        err_seq: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn nvram_read_error_log(
        buff: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_int,
        err_type: *mut ::core::ffi::c_uint,
        err_seq: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn nvram_clear_error_log() -> ::core::ffi::c_int;
    pub fn pSeries_nvram_init() -> ::core::ffi::c_int;
}

#[cfg(CONFIG_MMIO_NVRAM)]
extern "C" {
    pub fn mmio_nvram_init() -> ::core::ffi::c_int;
}

#[cfg(not(CONFIG_MMIO_NVRAM))]
#[inline]
pub unsafe fn mmio_nvram_init() -> ::core::ffi::c_int {
    -ENODEV
}

extern "C" {
    pub fn nvram_scan_partitions() -> ::core::ffi::c_int;
    pub fn nvram_create_partition(
        name: *const ::core::ffi::c_char,
        sig: ::core::ffi::c_int,
        req_size: ::core::ffi::c_int,
        min_size: ::core::ffi::c_int,
    ) -> loff_t;
    pub fn nvram_remove_partition(
        name: *const ::core::ffi::c_char,
        sig: ::core::ffi::c_int,
        exceptions: *const *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    pub fn nvram_get_partition_size(data_index: loff_t) -> ::core::ffi::c_int;
    pub fn nvram_find_partition(
        name: *const ::core::ffi::c_char,
        sig: ::core::ffi::c_int,
        out_size: *mut ::core::ffi::c_int,
    ) -> loff_t;

    /* Return partition offset in nvram */
    pub fn pmac_get_partition(partition: ::core::ffi::c_int) -> ::core::ffi::c_int;

    /* Direct access to XPRAM on PowerMacs */
    pub fn pmac_xpram_read(xpaddr: ::core::ffi::c_int) -> u8;
    pub fn pmac_xpram_write(xpaddr: ::core::ffi::c_int, data: u8);

    /* Initialize NVRAM OS partition */
    pub fn nvram_init_os_partition(part: *mut nvram_os_partition) -> ::core::ffi::c_int;

    /* Initialize NVRAM oops partition */
    pub fn nvram_init_oops_partition(rtas_partition_exists: ::core::ffi::c_int);

    /* Read a NVRAM partition */
    pub fn nvram_read_partition(
        part: *mut nvram_os_partition,
        buff: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_int,
        err_type: *mut ::core::ffi::c_uint,
        error_log_cnt: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    /* Write to NVRAM OS partition */
    pub fn nvram_write_os_partition(
        part: *mut nvram_os_partition,
        buff: *mut ::core::ffi::c_char,
        length: ::core::ffi::c_int,
        err_type: ::core::ffi::c_uint,
        error_log_cnt: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
