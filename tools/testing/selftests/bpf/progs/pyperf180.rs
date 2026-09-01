// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

pub const STACK_MAX_LEN: i32 = 180;

/* llvm upstream commit at clang18
 *   https://github.com/llvm/llvm-project/commit/1a2e77cf9e11dbf56b5720c607313a566eebb16e
 * changed inlining behavior and caused compilation failure as some branch
 * target distance exceeded 16bit representation which is the maximum for
 * cpu v1/v2/v3. Macro __BPF_CPU_VERSION__ is later implemented in clang18
 * to specify which cpu version is used for compilation. So a smaller
 * unroll_count can be set if __BPF_CPU_VERSION__ is less than 4, which
 * reduced some branch target distances and resolved the compilation failure.
 *
 * To capture the case where a developer/ci uses clang18 but the corresponding
 * repo checkpoint does not have __BPF_CPU_VERSION__, a smaller unroll_count
 * will be set as well to prevent potential compilation failures.
 */
// C preprocessor equivalent:
//   if defined(__BPF_CPU_VERSION__) && __BPF_CPU_VERSION__ < 4 {
//       #define UNROLL_COUNT 90
//   } else if !defined(__BPF_CPU_VERSION__) && __clang_major__ == 18 {
//       #define UNROLL_COUNT 90
//   }
// Rust has no direct file-local equivalent for these C compiler predefined
// macros without build-script configuration; preserve the intended value for
// affected builds here.
pub const UNROLL_COUNT: i32 = 90;

// Dependency intent from C:
//   #include "pyperf.h"

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
