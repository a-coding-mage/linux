// SPDX-License-Identifier: GPL-2.0-or-later
/* NOMMU mmap support for RomFS on MTD devices
 *
 * Copyright © 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Dependencies supplied by the surrounding kernel translation. */

/*
 * try to determine where a shared mapping can be made
 * - only supported for NOMMU at the moment (MMU can't doesn't copy private
 *   mappings)
 * - attempts to map through to the underlying MTD device
 */
unsafe fn romfs_get_unmapped_area(
    file: *mut file,
    addr: c_ulong,
    mut len: c_ulong,
    pgoff: c_ulong,
    flags: c_ulong,
) -> c_ulong {
    let inode = (*(*file).f_mapping).host;
    let mtd = (*(*inode).i_sb).s_mtd;
    let isize: c_ulong;
    let mut offset: c_ulong;
    let maxpages: c_ulong;
    let lpages: c_ulong;
    let mut ret: c_int;

    if mtd.is_null() {
        return (-ENOSYS) as c_ulong;
    }

    /* the mapping mustn't extend beyond the EOF */
    lpages = (len + PAGE_SIZE - 1) >> PAGE_SHIFT;
    isize = i_size_read(inode);
    offset = pgoff << PAGE_SHIFT;

    maxpages = (isize + PAGE_SIZE - 1) >> PAGE_SHIFT;
    if (pgoff >= maxpages) || (maxpages - pgoff < lpages) {
        return (-EINVAL) as c_ulong;
    }

    if addr != 0 {
        return (-EINVAL) as c_ulong;
    }

    if (len > (*mtd).size) || (pgoff >= ((*mtd).size >> PAGE_SHIFT)) {
        return (-EINVAL) as c_ulong;
    }

    offset += (*ROMFS_I(inode)).i_dataoffset;
    if offset >= (*mtd).size {
        return (-EINVAL) as c_ulong;
    }
    /* the mapping mustn't extend beyond the EOF */
    if (offset + len) > (*mtd).size {
        len = (*mtd).size - offset;
    }

    ret = mtd_get_unmapped_area(mtd, len, offset, flags);
    if ret == -EOPNOTSUPP {
        ret = -ENOSYS;
    }
    ret as c_ulong
}

/*
 * permit a R/O mapping to be made directly through onto an MTD device if
 * possible
 */
unsafe fn romfs_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    if is_nommu_shared_vma_flags((*desc).vma_flags) {
        0
    } else {
        -ENOSYS
    }
}

unsafe fn romfs_mmap_capabilities(file: *mut file) -> c_uint {
    let mtd = (*(*file_inode(file)).i_sb).s_mtd;

    if mtd.is_null() {
        NOMMU_MAP_COPY
    } else {
        mtd_mmap_capabilities(mtd)
    }
}

const romfs_ro_fops: file_operations = file_operations {
    llseek: Some(generic_file_llseek),
    read_iter: Some(generic_file_read_iter),
    splice_read: Some(filemap_splice_read),
    mmap_prepare: Some(romfs_mmap_prepare),
    get_unmapped_area: Some(romfs_get_unmapped_area),
    mmap_capabilities: Some(romfs_mmap_capabilities),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
