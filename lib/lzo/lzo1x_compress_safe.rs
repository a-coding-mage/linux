// SPDX-License-Identifier: GPL-2.0-only
/*
 *  LZO1X Compressor from LZO
 *
 *  Copyright (C) 1996-2012 Markus F.X.J. Oberhumer <markus@oberhumer.com>
 *
 *  The full LZO package can be found at:
 *  http://www.oberhumer.com/opensource/lzo/
 *
 *  Changed for Linux kernel use by:
 *  Nitin Gupta <nitingupta910@gmail.com>
 *  Richard Purdie <rpurdie@openedhand.com>
 */

// C preprocessor equivalent: #define LZO_SAFE(name) name##_safe
// Rust translation requires the corresponding safe-symbol naming at each use site.

// C preprocessor equivalent: #define HAVE_OP(x) ((size_t)(op_end - op) >= (size_t)(x))
// The macro depends on the pointer variables `op_end` and `op` from the included implementation.

// The original source includes "lzo1x_compress.c". Its implementation is an external dependency
// and is intentionally not copied or reimplemented here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
