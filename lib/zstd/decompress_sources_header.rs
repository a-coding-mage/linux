/* SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause */
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */

/*
 * This file includes every .c file needed for decompression.
 * It is used by lib/decompress_unzstd.c to include the decompression
 * source into the translation-unit, so it can be used for kernel
 * decompression.
 *
 * The corresponding C source inclusions are preserved as dependency intent:
 * common/debug.c
 * common/entropy_common.c
 * common/error_private.c
 * common/fse_decompress.c
 * common/zstd_common.c
 * decompress/huf_decompress.c
 * decompress/zstd_ddict.c
 * decompress/zstd_decompress.c
 * decompress/zstd_decompress_block.c
 * zstd_decompress_module.c
 */

/*
 * Disable the ASM Huffman implementation because we need to
 * include all the sources.
 */
pub const ZSTD_DISABLE_ASM: i32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
