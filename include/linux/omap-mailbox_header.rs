/* SPDX-License-Identifier: GPL-2.0 */
/*
 * omap-mailbox: interprocessor communication module for OMAP
 */

// C header guard: OMAP_MAILBOX_H

pub type mbox_msg_t = usize; // uintptr_t

macro_rules! omap_mbox_message {
    ($data:expr) => {
        ($data as mbox_msg_t as u32)
    };
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
