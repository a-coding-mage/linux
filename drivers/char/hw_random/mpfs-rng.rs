// SPDX-License-Identifier: GPL-2.0
/*
 * Microchip PolarFire SoC (MPFS) hardware random driver
 *
 * Copyright (c) 2020-2022 Microchip Corporation. All rights reserved.
 *
 * Author: Conor Dooley <conor.dooley@microchip.com>
 */

// Linux kernel dependencies supplied by other files.

const CMD_OPCODE: u32 = 0x21;
const CMD_DATA_SIZE: u32 = 0;
const CMD_DATA: *mut core::ffi::c_void = core::ptr::null_mut();
const MBOX_OFFSET: u32 = 0;
const RESP_OFFSET: u32 = 0;
const RNG_RESP_BYTES: usize = 32;

#[repr(C)]
pub struct mpfs_rng {
    pub sys_controller: *mut mpfs_sys_controller,
    pub rng: hwrng,
}

#[repr(C)]
pub struct mpfs_sys_controller {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hwrng {
    pub read: Option<unsafe extern "C" fn(*mut hwrng, *mut core::ffi::c_void, usize, bool) -> i32>,
    pub name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct mpfs_mss_response {
    pub resp_status: u32,
    pub resp_msg: *mut u32,
    pub resp_size: u32,
}

#[repr(C)]
pub struct mpfs_mss_msg {
    pub cmd_opcode: u32,
    pub cmd_data_size: u32,
    pub response: *mut mpfs_mss_response,
    pub cmd_data: *mut core::ffi::c_void,
    pub mbox_offset: u32,
    pub resp_offset: u32,
}

extern "C" {
    fn mpfs_blocking_transaction(
        sys_controller: *mut mpfs_sys_controller,
        msg: *mut mpfs_mss_msg,
    ) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn mpfs_rng_read(
    rng: *mut hwrng,
    buf: *mut core::ffi::c_void,
    max: usize,
    wait: bool,
) -> i32 {
    let rng_priv = (rng as *mut u8).sub(core::mem::offset_of!(mpfs_rng, rng)) as *mut mpfs_rng;
    let mut response_msg = [0u32; RNG_RESP_BYTES / core::mem::size_of::<u32>()];
    let mut count: usize = 0;
    let mut copy_size_bytes: usize;

    let mut response = mpfs_mss_response {
        resp_status: 0,
        resp_msg: response_msg.as_mut_ptr(),
        resp_size: RNG_RESP_BYTES as u32,
    };
    let mut msg = mpfs_mss_msg {
        cmd_opcode: CMD_OPCODE,
        cmd_data_size: CMD_DATA_SIZE,
        response: &mut response,
        cmd_data: CMD_DATA,
        mbox_offset: MBOX_OFFSET,
        resp_offset: RESP_OFFSET,
    };

    while count < max {
        let ret = mpfs_blocking_transaction((*rng_priv).sys_controller, &mut msg);
        if ret != 0 {
            return ret;
        }

        copy_size_bytes = if max - count > RNG_RESP_BYTES {
            RNG_RESP_BYTES
        } else {
            max - count
        };
        core::ptr::copy_nonoverlapping(
            response_msg.as_ptr() as *const u8,
            (buf as *mut u8).add(count),
            copy_size_bytes,
        );

        count += copy_size_bytes;
        if !wait {
            break;
        }
    }

    count as i32
}

// The platform-driver registration and kernel metadata are provided by the
// Linux kernel integration layer.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
