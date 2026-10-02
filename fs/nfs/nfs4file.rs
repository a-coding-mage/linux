// SPDX-License-Identifier: GPL-2.0
/* NFSv4 file operations, reconstructed from the unchanged nfs4file.c.
 * Copyright (C) 1992 Rick Sladkey
 * Configured layouts and C prototypes are generated from kernel headers.
 */
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/nfs4file_generated.rs"));
}
use bindings::*;
use core::mem::zeroed;
use core::ptr::{addr_of, addr_of_mut, null_mut};
use kernel::ffi::{c_int, c_void};
#[cfg(CONFIG_NFS_V4_2)]
use core::{mem::size_of, ptr::null};
#[cfg(CONFIG_NFS_V4_2)]
use kernel::ffi::{c_char, c_long};

#[inline]
fn neg(error: u32) -> c_int { -(error as c_int) }
#[inline]
fn is_err<T>(p: *const T) -> bool { p as usize >= usize::MAX - MAX_ERRNO as usize + 1 }
#[inline]
fn err_ptr<T>(error: c_int) -> *mut T { error as isize as *mut T }

unsafe extern "C" fn nfs4_file_open(mut inode: *mut inode, filp: *mut file) -> c_int {
    let dentry = rust_nfs4_file_dentry(filp);
    let mut openflags = (*filp).f_flags;
    rust_nfs4_debug_open(dentry);
    let mut err = nfs_check_flags(openflags as c_int);
    if err != 0 { return err; }

    // Only cached positive dentries reach this path. Creation is handled by
    // lookup/create; a stale result drops this dentry and makes the VFS retry.
    openflags &= !(O_CREAT | O_EXCL);
    let parent = dget_parent(dentry);
    let dir = rust_nfs4_d_inode(parent);
    let ctx = alloc_nfs_open_context(rust_nfs4_file_dentry(filp),
        rust_nfs4_flags_to_mode(openflags as c_int), filp);
    err = ctx as isize as c_int;
    if !is_err(ctx) {
        let mut attr: iattr = zeroed();
        attr.ia_valid = ATTR_OPEN;
        if openflags & O_TRUNC != 0 {
            attr.ia_valid |= ATTR_SIZE;
            attr.ia_size = 0;
            rust_nfs4_write_and_wait((*inode).i_mapping);
        }
        inode = (*rust_nfs4_proto(dir)).open_context.unwrap_unchecked()(
            dir, ctx, openflags as c_int, addr_of_mut!(attr), null_mut());
        let drop_dentry = if is_err(inode) {
            err = inode as isize as c_int;
            err == neg(ENOENT) || err == neg(ESTALE) || err == neg(EISDIR)
                || err == neg(ENOTDIR) || err == neg(ELOOP)
        } else {
            inode != rust_nfs4_d_inode(dentry)
        };
        if drop_dentry {
            d_drop(dentry);
            err = neg(EOPENSTALE);
        } else if !is_err(inode) {
            nfs_file_set_open_context(filp, ctx);
            rust_nfs4_fscache_open(inode, filp);
            err = 0;
            (*filp).f_mode |= RUST_NFS4_FMODE_CAN_ODIRECT;
            if rust_nfs4_test_bit(NFS_CONTEXT_O_DIRECT, addr_of!((*ctx).flags)) {
                (*filp).f_flags |= O_DIRECT;
            }
        }
        put_nfs_open_context(ctx);
    }
    dput(parent);
    err
}

unsafe extern "C" fn nfs4_file_flush(file: *mut file, _id: fl_owner_t) -> c_int {
    let inode = rust_nfs4_file_inode(file);
    rust_nfs4_debug_flush(file);
    rust_nfs4_inc_stats(inode, NFSIOS_VFSFLUSH);
    if (*file).f_mode & RUST_NFS4_FMODE_WRITE == 0 { return 0; }
    if !nfs4_delegation_flush_on_close(inode) {
        return filemap_fdatawrite((*file).f_mapping);
    }
    let since = rust_nfs4_sample_wb_err((*file).f_mapping);
    nfs_wb_all(inode);
    rust_nfs4_check_wb_err((*file).f_mapping, since)
}

#[cfg(CONFIG_NFS_V4_2)]
unsafe fn __nfs4_copy_file_range(file_in: *mut file, pos_in: loff_t,
    file_out: *mut file, pos_out: loff_t, count: usize, _flags: u32) -> isize {
    let mut cn_resp: *mut nfs42_copy_notify_res = null_mut();
    let mut nss: *mut nl4_server = null_mut();
    let mut cnrs: *mut nfs4_stateid = null_mut();
    if (*file_in).f_op != addr_of!(nfs4_file_operations.0) { return neg(EXDEV) as isize; }
    if (*rust_nfs4_server(rust_nfs4_file_inode(file_out))).caps & NFS_CAP_COPY == 0
        || (*rust_nfs4_server(rust_nfs4_file_inode(file_in))).caps & NFS_CAP_COPY == 0 {
        return neg(EOPNOTSUPP) as isize;
    }
    if rust_nfs4_file_inode(file_in) == rust_nfs4_file_inode(file_out) {
        return neg(EOPNOTSUPP) as isize;
    }
    // Keep the C unsigned-int multiplication before conversion to size_t.
    let sync = count <= (*rust_nfs4_server(rust_nfs4_file_inode(file_in))).rsize
        .wrapping_mul(2) as usize;
    loop {
        let mut ret = 0;
        if !rust_nfs4_same_server(file_in, file_out) {
            if sync { return neg(EOPNOTSUPP) as isize; }
            cn_resp = rust_nfs4_kzalloc(size_of::<nfs42_copy_notify_res>()).cast();
            if cn_resp.is_null() { return neg(ENOMEM) as isize; }
            if nfs42_proc_copy_notify(file_in, file_out, cn_resp) != 0 {
                ret = neg(EOPNOTSUPP) as isize;
            } else {
                nss = addr_of_mut!((*cn_resp).cnr_src);
                cnrs = addr_of_mut!((*cn_resp).cnr_stateid);
            }
        }
        if ret == 0 {
            ret = nfs42_proc_copy(file_in, pos_in, file_out, pos_out, count, nss, cnrs, sync);
        }
        kfree(cn_resp.cast());
        if ret != neg(EAGAIN) as isize { return ret; }
    }
}

#[cfg(CONFIG_NFS_V4_2)]
unsafe extern "C" fn nfs4_copy_file_range(file_in: *mut file, pos_in: loff_t,
    file_out: *mut file, pos_out: loff_t, count: usize, flags: u32) -> isize {
    let ret = __nfs4_copy_file_range(file_in, pos_in, file_out, pos_out, count, flags);
    if ret == neg(EOPNOTSUPP) as isize || ret == neg(EXDEV) as isize {
        rust_nfs4_splice_copy(file_in, pos_in, file_out, pos_out, count)
    } else { ret }
}

#[cfg(CONFIG_NFS_V4_2)]
unsafe extern "C" fn nfs4_file_llseek(filep: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    if whence == SEEK_HOLE as c_int || whence == SEEK_DATA as c_int {
        let ret = nfs42_proc_llseek(filep, offset, whence);
        if ret != neg(EOPNOTSUPP) as loff_t { return ret; }
    }
    nfs_file_llseek(filep, offset, whence)
}

#[cfg(CONFIG_NFS_V4_2)]
unsafe extern "C" fn nfs42_fallocate(filep: *mut file, mode: c_int,
    offset: loff_t, len: loff_t) -> c_long {
    let inode = rust_nfs4_file_inode(filep);
    if (*inode).i_mode as u32 & S_IFMT != S_IFREG { return neg(EOPNOTSUPP) as c_long; }
    if mode != 0 && mode != (FALLOC_FL_PUNCH_HOLE | FALLOC_FL_KEEP_SIZE) as c_int
        && mode != FALLOC_FL_ZERO_RANGE as c_int { return neg(EOPNOTSUPP) as c_long; }
    let ret = inode_newsize_ok(inode, offset.wrapping_add(len));
    if ret < 0 { return ret as c_long; }
    if mode & FALLOC_FL_PUNCH_HOLE as c_int != 0 {
        nfs42_proc_deallocate(filep, offset, len) as c_long
    } else if mode & FALLOC_FL_ZERO_RANGE as c_int != 0 {
        nfs42_proc_zero_range(filep, offset, len) as c_long
    } else { nfs42_proc_allocate(filep, offset, len) as c_long }
}

#[cfg(CONFIG_NFS_V4_2)]
unsafe extern "C" fn nfs42_remap_file_range(src_file: *mut file, src_off: loff_t,
    dst_file: *mut file, dst_off: loff_t, count: loff_t, remap_flags: u32) -> loff_t {
    let dst_inode = rust_nfs4_file_inode(dst_file);
    let src_inode = rust_nfs4_file_inode(src_file);
    let bs = (*rust_nfs4_server(dst_inode)).clone_blksize;
    if remap_flags & REMAP_FILE_DEDUP != 0 { return neg(EOPNOTSUPP) as loff_t; }
    if remap_flags & !REMAP_FILE_ADVISORY != 0 { return neg(EINVAL) as loff_t; }
    if rust_nfs4_is_swapfile(dst_inode) || rust_nfs4_is_swapfile(src_inode) {
        return neg(ETXTBSY) as loff_t;
    }
    if bs != 0 {
        let mask = (bs as loff_t).wrapping_sub(1);
        if src_off & mask != 0 || dst_off & mask != 0 { return neg(EINVAL) as loff_t; }
        if count & mask != 0 && rust_nfs4_i_size_read(src_inode) != src_off.wrapping_add(count) {
            return neg(EINVAL) as loff_t;
        }
    }
    lock_two_nondirectories(src_inode, dst_inode);
    let ret = 'locked: {
        rust_nfs4_block_o_direct(src_inode);
        let ret = nfs_sync_inode(src_inode);
        if ret != 0 { break 'locked ret; }
        rust_nfs4_block_o_direct(dst_inode);
        let ret = nfs_sync_inode(dst_inode);
        if ret != 0 { break 'locked ret; }
        let ret = nfs42_proc_clone(src_file, dst_file, src_off, dst_off, count);
        if ret == 0 {
            truncate_inode_pages_range(addr_of_mut!((*dst_inode).i_data), dst_off,
                dst_off.wrapping_add(count).wrapping_sub(1) as u64);
        }
        ret
    };
    unlock_two_nondirectories(src_inode, dst_inode);
    if ret < 0 { ret as loff_t } else { count }
}

#[cfg(CONFIG_NFS_V4_2)]
static mut read_name_gen: c_int = 1;

#[cfg(CONFIG_NFS_V4_2)]
unsafe extern "C" fn __nfs42_ssc_open(ss_mnt: *mut vfsmount, src_fh: *mut nfs_fh,
    stateid: *mut nfs4_stateid) -> *mut file {
    let fattr = nfs_alloc_fattr();
    let server = rust_nfs4_sb((*ss_mnt).mnt_sb);
    if fattr.is_null() { return err_ptr(neg(ENOMEM)); }
    let res = 'out: {
        let status = nfs4_proc_getattr(server, src_fh, fattr, null_mut());
        if status < 0 { break 'out err_ptr(status); }
        if (*fattr).mode as u32 & S_IFMT != S_IFREG { break 'out err_ptr(neg(EBADF)); }
        let name_format = c"ssc_read_%d";
        let len = name_format.to_bytes().len() + 16;
        let read_name: *mut c_char = rust_nfs4_kzalloc(len).cast();
        if read_name.is_null() { break 'out err_ptr(neg(ENOMEM)); }
        let sequence = read_name_gen;
        read_name_gen = read_name_gen.wrapping_add(1);
        snprintf(read_name, len, name_format.as_ptr().cast(), sequence);
        let res = 'out_free_name: {
            let r_ino = nfs_fhget((*ss_mnt).mnt_sb, src_fh, fattr);
            if is_err(r_ino) { break 'out_free_name r_ino.cast(); }
            let filep = alloc_file_pseudo(r_ino, ss_mnt, read_name, O_RDONLY as c_int,
                (*r_ino).__bindgen_anon_3.i_fop);
            if is_err(filep) {
                iput(r_ino);
                break 'out_free_name filep;
            }
            let res = 'out_filep: {
                let ctx = alloc_nfs_open_context((*filep).__bindgen_anon_1.f_path.dentry,
                    rust_nfs4_flags_to_mode((*filep).f_flags as c_int), filep);
                if is_err(ctx) { break 'out_filep ctx.cast(); }
                let res = 'out_ctx: {
                    let sp = nfs4_get_state_owner(server, (*ctx).cred, RUST_NFS4_GFP_KERNEL);
                    if sp.is_null() { break 'out_ctx err_ptr(neg(EINVAL)); }
                    (*ctx).state = nfs4_get_open_state(r_ino, sp);
                    if (*ctx).state.is_null() {
                        nfs4_put_state_owner(sp);
                        break 'out_ctx err_ptr(neg(EINVAL));
                    }
                    let state = (*ctx).state;
                    rust_nfs4_set_bit(NFS_SRV_SSC_COPY_STATE, addr_of_mut!((*state).flags));
                    core::ptr::copy_nonoverlapping(
                        addr_of!((*stateid).__bindgen_anon_1.__bindgen_anon_1.other).cast::<u8>(),
                        addr_of_mut!((*state).open_stateid.__bindgen_anon_1.__bindgen_anon_1.other).cast::<u8>(),
                        NFS4_STATEID_OTHER_SIZE as usize);
                    update_open_stateid(state, stateid, null(), (*filep).f_mode);
                    rust_nfs4_set_bit(NFS_OPEN_STATE, addr_of_mut!((*state).flags));
                    nfs_file_set_open_context(filep, ctx);
                    filep
                };
                put_nfs_open_context(ctx);
                if is_err(res) { break 'out_filep res; }
                file_ra_state_init(addr_of_mut!((*filep).__bindgen_anon_3.f_ra), (*(*(*filep).f_mapping).host).i_mapping);
                // Success owns filep; unlike the unwind path it does not fput.
                break 'out_free_name filep;
            };
            fput(filep);
            res
        };
        kfree(read_name.cast());
        res
    };
    rust_nfs4_free_fattr(fattr);
    res
}

#[cfg(CONFIG_NFS_V4_2)]
unsafe extern "C" fn __nfs42_ssc_close(filep: *mut file) {
    let ctx: *mut nfs_open_context = (*filep).private_data.cast();
    (*(*ctx).state).flags = 0;
}

#[cfg(CONFIG_NFS_V4_2)]
static nfs4_ssc_clnt_ops_tbl: nfs4_ssc_client_ops = nfs4_ssc_client_ops {
    sco_open: Some(__nfs42_ssc_open), sco_close: Some(__nfs42_ssc_close),
};

#[cfg(CONFIG_NFS_V4_2)]
#[no_mangle]
pub unsafe extern "C" fn nfs42_ssc_register_ops() { nfs42_ssc_register(addr_of!(nfs4_ssc_clnt_ops_tbl)); }

#[cfg(CONFIG_NFS_V4_2)]
#[no_mangle]
pub unsafe extern "C" fn nfs42_ssc_unregister_ops() { nfs42_ssc_unregister(addr_of!(nfs4_ssc_clnt_ops_tbl)); }

unsafe extern "C" fn nfs4_setlease(file: *mut file, arg: c_int,
    lease: *mut *mut file_lease, priv_: *mut *mut c_void) -> c_int {
    nfs4_proc_setlease(file, arg, lease, priv_)
}

// The generated C table includes raw pointers. This transparent wrapper keeps
// its C layout and lets the immutable operation table live in read-only data.
#[repr(transparent)]
pub struct Nfs4FileOperations(file_operations);
unsafe impl Sync for Nfs4FileOperations {}
#[no_mangle]
pub static nfs4_file_operations: Nfs4FileOperations = Nfs4FileOperations(file_operations {
    read_iter: Some(nfs_file_read), write_iter: Some(nfs_file_write),
    mmap_prepare: Some(nfs_file_mmap_prepare), open: Some(nfs4_file_open),
    flush: Some(nfs4_file_flush), release: Some(nfs_file_release), fsync: Some(nfs_file_fsync),
    lock: Some(nfs_lock), flock: Some(nfs_flock), splice_read: Some(nfs_file_splice_read),
    splice_write: Some(iter_file_splice_write), check_flags: Some(nfs_check_flags),
    setlease: Some(nfs4_setlease),
    #[cfg(CONFIG_NFS_V4_2)] copy_file_range: Some(nfs4_copy_file_range),
    #[cfg(CONFIG_NFS_V4_2)] llseek: Some(nfs4_file_llseek),
    #[cfg(CONFIG_NFS_V4_2)] fallocate: Some(nfs42_fallocate),
    #[cfg(CONFIG_NFS_V4_2)] remap_file_range: Some(nfs42_remap_file_range),
    #[cfg(not(CONFIG_NFS_V4_2))] llseek: Some(nfs_file_llseek),
    fop_flags: RUST_NFS4_FOP_DONTCACHE,
    ..unsafe { zeroed() }
});
