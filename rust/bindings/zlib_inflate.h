/* SPDX-License-Identifier: GPL-2.0-only */
/* Declaration-only input using this kernel's configured native layouts. */
#ifndef _RUST_BINDINGS_ZLIB_INFLATE_H
#define _RUST_BINDINGS_ZLIB_INFLATE_H

#include <linux/errno.h>
#include <linux/gfp.h>
#include <linux/zlib.h>
#include "../../lib/zlib_inflate/inftrees.h"
#include "../../lib/zlib_inflate/inflate.h"
#include "../../lib/zlib_inflate/infutil.h"
#include "../../lib/zlib_inflate/kernel_alloc.h"

#ifdef CONFIG_ZLIB_DFLTCC
#error "Rust kernel inflate does not yet implement the DFLTCC hooks"
#endif
#ifdef ASMINF
#error "Rust kernel inflate does not yet integrate an ASMINF assembly provider"
#endif
/* These are not selected by this tree's original zlib Makefile. Do not let
 * externally supplied C defines silently disagree with the Rust decoder. */
#if defined(INFLATE_STRICT) || defined(PKZIP_BUG_WORKAROUND)
#error "Rust kernel inflate does not yet map external zlib C preprocessor options"
#endif

const gfp_t RUST_ZLIB_GFP_KERNEL = GFP_KERNEL;

#endif
