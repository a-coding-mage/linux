/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the corresponding generic and userspace headers:
// <asm-generic/int-ll64.h>
// <uapi/asm/types.h>

// In assembler builds, the C macros expand to nothing:
// #define _ULCAST_
// #define _U64CAST_
// In non-assembler builds, they expand to `(unsigned long)` and `(u64)`:
// #define _ULCAST_ (unsigned long)
// #define _U64CAST_ (u64)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
