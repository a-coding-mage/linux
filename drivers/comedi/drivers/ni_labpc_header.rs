/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Header for ni_labpc ISA/PCMCIA/PCI drivers
 *
 * Copyright (C) 2003 Frank Mori Hess <fmhess@users.sourceforge.net>
 */

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum transfer_type {
    fifo_not_empty_transfer,
    fifo_half_full_transfer,
    isa_dma_transfer,
}

#[repr(C)]
pub struct labpc_boardinfo {
    pub name: *const ::std::os::raw::c_char,
    pub ai_speed: ::std::os::raw::c_int, /* maximum input speed in ns */
    /* can auto scan up in ai channels */
    pub ai_scan_up: ::std::os::raw::c_uint,
    /* has analog outputs */
    pub has_ao: ::std::os::raw::c_uint,
    /* has extra regs compared to pc+ */
    pub is_labpc1200: ::std::os::raw::c_uint,
}

#[repr(C)]
pub struct labpc_private {
    pub dma: *mut comedi_isadma,
    pub counter: *mut comedi_8254,

    /*  number of data points left to be taken */
    pub count: u64,
    /*  software copys of bits written to command registers */
    pub cmd1: ::std::os::raw::c_uint,
    pub cmd2: ::std::os::raw::c_uint,
    pub cmd3: ::std::os::raw::c_uint,
    pub cmd4: ::std::os::raw::c_uint,
    pub cmd5: ::std::os::raw::c_uint,
    pub cmd6: ::std::os::raw::c_uint,
    /*  store last read of board status registers */
    pub stat1: ::std::os::raw::c_uint,
    pub stat2: ::std::os::raw::c_uint,

    /* we are using dma/fifo-half-full/etc. */
    pub current_transfer: transfer_type,
    /*
     * function pointers so we can use inb/outb or readb/writeb as
     * appropriate
     */
    pub read_byte: Option<unsafe extern "C" fn(
        dev: *mut comedi_device,
        reg: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_uint>,
    pub write_byte: Option<unsafe extern "C" fn(
        dev: *mut comedi_device,
        byte: ::std::os::raw::c_uint,
        reg: ::std::os::raw::c_ulong,
    )>,
}

extern "C" {
    pub fn labpc_common_attach(
        dev: *mut comedi_device,
        irq: ::std::os::raw::c_uint,
        isr_flags: ::std::os::raw::c_ulong,
    ) -> ::std::os::raw::c_int;
    pub fn labpc_common_detach(dev: *mut comedi_device);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
