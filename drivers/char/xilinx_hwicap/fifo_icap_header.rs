/*
 *     Author: Xilinx, Inc.
 *
 *     This program is free software; you can redistribute it and/or modify it
 *     under the terms of the GNU General Public License as published by the
 *     Free Software Foundation; either version 2 of the License, or (at your
 *     option) any later version.
 *
 *     XILINX IS PROVIDING THIS DESIGN, CODE, OR INFORMATION "AS IS"
 *     AS A COURTESY TO YOU, SOLELY FOR USE IN DEVELOPING PROGRAMS AND
 *     SOLUTIONS FOR XILINX DEVICES.  BY PROVIDING THIS DESIGN, CODE,
 *     OR INFORMATION AS ONE POSSIBLE IMPLEMENTATION OF THIS FEATURE,
 *     APPLICATION OR STANDARD, XILINX IS MAKING NO REPRESENTATION
 *     THAT THIS IMPLEMENTATION IS FREE FROM ANY CLAIMS OF INFRINGEMENT,
 *     AND YOU ARE RESPONSIBLE FOR OBTAINING ANY RIGHTS YOU MAY REQUIRE
 *     FOR YOUR IMPLEMENTATION.  XILINX EXPRESSLY DISCLAIMS ANY
 *     WARRANTY WHATSOEVER WITH RESPECT TO THE ADEQUACY OF THE
 *     IMPLEMENTATION, INCLUDING BUT NOT LIMITED TO ANY WARRANTIES OR
 *     REPRESENTATIONS THAT THIS IMPLEMENTATION IS FREE FROM CLAIMS OF
 *     INFRINGEMENT, IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 *     FOR A PARTICULAR PURPOSE.
 *
 *     (c) Copyright 2007-2008 Xilinx Inc.
 *     All rights reserved.
 *
 *     You should have received a copy of the GNU General Public License along
 *     with this program; if not, write to the Free Software Foundation, Inc.,
 *     675 Mass Ave, Cambridge, MA 02139, USA.
 */

// C header dependencies: linux/types.h, linux/cdev.h, linux/platform_device.h,
// asm/io.h, and xilinx_hwicap.h supply the referenced types and interfaces.

/// Opaque type supplied by the external xilinx_hwicap dependency.
#[repr(C)]
pub struct hwicap_drvdata {
    _opaque: [u8; 0],
}

/* Reads integers from the device into the storage buffer. */
extern "C" {
    pub fn fifo_icap_get_configuration(
        drvdata: *mut hwicap_drvdata,
        FrameBuffer: *mut u32,
        NumWords: u32,
    ) -> i32;

    /* Writes integers to the device from the storage buffer. */
    pub fn fifo_icap_set_configuration(
        drvdata: *mut hwicap_drvdata,
        FrameBuffer: *mut u32,
        NumWords: u32,
    ) -> i32;

    pub fn fifo_icap_get_status(drvdata: *mut hwicap_drvdata) -> u32;
    pub fn fifo_icap_reset(drvdata: *mut hwicap_drvdata);
    pub fn fifo_icap_flush_fifo(drvdata: *mut hwicap_drvdata);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
