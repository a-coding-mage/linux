/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies supplied by the corresponding kernel headers. */

/**
 * struct pstore_device_info - back-end pstore/blk driver structure.
 *
 * @flags: Refer to macro starting with PSTORE_FLAGS defined in
 *         linux/pstore.h. It means what front-ends this device support.
 *         Zero means all backends for compatible.
 * @zone: The struct pstore_zone_info details.
 */
#[repr(C)]
pub struct pstore_device_info {
    pub flags: u32,
    pub zone: pstore_zone_info,
}

unsafe extern "C" {
    pub fn register_pstore_device(dev: *mut pstore_device_info) -> i32;
    pub fn unregister_pstore_device(dev: *mut pstore_device_info);
}

/**
 * struct pstore_blk_config - the pstore_blk backend configuration
 *
 * @device:      Name of the desired block device
 * @max_reason:  Maximum kmsg dump reason to store to block device
 * @kmsg_size:   Total size of for kmsg dumps
 * @pmsg_size:   Total size of the pmsg storage area
 * @console_size: Total size of the console storage area
 * @ftrace_size: Total size for ftrace logging data (for all CPUs)
 */
#[repr(C)]
pub struct pstore_blk_config {
    pub device: [i8; 80],
    pub max_reason: kmsg_dump_reason,
    pub kmsg_size: usize,
    pub pmsg_size: usize,
    pub console_size: usize,
    pub ftrace_size: usize,
}

/**
 * pstore_blk_get_config - get a copy of the pstore_blk backend configuration
 *
 * @info: The sturct pstore_blk_config to be filled in
 *
 * Failure returns negative error code, and success returns 0.
 */
unsafe extern "C" {
    pub fn pstore_blk_get_config(info: *mut pstore_blk_config) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
