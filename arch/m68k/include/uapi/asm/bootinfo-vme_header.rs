/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
** asm/bootinfo-vme.h -- VME-specific boot information definitions
*/

/* VME-specific tags */
pub const BI_VME_TYPE: u32 = 0x8000; // VME sub-architecture (__be32)
pub const BI_VME_BRDINFO: u32 = 0x8001; // VME board information (struct)

/* VME models (BI_VME_TYPE) */
pub const VME_TYPE_TP34V: u32 = 0x0034; // Tadpole TP34V
pub const VME_TYPE_MVME147: u32 = 0x0147; // Motorola MVME147
pub const VME_TYPE_MVME162: u32 = 0x0162; // Motorola MVME162
pub const VME_TYPE_MVME166: u32 = 0x0166; // Motorola MVME166
pub const VME_TYPE_MVME167: u32 = 0x0167; // Motorola MVME167
pub const VME_TYPE_MVME172: u32 = 0x0172; // Motorola MVME172
pub const VME_TYPE_MVME177: u32 = 0x0177; // Motorola MVME177
pub const VME_TYPE_BVME4000: u32 = 0x4000; // BVM Ltd. BVME4000
pub const VME_TYPE_BVME6000: u32 = 0x6000; // BVM Ltd. BVME6000

/*
 * Board ID data structure - pointer to this retrieved from Bug by head.S
 *
 * BI_VME_BRDINFO is a 32 byte struct as returned by the Bug code on
 * Motorola VME boards. Contains board number, Bug version, board
 * configuration options, etc.
 *
 * Note, bytes 12 and 13 are board no in BCD (0162,0166,0167,0177,etc)
 */
#[repr(C)]
pub struct t_bdid {
    pub bdid: [i8; 4],
    pub rev: u8,
    pub mth: u8,
    pub day: u8,
    pub yr: u8,
    pub size: u16,
    pub reserved: u16,
    pub brdno: u16,
    pub brdsuffix: [i8; 2],
    pub options: u32,
    pub clun: u16,
    pub dlun: u16,
    pub ctype: u16,
    pub dnum: u16,
    pub option2: u32,
}

pub type p_bdid = *mut t_bdid;

/* Latest VME bootinfo versions; MK_BI_VERSION(2, 0). */
pub const MVME147_BOOTI_VERSION: u32 = 0x0002_0000;
pub const MVME16x_BOOTI_VERSION: u32 = 0x0002_0000;
pub const BVME6000_BOOTI_VERSION: u32 = 0x0002_0000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
