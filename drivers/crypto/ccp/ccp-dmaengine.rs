// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Cryptographic Coprocessor (CCP) driver
 *
 * Copyright (C) 2016,2019 Advanced Micro Devices, Inc.
 *
 * Author: Gary R Hook <gary.hook@amd.com>
 */

// External Linux kernel, DMA engine, CCP, and local driver declarations are
// supplied by the surrounding translation unit/build environment.

#[inline]
unsafe fn ccp_dma_width(mask: u64) -> u32 {
    let mask = mask.wrapping_add(1);
    if mask == 0 { 64 } else { fls64(mask) }
}

static mut DMA_CHAN_ATTR: u32 = CCP_DMA_DFLT;
static mut DMAENGINE: u32 = 1;

unsafe fn ccp_get_dma_chan_attr(ccp: *mut ccp_device) -> u32 {
    match DMA_CHAN_ATTR {
        CCP_DMA_DFLT => (*(*ccp).vdata).dma_chan_attr,
        CCP_DMA_PRIV => DMA_PRIVATE,
        CCP_DMA_PUB => 0,
        _ => {
            dev_info_once((*ccp).dev, "Invalid value for dma_chan_attr: %d\n", DMA_CHAN_ATTR);
            (*(*ccp).vdata).dma_chan_attr
        }
    }
}

unsafe fn ccp_free_cmd_resources(ccp: *mut ccp_device, list: *mut list_head) {
    let mut cmd: *mut ccp_dma_cmd;
    let mut ctmp: *mut ccp_dma_cmd;
    list_for_each_entry_safe!(cmd, ctmp, list, entry, {
        list_del!(&mut (*cmd).entry);
        kmem_cache_free((*ccp).dma_cmd_cache, cmd as *mut c_void);
    });
}

unsafe fn ccp_free_desc_resources(ccp: *mut ccp_device, list: *mut list_head) {
    let mut desc: *mut ccp_dma_desc;
    let mut dtmp: *mut ccp_dma_desc;
    list_for_each_entry_safe!(desc, dtmp, list, entry, {
        ccp_free_cmd_resources(ccp, &mut (*desc).active);
        ccp_free_cmd_resources(ccp, &mut (*desc).pending);
        list_del!(&mut (*desc).entry);
        kmem_cache_free((*ccp).dma_desc_cache, desc as *mut c_void);
    });
}

unsafe fn ccp_free_chan_resources(dma_chan: *mut dma_chan) {
    let chan = container_of!(dma_chan, ccp_dma_chan, dma_chan);
    let mut flags: c_ulong = 0;
    dev_dbg!((*(*chan).ccp).dev, "%s - chan=%p\n", __func__, chan);
    spin_lock_irqsave!(&mut (*chan).lock, flags);
    ccp_free_desc_resources((*chan).ccp, &mut (*chan).complete);
    ccp_free_desc_resources((*chan).ccp, &mut (*chan).active);
    ccp_free_desc_resources((*chan).ccp, &mut (*chan).pending);
    ccp_free_desc_resources((*chan).ccp, &mut (*chan).created);
    spin_unlock_irqrestore!(&mut (*chan).lock, flags);
}

unsafe fn ccp_cleanup_desc_resources(ccp: *mut ccp_device, list: *mut list_head) {
    let mut desc: *mut ccp_dma_desc;
    let mut dtmp: *mut ccp_dma_desc;
    list_for_each_entry_safe_reverse!(desc, dtmp, list, entry, {
        if !async_tx_test_ack!(&mut (*desc).tx_desc) { continue; }
        dev_dbg!((*ccp).dev, "%s - desc=%p\n", __func__, desc);
        ccp_free_cmd_resources(ccp, &mut (*desc).active);
        ccp_free_cmd_resources(ccp, &mut (*desc).pending);
        list_del!(&mut (*desc).entry);
        kmem_cache_free((*ccp).dma_desc_cache, desc as *mut c_void);
    });
}

unsafe fn ccp_do_cleanup(data: c_ulong) {
    let chan = data as *mut ccp_dma_chan;
    let mut flags: c_ulong = 0;
    dev_dbg!((*(*chan).ccp).dev, "%s - chan=%s\n", __func__, dma_chan_name!(&mut (*chan).dma_chan));
    spin_lock_irqsave!(&mut (*chan).lock, flags);
    ccp_cleanup_desc_resources((*chan).ccp, &mut (*chan).complete);
    spin_unlock_irqrestore!(&mut (*chan).lock, flags);
}

unsafe fn ccp_issue_next_cmd(desc: *mut ccp_dma_desc) -> c_int {
    let cmd = list_first_entry!(&mut (*desc).pending, ccp_dma_cmd, entry);
    list_move!(&mut (*cmd).entry, &mut (*desc).active);
    dev_dbg!((*(*desc).ccp).dev, "%s - tx %d, cmd=%p\n", __func__, (*desc).tx_desc.cookie, cmd);
    let ret = ccp_enqueue_cmd(&mut (*cmd).ccp_cmd);
    if ret == 0 || ret == -EINPROGRESS || ret == -EBUSY { return 0; }
    dev_dbg!((*(*desc).ccp).dev, "%s - error: ret=%d, tx %d, cmd=%p\n", __func__, ret, (*desc).tx_desc.cookie, cmd);
    ret
}

unsafe fn ccp_free_active_cmd(desc: *mut ccp_dma_desc) {
    let cmd = list_first_entry_or_null!(&mut (*desc).active, ccp_dma_cmd, entry);
    if cmd.is_null() { return; }
    dev_dbg!((*(*desc).ccp).dev, "%s - freeing tx %d cmd=%p\n", __func__, (*desc).tx_desc.cookie, cmd);
    list_del!(&mut (*cmd).entry);
    kmem_cache_free((*(*desc).ccp).dma_cmd_cache, cmd as *mut c_void);
}

unsafe fn __ccp_next_dma_desc(chan: *mut ccp_dma_chan, mut desc: *mut ccp_dma_desc) -> *mut ccp_dma_desc {
    if !desc.is_null() { list_move!(&mut (*desc).entry, &mut (*chan).complete); }
    desc = list_first_entry_or_null!(&mut (*chan).active, ccp_dma_desc, entry);
    desc
}

unsafe fn ccp_handle_active_desc(chan: *mut ccp_dma_chan, mut desc: *mut ccp_dma_desc) -> *mut ccp_dma_desc {
    let mut tx_desc: *mut dma_async_tx_descriptor;
    let mut flags: c_ulong = 0;
    loop {
        if !desc.is_null() {
            ccp_free_active_cmd(desc);
            if !list_empty!(&(*desc).pending) {
                if (*desc).status != DMA_ERROR { return desc; }
                ccp_free_cmd_resources((*desc).ccp, &mut (*desc).pending);
            }
            tx_desc = &mut (*desc).tx_desc;
        } else { tx_desc = core::ptr::null_mut(); }
        spin_lock_irqsave!(&mut (*chan).lock, flags);
        if !desc.is_null() {
            if (*desc).status != DMA_ERROR { (*desc).status = DMA_COMPLETE; }
            dev_dbg!((*(*desc).ccp).dev, "%s - tx %d complete, status=%u\n", __func__, (*desc).tx_desc.cookie, (*desc).status);
            dma_cookie_complete!(tx_desc);
            dma_descriptor_unmap!(tx_desc);
        }
        desc = __ccp_next_dma_desc(chan, desc);
        spin_unlock_irqrestore!(&mut (*chan).lock, flags);
        if !tx_desc.is_null() { dmaengine_desc_get_callback_invoke!(tx_desc, core::ptr::null_mut()); dma_run_dependencies!(tx_desc); }
        if desc.is_null() { break; }
    }
    core::ptr::null_mut()
}

unsafe fn __ccp_pending_to_active(chan: *mut ccp_dma_chan) -> *mut ccp_dma_desc {
    if list_empty!(&(*chan).pending) { return core::ptr::null_mut(); }
    let desc = if list_empty!(&(*chan).active) { list_first_entry!(&mut (*chan).pending, ccp_dma_desc, entry) } else { core::ptr::null_mut() };
    list_splice_tail_init!(&mut (*chan).pending, &mut (*chan).active);
    desc
}

unsafe fn ccp_cmd_callback(data: *mut c_void, err: c_int) {
    let mut desc = data as *mut ccp_dma_desc;
    if err == -EINPROGRESS { return; }
    let chan = container_of!((*desc).tx_desc.chan, ccp_dma_chan, dma_chan);
    dev_dbg!((*(*chan).ccp).dev, "%s - tx %d callback, err=%d\n", __func__, (*desc).tx_desc.cookie, err);
    if err != 0 { (*desc).status = DMA_ERROR; }
    loop {
        desc = ccp_handle_active_desc(chan, desc);
        if desc.is_null() || (*chan).status == DMA_PAUSED { break; }
        let ret = ccp_issue_next_cmd(desc);
        if ret == 0 { break; }
        (*desc).status = DMA_ERROR;
    }
    tasklet_schedule!(&mut (*chan).cleanup_tasklet);
}

unsafe fn ccp_tx_submit(tx_desc: *mut dma_async_tx_descriptor) -> dma_cookie_t {
    let desc = container_of!(tx_desc, ccp_dma_desc, tx_desc);
    let chan = container_of!((*tx_desc).chan, ccp_dma_chan, dma_chan);
    let mut flags: c_ulong = 0;
    spin_lock_irqsave!(&mut (*chan).lock, flags);
    let cookie = dma_cookie_assign!(tx_desc);
    list_move_tail!(&mut (*desc).entry, &mut (*chan).pending);
    spin_unlock_irqrestore!(&mut (*chan).lock, flags);
    dev_dbg!((*(*chan).ccp).dev, "%s - added tx descriptor %d to pending list\n", __func__, cookie);
    cookie
}

unsafe fn ccp_alloc_dma_cmd(chan: *mut ccp_dma_chan) -> *mut ccp_dma_cmd {
    let cmd = kmem_cache_alloc((*(*chan).ccp).dma_cmd_cache, GFP_NOWAIT) as *mut ccp_dma_cmd;
    if !cmd.is_null() { memset!(cmd, 0, core::mem::size_of::<ccp_dma_cmd>()); }
    cmd
}

unsafe fn ccp_alloc_dma_desc(chan: *mut ccp_dma_chan, flags: c_ulong) -> *mut ccp_dma_desc {
    let desc = kmem_cache_zalloc((*(*chan).ccp).dma_desc_cache, GFP_NOWAIT) as *mut ccp_dma_desc;
    if desc.is_null() { return core::ptr::null_mut(); }
    dma_async_tx_descriptor_init!(&mut (*desc).tx_desc, &mut (*chan).dma_chan);
    (*desc).tx_desc.flags = flags;
    (*desc).tx_desc.tx_submit = Some(ccp_tx_submit);
    (*desc).ccp = (*chan).ccp;
    INIT_LIST_HEAD!(&mut (*desc).entry); INIT_LIST_HEAD!(&mut (*desc).pending); INIT_LIST_HEAD!(&mut (*desc).active);
    (*desc).status = DMA_IN_PROGRESS;
    desc
}

unsafe fn ccp_create_desc(dma_chan: *mut dma_chan, dst_sg: *mut scatterlist, mut dst_nents: c_uint, src_sg: *mut scatterlist, mut src_nents: c_uint, flags: c_ulong) -> *mut ccp_dma_desc {
    let chan = container_of!(dma_chan, ccp_dma_chan, dma_chan);
    let ccp = (*chan).ccp;
    if dst_sg.is_null() || src_sg.is_null() || dst_nents == 0 || src_nents == 0 { return core::ptr::null_mut(); }
    let desc = ccp_alloc_dma_desc(chan, flags); if desc.is_null() { return desc; }
    let mut total_len: usize = 0; let mut src_len = sg_dma_len!(src_sg); let mut src_offset: usize = 0; let mut dst_len = sg_dma_len!(dst_sg); let mut dst_offset: usize = 0;
    loop {
        if src_len == 0 { src_nents -= 1; if src_nents == 0 { break; } src_sg = sg_next!(src_sg); if src_sg.is_null() { break; } src_len = sg_dma_len!(src_sg); src_offset = 0; continue; }
        if dst_len == 0 { dst_nents -= 1; if dst_nents == 0 { break; } dst_sg = sg_next!(dst_sg); if dst_sg.is_null() { break; } dst_len = sg_dma_len!(dst_sg); dst_offset = 0; continue; }
        let len = core::cmp::min(dst_len, src_len);
        let cmd = ccp_alloc_dma_cmd(chan); if cmd.is_null() { goto_err!(); }
        let ccp_cmd = &mut (*cmd).ccp_cmd; ccp_cmd.ccp = chan as *mut ccp_device; let ccp_pt = &mut ccp_cmd.u.passthru_nomap;
        ccp_cmd.flags = CCP_CMD_MAY_BACKLOG | CCP_CMD_PASSTHRU_NO_DMA_MAP; ccp_cmd.engine = CCP_ENGINE_PASSTHRU; ccp_pt.bit_mod = CCP_PASSTHRU_BITWISE_NOOP; ccp_pt.byte_swap = CCP_PASSTHRU_BYTESWAP_NOOP;
        ccp_pt.src_dma = sg_dma_address!(src_sg).wrapping_add(src_offset as u64); ccp_pt.dst_dma = sg_dma_address!(dst_sg).wrapping_add(dst_offset as u64); ccp_pt.src_len = len; ccp_pt.final_ = 1; ccp_cmd.callback = Some(ccp_cmd_callback); ccp_cmd.data = desc as *mut c_void;
        list_add_tail!(&mut (*cmd).entry, &mut (*desc).pending); dev_dbg!((*ccp).dev, "%s - cmd=%p, src=%pad, dst=%pad, len=%llu\n", __func__, cmd, &ccp_pt.src_dma, &ccp_pt.dst_dma, ccp_pt.src_len);
        total_len += len as usize; src_len -= len; src_offset += len as usize; dst_len -= len; dst_offset += len as usize;
    }
    (*desc).len = total_len; if list_empty!(&(*desc).pending) { goto_err!(); }
    dev_dbg!((*ccp).dev, "%s - desc=%p\n", __func__, desc); let mut sflags = 0; spin_lock_irqsave!(&mut (*chan).lock, sflags); list_add_tail!(&mut (*desc).entry, &mut (*chan).created); spin_unlock_irqrestore!(&mut (*chan).lock, sflags); return desc;
    goto_err!();
    ccp_free_cmd_resources(ccp, &mut (*desc).pending); kmem_cache_free((*ccp).dma_desc_cache, desc as *mut c_void); core::ptr::null_mut()
}

unsafe fn ccp_prep_dma_memcpy(dma_chan: *mut dma_chan, dst: dma_addr_t, src: dma_addr_t, len: usize, flags: c_ulong) -> *mut dma_async_tx_descriptor {
    let chan = container_of!(dma_chan, ccp_dma_chan, dma_chan); let mut dst_sg = core::mem::zeroed::<scatterlist>(); let mut src_sg = core::mem::zeroed::<scatterlist>();
    dev_dbg!((*(*chan).ccp).dev, "%s - src=%pad, dst=%pad, len=%zu, flags=%#lx\n", __func__, &src, &dst, len, flags); sg_init_table!(&mut dst_sg, 1); sg_dma_address_set!(&mut dst_sg, dst); sg_dma_len_set!(&mut dst_sg, len); sg_init_table!(&mut src_sg, 1); sg_dma_address_set!(&mut src_sg, src); sg_dma_len_set!(&mut src_sg, len);
    let desc = ccp_create_desc(dma_chan, &mut dst_sg, 1, &mut src_sg, 1, flags); if desc.is_null() { return core::ptr::null_mut(); } &mut (*desc).tx_desc
}

unsafe fn ccp_prep_dma_interrupt(dma_chan: *mut dma_chan, flags: c_ulong) -> *mut dma_async_tx_descriptor { let chan = container_of!(dma_chan, ccp_dma_chan, dma_chan); let desc = ccp_alloc_dma_desc(chan, flags); if desc.is_null() { core::ptr::null_mut() } else { &mut (*desc).tx_desc } }

unsafe fn ccp_issue_pending(dma_chan: *mut dma_chan) { let chan = container_of!(dma_chan, ccp_dma_chan, dma_chan); let mut flags = 0; dev_dbg!((*(*chan).ccp).dev, "%s\n", __func__); spin_lock_irqsave!(&mut (*chan).lock, flags); let desc = __ccp_pending_to_active(chan); spin_unlock_irqrestore!(&mut (*chan).lock, flags); if !desc.is_null() { ccp_cmd_callback(desc as *mut c_void, 0); } }

unsafe fn ccp_tx_status(dma_chan: *mut dma_chan, cookie: dma_cookie_t, state: *mut dma_tx_state) -> dma_status { let chan = container_of!(dma_chan, ccp_dma_chan, dma_chan); if (*chan).status == DMA_PAUSED { return DMA_PAUSED; } let mut ret = dma_cookie_status!(dma_chan, cookie, state); if ret == DMA_COMPLETE { let mut flags = 0; spin_lock_irqsave!(&mut (*chan).lock, flags); let mut desc: *mut ccp_dma_desc; list_for_each_entry!(desc, &mut (*chan).complete, entry, { if (*desc).tx_desc.cookie == cookie { ret = (*desc).status; break; } }); spin_unlock_irqrestore!(&mut (*chan).lock, flags); } dev_dbg!((*(*chan).ccp).dev, "%s - %u\n", __func__, ret); ret }

unsafe fn ccp_pause(dma_chan: *mut dma_chan) -> c_int { let chan = container_of!(dma_chan, ccp_dma_chan, dma_chan); (*chan).status = DMA_PAUSED; /* TODO: Wait for active DMA to complete before returning? */ 0 }
unsafe fn ccp_resume(dma_chan: *mut dma_chan) -> c_int { let chan = container_of!(dma_chan, ccp_dma_chan, dma_chan); let mut flags = 0; spin_lock_irqsave!(&mut (*chan).lock, flags); let desc = list_first_entry_or_null!(&mut (*chan).active, ccp_dma_desc, entry); spin_unlock_irqrestore!(&mut (*chan).lock, flags); (*chan).status = DMA_IN_PROGRESS; if !desc.is_null() { ccp_cmd_callback(desc as *mut c_void, 0); } 0 }
unsafe fn ccp_terminate_all(dma_chan: *mut dma_chan) -> c_int { let chan = container_of!(dma_chan, ccp_dma_chan, dma_chan); let mut flags = 0; dev_dbg!((*(*chan).ccp).dev, "%s\n", __func__); spin_lock_irqsave!(&mut (*chan).lock, flags); ccp_free_desc_resources((*chan).ccp, &mut (*chan).active); ccp_free_desc_resources((*chan).ccp, &mut (*chan).pending); ccp_free_desc_resources((*chan).ccp, &mut (*chan).created); spin_unlock_irqrestore!(&mut (*chan).lock, flags); 0 }

unsafe fn ccp_dma_release(ccp: *mut ccp_device) { for i in 0..(*ccp).cmd_q_count { let chan = (*ccp).ccp_dma_chan.add(i as usize); tasklet_kill!(&mut (*chan).cleanup_tasklet); list_del_rcu!(&mut (*(*chan).dma_chan).device_node); } }
unsafe fn ccp_dma_release_channels(ccp: *mut ccp_device) { for i in 0..(*ccp).cmd_q_count { let dma_chan = &mut (*ccp).ccp_dma_chan.add(i as usize).as_mut().unwrap().dma_chan; if (*dma_chan).client_count != 0 { dma_release_channel!(dma_chan); } } }

pub unsafe fn ccp_dmaengine_register(ccp: *mut ccp_device) -> c_int { if DMAENGINE == 0 { return 0; } let dma_dev = &mut (*ccp).dma_dev; (*ccp).ccp_dma_chan = devm_kcalloc!((*ccp).dev, (*ccp).cmd_q_count, core::mem::size_of::<ccp_dma_chan>(), GFP_KERNEL); if (*ccp).ccp_dma_chan.is_null() { return -ENOMEM; } let name = devm_kasprintf!((*ccp).dev, GFP_KERNEL, "%s-dmaengine-cmd-cache", (*ccp).name); if name.is_null() { return -ENOMEM; } (*ccp).dma_cmd_cache = kmem_cache_create!(name, core::mem::size_of::<ccp_dma_cmd>(), core::mem::size_of::<*mut c_void>(), SLAB_HWCACHE_ALIGN, None); if (*ccp).dma_cmd_cache.is_null() { return -ENOMEM; } let desc_name = devm_kasprintf!((*ccp).dev, GFP_KERNEL, "%s-dmaengine-desc-cache", (*ccp).name); if desc_name.is_null() { kmem_cache_destroy!((*ccp).dma_cmd_cache); return -ENOMEM; } (*ccp).dma_desc_cache = kmem_cache_create!(desc_name, core::mem::size_of::<ccp_dma_desc>(), core::mem::size_of::<*mut c_void>(), SLAB_HWCACHE_ALIGN, None); if (*ccp).dma_desc_cache.is_null() { kmem_cache_destroy!((*ccp).dma_cmd_cache); return -ENOMEM; }
    (*dma_dev).dev = (*ccp).dev; (*dma_dev).src_addr_widths = ccp_dma_width(dma_get_mask!((*ccp).dev)); (*dma_dev).dst_addr_widths = (*dma_dev).src_addr_widths; (*dma_dev).directions = DMA_MEM_TO_MEM; (*dma_dev).residue_granularity = DMA_RESIDUE_GRANULARITY_DESCRIPTOR; dma_cap_set!(DMA_MEMCPY, &mut (*dma_dev).cap_mask); dma_cap_set!(DMA_INTERRUPT, &mut (*dma_dev).cap_mask); if ccp_get_dma_chan_attr(ccp) == DMA_PRIVATE { dma_cap_set!(DMA_PRIVATE, &mut (*dma_dev).cap_mask); } INIT_LIST_HEAD!(&mut (*dma_dev).channels);
    for i in 0..(*ccp).cmd_q_count { let chan = (*ccp).ccp_dma_chan.add(i as usize); (*chan).ccp = ccp; spin_lock_init!(&mut (*chan).lock); INIT_LIST_HEAD!(&mut (*chan).created); INIT_LIST_HEAD!(&mut (*chan).pending); INIT_LIST_HEAD!(&mut (*chan).active); INIT_LIST_HEAD!(&mut (*chan).complete); tasklet_init!(&mut (*chan).cleanup_tasklet, ccp_do_cleanup, chan as c_ulong); (*chan).dma_chan.device = dma_dev; dma_cookie_init!(&mut (*chan).dma_chan); list_add_tail!(&mut (*chan).dma_chan.device_node, &mut (*dma_dev).channels); }
    (*dma_dev).device_free_chan_resources = Some(ccp_free_chan_resources); (*dma_dev).device_prep_dma_memcpy = Some(ccp_prep_dma_memcpy); (*dma_dev).device_prep_dma_interrupt = Some(ccp_prep_dma_interrupt); (*dma_dev).device_issue_pending = Some(ccp_issue_pending); (*dma_dev).device_tx_status = Some(ccp_tx_status); (*dma_dev).device_pause = Some(ccp_pause); (*dma_dev).device_resume = Some(ccp_resume); (*dma_dev).device_terminate_all = Some(ccp_terminate_all); let ret = dma_async_device_register!(dma_dev); if ret != 0 { ccp_dma_release(ccp); kmem_cache_destroy!((*ccp).dma_desc_cache); kmem_cache_destroy!((*ccp).dma_cmd_cache); } ret }

pub unsafe fn ccp_dmaengine_unregister(ccp: *mut ccp_device) { if DMAENGINE == 0 { return; } let dma_dev = &mut (*ccp).dma_dev; ccp_dma_release_channels(ccp); dma_async_device_unregister!(dma_dev); ccp_dma_release(ccp); kmem_cache_destroy!((*ccp).dma_desc_cache); kmem_cache_destroy!((*ccp).dma_cmd_cache); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
