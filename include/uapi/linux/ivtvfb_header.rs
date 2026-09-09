/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
    On Screen Display cx23415 Framebuffer driver

    Copyright (C) 2006, 2007  Ian Armstrong <ian@iarmst.demon.co.uk>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 */

// The original header includes <linux/compiler.h> and <linux/types.h>.

/* Framebuffer external API */
#[repr(C)]
pub struct ivtvfb_dma_frame {
    pub source: *mut core::ffi::c_void,
    pub dest_offset: libc::c_ulong,
    pub count: libc::c_int,
}

// Original definition:
// #define IVTVFB_IOC_DMA_FRAME _IOW('V', BASE_VIDIOC_PRIVATE+0, struct ivtvfb_dma_frame)
// `_IOW` and `BASE_VIDIOC_PRIVATE` are supplied by the Linux ioctl headers.
pub const IVTVFB_IOC_DMA_FRAME: libc::c_ulong =
    _IOW(b'V' as libc::c_ulong, BASE_VIDIOC_PRIVATE + 0, ivtvfb_dma_frame);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
