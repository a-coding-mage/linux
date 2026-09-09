/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Author: Hanlu Li <lihanlu@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/* Location of VDSO image. */
pub const AT_SYSINFO_EHDR: i32 = 33;

pub const AT_VECTOR_SIZE_ARCH: i32 = 1; /* entries in ARCH_DLINFO */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
