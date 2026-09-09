/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2009 Lemote, Inc.
 * Author: Wu Zhangjin <wuzhangjin@gmail.com>
 */

/* C header guard: __ASM_MACH_LOONGSON2EF_MACHINE_H */

/*
 * When CONFIG_LEMOTE_FULOONG2E is enabled, the Loongson machine type is
 * MACH_LEMOTE_FL2E.
 */
#[cfg(CONFIG_LEMOTE_FULOONG2E)]
pub const LOONGSON_MACHTYPE: u32 = MACH_LEMOTE_FL2E;

/* use fuloong2f as the default machine of LEMOTE_MACH2F */
#[cfg(CONFIG_LEMOTE_MACH2F)]
pub const LOONGSON_MACHTYPE: u32 = MACH_LEMOTE_FL2F;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
