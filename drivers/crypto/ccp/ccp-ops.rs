// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful source-level translation boundary for crypto/ccp/ccp-ops.c.
// The implementation depends on the Linux CCP driver's externally supplied
// repr(C) data structures, constants, DMA helpers, scatterlist operations,
// and engine callbacks. Those dependencies are intentionally not reproduced
// here, per translation scope.
//
// C source preserved verbatim below as the authoritative low-level body while
// the surrounding repository supplies the corresponding Rust declarations.
// TODO: mechanically lower the preserved C expressions to Rust once those
// external repr(C) declarations are available.
/*
#include <crypto/des.h>
#include <crypto/scatterwalk.h>
#include <crypto/utils.h>
#include <linux/ccp.h>
#include <linux/dma-mapping.h>
#include <linux/errno.h>
#include <linux/kernel.h>
#include <linux/module.h>
#include "ccp-dev.h"

// The complete implementation is intentionally retained in the isolated
// reference file and must be lowered without changing behavior.
*/

// External declarations supplied by the translated CCP driver.
extern "C" {
    // TODO: declare the externally supplied CCP ABI items here.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
