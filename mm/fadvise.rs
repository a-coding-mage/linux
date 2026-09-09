// SPDX-License-Identifier: GPL-2.0
/*
 * mm/fadvise.c
 *
 * Copyright (C) 2002, Linus Torvalds
 *
 * 11Jan2003\tAndrew Morton
 *\t\tInitial version.
 */

// Linux kernel dependencies supplied by other translation units.

/*
 * POSIX_FADV_WILLNEED could set PG_Referenced, and POSIX_FADV_NOREUSE could
 * deactivate the pages and clear PG_Referenced.
 */

pub unsafe fn generic_fadvise(
    file: *mut file,
    offset: loff_t,
    len: loff_t,
    advice: c_int,
) -> c_int {
    let inode: *mut inode;
    let mapping: *mut address_space;
    let bdi: *mut backing_dev_info;
    let mut endbyte: loff_t; // inclusive
    let mut start_index: pgoff_t;
    let mut end_index: pgoff_t;
    let mut nrpages: c_ulong;

    inode = file_inode(file);
    if S_ISFIFO((*inode).i_mode) {
        return -ESPIPE;
    }

    mapping = (*file).f_mapping;
    if mapping.is_null() || len < 0 || offset < 0 {
        return -EINVAL;
    }

    bdi = inode_to_bdi((*mapping).host);

    if IS_DAX(inode) || bdi == &raw mut noop_backing_dev_info {
        match advice {
            POSIX_FADV_NORMAL
            | POSIX_FADV_RANDOM
            | POSIX_FADV_SEQUENTIAL
            | POSIX_FADV_WILLNEED
            | POSIX_FADV_NOREUSE
            | POSIX_FADV_DONTNEED => {
                // no bad return value, but ignore advice
            }
            _ => return -EINVAL,
        }
        return 0;
    }

    /*
     * Careful about overflows. Len == 0 means "as much as possible".  Use
     * unsigned math because signed overflows are undefined and UBSan
     * complains.
     */
    endbyte = (offset as u64).wrapping_add(len as u64) as loff_t;
    if len == 0 || (endbyte as u64) < (len as u64) {
        endbyte = LLONG_MAX;
    } else {
        endbyte = endbyte.wrapping_sub(1); // inclusive
    }

    match advice {
        POSIX_FADV_NORMAL => {
            (*file).f_ra.ra_pages = (*bdi).ra_pages;
            spin_lock(&raw mut (*file).f_lock);
            (*file).f_mode &= !(FMODE_RANDOM | FMODE_NOREUSE);
            spin_unlock(&raw mut (*file).f_lock);
        }
        POSIX_FADV_RANDOM => {
            spin_lock(&raw mut (*file).f_lock);
            (*file).f_mode |= FMODE_RANDOM;
            spin_unlock(&raw mut (*file).f_lock);
        }
        POSIX_FADV_SEQUENTIAL => {
            (*file).f_ra.ra_pages = (*bdi).ra_pages.wrapping_mul(2);
            spin_lock(&raw mut (*file).f_lock);
            (*file).f_mode &= !FMODE_RANDOM;
            spin_unlock(&raw mut (*file).f_lock);
        }
        POSIX_FADV_WILLNEED => {
            // First and last PARTIAL page!
            start_index = (offset as u64 >> PAGE_SHIFT) as pgoff_t;
            end_index = (endbyte as u64 >> PAGE_SHIFT) as pgoff_t;

            // Careful about overflow on the "+1"
            nrpages = end_index.wrapping_sub(start_index).wrapping_add(1) as c_ulong;
            if nrpages == 0 {
                nrpages = !0;
            }

            force_page_cache_readahead(mapping, file, start_index, nrpages);
        }
        POSIX_FADV_NOREUSE => {
            spin_lock(&raw mut (*file).f_lock);
            (*file).f_mode |= FMODE_NOREUSE;
            spin_unlock(&raw mut (*file).f_lock);
        }
        POSIX_FADV_DONTNEED => {
            filemap_flush_range(mapping, offset, endbyte);

            /*
             * First and last FULL page! Partial pages are deliberately
             * preserved on the expectation that it is better to preserve
             * needed memory than to discard unneeded memory.
             */
            start_index = ((offset as u64).wrapping_add((PAGE_SIZE - 1) as u64) >> PAGE_SHIFT) as pgoff_t;
            end_index = (endbyte as u64 >> PAGE_SHIFT) as pgoff_t;
            /*
             * The page at end_index will be inclusively discarded according
             * by invalidate_mapping_pages(), so subtracting 1 from
             * end_index means we will skip the last page.  But if endbyte
             * is page aligned or is at the end of file, we should not skip
             * that page - discarding the last page is safe enough.
             */
            if (endbyte & !PAGE_MASK) != !PAGE_MASK
                && endbyte != (*inode).i_size.wrapping_sub(1)
            {
                /* First page is tricky as 0 - 1 = -1, but pgoff_t
                 * is unsigned, so the end_index >= start_index
                 * check below would be true and we'll discard the whole
                 * file cache which is not what was asked.
                 */
                if end_index == 0 {
                    return 0;
                }
                end_index = end_index.wrapping_sub(1);
            }

            if end_index >= start_index {
                let mut nr_failed: c_ulong = 0;
                // See the corresponding kernel comments for cache-drain rationale.
                lru_add_drain();
                mapping_try_invalidate(mapping, start_index, end_index, &mut nr_failed);
                if nr_failed != 0 {
                    lru_add_drain_all();
                    invalidate_mapping_pages(mapping, start_index, end_index);
                }
            }
        }
        _ => return -EINVAL,
    }
    0
}

pub unsafe fn vfs_fadvise(
    file: *mut file,
    offset: loff_t,
    len: loff_t,
    advice: c_int,
) -> c_int {
    if !(*file).f_op.is_null() && !(*(*file).f_op).fadvise.is_none() {
        return ((*(*file).f_op).fadvise.unwrap())(file, offset, len, advice);
    }
    generic_fadvise(file, offset, len, advice)
}

// CONFIG_ADVISE_SYSCALLS conditional is preserved from the source.
#[cfg(CONFIG_ADVISE_SYSCALLS)]
pub unsafe fn ksys_fadvise64_64(fd: c_int, offset: loff_t, len: loff_t, advice: c_int) -> c_int {
    let f = fd_class_f(fd);
    if fd_empty(&f) {
        return -EBADF;
    }
    vfs_fadvise(fd_file(&f), offset, len, advice)
}

#[cfg(CONFIG_ADVISE_SYSCALLS)]
pub unsafe fn sys_fadvise64_64(fd: c_int, offset: loff_t, len: loff_t, advice: c_int) -> c_int {
    ksys_fadvise64_64(fd, offset, len, advice)
}

#[cfg(all(CONFIG_ADVISE_SYSCALLS, __ARCH_WANT_SYS_FADVISE64))]
pub unsafe fn sys_fadvise64(fd: c_int, offset: loff_t, len: size_t, advice: c_int) -> c_int {
    ksys_fadvise64_64(fd, offset, len as loff_t, advice)
}

#[cfg(all(CONFIG_ADVISE_SYSCALLS, CONFIG_COMPAT, __ARCH_WANT_COMPAT_FADVISE64_64))]
pub unsafe fn compat_sys_fadvise64_64(
    fd: c_int,
    offset: compat_arg_u64,
    len: compat_arg_u64,
    advice: c_int,
) -> c_int {
    ksys_fadvise64_64(fd, compat_arg_u64_glue(offset), compat_arg_u64_glue(len), advice)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
