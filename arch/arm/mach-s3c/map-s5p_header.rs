/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2010 Samsung Electronics Co., Ltd.
 *		http://www.samsung.com/
 *
 * S5P - Memory map definitions
 */

// Dependency equivalent of: #include "map-s3c.h"

#[inline]
pub const fn VA_VIC(x: usize) -> usize {
    S3C_VA_IRQ + (x * 0x10000)
}

pub const VA_VIC0: usize = VA_VIC(0);
pub const VA_VIC1: usize = VA_VIC(1);
pub const VA_VIC2: usize = VA_VIC(2);
pub const VA_VIC3: usize = VA_VIC(3);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
