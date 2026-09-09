// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Isochronous I/O functionality:
 *   - Isochronous DMA context management
 *   - Isochronous bus resource management (channels, bandwidth), client side
 *
 * Copyright (C) 2006 Kristian Hoegsberg <krh@bitplanet.net>
 */

// Linux kernel dependencies are supplied by other translated modules.

/* Isochronous DMA context management */

pub unsafe fn fw_iso_buffer_alloc(buffer: *mut fw_iso_buffer, page_count: i32) -> i32 {
    let page_array = kzalloc_objs::<*mut page>(page_count as usize);
    if page_array.is_null() {
        return -ENOMEM;
    }

    // Retrieve noncontiguous pages. The descriptors for 1394 OHCI isochronous DMA contexts
    // have a set of address and length per each, while the reason to use pages is the
    // convenience to map them into virtual address space of user process.
    let nr_populated = alloc_pages_bulk(GFP_KERNEL | GFP_DMA32 | __GFP_ZERO,
                                        page_count as usize, page_array);
    if nr_populated != page_count as usize {
        // Assuming the above call fills page_array sequentially from the beginning.
        release_pages(page_array, nr_populated);
        return -ENOMEM;
    }

    (*buffer).page_count = page_count;
    (*buffer).pages = page_array;
    0
}

pub unsafe fn fw_iso_buffer_map_dma(buffer: *mut fw_iso_buffer, card: *mut fw_card,
                                    direction: dma_data_direction) -> i32 {
    let dma_addrs = kzalloc_objs::<dma_addr_t>((*buffer).page_count as usize);
    let mut i = 0;
    if dma_addrs.is_null() {
        return -ENOMEM;
    }

    // Retrieve DMA mapping addresses for the pages. They are not contiguous. Maintain the cache
    // coherency for the pages by hand.
    while i < (*buffer).page_count {
        // The dma_map_phys() with a physical address per page is available here, instead.
        let dma_addr = dma_map_page((*card).device, *(*buffer).pages.add(i as usize), 0,
                                    PAGE_SIZE, direction);
        if dma_mapping_error((*card).device, dma_addr) {
            break;
        }
        *dma_addrs.add(i as usize) = dma_addr;
        i += 1;
    }
    if i < (*buffer).page_count {
        while i > 0 {
            i -= 1;
            dma_unmap_page((*card).device, *dma_addrs.add(i as usize), PAGE_SIZE,
                           (*buffer).direction);
        }
        return -ENOMEM;
    }

    (*buffer).direction = direction;
    (*buffer).dma_addrs = dma_addrs;
    0
}

pub unsafe fn fw_iso_buffer_init(buffer: *mut fw_iso_buffer, card: *mut fw_card,
                                  page_count: i32, direction: dma_data_direction) -> i32 {
    let mut ret = fw_iso_buffer_alloc(buffer, page_count);
    if ret < 0 {
        return ret;
    }
    ret = fw_iso_buffer_map_dma(buffer, card, direction);
    if ret < 0 {
        fw_iso_buffer_destroy(buffer, card);
    }
    ret
}

pub unsafe fn fw_iso_buffer_destroy(buffer: *mut fw_iso_buffer, card: *mut fw_card) {
    if !(*buffer).dma_addrs.is_null() {
        for i in 0..(*buffer).page_count {
            let dma_addr = *(*buffer).dma_addrs.add(i as usize);
            dma_unmap_page((*card).device, dma_addr, PAGE_SIZE, (*buffer).direction);
        }
        kfree((*buffer).dma_addrs);
        (*buffer).dma_addrs = core::ptr::null_mut();
    }
    if !(*buffer).pages.is_null() {
        release_pages((*buffer).pages, (*buffer).page_count as usize);
        kfree((*buffer).pages);
        (*buffer).pages = core::ptr::null_mut();
    }
    (*buffer).page_count = 0;
}

/* Convert DMA address to offset into virtually contiguous buffer. */
pub unsafe fn fw_iso_buffer_lookup(buffer: *mut fw_iso_buffer, completed: dma_addr_t) -> usize {
    for i in 0..(*buffer).page_count {
        let dma_addr = *(*buffer).dma_addrs.add(i as usize);
        let offset = completed as isize - dma_addr as isize;
        if offset > 0 && offset <= PAGE_SIZE as isize {
            return ((i as usize) << PAGE_SHIFT) + offset as usize;
        }
    }
    0
}

pub unsafe fn __fw_iso_context_create(card: *mut fw_card, type_: i32, channel: i32,
                                      speed: i32, header_size: usize,
                                      header_storage_size: usize, callback: fw_iso_callback,
                                      callback_data: *mut core::ffi::c_void) -> *mut fw_iso_context {
    let ctx = ((*card).driver).allocate_iso_context(card, type_, channel, header_size,
                                                    header_storage_size);
    if IS_ERR(ctx) { return ctx; }
    (*ctx).card = card; (*ctx).type_ = type_; (*ctx).channel = channel; (*ctx).speed = speed;
    (*ctx).flags = 0; (*ctx).header_size = header_size;
    (*ctx).header_storage_size = header_storage_size; (*ctx).callback = callback;
    (*ctx).callback_data = callback_data;
    trace_isoc_outbound_allocate(ctx, channel, speed);
    trace_isoc_inbound_single_allocate(ctx, channel, header_size);
    trace_isoc_inbound_multiple_allocate(ctx);
    ctx
}

pub unsafe fn fw_iso_context_destroy(ctx: *mut fw_iso_context) {
    trace_isoc_outbound_destroy(ctx); trace_isoc_inbound_single_destroy(ctx);
    trace_isoc_inbound_multiple_destroy(ctx); ((*(*ctx).card).driver).free_iso_context(ctx);
}

pub unsafe fn fw_iso_context_start(ctx: *mut fw_iso_context, cycle: i32, sync: i32, tags: i32) -> i32 {
    trace_isoc_outbound_start(ctx, cycle); trace_isoc_inbound_single_start(ctx, cycle, sync, tags);
    trace_isoc_inbound_multiple_start(ctx, cycle, sync, tags);
    ((*(*ctx).card).driver).start_iso(ctx, cycle, sync, tags)
}

pub unsafe fn fw_iso_context_set_channels(ctx: *mut fw_iso_context, channels: *mut u64) -> i32 {
    trace_isoc_inbound_multiple_channels(ctx, *channels);
    ((*(*ctx).card).driver).set_iso_channels(ctx, channels)
}

pub unsafe fn fw_iso_context_queue(ctx: *mut fw_iso_context, packet: *mut fw_iso_packet,
                                   buffer: *mut fw_iso_buffer, payload: usize) -> i32 {
    trace_isoc_outbound_queue(ctx, payload, packet); trace_isoc_inbound_single_queue(ctx, payload, packet);
    trace_isoc_inbound_multiple_queue(ctx, payload, packet);
    ((*(*ctx).card).driver).queue_iso(ctx, packet, buffer, payload)
}

pub unsafe fn fw_iso_context_queue_flush(ctx: *mut fw_iso_context) {
    trace_isoc_outbound_flush(ctx); trace_isoc_inbound_single_flush(ctx);
    trace_isoc_inbound_multiple_flush(ctx); ((*(*ctx).card).driver).flush_queue_iso(ctx);
}

/** fw_iso_context_flush_completions() - process isochronous context in current process context. */
pub unsafe fn fw_iso_context_flush_completions(ctx: *mut fw_iso_context) -> i32 {
    trace_isoc_outbound_flush_completions(ctx); trace_isoc_inbound_single_flush_completions(ctx);
    trace_isoc_inbound_multiple_flush_completions(ctx); might_sleep();
    if WARN_ON_ONCE(current_work() == &mut (*ctx).work) { return 0; }
    disable_work_sync(&mut (*ctx).work);
    let err = ((*(*ctx).card).driver).flush_iso_completions(ctx);
    enable_work(&mut (*ctx).work); err
}

pub unsafe fn fw_iso_context_stop(ctx: *mut fw_iso_context) -> i32 {
    trace_isoc_outbound_stop(ctx); trace_isoc_inbound_single_stop(ctx);
    trace_isoc_inbound_multiple_stop(ctx); might_sleep();
    if WARN_ON_ONCE(current_work() == &mut (*ctx).work) { return 0; }
    let err = ((*(*ctx).card).driver).stop_iso(ctx);
    cancel_work_sync(&mut (*ctx).work); err
}

static unsafe fn manage_bandwidth(card: *mut fw_card, irm_id: i32, generation: i32,
                                  bandwidth: i32, allocate: bool) -> i32 {
    let mut old = if allocate { BANDWIDTH_AVAILABLE_INITIAL } else { 0 };
    let mut data: [__be32; 2] = [0; 2];
    for _try in 0..5 {
        let new = if allocate { old - bandwidth } else { old + bandwidth };
        if new < 0 || new > BANDWIDTH_AVAILABLE_INITIAL { return -EBUSY; }
        data[0] = cpu_to_be32(old as u32); data[1] = cpu_to_be32(new as u32);
        match fw_run_transaction(card, TCODE_LOCK_COMPARE_SWAP, irm_id, generation, SCODE_100,
                                 CSR_REGISTER_BASE + CSR_BANDWIDTH_AVAILABLE, data.as_mut_ptr(), 8) {
            RCODE_GENERATION => return if allocate { -EAGAIN } else { bandwidth },
            RCODE_COMPLETE => { if be32_to_cpup(data.as_ptr()) as i32 == old { return bandwidth; }
                old = be32_to_cpup(data.as_ptr()) as i32; }
            _ => {}
        }
    }
    -EIO
}

static unsafe fn manage_channel(card: *mut fw_card, irm_id: i32, generation: i32,
                                channels_mask: u32, offset: u64, allocate: bool) -> i32 {
    let all = if allocate { u32::MAX } else { 0 };
    let mut old = all; let mut ret = -EIO; let mut retry = 5;
    let mut data: [__be32; 2] = [0; 2];
    let mut channel = 0;
    while channel < 32 {
        if channels_mask & (1 << channel) == 0 { channel += 1; continue; }
        ret = -EBUSY; let bit = cpu_to_be32(1 << (31 - channel));
        if (old & bit) != (all & bit) { channel += 1; continue; }
        data[0] = old; data[1] = old ^ bit;
        match fw_run_transaction(card, TCODE_LOCK_COMPARE_SWAP, irm_id, generation, SCODE_100,
                                 offset, data.as_mut_ptr(), 8) {
            RCODE_GENERATION => return if allocate { -EAGAIN } else { channel },
            RCODE_COMPLETE => { if data[0] == old { return channel; } old = data[0];
                if (data[0] & bit) == (data[1] & bit) { channel += 1; continue; } }
            _ => {}
        }
        if retry != 0 { retry -= 1; } else { ret = -EIO; }
        channel += 1;
    }
    ret
}

static unsafe fn deallocate_channel(card: *mut fw_card, irm_id: i32, generation: i32, channel: i32) {
    let mask = if channel < 32 { 1 << channel } else { 1 << (channel - 32) };
    let offset = if channel < 32 { CSR_REGISTER_BASE + CSR_CHANNELS_AVAILABLE_HI }
                 else { CSR_REGISTER_BASE + CSR_CHANNELS_AVAILABLE_LO };
    manage_channel(card, irm_id, generation, mask, offset, false);
}

pub unsafe fn fw_iso_resource_manage(card: *mut fw_card, generation: i32, channels_mask: u64,
                                     channel: *mut i32, bandwidth: *mut i32, allocate: bool) {
    let channels_hi = channels_mask as u32; let channels_lo = (channels_mask >> 32) as u32;
    let irm_id = (*(*card).irm_node).node_id; let mut c = -EINVAL;
    if channels_hi != 0 { c = manage_channel(card, irm_id, generation, channels_hi,
        CSR_REGISTER_BASE + CSR_CHANNELS_AVAILABLE_HI, allocate); }
    if channels_lo != 0 && c < 0 { c = manage_channel(card, irm_id, generation, channels_lo,
        CSR_REGISTER_BASE + CSR_CHANNELS_AVAILABLE_LO, allocate); if c >= 0 { c += 32; } }
    *channel = c;
    if allocate && channels_mask != 0 && c < 0 { *bandwidth = 0; }
    if *bandwidth == 0 { return; }
    let ret = manage_bandwidth(card, irm_id, generation, *bandwidth, allocate);
    if ret < 0 { *bandwidth = 0; }
    if allocate && ret < 0 { if c >= 0 { deallocate_channel(card, irm_id, generation, c); } *channel = ret; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
