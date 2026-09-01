/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies:
// #include <linux/mutex.h>
// #include <linux/types.h>

pub enum fw_unit {}

/**
 * struct fw_iso_resources - manages channel/bandwidth allocation
 * @channels_mask: if the device does not support all channel numbers, set this
 *                 bit mask to something else than the default (all ones)
 *
 * This structure manages (de)allocation of isochronous resources (channel and
 * bandwidth) for one isochronous stream.
 */
#[repr(C)]
pub struct fw_iso_resources {
    pub channels_mask: u64,
    /* private: */
    pub unit: *mut fw_unit,
    pub mutex: mutex,
    pub channel: u32,
    pub bandwidth: u32, /* in bandwidth units, without overhead */
    pub bandwidth_overhead: u32,
    pub generation: i32, /* in which allocation is valid */
    pub allocated: bool,
}

extern "C" {
    pub fn fw_iso_resources_init(r: *mut fw_iso_resources, unit: *mut fw_unit) -> i32;
    pub fn fw_iso_resources_destroy(r: *mut fw_iso_resources);

    pub fn fw_iso_resources_allocate(
        r: *mut fw_iso_resources,
        max_payload_bytes: u32,
        speed: i32,
    ) -> i32;
    pub fn fw_iso_resources_update(r: *mut fw_iso_resources) -> i32;
    pub fn fw_iso_resources_free(r: *mut fw_iso_resources);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
