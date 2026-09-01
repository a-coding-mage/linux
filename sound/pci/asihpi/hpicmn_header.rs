/* SPDX-License-Identifier: GPL-2.0-only */
/*

    AudioScience HPI driver
    Copyright (C) 1997-2014  AudioScience Inc. <support@audioscience.com>


*/

use core::ffi::{c_int, c_short, c_void};

/* a function that takes an adapter obj and returns an int */
pub type adapter_int_func =
    unsafe extern "C" fn(pao: *mut hpi_adapter_obj, message: u32) -> c_int;

pub const HPI_IRQ_NONE: c_int = 0;
pub const HPI_IRQ_MESSAGE: c_int = 1;
pub const HPI_IRQ_MIXER: c_int = 2;

#[repr(C)]
pub struct hpi_adapter_obj {
    pub pci: hpi_pci, /* PCI info - bus#,dev#,address etc */
    pub type_: u16,  /* 0x6644 == ASI6644 etc */
    pub index: u16,

    pub dsp_lock: hpios_spinlock,

    pub dsp_crashed: u16,
    pub has_control_cache: u16,
    pub priv_: *mut c_void,
    pub irq_query_and_clear: Option<adapter_int_func>,
    pub instream_host_buffer_status: *mut hpi_hostbuffer_status,
    pub outstream_host_buffer_status: *mut hpi_hostbuffer_status,
}

#[repr(C)]
pub struct hpi_control_cache {
    /** indicates whether the structures are initialized */
    pub init: u16,
    pub adap_idx: u16,
    pub control_count: u32,
    pub cache_size_in_bytes: u32,
    /** pointer to DSP's control cache. */
    pub p_cache: *mut u8,
    /** pointer to allocated memory of lookup pointers.
     *
     * C source uses a flexible array member:
     * struct hpi_control_cache_info *p_info[] __counted_by(control_count);
     */
    pub p_info: [*mut hpi_control_cache_info; 0],
}

unsafe extern "C" {
    pub fn hpi_find_adapter(adapter_index: u16) -> *mut hpi_adapter_obj;

    pub fn hpi_add_adapter(pao: *mut hpi_adapter_obj) -> u16;

    pub fn hpi_delete_adapter(pao: *mut hpi_adapter_obj);

    pub fn hpi_check_control_cache(
        pC: *mut hpi_control_cache,
        phm: *mut hpi_message,
        phr: *mut hpi_response,
    ) -> c_short;

    pub fn hpi_check_control_cache_single(
        pC: *mut hpi_control_cache_single,
        phm: *mut hpi_message,
        phr: *mut hpi_response,
    ) -> c_short;

    pub fn hpi_alloc_control_cache(
        number_of_controls: u32,
        size_in_bytes: u32,
        pDSP_control_buffer: *mut u8,
    ) -> *mut hpi_control_cache;

    pub fn hpi_free_control_cache(p_cache: *mut hpi_control_cache);

    pub fn hpi_cmn_control_cache_sync_to_msg(
        pC: *mut hpi_control_cache,
        phm: *mut hpi_message,
        phr: *mut hpi_response,
    );

    pub fn hpi_cmn_control_cache_sync_to_msg_single(
        pC: *mut hpi_control_cache_single,
        phm: *mut hpi_message,
        phr: *mut hpi_response,
    );

    pub fn hpi_validate_response(
        phm: *mut hpi_message,
        phr: *mut hpi_response,
    ) -> u16;

    pub static mut HPI_COMMON: hpi_handler_func;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
