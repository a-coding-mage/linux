// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010 SUSE Linux Products GmbH. All rights reserved.
 *
 * Authors:
 *     Alexander Graf <agraf@suse.de>
 */

// Translated from book3s_mmu_hpte.c.  Kernel declarations and hash/list
// primitives are supplied by the surrounding PPC KVM environment.

const PTE_SIZE: u32 = 12;

static mut hpte_cache: *mut kmem_cache = core::ptr::null_mut();

#[inline]
unsafe fn kvmppc_mmu_hash_pte(eaddr: u64) -> u64 {
    hash_64(eaddr >> PTE_SIZE, HPTEG_HASH_BITS_PTE)
}

#[inline]
unsafe fn kvmppc_mmu_hash_pte_long(eaddr: u64) -> u64 {
    hash_64((eaddr & 0x0ffff000) >> PTE_SIZE, HPTEG_HASH_BITS_PTE_LONG)
}

#[inline]
unsafe fn kvmppc_mmu_hash_vpte(vpage: u64) -> u64 {
    hash_64(vpage & 0xfffffffff, HPTEG_HASH_BITS_VPTE)
}

#[inline]
unsafe fn kvmppc_mmu_hash_vpte_long(vpage: u64) -> u64 {
    hash_64((vpage & 0xffffff000) >> 12, HPTEG_HASH_BITS_VPTE_LONG)
}

#[cfg(CONFIG_PPC_BOOK3S_64)]
#[inline]
unsafe fn kvmppc_mmu_hash_vpte_64k(vpage: u64) -> u64 {
    hash_64((vpage & 0xffffffff0) >> 4, HPTEG_HASH_BITS_VPTE_64K)
}

pub unsafe fn kvmppc_mmu_hpte_cache_map(vcpu: *mut kvm_vcpu, pte: *mut hpte_cache) {
    let vcpu3s = to_book3s(vcpu);
    trace_kvm_book3s_mmu_map(pte);
    spin_lock(&mut (*vcpu3s).mmu_lock);

    let mut index = kvmppc_mmu_hash_pte((*pte).pte.eaddr);
    hlist_add_head_rcu(&mut (*pte).list_pte, &mut (*vcpu3s).hpte_hash_pte[index as usize]);
    index = kvmppc_mmu_hash_pte_long((*pte).pte.eaddr);
    hlist_add_head_rcu(&mut (*pte).list_pte_long, &mut (*vcpu3s).hpte_hash_pte_long[index as usize]);
    index = kvmppc_mmu_hash_vpte((*pte).pte.vpage);
    hlist_add_head_rcu(&mut (*pte).list_vpte, &mut (*vcpu3s).hpte_hash_vpte[index as usize]);
    index = kvmppc_mmu_hash_vpte_long((*pte).pte.vpage);
    hlist_add_head_rcu(&mut (*pte).list_vpte_long, &mut (*vcpu3s).hpte_hash_vpte_long[index as usize]);
    #[cfg(CONFIG_PPC_BOOK3S_64)]
    {
        index = kvmppc_mmu_hash_vpte_64k((*pte).pte.vpage);
        hlist_add_head_rcu(&mut (*pte).list_vpte_64k, &mut (*vcpu3s).hpte_hash_vpte_64k[index as usize]);
    }
    (*vcpu3s).hpte_cache_count += 1;
    spin_unlock(&mut (*vcpu3s).mmu_lock);
}

unsafe fn invalidate_pte(vcpu: *mut kvm_vcpu, pte: *mut hpte_cache) {
    let vcpu3s = to_book3s(vcpu);
    trace_kvm_book3s_mmu_invalidate(pte);
    kvmppc_mmu_invalidate_pte(vcpu, pte);
    spin_lock(&mut (*vcpu3s).mmu_lock);
    if hlist_unhashed(&(*pte).list_pte) {
        spin_unlock(&mut (*vcpu3s).mmu_lock);
        return;
    }
    hlist_del_init_rcu(&mut (*pte).list_pte);
    hlist_del_init_rcu(&mut (*pte).list_pte_long);
    hlist_del_init_rcu(&mut (*pte).list_vpte);
    hlist_del_init_rcu(&mut (*pte).list_vpte_long);
    #[cfg(CONFIG_PPC_BOOK3S_64)] hlist_del_init_rcu(&mut (*pte).list_vpte_64k);
    (*vcpu3s).hpte_cache_count -= 1;
    spin_unlock(&mut (*vcpu3s).mmu_lock);
    kfree_rcu(pte, rcu_head);
}

unsafe fn kvmppc_mmu_pte_flush_all(vcpu: *mut kvm_vcpu) {
    let vcpu3s = to_book3s(vcpu);
    rcu_read_lock();
    for i in 0..HPTEG_HASH_NUM_VPTE_LONG {
        // hlist_for_each_entry_rcu(pte, list, list_vpte_long)
        for pte in hlist_entries_rcu(&mut (*vcpu3s).hpte_hash_vpte_long[i]) {
            invalidate_pte(vcpu, pte);
        }
    }
    rcu_read_unlock();
}

unsafe fn kvmppc_mmu_pte_flush_page(vcpu: *mut kvm_vcpu, guest_ea: ulong) {
    let vcpu3s = to_book3s(vcpu);
    let list = &mut (*vcpu3s).hpte_hash_pte[kvmppc_mmu_hash_pte(guest_ea as u64) as usize];
    rcu_read_lock();
    for pte in hlist_entries_rcu(list) {
        if ((*pte).pte.eaddr & !0xfff) == guest_ea as u64 { invalidate_pte(vcpu, pte); }
    }
    rcu_read_unlock();
}

unsafe fn kvmppc_mmu_pte_flush_long(vcpu: *mut kvm_vcpu, guest_ea: ulong) {
    let vcpu3s = to_book3s(vcpu);
    let list = &mut (*vcpu3s).hpte_hash_pte_long[kvmppc_mmu_hash_pte_long(guest_ea as u64) as usize];
    rcu_read_lock();
    for pte in hlist_entries_rcu(list) {
        if ((*pte).pte.eaddr & 0x0ffff000) == guest_ea as u64 { invalidate_pte(vcpu, pte); }
    }
    rcu_read_unlock();
}

pub unsafe fn kvmppc_mmu_pte_flush(vcpu: *mut kvm_vcpu, mut guest_ea: ulong, ea_mask: ulong) {
    trace_kvm_book3s_mmu_flush(core::ptr::null(), vcpu, guest_ea, ea_mask);
    guest_ea &= ea_mask;
    match ea_mask {
        x if x == !0xfff => kvmppc_mmu_pte_flush_page(vcpu, guest_ea),
        0x0ffff000 => kvmppc_mmu_pte_flush_long(vcpu, guest_ea),
        0 => kvmppc_mmu_pte_flush_all(vcpu),
        _ => { WARN_ON(1); }
    }
}

unsafe fn kvmppc_mmu_pte_vflush(vcpu: *mut kvm_vcpu, guest_vp: u64, vp_mask: u64) {
    let vcpu3s = to_book3s(vcpu);
    let list = &mut (*vcpu3s).hpte_hash_vpte_long[kvmppc_mmu_hash_vpte_long(guest_vp) as usize];
    rcu_read_lock();
    for pte in hlist_entries_rcu(list) {
        if ((*pte).pte.vpage & vp_mask) == guest_vp { invalidate_pte(vcpu, pte); }
    }
    rcu_read_unlock();
}

pub unsafe fn kvmppc_mmu_pte_vflush_dispatch(vcpu: *mut kvm_vcpu, mut guest_vp: u64, vp_mask: u64) {
    trace_kvm_book3s_mmu_flush(b"v\0".as_ptr(), vcpu, guest_vp, vp_mask);
    guest_vp &= vp_mask;
    match vp_mask {
        0xfffffffff => { let _ = kvmppc_mmu_pte_vflush(vcpu, guest_vp, 0xfffffffff); }
        0xffffff000 => kvmppc_mmu_pte_vflush(vcpu, guest_vp, 0xffffff000),
        _ => { WARN_ON(1); }
    }
}

pub unsafe fn kvmppc_mmu_pte_pflush(vcpu: *mut kvm_vcpu, pa_start: ulong, pa_end: ulong) {
    let vcpu3s = to_book3s(vcpu);
    trace_kvm_book3s_mmu_flush(b"p\0".as_ptr(), vcpu, pa_start, pa_end);
    rcu_read_lock();
    for i in 0..HPTEG_HASH_NUM_VPTE_LONG {
        for pte in hlist_entries_rcu(&mut (*vcpu3s).hpte_hash_vpte_long[i]) {
            if (*pte).pte.raddr >= pa_start as u64 && (*pte).pte.raddr < pa_end as u64 { invalidate_pte(vcpu, pte); }
        }
    }
    rcu_read_unlock();
}

pub unsafe fn kvmppc_mmu_hpte_cache_next(vcpu: *mut kvm_vcpu) -> *mut hpte_cache {
    let vcpu3s = to_book3s(vcpu);
    if (*vcpu3s).hpte_cache_count == HPTEG_CACHE_NUM { kvmppc_mmu_pte_flush_all(vcpu); }
    kmem_cache_zalloc(hpte_cache, GFP_KERNEL)
}

pub unsafe fn kvmppc_mmu_hpte_cache_free(pte: *mut hpte_cache) { kmem_cache_free(hpte_cache, pte); }
pub unsafe fn kvmppc_mmu_hpte_destroy(vcpu: *mut kvm_vcpu) { kvmppc_mmu_pte_flush(vcpu, 0, 0); }

unsafe fn kvmppc_mmu_hpte_init_hash(hash_list: *mut hlist_head, len: usize) {
    for i in 0..len { INIT_HLIST_HEAD(hash_list.add(i)); }
}

pub unsafe fn kvmppc_mmu_hpte_init(vcpu: *mut kvm_vcpu) -> i32 {
    let vcpu3s = to_book3s(vcpu);
    kvmppc_mmu_hpte_init_hash((*vcpu3s).hpte_hash_pte.as_mut_ptr(), (*vcpu3s).hpte_hash_pte.len());
    kvmppc_mmu_hpte_init_hash((*vcpu3s).hpte_hash_pte_long.as_mut_ptr(), (*vcpu3s).hpte_hash_pte_long.len());
    kvmppc_mmu_hpte_init_hash((*vcpu3s).hpte_hash_vpte.as_mut_ptr(), (*vcpu3s).hpte_hash_vpte.len());
    kvmppc_mmu_hpte_init_hash((*vcpu3s).hpte_hash_vpte_long.as_mut_ptr(), (*vcpu3s).hpte_hash_vpte_long.len());
    #[cfg(CONFIG_PPC_BOOK3S_64)] kvmppc_mmu_hpte_init_hash((*vcpu3s).hpte_hash_vpte_64k.as_mut_ptr(), (*vcpu3s).hpte_hash_vpte_64k.len());
    spin_lock_init(&mut (*vcpu3s).mmu_lock);
    0
}

pub unsafe fn kvmppc_mmu_hpte_sysinit() -> i32 {
    hpte_cache = kmem_cache_create(b"kvm-spt\0".as_ptr(), core::mem::size_of::<hpte_cache>(), core::mem::align_of::<hpte_cache>(), 0, None);
    0
}

pub unsafe fn kvmppc_mmu_hpte_sysexit() { kmem_cache_destroy(hpte_cache); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
