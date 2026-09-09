/*
 * This file is part of the Chelsio T4/T5/T6 Ethernet driver for Linux.
 *
 * Copyright (C) 2011-2016 Chelsio Communications.  All rights reserved.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation.
 *
 * Written and Maintained by:
 * Manoj Malviya (manojmalviya@chelsio.com)
 * Atul Gupta (atul.gupta@chelsio.com)
 * Jitendra Lulla (jlulla@chelsio.com)
 * Yeshaswi M R Gowda (yeshaswi@chelsio.com)
 * Harsh Jain (harsh@chelsio.com)
 */

// Linux kernel, crypto, and Chelsio headers supplied by external dependencies.

static mut drv_data: chcr_driver_data = chcr_driver_data::ZERO;

type chcr_handler_func = unsafe extern "C" fn(*mut adapter, *mut u8) -> i32;

unsafe extern "C" {
}

static mut work_handlers: [Option<chcr_handler_func>; NUM_CPL_CMDS] = [None; NUM_CPL_CMDS];

static mut chcr_uld_info: cxgb4_uld_info = cxgb4_uld_info {
    name: DRV_MODULE_NAME,
    nrxq: MAX_ULD_QSETS,
    rxq_size: 1024,
    add: Some(chcr_uld_add),
    state_change: Some(chcr_uld_state_change),
    rx_handler: Some(chcr_uld_rx_handler),
};

unsafe fn detach_work_fn(work: *mut work_struct) {
    let dev = container_of!(work, chcr_dev, detach_work.work);
    if atomic_read(&(*dev).inflight) != 0 {
        (*dev).wqretry -= 1;
        if (*dev).wqretry != 0 {
            pr_debug!("Request Inflight Count {}\n", atomic_read(&(*dev).inflight));
            schedule_delayed_work(&mut (*dev).detach_work, WQ_DETACH_TM);
        } else {
            WARN!(1, "CHCR:{} request Still Pending\n", atomic_read(&(*dev).inflight));
            complete(&mut (*dev).detach_comp);
        }
    } else {
        complete(&mut (*dev).detach_comp);
    }
}

pub unsafe fn assign_chcr_device() -> *mut uld_ctx {
    let mut u_ctx = core::ptr::null_mut();
    mutex_lock(&mut (*drv_data_ptr()).drv_mutex);
    if !list_empty(&(*drv_data_ptr()).act_dev) {
        u_ctx = (*drv_data_ptr()).last_dev;
        if list_is_last(&(*u_ctx).entry, &(*drv_data_ptr()).act_dev) {
            (*drv_data_ptr()).last_dev = list_first_entry!(&(*drv_data_ptr()).act_dev, uld_ctx, entry);
        } else {
            (*drv_data_ptr()).last_dev = list_next_entry!((*drv_data_ptr()).last_dev, entry);
        }
    }
    mutex_unlock(&mut (*drv_data_ptr()).drv_mutex);
    u_ctx
}

unsafe fn chcr_dev_add(u_ctx: *mut uld_ctx) {
    let dev = &mut (*u_ctx).dev;
    dev.state = CHCR_ATTACH;
    atomic_set(&mut dev.inflight, 0);
    mutex_lock(&mut (*drv_data_ptr()).drv_mutex);
    list_move(&mut (*u_ctx).entry, &mut (*drv_data_ptr()).act_dev);
    if (*drv_data_ptr()).last_dev.is_null() { (*drv_data_ptr()).last_dev = u_ctx; }
    mutex_unlock(&mut (*drv_data_ptr()).drv_mutex);
}

unsafe fn chcr_dev_init(u_ctx: *mut uld_ctx) {
    let dev = &mut (*u_ctx).dev;
    spin_lock_init(&mut dev.lock_chcr_dev);
    INIT_DELAYED_WORK!(&mut dev.detach_work, detach_work_fn);
    init_completion(&mut dev.detach_comp);
    dev.state = CHCR_INIT;
    dev.wqretry = WQ_RETRY;
    atomic_inc(&mut (*drv_data_ptr()).dev_count);
    atomic_set(&mut dev.inflight, 0);
    mutex_lock(&mut (*drv_data_ptr()).drv_mutex);
    list_add_tail(&mut (*u_ctx).entry, &mut (*drv_data_ptr()).inact_dev);
    mutex_unlock(&mut (*drv_data_ptr()).drv_mutex);
}

unsafe fn chcr_dev_move(u_ctx: *mut uld_ctx) -> i32 {
    mutex_lock(&mut (*drv_data_ptr()).drv_mutex);
    if (*drv_data_ptr()).last_dev == u_ctx {
        if list_is_last(&(*u_ctx).entry, &(*drv_data_ptr()).act_dev) {
            (*drv_data_ptr()).last_dev = list_first_entry!(&(*drv_data_ptr()).act_dev, uld_ctx, entry);
        } else {
            (*drv_data_ptr()).last_dev = list_next_entry!((*drv_data_ptr()).last_dev, entry);
        }
    }
    list_move(&mut (*u_ctx).entry, &mut (*drv_data_ptr()).inact_dev);
    if list_empty(&(*drv_data_ptr()).act_dev) { (*drv_data_ptr()).last_dev = core::ptr::null_mut(); }
    atomic_dec(&mut (*drv_data_ptr()).dev_count);
    mutex_unlock(&mut (*drv_data_ptr()).drv_mutex);
    0
}

unsafe fn cpl_fw6_pld_handler_impl(adap: *mut adapter, input: *mut u8) -> i32 {
    let fw6_pld = input as *mut cpl_fw6_pld;
    let req = be64_to_cpu((*fw6_pld).data[1]) as usize as *mut crypto_async_request;
    let ack_err_status = ntohl(*(input.add(4) as *const u32));
    let mut error_status = if CHK_MAC_ERR_BIT(ack_err_status) || CHK_PAD_ERR_BIT(ack_err_status) { -EBADMSG } else { 0 };
    if !req { error_status = chcr_handle_resp(req, input, error_status); }
    else { pr_err!("Incorrect request address from the firmware\n"); return -EFAULT; }
    if error_status != 0 { atomic_inc(&mut (*adap).chcr_stats.error); }
    0
}

unsafe fn chcr_uld_add(lld: *const cxgb4_lld_info) -> *mut core::ffi::c_void {
    pr_info_once!("{}\n", DRV_DESC);
    if (*lld).ulp_crypto & ULP_CRYPTO_LOOKASIDE == 0 { return ERR_PTR(-EOPNOTSUPP); }
    let u_ctx = kzalloc_obj::<uld_ctx>();
    if u_ctx.is_null() { return ERR_PTR(-ENOMEM); }
    (*u_ctx).lldi = *lld;
    chcr_dev_init(u_ctx);
    u_ctx as *mut core::ffi::c_void
}

unsafe fn chcr_uld_rx_handler(handle: *mut core::ffi::c_void, rsp: *const u64, pgl: *const pkt_gl) -> i32 {
    let u_ctx = handle as *mut uld_ctx;
    let dev = &mut (*u_ctx).dev;
    let adap = padap(dev);
    let rpl = rsp as *const cpl_fw6_pld;
    let handler = work_handlers[(*rpl).opcode as usize];
    if handler.is_none() { pr_err!("Unsupported opcode {} received\n", (*rpl).opcode); return 0; }
    if pgl.is_null() { handler.unwrap()(adap, rsp.add(1) as *mut u8); }
    else { handler.unwrap()(adap, (*pgl).va); }
    0
}

unsafe fn chcr_uld_state_change(handle: *mut core::ffi::c_void, state: cxgb4_state) -> i32 {
    let u_ctx = handle as *mut uld_ctx;
    let mut ret = 0;
    match state {
        CXGB4_STATE_UP => {
            if (*u_ctx).dev.state != CHCR_INIT { return 0; }
            chcr_dev_add(u_ctx); ret = start_crypto();
        }
        CXGB4_STATE_DETACH => {
            chcr_detach_device(u_ctx);
            if atomic_read(&(*drv_data_ptr()).dev_count) == 0 { stop_crypto(); }
        }
        CXGB4_STATE_START_RECOVERY | CXGB4_STATE_DOWN => {}
        _ => {}
    }
    ret
}

pub unsafe fn chcr_send_wr(skb: *mut sk_buff) -> i32 { cxgb4_crypto_send((*skb).dev, skb) }

unsafe fn chcr_detach_device(u_ctx: *mut uld_ctx) {
    let dev = &mut (*u_ctx).dev;
    if dev.state == CHCR_DETACH { pr_debug!("Detached Event received for already detach device\n"); return; }
    dev.state = CHCR_DETACH;
    if atomic_read(&dev.inflight) != 0 { schedule_delayed_work(&mut dev.detach_work, WQ_DETACH_TM); wait_for_completion(&mut dev.detach_comp); }
    chcr_dev_move(u_ctx);
}

unsafe fn chcr_crypto_init() -> i32 {
    INIT_LIST_HEAD!(&mut (*drv_data_ptr()).act_dev);
    INIT_LIST_HEAD!(&mut (*drv_data_ptr()).inact_dev);
    atomic_set(&mut (*drv_data_ptr()).dev_count, 0);
    mutex_init(&mut (*drv_data_ptr()).drv_mutex);
    (*drv_data_ptr()).last_dev = core::ptr::null_mut();
    cxgb4_register_uld(CXGB4_ULD_CRYPTO, &mut chcr_uld_info);
    0
}

unsafe fn chcr_crypto_exit() {
    stop_crypto();
    cxgb4_unregister_uld(CXGB4_ULD_CRYPTO);
    mutex_lock(&mut (*drv_data_ptr()).drv_mutex);
    list_for_each_entry_safe!(_u_ctx, _tmp, &mut (*drv_data_ptr()).act_dev, entry, {
        let adap = padap(&mut (*_u_ctx).dev);
        memset(&mut (*adap).chcr_stats, 0, core::mem::size_of::<chcr_stats>());
        list_del(&mut (*_u_ctx).entry); kfree(_u_ctx);
    });
    list_for_each_entry_safe!(_u_ctx, _tmp, &mut (*drv_data_ptr()).inact_dev, entry, {
        let adap = padap(&mut (*_u_ctx).dev);
        memset(&mut (*adap).chcr_stats, 0, core::mem::size_of::<chcr_stats>());
        list_del(&mut (*_u_ctx).entry); kfree(_u_ctx);
    });
    mutex_unlock(&mut (*drv_data_ptr()).drv_mutex);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
