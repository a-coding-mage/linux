/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Copyright (c) Meta Platforms, Inc. and affiliates. */

// Dependencies supplied by the surrounding translation unit:
// linux/types.h, linux/compiler.h, and asm/orc_hash.h.

/*
 * The header is currently a 20-byte hash of the ORC entry definition; see
 * scripts/orc_hash.sh.
 *
 * ORC_HASH is a build-time macro from asm/orc_hash.h. Rust has no direct
 * equivalent for macro token insertion here, so the array contents remain
 * supplied by that dependency when this translation is integrated.
 */
#[used]
#[link_section = ".orc_header"]
#[repr(align(4))]
static orc_header: [u8; 0] = [];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
