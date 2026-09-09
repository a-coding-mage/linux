/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * xtalk.h -- platform-independent crosstalk interface, derived from
 * IRIX <sys/PCI/bridge.h>, revision 1.38.
 *
 * Copyright (C) 1995 - 1997, 1999 Silcon Graphics, Inc.
 * Copyright (C) 1999 Ralf Baechle (ralf@gnu.org)
 */

/*
 * User-level device driver visible types
 */
pub type xwidgetnum_t = i8; /* xtalk widget number (0..15) */

pub const XWIDGET_NONE: xwidgetnum_t = -1;

pub type xwidget_part_num_t = i32; /* xtalk widget part number */

pub const XWIDGET_PART_NUM_NONE: xwidget_part_num_t = -1;

pub type xwidget_rev_num_t = i32; /* xtalk widget revision number */

pub const XWIDGET_REV_NUM_NONE: xwidget_rev_num_t = -1;

pub type xwidget_mfg_num_t = i32; /* xtalk widget manufacturing ID */

pub const XWIDGET_MFG_NUM_NONE: xwidget_mfg_num_t = -1;

#[repr(C)]
pub struct xtalk_piomap_s {
    _private: [u8; 0],
}

pub type xtalk_piomap_t = *mut xtalk_piomap_s;

/* It is often convenient to fold the XIO target port
 * number into the XIO address.
 */
pub const XIO_NOWHERE: u64 = 0xFFFFFFFFFFFFFFFFu64;
pub const XIO_ADDR_BITS: u64 = 0x0000FFFFFFFFFFFFu64;
pub const XIO_PORT_BITS: u64 = 0xF000000000000000u64;
pub const XIO_PORT_SHIFT: u32 = 60;

#[macro_export]
macro_rules! XIO_PACKED {
    ($x:expr) => {
        (($x & $crate::XIO_PORT_BITS) != 0)
    };
}

#[macro_export]
macro_rules! XIO_ADDR {
    ($x:expr) => {
        ($x & $crate::XIO_ADDR_BITS)
    };
}

#[macro_export]
macro_rules! XIO_PORT {
    ($x:expr) => {
        ((($x & $crate::XIO_PORT_BITS) >> $crate::XIO_PORT_SHIFT) as $crate::xwidgetnum_t)
    };
}

#[macro_export]
macro_rules! XIO_PACK {
    ($p:expr, $o:expr) => {
        ((($p as u64) << $crate::XIO_PORT_SHIFT) | ($o & $crate::XIO_ADDR_BITS))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
