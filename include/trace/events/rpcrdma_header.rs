/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (c) 2017, 2018 Oracle.  All rights reserved.
 *
 * Trace point definitions for the "rpcrdma" subsystem.
 */

// C dependencies supplied by other translation units:
// linux::scatterlist, linux::sunrpc::rpc_rdma_cid, linux::tracepoint,
// rdma::ib_cm, trace::misc::{rdma, sunrpc}.

// The original file consists of Linux tracepoint declarations.  These Rust
// declarations preserve their event-class names, arguments, entry layouts,
// assignments, and printk formats for the tracepoint implementation layer.

#[allow(non_camel_case_types, non_snake_case, dead_code)]
pub mod rpcrdma_trace {
    pub const DMA_BIDIRECTIONAL: u32 = 0;
    pub const DMA_TO_DEVICE: u32 = 1;
    pub const DMA_FROM_DEVICE: u32 = 2;
    pub const DMA_NONE: u32 = 3;

    #[repr(C)]
    pub struct rpcrdma_simple_cid_class {
        pub cq_id: u32,
        pub completion_id: i32,
    }

    #[repr(C)]
    pub struct rpcrdma_completion_class {
        pub cq_id: u32,
        pub completion_id: i32,
        pub status: usize,
        pub vendor_err: u32,
    }
    pub type rpcrdma_send_flush_class = rpcrdma_completion_class;
    pub type rpcrdma_mr_completion_class = rpcrdma_completion_class;
    pub type rpcrdma_receive_flush_class = rpcrdma_completion_class;

    #[repr(C)]
    pub struct rpcrdma_receive_completion_class {
        pub cq_id: u32,
        pub completion_id: i32,
        pub received: u32,
        pub status: usize,
        pub vendor_err: u32,
    }

    #[repr(C)]
    pub struct rpcrdma_receive_success_class {
        pub cq_id: u32,
        pub completion_id: i32,
        pub received: u32,
    }

    #[repr(C)]
    pub struct xprtrdma_reply_class {
        pub xid: u32,
        pub version: u32,
        pub proc: u32,
        pub addr: *const core::ffi::c_char,
        pub port: *const core::ffi::c_char,
    }

    #[repr(C)]
    pub struct xprtrdma_rxprt {
        pub addr: *const core::ffi::c_char,
        pub port: *const core::ffi::c_char,
    }

    #[repr(C)]
    pub struct xprtrdma_connect_class {
        pub rc: i32,
        pub connect_status: i32,
        pub addr: *const core::ffi::c_char,
        pub port: *const core::ffi::c_char,
    }

    #[repr(C)]
    pub struct xprtrdma_rdch_event {
        pub task_id: u32,
        pub client_id: u32,
        pub pos: u32,
        pub nents: i32,
        pub handle: u32,
        pub length: u32,
        pub offset: u64,
        pub is_last: bool,
    }

    #[repr(C)]
    pub struct xprtrdma_wrch_event {
        pub task_id: u32,
        pub client_id: u32,
        pub nents: i32,
        pub handle: u32,
        pub length: u32,
        pub offset: u64,
        pub is_last: bool,
    }

    #[repr(C)]
    pub struct xprtrdma_mr_class {
        pub task_id: u32,
        pub client_id: u32,
        pub mr_id: u32,
        pub nents: i32,
        pub handle: u32,
        pub length: u32,
        pub offset: u64,
        pub dir: u32,
    }
    pub type xprtrdma_anonymous_mr_class = xprtrdma_mr_class;

    #[repr(C)]
    pub struct xprtrdma_callback_class {
        pub xid: u32,
        pub addr: *const core::ffi::c_char,
        pub port: *const core::ffi::c_char,
    }

    #[repr(C)]
    pub struct xprtrdma_inline_thresh {
        pub inline_send: u32,
        pub inline_recv: u32,
        pub max_send: u32,
        pub max_recv: u32,
        pub srcaddr: [u8; 28],
        pub dstaddr: [u8; 28],
    }

    #[repr(C)]
    pub struct xprtrdma_device_removal {
        pub name: *const core::ffi::c_char,
        pub addr: [u8; 28],
    }

    #[repr(C)]
    pub struct xprtrdma_op_connect {
        pub delay: usize,
        pub addr: *const core::ffi::c_char,
        pub port: *const core::ffi::c_char,
    }

    // Preserve the source tracepoint/event declaration surface.
    macro_rules! DEFINE_SIMPLE_CID_EVENT { ($name:ident) => { pub type $name = rpcrdma_simple_cid_class; }; }
    macro_rules! DEFINE_COMPLETION_EVENT { ($name:ident) => { pub type $name = rpcrdma_completion_class; }; }
    macro_rules! DEFINE_SEND_FLUSH_EVENT { ($name:ident) => { pub type $name = rpcrdma_send_flush_class; }; }
    macro_rules! DEFINE_MR_COMPLETION_EVENT { ($name:ident) => { pub type $name = rpcrdma_mr_completion_class; }; }
    macro_rules! DEFINE_RECEIVE_COMPLETION_EVENT { ($name:ident) => { pub type $name = rpcrdma_receive_completion_class; }; }
    macro_rules! DEFINE_RECEIVE_SUCCESS_EVENT { ($name:ident) => { pub type $name = rpcrdma_receive_success_class; }; }
    macro_rules! DEFINE_RECEIVE_FLUSH_EVENT { ($name:ident) => { pub type $name = rpcrdma_receive_flush_class; }; }
    macro_rules! DEFINE_REPLY_EVENT { ($name:ident) => { pub type $name = xprtrdma_reply_class; }; }
    macro_rules! DEFINE_RXPRT_EVENT { ($name:ident) => { pub type $name = xprtrdma_rxprt; }; }
    macro_rules! DEFINE_CONN_EVENT { ($name:ident) => { pub type $name = xprtrdma_connect_class; }; }
    macro_rules! DEFINE_RDCH_EVENT { ($name:ident) => { pub type $name = xprtrdma_rdch_event; }; }
    macro_rules! DEFINE_WRCH_EVENT { ($name:ident) => { pub type $name = xprtrdma_wrch_event; }; }
    macro_rules! DEFINE_MR_EVENT { ($name:ident) => { pub type $name = xprtrdma_mr_class; }; }
    macro_rules! DEFINE_ANON_MR_EVENT { ($name:ident) => { pub type $name = xprtrdma_anonymous_mr_class; }; }
    macro_rules! DEFINE_CALLBACK_EVENT { ($name:ident) => { pub type $name = xprtrdma_callback_class; }; }

    pub fn xprtrdma_show_direction(x: u32) -> &'static str {
        match x {
            DMA_BIDIRECTIONAL => "BIDIR",
            DMA_TO_DEVICE => "TO_DEVICE",
            DMA_FROM_DEVICE => "FROM_DEVICE",
            DMA_NONE => "NONE",
            _ => "UNKNOWN",
        }
    }

    // DEFINE_CONN_EVENT(connect);
    // DEFINE_CONN_EVENT(disconnect);
    // DEFINE_RXPRT_EVENT(xprtrdma_op_inject_dsc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
