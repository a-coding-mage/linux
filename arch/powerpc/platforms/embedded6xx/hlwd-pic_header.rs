/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arch/powerpc/platforms/embedded6xx/hlwd-pic.h
 *
 * Nintendo Wii "Hollywood" interrupt controller support.
 * Copyright (C) 2009 The GameCube Linux Team
 * Copyright (C) 2009 Albert Herranz
 */

// C header guard: __HLWD_PIC_H

extern "C" {
    pub fn hlwd_pic_get_irq() -> u32;
    // C annotation: __init
    pub fn hlwd_pic_probe();
    pub fn hlwd_quiesce();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
