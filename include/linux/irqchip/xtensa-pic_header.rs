/*
 * Xtensa built-in interrupt controller
 *
 * Copyright (C) 2002 - 2013 Tensilica, Inc.
 * Copyright (C) 1992, 1998 Linus Torvalds, Ingo Molnar
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Corresponds to the C header guard __LINUX_IRQCHIP_XTENSA_PIC_H.

pub struct device_node;

unsafe extern "C" {
    pub fn xtensa_pic_init_legacy(interrupt_parent: *mut device_node);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
