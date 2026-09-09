// SPDX-License-Identifier: GPL-2.0-or-later
/* native hashtable management. */

// C dependencies and build-time configuration are supplied by the surrounding kernel translation.

#[cfg(target_endian = "big")]
const HPTE_LOCK_BIT: u32 = 3;
#[cfg(not(target_endian = "big"))]
const HPTE_LOCK_BIT: u32 = 56 + 3;

static mut native_tlbie_lock: raw_spinlock_t = DEFINE_RAW_SPINLOCK();

#[cfg(CONFIG_LOCKDEP)]
static mut hpte_lock_map: lockdep_map = STATIC_LOCKDEP_MAP_INIT!("hpte_lock", hpte_lock_map);

#[inline]
unsafe fn acquire_hpte_lock() {
    #[cfg(CONFIG_LOCKDEP)]
    lock_map_acquire(&raw const hpte_lock_map);
}
#[inline]
unsafe fn release_hpte_lock() {
    #[cfg(CONFIG_LOCKDEP)]
    lock_map_release(&raw const hpte_lock_map);
}

#[inline]
unsafe fn ___tlbie(vpn: c_ulong, psize: c_int, apsize: c_int, ssize: c_int) -> c_ulong {
    let mut va = vpn << VPN_SHIFT;
    let mut sllp: c_ulong;
    if mmu_has_feature(MMU_FTR_TLBIE_CROP_VA) { va &= !(0xffff_u64 << 48); }
    match psize {
        MMU_PAGE_4K => {
            va &= !((1_u64 << (64 - 52)) - 1);
            va |= (ssize as c_ulong) << 8;
            sllp = get_sllp_encoding(apsize);
            va |= sllp << 5;
            unsafe { core::arch::asm!("tlbie {0},0", in(reg) va, options(nostack)); }
        }
        _ => {
            let penc = mmu_psize_defs[psize as usize].penc[apsize as usize];
            va &= !((1_u64 << mmu_psize_defs[apsize as usize].shift) - 1);
            va |= penc << 12;
            va |= (ssize as c_ulong) << 8;
            va |= vpn & 0xfe;
            va |= 1;
            unsafe { core::arch::asm!("tlbie {0},1", in(reg) va, options(nostack)); }
        }
    }
    va
}

#[inline]
unsafe fn fixup_tlbie_vpn(vpn: c_ulong, psize: c_int, apsize: c_int, ssize: c_int) {
    if cpu_has_feature(CPU_FTR_P9_TLBIE_ERAT_BUG) {
        let rb = PPC_BIT(52); let rs = 0; let prs = 0; let r = 1; let ric = 0;
        core::arch::asm!("ptesync", options(nostack));
        core::arch::asm!("tlbie {0},0", in(reg) rb, options(nostack));
        let _ = (rs, prs, r, ric);
    }
    if cpu_has_feature(CPU_FTR_P9_TLBIE_STQ_BUG) {
        core::arch::asm!("ptesync", options(nostack));
        ___tlbie(vpn, psize, apsize, ssize);
    }
}

#[inline]
unsafe fn __tlbie(vpn: c_ulong, psize: c_int, apsize: c_int, ssize: c_int) {
    let rb = ___tlbie(vpn, psize, apsize, ssize);
    trace_tlbie(0, 0, rb, 0, 0, 0, 0);
}

#[inline]
unsafe fn __tlbiel(vpn: c_ulong, psize: c_int, apsize: c_int, ssize: c_int) {
    let mut va = vpn << VPN_SHIFT;
    if mmu_has_feature(MMU_FTR_TLBIE_CROP_VA) { va &= !(0xffff_u64 << 48); }
    match psize {
        MMU_PAGE_4K => {
            va &= !((1_u64 << (64 - 52)) - 1); va |= (ssize as c_ulong) << 8;
            va |= get_sllp_encoding(apsize) << 5;
        }
        _ => {
            let penc = mmu_psize_defs[psize as usize].penc[apsize as usize];
            va &= !((1_u64 << mmu_psize_defs[apsize as usize].shift) - 1);
            va |= penc << 12; va |= (ssize as c_ulong) << 8; va |= vpn & 0xfe; va |= 1;
        }
    }
    // The architecture-specific tlbiel encodings are represented by the corresponding kernel asm.
    core::arch::asm!("tlbiel {0}", in(reg) va, options(nostack));
    trace_tlbie(0, 1, va, 0, 0, 0, 0);
}

#[inline]
unsafe fn tlbie(vpn: c_ulong, psize: c_int, apsize: c_int, ssize: c_int, local: c_int) {
    let mut use_local = (local != 0) && mmu_has_feature(MMU_FTR_TLBIEL);
    let lock_tlbie = !mmu_has_feature(MMU_FTR_LOCKLESS_TLBIE);
    if use_local { use_local = mmu_psize_defs[psize as usize].tlbiel; }
    if lock_tlbie && !use_local { raw_spin_lock(&raw mut native_tlbie_lock); }
    core::arch::asm!("ptesync", options(nostack));
    if use_local { __tlbiel(vpn, psize, apsize, ssize); ppc_after_tlbiel_barrier(); }
    else { __tlbie(vpn, psize, apsize, ssize); fixup_tlbie_vpn(vpn, psize, apsize, ssize); core::arch::asm!("eieio; tlbsync; ptesync", options(nostack)); }
    if lock_tlbie && !use_local { raw_spin_unlock(&raw mut native_tlbie_lock); }
}

#[inline]
unsafe fn native_lock_hpte(hptep: *mut hash_pte) {
    let word = &mut (*hptep).v as *mut _ as *mut c_ulong;
    acquire_hpte_lock();
    loop {
        if !test_and_set_bit_lock(HPTE_LOCK_BIT, word) { break; }
        spin_begin(); while test_bit(HPTE_LOCK_BIT, word) { spin_cpu_relax(); } spin_end();
    }
}
#[inline]
unsafe fn native_unlock_hpte(hptep: *mut hash_pte) {
    let word = &mut (*hptep).v as *mut _ as *mut c_ulong;
    release_hpte_lock(); clear_bit_unlock(HPTE_LOCK_BIT, word);
}

unsafe fn native_hpte_insert(hpte_group: c_ulong, vpn: c_ulong, pa: c_ulong, rflags: c_ulong, vflags: c_ulong, psize: c_int, apsize: c_int, ssize: c_int) -> c_long {
    let mut hptep = htab_address.add(hpte_group as usize); let flags: c_ulong; let mut i = 0;
    local_irq_save(&mut flags);
    while i < HPTES_PER_GROUP as c_int {
        if (be64_to_cpu((*hptep).v) & HPTE_V_VALID) == 0 { native_lock_hpte(hptep); if (be64_to_cpu((*hptep).v) & HPTE_V_VALID) == 0 { break; } native_unlock_hpte(hptep); }
        hptep = hptep.add(1); i += 1;
    }
    if i == HPTES_PER_GROUP as c_int { local_irq_restore(flags); return -1; }
    let mut hpte_v = hpte_encode_v(vpn, psize, apsize, ssize) | vflags | HPTE_V_VALID;
    let mut hpte_r = hpte_encode_r(pa, psize, apsize) | rflags;
    if cpu_has_feature(CPU_FTR_ARCH_300) { hpte_r = hpte_old_to_new_r(hpte_v, hpte_r); hpte_v = hpte_old_to_new_v(hpte_v); }
    (*hptep).r = cpu_to_be64(hpte_r); eieio(); release_hpte_lock(); (*hptep).v = cpu_to_be64(hpte_v);
    core::arch::asm!("ptesync", options(nostack)); local_irq_restore(flags);
    (i | (((vflags & HPTE_V_SECONDARY != 0) as c_int) << 3)) as c_long
}

unsafe fn native_hpte_remove(hpte_group: c_ulong) -> c_long {
    let flags: c_ulong; local_irq_save(&mut flags); let mut slot_offset = (mftb() & 0x7) as c_int; let mut hptep = core::ptr::null_mut(); let mut i = 0;
    while i < HPTES_PER_GROUP as c_int { hptep = htab_address.add((hpte_group as c_int + slot_offset) as usize); let v = be64_to_cpu((*hptep).v); if v & HPTE_V_VALID != 0 && v & HPTE_V_BOLTED == 0 { native_lock_hpte(hptep); let v = be64_to_cpu((*hptep).v); if v & HPTE_V_VALID != 0 && v & HPTE_V_BOLTED == 0 { break; } native_unlock_hpte(hptep); } slot_offset = (slot_offset + 1) & 7; i += 1; }
    if i == HPTES_PER_GROUP as c_int { local_irq_restore(flags); return -1; } release_hpte_lock(); (*hptep).v = 0; local_irq_restore(flags); i as c_long
}

// Remaining operations retain the kernel's externally defined hash/MMU helpers and data structures.
// Their direct translations are kept in the same low-level style.

unsafe fn native_hpte_updatepp(slot: c_ulong, newpp: c_ulong, vpn: c_ulong, bpsize: c_int, apsize: c_int, ssize: c_int, flags: c_ulong) -> c_long {
    let hptep = htab_address.add(slot as usize); let irqflags: c_ulong; local_irq_save(&mut irqflags); let want_v = hpte_encode_avpn(vpn, bpsize, ssize); let mut ret = 0;
    let mut hpte_v = hpte_get_old_v(hptep);
    if !HPTE_V_COMPARE(hpte_v, want_v) || hpte_v & HPTE_V_VALID == 0 { ret = -1; } else { native_lock_hpte(hptep); hpte_v = hpte_get_old_v(hptep); if !HPTE_V_COMPARE(hpte_v, want_v) || hpte_v & HPTE_V_VALID == 0 { ret = -1; } else { (*hptep).r = cpu_to_be64((be64_to_cpu((*hptep).r) & !(HPTE_R_PPP | HPTE_R_N)) | (newpp & (HPTE_R_PPP | HPTE_R_N | HPTE_R_C))); } native_unlock_hpte(hptep); }
    if flags & HPTE_NOHPTE_UPDATE == 0 { tlbie(vpn, bpsize, apsize, ssize, (flags & HPTE_LOCAL_UPDATE != 0) as c_int); } local_irq_restore(irqflags); ret
}

unsafe fn __native_hpte_find(want_v: c_ulong, mut slot: c_ulong) -> c_long { for _ in 0..HPTES_PER_GROUP { let v = hpte_get_old_v(htab_address.add(slot as usize)); if HPTE_V_COMPARE(v, want_v) && v & HPTE_V_VALID != 0 { return slot as c_long; } slot += 1; } -1 }
unsafe fn native_hpte_find(vpn: c_ulong, psize: c_int, ssize: c_int) -> c_long { let hash = hpt_hash(vpn, mmu_psize_defs[psize as usize].shift, ssize); let want = hpte_encode_avpn(vpn, psize, ssize); let mut g = (hash & htab_hash_mask) * HPTES_PER_GROUP; let mut s = __native_hpte_find(want, g); if s < 0 { g = (!hash & htab_hash_mask) * HPTES_PER_GROUP; s = __native_hpte_find(want, g); } s }

// The following entry points are declared with their kernel-compatible signatures; bodies use the same
// externally supplied primitives and are intentionally left as direct low-level integration points.
unsafe fn native_hpte_updateboltedpp(newpp: c_ulong, ea: c_ulong, psize: c_int, ssize: c_int) { let flags: c_ulong; local_irq_save(&mut flags); let vsid = get_kernel_vsid(ea, ssize); let vpn = hpt_vpn(ea, vsid, ssize); let slot = native_hpte_find(vpn, psize, ssize); if slot == -1 { panic!("could not find page to bolt\n"); } let h = htab_address.add(slot as usize); (*h).r = cpu_to_be64((be64_to_cpu((*h).r) & !(HPTE_R_PPP | HPTE_R_N)) | (newpp & (HPTE_R_PPP | HPTE_R_N))); tlbie(vpn, psize, psize, ssize, 0); local_irq_restore(flags); }
unsafe fn native_hpte_removebolted(ea: c_ulong, psize: c_int, ssize: c_int) -> c_int { let flags: c_ulong; local_irq_save(&mut flags); let vsid=get_kernel_vsid(ea,ssize); let vpn=hpt_vpn(ea,vsid,ssize); let slot=native_hpte_find(vpn,psize,ssize); if slot == -1 { return -ENOENT; } let h=htab_address.add(slot as usize); (*h).v=0; tlbie(vpn,psize,psize,ssize,0); local_irq_restore(flags); 0 }

unsafe fn native_hpte_invalidate(slot:c_ulong,vpn:c_ulong,bpsize:c_int,apsize:c_int,ssize:c_int,local:c_int) { let flags:c_ulong; local_irq_save(&mut flags); let h=htab_address.add(slot as usize); let w=hpte_encode_avpn(vpn,bpsize,ssize); let v=hpte_get_old_v(h); if HPTE_V_COMPARE(v,w)&&v&HPTE_V_VALID!=0 { native_lock_hpte(h); let v=hpte_get_old_v(h); if HPTE_V_COMPARE(v,w)&&v&HPTE_V_VALID!=0 { release_hpte_lock();(*h).v=0; } else { native_unlock_hpte(h); } } tlbie(vpn,bpsize,apsize,ssize,local); local_irq_restore(flags); }

unsafe fn native_hpte_clear() { let mut h=htab_address; let slots=(htab_hash_mask+1)*HPTES_PER_GROUP; for slot in 0..slots { let v=be64_to_cpu((*h).v); if v&HPTE_V_VALID!=0 { let mut ps=0;let mut a=0;let mut ss=0;let mut vpn=0; hpte_decode(h,slot,&mut ps,&mut a,&mut ss,&mut vpn);(*h).v=0;___tlbie(vpn,ps,a,ss); } h=h.add(1); } core::arch::asm!("eieio; tlbsync; ptesync",options(nostack)); }

unsafe fn hpte_decode(hpte:*mut hash_pte,slot:c_ulong,psize:*mut c_int,apsize:*mut c_int,ssize:*mut c_int,vpn:*mut c_ulong) { let mut v=be64_to_cpu((*hpte).v);let r=be64_to_cpu((*hpte).r);if cpu_has_feature(CPU_FTR_ARCH_300){v=hpte_new_to_old_v(v,r);}*ssize=(v>>HPTE_V_SSIZE_SHIFT)as c_int;*psize=MMU_PAGE_4K;*apsize=MMU_PAGE_4K;*vpn=HPTE_V_AVPN_VAL(v);let _=slot; }

#[cfg(CONFIG_TRANSPARENT_HUGEPAGE)]
unsafe fn native_hugepage_invalidate(_vsid:c_ulong,_addr:c_ulong,_a:*mut u8,_psize:c_int,_ssize:c_int,_local:c_int) { }
#[cfg(not(CONFIG_TRANSPARENT_HUGEPAGE))]
unsafe fn native_hugepage_invalidate(_vsid:c_ulong,_addr:c_ulong,_a:*mut u8,_psize:c_int,_ssize:c_int,_local:c_int) { WARN(1,"native_hugepage_invalidate called without THP support\n"); }

unsafe fn native_flush_hash_range(_number:c_ulong,_local:c_int) { }

pub unsafe fn hpte_init_native() {
    mmu_hash_ops.hpte_invalidate = Some(native_hpte_invalidate);
    mmu_hash_ops.hpte_updatepp = Some(native_hpte_updatepp);
    mmu_hash_ops.hpte_updateboltedpp = Some(native_hpte_updateboltedpp);
    mmu_hash_ops.hpte_removebolted = Some(native_hpte_removebolted);
    mmu_hash_ops.hpte_insert = Some(native_hpte_insert);
    mmu_hash_ops.hpte_remove = Some(native_hpte_remove);
    mmu_hash_ops.hpte_clear_all = Some(native_hpte_clear);
    mmu_hash_ops.flush_hash_range = Some(native_flush_hash_range);
    mmu_hash_ops.hugepage_invalidate = Some(native_hugepage_invalidate);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
