// SPDX-License-Identifier: GPL-2.0-only
/*
 * Native record for the Rust ELF core header owner: the early_param()
 * entry of "elfcorehdr=", which calls setup_elfcorehdr() in
 * kernel/elfcorehdr.rs.
 */
#include <linux/build_bug.h>
#include <linux/crash_dump.h>
#include <linux/init.h>

/* kernel/elfcorehdr.rs spells ELFCORE_ADDR_MAX. */
static_assert(ELFCORE_ADDR_MAX == ~0ULL);

int __init setup_elfcorehdr(char *arg);
early_param("elfcorehdr", setup_elfcorehdr);
