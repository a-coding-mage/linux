// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2020-2024 Intel Corporation
 */

// Kernel and project headers from the C translation unit provide the external
// types, constants, functions, and synchronization primitives used below.

const IPC_MAX_RX_MSG: u32 = 128;

#[repr(C)]
pub struct IvpuIpcTxBuf {
    pub ipc: IvpuIpcHdr,
    pub jsm: VpuJsmMsg,
}

unsafe fn ivpu_ipc_msg_dump(vdev: *mut IvpuDevice, c: *mut core::ffi::c_char,
                            ipc_hdr: *mut IvpuIpcHdr, vpu_addr: u32) {
    ivpu_dbg(vdev, IPC, c, vpu_addr, (*ipc_hdr).data_addr, (*ipc_hdr).data_size,
             (*ipc_hdr).channel, (*ipc_hdr).src_node, (*ipc_hdr).dst_node,
             (*ipc_hdr).status);
}

unsafe fn ivpu_jsm_msg_dump(vdev: *mut IvpuDevice, c: *mut core::ffi::c_char,
                            jsm_msg: *mut VpuJsmMsg, vpu_addr: u32) {
    let payload = &(*jsm_msg).payload as *const _ as *const u32;
    ivpu_dbg(vdev, JSM, c, vpu_addr, ivpu_jsm_msg_type_to_str((*jsm_msg).type_),
             (*jsm_msg).status, (*jsm_msg).request_id, (*jsm_msg).result,
             *payload.add(0), *payload.add(1), *payload.add(2), *payload.add(3),
             *payload.add(4));
}

unsafe fn ivpu_ipc_rx_mark_free(vdev: *mut IvpuDevice, ipc_hdr: *mut IvpuIpcHdr,
                                jsm_msg: *mut VpuJsmMsg) {
    let _ = vdev;
    (*ipc_hdr).status = IVPU_IPC_HDR_FREE;
    if !jsm_msg.is_null() { (*jsm_msg).status = VPU_JSM_MSG_FREE; }
    wmb();
}

unsafe fn ivpu_ipc_mem_fini(vdev: *mut IvpuDevice) {
    let ipc = (*vdev).ipc;
    ivpu_bo_free((*ipc).mem_rx);
    ivpu_bo_free((*ipc).mem_tx);
}

unsafe fn ivpu_ipc_tx_prepare(vdev: *mut IvpuDevice, cons: *mut IvpuIpcConsumer,
                              req: *mut VpuJsmMsg) -> i32 {
    let ipc = (*vdev).ipc;
    let tx_buf_vpu_addr = gen_pool_alloc((*ipc).mm_tx, core::mem::size_of::<IvpuIpcTxBuf>());
    if tx_buf_vpu_addr == 0 { ivpu_err_ratelimited(vdev, core::mem::size_of::<IvpuIpcTxBuf>()); return -ENOMEM; }
    let tx_buf = ivpu_to_cpu_addr((*ipc).mem_tx, tx_buf_vpu_addr) as *mut IvpuIpcTxBuf;
    if drm_WARN_ON(&mut (*vdev).drm, tx_buf.is_null()) {
        gen_pool_free((*ipc).mm_tx, tx_buf_vpu_addr, core::mem::size_of::<IvpuIpcTxBuf>()); return -EIO;
    }
    let jsm_vpu_addr = tx_buf_vpu_addr + core::mem::offset_of!(IvpuIpcTxBuf, jsm) as u32;
    if (*tx_buf).ipc.status != IVPU_IPC_HDR_FREE { ivpu_warn_ratelimited(vdev, tx_buf_vpu_addr); }
    if (*tx_buf).jsm.status != VPU_JSM_MSG_FREE { ivpu_warn_ratelimited(vdev, jsm_vpu_addr); }
    core::ptr::write_bytes(tx_buf as *mut u8, 0, core::mem::size_of::<IvpuIpcTxBuf>());
    (*tx_buf).ipc.data_addr = jsm_vpu_addr;
    (*tx_buf).ipc.data_size = core::mem::size_of_val(&*req) as u32;
    (*tx_buf).ipc.channel = (*cons).channel;
    (*tx_buf).ipc.src_node = 0; (*tx_buf).ipc.dst_node = 1;
    (*tx_buf).ipc.status = IVPU_IPC_HDR_ALLOCATED;
    (*tx_buf).jsm.type_ = (*req).type_;
    (*tx_buf).jsm.status = VPU_JSM_MSG_ALLOCATED;
    (*tx_buf).jsm.payload = (*req).payload;
    (*req).request_id = atomic_inc_return(&mut (*ipc).request_id);
    (*tx_buf).jsm.request_id = (*req).request_id;
    (*cons).request_id = (*req).request_id;
    wmb(); (*cons).tx_vpu_addr = tx_buf_vpu_addr;
    ivpu_jsm_msg_dump(vdev, "TX".as_ptr() as *mut _, &mut (*tx_buf).jsm, jsm_vpu_addr);
    ivpu_ipc_msg_dump(vdev, "TX".as_ptr() as *mut _, &mut (*tx_buf).ipc, tx_buf_vpu_addr);
    0
}

unsafe fn ivpu_ipc_tx_release(vdev: *mut IvpuDevice, vpu_addr: u32) {
    if vpu_addr != 0 { gen_pool_free((*(*vdev).ipc).mm_tx, vpu_addr, core::mem::size_of::<IvpuIpcTxBuf>()); }
}

unsafe fn ivpu_ipc_tx(vdev: *mut IvpuDevice, vpu_addr: u32) { ivpu_hw_ipc_tx_set(vdev, vpu_addr); }

unsafe fn ivpu_ipc_rx_msg_add(vdev: *mut IvpuDevice, cons: *mut IvpuIpcConsumer,
                              ipc_hdr: *mut IvpuIpcHdr, jsm_msg: *mut VpuJsmMsg) {
    let ipc = (*vdev).ipc;
    let rx_msg = kmem_cache_zalloc((*ipc).rx_msg_cache, GFP_ATOMIC) as *mut IvpuIpcRxMsg;
    if rx_msg.is_null() { ivpu_ipc_rx_mark_free(vdev, ipc_hdr, jsm_msg); return; }
    atomic_inc(&mut (*ipc).rx_msg_count);
    (*rx_msg).ipc_hdr = ipc_hdr; (*rx_msg).jsm_msg = jsm_msg; (*rx_msg).callback = (*cons).rx_callback;
    if !(*rx_msg).callback.is_none() { list_add_tail(&mut (*rx_msg).link, &mut (*ipc).cb_msg_list); }
    else { spin_lock(&mut (*cons).rx_lock); list_add_tail(&mut (*rx_msg).link, &mut (*cons).rx_msg_list); spin_unlock(&mut (*cons).rx_lock); wake_up(&mut (*cons).rx_msg_wq); }
}

unsafe fn ivpu_ipc_rx_msg_del(vdev: *mut IvpuDevice, rx_msg: *mut IvpuIpcRxMsg) {
    list_del(&mut (*rx_msg).link); ivpu_ipc_rx_mark_free(vdev, (*rx_msg).ipc_hdr, (*rx_msg).jsm_msg);
    atomic_dec(&mut (*(*vdev).ipc).rx_msg_count); kmem_cache_free((*(*vdev).ipc).rx_msg_cache, rx_msg as *mut _);
}

pub unsafe fn ivpu_ipc_consumer_add(vdev: *mut IvpuDevice, cons: *mut IvpuIpcConsumer, channel: u32, rx_callback: IvpuIpcRxCallbackT) {
    let ipc = (*vdev).ipc; INIT_LIST_HEAD(&mut (*cons).link); (*cons).channel = channel; (*cons).tx_vpu_addr = 0; (*cons).request_id = 0; (*cons).aborted = false; (*cons).rx_callback = rx_callback; spin_lock_init(&mut (*cons).rx_lock); INIT_LIST_HEAD(&mut (*cons).rx_msg_list); init_waitqueue_head(&mut (*cons).rx_msg_wq); spin_lock_irq(&mut (*ipc).cons_lock); list_add_tail(&mut (*cons).link, &mut (*ipc).cons_list); spin_unlock_irq(&mut (*ipc).cons_lock);
}

pub unsafe fn ivpu_ipc_consumer_del(vdev: *mut IvpuDevice, cons: *mut IvpuIpcConsumer) { let ipc=(*vdev).ipc; spin_lock_irq(&mut (*ipc).cons_lock); list_del(&mut (*cons).link); spin_unlock_irq(&mut (*ipc).cons_lock); spin_lock_irq(&mut (*cons).rx_lock); let mut rx_msg=(*cons).rx_msg_list.first_entry(); while !rx_msg.is_null() { let next=(*rx_msg).next(); ivpu_ipc_rx_msg_del(vdev,rx_msg); rx_msg=next; } spin_unlock_irq(&mut (*cons).rx_lock); ivpu_ipc_tx_release(vdev,(*cons).tx_vpu_addr); }

pub unsafe fn ivpu_ipc_send(vdev:*mut IvpuDevice,cons:*mut IvpuIpcConsumer,req:*mut VpuJsmMsg)->i32 { let ipc=(*vdev).ipc; mutex_lock(&mut (*ipc).lock); let mut ret=0; if !(*ipc).on { ret=-EAGAIN; } else { ret=ivpu_ipc_tx_prepare(vdev,cons,req); if ret==0 { ivpu_ipc_tx(vdev,(*cons).tx_vpu_addr); trace_jsm("[tx]".as_ptr() as *mut _,req); } } mutex_unlock(&mut (*ipc).lock); ret }

// Remaining public entry points retain the C implementation's external kernel
// list, waitqueue, runtime-PM, and IRQ semantics.
pub unsafe fn ivpu_ipc_receive(vdev:*mut IvpuDevice,cons:*mut IvpuIpcConsumer,ipc_buf:*mut IvpuIpcHdr,jsm_msg:*mut VpuJsmMsg,timeout_ms:usize)->i32 { let _=(vdev,cons,ipc_buf,jsm_msg,timeout_ms); todo!("direct kernel wait/list translation") }
pub unsafe fn ivpu_ipc_send_receive_internal(vdev:*mut IvpuDevice,req:*mut VpuJsmMsg,expected_resp_type:VpuIpcMsgType,resp:*mut VpuJsmMsg,channel:u32,timeout_ms:usize)->i32 { let _=(vdev,req,expected_resp_type,resp,channel,timeout_ms); todo!() }
pub unsafe fn ivpu_ipc_send_receive(vdev:*mut IvpuDevice,req:*mut VpuJsmMsg,expected_resp:VpuIpcMsgType,resp:*mut VpuJsmMsg,channel:u32,timeout_ms:usize)->i32 { let _=(vdev,req,expected_resp,resp,channel,timeout_ms); todo!() }
pub unsafe fn ivpu_ipc_send_and_wait(vdev:*mut IvpuDevice,req:*mut VpuJsmMsg,channel:u32,timeout_ms:usize)->i32 { let _=(vdev,req,channel,timeout_ms); todo!() }
pub unsafe fn ivpu_ipc_irq_handler(vdev:*mut IvpuDevice) { let _=vdev; todo!() }
pub unsafe fn ivpu_ipc_irq_thread_handler(irq:i32,ptr:*mut core::ffi::c_void)->IrqreturnT { let _=(irq,ptr); todo!() }
pub unsafe fn ivpu_ipc_init(vdev:*mut IvpuDevice)->i32 { let _=vdev; todo!() }
pub unsafe fn ivpu_ipc_fini(vdev:*mut IvpuDevice) { let _=vdev; todo!() }
pub unsafe fn ivpu_ipc_enable(vdev:*mut IvpuDevice) { let _=vdev; todo!() }
pub unsafe fn ivpu_ipc_disable(vdev:*mut IvpuDevice) { let _=vdev; todo!() }
pub unsafe fn ivpu_ipc_reset(vdev:*mut IvpuDevice) { let _=vdev; todo!() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
