// SPDX-License-Identifier: GPL-2.0-only
/* Benchmark bitmap, IDA and Maple Tree allocation of variable-sized regions. */

// Kernel headers and module declarations are supplied by the surrounding kernel build.

const REGION_MAX_SIZE: usize = 32;

static mut BITMAP: *mut c_ulong = core::ptr::null_mut();
/* One more request guarantees that even an all-ones trace reaches ENOSPC. */
static mut REG_SZ: *mut u8 = core::ptr::null_mut();
static mut REG_IDX: *mut c_ulong = core::ptr::null_mut();
static mut CAPACITIES: [c_ulong; 64] = [1000000, 100000, 10000, 1000, 100, 10,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
static mut CAP_CNT: c_uint = 6;

// module_param_array(capacities, ulong, &cap_cnt, 0400);
// MODULE_PARM_DESC(capacities, "Region capacities to benchmark");

unsafe fn benchmark_bitmap(cap: c_ulong) -> c_ulong {
    let mut cnt: c_ulong;
    let mut idx: c_ulong;
    let mut alloc_time: ktime_t;
    let mut free_time: ktime_t;
    let sz: usize;

    bitmap_zero(BITMAP, cap);
    alloc_time = ktime_get();
    cnt = 0;
    while cnt <= cap {
        idx = bitmap_find_next_zero_area(BITMAP, cap, 0, *REG_SZ.add(cnt as usize), 0);
        if idx >= cap { break; }
        *REG_IDX.add(cnt as usize) = idx;
        bitmap_set(BITMAP, idx, *REG_SZ.add(cnt as usize));
        cnt += 1;
    }
    alloc_time = ktime_get() - alloc_time;
    idx = cnt;
    free_time = ktime_get();
    while idx != 0 {
        idx -= 1;
        bitmap_clear(BITMAP, *REG_IDX.add(idx as usize), *REG_SZ.add(idx as usize));
    }
    free_time = ktime_get() - free_time;
    WARN_ON(!bitmap_empty(BITMAP, cap));
    sz = BITS_TO_LONGS(cap) * core::mem::size_of::<c_ulong>();
    pr_err!("Bitmap  {:12}  {:12}  {:8}  {:8}  {:10}\n", alloc_time, free_time, cnt, cap, sz);
    cnt
}

unsafe fn ida_size(nr_ids: c_ulong) -> usize {
    let mut entries = DIV_ROUND_UP(nr_ids, IDA_BITMAP_BITS);
    let mut bitmaps = nr_ids / IDA_BITMAP_BITS;
    let mut nodes = 0;
    if nr_ids % IDA_BITMAP_BITS > BITS_PER_XA_VALUE { bitmaps += 1; }
    while entries > 1 { entries = DIV_ROUND_UP(entries, XA_CHUNK_SIZE); nodes += entries; }
    core::mem::size_of::<struct_ida>() + bitmaps as usize * core::mem::size_of::<struct_ida_bitmap>() + nodes as usize * core::mem::size_of::<struct_xa_node>()
}

unsafe fn benchmark_ida(cap: c_ulong) -> c_ulong {
    let mut ida = IDA_INIT!();
    let mut cnt = 0; let mut idx; let mut off; let mut nr_ids = 0;
    let mut alloc_time; let mut free_time;
    let mut id: c_int = -ENOSPC;
    alloc_time = ktime_get();
    while cnt <= cap {
        off = 0;
        while off < *REG_SZ.add(cnt as usize) as c_ulong {
            id = ida_alloc_max(&mut ida, cap - 1, GFP_KERNEL);
            if id < 0 { break; }
            if off == 0 { *REG_IDX.add(cnt as usize) = id as c_ulong; }
            off += 1;
        }
        if id < 0 {
            while off != 0 { off -= 1; ida_free(&mut ida, *REG_IDX.add(cnt as usize) + off); }
            break;
        }
        WARN_ON!(id as c_ulong != *REG_IDX.add(cnt as usize) + *REG_SZ.add(cnt as usize) as c_ulong - 1);
        nr_ids += *REG_SZ.add(cnt as usize) as c_ulong; cnt += 1;
    }
    alloc_time = ktime_get() - alloc_time;
    WARN_ON!(id != -ENOSPC); idx = cnt;
    free_time = ktime_get();
    while idx != 0 { idx -= 1; for off in 0..*REG_SZ.add(idx as usize) as c_ulong { ida_free(&mut ida, *REG_IDX.add(idx as usize) + off); } }
    free_time = ktime_get() - free_time;
    WARN_ON!(!ida_is_empty(&ida));
    pr_err!("IDA     {:12}  {:12}  {:8}  {:8}  {:10}\n", alloc_time, free_time, cnt, cap, ida_size(nr_ids));
    ida_destroy(&mut ida); cnt
}

unsafe fn benchmark_maple_tree(cap: c_ulong) -> c_ulong {
    let mut mt = MTREE_INIT!();
    let mut cnt = 0; let mut idx = 0; let mut alloc_time; let mut free_time; let mut ret;
    alloc_time = ktime_get();
    while cnt <= cap {
        ret = mtree_alloc_range(&mut mt, &mut idx, xa_mk_value(cnt + 1), *REG_SZ.add(cnt as usize) as c_ulong, 0, cap - 1, GFP_KERNEL);
        if ret != 0 { break; }
        *REG_IDX.add(cnt as usize) = idx; cnt += 1;
    }
    alloc_time = ktime_get() - alloc_time;
    WARN_ON!(ret != -EBUSY);
    idx = cnt; free_time = ktime_get();
    while idx != 0 { idx -= 1; mtree_erase(&mut mt, *REG_IDX.add(idx as usize)); }
    free_time = ktime_get() - free_time;
    WARN_ON!(!mtree_empty(&mt));
    let sz = core::mem::size_of_val(&mt) + DIV_ROUND_UP(cnt, MAPLE_ARANGE64_SLOTS) as usize * core::mem::size_of::<struct_maple_node>();
    pr_err!("Maple   {:12}  {:12}  {:8}  {:8}  {:10}\n", alloc_time, free_time, cnt, cap, sz);
    mtree_destroy(&mut mt); cnt
}

#[no_mangle]
pub unsafe extern "C" fn region_alloc_benchmark() -> c_int {
    let mut max_cap = 0; let mut i = 0; let mut ret = -ENOMEM;
    while i < CAP_CNT { if CAPACITIES[i as usize] == 0 { pr_err!("capacity must be nonzero\n"); return -EINVAL; } max_cap = core::cmp::max(max_cap, CAPACITIES[i as usize]); i += 1; }
    BITMAP = kvmalloc_array(BITS_TO_LONGS(max_cap), core::mem::size_of::<c_ulong>(), GFP_KERNEL);
    REG_SZ = kvmalloc_array(max_cap + 1, core::mem::size_of::<u8>(), GFP_KERNEL);
    REG_IDX = kvmalloc_array(max_cap, core::mem::size_of::<c_ulong>(), GFP_KERNEL);
    if BITMAP.is_null() || REG_SZ.is_null() || REG_IDX.is_null() { kvfree(REG_IDX); kvfree(REG_SZ); kvfree(BITMAP); return ret; }
    pr_err!("\nStart testing bitmap vs IDA vs Maple Tree region allocation\n");
    pr_err!("memory: bitmap is exact; IDA and Maple Tree are lower bounds\n");
    pr_err!("Type      alloc (ns)     free (ns)   regions  capacity  memory (B)\n");
    i = 0;
    while i < CAP_CNT {
        let max_size = core::cmp::min(REGION_MAX_SIZE as c_ulong, CAPACITIES[i as usize] / 10);
        let max_size = if max_size != 0 { max_size } else { 1 };
        let mut idx = 0;
        while idx <= CAPACITIES[i as usize] { *REG_SZ.add(idx as usize) = get_random_u32_below(max_size as u32) as u8 + 1; idx += 1; }
        let bitmap_count = benchmark_bitmap(CAPACITIES[i as usize]);
        let maple_count = benchmark_maple_tree(CAPACITIES[i as usize]);
        let ida_count = benchmark_ida(CAPACITIES[i as usize]);
        WARN_ON!(bitmap_count != ida_count); WARN_ON!(bitmap_count != maple_count); i += 1;
    }
    pr_info!("Region allocation benchmark complete\n"); ret = -EAGAIN; ret
}

// module_init(region_alloc_benchmark);
// MODULE_AUTHOR("Yury Norov <ynorov@nvidia.com>");
// MODULE_DESCRIPTION("Benchmark bitmap, IDA and Maple Tree region allocation");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
