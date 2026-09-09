// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/affs/symlink.c
 *
 *  1995  Hans-Joachim Widmaier - Modified for affs.
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 *
 *  affs symlink handling code
 */

// Dependency declarations supplied by affs.h and the surrounding kernel code
// are intentionally left external to this translation unit.

unsafe fn affs_symlink_read_folio(file: *mut file, folio: *mut folio) -> i32 {
    let mut bh: *mut buffer_head;
    let inode: *mut inode = (*(*folio).mapping).host;
    let link: *mut i8 = folio_address(folio);
    let lf: *mut slink_front;
    let mut i: i32;
    let mut j: i32;
    let mut c: i8;
    let mut lc: i8;

    pr_debug!("get_link(ino=%llu)\n", (*inode).i_ino);

    bh = affs_bread((*inode).i_sb, (*inode).i_ino);
    if bh.is_null() {
        goto_fail(folio);
        return -EIO;
    }
    i = 0;
    j = 0;
    lf = (*bh).b_data as *mut slink_front;
    lc = 0;

    if strchr((*lf).symname.as_ptr(), b':' as i8).is_some() {
        // Handle assign or volume name
        let sbi: *mut affs_sb_info = AFFS_SB((*inode).i_sb);
        let mut pf: *const i8;
        spin_lock(&mut (*sbi).symlink_lock);
        pf = if !(*sbi).s_prefix.is_null() {
            (*sbi).s_prefix
        } else {
            b"/\0".as_ptr() as *const i8
        };
        while i < 1023 && {
            c = *pf.add(i as usize);
            c != 0
        } {
            *link.add(i as usize) = c;
            i += 1;
        }
        spin_unlock(&mut (*sbi).symlink_lock);
        while i < 1023 && (*lf).symname[j as usize] != b':' as i8 {
            *link.add(i as usize) = (*lf).symname[j as usize];
            i += 1;
            j += 1;
        }
        if i < 1023 {
            *link.add(i as usize) = b'/' as i8;
            i += 1;
        }
        j += 1;
        lc = b'/' as i8;
    }
    while i < 1023 && {
        c = (*lf).symname[j as usize];
        c != 0
    } {
        if c == b'/' as i8 && lc == b'/' as i8 && i < 1020 {
            // parent dir
            *link.add(i as usize) = b'.' as i8;
            i += 1;
            *link.add(i as usize) = b'.' as i8;
            i += 1;
        }
        *link.add(i as usize) = c;
        i += 1;
        lc = c;
        j += 1;
    }
    *link.add(i as usize) = 0;
    affs_brelse(bh);
    folio_mark_uptodate(folio);
    folio_unlock(folio);
    0
}

unsafe fn goto_fail(folio: *mut folio) {
    folio_unlock(folio);
}

#[repr(C)]
pub struct address_space_operations {
    pub read_folio: Option<unsafe fn(*mut file, *mut folio) -> i32>,
}

pub static affs_symlink_aops: address_space_operations = address_space_operations {
    read_folio: Some(affs_symlink_read_folio),
};

#[repr(C)]
pub struct inode_operations {
    pub get_link: Option<unsafe fn()>,
    pub setattr: Option<unsafe fn()>,
}

pub static affs_symlink_inode_operations: inode_operations = inode_operations {
    get_link: Some(page_get_link),
    setattr: Some(affs_setattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
