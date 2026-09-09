/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Copyright (C) 1999-2002 Vojtech Pavlik
 * Copyright (C) 2004 Dmitry Torokhov
 */

/* C dependencies retained as external Rust types: linux/bitops.h,
 * linux/interrupt.h, linux/mutex.h, linux/types.h, and linux/wait.h.
 */

pub struct serio;

/**
 * enum ps2_disposition - indicates how received byte should be handled
 * @PS2_PROCESS: pass to the main protocol handler, process normally
 * @PS2_IGNORE: skip the byte
 * @PS2_ERROR: do not process the byte, abort command in progress
 */
#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ps2_disposition {
    PS2_PROCESS,
    PS2_IGNORE,
    PS2_ERROR,
}

pub type ps2_pre_receive_handler_t = Option<unsafe extern "C" fn(
    *mut ps2dev,
    u8,
    core::ffi::c_uint,
) -> ps2_disposition>;
pub type ps2_receive_handler_t = Option<unsafe extern "C" fn(*mut ps2dev, u8)>;

/**
 * struct ps2dev - represents a device using PS/2 protocol
 * @serio: a serio port used by the PS/2 device
 * @cmd_mutex: a mutex ensuring that only one command is executing at a time
 * @wait: a waitqueue used to signal completion from the serio interrupt handler
 * @flags: various internal flags indicating stages of PS/2 command execution
 * @cmdbuf: buffer holding command response
 * @cmdcnt: outstanding number of bytes of the command response
 * @nak: a byte transmitted by the device when it refuses command
 * @pre_receive_handler: checks communication errors and returns disposition
 * (&enum ps2_disposition) of the received data byte
 * @receive_handler: main handler of particular PS/2 protocol, such as keyboard
 *   or mouse protocol
 */
#[repr(C)]
pub struct ps2dev {
    pub serio: *mut serio,
    pub cmd_mutex: mutex,
    pub wait: wait_queue_head_t,
    pub flags: core::ffi::c_ulong,
    pub cmdbuf: [u8; 8],
    pub cmdcnt: u8,
    pub nak: u8,

    pub pre_receive_handler: ps2_pre_receive_handler_t,
    pub receive_handler: ps2_receive_handler_t,
}

extern "C" {
    pub fn ps2_init(
        ps2dev: *mut ps2dev,
        serio: *mut serio,
        pre_receive_handler: ps2_pre_receive_handler_t,
        receive_handler: ps2_receive_handler_t,
    );
    pub fn ps2_sendbyte(ps2dev: *mut ps2dev, byte: u8, timeout: core::ffi::c_uint) -> i32;
    pub fn ps2_drain(
        ps2dev: *mut ps2dev,
        maxbytes: usize,
        timeout: core::ffi::c_uint,
    );
    pub fn ps2_begin_command(ps2dev: *mut ps2dev);
    pub fn ps2_end_command(ps2dev: *mut ps2dev);
    pub fn __ps2_command(
        ps2dev: *mut ps2dev,
        param: *mut u8,
        command: core::ffi::c_uint,
    ) -> i32;
    pub fn ps2_command(
        ps2dev: *mut ps2dev,
        param: *mut u8,
        command: core::ffi::c_uint,
    ) -> i32;
    pub fn ps2_sliced_command(ps2dev: *mut ps2dev, command: u8) -> i32;
    pub fn ps2_is_keyboard_id(id: u8) -> bool;

    pub fn ps2_interrupt(
        serio: *mut serio,
        data: u8,
        flags: core::ffi::c_uint,
    ) -> irqreturn_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
