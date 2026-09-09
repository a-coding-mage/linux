/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <uapi/linux/pr.h> are supplied externally.

#[repr(C)]
pub struct pr_keys {
    pub generation: u32,
    pub num_keys: u32,
    pub keys: [u64; 0],
}

#[repr(C)]
pub struct pr_held_reservation {
    pub key: u64,
    pub generation: u32,
    pub type_: pr_type,
}

#[repr(C)]
pub struct pr_ops {
    pub pr_register:
        Option<unsafe extern "C" fn(bdev: *mut block_device, old_key: u64, new_key: u64, flags: u32) -> i32>,
    pub pr_reserve:
        Option<unsafe extern "C" fn(bdev: *mut block_device, key: u64, type_: pr_type, flags: u32) -> i32>,
    pub pr_release:
        Option<unsafe extern "C" fn(bdev: *mut block_device, key: u64, type_: pr_type) -> i32>,
    pub pr_preempt: Option<unsafe extern "C" fn(
        bdev: *mut block_device,
        old_key: u64,
        new_key: u64,
        type_: pr_type,
        abort: bool,
    ) -> i32>,
    pub pr_clear: Option<unsafe extern "C" fn(bdev: *mut block_device, key: u64) -> i32>,
    /*
     * pr_read_keys - Read the registered keys and return them in the
     * pr_keys->keys array. The keys array will have been allocated at the
     * end of the pr_keys struct, and pr_keys->num_keys must be set to the
     * number of keys the array can hold. If there are more than can fit
     * in the array, success will still be returned and pr_keys->num_keys
     * will reflect the total number of keys the device contains, so the
     * caller can retry with a larger array.
     */
    pub pr_read_keys:
        Option<unsafe extern "C" fn(bdev: *mut block_device, keys_info: *mut pr_keys) -> i32>,
    pub pr_read_reservation: Option<unsafe extern "C" fn(
        bdev: *mut block_device,
        rsv: *mut pr_held_reservation,
    ) -> i32>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
