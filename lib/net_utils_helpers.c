// SPDX-License-Identifier: GPL-2.0
/*
 * Compile-time ties for the Rust mac_pton() owner in lib/net_utils.rs,
 * which spells the ctype classes and string-length constants it uses.
 */
#include <linux/build_bug.h>
#include <linux/ctype.h>
#include <linux/if_ether.h>

static_assert(_D == 0x04);
static_assert(_X == 0x40);
static_assert(MAC_ADDR_STR_LEN == 3 * ETH_ALEN - 1);
