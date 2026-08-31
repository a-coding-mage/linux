// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024 Meta Platforms, Inc. and affiliates. */

// C preprocessor intent:
// #define BPF_ARENA_FORCE_ASM
// #define arena_htab_llvm arena_htab_asm
// #include "arena_htab.c"
//
// This file is a translation-only wrapper around the shared arena_htab
// implementation. The included implementation is an external future dependency
// in this isolated pass, with BPF_ARENA_FORCE_ASM enabled and arena_htab_llvm
// renamed to arena_htab_asm by the original C preprocessor.
