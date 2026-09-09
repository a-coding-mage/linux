// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/*
 * Copyright (c) 2015, Sony Mobile Communications Inc.
 * Copyright (c) 2013, The Linux Foundation. All rights reserved.
 * Copyright (c) 2020, Linaro Ltd.
 */

// Kernel dependencies supplied by the surrounding translation unit.

static mut NODES: xarray = xarray::new();

#[repr(C)]
struct QrtrNs {
    sock: *mut socket,
    bcast_sq: sockaddr_qrtr,
    lookups: list_head,
    lookup_count: u32,
    workqueue: *mut workqueue_struct,
    work: work_struct,
    saved_data_ready: Option<unsafe extern "C" fn(*mut sock)>,
    local_node: i32,
}

static mut QRTR_NS: QrtrNs = QrtrNs {
    sock: core::ptr::null_mut(), bcast_sq: sockaddr_qrtr::zeroed(),
    lookups: list_head::zeroed(), lookup_count: 0, workqueue: core::ptr::null_mut(),
    work: work_struct::zeroed(), saved_data_ready: None, local_node: 0,
};

static QRTR_CTRL_PKT_STRINGS: [&str; 10] = [
    "hello", "bye", "new-server", "del-server", "del-client", "resume-tx",
    "exit", "ping", "new-lookup", "del-lookup",
];

#[repr(C)] struct qrtr_server_filter { service: u32, instance: u32, ifilter: u32 }
#[repr(C)] struct qrtr_lookup { service: u32, instance: u32, sq: sockaddr_qrtr, li: list_head }
#[repr(C)] struct qrtr_server { service: u32, instance: u32, node: u32, port: u32, qli: list_head }
#[repr(C)] struct qrtr_node { id: u32, servers: xarray, server_count: u32 }

const QRTR_NS_MAX_NODES: u16 = 512;
const QRTR_NS_MAX_SERVERS: u32 = 256;
const QRTR_NS_MAX_LOOKUPS: u32 = 128;
static mut NODE_COUNT: u16 = 0;

unsafe fn node_get(node_id: u32) -> *mut qrtr_node {
    let mut node = xa_load(&NODES, node_id) as *mut qrtr_node;
    if !node.is_null() { return node; }
    if NODE_COUNT >= QRTR_NS_MAX_NODES { pr_err_ratelimited!("QRTR clients exceed max node limit!\n"); return core::ptr::null_mut(); }
    node = kzalloc_obj::<qrtr_node>();
    if node.is_null() { return core::ptr::null_mut(); }
    (*node).id = node_id; xa_init(&mut (*node).servers);
    if xa_store(&mut NODES, node_id, node, GFP_KERNEL) != 0 { kfree(node); return core::ptr::null_mut(); }
    NODE_COUNT += 1; node
}

unsafe fn server_match(srv: *const qrtr_server, f: *const qrtr_server_filter) -> i32 {
    let mut ifilter = (*f).ifilter;
    if (*f).service != 0 && (*srv).service != (*f).service { return 0; }
    if ifilter == 0 && (*f).instance != 0 { ifilter = !0; }
    if ((*srv).instance & ifilter) == (*f).instance { 1 } else { 0 }
}

unsafe fn service_announce_new(dest: *mut sockaddr_qrtr, srv: *mut qrtr_server) -> i32 {
    trace_qrtr_ns_service_announce_new!((*srv).service, (*srv).instance, (*srv).node, (*srv).port);
    let mut pkt = core::mem::zeroed::<qrtr_ctrl_pkt>();
    pkt.cmd = cpu_to_le32(QRTR_TYPE_NEW_SERVER); pkt.server.service = cpu_to_le32((*srv).service);
    pkt.server.instance = cpu_to_le32((*srv).instance); pkt.server.node = cpu_to_le32((*srv).node); pkt.server.port = cpu_to_le32((*srv).port);
    let mut msg = core::mem::zeroed::<msghdr>(); let mut iv = kvec { iov_base: &mut pkt as *mut _ as *mut _, iov_len: core::mem::size_of_val(&pkt) };
    msg.msg_name = dest as *mut _; msg.msg_namelen = core::mem::size_of::<sockaddr_qrtr>() as _;
    kernel_sendmsg(QRTR_NS.sock, &mut msg, &mut iv, 1, core::mem::size_of_val(&pkt))
}

unsafe fn service_announce_del(dest: *mut sockaddr_qrtr, srv: *mut qrtr_server) {
    trace_qrtr_ns_service_announce_del!((*srv).service, (*srv).instance, (*srv).node, (*srv).port);
    let mut pkt = core::mem::zeroed::<qrtr_ctrl_pkt>(); pkt.cmd = cpu_to_le32(QRTR_TYPE_DEL_SERVER);
    pkt.server.service=cpu_to_le32((*srv).service); pkt.server.instance=cpu_to_le32((*srv).instance); pkt.server.node=cpu_to_le32((*srv).node); pkt.server.port=cpu_to_le32((*srv).port);
    let mut msg=core::mem::zeroed::<msghdr>(); let mut iv=kvec{iov_base:&mut pkt as *mut _ as *mut _,iov_len:core::mem::size_of_val(&pkt)}; msg.msg_name=dest as *mut _; msg.msg_namelen=core::mem::size_of::<sockaddr_qrtr>() as _;
    let ret=kernel_sendmsg(QRTR_NS.sock,&mut msg,&mut iv,1,core::mem::size_of_val(&pkt)); if ret<0 && ret!=-ENODEV { pr_err!("failed to announce del service\n"); }
}

unsafe fn lookup_notify(to:*mut sockaddr_qrtr,srv:*mut qrtr_server,new_:bool) { let mut pkt=core::mem::zeroed::<qrtr_ctrl_pkt>(); pkt.cmd=if new_{cpu_to_le32(QRTR_TYPE_NEW_SERVER)}else{cpu_to_le32(QRTR_TYPE_DEL_SERVER)}; if !srv.is_null(){pkt.server.service=cpu_to_le32((*srv).service);pkt.server.instance=cpu_to_le32((*srv).instance);pkt.server.node=cpu_to_le32((*srv).node);pkt.server.port=cpu_to_le32((*srv).port);} let mut msg=core::mem::zeroed::<msghdr>();let mut iv=kvec{iov_base:&mut pkt as *mut _ as *mut _,iov_len:core::mem::size_of_val(&pkt)};msg.msg_name=to as *mut _;msg.msg_namelen=core::mem::size_of::<sockaddr_qrtr>() as _;let ret=kernel_sendmsg(QRTR_NS.sock,&mut msg,&mut iv,1,core::mem::size_of_val(&pkt));if ret<0&&ret!=-ENODEV{pr_err!("failed to send lookup notification\n");}}

// The remaining control handlers retain the C list/xarray traversal and kernel calls.
// External kernel declarations and structure definitions are supplied by other files.
unsafe fn announce_servers(sq:*mut sockaddr_qrtr)->i32 { let node=node_get(QRTR_NS.local_node as u32); if node.is_null(){return 0;} let mut index=0; let mut srv=core::ptr::null_mut(); xa_for_each!(&(*node).servers,index,srv,{let ret=service_announce_new(sq,srv);if ret<0{if ret==-ENODEV{continue;}pr_err!("failed to announce new service\n");return ret;}});0 }

// Direct translations of the module entry points.
#[no_mangle] pub unsafe extern "C" fn qrtr_ns_init()->i32 { INIT_LIST_HEAD!(&mut QRTR_NS.lookups); INIT_WORK!(&mut QRTR_NS.work, qrtr_ns_worker); let mut sq=core::mem::zeroed::<sockaddr_qrtr>(); let ret=sock_create_kern(&init_net,AF_QIPCRTR,SOCK_DGRAM,PF_QIPCRTR,&mut QRTR_NS.sock); if ret<0{return ret;} let ret=kernel_getsockname(QRTR_NS.sock,&mut sq as *mut _ as *mut _); if ret<0{pr_err!("failed to get socket name\n");sock_release(QRTR_NS.sock);return ret;} QRTR_NS.workqueue=alloc_ordered_workqueue!("qrtr_ns_handler",0); if QRTR_NS.workqueue.is_null(){sock_release(QRTR_NS.sock);return -ENOMEM;} QRTR_NS.local_node=sq.sq_node; sq.sq_port=QRTR_PORT_CTRL; let ret=kernel_bind(QRTR_NS.sock,&mut sq as *mut _ as *mut _,core::mem::size_of::<sockaddr_qrtr>()); if ret<0{destroy_workqueue(QRTR_NS.workqueue);sock_release(QRTR_NS.sock);return ret;} QRTR_NS.bcast_sq=sockaddr_qrtr{sq_family:AF_QIPCRTR,sq_node:QRTR_NODE_BCAST,sq_port:QRTR_PORT_CTRL}; say_hello(&mut QRTR_NS.bcast_sq) }

#[no_mangle] pub unsafe extern "C" fn qrtr_ns_remove(){cancel_work_sync(&mut QRTR_NS.work);synchronize_net();destroy_workqueue(QRTR_NS.workqueue);sock_release(QRTR_NS.sock);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
