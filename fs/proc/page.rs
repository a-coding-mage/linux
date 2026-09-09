// SPDX-License-Identifier: GPL-2.0

const KPMSIZE: usize = core::mem::size_of::<u64>();
const KPMMASK: usize = KPMSIZE - 1;

#[repr(C)]
#[derive(Copy, Clone)]
enum KpageOperation {
    KPAGE_FLAGS,
    KPAGE_COUNT,
    KPAGE_CGROUP,
}

unsafe fn get_max_dump_pfn() -> c_ulong {
    #[cfg(CONFIG_SPARSEMEM)]
    {
        // The memmap of early sections is completely populated and marked
        // online even if max_pfn does not fall on a section boundary -
        // pfn_to_online_page() will succeed on all pages. Allow inspecting
        // these memmaps.
        return round_up(max_pfn, PAGES_PER_SECTION);
    }
    #[cfg(not(CONFIG_SPARSEMEM))]
    {
        max_pfn
    }
}

unsafe fn get_kpage_count(page: *const page) -> u64 {
    let mut ps: page_snapshot = core::mem::zeroed();
    snapshot_page(&mut ps, page);

    #[cfg(CONFIG_PAGE_MAPCOUNT)]
    {
        folio_precise_page_mapcount(&ps.folio_snapshot, &ps.page_snapshot)
    }
    #[cfg(not(CONFIG_PAGE_MAPCOUNT))]
    {
        folio_average_page_mapcount(&ps.folio_snapshot)
    }
}

unsafe fn kpage_read(
    file: *mut file,
    buf: *mut core::ffi::c_char,
    count_in: usize,
    ppos: *mut loff_t,
    op: KpageOperation,
) -> isize {
    let max_dump_pfn: c_ulong = get_max_dump_pfn();
    let mut out = buf as *mut u64;
    let mut page: *mut page;
    let src: c_ulong = *ppos as c_ulong;
    let mut pfn: c_ulong;
    let mut count = count_in;
    let mut ret: isize = 0;
    let mut info: u64;

    pfn = src / KPMSIZE as c_ulong;
    if (src as usize & KPMMASK) != 0 || (count & KPMMASK) != 0 {
        return -EINVAL as isize;
    }
    if src >= max_dump_pfn * KPMSIZE as c_ulong {
        return 0;
    }
    count = core::cmp::min(
        count as c_ulong,
        max_dump_pfn * KPMSIZE as c_ulong - src,
    ) as usize;

    while count > 0 {
        // TODO: ZONE_DEVICE support requires to identify
        // memmaps that were actually initialized.
        page = pfn_to_online_page(pfn);

        if !page.is_null() {
            info = match op {
                KpageOperation::KPAGE_FLAGS => stable_page_flags(page),
                KpageOperation::KPAGE_COUNT => get_kpage_count(page),
                KpageOperation::KPAGE_CGROUP => page_cgroup_ino(page),
            };
        } else {
            info = 0;
        }

        if put_user(info, out) != 0 {
            ret = -EFAULT as isize;
            break;
        }

        pfn = pfn.wrapping_add(1);
        out = out.add(1);
        count -= KPMSIZE;

        cond_resched();
    }

    *ppos += (out as *mut core::ffi::c_char).offset_from(buf) as loff_t;
    if ret == 0 {
        ret = (out as *mut core::ffi::c_char).offset_from(buf) as isize;
    }
    ret
}

/* /proc/kpagecount - an array exposing page mapcounts
 *
 * Each entry is a u64 representing the corresponding
 * physical page mapcount.
 */
unsafe fn kpagecount_read(
    file: *mut file,
    buf: *mut core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> isize {
    kpage_read(file, buf, count, ppos, KpageOperation::KPAGE_COUNT)
}

static KPAGECOUNT_PROC_OPS: proc_ops = proc_ops {
    proc_flags: PROC_ENTRY_PERMANENT,
    proc_lseek: Some(mem_lseek),
    proc_read: Some(kpagecount_read),
};

#[inline]
unsafe fn kpf_copy_bit(kflags: u64, ubit: i32, kbit: i32) -> u64 {
    ((kflags >> kbit) & 1) << ubit
}

unsafe fn stable_page_flags(page: *const page) -> u64 {
    let mut folio: *const folio;
    let mut ps: page_snapshot = core::mem::zeroed();
    let k: c_ulong;
    let mut u: u64 = 0;

    /*
     * pseudo flag: KPF_NOPAGE
     * it differentiates a memory hole from a page with no flags
     */
    if page.is_null() {
        return 1u64 << KPF_NOPAGE;
    }

    snapshot_page(&mut ps, page);
    folio = &ps.folio_snapshot;
    k = (*folio).flags.f;

    /*
     * pseudo flags for the well known (anonymous) memory mapped pages
     */
    if folio_mapped(folio) {
        u |= 1u64 << KPF_MMAP;
    }
    if folio_test_anon(folio) {
        u |= 1u64 << KPF_ANON;
        if folio_test_ksm(folio) {
            u |= 1u64 << KPF_KSM;
        }
    }

    /*
     * compound pages: export both head/tail info
     * they together define a compound page's start/end pos and order
     */
    if ps.idx == 0 {
        u |= kpf_copy_bit(k as u64, KPF_COMPOUND_HEAD, PG_head);
    } else {
        u |= 1u64 << KPF_COMPOUND_TAIL;
    }
    if folio_test_hugetlb(folio) {
        u |= 1u64 << KPF_HUGE;
    } else if folio_test_large(folio) && folio_test_large_rmappable(folio) {
        /* Note: we indicate any THPs here, not just PMD-sized ones */
        u |= 1u64 << KPF_THP;
    } else if is_huge_zero_pfn(ps.pfn) {
        u |= 1u64 << KPF_ZERO_PAGE;
        u |= 1u64 << KPF_THP;
    } else if is_zero_pfn(ps.pfn) {
        u |= 1u64 << KPF_ZERO_PAGE;
    }

    if ps.flags & PAGE_SNAPSHOT_PG_BUDDY != 0 {
        u |= 1u64 << KPF_BUDDY;
    }
    if ps.flags & PAGE_SNAPSHOT_PG_IDLE != 0 {
        u |= 1u64 << KPF_IDLE;
    }
    if folio_test_offline(folio) {
        u |= 1u64 << KPF_OFFLINE;
    }
    if folio_test_pgtable(folio) {
        u |= 1u64 << KPF_PGTABLE;
    }
    if folio_test_slab(folio) {
        u |= 1u64 << KPF_SLAB;
    }

    u |= kpf_copy_bit(k as u64, KPF_LOCKED, PG_locked);
    u |= kpf_copy_bit(k as u64, KPF_DIRTY, PG_dirty);
    u |= kpf_copy_bit(k as u64, KPF_UPTODATE, PG_uptodate);
    u |= kpf_copy_bit(k as u64, KPF_WRITEBACK, PG_writeback);
    u |= kpf_copy_bit(k as u64, KPF_LRU, PG_lru);
    u |= kpf_copy_bit(k as u64, KPF_REFERENCED, PG_referenced);
    u |= kpf_copy_bit(k as u64, KPF_ACTIVE, PG_active);
    u |= kpf_copy_bit(k as u64, KPF_RECLAIM, PG_reclaim);

    if folio_test_swapcache(folio) {
        u |= 1u64 << KPF_SWAPCACHE;
    }
    u |= kpf_copy_bit(k as u64, KPF_SWAPBACKED, PG_swapbacked);
    u |= kpf_copy_bit(k as u64, KPF_UNEVICTABLE, PG_unevictable);
    u |= kpf_copy_bit(k as u64, KPF_MLOCKED, PG_mlocked);

    #[cfg(CONFIG_MEMORY_FAILURE)]
    {
        if u & (1u64 << KPF_HUGE) != 0 {
            u |= kpf_copy_bit(k as u64, KPF_HWPOISON, PG_hwpoison);
        } else {
            u |= kpf_copy_bit(ps.page_snapshot.flags.f as u64, KPF_HWPOISON, PG_hwpoison);
        }
    }

    u |= kpf_copy_bit(k as u64, KPF_RESERVED, PG_reserved);
    u |= kpf_copy_bit(k as u64, KPF_OWNER_2, PG_owner_2);
    u |= kpf_copy_bit(k as u64, KPF_PRIVATE, PG_private);
    u |= kpf_copy_bit(k as u64, KPF_PRIVATE_2, PG_private_2);
    u |= kpf_copy_bit(k as u64, KPF_OWNER_PRIVATE, PG_owner_priv_1);
    u |= kpf_copy_bit(k as u64, KPF_ARCH, PG_arch_1);
    #[cfg(CONFIG_ARCH_USES_PG_ARCH_2)]
    {
        u |= kpf_copy_bit(k as u64, KPF_ARCH_2, PG_arch_2);
    }
    #[cfg(CONFIG_ARCH_USES_PG_ARCH_3)]
    {
        u |= kpf_copy_bit(k as u64, KPF_ARCH_3, PG_arch_3);
    }

    u
}

/* /proc/kpageflags - an array exposing page flags
 *
 * Each entry is a u64 representing the corresponding
 * physical page flags.
 */
unsafe fn kpageflags_read(
    file: *mut file,
    buf: *mut core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> isize {
    kpage_read(file, buf, count, ppos, KpageOperation::KPAGE_FLAGS)
}

static KPAGEFLAGS_PROC_OPS: proc_ops = proc_ops {
    proc_flags: PROC_ENTRY_PERMANENT,
    proc_lseek: Some(mem_lseek),
    proc_read: Some(kpageflags_read),
};

#[cfg(CONFIG_MEMCG)]
unsafe fn kpagecgroup_read(
    file: *mut file,
    buf: *mut core::ffi::c_char,
    count: usize,
    ppos: *mut loff_t,
) -> isize {
    kpage_read(file, buf, count, ppos, KpageOperation::KPAGE_CGROUP)
}

#[cfg(CONFIG_MEMCG)]
static KPAGECGROUP_PROC_OPS: proc_ops = proc_ops {
    proc_flags: PROC_ENTRY_PERMANENT,
    proc_lseek: Some(mem_lseek),
    proc_read: Some(kpagecgroup_read),
};

unsafe fn proc_page_init() -> i32 {
    proc_create("kpagecount", S_IRUSR, core::ptr::null_mut(), &KPAGECOUNT_PROC_OPS);
    proc_create("kpageflags", S_IRUSR, core::ptr::null_mut(), &KPAGEFLAGS_PROC_OPS);
    #[cfg(CONFIG_MEMCG)]
    proc_create("kpagecgroup", S_IRUSR, core::ptr::null_mut(), &KPAGECGROUP_PROC_OPS);
    0
}

// fs_initcall(proc_page_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
