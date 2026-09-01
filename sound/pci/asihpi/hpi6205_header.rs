/* SPDX-License-Identifier: GPL-2.0-only */
/*****************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2011  AudioScience Inc. <support@audioscience.com>


Host Interface module for an ASI6205 based
bus mastering PCI adapter.

Copyright AudioScience, Inc., 2003
******************************************************************************/

/* C header dependency: "hpi_internal.h" */

/***********************************************************
	Defines used for basic messaging
************************************************************/
pub const H620_HIF_RESET: u32 = 0;
pub const H620_HIF_IDLE: u32 = 1;
pub const H620_HIF_GET_RESP: u32 = 2;
pub const H620_HIF_DATA_DONE: u32 = 3;
pub const H620_HIF_DATA_MASK: u32 = 0x10;
pub const H620_HIF_SEND_DATA: u32 = 0x14;
pub const H620_HIF_GET_DATA: u32 = 0x15;
pub const H620_HIF_UNKNOWN: u32 = 0x0000ffff;

/***********************************************************
	Types used for mixer control caching
************************************************************/

pub const H620_MAX_ISTREAMS: usize = 32;
pub const H620_MAX_OSTREAMS: usize = 32;
pub const HPI_NMIXER_CONTROLS: usize = 2048;

/*********************************************************************
This is used for dynamic control cache allocation
**********************************************************************/
#[repr(C)]
pub struct controlcache_6205 {
	pub number_of_controls: u32,
	pub physical_address32: u32,
	pub size_in_bytes: u32,
}

/*********************************************************************
This is used for dynamic allocation of async event array
**********************************************************************/
#[repr(C)]
pub struct async_event_buffer_6205 {
	pub physical_address32: u32,
	pub spare: u32,
	pub b: hpi_fifo_buffer,
}

/***********************************************************
The Host located memory buffer that the 6205 will bus master
in and out of.
************************************************************/
pub const HPI6205_SIZEOF_DATA: usize = 16 * 1024;

#[repr(C)]
pub struct message_buffer_6205 {
	pub message: hpi_message,
	pub data: [::std::os::raw::c_char; 256],
}

#[repr(C)]
pub struct response_buffer_6205 {
	pub response: hpi_response,
	pub data: [::std::os::raw::c_char; 256],
}

#[repr(C)]
pub union buffer_6205 {
	pub message_buffer: ::std::mem::ManuallyDrop<message_buffer_6205>,
	pub response_buffer: ::std::mem::ManuallyDrop<response_buffer_6205>,
	pub b_data: [u8; HPI6205_SIZEOF_DATA],
}

#[repr(C)]
pub struct bus_master_interface {
	pub host_cmd: u32,
	pub dsp_ack: u32,
	pub transfer_size_in_bytes: u32,
	pub u: buffer_6205,
	pub control_cache: controlcache_6205,
	pub async_buffer: async_event_buffer_6205,
	pub instream_host_buffer_status: [hpi_hostbuffer_status; H620_MAX_ISTREAMS],
	pub outstream_host_buffer_status: [hpi_hostbuffer_status; H620_MAX_OSTREAMS],
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
