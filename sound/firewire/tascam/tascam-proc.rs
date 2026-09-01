// SPDX-License-Identifier: GPL-2.0-only
/*
 * tascam-proc.h - a part of driver for TASCAM FireWire series
 *
 * Copyright (c) 2015 Takashi Sakamoto
 */

// C dependency intent: #include "./tascam.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem;

type __be32 = u32;

const S_IFDIR: c_uint = 0o040000;

#[repr(C)]
pub struct snd_info_buffer {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fw_unit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub proc_root: *mut snd_info_entry,
}

#[repr(C)]
pub struct snd_tscm {
    pub card: *mut snd_card,
    pub unit: *mut fw_unit,
}

#[repr(C)]
pub struct snd_info_entry {
    pub private_data: *mut c_void,
    pub mode: c_uint,
}

type snd_info_text_op =
    Option<unsafe extern "C" fn(e: *mut snd_info_entry, b: *mut snd_info_buffer)>;

unsafe extern "C" {
    static TSCM_ADDR_BASE: u64;
    static TSCM_OFFSET_FIRMWARE_REGISTER: u64;
    static TSCM_OFFSET_FIRMWARE_FPGA: u64;
    static TSCM_OFFSET_FIRMWARE_ARM: u64;
    static TSCM_OFFSET_FIRMWARE_HW: u64;

    static TCODE_READ_QUADLET_REQUEST: c_int;

    fn snd_fw_transaction(
        unit: *mut fw_unit,
        tcode: c_int,
        offset: u64,
        buffer: *mut c_void,
        length: usize,
        flags: c_uint,
    ) -> c_int;
    fn be32_to_cpu(value: __be32) -> c_uint;
    fn snd_iprintf(buffer: *mut snd_info_buffer, fmt: *const c_char, ...);
    fn snd_info_create_card_entry(
        card: *mut snd_card,
        name: *const c_char,
        root: *mut snd_info_entry,
    ) -> *mut snd_info_entry;
    fn snd_info_set_text_ops(
        entry: *mut snd_info_entry,
        private_data: *mut c_void,
        op: snd_info_text_op,
    );
}

unsafe extern "C" fn proc_read_firmware(
    entry: *mut snd_info_entry,
    buffer: *mut snd_info_buffer,
) {
    let tscm = unsafe { (*entry).private_data as *mut snd_tscm };
    let mut data: __be32 = 0;
    let mut reg: c_uint;
    let mut fpga: c_uint;
    let mut arm: c_uint;
    let mut hw: c_uint;
    let mut err: c_int;

    err = unsafe {
        snd_fw_transaction(
            (*tscm).unit,
            TCODE_READ_QUADLET_REQUEST,
            TSCM_ADDR_BASE + TSCM_OFFSET_FIRMWARE_REGISTER,
            &mut data as *mut __be32 as *mut c_void,
            mem::size_of_val(&data),
            0,
        )
    };
    if err < 0 {
        return;
    }
    reg = unsafe { be32_to_cpu(data) };

    err = unsafe {
        snd_fw_transaction(
            (*tscm).unit,
            TCODE_READ_QUADLET_REQUEST,
            TSCM_ADDR_BASE + TSCM_OFFSET_FIRMWARE_FPGA,
            &mut data as *mut __be32 as *mut c_void,
            mem::size_of_val(&data),
            0,
        )
    };
    if err < 0 {
        return;
    }
    fpga = unsafe { be32_to_cpu(data) };

    err = unsafe {
        snd_fw_transaction(
            (*tscm).unit,
            TCODE_READ_QUADLET_REQUEST,
            TSCM_ADDR_BASE + TSCM_OFFSET_FIRMWARE_ARM,
            &mut data as *mut __be32 as *mut c_void,
            mem::size_of_val(&data),
            0,
        )
    };
    if err < 0 {
        return;
    }
    arm = unsafe { be32_to_cpu(data) };

    err = unsafe {
        snd_fw_transaction(
            (*tscm).unit,
            TCODE_READ_QUADLET_REQUEST,
            TSCM_ADDR_BASE + TSCM_OFFSET_FIRMWARE_HW,
            &mut data as *mut __be32 as *mut c_void,
            mem::size_of_val(&data),
            0,
        )
    };
    if err < 0 {
        return;
    }
    hw = unsafe { be32_to_cpu(data) };

    unsafe {
        snd_iprintf(
            buffer,
            b"Register: %d (0x%08x)\n\0".as_ptr() as *const c_char,
            reg & 0xffff,
            reg,
        );
        snd_iprintf(
            buffer,
            b"FPGA:     %d (0x%08x)\n\0".as_ptr() as *const c_char,
            fpga & 0xffff,
            fpga,
        );
        snd_iprintf(
            buffer,
            b"ARM:      %d (0x%08x)\n\0".as_ptr() as *const c_char,
            arm & 0xffff,
            arm,
        );
        snd_iprintf(
            buffer,
            b"Hardware: %d (0x%08x)\n\0".as_ptr() as *const c_char,
            hw >> 16,
            hw,
        );
    }
}

unsafe extern "C" fn add_node(
    tscm: *mut snd_tscm,
    root: *mut snd_info_entry,
    name: *const c_char,
    op: snd_info_text_op,
) {
    let entry: *mut snd_info_entry;

    entry = unsafe { snd_info_create_card_entry((*tscm).card, name, root) };
    if !entry.is_null() {
        unsafe { snd_info_set_text_ops(entry, tscm as *mut c_void, op) };
    }
}

#[no_mangle]
pub unsafe extern "C" fn snd_tscm_proc_init(tscm: *mut snd_tscm) {
    let root: *mut snd_info_entry;

    /*
     * All nodes are automatically removed at snd_card_disconnect(),
     * by following to link list.
     */
    root = unsafe {
        snd_info_create_card_entry(
            (*tscm).card,
            b"firewire\0".as_ptr() as *const c_char,
            (*(*tscm).card).proc_root,
        )
    };
    if root.is_null() {
        return;
    }
    unsafe {
        (*root).mode = S_IFDIR | 0o555;
    }

    unsafe {
        add_node(
            tscm,
            root,
            b"firmware\0".as_ptr() as *const c_char,
            Some(proc_read_firmware),
        );
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
