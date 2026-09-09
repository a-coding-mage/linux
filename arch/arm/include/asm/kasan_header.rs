/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/arm/include/asm/kasan.h
 *
 * Copyright (c) 2015 Samsung Electronics Co., Ltd.
 * Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
 *
 */

// The following declarations are active when CONFIG_KASAN is enabled.
// Dependency: <asm/kasan_def.h>

#[cfg(CONFIG_KASAN)]
pub const KASAN_SHADOW_SCALE_SHIFT: u32 = 3;

/*
 * The compiler uses a shadow offset assuming that addresses start
 * from 0. Kernel addresses don't start from 0, so shadow
 * for kernel really starts from 'compiler's shadow offset' +
 * ('kernel address space start' >> KASAN_SHADOW_SCALE_SHIFT)
 */

#[cfg(CONFIG_KASAN)]
extern "C" {
    pub fn kasan_early_init();
    pub fn kasan_init();
}

#[cfg(not(CONFIG_KASAN))]
#[inline]
pub fn kasan_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
