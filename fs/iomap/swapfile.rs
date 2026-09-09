// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2018 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <darrick.wong@oracle.com>
 */

/* Swapfile activation */

#[repr(C)]
struct iomap_swapfile_info {
    iomap: iomap,                         /* accumulated iomap */
    sis: *mut swap_info_struct,
    lowest_ppage: u64,                    /* lowest physical addr seen (pages) */
    highest_ppage: u64,                   /* highest physical addr seen (pages) */
    nr_pages: usize,                      /* number of pages collected */
    nr_extents: i32,                      /* extent count */
    file: *mut file,
}

/*
 * Collect physical extents for this swap file.  Physical extents reported to
 * the swap code must be trimmed to align to a page boundary.  The logical
 * offset within the file is irrelevant since the swapfile code maps logical
 * page numbers of the swap device to the physical page-aligned extents.
 */
unsafe fn iomap_swapfile_add_extent(isi: *mut iomap_swapfile_info) -> i32 {
    let iomap = &mut (*isi).iomap;
    let mut nr_pages: usize;
    let max_pages: usize;
    let first_ppage: u64;
    let first_ppage_reported: u64;
    let next_ppage: u64;
    let error: i32;

    if (*isi).nr_pages >= (*(*isi).sis).max {
        return 0;
    }
    max_pages = (*(*isi).sis).max - (*isi).nr_pages;

    /*
     * Round the start up and the end down so that the physical
     * extent aligns to a page boundary.
     */
    first_ppage = (iomap.addr.wrapping_add(PAGE_SIZE - 1) & !(PAGE_SIZE - 1)) >> PAGE_SHIFT;
    next_ppage = (iomap.addr.wrapping_add(iomap.length) & !(PAGE_SIZE - 1)) >> PAGE_SHIFT;

    /* Skip too-short physical extents. */
    if first_ppage >= next_ppage {
        return 0;
    }
    nr_pages = (next_ppage - first_ppage) as usize;
    nr_pages = core::cmp::min(nr_pages, max_pages);

    /*
     * Calculate how much swap space we're adding; the first page contains
     * the swap header and doesn't count.  The mm still wants that first
     * page fed to add_swap_extent, however.
     */
    first_ppage_reported = if iomap.offset == 0 {
        first_ppage + 1
    } else {
        first_ppage
    };
    if (*isi).lowest_ppage > first_ppage_reported {
        (*isi).lowest_ppage = first_ppage_reported;
    }
    if (*isi).highest_ppage < next_ppage - 1 {
        (*isi).highest_ppage = next_ppage - 1;
    }

    /* Add extent, set up for the next call. */
    error = add_swap_extent((*isi).sis, (*isi).nr_pages, nr_pages, first_ppage);
    if error < 0 {
        return error;
    }
    (*isi).nr_extents += error;
    (*isi).nr_pages += nr_pages;
    0
}

unsafe fn iomap_swapfile_fail(isi: *mut iomap_swapfile_info, str_: *const i8) -> i32 {
    let mut buf: *mut i8;
    let mut p: *mut i8 = err_ptr(-ENOMEM);

    buf = kmalloc(PATH_MAX, GFP_KERNEL);
    if !buf.is_null() {
        p = file_path((*isi).file, buf, PATH_MAX);
    }
    pr_err(
        c"swapon: file %s %s\n".as_ptr(),
        if is_err(p) { c"<unknown>\0".as_ptr() } else { p },
        str_,
    );
    kfree(buf);
    -EINVAL
}

/*
 * Accumulate iomaps for this swap file.  We have to accumulate iomaps because
 * swap only cares about contiguous page-aligned physical extents and makes no
 * distinction between written and unwritten extents.
 */
unsafe fn iomap_swapfile_iter(
    iter: *mut iomap_iter,
    iomap_: *mut iomap,
    isi: *mut iomap_swapfile_info,
) -> i32 {
    match (*iomap_).type_ {
        IOMAP_MAPPED | IOMAP_UNWRITTEN => { /* Only real or unwritten extents. */ }
        IOMAP_INLINE => {
            /* No inline data. */
            return iomap_swapfile_fail(isi, c"is inline".as_ptr());
        }
        _ => return iomap_swapfile_fail(isi, c"has unallocated extents".as_ptr()),
    }

    /* No uncommitted metadata or shared blocks. */
    if (*iomap_).flags & IOMAP_F_DIRTY != 0 {
        return iomap_swapfile_fail(isi, c"is not committed".as_ptr());
    }
    if (*iomap_).flags & IOMAP_F_SHARED != 0 {
        return iomap_swapfile_fail(isi, c"has shared extents".as_ptr());
    }

    /* Only one bdev per swap file. */
    if (*iomap_).bdev != (*(*isi).sis).bdev {
        return iomap_swapfile_fail(isi, c"outside the main device".as_ptr());
    }

    if (*isi).iomap.length == 0 {
        /* No accumulated extent, so just store it. */
        core::ptr::copy_nonoverlapping(iomap_, &mut (*isi).iomap, 1);
    } else if (*isi).iomap.addr.wrapping_add((*isi).iomap.length) == (*iomap_).addr {
        /* Append this to the accumulated extent. */
        (*isi).iomap.length = (*isi).iomap.length.wrapping_add((*iomap_).length);
    } else {
        /* Otherwise, add the retained iomap and store this one. */
        let error = iomap_swapfile_add_extent(isi);
        if error != 0 {
            return error;
        }
        core::ptr::copy_nonoverlapping(iomap_, &mut (*isi).iomap, 1);
    }

    iomap_iter_advance_full(iter)
}

/*
 * Iterate a swap file's iomaps to construct physical extents that can be
 * passed to the swapfile subsystem.
 */
unsafe fn iomap_swapfile_activate(
    sis: *mut swap_info_struct,
    swap_file: *mut file,
    pagespan: *mut sector_t,
    ops: *const iomap_ops,
) -> i32 {
    let inode = (*(*swap_file).f_mapping).host;
    let mut iter = iomap_iter {
        inode,
        pos: 0,
        len: i_size_read(inode) & !(PAGE_SIZE - 1),
        flags: IOMAP_REPORT,
        ..core::mem::zeroed()
    };
    let mut isi = iomap_swapfile_info {
        sis,
        lowest_ppage: u64::MAX,
        file: swap_file,
        ..core::mem::zeroed()
    };
    let mut ret: i32;

    /*
     * Persist all file mapping metadata so that we won't have any
     * IOMAP_F_DIRTY iomaps.
     */
    ret = vfs_fsync(swap_file, 1);
    if ret != 0 {
        return ret;
    }

    while {
        ret = iomap_iter(&mut iter, ops);
        ret > 0
    } {
        iter.status = iomap_swapfile_iter(&mut iter, &mut iter.iomap, &mut isi);
    }
    if ret < 0 {
        return ret;
    }

    if isi.iomap.length != 0 {
        ret = iomap_swapfile_add_extent(&mut isi);
        if ret != 0 {
            return ret;
        }
    }

    /*
     * If this swapfile doesn't contain even a single page-aligned
     * contiguous range of blocks, reject this useless swapfile to
     * prevent confusion later on.
     */
    if isi.nr_pages == 0 {
        pr_warn(c"swapon: Cannot find a single usable page in file.\n".as_ptr());
        return -EINVAL;
    }

    *pagespan = 1 + isi.highest_ppage - isi.lowest_ppage;
    (*sis).max = isi.nr_pages;
    (*sis).pages = isi.nr_pages - 1;
    isi.nr_extents
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
