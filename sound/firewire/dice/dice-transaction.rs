// SPDX-License-Identifier: GPL-2.0-only
/*
 * dice_transaction.c - a part of driver for Dice based devices
 *
 * Copyright (c) Clemens Ladisch
 * Copyright (c) 2014 Takashi Sakamoto
 */

// Translated from C implementation source. External types, constants, and
// functions are provided by the surrounding driver/kernel bindings.

type u32 = u32;
type u64 = u64;
type __be32 = u32;
type __be64 = u64;
type size_t = usize;

#[repr(C)]
pub struct snd_dice {
    pub tx_offset: u64,
    pub rx_offset: u64,
    pub sync_offset: u64,
    pub rsrv_offset: u64,
    pub global_offset: u64,
    pub unit: *mut fw_unit,
    pub global_enabled: bool,
    pub owner_generation: i32,
    pub notification_handler: fw_address_handler,
    pub lock: spinlock_t,
    pub notification_bits: u32,
    pub clock_accepted: completion,
    pub hwdep_wait: wait_queue_head_t,
    pub clock_caps: u32,
}

#[repr(C)]
pub struct fw_unit {
    pub device: device,
}

#[repr(C)]
pub struct fw_device {
    pub card: *mut fw_card,
    pub generation: i32,
}

#[repr(C)]
pub struct fw_card {
    pub node_id: u32,
}

#[repr(C)]
pub struct fw_request {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_address_handler {
    pub length: usize,
    pub address_callback: Option<
        unsafe extern "C" fn(
            *mut fw_card,
            *mut fw_request,
            i32,
            i32,
            i32,
            i32,
            u64,
            *mut core::ffi::c_void,
            size_t,
            *mut core::ffi::c_void,
        ),
    >,
    pub callback_data: *mut core::ffi::c_void,
    pub offset: u64,
}

#[repr(C)]
pub struct fw_address_region {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct completion {
    _private: [u8; 0],
}

#[repr(C)]
pub struct wait_queue_head_t {
    _private: [u8; 0],
}

#[repr(C)]
pub enum snd_dice_addr_type {
    SND_DICE_ADDR_TYPE_TX,
    SND_DICE_ADDR_TYPE_RX,
    SND_DICE_ADDR_TYPE_SYNC,
    SND_DICE_ADDR_TYPE_RSRV,
    SND_DICE_ADDR_TYPE_GLOBAL,
}

extern "C" {
    static snd_dice_rates: [u32; SND_DICE_RATES_COUNT as usize];
    static mut fw_high_memory_region: fw_address_region;

    fn snd_fw_transaction(
        unit: *mut fw_unit,
        tcode: i32,
        offset: u64,
        buffer: *mut core::ffi::c_void,
        length: u32,
        flags: u32,
    ) -> i32;
    fn fw_parent_device(unit: *mut fw_unit) -> *mut fw_device;
    fn fw_send_response(card: *mut fw_card, request: *mut fw_request, rcode: i32);
    fn fw_core_add_address_handler(
        handler: *mut fw_address_handler,
        region: *mut fw_address_region,
    ) -> i32;
    fn fw_core_remove_address_handler(handler: *mut fw_address_handler);
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kmalloc_array(n: usize, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn msleep(msecs: u32);
    fn smp_rmb();
    fn complete(x: *mut completion);
    fn wake_up(x: *mut wait_queue_head_t);
    fn spin_lock_irqsave(lock: *mut spinlock_t) -> u64;
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: u64);
}

extern "C" {
    fn snd_dice_transaction_read_global(
        dice: *mut snd_dice,
        offset: u32,
        buf: *mut core::ffi::c_void,
        len: u32,
    ) -> i32;
}

const DICE_PRIVATE_SPACE: u64 = 0xffff_e000_0000;
const GLOBAL_CLOCK_SELECT: u32 = 0;
const GLOBAL_ENABLE: u32 = 0;
const GLOBAL_OWNER: u32 = 0;
const GLOBAL_VERSION: u32 = 0;
const CLOCK_SOURCE_MASK: u32 = 0;
const CLOCK_RATE_MASK: u32 = 0;
const CLOCK_RATE_SHIFT: u32 = 0;
const SND_DICE_RATES_COUNT: u32 = 0;
const TCODE_WRITE_QUADLET_REQUEST: i32 = 0;
const TCODE_WRITE_BLOCK_REQUEST: i32 = 0;
const TCODE_READ_QUADLET_REQUEST: i32 = 0;
const TCODE_READ_BLOCK_REQUEST: i32 = 0;
const TCODE_LOCK_COMPARE_SWAP: i32 = 0;
const FW_FIXED_GENERATION: u32 = 0;
const FW_QUIET: u32 = 0;
const RCODE_TYPE_ERROR: i32 = 0;
const RCODE_ADDRESS_ERROR: i32 = 0;
const RCODE_COMPLETE: i32 = 0;
const NOTIFY_CLOCK_ACCEPTED: u32 = 0;
const OWNER_NO_OWNER: u64 = 0;
const OWNER_NODE_SHIFT: u32 = 0;
const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;
const ENOSYS: i32 = 38;
const EBUSY: i32 = 16;
const EAGAIN: i32 = 11;
const EINVAL: i32 = 22;
const ENODEV: i32 = 19;

#[inline]
unsafe fn cpu_to_be32(value: u32) -> __be32 {
    value.to_be()
}

#[inline]
unsafe fn be32_to_cpu(value: __be32) -> u32 {
    u32::from_be(value)
}

#[inline]
unsafe fn be32_to_cpup(p: *const core::ffi::c_void) -> u32 {
    be32_to_cpu(*(p as *const __be32))
}

#[inline]
unsafe fn cpu_to_be64(value: u64) -> __be64 {
    value.to_be()
}

unsafe fn get_subaddr(
    dice: *mut snd_dice,
    type_: snd_dice_addr_type,
    mut offset: u64,
) -> u64 {
    match type_ {
        snd_dice_addr_type::SND_DICE_ADDR_TYPE_TX => {
            offset = offset.wrapping_add((*dice).tx_offset);
        }
        snd_dice_addr_type::SND_DICE_ADDR_TYPE_RX => {
            offset = offset.wrapping_add((*dice).rx_offset);
        }
        snd_dice_addr_type::SND_DICE_ADDR_TYPE_SYNC => {
            offset = offset.wrapping_add((*dice).sync_offset);
        }
        snd_dice_addr_type::SND_DICE_ADDR_TYPE_RSRV => {
            offset = offset.wrapping_add((*dice).rsrv_offset);
        }
        snd_dice_addr_type::SND_DICE_ADDR_TYPE_GLOBAL => {
            offset = offset.wrapping_add((*dice).global_offset);
        }
    }
    offset = offset.wrapping_add(DICE_PRIVATE_SPACE);
    offset
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_transaction_write(
    dice: *mut snd_dice,
    type_: snd_dice_addr_type,
    offset: u32,
    buf: *mut core::ffi::c_void,
    len: u32,
) -> i32 {
    snd_fw_transaction(
        (*dice).unit,
        if len == 4 {
            TCODE_WRITE_QUADLET_REQUEST
        } else {
            TCODE_WRITE_BLOCK_REQUEST
        },
        get_subaddr(dice, type_, offset as u64),
        buf,
        len,
        0,
    )
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_transaction_read(
    dice: *mut snd_dice,
    type_: snd_dice_addr_type,
    offset: u32,
    buf: *mut core::ffi::c_void,
    len: u32,
) -> i32 {
    snd_fw_transaction(
        (*dice).unit,
        if len == 4 {
            TCODE_READ_QUADLET_REQUEST
        } else {
            TCODE_READ_BLOCK_REQUEST
        },
        get_subaddr(dice, type_, offset as u64),
        buf,
        len,
        0,
    )
}

unsafe fn get_clock_info(dice: *mut snd_dice, info: *mut __be32) -> u32 {
    snd_dice_transaction_read_global(
        dice,
        GLOBAL_CLOCK_SELECT,
        info as *mut core::ffi::c_void,
        4,
    ) as u32
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_transaction_get_clock_source(
    dice: *mut snd_dice,
    source: *mut u32,
) -> i32 {
    let mut info: __be32 = 0;
    let err: i32;

    err = get_clock_info(dice, &mut info) as i32;
    if err >= 0 {
        *source = be32_to_cpu(info) & CLOCK_SOURCE_MASK;
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_transaction_get_rate(
    dice: *mut snd_dice,
    rate: *mut u32,
) -> i32 {
    let mut info: __be32 = 0;
    let index: u32;
    let mut err: i32;

    err = get_clock_info(dice, &mut info) as i32;
    if err < 0 {
        return err;
    }

    index = (be32_to_cpu(info) & CLOCK_RATE_MASK) >> CLOCK_RATE_SHIFT;
    if index >= SND_DICE_RATES_COUNT {
        err = -ENOSYS;
        return err;
    }

    *rate = snd_dice_rates[index as usize];
    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_transaction_set_enable(dice: *mut snd_dice) -> i32 {
    let mut value: __be32;
    let mut err: i32 = 0;

    if (*dice).global_enabled {
        return err;
    }

    value = cpu_to_be32(1);
    err = snd_fw_transaction(
        (*dice).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        get_subaddr(
            dice,
            snd_dice_addr_type::SND_DICE_ADDR_TYPE_GLOBAL,
            GLOBAL_ENABLE as u64,
        ),
        &mut value as *mut __be32 as *mut core::ffi::c_void,
        4,
        FW_FIXED_GENERATION | ((*dice).owner_generation as u32),
    );
    if err < 0 {
        return err;
    }

    (*dice).global_enabled = true;
    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_transaction_clear_enable(dice: *mut snd_dice) {
    let mut value: __be32;

    value = 0;
    snd_fw_transaction(
        (*dice).unit,
        TCODE_WRITE_QUADLET_REQUEST,
        get_subaddr(
            dice,
            snd_dice_addr_type::SND_DICE_ADDR_TYPE_GLOBAL,
            GLOBAL_ENABLE as u64,
        ),
        &mut value as *mut __be32 as *mut core::ffi::c_void,
        4,
        FW_QUIET | FW_FIXED_GENERATION | ((*dice).owner_generation as u32),
    );

    (*dice).global_enabled = false;
}

unsafe extern "C" fn dice_notification(
    card: *mut fw_card,
    request: *mut fw_request,
    tcode: i32,
    _destination: i32,
    _source: i32,
    _generation: i32,
    offset: u64,
    data: *mut core::ffi::c_void,
    _length: size_t,
    callback_data: *mut core::ffi::c_void,
) {
    let dice: *mut snd_dice = callback_data as *mut snd_dice;
    let bits: u32;

    if tcode != TCODE_WRITE_QUADLET_REQUEST {
        fw_send_response(card, request, RCODE_TYPE_ERROR);
        return;
    }
    if (offset & 3) != 0 {
        fw_send_response(card, request, RCODE_ADDRESS_ERROR);
        return;
    }

    bits = be32_to_cpup(data);

    {
        let flags = spin_lock_irqsave(&mut (*dice).lock);
        (*dice).notification_bits |= bits;
        spin_unlock_irqrestore(&mut (*dice).lock, flags);
    }

    fw_send_response(card, request, RCODE_COMPLETE);

    if (bits & NOTIFY_CLOCK_ACCEPTED) != 0 {
        complete(&mut (*dice).clock_accepted);
    }
    wake_up(&mut (*dice).hwdep_wait);
}

unsafe fn register_notification_address(dice: *mut snd_dice, retry: bool) -> i32 {
    let device: *mut fw_device = fw_parent_device((*dice).unit);
    let mut buffer: *mut __be64;
    let mut retries: u32;
    let mut err: i32;

    retries = if retry { 3 } else { 0 };

    buffer = kmalloc(2 * 8, GFP_KERNEL) as *mut __be64;
    if buffer.is_null() {
        return -ENOMEM;
    }

    loop {
        *buffer.add(0) = cpu_to_be64(OWNER_NO_OWNER);
        *buffer.add(1) = cpu_to_be64(
            (((*(*device).card).node_id as u64) << OWNER_NODE_SHIFT)
                | (*dice).notification_handler.offset,
        );

        (*dice).owner_generation = (*device).generation;
        smp_rmb(); /* node_id vs. generation */
        err = snd_fw_transaction(
            (*dice).unit,
            TCODE_LOCK_COMPARE_SWAP,
            get_subaddr(
                dice,
                snd_dice_addr_type::SND_DICE_ADDR_TYPE_GLOBAL,
                GLOBAL_OWNER as u64,
            ),
            buffer as *mut core::ffi::c_void,
            2 * 8,
            FW_FIXED_GENERATION | ((*dice).owner_generation as u32),
        );
        if err == 0 {
            /* success */
            if *buffer.add(0) == cpu_to_be64(OWNER_NO_OWNER) {
                break;
            }
            /* The address seems to be already registered. */
            if *buffer.add(0) == *buffer.add(1) {
                break;
            }

            dev_err(
                &mut (*(*dice).unit).device,
                b"device is already in use\n\0".as_ptr() as *const i8,
            );
            err = -EBUSY;
        }
        if err != -EAGAIN || {
            let old = retries;
            retries = retries.wrapping_sub(1);
            old > 0
        } {
            break;
        }

        msleep(20);
    }

    kfree(buffer as *mut core::ffi::c_void);

    if err < 0 {
        (*dice).owner_generation = -1;
    }

    err
}

unsafe fn unregister_notification_address(dice: *mut snd_dice) {
    let device: *mut fw_device = fw_parent_device((*dice).unit);
    let buffer: *mut __be64;

    buffer = kmalloc(2 * 8, GFP_KERNEL) as *mut __be64;
    if buffer.is_null() {
        return;
    }

    *buffer.add(0) = cpu_to_be64(
        (((*(*device).card).node_id as u64) << OWNER_NODE_SHIFT)
            | (*dice).notification_handler.offset,
    );
    *buffer.add(1) = cpu_to_be64(OWNER_NO_OWNER);
    snd_fw_transaction(
        (*dice).unit,
        TCODE_LOCK_COMPARE_SWAP,
        get_subaddr(
            dice,
            snd_dice_addr_type::SND_DICE_ADDR_TYPE_GLOBAL,
            GLOBAL_OWNER as u64,
        ),
        buffer as *mut core::ffi::c_void,
        2 * 8,
        FW_QUIET | FW_FIXED_GENERATION | ((*dice).owner_generation as u32),
    );

    kfree(buffer as *mut core::ffi::c_void);

    (*dice).owner_generation = -1;
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_transaction_destroy(dice: *mut snd_dice) {
    let handler: *mut fw_address_handler = &mut (*dice).notification_handler;

    if (*handler).callback_data.is_null() {
        return;
    }

    unregister_notification_address(dice);

    fw_core_remove_address_handler(handler);
    (*handler).callback_data = core::ptr::null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_transaction_reinit(dice: *mut snd_dice) -> i32 {
    let handler: *mut fw_address_handler = &mut (*dice).notification_handler;

    if (*handler).callback_data.is_null() {
        return -EINVAL;
    }

    register_notification_address(dice, false)
}

unsafe fn get_subaddrs(dice: *mut snd_dice) -> i32 {
    static MIN_VALUES: [i32; 10] = [
        10,
        0x60 / 4,
        10,
        0x18 / 4,
        10,
        0x18 / 4,
        0,
        0,
        0,
        0,
    ];
    let pointers: *mut __be32;
    let mut version: __be32 = 0;
    let mut data: u32;
    let mut i: u32;
    let mut err: i32;

    pointers = kmalloc_array(
        MIN_VALUES.len(),
        core::mem::size_of::<__be32>(),
        GFP_KERNEL,
    ) as *mut __be32;
    if pointers.is_null() {
        return -ENOMEM;
    }

    /*
     * Check that the sub address spaces exist and are located inside the
     * private address space.  The minimum values are chosen so that all
     * minimally required registers are included.
     */
    err = snd_fw_transaction(
        (*dice).unit,
        TCODE_READ_BLOCK_REQUEST,
        DICE_PRIVATE_SPACE,
        pointers as *mut core::ffi::c_void,
        (core::mem::size_of::<__be32>() * MIN_VALUES.len()) as u32,
        0,
    );
    if err < 0 {
        kfree(pointers as *mut core::ffi::c_void);
        return err;
    }

    i = 0;
    while (i as usize) < MIN_VALUES.len() {
        data = be32_to_cpu(*pointers.add(i as usize));
        if data < MIN_VALUES[i as usize] as u32 || data >= 0x40000 {
            err = -ENODEV;
            kfree(pointers as *mut core::ffi::c_void);
            return err;
        }
        i += 1;
    }

    if be32_to_cpu(*pointers.add(1)) > 0x18 {
        /*
         * Check that the implemented DICE driver specification major
         * version number matches.
         */
        err = snd_fw_transaction(
            (*dice).unit,
            TCODE_READ_QUADLET_REQUEST,
            DICE_PRIVATE_SPACE
                + (be32_to_cpu(*pointers.add(0)) as u64) * 4
                + GLOBAL_VERSION as u64,
            &mut version as *mut __be32 as *mut core::ffi::c_void,
            core::mem::size_of_val(&version) as u32,
            0,
        );
        if err < 0 {
            kfree(pointers as *mut core::ffi::c_void);
            return err;
        }

        if (version & cpu_to_be32(0xff000000)) != cpu_to_be32(0x01000000) {
            dev_err(
                &mut (*(*dice).unit).device,
                b"unknown DICE version: 0x%08x\n\0".as_ptr() as *const i8,
                be32_to_cpu(version),
            );
            err = -ENODEV;
            kfree(pointers as *mut core::ffi::c_void);
            return err;
        }

        /* Set up later. */
        (*dice).clock_caps = 1;
    }

    (*dice).global_offset = (be32_to_cpu(*pointers.add(0)) as u64) * 4;
    (*dice).tx_offset = (be32_to_cpu(*pointers.add(2)) as u64) * 4;
    (*dice).rx_offset = (be32_to_cpu(*pointers.add(4)) as u64) * 4;

    /* Old firmware doesn't support these fields. */
    if *pointers.add(7) != 0 {
        (*dice).sync_offset = (be32_to_cpu(*pointers.add(6)) as u64) * 4;
    }
    if *pointers.add(9) != 0 {
        (*dice).rsrv_offset = (be32_to_cpu(*pointers.add(8)) as u64) * 4;
    }

    kfree(pointers as *mut core::ffi::c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn snd_dice_transaction_init(dice: *mut snd_dice) -> i32 {
    let handler: *mut fw_address_handler = &mut (*dice).notification_handler;
    let mut err: i32;

    err = get_subaddrs(dice);
    if err < 0 {
        return err;
    }

    /* Allocation callback in address space over host controller */
    (*handler).length = 4;
    (*handler).address_callback = Some(dice_notification);
    (*handler).callback_data = dice as *mut core::ffi::c_void;
    err = fw_core_add_address_handler(handler, &mut fw_high_memory_region);
    if err < 0 {
        (*handler).callback_data = core::ptr::null_mut();
        return err;
    }

    /* Register the address space */
    err = register_notification_address(dice, true);
    if err < 0 {
        fw_core_remove_address_handler(handler);
        (*handler).callback_data = core::ptr::null_mut();
    }

    err
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
