/*
 * This file is part of the Chelsio T6 Crypto driver for Linux.
 *
 * Copyright (c) 2003-2016 Chelsio Communications, Inc. All rights reserved.
 *
 * This software is available under a choice of one of two
 * licenses.  You may choose to be licensed under the terms of the GNU
 * General Public License (GPL) Version 2, available from the file
 * COPYING in the main directory of this source tree, or the
 * OpenIB.org BSD license below:
 *
 *     Redistribution and use in source and binary forms, with or
 *     without modification, are permitted provided that the following
 *     conditions are met:
 *
 *      - Redistributions of source code must retain the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer.
 *
 *      - Redistributions in binary form must reproduce the above
 *        copyright notice, this list of conditions and the following
 *        disclaimer in the documentation and/or other materials
 *        provided with the distribution.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
 * NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
 * BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
 * ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

// C dependencies supplied by other translation units:
// crypto/algapi.h, net/tls.h, t4_hw.h, cxgb4.h, t4_msg.h, cxgb4_uld.h

pub const DRV_MODULE_NAME: &str = "chcr";
pub const DRV_DESC: &str = "Chelsio T6 Crypto Co-processor Driver";

pub const MAX_PENDING_REQ_TO_HW: u32 = 20;
pub const CHCR_TEST_RESPONSE_TIMEOUT: u32 = 1000;
// WQ_DETACH_TM = msecs_to_jiffies(50), dependent on the kernel build.
pub const PAD_ERROR_BIT: u32 = 1;
#[inline]
pub const fn CHK_PAD_ERR_BIT(x: u64) -> u64 { (x >> PAD_ERROR_BIT) & 1 }

pub const MAC_ERROR_BIT: u32 = 0;
#[inline]
pub const fn CHK_MAC_ERR_BIT(x: u64) -> u64 { (x >> MAC_ERROR_BIT) & 1 }
pub const MAX_SALT: usize = 4;
// CIP_WR_MIN_LEN = sizeof(struct chcr_wr) + sizeof(struct cpl_rx_phys_dsgl)
//                   + sizeof(struct ulptx_sgl) + 16 (IV).
// HASH_WR_MIN_LEN = sizeof(struct chcr_wr) + DUMMY_BYTES + sizeof(struct ulptx_sgl).

#[repr(C)]
pub struct _key_ctx {
    pub ctx_hdr: __be32,
    pub salt: [u8; MAX_SALT],
    pub iv_to_auth: __be64,
    pub key: [u8; 0],
}

pub const WQ_RETRY: u32 = 5;

#[repr(C)]
pub struct chcr_driver_data {
    pub act_dev: list_head,
    pub inact_dev: list_head,
    pub dev_count: atomic_t,
    pub drv_mutex: mutex,
    pub last_dev: *mut uld_ctx,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum chcr_state {
    CHCR_INIT = 0,
    CHCR_ATTACH,
    CHCR_DETACH,
}

#[repr(C)]
pub struct chcr_wr {
    pub wreq: fw_crypto_lookaside_wr,
    pub ulptx: ulp_txpkt,
    pub sc_imm: ulptx_idata,
    pub sec_cpl: cpl_tx_sec_pdu,
    pub key_ctx: _key_ctx,
}

#[repr(C)]
pub struct chcr_dev {
    pub lock_chcr_dev: spinlock_t,
    pub state: chcr_state,
    pub inflight: atomic_t,
    pub wqretry: i32,
    pub detach_work: delayed_work,
    pub detach_comp: completion,
}

#[repr(C)]
pub struct uld_ctx {
    pub entry: list_head,
    pub lldi: cxgb4_lld_info,
    pub dev: chcr_dev,
}

/*
 *      sgl_len - calculates the size of an SGL of the given capacity
 *      @n: the number of SGL entries
 *      Calculates the number of flits needed for a scatter/gather list that
 *      can hold the given number of entries.
 */
#[inline]
pub fn sgl_len(mut n: u32) -> u32 {
    n -= 1;
    (3 * n) / 2 + (n & 1) + 2
}

#[inline]
pub unsafe fn padap(dev: *mut chcr_dev) -> *mut core::ffi::c_void {
    let u_ctx = container_of!(dev, uld_ctx, dev);
    pci_get_drvdata((*u_ctx).lldi.pdev)
}

extern "C" {
    pub fn assign_chcr_device() -> *mut uld_ctx;
    pub fn chcr_send_wr(skb: *mut sk_buff) -> i32;
    pub fn start_crypto() -> i32;
    pub fn stop_crypto() -> i32;
    pub fn chcr_uld_rx_handler(handle: *mut core::ffi::c_void, rsp: *const __be64, pgl: *const pkt_gl) -> i32;
    pub fn chcr_handle_resp(req: *mut crypto_async_request, input: *mut u8, err: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
