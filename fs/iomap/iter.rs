// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2010 Red Hat, Inc.
 * Copyright (c) 2016-2021 Christoph Hellwig.
 */
// Dependencies supplied by the surrounding iomap and tracing code are
// intentionally referenced but not redefined here.

#[inline]
fn iomap_iter_clean_fbatch(iter: &IomapIter, iomap: &mut Iomap) {
    if iter.fbatch.is_null() {
        return;
    }
    iomap.flags &= !IOMAP_F_FOLIO_BATCH;
    if unsafe { folio_batch_count(iter.fbatch) } != 0 {
        unsafe {
            folio_batch_release(iter.fbatch);
            folio_batch_reinit(iter.fbatch);
        }
    }
}

/* Advance the current iterator position and decrement the remaining length */
pub fn iomap_iter_advance(iter: &mut IomapIter, count: u64) -> i32 {
    if WARN_ON_ONCE(count > iomap_length(iter)) {
        return -EIO;
    }
    iter.pos = iter.pos.wrapping_add(count);
    iter.len = iter.len.wrapping_sub(count);
    0
}

#[inline]
fn iomap_iter_done(iter: &mut IomapIter) {
    WARN_ON_ONCE(iter.iomap.offset > iter.pos);
    WARN_ON_ONCE(iter.iomap.length == 0);
    WARN_ON_ONCE(iter.iomap.offset.wrapping_add(iter.iomap.length) <= iter.pos);
    WARN_ON_ONCE(iter.iomap.flags & IOMAP_F_STALE != 0);

    iter.iter_start_pos = iter.pos;

    trace_iomap_iter_dstmap(iter.inode, &iter.iomap);
    if iter.srcmap.type_ != IOMAP_HOLE {
        trace_iomap_iter_srcmap(iter.inode, &iter.srcmap);
    }
}

/**
 * iomap_iter_continue - decide whether iteration should continue
 * @iter: iteration structure
 * @iomap: the mapping that was just processed
 * @srcmap: the source mapping that was just processed
 *
 * Helper normally called via iomap_iter_next(). Called after the previous
 * mapping has been finished to determine whether there is more of the file
 * range left to process.
 *
 * Returns 1 if there is more work to do, in which case @iomap and @srcmap are
 * cleared so the caller can produce the next mapping; zero if the range is
 * fully consumed; or a negative errno on error.
 */
pub fn iomap_iter_continue(
    iter: &IomapIter,
    iomap: &mut Iomap,
    srcmap: &mut Iomap,
    mut ret: i32,
) -> i32 {
    let stale = iomap.flags & IOMAP_F_STALE != 0;
    let advanced = (iter.pos as isize).wrapping_sub(iter.iter_start_pos as isize);

    if ret < 0 && advanced == 0 {
        return ret;
    }

    if WARN_ON_ONCE(iter.status > 0) {
        /* detect old return semantics where this would advance */
        ret = -EIO;
    } else if iter.status < 0 {
        ret = iter.status;
    } else if iter.len == 0 || (advanced == 0 && !stale) {
        ret = 0;
    } else {
        ret = 1;
    }

    iomap_iter_clean_fbatch(iter, iomap);

    if ret <= 0 {
        return ret;
    }

    unsafe {
        core::ptr::write_bytes(iomap, 0, 1);
        core::ptr::write_bytes(srcmap, 0, 1);
    }

    ret
}

/**
 * iomap_iter - iterate over ranges in a file
 * @iter: iteration structure
 * @ops: iomap ops provided by the filesystem
 *
 * Iterate over filesystem-provided space mappings for the provided file range.
 */
pub fn iomap_iter(iter: &mut IomapIter, ops: &IomapOps) -> i32 {
    let ret;

    trace_iomap_iter(iter, ops, _RET_IP_);

    if let Some(iomap_next) = ops.iomap_next {
        ret = iomap_next(iter, &mut iter.iomap, &mut iter.srcmap);
    } else {
        ret = iomap_iter_next(
            iter,
            &mut iter.iomap,
            &mut iter.srcmap,
            ops.iomap_begin,
            ops.iomap_end,
        );
    }

    iter.status = 0;
    if ret > 0 {
        iomap_iter_done(iter);
    } else if ret < 0 {
        iomap_iter_clean_fbatch(iter, &mut iter.iomap);
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
