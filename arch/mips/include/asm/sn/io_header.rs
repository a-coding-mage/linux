/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2000, 2003 Ralf Baechle
 * Copyright (C) 2000 Silicon Graphics, Inc.
 */

// The CONFIG_SGI_IP27 conditional include of <asm/sn/sn0/hubio.h> is a
// build-time dependency supplied by the surrounding translation unit.

pub const IIO_ITTE_BASE: usize = 0x400160; // base of translation table entries

#[inline]
pub const fn IIO_ITTE(bigwin: usize) -> usize {
    IIO_ITTE_BASE.wrapping_add(8usize.wrapping_mul(bigwin))
}

pub const IIO_ITTE_OFFSET_BITS: u32 = 5; // size of offset field
pub const IIO_ITTE_OFFSET_MASK: usize = (1usize << IIO_ITTE_OFFSET_BITS) - 1;
pub const IIO_ITTE_OFFSET_SHIFT: u32 = 0;

pub const IIO_ITTE_WIDGET_BITS: u32 = 4; // size of widget field
pub const IIO_ITTE_WIDGET_MASK: usize = (1usize << IIO_ITTE_WIDGET_BITS) - 1;
pub const IIO_ITTE_WIDGET_SHIFT: u32 = 8;

pub const IIO_ITTE_IOSP: usize = 1; // I/O Space bit
pub const IIO_ITTE_IOSP_MASK: usize = 1;
pub const IIO_ITTE_IOSP_SHIFT: u32 = 12;
pub const HUB_PIO_MAP_TO_MEM: usize = 0;
pub const HUB_PIO_MAP_TO_IO: usize = 1;

pub const IIO_ITTE_INVALID_WIDGET: usize = 3; // an invalid widget

// External symbols supplied by the surrounding translation unit.
extern "C" {
    pub fn REMOTE_HUB_S(nasid: usize, offset: usize, value: usize);
    pub fn REMOTE_HUB_PTR(nasid: usize, offset: usize) -> *mut usize;
}

#[inline]
pub unsafe fn IIO_ITTE_PUT(
    nasid: usize,
    bigwin: usize,
    io_or_mem: usize,
    widget: usize,
    addr: usize,
) {
    REMOTE_HUB_S(
        nasid,
        IIO_ITTE(bigwin),
        ((((addr >> BWIN_SIZE_BITS) & IIO_ITTE_OFFSET_MASK) << IIO_ITTE_OFFSET_SHIFT)
            | (io_or_mem << IIO_ITTE_IOSP_SHIFT)
            | (((widget & IIO_ITTE_WIDGET_MASK) << IIO_ITTE_WIDGET_SHIFT))),
    );
}

#[inline]
pub unsafe fn IIO_ITTE_DISABLE(nasid: usize, bigwin: usize) {
    IIO_ITTE_PUT(
        nasid,
        HUB_PIO_MAP_TO_MEM,
        bigwin,
        IIO_ITTE_INVALID_WIDGET,
        0,
    );
}

#[inline]
pub unsafe fn IIO_ITTE_GET(nasid: usize, bigwin: usize) -> *mut usize {
    REMOTE_HUB_PTR(nasid, IIO_ITTE(bigwin))
}

/*
 * Takes the widget number and returns the IO PRB address of that widget.
 * The value is expected to be a widget number in the range 0, 8 - 0xF.
 */
#[inline]
pub const fn IIO_IOPRB(x: usize) -> usize {
    IIO_IOPRB_0.wrapping_add(
        (((if x < HUB_WIDGET_ID_MIN {
            x
        } else {
            x.wrapping_sub(HUB_WIDGET_ID_MIN.wrapping_sub(1))
        }) << 3)),
    )
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
