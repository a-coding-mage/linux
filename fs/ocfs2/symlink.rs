/*
 *  linux/cluster/ssi/cfs/symlink.c
 *
 *	This program is free software; you can redistribute it and/or
 *	modify it under the terms of the GNU General Public License as
 *	published by the Free Software Foundation; either version 2 of
 *	the License, or (at your option) any later version.
 *
 *	This program is distributed in the hope that it will be useful,
 *	but WITHOUT ANY WARRANTY; without even the implied warranty of
 *	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE
 *	or NON INFRINGEMENT.  See the GNU General Public License for more
 *	details.
 *
 * 	You should have received a copy of the GNU General Public License
 * 	along with this program; if not, write to the Free Software
 * 	Foundation, Inc., 675 Mass Ave, Cambridge, MA 02139, USA.
 *
 *	Questions/Comments/Bugfixes to ssic-linux-devel@lists.sourceforge.net
 *
 * 	Copyright (C) 1992  Rick Sladkey
 *
 * 	Optimization changes Copyright (C) 1994 Florian La Roche
 *
 * 	Jun 7 1999, cache symlink lookups in the page cache.  -DaveM
 *
 * 	Portions Copyright (C) 2001 Compaq Computer Corporation
 *
 * 	ocfs2 symlink handling code.
 *
 * 	Copyright (C) 2004, 2005 Oracle.
 */

// Kernel and OCFS2 dependencies are provided by the surrounding translation unit.

unsafe fn ocfs2_fast_symlink_read_folio(
    f: *mut file,
    folio: *mut folio,
) -> i32 {
    let inode: *mut inode = unsafe { (*(*folio).mapping).host };
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let status: i32 = unsafe { ocfs2_read_inode_block(inode, &mut bh) };
    let fe: *mut ocfs2_dinode;
    let link: *const core::ffi::c_char;
    let len: usize;

    if status < 0 {
        unsafe { mlog_errno(status); }
        unsafe { folio_end_read(folio, false); }
        unsafe { brelse(bh); }
        return status;
    }

    fe = unsafe { (*bh).b_data as *mut ocfs2_dinode };
    link = unsafe { (*fe).id2.i_symlink.as_ptr() as *const core::ffi::c_char };
    // will be less than a page size
    len = unsafe { strnlen(link, ocfs2_fast_symlink_chars((*inode).i_sb)) };
    unsafe { memcpy_to_folio(folio, 0, link as *const core::ffi::c_void, len + 1); }

    unsafe { folio_end_read(folio, status == 0); }
    unsafe { brelse(bh); }
    let _ = f;
    status
}

pub static ocfs2_fast_symlink_aops: address_space_operations = address_space_operations {
    read_folio: Some(ocfs2_fast_symlink_read_folio),
};

pub static ocfs2_symlink_inode_operations: inode_operations = inode_operations {
    get_link: Some(page_get_link),
    getattr: Some(ocfs2_getattr),
    setattr: Some(ocfs2_setattr),
    listxattr: Some(ocfs2_listxattr),
    fiemap: Some(ocfs2_fiemap),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
