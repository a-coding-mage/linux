/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	Definitions of structures for vfsv0 quota format
 */

/* Dependencies: <linux/types.h> and <linux/quota.h>. */

/*
 *  Structure of header of block with quota structures. It is padded to 16 bytes so
 *  there will be space for exactly 21 quota-entries in a block
 */
#[repr(C)]
pub struct qt_disk_dqdbheader {
	pub dqdh_next_free: u32, /* Number of next block with free entry */
	pub dqdh_prev_free: u32, /* Number of previous block with free entry */
	pub dqdh_entries: u16, /* Number of valid entries in block */
	pub dqdh_pad1: u16,
	pub dqdh_pad2: u32,
}

pub const QT_TREEOFF: u32 = 1; /* Offset of tree in file in blocks */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
