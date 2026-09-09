/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// C header guard: __LOONGARCH_KERNEL_IMAGE_VARS_H

// CONFIG_EFI_STUB
//
// The source declarations below are linker-symbol assignments. Rust has no
// file-local equivalent for assigning one externally defined symbol to another;
// preserve the required aliases here for the linker integration:
//   __efistub_strcmp       = strcmp;
//   __efistub_kernel_entry = kernel_entry;
//   __efistub_kernel_asize = kernel_asize;
//   __efistub_kernel_fsize = kernel_fsize;
//
// CONFIG_EFI_EARLYCON || CONFIG_SYSFB
//   __efistub_sysfb_primary_display = sysfb_primary_display;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
