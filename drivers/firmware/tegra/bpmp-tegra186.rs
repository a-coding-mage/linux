// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2018, NVIDIA CORPORATION.
 */

// Dependencies supplied by the surrounding kernel/Rust bindings.

#[repr(C)]
struct Tegra186Bpmp {
    parent: *mut tegra_bpmp,
    tx: Tegra186BpmpMem,
    rx: Tegra186BpmpMem,
    mbox: Tegra186BpmpMbox,
}

#[repr(C)]
struct Tegra186BpmpMem {
    pool: *mut gen_pool,
    mem: Tegra186BpmpMemUnion,
    phys: dma_addr_t,
}

#[repr(C)]
union Tegra186BpmpMemUnion {
    sram: *mut core::ffi::c_void,
    dram: *mut core::ffi::c_void,
}

#[repr(C)]
struct Tegra186BpmpMbox {
    client: mbox_client,
    channel: *mut mbox_chan,
}

unsafe fn mbox_client_to_bpmp(client: *mut mbox_client) -> *mut tegra_bpmp {
    // Equivalent to container_of(client, struct tegra186_bpmp, mbox.client).
    let priv_ = (client as *mut u8)
        .sub(core::mem::offset_of!(Tegra186Bpmp, mbox) +
             core::mem::offset_of!(Tegra186BpmpMbox, client)) as *mut Tegra186Bpmp;
    (*priv_).parent
}

unsafe fn tegra186_bpmp_is_message_ready(channel: *mut tegra_bpmp_channel) -> bool {
    let err = tegra_ivc_read_get_next_frame((*channel).ivc, &mut (*channel).ib);
    if err != 0 {
        iosys_map_clear(&mut (*channel).ib);
        return false;
    }
    true
}

unsafe fn tegra186_bpmp_is_channel_free(channel: *mut tegra_bpmp_channel) -> bool {
    let err = tegra_ivc_write_get_next_frame((*channel).ivc, &mut (*channel).ob);
    if err != 0 {
        iosys_map_clear(&mut (*channel).ob);
        return false;
    }
    true
}

unsafe fn tegra186_bpmp_ack_message(channel: *mut tegra_bpmp_channel) -> i32 {
    tegra_ivc_read_advance((*channel).ivc)
}

unsafe fn tegra186_bpmp_post_message(channel: *mut tegra_bpmp_channel) -> i32 {
    tegra_ivc_write_advance((*channel).ivc)
}

unsafe fn tegra186_bpmp_ring_doorbell(bpmp: *mut tegra_bpmp) -> i32 {
    let priv_ = (*bpmp).priv_ as *mut Tegra186Bpmp;
    let err = mbox_send_message((*priv_).mbox.channel, core::ptr::null_mut());
    if err < 0 { return err; }
    mbox_client_txdone((*priv_).mbox.channel, 0);
    0
}

unsafe extern "C" fn tegra186_bpmp_ivc_notify(_ivc: *mut tegra_ivc, data: *mut core::ffi::c_void) {
    let bpmp = data as *mut tegra_bpmp;
    let priv_ = (*bpmp).priv_ as *mut Tegra186Bpmp;
    if WARN_ON((*priv_).mbox.channel.is_null()) { return; }
    tegra186_bpmp_ring_doorbell(bpmp);
}

unsafe fn tegra186_bpmp_channel_init(channel: *mut tegra_bpmp_channel,
                                     bpmp: *mut tegra_bpmp,
                                     index: u32) -> i32 {
    let priv_ = (*bpmp).priv_ as *mut Tegra186Bpmp;
    let ivc = devm_kzalloc((*bpmp).dev, core::mem::size_of::<tegra_ivc>(), GFP_KERNEL)
        as *mut tegra_ivc;
    if ivc.is_null() { return -ENOMEM; }
    (*channel).ivc = ivc;
    let message_size = tegra_ivc_align(MSG_MIN_SZ);
    let queue_size = tegra_ivc_total_queue_size(message_size);
    let offset = queue_size.wrapping_mul(index as usize);
    let mut rx: iosys_map = core::mem::zeroed();
    let mut tx: iosys_map = core::mem::zeroed();
    if !(*priv_).rx.pool.is_null() {
        iosys_map_set_vaddr_iomem(&mut rx, (*priv_).rx.mem.sram.add(offset));
        iosys_map_set_vaddr_iomem(&mut tx, (*priv_).tx.mem.sram.add(offset));
    } else {
        iosys_map_set_vaddr(&mut rx, (*priv_).rx.mem.dram.add(offset));
        iosys_map_set_vaddr(&mut tx, (*priv_).tx.mem.dram.add(offset));
    }
    let err = tegra_ivc_init(ivc, core::ptr::null_mut(), &mut rx,
        (*priv_).rx.phys.wrapping_add(offset as dma_addr_t), &mut tx,
        (*priv_).tx.phys.wrapping_add(offset as dma_addr_t), 1, message_size,
        Some(tegra186_bpmp_ivc_notify), bpmp as *mut core::ffi::c_void);
    if err < 0 { dev_err((*bpmp).dev, "failed to setup IVC for channel %u: %d\n", index, err); return err; }
    init_completion(&mut (*channel).completion);
    (*channel).bpmp = bpmp;
    0
}

unsafe fn tegra186_bpmp_channel_reset(channel: *mut tegra_bpmp_channel) {
    tegra_ivc_reset((*channel).ivc);
    while tegra_ivc_notified((*channel).ivc) {}
}

unsafe fn tegra186_bpmp_channel_cleanup(channel: *mut tegra_bpmp_channel) { tegra_ivc_cleanup((*channel).ivc); }

unsafe extern "C" fn mbox_handle_rx(client: *mut mbox_client, _data: *mut core::ffi::c_void) {
    tegra_bpmp_handle_rx(mbox_client_to_bpmp(client));
}

unsafe fn tegra186_bpmp_teardown_channels(bpmp: *mut tegra_bpmp) {
    let priv_ = (*bpmp).priv_ as *mut Tegra186Bpmp;
    for i in 0..(*bpmp).threaded.count {
        let channel = (*bpmp).threaded_channels.add(i);
        if !(*channel).bpmp.is_null() { tegra186_bpmp_channel_cleanup(channel); }
    }
    tegra186_bpmp_channel_cleanup((*bpmp).rx_channel);
    tegra186_bpmp_channel_cleanup((*bpmp).tx_channel);
    if !(*priv_).tx.pool.is_null() {
        gen_pool_free((*priv_).tx.pool, (*priv_).tx.mem.sram as usize, 4096);
        gen_pool_free((*priv_).rx.pool, (*priv_).rx.mem.sram as usize, 4096);
    }
}

// DRAM/SRAM initialization, setup, reset, lifecycle, and ops retain the C control flow.
// External kernel helpers and types are intentionally referenced rather than implemented here.

unsafe fn tegra186_bpmp_dram_init(bpmp: *mut tegra_bpmp) -> i32 {
    let priv_ = (*bpmp).priv_ as *mut Tegra186Bpmp;
    let mut res: resource = core::mem::zeroed();
    let err = of_reserved_mem_region_to_resource((*bpmp).dev.of_node, 0, &mut res);
    if err < 0 { if err != -ENODEV { dev_warn((*bpmp).dev, "failed to parse memory region: %d\n", err); } return err; }
    let size = resource_size(&res);
    if size < SZ_8K { dev_warn((*bpmp).dev, "DRAM region must be larger than 8 KiB\n"); return -EINVAL; }
    (*priv_).tx.phys = res.start; (*priv_).rx.phys = res.start + SZ_4K;
    (*priv_).tx.mem.dram = devm_memremap((*bpmp).dev, (*priv_).tx.phys, size, MEMREMAP_WC);
    if IS_ERR((*priv_).tx.mem.dram) { let e = PTR_ERR((*priv_).tx.mem.dram); dev_warn((*bpmp).dev, "failed to map DRAM region: %d\n", e); return e; }
    (*priv_).rx.mem.dram = (*priv_).tx.mem.dram.add(SZ_4K); 0
}
unsafe fn tegra186_bpmp_sram_init(bpmp: *mut tegra_bpmp) -> i32 {
    let p = (*bpmp).priv_ as *mut Tegra186Bpmp;
    (*p).tx.pool = of_gen_pool_get((*bpmp).dev.of_node, "shmem", 0);
    if (*p).tx.pool.is_null() { dev_err((*bpmp).dev, "TX shmem pool not found\n"); return -EPROBE_DEFER; }
    (*p).tx.mem.sram = gen_pool_dma_alloc((*p).tx.pool, 4096, &mut (*p).tx.phys) as *mut _;
    if (*p).tx.mem.sram.is_null() { dev_err((*bpmp).dev, "failed to allocate from TX pool\n"); return -ENOMEM; }
    (*p).rx.pool = of_gen_pool_get((*bpmp).dev.of_node, "shmem", 1);
    if (*p).rx.pool.is_null() { gen_pool_free((*p).tx.pool, (*p).tx.mem.sram as usize, 4096); return -EPROBE_DEFER; }
    (*p).rx.mem.sram = gen_pool_dma_alloc((*p).rx.pool, 4096, &mut (*p).rx.phys) as *mut _;
    if (*p).rx.mem.sram.is_null() { gen_pool_free((*p).tx.pool, (*p).tx.mem.sram as usize, 4096); return -ENOMEM; } 0
}
unsafe fn tegra186_bpmp_setup_channels(bpmp: *mut tegra_bpmp) -> i32 {
    let mut err = tegra186_bpmp_dram_init(bpmp); if err == -ENODEV { err = tegra186_bpmp_sram_init(bpmp); if err < 0 { return err; } }
    if err < 0 { return err; }
    err = tegra186_bpmp_channel_init((*bpmp).tx_channel, bpmp, (*bpmp).soc.channels.cpu_tx.offset); if err < 0 { return err; }
    err = tegra186_bpmp_channel_init((*bpmp).rx_channel, bpmp, (*bpmp).soc.channels.cpu_rx.offset); if err < 0 { tegra186_bpmp_channel_cleanup((*bpmp).tx_channel); return err; }
    for i in 0..(*bpmp).threaded.count { err = tegra186_bpmp_channel_init((*bpmp).threaded_channels.add(i), bpmp, (*bpmp).soc.channels.thread.offset + i as u32); if err < 0 { break; } }
    if err < 0 { tegra186_bpmp_teardown_channels(bpmp); } err
}
unsafe fn tegra186_bpmp_reset_channels(bpmp: *mut tegra_bpmp) { tegra186_bpmp_channel_reset((*bpmp).tx_channel); tegra186_bpmp_channel_reset((*bpmp).rx_channel); for i in 0..(*bpmp).threaded.count { tegra186_bpmp_channel_reset((*bpmp).threaded_channels.add(i)); } }
unsafe fn tegra186_bpmp_init(bpmp: *mut tegra_bpmp) -> i32 { let p = devm_kzalloc((*bpmp).dev, core::mem::size_of::<Tegra186Bpmp>(), GFP_KERNEL) as *mut Tegra186Bpmp; if p.is_null() { return -ENOMEM; } (*p).parent = bpmp; (*bpmp).priv_ = p as *mut _; let e = tegra186_bpmp_setup_channels(bpmp); if e < 0 { return e; } (*p).mbox.client.dev = (*bpmp).dev; (*p).mbox.client.rx_callback = Some(mbox_handle_rx); (*p).mbox.client.tx_block = false; (*p).mbox.client.knows_txdone = false; (*p).mbox.channel = mbox_request_channel(&mut (*p).mbox.client, 0); if IS_ERR((*p).mbox.channel) { let e = PTR_ERR((*p).mbox.channel); tegra186_bpmp_teardown_channels(bpmp); return e; } tegra186_bpmp_reset_channels(bpmp); 0 }
unsafe fn tegra186_bpmp_deinit(bpmp: *mut tegra_bpmp) { let p = (*bpmp).priv_ as *mut Tegra186Bpmp; mbox_free_channel((*p).mbox.channel); tegra186_bpmp_teardown_channels(bpmp); }
unsafe fn tegra186_bpmp_resume(bpmp: *mut tegra_bpmp) -> i32 { tegra186_bpmp_reset_channels(bpmp); 0 }

const tegra186_bpmp_ops: tegra_bpmp_ops = tegra_bpmp_ops {
    init: Some(tegra186_bpmp_init), deinit: Some(tegra186_bpmp_deinit),
    is_response_ready: Some(tegra186_bpmp_is_message_ready),
    is_request_ready: Some(tegra186_bpmp_is_message_ready),
    ack_response: Some(tegra186_bpmp_ack_message), ack_request: Some(tegra186_bpmp_ack_message),
    is_response_channel_free: Some(tegra186_bpmp_is_channel_free),
    is_request_channel_free: Some(tegra186_bpmp_is_channel_free),
    post_response: Some(tegra186_bpmp_post_message), post_request: Some(tegra186_bpmp_post_message),
    ring_doorbell: Some(tegra186_bpmp_ring_doorbell), resume: Some(tegra186_bpmp_resume),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
