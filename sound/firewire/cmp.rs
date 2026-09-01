// SPDX-License-Identifier: GPL-2.0-only
/*
 * Connection Management Procedures (IEC 61883-1) helper functions
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// C dependencies:
// #include <linux/device.h>
// #include <linux/firewire.h>
// #include <linux/firewire-constants.h>
// #include <linux/module.h>
// #include <linux/sched.h>
// #include "lib.h"
// #include "iso-resources.h"
// #include "cmp.h"

use core::ffi::{c_char, c_int, c_uint, c_void};

type __be32 = u32;
type u32_ = u32;
type u64_ = u64;

/* MPR common fields */
const MPR_SPEED_MASK: u32_ = 0xc0000000;
const MPR_SPEED_SHIFT: u32_ = 30;
const MPR_XSPEED_MASK: u32_ = 0x00000060;
const MPR_XSPEED_SHIFT: u32_ = 5;
const MPR_PLUGS_MASK: u32_ = 0x0000001f;

/* PCR common fields */
const PCR_ONLINE: u32_ = 0x80000000;
const PCR_BCAST_CONN: u32_ = 0x40000000;
const PCR_P2P_CONN_MASK: u32_ = 0x3f000000;
const PCR_P2P_CONN_SHIFT: u32_ = 24;
const PCR_CHANNEL_MASK: u32_ = 0x003f0000;
const PCR_CHANNEL_SHIFT: u32_ = 16;

/* oPCR specific fields */
const OPCR_XSPEED_MASK: u32_ = 0x00C00000;
const OPCR_XSPEED_SHIFT: u32_ = 22;
const OPCR_SPEED_MASK: u32_ = 0x0000C000;
const OPCR_SPEED_SHIFT: u32_ = 14;
const OPCR_OVERHEAD_ID_MASK: u32_ = 0x00003C00;
const OPCR_OVERHEAD_ID_SHIFT: u32_ = 10;

const EAGAIN: c_int = 11;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const ECONNREFUSED: c_int = 111;
const EISCONN: c_int = 106;

extern "C" {
    static CSR_REGISTER_BASE: u64_;
    static CSR_IMPR: u64_;
    static CSR_OMPR: u64_;
    static TCODE_LOCK_COMPARE_SWAP: c_int;
    static TCODE_READ_QUADLET_REQUEST: c_int;
    static FW_FIXED_GENERATION: c_int;
    static SCODE_BETA: c_uint;
    static SCODE_400: c_uint;
    static SCODE_800: c_uint;

    fn CSR_IPCR(index: c_uint) -> u64_;
    fn CSR_OPCR(index: c_uint) -> u64_;

    fn snd_fw_transaction(
        unit: *mut fw_unit,
        tcode: c_int,
        offset: u64_,
        buffer: *mut c_void,
        length: usize,
        flags: c_int,
    ) -> c_int;
    fn fw_iso_resources_init(r: *mut fw_iso_resources, unit: *mut fw_unit) -> c_int;
    fn fw_iso_resources_destroy(r: *mut fw_iso_resources);
    fn fw_iso_resources_allocate(
        r: *mut fw_iso_resources,
        max_payload_bytes: c_uint,
        speed: c_uint,
    ) -> c_int;
    fn fw_iso_resources_free(r: *mut fw_iso_resources);
    fn fw_iso_resources_update(r: *mut fw_iso_resources) -> c_int;
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;

    fn mutex_init(mutex: *mut mutex);
    fn mutex_destroy(mutex: *mut mutex);
    fn mutex_lock(mutex: *mut mutex);
    fn mutex_unlock(mutex: *mut mutex);

    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn WARN_ON(condition: bool) -> bool;
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_unit {
    pub device: device,
}

#[repr(C)]
pub struct fw_device {
    pub max_speed: c_uint,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_iso_resources {
    pub unit: *mut fw_unit,
    pub generation: c_int,
    pub allocated: bool,
    pub channel: c_uint,
    pub bandwidth_overhead: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmp_direction {
    CMP_INPUT,
    CMP_OUTPUT,
}

#[repr(C)]
pub struct cmp_connection {
    pub resources: fw_iso_resources,
    pub direction: cmp_direction,
    pub pcr_index: c_uint,
    pub connected: bool,
    pub mutex: mutex,
    pub last_pcr_value: __be32,
    pub max_speed: c_uint,
    pub speed: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum bus_reset_handling {
    ABORT_ON_BUS_RESET,
    SUCCEED_ON_BUS_RESET,
}

#[inline]
fn cpu_to_be32(value: u32_) -> __be32 {
    value.to_be()
}

#[inline]
fn be32_to_cpu(value: __be32) -> u32_ {
    u32::from_be(value)
}

unsafe fn cmp_error(c: *mut cmp_connection, fmt: *const c_char) {
    let prefix = if (*c).direction == cmp_direction::CMP_INPUT {
        b'i' as c_int
    } else {
        b'o' as c_int
    };

    dev_err(
        &mut (*(*c).resources.unit).device,
        b"%cPCR%u: %s\0".as_ptr() as *const c_char,
        prefix,
        (*c).pcr_index,
        fmt,
    );
}

unsafe fn mpr_address(c: *mut cmp_connection) -> u64_ {
    if (*c).direction == cmp_direction::CMP_INPUT {
        CSR_REGISTER_BASE + CSR_IMPR
    } else {
        CSR_REGISTER_BASE + CSR_OMPR
    }
}

unsafe fn pcr_address(c: *mut cmp_connection) -> u64_ {
    if (*c).direction == cmp_direction::CMP_INPUT {
        CSR_REGISTER_BASE + CSR_IPCR((*c).pcr_index)
    } else {
        CSR_REGISTER_BASE + CSR_OPCR((*c).pcr_index)
    }
}

unsafe fn pcr_modify(
    c: *mut cmp_connection,
    modify: unsafe fn(*mut cmp_connection, __be32) -> __be32,
    check: Option<unsafe fn(*mut cmp_connection, __be32) -> c_int>,
    bus_reset_handling: bus_reset_handling,
) -> c_int {
    let mut old_arg: __be32;
    let mut buffer: [__be32; 2] = [0; 2];
    let mut err: c_int;

    buffer[0] = (*c).last_pcr_value;
    loop {
        old_arg = buffer[0];
        buffer[1] = modify(c, buffer[0]);

        err = snd_fw_transaction(
            (*c).resources.unit,
            TCODE_LOCK_COMPARE_SWAP,
            pcr_address(c),
            buffer.as_mut_ptr() as *mut c_void,
            8,
            FW_FIXED_GENERATION | (*c).resources.generation,
        );

        if err < 0 {
            if err == -EAGAIN && bus_reset_handling == bus_reset_handling::SUCCEED_ON_BUS_RESET {
                err = 0;
            }
            return err;
        }

        if buffer[0] == old_arg {
            /* success? */
            break;
        }

        if let Some(check_fn) = check {
            err = check_fn(c, buffer[0]);
            if err < 0 {
                return err;
            }
        }
    }
    (*c).last_pcr_value = buffer[1];

    0
}

/**
 * cmp_connection_init - initializes a connection manager
 * @c: the connection manager to initialize
 * @unit: a unit of the target device
 * @direction: input or output
 * @pcr_index: the index of the iPCR/oPCR on the target device
 */
#[no_mangle]
pub unsafe extern "C" fn cmp_connection_init(
    c: *mut cmp_connection,
    unit: *mut fw_unit,
    direction: cmp_direction,
    pcr_index: c_uint,
) -> c_int {
    let mut mpr_be: __be32 = 0;
    let mut mpr: u32_;
    let mut err: c_int;

    (*c).direction = direction;
    err = snd_fw_transaction(
        unit,
        TCODE_READ_QUADLET_REQUEST,
        mpr_address(c),
        &mut mpr_be as *mut __be32 as *mut c_void,
        4,
        0,
    );
    if err < 0 {
        return err;
    }
    mpr = be32_to_cpu(mpr_be);

    if pcr_index >= (mpr & MPR_PLUGS_MASK) {
        return -EINVAL;
    }

    err = fw_iso_resources_init(&mut (*c).resources, unit);
    if err < 0 {
        return err;
    }

    (*c).connected = false;
    mutex_init(&mut (*c).mutex);
    (*c).last_pcr_value = cpu_to_be32(0x80000000);
    (*c).pcr_index = pcr_index;
    (*c).max_speed = (mpr & MPR_SPEED_MASK) >> MPR_SPEED_SHIFT;
    if (*c).max_speed == SCODE_BETA {
        (*c).max_speed += (mpr & MPR_XSPEED_MASK) >> MPR_XSPEED_SHIFT;
    }

    0
}
// EXPORT_SYMBOL(cmp_connection_init);

/**
 * cmp_connection_check_used - check connection is already esablished or not
 * @c: the connection manager to be checked
 * @used: the pointer to store the result of checking the connection
 */
#[no_mangle]
pub unsafe extern "C" fn cmp_connection_check_used(
    c: *mut cmp_connection,
    used: *mut bool,
) -> c_int {
    let mut pcr: __be32 = 0;
    let err: c_int;

    err = snd_fw_transaction(
        (*c).resources.unit,
        TCODE_READ_QUADLET_REQUEST,
        pcr_address(c),
        &mut pcr as *mut __be32 as *mut c_void,
        4,
        0,
    );
    if err >= 0 {
        *used = (pcr & cpu_to_be32(PCR_BCAST_CONN | PCR_P2P_CONN_MASK)) != 0;
    }

    err
}
// EXPORT_SYMBOL(cmp_connection_check_used);

/**
 * cmp_connection_destroy - free connection manager resources
 * @c: the connection manager
 */
#[no_mangle]
pub unsafe extern "C" fn cmp_connection_destroy(c: *mut cmp_connection) {
    WARN_ON((*c).connected);
    mutex_destroy(&mut (*c).mutex);
    fw_iso_resources_destroy(&mut (*c).resources);
}
// EXPORT_SYMBOL(cmp_connection_destroy);

#[no_mangle]
pub unsafe extern "C" fn cmp_connection_reserve(
    c: *mut cmp_connection,
    max_payload_bytes: c_uint,
) -> c_int {
    mutex_lock(&mut (*c).mutex);

    let ret = if WARN_ON((*c).resources.allocated) {
        -EBUSY
    } else {
        (*c).speed = core::cmp::min((*c).max_speed, (*fw_parent_device((*c).resources.unit)).max_speed);

        fw_iso_resources_allocate(&mut (*c).resources, max_payload_bytes, (*c).speed)
    };

    mutex_unlock(&mut (*c).mutex);
    ret
}
// EXPORT_SYMBOL(cmp_connection_reserve);

#[no_mangle]
pub unsafe extern "C" fn cmp_connection_release(c: *mut cmp_connection) {
    mutex_lock(&mut (*c).mutex);
    fw_iso_resources_free(&mut (*c).resources);
    mutex_unlock(&mut (*c).mutex);
}
// EXPORT_SYMBOL(cmp_connection_release);

unsafe fn ipcr_set_modify(c: *mut cmp_connection, mut ipcr: __be32) -> __be32 {
    ipcr &= !cpu_to_be32(PCR_BCAST_CONN | PCR_P2P_CONN_MASK | PCR_CHANNEL_MASK);
    ipcr |= cpu_to_be32(1 << PCR_P2P_CONN_SHIFT);
    ipcr |= cpu_to_be32((*c).resources.channel << PCR_CHANNEL_SHIFT);

    ipcr
}

unsafe fn get_overhead_id(c: *mut cmp_connection) -> c_int {
    let mut id: c_int;

    /*
     * apply "oPCR overhead ID encoding"
     * the encoding table can convert up to 512.
     * here the value over 512 is converted as the same way as 512.
     */
    id = 1;
    while id < 16 {
        if (*c).resources.bandwidth_overhead < ((id as c_uint) << 5) {
            break;
        }
        id += 1;
    }
    if id == 16 {
        id = 0;
    }

    id
}

unsafe fn opcr_set_modify(c: *mut cmp_connection, mut opcr: __be32) -> __be32 {
    let spd: c_uint;
    let xspd: c_uint;

    /* generate speed and extended speed field value */
    if (*c).speed > SCODE_400 {
        spd = SCODE_800;
        xspd = (*c).speed - SCODE_800;
    } else {
        spd = (*c).speed;
        xspd = 0;
    }

    opcr &= !cpu_to_be32(
        PCR_BCAST_CONN
            | PCR_P2P_CONN_MASK
            | OPCR_XSPEED_MASK
            | PCR_CHANNEL_MASK
            | OPCR_SPEED_MASK
            | OPCR_OVERHEAD_ID_MASK,
    );
    opcr |= cpu_to_be32(1 << PCR_P2P_CONN_SHIFT);
    opcr |= cpu_to_be32(xspd << OPCR_XSPEED_SHIFT);
    opcr |= cpu_to_be32((*c).resources.channel << PCR_CHANNEL_SHIFT);
    opcr |= cpu_to_be32(spd << OPCR_SPEED_SHIFT);
    opcr |= cpu_to_be32((get_overhead_id(c) as c_uint) << OPCR_OVERHEAD_ID_SHIFT);

    opcr
}

unsafe fn pcr_set_check(c: *mut cmp_connection, pcr: __be32) -> c_int {
    if (pcr & cpu_to_be32(PCR_BCAST_CONN | PCR_P2P_CONN_MASK)) != 0 {
        cmp_error(c, b"plug is already in use\n\0".as_ptr() as *const c_char);
        return -EBUSY;
    }
    if (pcr & cpu_to_be32(PCR_ONLINE)) == 0 {
        cmp_error(c, b"plug is not on-line\n\0".as_ptr() as *const c_char);
        return -ECONNREFUSED;
    }

    0
}

/**
 * cmp_connection_establish - establish a connection to the target
 * @c: the connection manager
 *
 * This function establishes a point-to-point connection from the local
 * computer to the target by allocating isochronous resources (channel and
 * bandwidth) and setting the target's input/output plug control register.
 * When this function succeeds, the caller is responsible for starting
 * transmitting packets.
 */
#[no_mangle]
pub unsafe extern "C" fn cmp_connection_establish(c: *mut cmp_connection) -> c_int {
    let mut err: c_int;

    mutex_lock(&mut (*c).mutex);

    if WARN_ON((*c).connected) {
        mutex_unlock(&mut (*c).mutex);
        return -EISCONN;
    }

    loop {
        if (*c).direction == cmp_direction::CMP_OUTPUT {
            err = pcr_modify(
                c,
                opcr_set_modify,
                Some(pcr_set_check),
                bus_reset_handling::ABORT_ON_BUS_RESET,
            );
        } else {
            err = pcr_modify(
                c,
                ipcr_set_modify,
                Some(pcr_set_check),
                bus_reset_handling::ABORT_ON_BUS_RESET,
            );
        }

        if err == -EAGAIN {
            err = fw_iso_resources_update(&mut (*c).resources);
            if err >= 0 {
                continue;
            }
        }
        break;
    }
    if err >= 0 {
        (*c).connected = true;
    }

    mutex_unlock(&mut (*c).mutex);
    err
}
// EXPORT_SYMBOL(cmp_connection_establish);

unsafe fn pcr_break_modify(_c: *mut cmp_connection, pcr: __be32) -> __be32 {
    pcr & !cpu_to_be32(PCR_BCAST_CONN | PCR_P2P_CONN_MASK)
}

/**
 * cmp_connection_break - break the connection to the target
 * @c: the connection manager
 *
 * This function deactives the connection in the target's input/output plug
 * control register, and frees the isochronous resources of the connection.
 * Before calling this function, the caller should cease transmitting packets.
 */
#[no_mangle]
pub unsafe extern "C" fn cmp_connection_break(c: *mut cmp_connection) {
    let err: c_int;

    mutex_lock(&mut (*c).mutex);

    if !(*c).connected {
        mutex_unlock(&mut (*c).mutex);
        return;
    }

    err = pcr_modify(
        c,
        pcr_break_modify,
        None,
        bus_reset_handling::SUCCEED_ON_BUS_RESET,
    );
    if err < 0 {
        cmp_error(c, b"plug is still connected\n\0".as_ptr() as *const c_char);
    }

    (*c).connected = false;
    mutex_unlock(&mut (*c).mutex);
}
// EXPORT_SYMBOL(cmp_connection_break);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
