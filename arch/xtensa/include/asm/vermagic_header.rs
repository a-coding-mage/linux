/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// Dependency intent preserved from <linux/stringify.h> and <variant/core.h>.

pub const MODULE_ARCH_VERMAGIC: &str = concat!("xtensa-", stringify!(XCHAL_CORE_ID), " ");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
