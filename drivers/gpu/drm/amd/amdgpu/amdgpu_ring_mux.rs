/*
 * Copyright 2022 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

const AMDGPU_MUX_RESUBMIT_JIFFIES_TIMEOUT: u64 = HZ / 2;
const AMDGPU_MAX_LAST_UNSIGNALED_THRESHOLD_US: u64 = 10000;

#[repr(C)]
struct RingInfo {
    hw_pio: u32,
    ring_name: *const core::ffi::c_char,
}

static SW_RING_INFO: [RingInfo; 2] = [
    RingInfo { hw_pio: AMDGPU_RING_PRIO_DEFAULT, ring_name: b"gfx_low\0".as_ptr() as *const _ },
    RingInfo { hw_pio: AMDGPU_RING_PRIO_2, ring_name: b"gfx_high\0".as_ptr() as *const _ },
];

static mut AMDGPU_MUX_CHUNK_SLAB: *mut kmem_cache = core::ptr::null_mut();

#[inline]
unsafe fn amdgpu_ring_mux_sw_entry(
    mux: *mut amdgpu_ring_mux,
    ring: *mut amdgpu_ring,
) -> *mut amdgpu_mux_entry {
    if (*ring).entry_index < (*mux).ring_entry_size {
        (*mux).ring_entry.add((*ring).entry_index)
    } else {
        core::ptr::null_mut()
    }
}

unsafe fn amdgpu_ring_mux_copy_pkt_from_sw_ring(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring, s_start: u64, s_end: u64) {
    let start = s_start & (*ring).buf_mask;
    let end = s_end & (*ring).buf_mask;
    let real_ring = (*mux).real_ring;
    if start == end { DRM_ERROR!("no more data copied from sw ring\n"); return; }
    if start > end {
        amdgpu_ring_alloc(real_ring, ((*ring).ring_size >> 2) + end - start);
        amdgpu_ring_write_multiple(real_ring, (*ring).ring.add(start as usize) as *mut _, ((*ring).ring_size >> 2) - start);
        amdgpu_ring_write_multiple(real_ring, (*ring).ring as *mut _, end);
    } else {
        amdgpu_ring_alloc(real_ring, end - start);
        amdgpu_ring_write_multiple(real_ring, (*ring).ring.add(start as usize) as *mut _, end - start);
    }
}

unsafe fn amdgpu_mux_resubmit_chunks(mux: *mut amdgpu_ring_mux) {
    let mut e: *mut amdgpu_mux_entry = core::ptr::null_mut();
    if !(*mux).s_resubmit { return; }
    for i in 0..(*mux).num_ring_entries {
        let candidate = (*mux).ring_entry.add(i);
        if (*(*candidate).ring).hw_prio <= AMDGPU_RING_PRIO_DEFAULT { e = candidate; break; }
    }
    if e.is_null() { DRM_ERROR!("{} no low priority ring found\n", "amdgpu_mux_resubmit_chunks"); return; }
    let last_seq = atomic_read(&(*(*e).ring).fence_drv.last_seq);
    let seq = (*mux).seqno_to_resubmit;
    if last_seq < seq {
        let mut chunk = list_first_entry(&(*e).list, amdgpu_mux_chunk, entry);
        while !chunk.is_null() {
            let next = list_next_entry(chunk, entry);
            if (*chunk).sync_seq > last_seq && (*chunk).sync_seq <= seq {
                amdgpu_fence_update_start_timestamp((*e).ring, (*chunk).sync_seq, ktime_get());
                if (*chunk).sync_seq == le32_to_cpu(*((*(*e).ring).fence_drv.cpu_addr.add(2))) {
                    if (*chunk).cntl_offset <= (*(*e).ring).buf_mask { amdgpu_ring_patch_cntl((*e).ring, (*chunk).cntl_offset); }
                    if (*chunk).ce_offset <= (*(*e).ring).buf_mask { amdgpu_ring_patch_ce((*e).ring, (*chunk).ce_offset); }
                    if (*chunk).de_offset <= (*(*e).ring).buf_mask { amdgpu_ring_patch_de((*e).ring, (*chunk).de_offset); }
                }
                amdgpu_ring_mux_copy_pkt_from_sw_ring(mux, (*e).ring, (*chunk).start, (*chunk).end);
                (*mux).wptr_resubmit = (*chunk).end;
                amdgpu_ring_commit((*mux).real_ring);
            }
            chunk = next;
        }
    }
    timer_delete(&mut (*mux).resubmit_timer);
    (*mux).s_resubmit = false;
}

unsafe fn amdgpu_ring_mux_schedule_resubmit(mux: *mut amdgpu_ring_mux) { mod_timer(&mut (*mux).resubmit_timer, jiffies + AMDGPU_MUX_RESUBMIT_JIFFIES_TIMEOUT); }

unsafe fn amdgpu_mux_resubmit_fallback(t: *mut timer_list) {
    let mux = timer_container_of!(t, amdgpu_ring_mux, resubmit_timer);
    if !spin_trylock(&mut (*mux).lock) { amdgpu_ring_mux_schedule_resubmit(mux); DRM_ERROR!("reschedule resubmit\n"); return; }
    amdgpu_mux_resubmit_chunks(mux);
    spin_unlock(&mut (*mux).lock);
}

pub unsafe fn amdgpu_ring_mux_init(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring, entry_size: usize) -> i32 {
    (*mux).real_ring = ring; (*mux).num_ring_entries = 0;
    (*mux).ring_entry = kzalloc_objs::<amdgpu_mux_entry>(entry_size);
    if (*mux).ring_entry.is_null() { return -ENOMEM; }
    (*mux).ring_entry_size = entry_size; (*mux).s_resubmit = false;
    AMDGPU_MUX_CHUNK_SLAB = KMEM_CACHE!(amdgpu_mux_chunk, SLAB_HWCACHE_ALIGN);
    if AMDGPU_MUX_CHUNK_SLAB.is_null() { DRM_ERROR!("create amdgpu_mux_chunk cache failed\n"); return -ENOMEM; }
    spin_lock_init(&mut (*mux).lock); timer_setup(&mut (*mux).resubmit_timer, amdgpu_mux_resubmit_fallback, 0); 0
}

pub unsafe fn amdgpu_ring_mux_fini(mux: *mut amdgpu_ring_mux) {
    for i in 0..(*mux).num_ring_entries { let e = (*mux).ring_entry.add(i); let mut chunk = list_first_entry(&(*e).list, amdgpu_mux_chunk, entry); while !chunk.is_null() { let next = list_next_entry(chunk, entry); list_del(&mut (*chunk).entry); kmem_cache_free(AMDGPU_MUX_CHUNK_SLAB, chunk); chunk = next; } }
    kmem_cache_destroy(AMDGPU_MUX_CHUNK_SLAB); kfree((*mux).ring_entry); (*mux).ring_entry = core::ptr::null_mut(); (*mux).num_ring_entries = 0; (*mux).ring_entry_size = 0;
}

pub unsafe fn amdgpu_ring_mux_add_sw_ring(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring) -> i32 {
    if (*mux).num_ring_entries >= (*mux).ring_entry_size { DRM_ERROR!("add sw ring exceeding max entry size\n"); return -ENOENT; }
    let e = (*mux).ring_entry.add((*mux).num_ring_entries); (*ring).entry_index = (*mux).num_ring_entries; (*e).ring = ring; INIT_LIST_HEAD(&mut (*e).list); (*mux).num_ring_entries += 1; 0
}

pub unsafe fn amdgpu_ring_mux_set_wptr(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring, wptr: u64) {
    spin_lock(&mut (*mux).lock); if (*ring).hw_prio <= AMDGPU_RING_PRIO_DEFAULT { amdgpu_mux_resubmit_chunks(mux); }
    let e = amdgpu_ring_mux_sw_entry(mux, ring); if e.is_null() { DRM_ERROR!("cannot find entry for sw ring\n"); spin_unlock(&mut (*mux).lock); return; }
    if (*ring).hw_prio <= AMDGPU_RING_PRIO_DEFAULT && (*mux).pending_trailing_fence_signaled { spin_unlock(&mut (*mux).lock); return; }
    (*e).sw_cptr = (*e).sw_wptr; if (*ring).hw_prio <= AMDGPU_RING_PRIO_DEFAULT && (*e).sw_cptr < (*mux).wptr_resubmit { (*e).sw_cptr = (*mux).wptr_resubmit; } (*e).sw_wptr = wptr; (*e).start_ptr_in_hw_ring = (*mux).real_ring.wptr;
    if (*ring).hw_prio > AMDGPU_RING_PRIO_DEFAULT || (*mux).wptr_resubmit < wptr { amdgpu_ring_mux_copy_pkt_from_sw_ring(mux, ring, (*e).sw_cptr, wptr); (*e).end_ptr_in_hw_ring = (*mux).real_ring.wptr; amdgpu_ring_commit((*mux).real_ring); } else { (*e).end_ptr_in_hw_ring = (*mux).real_ring.wptr; } spin_unlock(&mut (*mux).lock);
}

pub unsafe fn amdgpu_ring_mux_get_wptr(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring) -> u64 { let e = amdgpu_ring_mux_sw_entry(mux, ring); if e.is_null() { DRM_ERROR!("cannot find entry for sw ring\n"); 0 } else { (*e).sw_wptr } }

pub unsafe fn amdgpu_ring_mux_get_rptr(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring) -> u64 {
    let e = amdgpu_ring_mux_sw_entry(mux, ring); if e.is_null() { DRM_ERROR!("no sw entry found!\n"); return 0; }
    let mut readp = amdgpu_ring_get_rptr((*mux).real_ring); let start = (*e).start_ptr_in_hw_ring & (*mux).real_ring.buf_mask; let mut end = (*e).end_ptr_in_hw_ring & (*mux).real_ring.buf_mask;
    if start > end { if readp <= end { readp += (*mux).real_ring.ring_size >> 2; } end += (*mux).real_ring.ring_size >> 2; }
    if start <= readp && readp <= end { (*e).sw_rptr = ((*e).sw_cptr + readp - start) & (*ring).buf_mask; } else if readp < start { (*e).sw_rptr = (*e).sw_cptr; } else { (*e).sw_rptr = (*e).sw_wptr; } (*e).sw_rptr
}

pub unsafe fn amdgpu_sw_ring_get_rptr_gfx(ring: *mut amdgpu_ring) -> u64 { WARN_ON!(!(*ring).is_sw_ring); amdgpu_ring_mux_get_rptr(&mut (*(*ring).adev).gfx.muxer, ring) }
pub unsafe fn amdgpu_sw_ring_get_wptr_gfx(ring: *mut amdgpu_ring) -> u64 { WARN_ON!(!(*ring).is_sw_ring); amdgpu_ring_mux_get_wptr(&mut (*(*ring).adev).gfx.muxer, ring) }
pub unsafe fn amdgpu_sw_ring_set_wptr_gfx(ring: *mut amdgpu_ring) { WARN_ON!(!(*ring).is_sw_ring); amdgpu_ring_mux_set_wptr(&mut (*(*ring).adev).gfx.muxer, ring, (*ring).wptr); }
pub unsafe fn amdgpu_sw_ring_insert_nop(ring: *mut amdgpu_ring, _count: u32) { WARN_ON!(!(*ring).is_sw_ring); }
pub unsafe fn amdgpu_sw_ring_name(idx: usize) -> *const core::ffi::c_char { if idx < SW_RING_INFO.len() { SW_RING_INFO[idx].ring_name } else { core::ptr::null() } }
pub unsafe fn amdgpu_sw_ring_priority(idx: usize) -> u32 { if idx < SW_RING_INFO.len() { SW_RING_INFO[idx].hw_pio } else { AMDGPU_RING_PRIO_DEFAULT } }

unsafe fn amdgpu_mcbp_scan(mux: *mut amdgpu_ring_mux) -> i32 { let mut need_preempt = 0; for i in 0..(*mux).num_ring_entries { let ring = (*mux).ring_entry.add(i).as_ref().unwrap().ring; if (*ring).hw_prio > AMDGPU_RING_PRIO_DEFAULT && amdgpu_fence_count_emitted(ring) > 0 { return 0; } if (*ring).hw_prio <= AMDGPU_RING_PRIO_DEFAULT && amdgpu_fence_last_unsignaled_time_us(ring) > AMDGPU_MAX_LAST_UNSIGNALED_THRESHOLD_US { need_preempt = 1; } } if need_preempt != 0 && !(*mux).s_resubmit { 1 } else { 0 } }
unsafe fn amdgpu_mcbp_trigger_preempt(mux: *mut amdgpu_ring_mux) -> i32 { spin_lock(&mut (*mux).lock); (*mux).pending_trailing_fence_signaled = true; let r = amdgpu_ring_preempt_ib((*mux).real_ring); spin_unlock(&mut (*mux).lock); r }

pub unsafe fn amdgpu_sw_ring_ib_begin(ring: *mut amdgpu_ring) { let adev = (*ring).adev; let mux = &mut (*adev).gfx.muxer; WARN_ON!(!(*ring).is_sw_ring); if (*adev).gfx.mcbp && (*ring).hw_prio > AMDGPU_RING_PRIO_DEFAULT { if amdgpu_mcbp_scan(mux) > 0 { amdgpu_mcbp_trigger_preempt(mux); } return; } amdgpu_ring_mux_start_ib(mux, ring); }
pub unsafe fn amdgpu_sw_ring_ib_end(ring: *mut amdgpu_ring) { let adev = (*ring).adev; let mux = &mut (*adev).gfx.muxer; WARN_ON!(!(*ring).is_sw_ring); if (*adev).gfx.mcbp && (*ring).hw_prio > AMDGPU_RING_PRIO_DEFAULT { return; } amdgpu_ring_mux_end_ib(mux, ring); }
pub unsafe fn amdgpu_sw_ring_ib_mark_offset(ring: *mut amdgpu_ring, ty: amdgpu_ring_mux_offset_type) { let mux = &mut (*(*ring).adev).gfx.muxer; if (*ring).hw_prio > AMDGPU_RING_PRIO_DEFAULT { return; } amdgpu_ring_mux_ib_mark_offset(mux, ring, (*ring).wptr & (*ring).buf_mask, ty); }

pub unsafe fn amdgpu_ring_mux_start_ib(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring) { spin_lock(&mut (*mux).lock); amdgpu_mux_resubmit_chunks(mux); spin_unlock(&mut (*mux).lock); let e = amdgpu_ring_mux_sw_entry(mux, ring); if e.is_null() { DRM_ERROR!("cannot find entry!\n"); return; } let chunk = kmem_cache_alloc(AMDGPU_MUX_CHUNK_SLAB, GFP_KERNEL); if chunk.is_null() { DRM_ERROR!("alloc amdgpu_mux_chunk_slab failed\n"); return; } (*chunk).start = (*ring).wptr; (*chunk).cntl_offset = (*ring).buf_mask + 1; (*chunk).de_offset = (*ring).buf_mask + 1; (*chunk).ce_offset = (*ring).buf_mask + 1; list_add_tail(&mut (*chunk).entry, &mut (*e).list); }

unsafe fn scan_and_remove_signaled_chunk(_mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring) { let e = amdgpu_ring_mux_sw_entry(_mux, ring); if e.is_null() { DRM_ERROR!("cannot find entry!\n"); return; } let last_seq = atomic_read(&(*ring).fence_drv.last_seq); let mut chunk = list_first_entry(&(*e).list, amdgpu_mux_chunk, entry); while !chunk.is_null() { let next = list_next_entry(chunk, entry); if (*chunk).sync_seq <= last_seq { list_del(&mut (*chunk).entry); kmem_cache_free(AMDGPU_MUX_CHUNK_SLAB, chunk); } chunk = next; } }

pub unsafe fn amdgpu_ring_mux_ib_mark_offset(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring, offset: u64, ty: amdgpu_ring_mux_offset_type) { let e = amdgpu_ring_mux_sw_entry(mux, ring); if e.is_null() { DRM_ERROR!("cannot find entry!\n"); return; } let chunk = list_last_entry(&(*e).list, amdgpu_mux_chunk, entry); if chunk.is_null() { DRM_ERROR!("cannot find chunk!\n"); return; } match ty { AMDGPU_MUX_OFFSET_TYPE_CONTROL => (*chunk).cntl_offset = offset, AMDGPU_MUX_OFFSET_TYPE_DE => (*chunk).de_offset = offset, AMDGPU_MUX_OFFSET_TYPE_CE => (*chunk).ce_offset = offset, _ => DRM_ERROR!("invalid type ({})\n", ty as i32), } }
pub unsafe fn amdgpu_ring_mux_end_ib(mux: *mut amdgpu_ring_mux, ring: *mut amdgpu_ring) { let e = amdgpu_ring_mux_sw_entry(mux, ring); if e.is_null() { DRM_ERROR!("cannot find entry!\n"); return; } let chunk = list_last_entry(&(*e).list, amdgpu_mux_chunk, entry); if chunk.is_null() { DRM_ERROR!("cannot find chunk!\n"); return; } (*chunk).end = (*ring).wptr; (*chunk).sync_seq = READ_ONCE!((*ring).fence_drv.sync_seq); scan_and_remove_signaled_chunk(mux, ring); }

pub unsafe fn amdgpu_mcbp_handle_trailing_fence_irq(mux: *mut amdgpu_ring_mux) -> bool { if !(*mux).pending_trailing_fence_signaled || (*mux).real_ring.trail_seq != le32_to_cpu(*(*mux).real_ring.trail_fence_cpu_addr) { return false; } let mut ring = core::ptr::null_mut(); for i in 0..(*mux).num_ring_entries { let e = (*mux).ring_entry.add(i); if (*e).ring.hw_prio <= AMDGPU_RING_PRIO_DEFAULT { ring = (*e).ring; break; } } if ring.is_null() { DRM_ERROR!("cannot find low priority ring\n"); return false; } amdgpu_fence_process(ring); if amdgpu_fence_count_emitted(ring) > 0 { (*mux).s_resubmit = true; (*mux).seqno_to_resubmit = (*ring).fence_drv.sync_seq; amdgpu_ring_mux_schedule_resubmit(mux); } (*mux).pending_trailing_fence_signaled = false; true }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
