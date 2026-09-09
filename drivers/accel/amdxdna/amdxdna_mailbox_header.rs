/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2022-2026, Advanced Micro Devices, Inc.
 */

// C header guard: _AIE_MAILBOX_H_

use core::ffi::c_void;

pub enum mailbox {}
pub enum mailbox_channel {}
pub enum drm_device {}

/*
 * xdna_mailbox_msg - message struct
 *
 * @opcode:\topcode for firmware
 * @handle:\t handle used for the notify callback
 * @notify_cb:  callback function to notify the sender when there is response
 * @send_data:\tpointing to sending data
 * @send_size:\tsize of the sending data
 *
 * The mailbox will split the sending data in to multiple firmware message if
 * the size of the data is too big. This is transparent to the sender. The
 * sender will receive one notification.
 */
#[repr(C)]
pub struct xdna_mailbox_msg {
    pub opcode: u32,
    pub handle: *mut c_void,
    pub notify_cb:
        Option<unsafe extern "C" fn(handle: *mut c_void, data: *mut c_void, size: usize) -> i32>,
    pub send_data: *mut u8,
    pub send_size: usize,
}

/*
 * xdna_mailbox_res - mailbox hardware resource
 *
 * @ringbuf_base: ring buffer base address
 * @ringbuf_size: ring buffer size
 * @mbox_base: mailbox base address
 * @mbox_size: mailbox size
 */
#[repr(C)]
pub struct xdna_mailbox_res {
    pub ringbuf_base: *mut c_void,
    pub ringbuf_size: usize,
    pub mbox_base: *mut c_void,
    pub mbox_size: usize,
    pub name: *const core::ffi::c_char,
}

/*
 * xdna_mailbox_chann_res - resources
 *
 * @rb_start_addr: ring buffer start address
 * @rb_size: ring buffer size
 * @mb_head_ptr_reg: mailbox head pointer register
 * @mb_tail_ptr_reg: mailbox tail pointer register
 */
#[repr(C)]
pub struct xdna_mailbox_chann_res {
    pub rb_start_addr: u32,
    pub rb_size: u32,
    pub mb_head_ptr_reg: u32,
    pub mb_tail_ptr_reg: u32,
}

/*
 * xdna_mailbox_create() -- create mailbox subsystem and initialize
 *
 * @ddev: device pointer
 * @res: SRAM and mailbox resources
 *
 * Return: If success, return a handle of mailbox subsystem.
 * Otherwise, return NULL pointer.
 */
unsafe extern "C" {
    pub fn xdnam_mailbox_create(
        ddev: *mut drm_device,
        res: *const xdna_mailbox_res,
    ) -> *mut mailbox;

    /*
     * xdna_mailbox_alloc_channel() -- alloc a mailbox channel
     *
     * @mb: mailbox handle
     */
    pub fn xdna_mailbox_alloc_channel(mb: *mut mailbox) -> *mut mailbox_channel;

    /*
     * xdna_mailbox_start_channel() -- start a mailbox channel instance
     *
     * @mb_chann: the handle return from xdna_mailbox_alloc_channel()
     * @x2i: host to firmware mailbox resources
     * @i2x: firmware to host mailbox resources
     * @xdna_mailbox_intr_reg: register addr of MSI-X interrupt
     * @mb_irq: Linux IRQ number associated with mailbox MSI-X interrupt vector index
     *
     * Return: If success, return a handle of mailbox channel. Otherwise, return NULL.
     */
    pub fn xdna_mailbox_start_channel(
        mb_chann: *mut mailbox_channel,
        x2i: *const xdna_mailbox_chann_res,
        i2x: *const xdna_mailbox_chann_res,
        xdna_mailbox_intr_reg: u32,
        mb_irq: i32,
    ) -> i32;

    /*
     * xdna_mailbox_free_channel() -- free mailbox channel
     *
     * @mailbox_chann: the handle return from xdna_mailbox_create_channel()
     */
    pub fn xdna_mailbox_free_channel(mailbox_chann: *mut mailbox_channel);

    /*
     * xdna_mailbox_stop_channel() -- stop mailbox channel
     *
     * @mailbox_chann: the handle return from xdna_mailbox_create_channel()
     */
    pub fn xdna_mailbox_stop_channel(mailbox_chann: *mut mailbox_channel);

    /*
     * xdna_mailbox_send_msg() -- Send a message
     *
     * @mailbox_chann: Mailbox channel handle
     * @msg: message struct for message information
     * @tx_timeout: the timeout value for sending the message in ms.
     *
     * Return: If success return 0, otherwise, return error code
     */
    pub fn xdna_mailbox_send_msg(
        mailbox_chann: *mut mailbox_channel,
        msg: *const xdna_mailbox_msg,
        tx_timeout: u64,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
