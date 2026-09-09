// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016, NVIDIA CORPORATION.  All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

const MSG_ACK: ::core::primitive::c_ulong = BIT(0);
const MSG_RING: ::core::primitive::c_ulong = BIT(1);
const TAG_SZ: usize = 32;

#[inline]
unsafe fn channel_to_ops(channel: *mut tegra_bpmp_channel) -> *const tegra_bpmp_ops {
    let bpmp = (*channel).bpmp;
    (*(*bpmp).soc).ops
}

pub unsafe fn tegra_bpmp_get_with_id(dev: *mut device, id: *mut ::core::ffi::c_uint) -> *mut tegra_bpmp {
    let mut args: of_phandle_args = ::core::mem::zeroed();
    let err = __of_parse_phandle_with_args((*dev).of_node, b"nvidia,bpmp\0".as_ptr() as *const _, core::ptr::null(), 1, 0, &mut args);
    if err < 0 { return ERR_PTR(err); }
    let pdev = of_find_device_by_node(args.np);
    let bpmp;
    if pdev.is_null() {
        bpmp = ERR_PTR(-ENODEV);
    } else {
        bpmp = platform_get_drvdata(pdev);
        if bpmp.is_null() {
            let result = ERR_PTR(-EPROBE_DEFER);
            put_device(&mut (*pdev).dev);
            of_node_put(args.np);
            return result;
        }
        if !id.is_null() { *id = args.args[0]; }
    }
    of_node_put(args.np);
    bpmp
}

pub unsafe fn tegra_bpmp_get(dev: *mut device) -> *mut tegra_bpmp {
    let np = of_parse_phandle((*dev).of_node, b"nvidia,bpmp\0".as_ptr() as *const _, 0);
    if np.is_null() { return ERR_PTR(-ENOENT); }
    let pdev = of_find_device_by_node(np);
    let bpmp;
    if pdev.is_null() {
        bpmp = ERR_PTR(-ENODEV);
    } else {
        bpmp = platform_get_drvdata(pdev);
        if bpmp.is_null() {
            let result = ERR_PTR(-EPROBE_DEFER);
            put_device(&mut (*pdev).dev);
            of_node_put(np);
            return result;
        }
    }
    of_node_put(np);
    bpmp
}

pub unsafe fn tegra_bpmp_put(bpmp: *mut tegra_bpmp) {
    if !bpmp.is_null() { put_device((*bpmp).dev); }
}

unsafe fn tegra_bpmp_channel_get_thread_index(channel: *mut tegra_bpmp_channel) -> ::core::ffi::c_int {
    let bpmp = (*channel).bpmp;
    let count = (*(*bpmp).soc).channels.thread.count;
    let index = channel.offset_from((*bpmp).threaded_channels) as ::core::ffi::c_int;
    if index < 0 || index >= count as ::core::ffi::c_int { return -EINVAL; }
    index
}

unsafe fn tegra_bpmp_message_valid(msg: *const tegra_bpmp_message) -> bool {
    (*msg).tx.size <= MSG_DATA_MIN_SZ && (*msg).rx.size <= MSG_DATA_MIN_SZ &&
        ((*msg).tx.size == 0 || !(*msg).tx.data.is_null()) &&
        ((*msg).rx.size == 0 || !(*msg).rx.data.is_null())
}

unsafe fn tegra_bpmp_is_response_ready(channel: *mut tegra_bpmp_channel) -> bool { (*channel_to_ops(channel)).is_response_ready(channel) }
unsafe fn tegra_bpmp_is_request_ready(channel: *mut tegra_bpmp_channel) -> bool { (*channel_to_ops(channel)).is_request_ready(channel) }

unsafe fn tegra_bpmp_wait_response(channel: *mut tegra_bpmp_channel) -> ::core::ffi::c_int {
    let timeout = (*(*(*channel).bpmp).soc).channels.cpu_tx.timeout;
    let end = ktime_add_us(ktime_get(), timeout);
    loop { if tegra_bpmp_is_response_ready(channel) { return 0; } if !ktime_before(ktime_get(), end) { break; } }
    -ETIMEDOUT
}

unsafe fn tegra_bpmp_ack_response(channel: *mut tegra_bpmp_channel) -> ::core::ffi::c_int { (*channel_to_ops(channel)).ack_response(channel) }
unsafe fn tegra_bpmp_ack_request(channel: *mut tegra_bpmp_channel) -> ::core::ffi::c_int { (*channel_to_ops(channel)).ack_request(channel) }
unsafe fn tegra_bpmp_is_request_channel_free(channel: *mut tegra_bpmp_channel) -> bool { (*channel_to_ops(channel)).is_request_channel_free(channel) }
unsafe fn tegra_bpmp_is_response_channel_free(channel: *mut tegra_bpmp_channel) -> bool { (*channel_to_ops(channel)).is_response_channel_free(channel) }

unsafe fn tegra_bpmp_wait_request_channel_free(channel: *mut tegra_bpmp_channel) -> ::core::ffi::c_int {
    let timeout = (*(*(*channel).bpmp).soc).channels.cpu_tx.timeout;
    let start = ns_to_ktime(local_clock());
    loop { if tegra_bpmp_is_request_channel_free(channel) { return 0; } let now = ns_to_ktime(local_clock()); if ktime_us_delta(now, start) >= timeout { break; } }
    -ETIMEDOUT
}

unsafe fn tegra_bpmp_post_request(channel: *mut tegra_bpmp_channel) -> ::core::ffi::c_int { (*channel_to_ops(channel)).post_request(channel) }
unsafe fn tegra_bpmp_post_response(channel: *mut tegra_bpmp_channel) -> ::core::ffi::c_int { (*channel_to_ops(channel)).post_response(channel) }
unsafe fn tegra_bpmp_ring_doorbell(bpmp: *mut tegra_bpmp) -> ::core::ffi::c_int { (*(*bpmp).soc).ops.ring_doorbell(bpmp) }

unsafe fn __tegra_bpmp_channel_read(channel: *mut tegra_bpmp_channel, data: *mut ::core::ffi::c_void, size: usize, ret: *mut ::core::ffi::c_int) -> isize {
    if !data.is_null() && size > 0 { tegra_bpmp_mb_read(data, &(*channel).ib, size); }
    let err = tegra_bpmp_ack_response(channel); if err < 0 { return err as isize; }
    *ret = tegra_bpmp_mb_read_field(&(*channel).ib, code); 0
}

unsafe fn tegra_bpmp_channel_read(channel: *mut tegra_bpmp_channel, data: *mut ::core::ffi::c_void, size: usize, ret: *mut ::core::ffi::c_int) -> isize {
    let bpmp = (*channel).bpmp; let index = tegra_bpmp_channel_get_thread_index(channel); let err;
    if index < 0 { err = index as isize; } else { let mut flags = 0; spin_lock_irqsave(&mut (*bpmp).lock, &mut flags); err = __tegra_bpmp_channel_read(channel, data, size, ret); clear_bit(index as usize, (*bpmp).threaded.allocated); spin_unlock_irqrestore(&mut (*bpmp).lock, flags); }
    up(&mut (*bpmp).threaded.lock); err
}

unsafe fn __tegra_bpmp_channel_write(channel: *mut tegra_bpmp_channel, mrq: ::core::ffi::c_uint, flags: ::core::ffi::c_ulong, data: *const ::core::ffi::c_void, size: usize) -> ::core::ffi::c_int {
    tegra_bpmp_mb_write_field(&mut (*channel).ob, code, mrq); tegra_bpmp_mb_write_field(&mut (*channel).ob, flags, flags);
    if !data.is_null() && size > 0 { tegra_bpmp_mb_write(&mut (*channel).ob, data, size); }
    tegra_bpmp_post_request(channel)
}

unsafe fn tegra_bpmp_write_threaded(bpmp: *mut tegra_bpmp, mrq: ::core::ffi::c_uint, data: *const ::core::ffi::c_void, size: usize) -> *mut tegra_bpmp_channel {
    let timeout = (*(*bpmp).soc).channels.thread.timeout; let count = (*(*bpmp).soc).channels.thread.count;
    let mut err = down_timeout(&mut (*bpmp).threaded.lock, usecs_to_jiffies(timeout)); if err < 0 { return ERR_PTR(err); }
    let mut flags = 0; spin_lock_irqsave(&mut (*bpmp).lock, &mut flags);
    let index = find_first_zero_bit((*bpmp).threaded.allocated, count); if index == count { err = -EBUSY; spin_unlock_irqrestore(&mut (*bpmp).lock, flags); up(&mut (*bpmp).threaded.lock); return ERR_PTR(err); }
    let channel = (*bpmp).threaded_channels.add(index); if !tegra_bpmp_is_request_channel_free(channel) { err = -EBUSY; spin_unlock_irqrestore(&mut (*bpmp).lock, flags); up(&mut (*bpmp).threaded.lock); return ERR_PTR(err); }
    set_bit(index, (*bpmp).threaded.allocated); err = __tegra_bpmp_channel_write(channel, mrq, MSG_ACK | MSG_RING, data, size);
    if err < 0 { clear_bit(index, (*bpmp).threaded.allocated); spin_unlock_irqrestore(&mut (*bpmp).lock, flags); up(&mut (*bpmp).threaded.lock); return ERR_PTR(err); }
    set_bit(index, (*bpmp).threaded.busy); spin_unlock_irqrestore(&mut (*bpmp).lock, flags); channel
}

unsafe fn tegra_bpmp_channel_write(channel: *mut tegra_bpmp_channel, mrq: ::core::ffi::c_uint, flags: ::core::ffi::c_ulong, data: *const ::core::ffi::c_void, size: usize) -> ::core::ffi::c_int {
    let err = tegra_bpmp_wait_request_channel_free(channel); if err < 0 { return err; } __tegra_bpmp_channel_write(channel, mrq, flags, data, size)
}

unsafe fn tegra_bpmp_resume(dev: *mut device) -> ::core::ffi::c_int;

pub unsafe fn tegra_bpmp_transfer_atomic(bpmp: *mut tegra_bpmp, msg: *mut tegra_bpmp_message) -> ::core::ffi::c_int {
    if WARN_ON(!irqs_disabled()) { return -EPERM; } if !tegra_bpmp_message_valid(msg) { return -EINVAL; }
    if (*bpmp).suspended { if (*msg).flags & TEGRA_BPMP_MESSAGE_RESET != 0 { tegra_bpmp_resume((*bpmp).dev); } else { return -EAGAIN; } }
    let channel = (*bpmp).tx_channel; spin_lock(&mut (*bpmp).atomic_tx_lock); let err = tegra_bpmp_channel_write(channel, (*msg).mrq, MSG_ACK, (*msg).tx.data, (*msg).tx.size); if err < 0 { spin_unlock(&mut (*bpmp).atomic_tx_lock); return err; } spin_unlock(&mut (*bpmp).atomic_tx_lock);
    let err = tegra_bpmp_ring_doorbell(bpmp); if err < 0 { return err; } let err = tegra_bpmp_wait_response(channel); if err < 0 { return err; } __tegra_bpmp_channel_read(channel, (*msg).rx.data, (*msg).rx.size, &mut (*msg).rx.ret) as ::core::ffi::c_int
}

pub unsafe fn tegra_bpmp_transfer(bpmp: *mut tegra_bpmp, msg: *mut tegra_bpmp_message) -> ::core::ffi::c_int {
    if WARN_ON(irqs_disabled()) { return -EPERM; } if !tegra_bpmp_message_valid(msg) { return -EINVAL; }
    if (*bpmp).suspended { if (*msg).flags & TEGRA_BPMP_MESSAGE_RESET != 0 { tegra_bpmp_resume((*bpmp).dev); } else { return -EAGAIN; } }
    let channel = tegra_bpmp_write_threaded(bpmp, (*msg).mrq, (*msg).tx.data, (*msg).tx.size); if IS_ERR(channel) { return PTR_ERR(channel); }
    let err = tegra_bpmp_ring_doorbell(bpmp); if err < 0 { return err; } let timeout = usecs_to_jiffies((*(*bpmp).soc).channels.thread.timeout); let err = wait_for_completion_timeout(&mut (*channel).completion, timeout); if err == 0 { return -ETIMEDOUT; }
    tegra_bpmp_channel_read(channel, (*msg).rx.data, (*msg).rx.size, &mut (*msg).rx.ret) as ::core::ffi::c_int
}

unsafe fn tegra_bpmp_find_mrq(bpmp: *mut tegra_bpmp, mrq: ::core::ffi::c_uint) -> *mut tegra_bpmp_mrq {
    let mut entry = (*bpmp).mrqs.next as *mut tegra_bpmp_mrq; while entry != (&(*bpmp).mrqs as *const _ as *mut _ ) { if (*entry).mrq == mrq { return entry; } entry = (*entry).list.next as *mut tegra_bpmp_mrq; } core::ptr::null_mut()
}

pub unsafe fn tegra_bpmp_mrq_return(channel: *mut tegra_bpmp_channel, code: ::core::ffi::c_int, data: *const ::core::ffi::c_void, size: usize) {
    let flags = tegra_bpmp_mb_read_field(&(*channel).ib, flags); let bpmp = (*channel).bpmp; if WARN_ON(size > MSG_DATA_MIN_SZ) { return; } let err = tegra_bpmp_ack_request(channel); if WARN_ON(err < 0) { return; } if flags & MSG_ACK == 0 { return; } if WARN_ON(!tegra_bpmp_is_response_channel_free(channel)) { return; }
    tegra_bpmp_mb_write_field(&mut (*channel).ob, code, code); if !data.is_null() && size > 0 { tegra_bpmp_mb_write(&mut (*channel).ob, data, size); } let err = tegra_bpmp_post_response(channel); if WARN_ON(err < 0) { return; } if flags & MSG_RING != 0 { let err = tegra_bpmp_ring_doorbell(bpmp); if WARN_ON(err < 0) { return; } }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
