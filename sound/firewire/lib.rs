// SPDX-License-Identifier: GPL-2.0-only
/*
 * miscellaneous helper functions
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// C dependencies:
// #include <linux/delay.h>
// #include <linux/device.h>
// #include <linux/firewire.h>
// #include <linux/module.h>
// #include <linux/slab.h>
// #include "lib.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const ERROR_RETRY_DELAY_MS: c_uint = 20;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_device {
    pub card: *mut fw_card,
    pub node_id: c_int,
    pub generation: c_int,
    pub max_speed: c_int,
}

#[repr(C)]
pub struct fw_unit {
    pub device: device,
}

unsafe extern "C" {
    static FW_GENERATION_MASK: c_uint;
    static FW_FIXED_GENERATION: c_uint;
    static FW_QUIET: c_uint;

    static RCODE_COMPLETE: c_int;
    static RCODE_GENERATION: c_int;

    static EAGAIN: c_int;
    static EIO: c_int;

    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn smp_rmb();
    fn fw_run_transaction(
        card: *mut fw_card,
        tcode: c_int,
        node_id: c_int,
        generation: c_int,
        speed: c_int,
        offset: u64,
        buffer: *mut c_void,
        length: usize,
    ) -> c_int;
    fn rcode_is_permanent_error(rcode: c_int) -> bool;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn fw_rcode_string(rcode: c_int) -> *const c_char;
    fn msleep(msecs: c_uint);
}

/**
 * snd_fw_transaction - send a request and wait for its completion
 * @unit: the driver's unit on the target device
 * @tcode: the transaction code
 * @offset: the address in the target's address space
 * @buffer: input/output data
 * @length: length of @buffer
 * @flags: use %FW_FIXED_GENERATION and add the generation value to attempt the
 *         request only in that generation; use %FW_QUIET to suppress error
 *         messages
 *
 * Submits an asynchronous request to the target device, and waits for the
 * response.  The node ID and the current generation are derived from @unit.
 * On a bus reset or an error, the transaction is retried a few times.
 * Returns zero on success, or a negative error code.
 */
#[no_mangle]
pub unsafe extern "C" fn snd_fw_transaction(
    unit: *mut fw_unit,
    tcode: c_int,
    offset: u64,
    buffer: *mut c_void,
    length: usize,
    flags: c_uint,
) -> c_int {
    unsafe {
        let device = fw_parent_device(unit);
        let mut generation: c_int;
        let mut rcode: c_int;
        let mut tries: c_int = 0;

        generation = (flags & FW_GENERATION_MASK) as c_int;
        loop {
            if flags & FW_FIXED_GENERATION == 0 {
                generation = (*device).generation;
                smp_rmb(); /* node_id vs. generation */
            }
            rcode = fw_run_transaction(
                (*device).card,
                tcode,
                (*device).node_id,
                generation,
                (*device).max_speed,
                offset,
                buffer,
                length,
            );

            if rcode == RCODE_COMPLETE {
                return 0;
            }

            if rcode == RCODE_GENERATION && flags & FW_FIXED_GENERATION != 0 {
                return -EAGAIN;
            }

            tries += 1;
            if rcode_is_permanent_error(rcode) || tries >= 3 {
                if flags & FW_QUIET == 0 {
                    dev_err(
                        ptr::addr_of_mut!((*unit).device),
                        b"transaction failed: %s\n\0".as_ptr() as *const c_char,
                        fw_rcode_string(rcode),
                    );
                }
                return -EIO;
            }

            msleep(ERROR_RETRY_DELAY_MS);
        }
    }
}

// EXPORT_SYMBOL(snd_fw_transaction);

// MODULE_DESCRIPTION("FireWire audio helper functions");
// MODULE_AUTHOR("Clemens Ladisch <clemens@ladisch.de>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
