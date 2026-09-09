// SPDX-License-Identifier: GPL-2.0
/*
 * f2fs iostat support
 *
 * Copyright 2021 Google LLC
 * Author: Daeho Jeong <daehojeong@google.com>
 */

// Linux and f2fs dependencies supplied by the surrounding translation unit.

static mut bio_iostat_ctx_cache: *mut kmem_cache = core::ptr::null_mut();
static mut bio_iostat_ctx_pool: *mut mempool_t = core::ptr::null_mut();

#[inline]
unsafe fn iostat_get_avg_bytes(sbi: *mut f2fs_sb_info, type_: enum_iostat_type) -> u64 {
    if (*sbi).iostat_count[type_ as usize] != 0 {
        div64_u64((*sbi).iostat_bytes[type_ as usize], (*sbi).iostat_count[type_ as usize])
    } else {
        0
    }
}

macro_rules! iostat_info_show {
    ($seq:expr, $sbi:expr, $name:expr, $type_:expr) => {
        seq_printf(
            $seq,
            b"%-23s %-16llu %-16llu %-16llu\n\0".as_ptr() as *const i8,
            concat!($name, ":\0").as_ptr(),
            (*$sbi).iostat_bytes[$type_ as usize],
            (*$sbi).iostat_count[$type_ as usize],
            iostat_get_avg_bytes($sbi, $type_),
        )
    };
}

pub unsafe fn iostat_info_seq_show(seq: *mut seq_file, _offset: *mut core::ffi::c_void) -> i32 {
    let sb = (*seq).private as *mut super_block;
    let sbi = F2FS_SB(sb);
    let mut i: i32;

    if !(*sbi).iostat_enable {
        return 0;
    }

    seq_printf(seq, b"time:\t\t%-16llu\n\0".as_ptr() as *const i8, ktime_get_real_seconds());
    seq_printf(
        seq,
        b"\t\t\t%-16s %-16s %-16s\n\0".as_ptr() as *const i8,
        b"io_bytes\0".as_ptr(), b"count\0".as_ptr(), b"avg_bytes\0".as_ptr(),
    );

    /* print app write IOs */
    seq_puts(seq, b"[WRITE]\n\0".as_ptr() as *const i8);
    iostat_info_show!(seq, sbi, "app buffered data", APP_BUFFERED_IO);
    iostat_info_show!(seq, sbi, "app direct data", APP_DIRECT_IO);
    iostat_info_show!(seq, sbi, "app mapped data", APP_MAPPED_IO);
    iostat_info_show!(seq, sbi, "app buffered cdata", APP_BUFFERED_CDATA_IO);
    iostat_info_show!(seq, sbi, "app mapped cdata", APP_MAPPED_CDATA_IO);

    /* print fs write IOs */
    iostat_info_show!(seq, sbi, "fs data", FS_DATA_IO);
    iostat_info_show!(seq, sbi, "fs cdata", FS_CDATA_IO);
    iostat_info_show!(seq, sbi, "fs node", FS_NODE_IO);
    iostat_info_show!(seq, sbi, "fs meta", FS_META_IO);
    iostat_info_show!(seq, sbi, "fs gc data", FS_GC_DATA_IO);
    iostat_info_show!(seq, sbi, "fs gc node", FS_GC_NODE_IO);
    iostat_info_show!(seq, sbi, "fs cp data", FS_CP_DATA_IO);
    iostat_info_show!(seq, sbi, "fs cp node", FS_CP_NODE_IO);
    iostat_info_show!(seq, sbi, "fs cp meta", FS_CP_META_IO);

    /* print app read IOs */
    seq_puts(seq, b"[READ]\n\0".as_ptr() as *const i8);
    iostat_info_show!(seq, sbi, "app buffered data", APP_BUFFERED_READ_IO);
    iostat_info_show!(seq, sbi, "app direct data", APP_DIRECT_READ_IO);
    iostat_info_show!(seq, sbi, "app mapped data", APP_MAPPED_READ_IO);
    iostat_info_show!(seq, sbi, "app buffered cdata", APP_BUFFERED_CDATA_READ_IO);
    iostat_info_show!(seq, sbi, "app mapped cdata", APP_MAPPED_CDATA_READ_IO);

    /* print fs read IOs */
    iostat_info_show!(seq, sbi, "fs data", FS_DATA_READ_IO);
    iostat_info_show!(seq, sbi, "fs gc data", FS_GDATA_READ_IO);
    iostat_info_show!(seq, sbi, "fs cdata", FS_CDATA_READ_IO);
    iostat_info_show!(seq, sbi, "fs node", FS_NODE_READ_IO);
    iostat_info_show!(seq, sbi, "fs meta", FS_META_READ_IO);

    /* print read folio order stats */
    seq_printf(seq, b"%-23s\0".as_ptr() as *const i8, b"fs read folio order:\0".as_ptr());
    i = 0;
    while i < NR_PAGE_ORDERS {
        seq_printf(seq, b" %llu\0".as_ptr() as *const i8, (*sbi).iostat_read_folio_count[i as usize]);
        i += 1;
    }
    seq_putc(seq, b'\n' as i32);

    /* print other IOs */
    seq_puts(seq, b"[OTHER]\n\0".as_ptr() as *const i8);
    iostat_info_show!(seq, sbi, "fs discard", FS_DISCARD_IO);
    iostat_info_show!(seq, sbi, "fs flush", FS_FLUSH_IO);
    iostat_info_show!(seq, sbi, "fs zone reset", FS_ZONE_RESET_IO);

    0
}

#[inline]
unsafe fn __record_iostat_latency(sbi: *mut f2fs_sb_info) {
    let mut io: i32;
    let mut idx: i32;
    let mut iostat_lat: [[f2fs_iostat_latency; NR_PAGE_TYPE as usize]; MAX_IO_TYPE as usize] = core::mem::zeroed();
    let io_lat = (*sbi).iostat_io_lat;
    let mut flags: ulong = 0;

    spin_lock_irqsave(&mut (*sbi).iostat_lat_lock, &mut flags);
    idx = 0;
    while idx < MAX_IO_TYPE {
        io = 0;
        while io < NR_PAGE_TYPE {
            (*iostat_lat.as_mut_ptr().add(idx as usize).add(io as usize)).peak_lat =
                jiffies_to_msecs((*io_lat).peak_lat[idx as usize][io as usize]);
            (*iostat_lat.as_mut_ptr().add(idx as usize).add(io as usize)).cnt =
                (*io_lat).bio_cnt[idx as usize][io as usize];
            (*iostat_lat.as_mut_ptr().add(idx as usize).add(io as usize)).avg_lat =
                if (*iostat_lat.as_mut_ptr().add(idx as usize).add(io as usize)).cnt != 0 {
                    jiffies_to_msecs((*io_lat).sum_lat[idx as usize][io as usize]) /
                        (*iostat_lat.as_mut_ptr().add(idx as usize).add(io as usize)).cnt
                } else { 0 };
            (*io_lat).sum_lat[idx as usize][io as usize] = 0;
            (*io_lat).peak_lat[idx as usize][io as usize] = 0;
            (*io_lat).bio_cnt[idx as usize][io as usize] = 0;
            io += 1;
        }
        idx += 1;
    }
    spin_unlock_irqrestore(&mut (*sbi).iostat_lat_lock, flags);
    trace_f2fs_iostat_latency(sbi, iostat_lat.as_mut_ptr());
}

#[inline]
unsafe fn f2fs_record_iostat(sbi: *mut f2fs_sb_info) {
    let mut iostat_diff: [u64; NR_IO_TYPE as usize] = [0; NR_IO_TYPE as usize];
    let mut read_folio_count_diff: [u64; NR_PAGE_ORDERS as usize] = [0; NR_PAGE_ORDERS as usize];
    let mut i: i32;
    let mut flags: ulong = 0;

    if time_is_after_jiffies((*sbi).iostat_next_period) { return; }
    spin_lock_irqsave(&mut (*sbi).iostat_lock, &mut flags);
    if time_is_after_jiffies((*sbi).iostat_next_period) {
        spin_unlock_irqrestore(&mut (*sbi).iostat_lock, flags);
        return;
    }
    (*sbi).iostat_next_period = jiffies + msecs_to_jiffies((*sbi).iostat_period_ms);
    i = 0;
    while i < NR_IO_TYPE {
        iostat_diff[i as usize] = (*sbi).iostat_bytes[i as usize] - (*sbi).prev_iostat_bytes[i as usize];
        (*sbi).prev_iostat_bytes[i as usize] = (*sbi).iostat_bytes[i as usize];
        i += 1;
    }
    i = 0;
    while i < NR_PAGE_ORDERS {
        read_folio_count_diff[i as usize] = (*sbi).iostat_read_folio_count[i as usize] - (*sbi).prev_iostat_read_folio_count[i as usize];
        (*sbi).prev_iostat_read_folio_count[i as usize] = (*sbi).iostat_read_folio_count[i as usize];
        i += 1;
    }
    spin_unlock_irqrestore(&mut (*sbi).iostat_lock, flags);
    trace_f2fs_iostat(sbi, iostat_diff.as_mut_ptr(), read_folio_count_diff.as_mut_ptr());
    __record_iostat_latency(sbi);
}

pub unsafe fn f2fs_reset_iostat(sbi: *mut f2fs_sb_info) {
    let io_lat = (*sbi).iostat_io_lat;
    let mut i: i32;
    spin_lock_irq(&mut (*sbi).iostat_lock);
    i = 0;
    while i < NR_IO_TYPE { (*sbi).iostat_count[i as usize] = 0; (*sbi).iostat_bytes[i as usize] = 0; (*sbi).prev_iostat_bytes[i as usize] = 0; i += 1; }
    i = 0;
    while i < NR_PAGE_ORDERS { (*sbi).iostat_read_folio_count[i as usize] = 0; (*sbi).prev_iostat_read_folio_count[i as usize] = 0; i += 1; }
    spin_unlock_irq(&mut (*sbi).iostat_lock);
    spin_lock_irq(&mut (*sbi).iostat_lat_lock);
    memset(io_lat as *mut core::ffi::c_void, 0, core::mem::size_of::<iostat_lat_info>());
    spin_unlock_irq(&mut (*sbi).iostat_lat_lock);
}

#[inline]
unsafe fn __f2fs_update_iostat(sbi: *mut f2fs_sb_info, type_: enum_iostat_type, io_bytes: u64) {
    (*sbi).iostat_bytes[type_ as usize] += io_bytes;
    (*sbi).iostat_count[type_ as usize] += 1;
}

pub unsafe fn f2fs_update_read_folio_count(sbi: *mut f2fs_sb_info, folio: *mut folio) {
    let mut order = folio_order(folio);
    let mut flags: ulong = 0;
    if !(*sbi).iostat_enable { return; }
    if order >= NR_PAGE_ORDERS { order = NR_PAGE_ORDERS - 1; }
    spin_lock_irqsave(&mut (*sbi).iostat_lock, &mut flags);
    (*sbi).iostat_read_folio_count[order as usize] += 1;
    spin_unlock_irqrestore(&mut (*sbi).iostat_lock, flags);
    f2fs_record_iostat(sbi);
}

pub unsafe fn f2fs_update_iostat(sbi: *mut f2fs_sb_info, inode: *mut inode, type_: enum_iostat_type, io_bytes: u64) {
    let mut flags: ulong = 0;
    if !(*sbi).iostat_enable { return; }
    spin_lock_irqsave(&mut (*sbi).iostat_lock, &mut flags);
    __f2fs_update_iostat(sbi, type_, io_bytes);
    if type_ == APP_BUFFERED_IO || type_ == APP_DIRECT_IO { __f2fs_update_iostat(sbi, APP_WRITE_IO, io_bytes); }
    if type_ == APP_BUFFERED_READ_IO || type_ == APP_DIRECT_READ_IO { __f2fs_update_iostat(sbi, APP_READ_IO, io_bytes); }
    // CONFIG_F2FS_FS_COMPRESSION condition is preserved from the C source.
    #[cfg(feature = "CONFIG_F2FS_FS_COMPRESSION")]
    if !inode.is_null() && f2fs_compressed_file(inode) {
        if type_ == APP_BUFFERED_IO { __f2fs_update_iostat(sbi, APP_BUFFERED_CDATA_IO, io_bytes); }
        if type_ == APP_BUFFERED_READ_IO { __f2fs_update_iostat(sbi, APP_BUFFERED_CDATA_READ_IO, io_bytes); }
        if type_ == APP_MAPPED_READ_IO { __f2fs_update_iostat(sbi, APP_MAPPED_CDATA_READ_IO, io_bytes); }
        if type_ == APP_MAPPED_IO { __f2fs_update_iostat(sbi, APP_MAPPED_CDATA_IO, io_bytes); }
        if type_ == FS_DATA_READ_IO { __f2fs_update_iostat(sbi, FS_CDATA_READ_IO, io_bytes); }
        if type_ == FS_DATA_IO { __f2fs_update_iostat(sbi, FS_CDATA_IO, io_bytes); }
    }
    spin_unlock_irqrestore(&mut (*sbi).iostat_lock, flags);
    f2fs_record_iostat(sbi);
}

#[inline]
unsafe fn __update_iostat_latency(iostat_ctx: *mut bio_iostat_ctx, lat_type: enum_iostat_lat_type) {
    let mut ts_diff: ulong = jiffies - (*iostat_ctx).submit_ts;
    let mut page_type = (*iostat_ctx).type_;
    let sbi = (*iostat_ctx).sbi;
    let io_lat = (*sbi).iostat_io_lat;
    let mut flags: ulong = 0;
    if !(*sbi).iostat_enable { return; }
    if page_type == META_FLUSH { page_type = META; }
    else if page_type >= NR_PAGE_TYPE { f2fs_warn(sbi, b"%s: %d over NR_PAGE_TYPE\0".as_ptr() as *const i8, b"__update_iostat_latency\0".as_ptr(), page_type); return; }
    spin_lock_irqsave(&mut (*sbi).iostat_lat_lock, &mut flags);
    (*io_lat).sum_lat[lat_type as usize][page_type as usize] += ts_diff;
    (*io_lat).bio_cnt[lat_type as usize][page_type as usize] += 1;
    if ts_diff > (*io_lat).peak_lat[lat_type as usize][page_type as usize] { (*io_lat).peak_lat[lat_type as usize][page_type as usize] = ts_diff; }
    spin_unlock_irqrestore(&mut (*sbi).iostat_lat_lock, flags);
}

pub unsafe fn iostat_update_and_unbind_ctx(bio: *mut bio) {
    let iostat_ctx = (*bio).bi_private as *mut bio_iostat_ctx;
    let lat_type: enum_iostat_lat_type;
    if op_is_write(bio_op(bio)) { lat_type = if (*bio).bi_opf & REQ_SYNC != 0 { WRITE_SYNC_IO } else { WRITE_ASYNC_IO }; (*bio).bi_private = (*iostat_ctx).sbi as *mut core::ffi::c_void; }
    else { lat_type = READ_IO; (*bio).bi_private = (*iostat_ctx).post_read_ctx as *mut core::ffi::c_void; }
    __update_iostat_latency(iostat_ctx, lat_type);
    mempool_free(iostat_ctx as *mut core::ffi::c_void, bio_iostat_ctx_pool);
}

pub unsafe fn iostat_alloc_and_bind_ctx(sbi: *mut f2fs_sb_info, bio: *mut bio, ctx: *mut bio_post_read_ctx) {
    /* Due to the mempool, this never fails. */
    let iostat_ctx = mempool_alloc(bio_iostat_ctx_pool, GFP_NOFS) as *mut bio_iostat_ctx;
    (*iostat_ctx).sbi = sbi; (*iostat_ctx).submit_ts = 0; (*iostat_ctx).type_ = 0; (*iostat_ctx).post_read_ctx = ctx;
    (*bio).bi_private = iostat_ctx as *mut core::ffi::c_void;
}

pub unsafe fn f2fs_init_iostat_processing() -> i32 {
    bio_iostat_ctx_cache = kmem_cache_create(b"f2fs_bio_iostat_ctx\0".as_ptr() as *const i8, core::mem::size_of::<bio_iostat_ctx>(), 0, 0, None);
    if bio_iostat_ctx_cache.is_null() { return -ENOMEM; }
    bio_iostat_ctx_pool = mempool_create_slab_pool(NUM_PREALLOC_IOSTAT_CTXS, bio_iostat_ctx_cache);
    if bio_iostat_ctx_pool.is_null() { kmem_cache_destroy(bio_iostat_ctx_cache); return -ENOMEM; }
    0
}

pub unsafe fn f2fs_destroy_iostat_processing() {
    mempool_destroy(bio_iostat_ctx_pool);
    kmem_cache_destroy(bio_iostat_ctx_cache);
}

pub unsafe fn f2fs_init_iostat(sbi: *mut f2fs_sb_info) -> i32 {
    /* The f2fs_iostat tracepoint emits a fixed number of read folio order
     * buckets; make sure every order fits so none is silently dropped. */
    // BUILD_BUG_ON(NR_PAGE_ORDERS > F2FS_IOSTAT_RD_FOLIO_ORDERS);
    spin_lock_init(&mut (*sbi).iostat_lock);
    spin_lock_init(&mut (*sbi).iostat_lat_lock);
    (*sbi).iostat_enable = false;
    (*sbi).iostat_period_ms = DEFAULT_IOSTAT_PERIOD_MS;
    (*sbi).iostat_io_lat = f2fs_kzalloc(sbi, core::mem::size_of::<iostat_lat_info>(), GFP_KERNEL) as *mut iostat_lat_info;
    if (*sbi).iostat_io_lat.is_null() { return -ENOMEM; }
    0
}

pub unsafe fn f2fs_destroy_iostat(sbi: *mut f2fs_sb_info) {
    kfree((*sbi).iostat_io_lat as *mut core::ffi::c_void);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
