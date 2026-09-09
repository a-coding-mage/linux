/* SPDX-License-Identifier: GPL-2.0 */
/*
 * LoongArch specific ACPICA environments and implementation
 *
 * Author: Jianmin Lv <lvjianmin@loongson.cn>
 *         Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/*
 * C source condition:
 *
 * #ifdef CONFIG_ARCH_STRICT_ALIGN
 * #define ACPI_MISALIGNMENT_NOT_SUPPORTED
 * #endif
 *
 * The build-time CONFIG_ARCH_STRICT_ALIGN condition is preserved here.  The
 * marker is represented as a Rust configuration flag when that configuration
 * is enabled.
 */
#[cfg(CONFIG_ARCH_STRICT_ALIGN)]
pub const ACPI_MISALIGNMENT_NOT_SUPPORTED: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
