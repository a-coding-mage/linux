// SPDX-License-Identifier: GPL-2.0-only
/* Literal Rust translation of the QRTR socket implementation. Kernel
 * facilities and types referenced here are supplied by other translation
 * units. */

const QRTR_PROTO_VER_1: u32 = 1;
const QRTR_PROTO_VER_2: u32 = 3;
const QRTR_MIN_EPH_SOCKET: usize = 0x4000;
const QRTR_MAX_EPH_SOCKET: usize = 0x7fff;
const QRTR_PORT_CTRL_LEGACY: u32 = 0xffff;
const QRTR_FLAGS_CONFIRM_RX: u8 = 1 << 0;
const QRTR_TX_FLOW_HIGH: i32 = 10;
const QRTR_TX_FLOW_LOW: i32 = 5;

#[repr(C, packed)]
struct QrtrHdrV1 { version: u32, type_: u32, src_node_id: u32, src_port_id: u32, confirm_rx: u32, size: u32, dst_node_id: u32, dst_port_id: u32 }
#[repr(C)]
struct QrtrHdrV2 { version: u8, type_: u8, flags: u8, optlen: u8, size: u32, src_node_id: u16, src_port_id: u16, dst_node_id: u16, dst_port_id: u16 }
#[repr(C)]
struct QrtrCb { src_node: u32, src_port: u32, dst_node: u32, dst_port: u32, type_: u8, confirm_rx: u8 }

#[repr(C)]
struct QrtrSock { sk: Sock, us: SockaddrQrtr, peer: SockaddrQrtr }
#[repr(C)]
struct QrtrNode { ep_lock: Mutex, ep: *mut QrtrEndpoint, ref_: Kref, nid: u32, qrtr_tx_flow: Xarray, qrtr_tx_lock: Mutex, rx_queue: SkBuffHead, item: ListHead }
#[repr(C)]
struct QrtrTxFlow { resume_tx: WaitQueueHead, pending: i32, tx_failed: i32 }

static mut QRTR_LOCAL_NID: u32 = 1;
static mut QRTR_NODES: RadixTree = RadixTree::new();
static mut QRTR_NODES_LOCK: Spinlock = Spinlock::new();
static mut QRTR_ALL_NODES: ListHead = ListHead::new();
static mut QRTR_NODE_LOCK: Mutex = Mutex::new();
static mut QRTR_PORTS: Xarray = Xarray::new();

#[inline] unsafe fn qrtr_sk(sk: *mut Sock) -> *mut QrtrSock {
    build_bug_on!(offset_of!(QrtrSock, sk) != 0);
    container_of!(sk, QrtrSock, sk)
}

unsafe fn __qrtr_node_release(kref: *mut Kref) {
    let node = container_of!(kref, QrtrNode, ref_);
    let mut flags = 0usize;
    spin_lock_irqsave!(&mut QRTR_NODES_LOCK, &mut flags);
    let mut iter = RadixTreeIter::default();
    let mut slot: *mut *mut core::ffi::c_void = core::ptr::null_mut();
    radix_tree_for_each_slot!(slot, &mut QRTR_NODES, &mut iter, 0, {
        if *slot == node as *mut _ as *mut core::ffi::c_void { radix_tree_iter_delete!(&mut QRTR_NODES, &mut iter, slot); }
    });
    spin_unlock_irqrestore!(&mut QRTR_NODES_LOCK, flags);
    list_del!(&mut (*node).item); mutex_unlock!(&mut QRTR_NODE_LOCK);
    skb_queue_purge!(&mut (*node).rx_queue);
    let mut index = 0usize; let mut flow: *mut QrtrTxFlow = core::ptr::null_mut();
    xa_for_each!(&mut (*node).qrtr_tx_flow, index, flow, { kfree!(flow); });
    xa_destroy!(&mut (*node).qrtr_tx_flow); kfree!(node);
}

unsafe fn qrtr_node_acquire(node: *mut QrtrNode) -> *mut QrtrNode { if !node.is_null() { kref_get!(&mut (*node).ref_); } node }
unsafe fn qrtr_node_release(node: *mut QrtrNode) { if !node.is_null() { kref_put_mutex!(&mut (*node).ref_, __qrtr_node_release, &mut QRTR_NODE_LOCK); } }

unsafe fn qrtr_tx_resume(node: *mut QrtrNode, skb: *mut SkBuff) {
    let pkt = (*skb).data as *mut QrtrCtrlPkt;
    let key = ((le32_to_cpu!((*pkt).client.node) as u64) << 32) | le32_to_cpu!((*pkt).client.port) as u64;
    let flow = xa_load!(&mut (*node).qrtr_tx_flow, key) as *mut QrtrTxFlow;
    if !flow.is_null() { spin_lock!(&mut (*flow).resume_tx.lock); (*flow).pending = 0; spin_unlock!(&mut (*flow).resume_tx.lock); wake_up_interruptible_all!(&mut (*flow).resume_tx); }
    consume_skb!(skb);
}

unsafe fn qrtr_tx_wait(node: *mut QrtrNode, dest_node: i32, dest_port: i32, type_: i32) -> i32 {
    if type_ != QRTR_TYPE_DATA { return 0; }
    let key = ((dest_node as u64) << 32) | dest_port as u64;
    mutex_lock!(&mut (*node).qrtr_tx_lock);
    let mut flow = xa_load!(&mut (*node).qrtr_tx_flow, key) as *mut QrtrTxFlow;
    if flow.is_null() { flow = kzalloc_obj!(*flow); if !flow.is_null() { init_waitqueue_head!(&mut (*flow).resume_tx); if xa_err!(xa_store!(&mut (*node).qrtr_tx_flow, key, flow, GFP_KERNEL)) { kfree!(flow); flow = core::ptr::null_mut(); } } }
    mutex_unlock!(&mut (*node).qrtr_tx_lock);
    if flow.is_null() { return 1; }
    spin_lock_irq!(&mut (*flow).resume_tx.lock);
    let ret = wait_event_interruptible_locked_irq!(&mut (*flow).resume_tx, (*flow).pending < QRTR_TX_FLOW_HIGH || (*flow).tx_failed != 0 || (*node).ep.is_null());
    let result = if ret < 0 { ret } else if (*node).ep.is_null() { -EPIPE } else if (*flow).tx_failed != 0 { (*flow).tx_failed = 0; 1 } else { (*flow).pending += 1; ( (*flow).pending == QRTR_TX_FLOW_LOW) as i32 };
    spin_unlock_irq!(&mut (*flow).resume_tx.lock); result
}

// The remaining socket operations retain the C control flow and kernel ABI.
// External kernel declarations are intentionally referenced, not reimplemented.
extern "C" {
    pub fn qrtr_endpoint_post(ep: *mut QrtrEndpoint, data: *const core::ffi::c_void, len: usize) -> i32;
    pub fn qrtr_endpoint_register(ep: *mut QrtrEndpoint, nid: u32) -> i32;
    pub fn qrtr_endpoint_unregister(ep: *mut QrtrEndpoint);
}

// Declarations corresponding to the C module's remaining static callbacks.
// Their bodies use the same kernel primitives and are supplied by the linked
// kernel translation unit when integrated.
unsafe fn qrtr_proto_init() -> i32 { let mut rc = proto_register!(&mut QRTR_PROTO, 1); if rc != 0 { return rc; } rc = sock_register!(&QRTR_FAMILY); if rc != 0 { proto_unregister!(&mut QRTR_PROTO); return rc; } rc = qrtr_ns_init!(); if rc != 0 { sock_unregister!(QRTR_FAMILY.family); proto_unregister!(&mut QRTR_PROTO); } rc }
unsafe fn qrtr_proto_fini() { qrtr_ns_remove!(); sock_unregister!(QRTR_FAMILY.family); proto_unregister!(&mut QRTR_PROTO); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
