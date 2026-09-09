/* SPDX-License-Identifier: GPL-2.0 */

pub const KLDIR_MAGIC: u64 = 0x434d_5f53_505f_5357;

pub const KLDIR_OFF_MAGIC: usize = 0x00;
pub const KLDIR_OFF_OFFSET: usize = 0x08;
pub const KLDIR_OFF_POINTER: usize = 0x10;
pub const KLDIR_OFF_SIZE: usize = 0x18;
pub const KLDIR_OFF_COUNT: usize = 0x20;
pub const KLDIR_OFF_STRIDE: usize = 0x28;

pub const KLDIR_ENT_SIZE: usize = 0x40;
pub const KLDIR_MAX_ENTRIES: usize = 0x400 / 0x40;

#[repr(C)]
pub struct kldir_ent_s {
    pub magic: u64,          /* Indicates validity of entry     */
    pub offset: off_t,      /* Offset from start of node space  */
    pub pointer: usize,     /* Pointer to area in some cases    */
    pub size: usize,        /* Size in bytes                    */
    pub count: u64,         /* Repeat count if array, 1 if not  */
    pub stride: usize,      /* Stride if array, 0 if not       */
    pub rsvd: [i8; 16],     /* Pad entry to 0x40 bytes          */
    /* NOTE: These 16 bytes are used in the Partition KLDIR
       entry to store partition info. Refer to klpart.h for this. */
}

pub type kldir_ent_t = kldir_ent_s;

/* When CONFIG_SGI_IP27 is enabled, the C header includes <asm/sn/sn0/kldir.h>. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
