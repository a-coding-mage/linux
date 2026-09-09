/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * das08.h
 *
 * Header for common DAS08 support (used by ISA/PCI/PCMCIA drivers)
 *
 * Copyright (C) 2003 Frank Mori Hess <fmhess@users.sourceforge.net>
 */

use core::ffi::{c_char, c_ulong};

// Forward declaration corresponding to `struct comedi_device`.
#[repr(C)]
pub struct comedi_device {
    _private: [u8; 0],
}

/* different ways ai data is encoded in first two registers */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum das08_ai_encoding {
    das08_encode12,
    das08_encode16,
    das08_pcm_encode12,
}

/* types of ai range table used by different boards */
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum das08_lrange {
    das08_pg_none,
    das08_bipolar5,
    das08_pgh,
    das08_pgl,
    das08_pgm,
}

#[repr(C)]
pub struct das08_board_struct {
    pub name: *const c_char,
    pub is_jr: bool, /* true for 'JR' boards */
    pub ai_nbits: u32,
    pub ai_pg: das08_lrange,
    pub ai_encoding: das08_ai_encoding,
    pub ao_nbits: u32,
    pub di_nchan: u32,
    pub do_nchan: u32,
    pub i8255_offset: u32,
    pub i8254_offset: u32,
    pub iosize: u32, /* number of ioports used */
}

#[repr(C)]
pub struct das08_private_struct {
    /* bits for do/mux register on boards without separate do register */
    pub do_mux_bits: u32,
    pub pg_gainlist: *const u32,
}

unsafe extern "C" {
    pub fn das08_common_attach(dev: *mut comedi_device, iobase: c_ulong) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
