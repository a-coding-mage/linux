// SPDX-License-Identifier: GPL-2.0-only
//
// Translation of linux/arch/arm/lib/copypage-armv4mc.S.
// The Linux headers and symbols referenced by the original file are supplied
// by the surrounding kernel translation unit.

// #define minicache_pgprot __pgprot(L_PTE_PRESENT | L_PTE_YOUNG | L_PTE_MT_MINICACHE)
const MINICACHE_PGPROT: usize = L_PTE_PRESENT | L_PTE_YOUNG | L_PTE_MT_MINICACHE;

static mut MINICACHE_LOCK: raw_spinlock_t = raw_spinlock_t { _private: 0 };

/* ARMv4 mini-dcache optimised copy_user_highpage. */
unsafe fn mc_copy_user_page(mut from: *mut core::ffi::c_void,
                            mut to: *mut core::ffi::c_void) {
    let mut tmp: usize;
    let mut count = PAGE_SIZE / 64;

    // Original ARM assembly, including invalidate-D-line ordering:
    // ldmia from!, {r2, r3, ip, lr};
    // 1: mcr p15, 0, to, c7, c6, 1; stmia to!, {r2, r3, ip, lr};
    // ldmia from!, {r2, r3, ip, lr}; stmia to!, {r2, r3, ip, lr};
    // ldmia from!, {r2, r3, ip, lr}; mcr p15, 0, to, c7, c6, 1;
    // stmia to!, {r2, r3, ip, lr}; ldmia from!, {r2, r3, ip, lr};
    // subs tmp, tmp, #1; stmia to!, {r2, r3, ip, lr};
    // ldmiane from!, {r2, r3, ip, lr}; bne 1b
    while count != 0 {
        let a = (from as *const u32).read_volatile();
        let b = (from.add(4) as *const u32).read_volatile();
        let c = (from.add(8) as *const u32).read_volatile();
        let d = (from.add(12) as *const u32).read_volatile();
        (to as *mut u32).write_volatile(a);
        to.add(4).cast::<u32>().write_volatile(b);
        to.add(8).cast::<u32>().write_volatile(c);
        to.add(12).cast::<u32>().write_volatile(d);
        from = from.add(16);
        to = to.add(16);
        let a = (from as *const u32).read_volatile();
        let b = (from.add(4) as *const u32).read_volatile();
        let c = (from.add(8) as *const u32).read_volatile();
        let d = (from.add(12) as *const u32).read_volatile();
        (to as *mut u32).write_volatile(a);
        to.add(4).cast::<u32>().write_volatile(b);
        to.add(8).cast::<u32>().write_volatile(c);
        to.add(12).cast::<u32>().write_volatile(d);
        from = from.add(16);
        to = to.add(16);
        tmp = count - 1;
        count = tmp;
    }
}

pub unsafe fn v4_mc_copy_user_highpage(to: *mut page, from: *mut page,
                                       _vaddr: c_ulong, _vma: *mut vm_area_struct) {
    let src = page_folio(from);
    let kto = kmap_atomic(to);
    if !test_and_set_bit(PG_dcache_clean, &mut (*src).flags.f) {
        __flush_dcache_folio(folio_flush_mapping(src), src);
    }
    raw_spin_lock(&mut MINICACHE_LOCK);
    set_top_pte(COPYPAGE_MINICACHE, mk_pte(from, MINICACHE_PGPROT));
    mc_copy_user_page(COPYPAGE_MINICACHE as *mut core::ffi::c_void, kto);
    raw_spin_unlock(&mut MINICACHE_LOCK);
    kunmap_atomic(kto);
}

/* ARMv4 optimised clear_user_page. */
pub unsafe fn v4_mc_clear_user_highpage(page: *mut page, _vaddr: c_ulong) {
    let mut ptr = kmap_atomic(page) as *mut u32;
    let mut count = PAGE_SIZE / 64;
    // Original assembly invalidates each destination D-cache line before
    // storing four zero words twice per line pair.
    while count != 0 {
        ptr.write_volatile(0); ptr = ptr.add(1);
        ptr.write_volatile(0); ptr = ptr.add(1);
        ptr.write_volatile(0); ptr = ptr.add(1);
        ptr.write_volatile(0); ptr = ptr.add(1);
        ptr.write_volatile(0); ptr = ptr.add(1);
        ptr.write_volatile(0); ptr = ptr.add(1);
        ptr.write_volatile(0); ptr = ptr.add(1);
        ptr.write_volatile(0); ptr = ptr.add(1);
        count -= 1;
    }
    kunmap_atomic(ptr as *mut core::ffi::c_void);
}

pub static mut V4_MC_USER_FNS: cpu_user_fns = cpu_user_fns {
    cpu_clear_user_highpage: Some(v4_mc_clear_user_highpage),
    cpu_copy_user_highpage: Some(v4_mc_copy_user_highpage),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
