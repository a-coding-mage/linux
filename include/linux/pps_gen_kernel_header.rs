/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * PPS generator API kernel header
 *
 * Copyright (C) 2024 Rodolfo Giometti <giometti@enneenne.com>
 */

/* Dependencies supplied by the corresponding kernel headers. */

/* Global defines */
pub const PPS_GEN_MAX_SOURCES: usize = 16; /* should be enough... */

#[repr(C)]
pub struct pps_gen_device;

/**
 * struct pps_gen_source_info - the specific PPS generator info
 * @use_system_clock: true, if the system clock is used to generate pulses
 * @get_time: query the time stored into the generator clock
 * @enable: enable/disable the PPS pulses generation
 *
 * This is the main generator struct where all needed information must be
 * placed before calling the pps_gen_register_source().
 */
#[repr(C)]
pub struct pps_gen_source_info {
    pub use_system_clock: bool,

    pub get_time: Option<unsafe extern "C" fn(
        pps_gen: *mut pps_gen_device,
        time: *mut timespec64,
    ) -> core::ffi::c_int>,
    pub enable: Option<unsafe extern "C" fn(
        pps_gen: *mut pps_gen_device,
        enable: bool,
    ) -> core::ffi::c_int>,

    /* private: internal use only */
    pub owner: *mut module,
    pub parent: *mut device, /* for device_create */
}

/* The main struct */
#[repr(C)]
pub struct pps_gen_device {
    pub info: *const pps_gen_source_info, /* PSS generator info */
    pub enabled: bool,                    /* PSS generator status */

    pub event: core::ffi::c_uint,
    pub sequence: core::ffi::c_uint,

    pub last_ev: core::ffi::c_uint, /* last PPS event id */
    pub queue: wait_queue_head_t,   /* PPS event queue */

    pub id: core::ffi::c_uint, /* PPS generator unique ID */
    pub cdev: cdev,
    pub dev: *mut device,
    pub async_queue: *mut fasync_struct, /* fasync method */
    pub lock: spinlock_t,
}

/* Global variables */
extern "C" {
    pub static mut pps_gen_groups: *const *const attribute_group;
}

/* Exported functions */
extern "C" {
    pub fn pps_gen_register_source(
        info: *const pps_gen_source_info,
    ) -> *mut pps_gen_device;
    pub fn pps_gen_unregister_source(pps_gen: *mut pps_gen_device);
    pub fn pps_gen_event(
        pps_gen: *mut pps_gen_device,
        event: core::ffi::c_uint,
        data: *mut core::ffi::c_void,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
