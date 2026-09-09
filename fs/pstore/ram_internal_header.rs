/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2010 Marco Stornelli <marco.stornelli@gmail.com>
 * Copyright (C) 2011 Kees Cook <keescook@chromium.org>
 * Copyright (C) 2011 Google, Inc.
 */

/* Dependency: types from <linux/pstore_ram.h> are supplied externally. */

/*
 * Choose whether access to the RAM zone requires locking or not.  If a zone
 * can be written to from different CPUs like with ftrace for example, then
 * PRZ_FLAG_NO_LOCK is used. For all other cases, locking is required.
 */
pub const PRZ_FLAG_NO_LOCK: u32 = 1u32 << 0;

/*
 * If a PRZ should only have a single-boot lifetime, this marks it as
 * getting wiped after its contents get copied out after boot.
 */
pub const PRZ_FLAG_ZAP_OLD: u32 = 1u32 << 1;

/**
 * struct persistent_ram_zone - Details of a persistent RAM zone (PRZ)
 *                              used as a pstore backend
 *
 * @paddr: physical address of the mapped RAM area
 * @size: size of mapping
 * @label: unique name of this PRZ
 * @type: frontend type for this PRZ
 * @flags: holds PRZ_FLAGS_* bits
 *
 * @buffer_lock:
 * locks access to @buffer "size" bytes and "start" offset
 * @buffer:
 * pointer to actual RAM area managed by this PRZ
 * @buffer_size:
 * bytes in @buffer->data (not including any trailing ECC bytes)
 *
 * @par_buffer:
 * pointer into @buffer->data containing ECC bytes for @buffer->data
 * @par_header:
 * pointer into @buffer->data containing ECC bytes for @buffer header
 * (i.e. all fields up to @data)
 * @rs_decoder:
 * RSLIB instance for doing ECC calculations
 * @corrected_bytes:
 * ECC corrected bytes accounting since boot
 * @bad_blocks:
 * ECC uncorrectable bytes accounting since boot
 * @ecc_info:
 * ECC configuration details
 *
 * @old_log:
 * saved copy of @buffer->data prior to most recent wipe
 * @old_log_size:
 * bytes contained in @old_log
 *
 */
#[repr(C)]
pub struct persistent_ram_zone {
    pub paddr: phys_addr_t,
    pub size: size_t,
    pub vaddr: *mut core::ffi::c_void,
    pub label: *mut core::ffi::c_char,
    pub type_: pstore_type_id,
    pub flags: u32,

    pub buffer_lock: raw_spinlock_t,
    pub buffer: *mut persistent_ram_buffer,
    pub buffer_size: size_t,

    pub par_buffer: *mut core::ffi::c_char,
    pub par_header: *mut core::ffi::c_char,
    pub rs_decoder: *mut rs_control,
    pub corrected_bytes: i32,
    pub bad_blocks: i32,
    pub ecc_info: persistent_ram_ecc_info,

    pub old_log: *mut core::ffi::c_char,
    pub old_log_size: size_t,
}

unsafe extern "C" {
    pub fn persistent_ram_new(
        start: phys_addr_t,
        size: size_t,
        sig: u32,
        ecc_info: *mut persistent_ram_ecc_info,
        memtype: core::ffi::c_uint,
        flags: u32,
        label: *mut core::ffi::c_char,
    ) -> *mut persistent_ram_zone;
    pub fn persistent_ram_free(_prz: *mut *mut persistent_ram_zone);
    pub fn persistent_ram_zap(prz: *mut persistent_ram_zone);

    pub fn persistent_ram_write(
        prz: *mut persistent_ram_zone,
        s: *const core::ffi::c_void,
        count: core::ffi::c_uint,
    ) -> core::ffi::c_int;
    pub fn persistent_ram_write_user(
        prz: *mut persistent_ram_zone,
        s: *const core::ffi::c_void,
        count: core::ffi::c_uint,
    ) -> core::ffi::c_int;

    pub fn persistent_ram_save_old(prz: *mut persistent_ram_zone);
    pub fn persistent_ram_old_size(prz: *mut persistent_ram_zone) -> size_t;
    pub fn persistent_ram_old(prz: *mut persistent_ram_zone) -> *mut core::ffi::c_void;
    pub fn persistent_ram_free_old(prz: *mut persistent_ram_zone);
    pub fn persistent_ram_ecc_string(
        prz: *mut persistent_ram_zone,
        str_: *mut core::ffi::c_char,
        len: size_t,
    ) -> ssize_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
