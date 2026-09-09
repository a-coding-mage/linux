/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2010-2011, The Linux Foundation. All rights reserved.
 */

// The C header includes linux/stringify.h for __stringify.
// PROCESSOR_MODEL_NAME is expected to be supplied by the build configuration.
pub const MODULE_ARCH_VERMAGIC: &str = concat!(stringify!(PROCESSOR_MODEL_NAME), " ");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
