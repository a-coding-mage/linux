/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Author: Huacai Chen <chenhuacai@loongson.cn>
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// The C header guard `_ASM_VDSO_VDSO_H` has no direct Rust equivalent.

// The declarations below correspond to the C header's non-assembler section.
// Dependencies supplied by the surrounding build provide `VDSO_NR_PAGES`,
// `PAGE_SHIFT`, and the architecture-specific definitions included by C.

pub const VVAR_SIZE: usize = VDSO_NR_PAGES << PAGE_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
