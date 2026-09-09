/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2006 Sony Computer Entertainment Inc.
 * Copyright 2006, 2007 Sony Corporation
 */

// Translated from the PowerPC Linux UAPI header `ps3fb.h`.
// The original header includes <linux/types.h> and <linux/ioctl.h>.

/* ioctl */
const PS3FB_IOC_NRBITS: u32 = 8;
const PS3FB_IOC_TYPEBITS: u32 = 8;
const PS3FB_IOC_SIZEBITS: u32 = 14;
const PS3FB_IOC_NRSHIFT: u32 = 0;
const PS3FB_IOC_TYPESHIFT: u32 = PS3FB_IOC_NRSHIFT + PS3FB_IOC_NRBITS;
const PS3FB_IOC_SIZESHIFT: u32 = PS3FB_IOC_TYPESHIFT + PS3FB_IOC_TYPEBITS;
const PS3FB_IOC_DIRSHIFT: u32 = PS3FB_IOC_SIZESHIFT + PS3FB_IOC_SIZEBITS;
const PS3FB_IOC_WRITE: u32 = 1;
const PS3FB_IOC_READ: u32 = 2;

const fn ps3fb_ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << PS3FB_IOC_DIRSHIFT)
        | (size << PS3FB_IOC_SIZESHIFT)
        | (ty << PS3FB_IOC_TYPESHIFT)
        | (nr << PS3FB_IOC_NRSHIFT)
}

pub const PS3FB_IOCTL_SETMODE: u32 = ps3fb_ioc(PS3FB_IOC_WRITE, b'r' as u32, 1, core::mem::size_of::<i32>() as u32); /* set video mode */
pub const PS3FB_IOCTL_GETMODE: u32 = ps3fb_ioc(PS3FB_IOC_READ, b'r' as u32, 2, core::mem::size_of::<i32>() as u32); /* get video mode */
pub const PS3FB_IOCTL_SCREENINFO: u32 = ps3fb_ioc(PS3FB_IOC_READ, b'r' as u32, 3, core::mem::size_of::<i32>() as u32); /* get screen info */
pub const PS3FB_IOCTL_ON: u32 = ps3fb_ioc(0, b'r' as u32, 4, 0); /* use IOCTL_FSEL */
pub const PS3FB_IOCTL_OFF: u32 = ps3fb_ioc(0, b'r' as u32, 5, 0); /* return to normal-flip */
pub const PS3FB_IOCTL_FSEL: u32 = ps3fb_ioc(PS3FB_IOC_WRITE, b'r' as u32, 6, core::mem::size_of::<i32>() as u32); /* blit and flip request */

// Defined only when FBIO_WAITFORVSYNC is not supplied by the surrounding UAPI.
pub const FBIO_WAITFORVSYNC: u32 = ps3fb_ioc(PS3FB_IOC_WRITE, b'F' as u32, 0x20, core::mem::size_of::<u32>() as u32); /* wait for vsync */

#[repr(C)]
pub struct ps3fb_ioctl_res {
    pub xres: u32, /* frame buffer x_size */
    pub yres: u32, /* frame buffer y_size */
    pub xoff: u32, /* margine x  */
    pub yoff: u32, /* margine y */
    pub num_frames: u32, /* num of frame buffers */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
