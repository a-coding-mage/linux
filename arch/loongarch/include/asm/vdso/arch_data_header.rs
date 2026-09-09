/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/* The C header is excluded when assembling. */

#[repr(C)]
pub struct vdso_pcpu_data {
    pub node: u32,
}

#[repr(C)]
pub struct vdso_arch_data {
    pub pdata: [vdso_pcpu_data; NR_CPUS],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
