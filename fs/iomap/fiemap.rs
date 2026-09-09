// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2016-2021 Christoph Hellwig.
 */

// Types, constants, and functions referenced below are supplied by the
// corresponding kernel iomap, fiemap, and pagemap dependencies.

unsafe fn iomap_to_fiemap(
    fi: *mut fiemap_extent_info,
    iomap: *const iomap,
    mut flags: u32,
) -> i32 {
    match (*iomap).type_ {
        IOMAP_HOLE => {
            /* skip holes */
            return 0;
        }
        IOMAP_DELALLOC => {
            flags |= FIEMAP_EXTENT_DELALLOC | FIEMAP_EXTENT_UNKNOWN;
        }
        IOMAP_MAPPED => {}
        IOMAP_UNWRITTEN => {
            flags |= FIEMAP_EXTENT_UNWRITTEN;
        }
        IOMAP_INLINE => {
            flags |= FIEMAP_EXTENT_DATA_INLINE;
        }
        _ => {}
    }

    if (*iomap).flags & IOMAP_F_MERGED != 0 {
        flags |= FIEMAP_EXTENT_MERGED;
    }
    if (*iomap).flags & IOMAP_F_SHARED != 0 {
        flags |= FIEMAP_EXTENT_SHARED;
    }

    fiemap_fill_next_extent(
        fi,
        (*iomap).offset,
        if (*iomap).addr != IOMAP_NULL_ADDR {
            (*iomap).addr
        } else {
            0
        },
        (*iomap).length,
        flags,
    )
}

unsafe fn iomap_fiemap_iter(
    iter: *mut iomap_iter,
    fi: *mut fiemap_extent_info,
    prev: *mut iomap,
) -> i32 {
    let ret: i32;

    if (*iter).iomap.type_ == IOMAP_HOLE {
        return iomap_iter_advance_full(iter);
    }

    ret = iomap_to_fiemap(fi, prev, 0);
    *prev = (*iter).iomap;
    if ret < 0 {
        return ret;
    }
    if ret == 1 {
        /* extent array full */
        return 0;
    }

    iomap_iter_advance_full(iter)
}

unsafe fn iomap_fiemap(
    inode: *mut inode,
    fi: *mut fiemap_extent_info,
    start: u64,
    len: u64,
    ops: *const iomap_ops,
) -> i32 {
    let mut iter = iomap_iter {
        inode,
        pos: start,
        len,
        flags: IOMAP_REPORT,
        ..core::mem::zeroed()
    };
    let mut prev: iomap = iomap {
        type_: IOMAP_HOLE,
        ..core::mem::zeroed()
    };
    let mut ret: i32;

    ret = fiemap_prep(inode, fi, start, &mut iter.len, 0);
    if ret != 0 {
        return ret;
    }

    while {
        ret = iomap_iter(&mut iter, ops);
        ret > 0
    } {
        iter.status = iomap_fiemap_iter(&mut iter, fi, &mut prev);
    }

    if prev.type_ != IOMAP_HOLE {
        ret = iomap_to_fiemap(fi, &prev, FIEMAP_EXTENT_LAST);
        if ret < 0 {
            return ret;
        }
    }

    /* inode with no (attribute) mapping will give ENOENT */
    if ret < 0 && ret != -ENOENT {
        return ret;
    }
    0
}

// EXPORT_SYMBOL_GPL(iomap_fiemap);

/* legacy ->bmap interface.  0 is the error return (!) */
unsafe fn iomap_bmap(
    mapping: *mut address_space,
    mut bno: sector_t,
    ops: *const iomap_ops,
) -> sector_t {
    let host = (*mapping).host;
    let mut iter = iomap_iter {
        inode: host,
        pos: (bno as loff_t) << (*host).i_blkbits,
        len: i_blocksize(host),
        flags: IOMAP_REPORT,
        ..core::mem::zeroed()
    };
    let blkshift: u32 = (*host).i_blkbits - SECTOR_SHIFT;
    let mut ret: i32;

    if filemap_write_and_wait(mapping) != 0 {
        return 0;
    }

    bno = 0;
    while {
        ret = iomap_iter(&mut iter, ops);
        ret > 0
    } {
        if iter.iomap.type_ == IOMAP_MAPPED {
            bno = iomap_sector(&mut iter.iomap, iter.pos) >> blkshift;
        }
        /* leave iter.status unset to abort loop */
    }
    if ret != 0 {
        return 0;
    }

    bno
}

// EXPORT_SYMBOL_GPL(iomap_bmap);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
