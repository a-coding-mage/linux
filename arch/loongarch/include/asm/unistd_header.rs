/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Author: Hanlu Li <lihanlu@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependency supplied by the included UAPI header: `uapi/asm/unistd.h`.

// Architecture syscall feature-selection macros from the C header.
pub const __ARCH_WANT_NEW_STAT: bool = true;
pub const __ARCH_WANT_SYS_CLONE: bool = true;
pub const __ARCH_WANT_MEMFD_SECRET: bool = true;

// `__NR_syscalls` is supplied by the included UAPI header.
pub const NR_syscalls: u32 = __NR_syscalls;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
