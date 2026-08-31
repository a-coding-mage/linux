// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// C header guard removed: TOOLS_ARCH_HEXAGON_UAPI_ASM_MMAN_FIX_H.
// Depends on definitions from <uapi/asm-generic/mman.h>.

// MAP_32BIT is undefined on hexagon, fix it for perf
pub const MAP_32BIT: i32 = 0;
