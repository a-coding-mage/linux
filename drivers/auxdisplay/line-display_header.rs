/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Character line display core support
 *
 * Copyright (C) 2016 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 *
 * Copyright (C) 2021 Glider bv
 * Copyright (C) 2025 Jean-François Lessard
 */

// Dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct timer_list {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg7_conversion_map {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seg14_conversion_map {
    _private: [u8; 0],
}

pub struct linedisp;

/**
 * enum linedisp_map_type - type of the character mapping
 * @LINEDISP_MAP_SEG7: Map characters to 7 segment display
 * @LINEDISP_MAP_SEG14: Map characters to 14 segment display
 */
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum linedisp_map_type {
    LINEDISP_MAP_SEG7,
    LINEDISP_MAP_SEG14,
}

/**
 * struct linedisp_map - character mapping
 * @type: type of the character mapping
 * @map: conversion character mapping
 * @size: size of the @map
 */
#[repr(C)]
pub union linedisp_map_union {
    pub seg7: seg7_conversion_map,
    pub seg14: seg14_conversion_map,
}

#[repr(C)]
pub struct linedisp_map {
    pub type_: linedisp_map_type,
    pub map: linedisp_map_union,
    pub size: u32,
}

/**
 * struct linedisp_ops - character line display operations
 * @get_map_type: Function called to get the character mapping, if required
 * @update: Function called to update the display. This must not sleep!
 */
#[repr(C)]
pub struct linedisp_ops {
    pub get_map_type: Option<unsafe extern "C" fn(linedisp: *mut linedisp) -> i32>,
    pub update: Option<unsafe extern "C" fn(linedisp: *mut linedisp)>,
}

/**
 * struct linedisp - character line display private data structure
 * @dev: the line display device
 * @timer: timer used to implement scrolling
 * @ops: character line display operations
 * @buf: pointer to the buffer for the string currently displayed
 * @message: the full message to display or scroll on the display
 * @num_chars: the number of characters that can be displayed
 * @message_len: the length of the @message string
 * @scroll_pos: index of the first character of @message currently displayed
 * @scroll_rate: scroll interval in jiffies
 * @id: instance id of this display
 */
#[repr(C)]
pub struct linedisp {
    pub dev: device,
    pub timer: timer_list,
    pub ops: *const linedisp_ops,
    pub map: *mut linedisp_map,
    pub buf: *mut i8,
    pub message: *mut i8,
    pub num_chars: u32,
    pub message_len: u32,
    pub scroll_pos: u32,
    pub scroll_rate: u32,
    pub id: u32,
}

extern "C" {
    pub fn linedisp_attach(
        linedisp: *mut linedisp,
        dev: *mut device,
        num_chars: u32,
        ops: *const linedisp_ops,
    ) -> i32;
    pub fn linedisp_detach(dev: *mut device);
    pub fn linedisp_register(
        linedisp: *mut linedisp,
        parent: *mut device,
        num_chars: u32,
        ops: *const linedisp_ops,
    ) -> i32;
    pub fn linedisp_unregister(linedisp: *mut linedisp);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
