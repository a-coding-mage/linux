/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * arch/powerpc/platforms/embedded6xx/flipper-pic.h
 *
 * Nintendo GameCube/Wii "Flipper" interrupt controller support.
 * Copyright (C) 2004-2009 The GameCube Linux Team
 * Copyright (C) 2007,2008,2009 Albert Herranz
 */

// C header guard: __FLIPPER_PIC_H

unsafe extern "C" {
    pub fn flipper_pic_get_irq() -> u32;

    // C __init annotation preserved as source intent.
    pub fn flipper_pic_probe();

    pub fn flipper_quiesce();
    pub fn flipper_platform_reset();
    pub fn flipper_is_reset_button_pressed() -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
