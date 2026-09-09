// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2022 Intel Corporation */

// Dependencies supplied by the surrounding kernel translation.

const ADF_MAX_RETRIES: i32 = 20;

unsafe fn qat_alg_send_message_retry(req: *mut qat_alg_req) -> i32 {
    let mut ret: i32 = 0;
    let mut ctr: i32 = 0;

    loop {
        ret = adf_send_message((*req).tx_ring, (*req).fw_req);
        if !(ret == -EAGAIN && {
            ctr += 1;
            ctr <= ADF_MAX_RETRIES
        }) {
            break;
        }
    }

    if ret == -EAGAIN {
        return -ENOSPC;
    }

    -EINPROGRESS
}

pub unsafe fn qat_alg_send_backlog(backlog: *mut qat_instance_backlog) {
    let mut req: *mut qat_alg_req;
    let mut tmp: *mut qat_alg_req;

    spin_lock_bh(&mut (*backlog).lock);
    list_for_each_entry_safe!(req, tmp, &mut (*backlog).list, list, {
        if adf_send_message((*req).tx_ring, (*req).fw_req) != 0 {
            /* The HW ring is full. Do nothing.
             * qat_alg_send_backlog() will be invoked again by
             * another callback.
             */
            break;
        }
        list_del(&mut (*req).list);
        crypto_request_complete((*req).base, -EINPROGRESS);
    });
    spin_unlock_bh(&mut (*backlog).lock);
}

unsafe fn qat_alg_try_enqueue(req: *mut qat_alg_req) -> bool {
    let backlog: *mut qat_instance_backlog = (*req).backlog;
    let tx_ring: *mut adf_etr_ring_data = (*req).tx_ring;
    let fw_req: *mut u32 = (*req).fw_req;

    /* Check if any request is already backlogged */
    if !list_empty(&(*backlog).list) {
        return false;
    }

    /* Check if ring is nearly full */
    if adf_ring_nearly_full(tx_ring) {
        return false;
    }

    /* Try to enqueue to HW ring */
    if adf_send_message(tx_ring, fw_req) != 0 {
        return false;
    }

    true
}

unsafe fn qat_alg_send_message_maybacklog(req: *mut qat_alg_req) -> i32 {
    let backlog: *mut qat_instance_backlog = (*req).backlog;
    let mut ret: i32 = -EINPROGRESS;

    if qat_alg_try_enqueue(req) {
        return ret;
    }

    spin_lock_bh(&mut (*backlog).lock);
    if !qat_alg_try_enqueue(req) {
        list_add_tail(&mut (*req).list, &mut (*backlog).list);
        ret = -EBUSY;
    }
    spin_unlock_bh(&mut (*backlog).lock);

    ret
}

pub unsafe fn qat_alg_send_message(req: *mut qat_alg_req) -> i32 {
    let flags: u32 = (*(*req).base).flags;

    if flags & CRYPTO_TFM_REQ_MAY_BACKLOG != 0 {
        qat_alg_send_message_maybacklog(req)
    } else {
        qat_alg_send_message_retry(req)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
