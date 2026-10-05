/* SPDX-License-Identifier: GPL-2.0 */
#ifndef LUPOS_CAPABILITY_BOOT_NATIVE_H
#define LUPOS_CAPABILITY_BOOT_NATIVE_H

/* Native declaration of the single Rust-owned file_caps_enabled object. */
#include <linux/capability.h>

/* The Rust handler uses kernel::ffi::{c_int, c_char}, not a guessed ABI. */
int lupos_capability_file_caps_disable(char *str);

#endif /* LUPOS_CAPABILITY_BOOT_NATIVE_H */
