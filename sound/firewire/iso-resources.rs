// SPDX-License-Identifier: GPL-2.0-only
/*
 * isochronous resources helper functions
 *
 * Copyright (c) Clemens Ladisch <clemens@ladisch.de>
 */

// Translated from C implementation source. External types, constants, macros,
// and helper functions are supplied by the surrounding kernel/firewire code.

extern "C" {
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn mutex_init(mutex: *mut mutex);
    fn mutex_destroy(mutex: *mut mutex);
    fn fw_iso_resource_manage(
        card: *mut fw_card,
        generation: c_int,
        channels_mask: u64,
        channel: *mut c_int,
        bandwidth: *mut c_int,
        allocate: bool,
    );
    fn get_jiffies_64() -> s64;
    fn schedule_timeout_interruptible(timeout: s64) -> s64;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn WARN_ON(condition: bool) -> bool;
}

#[allow(non_camel_case_types)]
type c_int = i32;
#[allow(non_camel_case_types)]
type c_uint = u32;
#[allow(non_camel_case_types)]
type c_char = i8;
#[allow(non_camel_case_types)]
type s64 = i64;

const SCODE_400: c_int = 2;
const HZ: s64 = 100;
const ERESTARTSYS: c_int = 512;
const EBADFD: c_int = 77;
const EAGAIN: c_int = 11;
const EBUSY: c_int = 16;

#[repr(C)]
pub struct fw_iso_resources {
    pub channels_mask: u64,
    pub unit: *mut fw_unit,
    pub mutex: mutex,
    pub allocated: bool,
    pub bandwidth: c_int,
    pub bandwidth_overhead: c_int,
    pub generation: c_int,
    pub channel: c_int,
}

#[repr(C)]
pub struct fw_unit {
    pub device: device,
}

#[repr(C)]
pub struct fw_device {
    pub card: *mut fw_card,
}

#[repr(C)]
pub struct fw_card {
    pub lock: spinlock_t,
    pub gap_count: c_int,
    pub reset_jiffies: s64,
    pub generation: c_int,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[inline]
fn ALIGN(value: c_uint, align: c_uint) -> c_uint {
    (value.wrapping_add(align).wrapping_sub(1)) & !(align.wrapping_sub(1))
}

#[inline]
fn DIV_ROUND_UP(n: c_uint, d: c_uint) -> c_uint {
    n.wrapping_add(d).wrapping_sub(1) / d
}

/**
 * fw_iso_resources_init - initializes a &struct fw_iso_resources
 * @r: the resource manager to initialize
 * @unit: the device unit for which the resources will be needed
 *
 * If the device does not support all channel numbers, change @r->channels_mask
 * after calling this function.
 */
#[no_mangle]
pub unsafe extern "C" fn fw_iso_resources_init(
    r: *mut fw_iso_resources,
    unit: *mut fw_unit,
) -> c_int {
    (*r).channels_mask = !0u64;
    (*r).unit = unit;
    mutex_init(&mut (*r).mutex);
    (*r).allocated = false;

    0
}
// EXPORT_SYMBOL(fw_iso_resources_init);

/**
 * fw_iso_resources_destroy - destroy a resource manager
 * @r: the resource manager that is no longer needed
 */
#[no_mangle]
pub unsafe extern "C" fn fw_iso_resources_destroy(r: *mut fw_iso_resources) {
    WARN_ON((*r).allocated);
    mutex_destroy(&mut (*r).mutex);
}
// EXPORT_SYMBOL(fw_iso_resources_destroy);

unsafe fn packet_bandwidth(max_payload_bytes: c_uint, speed: c_int) -> c_uint {
    let bytes: c_uint;
    let s400_bytes: c_uint;

    /* iso packets have three header quadlets and quadlet-aligned payload */
    bytes = 3u32.wrapping_mul(4).wrapping_add(ALIGN(max_payload_bytes, 4));

    /* convert to bandwidth units (quadlets at S1600 = bytes at S400) */
    if speed <= SCODE_400 {
        s400_bytes = bytes.wrapping_mul(1u32 << (SCODE_400 - speed));
    } else {
        s400_bytes = DIV_ROUND_UP(bytes, 1u32 << (speed - SCODE_400));
    }

    s400_bytes
}

unsafe fn current_bandwidth_overhead(card: *mut fw_card) -> c_int {
    /*
     * Under the usual pessimistic assumption (cable length 4.5 m), the
     * isochronous overhead for N cables is 1.797 µs + N * 0.494 µs, or
     * 88.3 + N * 24.3 in bandwidth units.
     *
     * The calculation below tries to deduce N from the current gap count.
     * If the gap count has been optimized by measuring the actual packet
     * transmission time, this derived overhead should be near the actual
     * overhead as well.
     */
    if (*card).gap_count < 63 {
        (*card).gap_count * 97 / 10 + 89
    } else {
        512
    }
}

unsafe fn wait_isoch_resource_delay_after_bus_reset(card: *mut fw_card) -> c_int {
    loop {
        let delay: s64 = ((*card).reset_jiffies + HZ) - get_jiffies_64();
        if delay <= 0 {
            return 0;
        }
        if schedule_timeout_interruptible(delay) > 0 {
            return -ERESTARTSYS;
        }
    }
}

/**
 * fw_iso_resources_allocate - allocate isochronous channel and bandwidth
 * @r: the resource manager
 * @max_payload_bytes: the amount of data (including CIP headers) per packet
 * @speed: the speed (e.g., SCODE_400) at which the packets will be sent
 *
 * This function allocates one isochronous channel and enough bandwidth for the
 * specified packet size.
 *
 * Returns the channel number that the caller must use for streaming, or
 * a negative error code.  Due to potentionally long delays, this function is
 * interruptible and can return -ERESTARTSYS.  On success, the caller is
 * responsible for calling fw_iso_resources_update() on bus resets, and
 * fw_iso_resources_free() when the resources are not longer needed.
 */
#[no_mangle]
pub unsafe extern "C" fn fw_iso_resources_allocate(
    r: *mut fw_iso_resources,
    max_payload_bytes: c_uint,
    speed: c_int,
) -> c_int {
    let card: *mut fw_card = (*fw_parent_device((*r).unit)).card;
    let mut bandwidth: c_int;
    let mut channel: c_int = 0;
    let mut err: c_int;

    if WARN_ON((*r).allocated) {
        return -EBADFD;
    }

    (*r).bandwidth = packet_bandwidth(max_payload_bytes, speed) as c_int;

    'retry_after_bus_reset: loop {
        // scoped_guard(spinlock_irq, &card->lock)
        {
            (*r).generation = (*card).generation;
            (*r).bandwidth_overhead = current_bandwidth_overhead(card);
        }

        err = wait_isoch_resource_delay_after_bus_reset(card);
        if err < 0 {
            return err;
        }

        // scoped_guard(mutex, &r->mutex)
        {
            bandwidth = (*r).bandwidth + (*r).bandwidth_overhead;
            fw_iso_resource_manage(
                card,
                (*r).generation,
                (*r).channels_mask,
                &mut channel,
                &mut bandwidth,
                true,
            );
            if channel == -EAGAIN {
                continue 'retry_after_bus_reset;
            }
            if channel >= 0 {
                (*r).channel = channel;
                (*r).allocated = true;
            } else if channel == -EBUSY {
                dev_err(
                    &mut (*(*r).unit).device,
                    b"isochronous resources exhausted\n\0".as_ptr() as *const c_char,
                );
            } else {
                dev_err(
                    &mut (*(*r).unit).device,
                    b"isochronous resource allocation failed\n\0".as_ptr() as *const c_char,
                );
            }
        }

        return channel;
    }
}
// EXPORT_SYMBOL(fw_iso_resources_allocate);

/**
 * fw_iso_resources_update - update resource allocations after a bus reset
 * @r: the resource manager
 *
 * This function must be called from the driver's .update handler to reallocate
 * any resources that were allocated before the bus reset.  It is safe to call
 * this function if no resources are currently allocated.
 *
 * Returns a negative error code on failure.  If this happens, the caller must
 * stop streaming.
 */
#[no_mangle]
pub unsafe extern "C" fn fw_iso_resources_update(r: *mut fw_iso_resources) -> c_int {
    let card: *mut fw_card = (*fw_parent_device((*r).unit)).card;
    let mut bandwidth: c_int;
    let mut channel: c_int = 0;

    // guard(mutex)(&r->mutex);

    if !(*r).allocated {
        return 0;
    }

    // scoped_guard(spinlock_irq, &card->lock)
    {
        (*r).generation = (*card).generation;
        (*r).bandwidth_overhead = current_bandwidth_overhead(card);
    }

    bandwidth = (*r).bandwidth + (*r).bandwidth_overhead;

    fw_iso_resource_manage(
        card,
        (*r).generation,
        1u64 << (*r).channel,
        &mut channel,
        &mut bandwidth,
        true,
    );
    /*
     * When another bus reset happens, pretend that the allocation
     * succeeded; we will try again for the new generation later.
     */
    if channel < 0 && channel != -EAGAIN {
        (*r).allocated = false;
        if channel == -EBUSY {
            dev_err(
                &mut (*(*r).unit).device,
                b"isochronous resources exhausted\n\0".as_ptr() as *const c_char,
            );
        } else {
            dev_err(
                &mut (*(*r).unit).device,
                b"isochronous resource allocation failed\n\0".as_ptr() as *const c_char,
            );
        }
    }

    channel
}
// EXPORT_SYMBOL(fw_iso_resources_update);

/**
 * fw_iso_resources_free - frees allocated resources
 * @r: the resource manager
 *
 * This function deallocates the channel and bandwidth, if allocated.
 */
#[no_mangle]
pub unsafe extern "C" fn fw_iso_resources_free(r: *mut fw_iso_resources) {
    let card: *mut fw_card;
    let mut bandwidth: c_int;
    let mut channel: c_int = 0;

    /* Not initialized. */
    if (*r).unit.is_null() {
        return;
    }
    card = (*fw_parent_device((*r).unit)).card;

    // guard(mutex)(&r->mutex);

    if (*r).allocated {
        bandwidth = (*r).bandwidth + (*r).bandwidth_overhead;
        fw_iso_resource_manage(
            card,
            (*r).generation,
            1u64 << (*r).channel,
            &mut channel,
            &mut bandwidth,
            false,
        );
        if channel < 0 {
            dev_err(
                &mut (*(*r).unit).device,
                b"isochronous resource deallocation failed\n\0".as_ptr() as *const c_char,
            );
        }

        (*r).allocated = false;
    }
}
// EXPORT_SYMBOL(fw_iso_resources_free);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
