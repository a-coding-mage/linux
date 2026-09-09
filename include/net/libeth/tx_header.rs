/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2024-2025 Intel Corporation */

// Translated from tx.h. Types and functions supplied by the included kernel
// and libeth headers remain external dependencies.

/* Tx buffer completion */

#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libeth_sqe_type {
    LIBETH_SQE_EMPTY = 0,
    LIBETH_SQE_CTX,
    LIBETH_SQE_SLAB,
    LIBETH_SQE_FRAG,
    LIBETH_SQE_SKB,

    __LIBETH_SQE_XDP_START,
    LIBETH_SQE_XDP_TX = __LIBETH_SQE_XDP_START,
    LIBETH_SQE_XDP_XMIT,
    LIBETH_SQE_XDP_XMIT_FRAG,
    LIBETH_SQE_XSK_TX,
    LIBETH_SQE_XSK_TX_FRAG,
}

#[repr(C, align(8))]
pub struct libeth_sqe {
    pub type_: libeth_sqe_type,
    pub rs_idx: u32,

    pub ptr: libeth_sqe_ptr,

    // DEFINE_DMA_UNMAP_ADDR(dma);
    pub dma: u64,
    // DEFINE_DMA_UNMAP_LEN(len);
    pub len: usize,

    pub nr_frags: u32,
    pub packets: u32,
    pub bytes: u32,

    pub priv_: usize,
}

#[repr(C)]
pub union libeth_sqe_ptr {
    pub raw: *mut core::ffi::c_void,
    pub skb: *mut sk_buff,
    pub sinfo: *mut skb_shared_info,
    pub xdpf: *mut xdp_frame,
    pub xsk: *mut libeth_xdp_buff,
}

// LIBETH_SQE_CHECK_PRIV(p): static_assert(sizeof(p) <= sizeof_field(struct libeth_sqe, priv))

#[repr(C)]
pub struct libeth_cq_pp {
    pub dev: *mut device,
    pub bq: *mut xdp_frame_bulk,

    pub stats: libeth_cq_pp_stats,
    pub xdp_tx: u32,

    pub napi: bool,
}

#[repr(C)]
pub union libeth_cq_pp_stats {
    pub ss: *mut libeth_sq_napi_stats,
    pub xss: *mut libeth_xdpsq_napi_stats,
}

pub unsafe fn libeth_tx_complete(sqe: *mut libeth_sqe, cp: *const libeth_cq_pp) {
    match (*sqe).type_ {
        libeth_sqe_type::LIBETH_SQE_EMPTY => return,
        libeth_sqe_type::LIBETH_SQE_SKB
        | libeth_sqe_type::LIBETH_SQE_FRAG
        | libeth_sqe_type::LIBETH_SQE_SLAB => {
            dma_unmap_page(
                (*cp).dev,
                (*sqe).dma,
                (*sqe).len,
                DMA_TO_DEVICE,
            );
        }
        _ => {}
    }

    match (*sqe).type_ {
        libeth_sqe_type::LIBETH_SQE_SKB => {
            (*(*cp).stats.ss).packets += (*sqe).packets;
            (*(*cp).stats.ss).bytes += (*sqe).bytes;
            napi_consume_skb((*sqe).ptr.skb, (*cp).napi);
        }
        libeth_sqe_type::LIBETH_SQE_SLAB => {
            kfree((*sqe).ptr.raw);
        }
        _ => {}
    }

    (*sqe).type_ = libeth_sqe_type::LIBETH_SQE_EMPTY;
}

extern "C" {
    pub fn libeth_tx_complete_any(sqe: *mut libeth_sqe, cp: *mut libeth_cq_pp);

    fn dma_unmap_page(dev: *mut device, addr: u64, len: usize, direction: u32);
    fn napi_consume_skb(skb: *mut sk_buff, budget: bool);
    fn kfree(ptr: *mut core::ffi::c_void);
}

// External types and constants from <linux/skbuff.h> and <net/libeth/types.h>.
#[allow(non_camel_case_types)]
pub enum device {}
#[allow(non_camel_case_types)]
pub enum sk_buff {}
#[allow(non_camel_case_types)]
pub enum skb_shared_info {}
#[allow(non_camel_case_types)]
pub enum xdp_frame {}
#[allow(non_camel_case_types)]
pub enum libeth_xdp_buff {}
#[allow(non_camel_case_types)]
pub enum xdp_frame_bulk {}
#[allow(non_camel_case_types)]
pub struct libeth_sq_napi_stats {
    pub packets: u32,
    pub bytes: u32,
}
#[allow(non_camel_case_types)]
pub enum libeth_xdpsq_napi_stats {}

pub const DMA_TO_DEVICE: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
