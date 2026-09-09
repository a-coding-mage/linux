/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <uapi/asm/fbio.h>.

// The following ioctl values correspond to the C _IOW macros:
// FBIOPUTCMAP_SPARC = _IOW('F', 3, struct fbcmap)
// FBIOGETCMAP_SPARC = _IOW('F', 4, struct fbcmap)

/* Addresses on the fd of a cgsix that are mappable */
pub const CG6_FBC: u32 = 0x70000000;
pub const CG6_TEC: u32 = 0x70001000;
pub const CG6_BTREGS: u32 = 0x70002000;
pub const CG6_FHC: u32 = 0x70004000;
pub const CG6_THC: u32 = 0x70005000;
pub const CG6_ROM: u32 = 0x70006000;
pub const CG6_RAM: u32 = 0x70016000;
pub const CG6_DHC: u32 = 0x80000000;

pub const CG3_MMAP_OFFSET: u32 = 0x4000000;

/* Addresses on the fd of a tcx that are mappable */
pub const TCX_RAM8BIT: u32 = 0x00000000;
pub const TCX_RAM24BIT: u32 = 0x01000000;
pub const TCX_UNK3: u32 = 0x10000000;
pub const TCX_UNK4: u32 = 0x20000000;
pub const TCX_CONTROLPLANE: u32 = 0x28000000;
pub const TCX_UNK6: u32 = 0x30000000;
pub const TCX_UNK7: u32 = 0x38000000;
pub const TCX_TEC: u32 = 0x70000000;
pub const TCX_BTREGS: u32 = 0x70002000;
pub const TCX_THC: u32 = 0x70004000;
pub const TCX_DHC: u32 = 0x70008000;
pub const TCX_ALT: u32 = 0x7000a000;
pub const TCX_SYNC: u32 = 0x7000e000;
pub const TCX_UNK2: u32 = 0x70010000;

/* CG14 definitions */

/* Offsets into the OBIO space: */
pub const CG14_REGS: u32 = 0;       /* registers */
pub const CG14_CURSORREGS: u32 = 0x1000;  /* cursor registers */
pub const CG14_DACREGS: u32 = 0x2000;  /* DAC registers */
pub const CG14_XLUT: u32 = 0x3000;  /* X Look Up Table -- ??? */
pub const CG14_CLUT1: u32 = 0x4000;  /* Color Look Up Table */
pub const CG14_CLUT2: u32 = 0x5000;  /* Color Look Up Table */
pub const CG14_CLUT3: u32 = 0x6000;  /* Color Look Up Table */
pub const CG14_AUTO: u32 = 0xf000;

#[repr(C)]
pub struct fbcmap32 {
    pub index: i32,          /* first element (0 origin) */
    pub count: i32,
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

// The following ioctl values correspond to the C _IOW macros:
// FBIOPUTCMAP32 = _IOW('F', 3, struct fbcmap32)
// FBIOGETCMAP32 = _IOW('F', 4, struct fbcmap32)

#[repr(C)]
pub struct fbcursor32 {
    pub set: i16,       /* what to set, choose from the list above */
    pub enable: i16,    /* cursor on/off */
    pub pos: fbcurpos,  /* cursor position */
    pub hot: fbcurpos,  /* cursor hot spot */
    pub cmap: fbcmap32, /* color map info */
    pub size: fbcurpos, /* cursor bit map size */
    pub image: u32,     /* cursor image bits */
    pub mask: u32,      /* cursor mask bits */
}

// The following ioctl values correspond to the C _IOW macros:
// FBIOSCURSOR32 = _IOW('F', 24, struct fbcursor32)
// FBIOGCURSOR32 = _IOW('F', 25, struct fbcursor32)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
