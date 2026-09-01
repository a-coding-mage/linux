/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <linux/dma-mapping.h>
// #include <linux/firewire.h>

/**
 * struct iso_packets_buffer - manages a buffer for many packets
 * @iso_buffer: the memory containing the packets
 * @packets: an array, with each element pointing to one packet
 */
#[repr(C)]
pub struct iso_packets_buffer {
    pub iso_buffer: fw_iso_buffer,
    pub packets: *mut iso_packets_buffer_packet,
}

#[repr(C)]
pub struct iso_packets_buffer_packet {
    pub buffer: *mut core::ffi::c_void,
    pub offset: core::ffi::c_uint,
}

unsafe extern "C" {
    pub fn iso_packets_buffer_init(
        b: *mut iso_packets_buffer,
        unit: *mut fw_unit,
        count: core::ffi::c_uint,
        packet_size: core::ffi::c_uint,
        direction: dma_data_direction,
    ) -> core::ffi::c_int;

    pub fn iso_packets_buffer_destroy(b: *mut iso_packets_buffer, unit: *mut fw_unit);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
