// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook
pub const STACK_MAX_LEN: i32 = 600;
/* Full unroll of 600 iterations will have total
 * program size close to 298k insns and this may
 * cause BPF_JMP insn out of 16-bit integer range.
 * So limit the unroll size to 150 so the
 * total program size is around 80k insns but
 * the loop will still execute 600 times.
 */
pub const UNROLL_COUNT: i32 = 150;
// Depends on declarations and definitions from "pyperf.h".
