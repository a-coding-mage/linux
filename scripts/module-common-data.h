/* SPDX-License-Identifier: GPL-2.0 */
/* Preprocessor input only. module-common.c supplies frontend validation. */
#define INCLUDE_VERMAGIC
#include <linux/vermagic.h>
#ifdef __GNUC_EXECUTION_CHARSET_NAME
LUPOS_EXEC_CHARSET __GNUC_EXECUTION_CHARSET_NAME
#elif defined(__clang__)
/* Clang only supports UTF-8 narrow execution strings. */
LUPOS_EXEC_CHARSET "UTF-8"
#else
#error "Rust common metadata requires a known narrow execution character set"
#endif
LUPOS_VERMAGIC VERMAGIC_STRING
LUPOS_BUILD_SALT CONFIG_BUILD_SALT
#ifdef CONFIG_LTO
LUPOS_LTO 1
#else
LUPOS_LTO 0
#endif
#ifdef CONFIG_UNWINDER_ORC
#include <asm/orc_hash.h>
LUPOS_ORC ORC_HASH
#endif
