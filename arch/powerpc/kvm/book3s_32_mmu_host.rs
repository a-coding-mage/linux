// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 SUSE Linux Products GmbH. All rights reserved.
 *
 * Authors:
 *     Alexander Graf <agraf@suse.de>
 */

// Translated from book3s_32_mmu_host.c.  Kernel dependencies are supplied by
// the surrounding translation unit.

static mut HTAB: usize = 0;
static mut HTABMASK: u32 = 0;

pub unsafe fn kvmppc_mmu_invalidate_pte(vcpu: *mut kvm_vcpu, pte: *mut hpte_cache) {
    let pteg = (*pte).slot as *mut u32;
    core::ptr::write_volatile(pteg, 0);
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    // PowerPC tlbie/tlbsync instructions are provided by the target kernel.
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}

static unsafe fn kvmppc_sid_hash(vcpu: *mut kvm_vcpu, gvsid: u64) -> u16 {
    (((gvsid >> (SID_MAP_BITS * 7)) & SID_MAP_MASK as u64)
        ^ ((gvsid >> (SID_MAP_BITS * 6)) & SID_MAP_MASK as u64)
        ^ ((gvsid >> (SID_MAP_BITS * 5)) & SID_MAP_MASK as u64)
        ^ ((gvsid >> (SID_MAP_BITS * 4)) & SID_MAP_MASK as u64)
        ^ ((gvsid >> (SID_MAP_BITS * 3)) & SID_MAP_MASK as u64)
        ^ ((gvsid >> (SID_MAP_BITS * 2)) & SID_MAP_MASK as u64)
        ^ ((gvsid >> (SID_MAP_BITS * 1)) & SID_MAP_MASK as u64)
        ^ ((gvsid >> (SID_MAP_BITS * 0)) & SID_MAP_MASK as u64)) as u16
}

unsafe fn find_sid_vsid(vcpu: *mut kvm_vcpu, mut gvsid: u64) -> *mut kvmppc_sid_map {
    if kvmppc_get_msr(vcpu) & MSR_PR != 0 { gvsid |= VSID_PR as u64; }
    let mask = kvmppc_sid_hash(vcpu, gvsid) as usize;
    let book = to_book3s(vcpu);
    let mut map = &mut (*book).sid_map[mask] as *mut kvmppc_sid_map;
    if (*map).guest_vsid == gvsid { return map; }
    map = &mut (*book).sid_map[SID_MAP_MASK as usize - mask] as *mut kvmppc_sid_map;
    if (*map).guest_vsid == gvsid { return map; }
    core::ptr::null_mut()
}

unsafe fn kvmppc_mmu_get_pteg(vcpu: *mut kvm_vcpu, vsid: u32, eaddr: u32, primary: bool) -> *mut u32 {
    let page = (eaddr & !ESID_MASK) >> 12;
    let mut hash = (vsid ^ page) << 6;
    if !primary { hash = !hash; }
    hash &= HTABMASK;
    (HTAB | hash as usize) as *mut u32
}

pub unsafe fn kvmppc_mmu_map_page(vcpu: *mut kvm_vcpu, orig_pte: *mut kvmppc_pte, iswrite: bool) -> i32 {
    let mut page: *mut page = core::ptr::null_mut();
    let mut writable = false;
    let mut hpaddr = kvmppc_gpa_to_pfn(vcpu, (*orig_pte).raddr, iswrite, &mut writable, &mut page);
    if is_error_noslot_pfn(hpaddr) { return -EINVAL; }
    hpaddr <<= PAGE_SHIFT;
    let mut vsid = 0u64;
    ((*vcpu).arch.mmu.esid_to_vsid)(vcpu, (*orig_pte).eaddr >> SID_SHIFT, &mut vsid);
    let mut map = find_sid_vsid(vcpu, vsid);
    if map.is_null() { kvmppc_mmu_map_segment(vcpu, (*orig_pte).eaddr as usize); map = find_sid_vsid(vcpu, vsid); }
    BUG_ON(map.is_null());
    vsid = (*map).host_vsid as u64;
    let vpn = (vsid << (SID_SHIFT - VPN_SHIFT)) | (((*orig_pte).eaddr & !ESID_MASK) >> VPN_SHIFT) as u64;
    let mut rr = 0i32;
    let mut primary = false;
    let mut evict = false;
    let pteg: *mut u32;
    loop {
        if rr == 16 { primary = !primary; evict = true; rr = 0; }
        pteg = kvmppc_mmu_get_pteg(vcpu, vsid as u32, (*orig_pte).eaddr, primary);
        if !evict && core::ptr::read_volatile(pteg.add(rr as usize)) & PTE_V != 0 { rr += 2; continue; }
        break;
    }
    let pteg0 = (((*orig_pte).eaddr & 0x0fffffff) >> 22) | ((vsid as u32) << 7) | PTE_V | if primary { 0 } else { PTE_SEC };
    let mut pteg1 = hpaddr as u32 | PTE_M | PTE_R | PTE_C;
    if (*orig_pte).may_write && writable { pteg1 |= PP_RWRW; mark_page_dirty((*vcpu).kvm, (*orig_pte).raddr >> PAGE_SHIFT); } else { pteg1 |= PP_RWRX; }
    if (*orig_pte).may_execute { kvmppc_mmu_flush_icache(hpaddr >> PAGE_SHIFT); }
    local_irq_disable();
    if core::ptr::read_volatile(pteg.add(rr as usize)) != 0 { core::ptr::write_volatile(pteg.add(rr as usize), 0); core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst); }
    core::ptr::write_volatile(pteg.add((rr + 1) as usize), pteg1);
    core::ptr::write_volatile(pteg.add(rr as usize), pteg0);
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    local_irq_enable();
    let pte = kvmppc_mmu_hpte_cache_next(vcpu);
    if pte.is_null() { kvm_release_page_unused(page); return -EAGAIN; }
    (*pte).slot = pteg.add(rr as usize) as usize;
    (*pte).host_vpn = vpn;
    (*pte).pte = *orig_pte;
    (*pte).pfn = hpaddr >> PAGE_SHIFT;
    kvmppc_mmu_hpte_cache_map(vcpu, pte);
    kvm_release_page_clean(page);
    0
}

pub unsafe fn kvmppc_mmu_unmap_page(vcpu: *mut kvm_vcpu, pte: *mut kvmppc_pte) {
    kvmppc_mmu_pte_vflush(vcpu, (*pte).vpage, 0xfffffffff);
}

unsafe fn create_sid_map(vcpu: *mut kvm_vcpu, mut gvsid: u64) -> *mut kvmppc_sid_map {
    static mut BACKWARDS_MAP: bool = false;
    if kvmppc_get_msr(vcpu) & MSR_PR != 0 { gvsid |= VSID_PR as u64; }
    let mut mask = kvmppc_sid_hash(vcpu, gvsid) as usize;
    if BACKWARDS_MAP { mask = SID_MAP_MASK as usize - mask; }
    BACKWARDS_MAP = !BACKWARDS_MAP;
    let b = to_book3s(vcpu);
    if (*b).vsid_next >= VSID_POOL_SIZE { (*b).vsid_next = 0; core::ptr::write_bytes((*b).sid_map.as_mut_ptr(), 0, SID_MAP_NUM as usize); kvmppc_mmu_pte_flush(vcpu, 0, 0); kvmppc_mmu_flush_segments(vcpu); }
    let map = &mut (*b).sid_map[mask] as *mut kvmppc_sid_map;
    (*map).host_vsid = (*b).vsid_pool[(*b).vsid_next as usize]; (*b).vsid_next += 1;
    (*map).guest_vsid = gvsid; (*map).valid = true; map
}

pub unsafe fn kvmppc_mmu_map_segment(vcpu: *mut kvm_vcpu, eaddr: usize) -> i32 {
    let esid = (eaddr >> SID_SHIFT) as u32; let mut gvsid = 0u64; let svcpu = svcpu_get(vcpu); let mut r = 0;
    if ((*vcpu).arch.mmu.esid_to_vsid)(vcpu, esid, &mut gvsid) != 0 { (*svcpu).sr[esid as usize] = SR_INVALID; r = -ENOENT; } else { let mut map = find_sid_vsid(vcpu, gvsid); if map.is_null() { map = create_sid_map(vcpu, gvsid); } (*map).guest_esid = esid; (*svcpu).sr[esid as usize] = (*map).host_vsid | SR_KP; }
    svcpu_put(svcpu); r
}

pub unsafe fn kvmppc_mmu_flush_segments(vcpu: *mut kvm_vcpu) { let s = svcpu_get(vcpu); for x in (*s).sr.iter_mut() { *x = SR_INVALID; } svcpu_put(s); }

pub unsafe fn kvmppc_mmu_destroy_pr(vcpu: *mut kvm_vcpu) { kvmppc_mmu_hpte_destroy(vcpu); preempt_disable(); for i in 0..SID_CONTEXTS { __destroy_context((*to_book3s(vcpu)).context_id[i as usize]); } preempt_enable(); }

pub unsafe fn kvmppc_mmu_init_pr(vcpu: *mut kvm_vcpu) -> i32 {
    let b = to_book3s(vcpu); let mut i = 0;
    while i < SID_CONTEXTS { let err = __init_new_context(); if err < 0 { for j in 0..i { if (*b).context_id[j as usize] != 0 { __destroy_context((*b).context_id[j as usize]); } } return -1; } (*b).context_id[i as usize] = err; for j in 0..16 { (*b).vsid_pool[(i * 16 + j) as usize] = CTX_TO_VSID(err, j); } i += 1; }
    (*b).vsid_next = 0;
    let mut sdr1: usize = 0;
    // mfsdr1 is target-specific PowerPC assembly.
    HTABMASK = (((sdr1 & 0x1ff) << 16) | 0xffc0) as u32; HTAB = __va(sdr1 & 0xffff0000);
    kvmppc_mmu_hpte_init(vcpu); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
