// SPDX-License-Identifier: GPL-2.0+ OR BSD-3-Clause
/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 * All rights reserved.
 *
 * This source code is licensed under both the BSD-style license (found in the
 * LICENSE file in the root directory of this source tree) and the GPLv2 (found
 * in the COPYING file in the root directory of this source tree).
 * You may select, at your option, one of the above-listed licenses.
 */

// C dependencies supplied by other translation units:
// <linux/module.h>
// "common/huf.h"
// "common/fse.h"
// "common/zstd_internal.h"

// Export symbols shared by compress and decompress into a common module.
// The following symbols are exported with EXPORT_SYMBOL_GPL in the C source:
// FSE_readNCount, HUF_readStats, HUF_readStats_wksp, ZSTD_isError,
// ZSTD_getErrorName, and ZSTD_getErrorCode.

// ZSTD_isError is defined within zstd_internal.h in the C source; its
// preprocessor definition was undefined before exporting the symbol.

// Kernel module metadata from the C source:
// MODULE_LICENSE("Dual BSD/GPL");
// MODULE_DESCRIPTION("Zstd Common");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
