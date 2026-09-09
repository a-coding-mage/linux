/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 *  linux/zorro.h -- Amiga AutoConfig (Zorro) Bus Definitions
 *
 *  Copyright (C) 1995--2003 Geert Uytterhoeven
 *
 *  This file is subject to the terms and conditions of the GNU General Public
 *  License.  See the file COPYING in the main directory of this archive
 *  for more details.
 */

/* Dependency: Linux integer types and the zorro ID list are supplied externally. */

/*
 *  Each Zorro board has a 32-bit ID of the form
 *
 *      mmmmmmmmmmmmmmmmppppppppeeeeeeee
 *
 *  with
 *
 *      mmmmmmmmmmmmmmmm  16-bit Manufacturer ID (assigned by CBM (sigh))
 *      pppppppp           8-bit Product ID (assigned by manufacturer)
 *      eeeeeeee           8-bit Extended Product ID (currently only used
 *                         for some GVP boards)
 */

#[inline]
pub const fn zorro_manuf(id: u32) -> u32 {
    id >> 16
}

#[inline]
pub const fn zorro_prod(id: u32) -> u32 {
    (id >> 8) & 0xff
}

#[inline]
pub const fn zorro_epc(id: u32) -> u32 {
    id & 0xff
}

/* C macro ZORRO_ID(manuf, prod, epc): ZORRO_MANUF_##manuf is token-pasted. */

pub type zorro_id = u32;

/* Include the ID list supplied by the surrounding translation unit. */

/*
 *  GVP identifies most of its products through the 'extended product code'
 *  (epc). The epc has to be ANDed with the GVP_PRODMASK before the
 *  identification.
 */

pub const GVP_PRODMASK: u32 = 0xf8;
pub const GVP_SCSICLKMASK: u32 = 0x01;

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GVP_flags {
    GVP_IO = 0x01,
    GVP_ACCEL = 0x02,
    GVP_SCSI = 0x04,
    GVP_24BITDMA = 0x08,
    GVP_25BITDMA = 0x10,
    GVP_NOBANK = 0x20,
    GVP_14MHZ = 0x40,
}

#[repr(C, packed)]
pub struct Node {
    pub ln_Succ: u32, // Pointer to next (successor)
    pub ln_Pred: u32, // Pointer to previous (predecessor)
    pub ln_Type: u8,
    pub ln_Pri: i8, // Priority, for sorting
    pub ln_Name: u32, // ID string, null terminated
}

#[repr(C, packed)]
pub struct ExpansionRom {
    /* -First 16 bytes of the expansion ROM */
    pub er_Type: u8, // Board type, size and flags
    pub er_Product: u8, // Product number, assigned by manufacturer
    pub er_Flags: u8, // Flags
    pub er_Reserved03: u8, // Must be zero ($ff inverted)
    pub er_Manufacturer: u16, // Unique ID, ASSIGNED BY COMMODORE-AMIGA!
    pub er_SerialNumber: u32, // Available for use by manufacturer
    pub er_InitDiagVec: u16, // Offset to optional "DiagArea" structure
    pub er_Reserved0c: u8,
    pub er_Reserved0d: u8,
    pub er_Reserved0e: u8,
    pub er_Reserved0f: u8,
}

/* er_Type board type bits */
pub const ERT_TYPEMASK: u8 = 0xc0;
pub const ERT_ZORROII: u8 = 0xc0;
pub const ERT_ZORROIII: u8 = 0x80;

/* other bits defined in er_Type */
pub const ERTB_MEMLIST: u32 = 5; // Link RAM into free memory list
pub const ERTF_MEMLIST: u8 = 1 << 5;

#[repr(C, packed)]
pub struct ConfigDev {
    pub cd_Node: Node,
    pub cd_Flags: u8, // (read/write)
    pub cd_Pad: u8, // reserved
    pub cd_Rom: ExpansionRom, // copy of board's expansion ROM
    pub cd_BoardAddr: u32, // where in memory the board was placed
    pub cd_BoardSize: u32, // size of board in bytes
    pub cd_SlotAddr: u16, // which slot number (PRIVATE)
    pub cd_SlotSize: u16, // number of slots (PRIVATE)
    pub cd_Driver: u32, // pointer to node of driver
    pub cd_NextCD: u32, // linked list of drivers to config
    pub cd_Unused: [u32; 4], // for whatever the driver wants
}

pub const ZORRO_NUM_AUTO: u32 = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
