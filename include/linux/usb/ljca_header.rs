/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2023, Intel Corporation. All rights reserved.
 */

// Dependencies supplied by the surrounding Linux/Rust environment:
// linux/auxiliary_bus.h, linux/list.h, linux/spinlock.h, linux/types.h

pub const LJCA_MAX_GPIO_NUM: usize = 64;

// Equivalent of auxiliary_dev_to_ljca_client(auxiliary_dev), using the
// surrounding environment's container_of implementation.
#[macro_export]
macro_rules! auxiliary_dev_to_ljca_client {
    ($auxiliary_dev:expr) => {
        container_of!($auxiliary_dev, ljca_client, auxdev)
    };
}

pub struct ljca_adapter;

/**
 * typedef ljca_event_cb_t - event callback function signature
 *
 * @context: the execution context of who registered this callback
 * @cmd: the command from device for this event
 * @evt_data: the event data payload
 * @len: the event data payload length
 *
 * The callback function is called in interrupt context and the data payload is
 * only valid during the call. If the user needs later access of the data, it
 * must copy it.
 */
pub type ljca_event_cb_t = Option<unsafe extern "C" fn(
    context: *mut core::ffi::c_void,
    cmd: u8,
    evt_data: *const core::ffi::c_void,
    len: i32,
)>;

/**
 * struct ljca_client - represent a ljca client device
 *
 * @type: ljca client type
 * @id: ljca client id within same client type
 * @link: ljca client on the same ljca adapter
 * @auxdev: auxiliary device object
 * @adapter: ljca adapter the ljca client sit on
 * @context: the execution context of the event callback
 * @event_cb: ljca client driver register this callback to get
 *\tfirmware asynchronous rx buffer pending notifications
 * @event_cb_lock: spinlock to protect event callback
 */
#[repr(C)]
pub struct ljca_client {
    pub type_: u8,
    pub id: u8,
    pub link: list_head,
    pub auxdev: auxiliary_device,
    pub adapter: *mut ljca_adapter,
    pub context: *mut core::ffi::c_void,
    pub event_cb: ljca_event_cb_t,
    /* lock to protect event_cb */
    pub event_cb_lock: spinlock_t,
}

/**
 * struct ljca_gpio_info - ljca gpio client device info
 *
 * @num: ljca gpio client device pin number
 * @valid_pin_map: ljca gpio client device valid pin mapping
 */
#[repr(C)]
pub struct ljca_gpio_info {
    pub num: core::ffi::c_uint,
    pub valid_pin_map: [core::ffi::c_ulong; 1],
}

/**
 * struct ljca_i2c_info - ljca i2c client device info
 *
 * @id: ljca i2c client device identification number
 * @capacity: ljca i2c client device capacity
 * @intr_pin: ljca i2c client device interrupt pin number if exists
 */
#[repr(C)]
pub struct ljca_i2c_info {
    pub id: u8,
    pub capacity: u8,
    pub intr_pin: u8,
}

/**
 * struct ljca_spi_info - ljca spi client device info
 *
 * @id: ljca spi client device identification number
 * @capacity: ljca spi client device capacity
 */
#[repr(C)]
pub struct ljca_spi_info {
    pub id: u8,
    pub capacity: u8,
}

extern "C" {
    /**
     * ljca_register_event_cb - register a callback function to receive events
     *
     * @client: ljca client device
     * @event_cb: callback function
     * @context: execution context of event callback
     *
     * Return: 0 in case of success, negative value in case of error
     */
    pub fn ljca_register_event_cb(
        client: *mut ljca_client,
        event_cb: ljca_event_cb_t,
        context: *mut core::ffi::c_void,
    ) -> i32;

    /**
     * ljca_unregister_event_cb - unregister the callback function for an event
     *
     * @client: ljca client device
     */
    pub fn ljca_unregister_event_cb(client: *mut ljca_client);

    /**
     * ljca_transfer - issue a LJCA command and wait for a response
     *
     * @client: ljca client device
     * @cmd: the command to be sent to the device
     * @obuf: the buffer to be sent to the device; it can be NULL if the user
     *\tdoesn't need to transmit data with this command
     * @obuf_len: the size of the buffer to be sent to the device; it should
     *\tbe 0 when obuf is NULL
     * @ibuf: any data associated with the response will be copied here; it can be
     *\tNULL if the user doesn't need the response data
     * @ibuf_len: must be initialized to the input buffer size
     *
     * Return: the actual length of response data for success, negative value for errors
     */
    pub fn ljca_transfer(
        client: *mut ljca_client,
        cmd: u8,
        obuf: *const u8,
        obuf_len: u8,
        ibuf: *mut u8,
        ibuf_len: u8,
    ) -> i32;

    /**
     * ljca_transfer_noack - issue a LJCA command without a response
     *
     * @client: ljca client device
     * @cmd: the command to be sent to the device
     * @obuf: the buffer to be sent to the device; it can be NULL if the user
     *\tdoesn't need to transmit data with this command
     * @obuf_len: the size of the buffer to be sent to the device
     *
     * Return: 0 for success, negative value for errors
     */
    pub fn ljca_transfer_noack(
        client: *mut ljca_client,
        cmd: u8,
        obuf: *const u8,
        obuf_len: u8,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
