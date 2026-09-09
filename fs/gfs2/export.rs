// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) Sistina Software, Inc.  1997-2003 All rights reserved.
 * Copyright (C) 2004-2006 Red Hat, Inc.  All rights reserved.
 */

// Linux kernel and local GFS2 dependencies supplied by other translation units.

const GFS2_SMALL_FH_SIZE: usize = 4;
const GFS2_LARGE_FH_SIZE: usize = 8;
const GFS2_OLD_FH_SIZE: usize = 10;

unsafe fn gfs2_encode_fh(
    inode: *mut inode,
    p: *mut u32,
    len: *mut i32,
    parent: *mut inode,
) -> i32 {
    let fh = p as *mut u32;
    let sb = (*inode).i_sb;
    let mut ip = GFS2_I(inode);

    if !parent.is_null() && (*len as usize) < GFS2_LARGE_FH_SIZE {
        *len = GFS2_LARGE_FH_SIZE as i32;
        return FILEID_INVALID;
    } else if (*len as usize) < GFS2_SMALL_FH_SIZE {
        *len = GFS2_SMALL_FH_SIZE as i32;
        return FILEID_INVALID;
    }

    (*fh.add(0)) = cpu_to_be32(((*ip).i_no_formal_ino >> 32) as u32);
    (*fh.add(1)) = cpu_to_be32((*ip).i_no_formal_ino as u32);
    (*fh.add(2)) = cpu_to_be32(((*ip).i_no_addr >> 32) as u32);
    (*fh.add(3)) = cpu_to_be32((*ip).i_no_addr as u32);
    *len = GFS2_SMALL_FH_SIZE as i32;

    if parent.is_null() || inode == d_inode((*sb).s_root) {
        return *len;
    }

    ip = GFS2_I(parent);

    (*fh.add(4)) = cpu_to_be32(((*ip).i_no_formal_ino >> 32) as u32);
    (*fh.add(5)) = cpu_to_be32((*ip).i_no_formal_ino as u32);
    (*fh.add(6)) = cpu_to_be32(((*ip).i_no_addr >> 32) as u32);
    (*fh.add(7)) = cpu_to_be32((*ip).i_no_addr as u32);
    *len = GFS2_LARGE_FH_SIZE as i32;

    *len
}

#[repr(C)]
struct get_name_filldir {
    ctx: dir_context,
    inum: gfs2_inum_host,
    name: *mut i8,
}

unsafe extern "C" fn get_name_filldir(
    ctx: *mut dir_context,
    name: *const i8,
    length: i32,
    _offset: loff_t,
    inum: u64,
    _type: u32,
) -> bool {
    let gnfd = container_of!(ctx, get_name_filldir, ctx);

    if inum != (*gnfd).inum.no_addr {
        return true;
    }

    memcpy((*gnfd).name as *mut u8, name as *const u8, length as usize);
    *((*gnfd).name.add(length as usize)) = 0;

    false
}

unsafe fn gfs2_get_name(
    parent: *mut dentry,
    name: *mut i8,
    child: *mut dentry,
) -> i32 {
    let dir = d_inode(parent);
    let inode = d_inode(child);
    let dip: *mut gfs2_inode;
    let ip: *mut gfs2_inode;
    let mut gnfd = get_name_filldir {
        ctx: dir_context {
            actor: Some(get_name_filldir),
            ..core::mem::zeroed()
        },
        inum: core::mem::zeroed(),
        name,
    };
    let mut gh: gfs2_holder = core::mem::zeroed();
    let mut error: i32;
    let mut f_ra = file_ra_state { start: 0, ..core::mem::zeroed() };

    if dir.is_null() {
        return -EINVAL;
    }

    if !S_ISDIR((*dir).i_mode) || inode.is_null() {
        return -EINVAL;
    }

    dip = GFS2_I(dir);
    ip = GFS2_I(inode);

    *name = 0;
    gnfd.inum.no_addr = (*ip).i_no_addr;
    gnfd.inum.no_formal_ino = (*ip).i_no_formal_ino;

    error = gfs2_glock_nq_init((*dip).i_gl, LM_ST_SHARED, 0, &mut gh);
    if error != 0 {
        return error;
    }

    error = gfs2_dir_read(dir, &mut gnfd.ctx, &mut f_ra);

    gfs2_glock_dq_uninit(&mut gh);

    if error == 0 && *name == 0 {
        error = -ENOENT;
    }

    error
}

unsafe fn gfs2_get_parent(child: *mut dentry) -> *mut dentry {
    d_obtain_alias(gfs2_lookupi(d_inode(child), &gfs2_qdotdot, 1))
}

unsafe fn gfs2_get_dentry(
    sb: *mut super_block,
    inum: *mut gfs2_inum_host,
) -> *mut dentry {
    let sdp = (*sb).s_fs_info as *mut gfs2_sbd;
    let mut inode: *mut inode;

    if (*inum).no_formal_ino == 0 {
        return ERR_PTR(-ESTALE);
    }
    inode = gfs2_lookup_by_inum(sdp, (*inum).no_addr, (*inum).no_formal_ino,
                                GFS2_BLKST_DINODE);
    d_obtain_alias(inode)
}

unsafe fn gfs2_fh_to_dentry(
    sb: *mut super_block,
    fid: *mut fid,
    fh_len: i32,
    fh_type: i32,
) -> *mut dentry {
    let mut this: gfs2_inum_host = core::mem::zeroed();
    let fh = (*fid).raw.as_mut_ptr() as *mut u32;

    match fh_type {
        GFS2_SMALL_FH_SIZE as i32 | GFS2_LARGE_FH_SIZE as i32 | GFS2_OLD_FH_SIZE as i32 => {
            if fh_len < GFS2_SMALL_FH_SIZE as i32 {
                return core::ptr::null_mut();
            }
            this.no_formal_ino = (be32_to_cpu(*fh.add(0)) as u64) << 32;
            this.no_formal_ino |= be32_to_cpu(*fh.add(1)) as u64;
            this.no_addr = (be32_to_cpu(*fh.add(2)) as u64) << 32;
            this.no_addr |= be32_to_cpu(*fh.add(3)) as u64;
            gfs2_get_dentry(sb, &mut this)
        }
        _ => core::ptr::null_mut(),
    }
}

unsafe fn gfs2_fh_to_parent(
    sb: *mut super_block,
    fid: *mut fid,
    fh_len: i32,
    fh_type: i32,
) -> *mut dentry {
    let mut parent: gfs2_inum_host = core::mem::zeroed();
    let fh = (*fid).raw.as_mut_ptr() as *mut u32;

    match fh_type {
        GFS2_LARGE_FH_SIZE as i32 | GFS2_OLD_FH_SIZE as i32 => {
            if fh_len < GFS2_LARGE_FH_SIZE as i32 {
                return core::ptr::null_mut();
            }
            parent.no_formal_ino = (be32_to_cpu(*fh.add(4)) as u64) << 32;
            parent.no_formal_ino |= be32_to_cpu(*fh.add(5)) as u64;
            parent.no_addr = (be32_to_cpu(*fh.add(6)) as u64) << 32;
            parent.no_addr |= be32_to_cpu(*fh.add(7)) as u64;
            gfs2_get_dentry(sb, &mut parent)
        }
        _ => core::ptr::null_mut(),
    }
}

#[repr(C)]
static gfs2_export_ops: export_operations = export_operations {
    encode_fh: Some(gfs2_encode_fh),
    fh_to_dentry: Some(gfs2_fh_to_dentry),
    fh_to_parent: Some(gfs2_fh_to_parent),
    get_name: Some(gfs2_get_name),
    get_parent: Some(gfs2_get_parent),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
