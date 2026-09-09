// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2009 SUSE Linux Products GmbH. All rights reserved.
 *
 * Authors:
 *     Alexander Graf <agraf@suse.de>
 *     Kevin Wolf <mail@kevin-wolf.de>
 */

// External kernel declarations and build-time configuration are supplied by the surrounding tree.
const PTE_SIZE: usize = 12;

pub unsafe fn kvmppc_mmu_invalidate_pte(vcpu: *mut kvm_vcpu, pte: *mut hpte_cache) {
    mmu_hash_ops.hpte_invalidate((*pte).slot, (*pte).host_vpn, (*pte).pagesize,
        (*pte).pagesize, MMU_SEGSIZE_256M, false);
}

unsafe fn kvmppc_sid_hash(vcpu: *mut kvm_vcpu, gvsid: u64) -> u16 {
    (((gvsid >> (SID_MAP_BITS * 7)) & SID_MAP_MASK) ^
     ((gvsid >> (SID_MAP_BITS * 6)) & SID_MAP_MASK) ^
     ((gvsid >> (SID_MAP_BITS * 5)) & SID_MAP_MASK) ^
     ((gvsid >> (SID_MAP_BITS * 4)) & SID_MAP_MASK) ^
     ((gvsid >> (SID_MAP_BITS * 3)) & SID_MAP_MASK) ^
     ((gvsid >> (SID_MAP_BITS * 2)) & SID_MAP_MASK) ^
     ((gvsid >> (SID_MAP_BITS * 1)) & SID_MAP_MASK) ^
     ((gvsid >> (SID_MAP_BITS * 0)) & SID_MAP_MASK)) as u16
}

unsafe fn find_sid_vsid(vcpu: *mut kvm_vcpu, mut gvsid: u64) -> *mut kvmppc_sid_map {
    if kvmppc_get_msr(vcpu) & MSR_PR != 0 { gvsid |= VSID_PR; }
    let sid_map_mask = kvmppc_sid_hash(vcpu, gvsid);
    let map = &mut (*to_book3s(vcpu)).sid_map[sid_map_mask as usize] as *mut kvmppc_sid_map;
    if (*map).valid && (*map).guest_vsid == gvsid {
        trace_kvm_book3s_slb_found(gvsid, (*map).host_vsid); return map;
    }
    let map = &mut (*to_book3s(vcpu)).sid_map[(SID_MAP_MASK - sid_map_mask) as usize] as *mut kvmppc_sid_map;
    if (*map).valid && (*map).guest_vsid == gvsid {
        trace_kvm_book3s_slb_found(gvsid, (*map).host_vsid); return map;
    }
    trace_kvm_book3s_slb_fail(sid_map_mask, gvsid); core::ptr::null_mut()
}

pub unsafe fn kvmppc_mmu_map_page(vcpu: *mut kvm_vcpu, orig_pte: *mut kvmppc_pte, iswrite: bool) -> i32 {
    let mut hpaddr: kvm_pfn_t; let mut hash: ulong; let mut hpteg: ulong; let mut vsid = 0u64;
    let mut ret: i32; let mut rflags: i32 = 0x192; let mut vflags: i32 = 0; let mut attempt = 0;
    let mut hpsize = MMU_PAGE_4K; let mut writable = false; let mut mmu_seq: ulong;
    let kvm = (*vcpu).kvm; let mut cpte: *mut hpte_cache; let gfn = (*orig_pte).raddr >> PAGE_SHIFT;
    let mut pfn: ulong; let mut page: *mut page;
    mmu_seq = (*kvm).mmu_invalidate_seq; smp_rmb();
    pfn = kvmppc_gpa_to_pfn(vcpu, (*orig_pte).raddr, iswrite, &mut writable, &mut page);
    if is_error_noslot_pfn(pfn) { printk(KERN_INFO, b"Couldn't get guest page for gpa %lx!\n", (*orig_pte).raddr); return -EINVAL; }
    hpaddr = pfn << PAGE_SHIFT;
    (*vcpu).arch.mmu.esid_to_vsid(vcpu, (*orig_pte).eaddr >> SID_SHIFT, &mut vsid);
    let mut map = find_sid_vsid(vcpu, vsid);
    if map.is_null() { ret = kvmppc_mmu_map_segment(vcpu, (*orig_pte).eaddr); WARN_ON(ret < 0); map = find_sid_vsid(vcpu, vsid); }
    if map.is_null() { printk(KERN_ERR, b"KVM: Segment map for 0x%llx (0x%lx) failed\n", vsid, (*orig_pte).eaddr); WARN_ON(true); return -EINVAL; }
    let vpn = hpt_vpn((*orig_pte).eaddr, (*map).host_vsid, MMU_SEGSIZE_256M);
    if !(*orig_pte).may_write || !writable { rflags |= PP_RXRX; } else { mark_page_dirty((*vcpu).kvm, gfn); }
    if !(*orig_pte).may_execute { rflags |= HPTE_R_N; } else { kvmppc_mmu_flush_icache(pfn); }
    rflags |= pte_to_hpte_pkey_bits(0, HPTE_USE_KERNEL_KEY); rflags = (rflags & !HPTE_R_WIMG) | (*orig_pte).wimg;
    if vsid & VSID_64K != 0 { hpsize = MMU_PAGE_64K; } else { hpaddr |= (*orig_pte).raddr & (!0xfff_u64 & !PAGE_MASK); }
    hash = hpt_hash(vpn, mmu_psize_defs[hpsize as usize].shift, MMU_SEGSIZE_256M);
    cpte = kvmppc_mmu_hpte_cache_next(vcpu); spin_lock(&mut (*kvm).mmu_lock);
    if cpte.is_null() || mmu_invalidate_retry(kvm, mmu_seq) { spin_unlock(&mut (*kvm).mmu_lock); return -EAGAIN; }
    loop {
        hpteg = (hash & htab_hash_mask) * HPTES_PER_GROUP;
        if attempt > 1 && mmu_hash_ops.hpte_remove(hpteg) < 0 { ret = -1; break; }
        ret = mmu_hash_ops.hpte_insert(hpteg, vpn, hpaddr, rflags, vflags, hpsize, hpsize, MMU_SEGSIZE_256M);
        if ret == -1 { hash = !hash; vflags ^= HPTE_V_SECONDARY; attempt += 1; continue; }
        if ret < 0 { ret = -EIO; break; }
        trace_kvm_book3s_64_mmu_map(rflags, hpteg, vpn, hpaddr, orig_pte);
        if ret & _PTEIDX_SECONDARY != 0 && vflags & HPTE_V_SECONDARY == 0 { hash = !hash; hpteg = (hash & htab_hash_mask) * HPTES_PER_GROUP; }
        (*cpte).slot = hpteg + (ret & 7) as ulong; (*cpte).host_vpn = vpn; (*cpte).pte = *orig_pte; (*cpte).pfn = pfn; (*cpte).pagesize = hpsize;
        kvmppc_mmu_hpte_cache_map(vcpu, cpte); cpte = core::ptr::null_mut(); break;
    }
    kvm_release_faultin_page(kvm, page, false, (*orig_pte).may_write && writable); spin_unlock(&mut (*kvm).mmu_lock);
    if !cpte.is_null() { kvmppc_mmu_hpte_cache_free(cpte); } ret
}

pub unsafe fn kvmppc_mmu_unmap_page(vcpu: *mut kvm_vcpu, pte: *mut kvmppc_pte) {
    let mut mask = 0xfffffffff_u64; let mut vsid = 0u64;
    (*vcpu).arch.mmu.esid_to_vsid(vcpu, (*pte).eaddr >> SID_SHIFT, &mut vsid);
    if vsid & VSID_64K != 0 { mask = 0xffffffff0; } kvmppc_mmu_pte_vflush(vcpu, (*pte).vpage, mask);
}

unsafe fn create_sid_map(vcpu: *mut kvm_vcpu, mut gvsid: u64) -> *mut kvmppc_sid_map {
    let mut vsid_bits = VSID_BITS_65_256M; let b = to_book3s(vcpu); static mut BACKWARDS_MAP: bool = false;
    if kvmppc_get_msr(vcpu) & MSR_PR != 0 { gvsid |= VSID_PR; }
    let mut idx = kvmppc_sid_hash(vcpu, gvsid); if BACKWARDS_MAP { idx = SID_MAP_MASK - idx; } let map = &mut (*b).sid_map[idx as usize] as *mut kvmppc_sid_map; BACKWARDS_MAP = !BACKWARDS_MAP;
    if (*b).proto_vsid_next == (*b).proto_vsid_max { (*b).proto_vsid_next = (*b).proto_vsid_first; memset((*b).sid_map.as_mut_ptr(), 0, core::mem::size_of::<kvmppc_sid_map>() * SID_MAP_NUM); kvmppc_mmu_pte_flush(vcpu, 0, 0); kvmppc_mmu_flush_segments(vcpu); }
    if mmu_has_feature(MMU_FTR_68_BIT_VA) { vsid_bits = VSID_BITS_256M; }
    (*map).host_vsid = vsid_scramble({ let x = (*b).proto_vsid_next; (*b).proto_vsid_next += 1; x }, VSID_MULTIPLIER_256M, vsid_bits); (*map).guest_vsid = gvsid; (*map).valid = true; trace_kvm_book3s_slb_map(idx, gvsid, (*map).host_vsid); map
}

pub unsafe fn kvmppc_mmu_next_segment(vcpu: *mut kvm_vcpu, esid: ulong) -> i32 {
    let s = svcpu_get(vcpu); let mut found = -1; let mut i = 0; while i < (*s).slb_max { if (*s).slb[i as usize].esid & SLB_ESID_V == 0 { found = i; } else if (*s).slb[i as usize].esid & ESID_MASK == esid { svcpu_put(s); return i; } i += 1; }
    if found >= 0 { svcpu_put(s); return found; } let max = if mmu_slb_size < 64 { mmu_slb_size } else { 64 }; if (*s).slb_max == max { kvmppc_mmu_flush_segments(vcpu); } let r = (*s).slb_max; (*s).slb_max += 1; svcpu_put(s); r
}

pub unsafe fn kvmppc_mmu_map_segment(vcpu: *mut kvm_vcpu, eaddr: ulong) -> i32 {
    let s = svcpu_get(vcpu); let esid = eaddr >> SID_SHIFT; let mut slb_esid = (eaddr & ESID_MASK) | SLB_ESID_V; let mut slb_vsid = SLB_VSID_USER; let mut gvsid = 0; let idx = kvmppc_mmu_next_segment(vcpu, eaddr & ESID_MASK); let mut r = 0;
    if (*vcpu).arch.mmu.esid_to_vsid(vcpu, esid, &mut gvsid) { (*s).slb[idx as usize].esid = 0; svcpu_put(s); return -ENOENT; }
    let mut map = find_sid_vsid(vcpu, gvsid); if map.is_null() { map = create_sid_map(vcpu, gvsid); } (*map).guest_esid = esid; slb_vsid |= (*map).host_vsid << 12; slb_vsid &= !SLB_VSID_KP; slb_esid |= idx as u64; (*s).slb[idx as usize].esid = slb_esid; (*s).slb[idx as usize].vsid = slb_vsid; trace_kvm_book3s_slbmte(slb_vsid, slb_esid); svcpu_put(s); r
}

pub unsafe fn kvmppc_mmu_flush_segment(vcpu: *mut kvm_vcpu, ea: ulong, seg_size: ulong) { let s = svcpu_get(vcpu); let mask = 0usize.wrapping_sub(seg_size as usize) as ulong; for i in 0..(*s).slb_max { if (*s).slb[i as usize].esid & SLB_ESID_V != 0 && (*s).slb[i as usize].esid & mask == ea { (*s).slb[i as usize].esid = 0; } } svcpu_put(s); }
pub unsafe fn kvmppc_mmu_flush_segments(vcpu: *mut kvm_vcpu) { let s = svcpu_get(vcpu); (*s).slb_max = 0; (*s).slb[0].esid = 0; svcpu_put(s); }
pub unsafe fn kvmppc_mmu_destroy_pr(vcpu: *mut kvm_vcpu) { kvmppc_mmu_hpte_destroy(vcpu); __destroy_context((*to_book3s(vcpu)).context_id[0]); }
pub unsafe fn kvmppc_mmu_init_pr(vcpu: *mut kvm_vcpu) -> i32 { let b = to_book3s(vcpu); let err = hash__alloc_context_id(); if err < 0 { return -1; } (*b).context_id[0] = err; (*b).proto_vsid_max = (((err + 1) as u64) << ESID_BITS) - 1; (*b).proto_vsid_first = (err as u64) << ESID_BITS; (*b).proto_vsid_next = (*b).proto_vsid_first; kvmppc_mmu_hpte_init(vcpu); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
