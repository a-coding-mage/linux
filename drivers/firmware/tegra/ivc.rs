// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2014-2016, NVIDIA CORPORATION.  All rights reserved.
 */

// Dependency declarations are supplied by the surrounding kernel bindings.

pub const TEGRA_IVC_ALIGN: usize = 64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tegra_ivc_state {
    TEGRA_IVC_STATE_ESTABLISHED = 0,
    TEGRA_IVC_STATE_SYNC,
    TEGRA_IVC_STATE_ACK,
}

#[repr(C)]
pub struct tegra_ivc_header_tx_fields { pub count: u32, pub state: u32 }
#[repr(C)]
pub union tegra_ivc_header_tx { pub fields: tegra_ivc_header_tx_fields, pub pad: [u8; TEGRA_IVC_ALIGN] }
#[repr(C)]
pub union tegra_ivc_header_rx { pub count: u32, pub pad: [u8; TEGRA_IVC_ALIGN] }
#[repr(C)]
pub struct tegra_ivc_header { pub tx: tegra_ivc_header_tx, pub rx: tegra_ivc_header_rx }

#[inline]
unsafe fn tegra_ivc_invalidate(ivc: *mut tegra_ivc, phys: dma_addr_t) {
    if (*ivc).peer.is_null() { return; }
    dma_sync_single_for_cpu((*ivc).peer, phys, TEGRA_IVC_ALIGN, DMA_FROM_DEVICE);
}

#[inline]
unsafe fn tegra_ivc_flush(ivc: *mut tegra_ivc, phys: dma_addr_t) {
    if (*ivc).peer.is_null() { return; }
    dma_sync_single_for_device((*ivc).peer, phys, TEGRA_IVC_ALIGN, DMA_TO_DEVICE);
}

#[inline]
unsafe fn tegra_ivc_empty(ivc: *mut tegra_ivc, map: *mut iosys_map) -> bool {
    // Snapshot the shared values so all security-sensitive checks use the same values.
    let tx = tegra_ivc_header_read_field(map, "tx.count");
    let rx = tegra_ivc_header_read_field(map, "rx.count");
    if tx.wrapping_sub(rx) > (*ivc).num_frames { return true; }
    tx == rx
}

#[inline]
unsafe fn tegra_ivc_full(ivc: *mut tegra_ivc, map: *mut iosys_map) -> bool {
    let tx = tegra_ivc_header_read_field(map, "tx.count");
    let rx = tegra_ivc_header_read_field(map, "rx.count");
    tx.wrapping_sub(rx) >= (*ivc).num_frames
}

#[inline]
unsafe fn tegra_ivc_available(ivc: *mut tegra_ivc, map: *mut iosys_map) -> u32 {
    let tx = tegra_ivc_header_read_field(map, "tx.count");
    let rx = tegra_ivc_header_read_field(map, "rx.count");
    tx.wrapping_sub(rx)
}

#[inline]
unsafe fn tegra_ivc_advance_tx(ivc: *mut tegra_ivc) {
    let count = tegra_ivc_header_read_field(&mut (*ivc).tx.map, "tx.count");
    tegra_ivc_header_write_field(&mut (*ivc).tx.map, "tx.count", count.wrapping_add(1));
    if (*ivc).tx.position == (*ivc).num_frames - 1 { (*ivc).tx.position = 0; } else { (*ivc).tx.position += 1; }
}

#[inline]
unsafe fn tegra_ivc_advance_rx(ivc: *mut tegra_ivc) {
    let count = tegra_ivc_header_read_field(&mut (*ivc).rx.map, "rx.count");
    tegra_ivc_header_write_field(&mut (*ivc).rx.map, "rx.count", count.wrapping_add(1));
    if (*ivc).rx.position == (*ivc).num_frames - 1 { (*ivc).rx.position = 0; } else { (*ivc).rx.position += 1; }
}

#[inline]
unsafe fn tegra_ivc_check_read(ivc: *mut tegra_ivc) -> i32 {
    let state = tegra_ivc_header_read_field(&mut (*ivc).tx.map, "tx.state");
    if state != TEGRA_IVC_STATE_ESTABLISHED as u32 { return -ECONNRESET; }
    if !tegra_ivc_empty(ivc, &mut (*ivc).rx.map) { return 0; }
    tegra_ivc_invalidate(ivc, (*ivc).rx.phys + core::mem::size_of::<u32>() * 2);
    if tegra_ivc_empty(ivc, &mut (*ivc).rx.map) { return -ENOSPC; }
    0
}

#[inline]
unsafe fn tegra_ivc_check_write(ivc: *mut tegra_ivc) -> i32 {
    let state = tegra_ivc_header_read_field(&mut (*ivc).tx.map, "tx.state");
    if state != TEGRA_IVC_STATE_ESTABLISHED as u32 { return -ECONNRESET; }
    if !tegra_ivc_full(ivc, &mut (*ivc).tx.map) { return 0; }
    tegra_ivc_invalidate(ivc, (*ivc).tx.phys + core::mem::size_of::<u32>() * 2);
    if tegra_ivc_full(ivc, &mut (*ivc).tx.map) { return -ENOSPC; }
    0
}

#[inline]
unsafe fn tegra_ivc_frame_virt(ivc: *mut tegra_ivc, header: *const iosys_map, frame: usize, map: *mut iosys_map) -> i32 {
    let offset = core::mem::size_of::<tegra_ivc_header>() + (*ivc).frame_size * frame;
    if frame >= (*ivc).num_frames { return -EINVAL; }
    *map = IOSYS_MAP_INIT_OFFSET(header, offset);
    0
}

#[inline]
unsafe fn tegra_ivc_frame_phys(ivc: *mut tegra_ivc, phys: dma_addr_t, frame: usize) -> dma_addr_t {
    phys + core::mem::size_of::<tegra_ivc_header>() as dma_addr_t + ((*ivc).frame_size * frame) as dma_addr_t
}

#[inline]
unsafe fn tegra_ivc_invalidate_frame(ivc: *mut tegra_ivc, mut phys: dma_addr_t, frame: usize, offset: usize, size: usize) {
    if (*ivc).peer.is_null() || frame >= (*ivc).num_frames { return; }
    phys = tegra_ivc_frame_phys(ivc, phys, frame) + offset as dma_addr_t;
    dma_sync_single_for_cpu((*ivc).peer, phys, size, DMA_FROM_DEVICE);
}

#[inline]
unsafe fn tegra_ivc_flush_frame(ivc: *mut tegra_ivc, mut phys: dma_addr_t, frame: usize, offset: usize, size: usize) {
    if (*ivc).peer.is_null() || frame >= (*ivc).num_frames { return; }
    phys = tegra_ivc_frame_phys(ivc, phys, frame) + offset as dma_addr_t;
    dma_sync_single_for_device((*ivc).peer, phys, size, DMA_TO_DEVICE);
}

pub unsafe fn tegra_ivc_read_get_next_frame(ivc: *mut tegra_ivc, map: *mut iosys_map) -> i32 {
    if ivc.is_null() { return -EINVAL; }
    let err = tegra_ivc_check_read(ivc); if err < 0 { return err; }
    smp_rmb();
    tegra_ivc_invalidate_frame(ivc, (*ivc).rx.phys, (*ivc).rx.position, 0, (*ivc).frame_size);
    tegra_ivc_frame_virt(ivc, &(*ivc).rx.map, (*ivc).rx.position, map)
}

pub unsafe fn tegra_ivc_read_advance(ivc: *mut tegra_ivc) -> i32 {
    let err = tegra_ivc_check_read(ivc); if err < 0 { return err; }
    tegra_ivc_advance_rx(ivc); tegra_ivc_flush(ivc, (*ivc).rx.phys + 64);
    smp_mb(); tegra_ivc_invalidate(ivc, (*ivc).rx.phys);
    if tegra_ivc_available(ivc, &mut (*ivc).rx.map) == (*ivc).num_frames as u32 - 1 { ((*ivc).notify)(ivc, (*ivc).notify_data); }
    0
}

pub unsafe fn tegra_ivc_write_get_next_frame(ivc: *mut tegra_ivc, map: *mut iosys_map) -> i32 {
    let err = tegra_ivc_check_write(ivc); if err < 0 { return err; }
    tegra_ivc_frame_virt(ivc, &(*ivc).tx.map, (*ivc).tx.position, map)
}

pub unsafe fn tegra_ivc_write_advance(ivc: *mut tegra_ivc) -> i32 {
    let err = tegra_ivc_check_write(ivc); if err < 0 { return err; }
    tegra_ivc_flush_frame(ivc, (*ivc).tx.phys, (*ivc).tx.position, 0, (*ivc).frame_size);
    smp_wmb(); tegra_ivc_advance_tx(ivc); tegra_ivc_flush(ivc, (*ivc).tx.phys);
    smp_mb(); tegra_ivc_invalidate(ivc, (*ivc).tx.phys + 64);
    if tegra_ivc_available(ivc, &mut (*ivc).tx.map) == 1 { ((*ivc).notify)(ivc, (*ivc).notify_data); }
    0
}

pub unsafe fn tegra_ivc_reset(ivc: *mut tegra_ivc) {
    tegra_ivc_header_write_field(&mut (*ivc).tx.map, "tx.state", TEGRA_IVC_STATE_SYNC as u32);
    tegra_ivc_flush(ivc, (*ivc).tx.phys); ((*ivc).notify)(ivc, (*ivc).notify_data);
}

pub unsafe fn tegra_ivc_notified(ivc: *mut tegra_ivc) -> i32 {
    tegra_ivc_invalidate(ivc, (*ivc).rx.phys);
    let rx_state = tegra_ivc_header_read_field(&mut (*ivc).rx.map, "tx.state");
    let tx_state = tegra_ivc_header_read_field(&mut (*ivc).tx.map, "tx.state");
    if rx_state == TEGRA_IVC_STATE_SYNC as u32 || (tx_state == TEGRA_IVC_STATE_SYNC as u32 && rx_state == TEGRA_IVC_STATE_ACK as u32) {
        smp_rmb(); tegra_ivc_header_write_field(&mut (*ivc).tx.map, "tx.count", 0); tegra_ivc_header_write_field(&mut (*ivc).rx.map, "rx.count", 0);
        (*ivc).tx.position = 0; (*ivc).rx.position = 0; smp_wmb();
        let state = if rx_state == TEGRA_IVC_STATE_SYNC as u32 { TEGRA_IVC_STATE_ACK } else { TEGRA_IVC_STATE_ESTABLISHED };
        tegra_ivc_header_write_field(&mut (*ivc).tx.map, "tx.state", state as u32); tegra_ivc_flush(ivc, (*ivc).tx.phys); ((*ivc).notify)(ivc, (*ivc).notify_data);
    } else if tx_state == TEGRA_IVC_STATE_ACK as u32 {
        smp_rmb(); tegra_ivc_header_write_field(&mut (*ivc).tx.map, "tx.state", TEGRA_IVC_STATE_ESTABLISHED as u32); tegra_ivc_flush(ivc, (*ivc).tx.phys); ((*ivc).notify)(ivc, (*ivc).notify_data);
    }
    if tx_state != TEGRA_IVC_STATE_ESTABLISHED as u32 { -EAGAIN } else { 0 }
}

pub fn tegra_ivc_align(size: usize) -> usize { (size + TEGRA_IVC_ALIGN - 1) & !(TEGRA_IVC_ALIGN - 1) }

pub fn tegra_ivc_total_queue_size(queue_size: usize) -> usize {
    if queue_size % TEGRA_IVC_ALIGN != 0 { return 0; }
    queue_size + core::mem::size_of::<tegra_ivc_header>()
}

pub unsafe fn tegra_ivc_init(ivc: *mut tegra_ivc, peer: *mut device, rx: *const iosys_map, rx_phys: dma_addr_t, tx: *const iosys_map, tx_phys: dma_addr_t, num_frames: usize, frame_size: usize, notify: unsafe extern "C" fn(*mut tegra_ivc, *mut core::ffi::c_void), data: *mut core::ffi::c_void) -> i32 {
    if ivc.is_null() || notify as usize == 0 { return -EINVAL; }
    if frame_size > i32::MAX as usize { return -E2BIG; }
    if frame_size % TEGRA_IVC_ALIGN != 0 || rx_phys % TEGRA_IVC_ALIGN as u64 != 0 || tx_phys % TEGRA_IVC_ALIGN as u64 != 0 { return -EINVAL; }
    (*ivc).rx.phys = if peer.is_null() { rx_phys } else { dma_map_single(peer, iosys_map_get_vaddr(rx), tegra_ivc_total_queue_size(num_frames * frame_size), DMA_BIDIRECTIONAL) };
    (*ivc).tx.phys = if peer.is_null() { tx_phys } else { dma_map_single(peer, iosys_map_get_vaddr(tx), tegra_ivc_total_queue_size(num_frames * frame_size), DMA_BIDIRECTIONAL) };
    (*ivc).rx.map = *rx; (*ivc).tx.map = *tx; (*ivc).peer = peer; (*ivc).notify = notify; (*ivc).notify_data = data; (*ivc).frame_size = frame_size; (*ivc).num_frames = num_frames; (*ivc).tx.position = 0; (*ivc).rx.position = 0; 0
}

pub unsafe fn tegra_ivc_cleanup(ivc: *mut tegra_ivc) {
    if !(*ivc).peer.is_null() { let size = tegra_ivc_total_queue_size((*ivc).num_frames * (*ivc).frame_size); dma_unmap_single((*ivc).peer, (*ivc).rx.phys, size, DMA_BIDIRECTIONAL); dma_unmap_single((*ivc).peer, (*ivc).tx.phys, size, DMA_BIDIRECTIONAL); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
