/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <linux/notifier.h> are supplied externally.

#[repr(i32)]
pub enum die_val {
    DIE_OOPS = 1,
    DIE_FP,
    DIE_TRAP,
    DIE_RI,
    DIE_PAGE_FAULT,
    DIE_BREAK,
    DIE_SSTEPBP,
    DIE_MSAFP,
    DIE_UPROBE,
    DIE_UPROBE_XOL,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
