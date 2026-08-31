/* SPDX-License-Identifier: GPL-2.0
 * Copyright(c) 2020 Intel Corporation.
 */

/* C header dependencies:
 * <limits.h>
 * "xsk_xdp_progs.skel.h"
 * "xsk_xdp_common.h"
 */

/* Defined only when missing in C. */
pub const SOL_XDP: u32 = 283;

/* Defined only when missing in C. */
pub const AF_XDP: u32 = 44;

/* Defined only when missing in C. */
pub const PF_XDP: u32 = AF_XDP;

pub const MAX_TEARDOWN_ITER: u32 = 10;
pub const MAX_ETH_JUMBO_SIZE: u32 = 9000;
pub const SOCK_RECONF_CTR: u32 = 10;
pub const RX_FULL_RXQSIZE: u32 = 32;
pub const UMEM_HEADROOM_TEST_SIZE: u32 = 128;
pub const XSK_UMEM__INVALID_FRAME_SIZE: u32 = MAX_ETH_JUMBO_SIZE + 1;
pub const RUN_ALL_TESTS: u32 = u32::MAX;
pub const NUM_MAC_ADDRESSES: u32 = 4;
