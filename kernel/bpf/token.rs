// SPDX-License-Identifier: GPL-2.0
//! BPF token provider translated from the retained token.c.
//!
//! Policy, callbacks, initialization and ownership transitions live in Rust.
//! Configured bindings supply original C layouts. The C companion supplies
//! only macro/static-inline boundaries and the original operations tables.
#![allow(missing_docs, unsafe_op_in_unsafe_fn)]

#[allow(clippy::all, dead_code, non_camel_case_types, non_snake_case,
         non_upper_case_globals, improper_ctypes, unreachable_pub)]
mod bindings {
    use kernel::ffi;
    type __kernel_size_t = usize;
    type __kernel_ssize_t = isize;
    type __kernel_ptrdiff_t = isize;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/bpf_token_generated.rs"));
}

use bindings::*;
use core::{
    mem::{align_of, offset_of, size_of, MaybeUninit},
    ptr::{self, addr_of, addr_of_mut, null_mut},
};
use kernel::ffi::{c_char, c_int, c_void};

// Values evaluated from the same configured C headers as the bindings.
// Work/refcnt ordering and CONFIG_SECURITY are never handwritten mirrors.
const _: () = {
    assert!(size_of::<bpf_token>() == LUPOS_TOKEN_SIZE as usize);
    assert!(align_of::<bpf_token>() == LUPOS_TOKEN_ALIGN as usize);
    assert!(offset_of!(bpf_token, work) == LUPOS_TOKEN_WORK_OFFSET as usize);
    assert!(offset_of!(bpf_token, refcnt) == LUPOS_TOKEN_REFCNT_OFFSET as usize);
    assert!(offset_of!(bpf_token, userns) == LUPOS_TOKEN_USERNS_OFFSET as usize);
    assert!(offset_of!(bpf_token, allowed_cmds) == LUPOS_TOKEN_CMDS_OFFSET as usize);
    assert!(offset_of!(bpf_token, allowed_maps) == LUPOS_TOKEN_MAPS_OFFSET as usize);
    assert!(offset_of!(bpf_token, allowed_progs) == LUPOS_TOKEN_PROGS_OFFSET as usize);
    assert!(offset_of!(bpf_token, allowed_attachs) == LUPOS_TOKEN_ATTACHS_OFFSET as usize);
    assert!(size_of::<bpf_token_info>() == LUPOS_TOKEN_INFO_SIZE as usize);
    assert!(align_of::<bpf_token_info>() == LUPOS_TOKEN_INFO_ALIGN as usize);
    assert!(offset_of!(bpf_token_info, allowed_cmds) == LUPOS_TOKEN_INFO_CMDS_OFFSET as usize);
    assert!(offset_of!(bpf_token_info, allowed_maps) == LUPOS_TOKEN_INFO_MAPS_OFFSET as usize);
    assert!(offset_of!(bpf_token_info, allowed_progs) == LUPOS_TOKEN_INFO_PROGS_OFFSET as usize);
    assert!(offset_of!(bpf_token_info, allowed_attachs) == LUPOS_TOKEN_INFO_ATTACHS_OFFSET as usize);
    assert!(LUPOS_TOKEN_MAX_CMD < 64);
    assert!(LUPOS_TOKEN_MAX_MAP_TYPE < 64);
    assert!(LUPOS_TOKEN_MAX_PROG_TYPE < 64);
    assert!(LUPOS_TOKEN_MAX_ATTACH_TYPE < 64);
};

// Guard declaration order matches C cleanup order: prepared fd/file,
// borrowed fd, then untransferred token storage.
struct TokenAllocation(*mut bpf_token);
impl Drop for TokenAllocation {
    fn drop(&mut self) { unsafe { kfree(self.0.cast()); } }
}
struct FetchedFd(fd);
impl FetchedFd {
    unsafe fn file(&self) -> *mut file { lupos_token_fd_file(addr_of!(self.0)) }
}
impl Drop for FetchedFd {
    fn drop(&mut self) { unsafe { lupos_token_fdput(addr_of!(self.0)); } }
}
struct PreparedFd(fd_prepare);
impl Drop for PreparedFd {
    fn drop(&mut self) { unsafe { lupos_token_fd_prepare_cleanup(addr_of!(self.0)); } }
}

unsafe fn bpf_ns_capable(ns: *mut user_namespace, cap: c_int) -> bool {
    lupos_token_ns_capable(ns, cap)
        || (cap != LUPOS_TOKEN_CAP_SYS_ADMIN as c_int
            && lupos_token_ns_capable(ns, LUPOS_TOKEN_CAP_SYS_ADMIN as c_int))
}

#[no_mangle]
pub unsafe extern "C" fn bpf_token_capable(token: *const bpf_token, cap: c_int) -> bool {
    let userns = if token.is_null() { addr_of!(init_user_ns).cast_mut() } else { (*token).userns };
    if !bpf_ns_capable(userns, cap) { return false; }
    if !token.is_null() && lupos_token_security_capable(token, cap) < 0 { return false; }
    true
}

#[no_mangle]
pub unsafe extern "C" fn bpf_token_inc(token: *mut bpf_token) {
    lupos_token_atomic64_inc(addr_of_mut!((*token).refcnt));
}

unsafe fn bpf_token_free(token: *mut bpf_token) {
    lupos_token_security_free(token);
    lupos_token_put_user_ns((*token).userns);
    kfree(token.cast());
}

unsafe extern "C" fn bpf_token_put_deferred(work: *mut work_struct) {
    let token = work.cast::<u8>().sub(offset_of!(bpf_token, work)).cast::<bpf_token>();
    bpf_token_free(token);
}

#[no_mangle]
pub unsafe extern "C" fn bpf_token_put(token: *mut bpf_token) {
    if token.is_null() { return; }
    if !lupos_token_atomic64_dec_and_test(addr_of_mut!((*token).refcnt)) { return; }
    lupos_token_init_work(addr_of_mut!((*token).work), Some(bpf_token_put_deferred));
    lupos_token_schedule_work(addr_of_mut!((*token).work));
}

#[no_mangle]
pub unsafe extern "C" fn lupos_bpf_token_release(_inode: *mut inode, filp: *mut file) -> c_int {
    bpf_token_put((*filp).private_data.cast());
    0
}

#[no_mangle]
pub unsafe extern "C" fn lupos_bpf_token_show_fdinfo(m: *mut seq_file, filp: *mut file) {
    let token = (*filp).private_data.cast::<bpf_token>();
    let mask = (1u64 << LUPOS_TOKEN_MAX_CMD) - 1;
    if ((*token).allowed_cmds & mask) == mask {
        seq_printf(m, b"allowed_cmds:\tany\n\0".as_ptr().cast::<c_char>());
    } else {
        seq_printf(m, b"allowed_cmds:\t0x%llx\n\0".as_ptr().cast::<c_char>(), (*token).allowed_cmds);
    }
    let mask = (1u64 << LUPOS_TOKEN_MAX_MAP_TYPE) - 1;
    if ((*token).allowed_maps & mask) == mask {
        seq_printf(m, b"allowed_maps:\tany\n\0".as_ptr().cast::<c_char>());
    } else {
        seq_printf(m, b"allowed_maps:\t0x%llx\n\0".as_ptr().cast::<c_char>(), (*token).allowed_maps);
    }
    let mask = (1u64 << LUPOS_TOKEN_MAX_PROG_TYPE) - 1;
    if ((*token).allowed_progs & mask) == mask {
        seq_printf(m, b"allowed_progs:\tany\n\0".as_ptr().cast::<c_char>());
    } else {
        seq_printf(m, b"allowed_progs:\t0x%llx\n\0".as_ptr().cast::<c_char>(), (*token).allowed_progs);
    }
    let mask = (1u64 << LUPOS_TOKEN_MAX_ATTACH_TYPE) - 1;
    if ((*token).allowed_attachs & mask) == mask {
        seq_printf(m, b"allowed_attachs:\tany\n\0".as_ptr().cast::<c_char>());
    } else {
        seq_printf(m, b"allowed_attachs:\t0x%llx\n\0".as_ptr().cast::<c_char>(), (*token).allowed_attachs);
    }
}

#[no_mangle]
pub unsafe extern "C" fn bpf_token_create(attr: *mut bpf_attr) -> c_int {
    let mut token = TokenAllocation(null_mut());
    let f = FetchedFd(fdget((*attr).token_create.bpffs_fd));
    if lupos_token_fd_empty(addr_of!(f.0)) { return -(EBADF as c_int); }

    let path = (*f.file()).__bindgen_anon_1.f_path;
    let sb = (*path.dentry).d_sb;
    if path.dentry != (*sb).s_root { return -(EINVAL as c_int); }
    if (*sb).s_op != addr_of!(bpf_super_ops) { return -(EINVAL as c_int); }
    let err = lupos_token_path_permission(addr_of!(path), LUPOS_TOKEN_MAY_ACCESS as c_int);
    if err != 0 { return err; }

    let userns = (*sb).s_user_ns;
    // Creation requires the exact owning user namespace and CAP_BPF there.
    // CAP_SYS_ADMIN fallback belongs only to bpf_token_capable().
    if lupos_token_current_user_ns() != userns { return -(EPERM as c_int); }
    if !lupos_token_ns_capable(userns, LUPOS_TOKEN_CAP_BPF as c_int) { return -(EPERM as c_int); }
    if lupos_token_current_user_ns() == addr_of!(init_user_ns).cast_mut() {
        return -(EOPNOTSUPP as c_int);
    }

    let mnt_opts = (*sb).s_fs_info.cast::<bpf_mount_opts>();
    if (*mnt_opts).delegate_cmds == 0 && (*mnt_opts).delegate_maps == 0
        && (*mnt_opts).delegate_progs == 0 && (*mnt_opts).delegate_attachs == 0
    {
        return -(ENOENT as c_int);
    }
    let mode = (LUPOS_TOKEN_S_IFREG as c_int
        | ((LUPOS_TOKEN_S_IRUSR | LUPOS_TOKEN_S_IWUSR) as c_int
            & !lupos_token_current_umask())) as umode_t;
    let inode = bpf_get_inode(sb, null_mut(), mode);
    if lupos_token_is_err(inode.cast()) { return lupos_token_ptr_err(inode.cast()) as c_int; }
    (*inode).i_op = addr_of!(lupos_bpf_token_iops);
    (*inode).__bindgen_anon_3.i_fop = addr_of!(bpf_token_fops);
    clear_nlink(inode);

    // Preserve FD_PREPARE's lazy file allocation and macro cleanup/publish.
    let mut fdf = PreparedFd(lupos_token_fd_prepare(inode, path.mnt));
    if fdf.0.err != 0 { return fdf.0.err; }
    token.0 = lupos_token_zalloc();
    if token.0.is_null() { return -(ENOMEM as c_int); }
    lupos_token_atomic64_set(addr_of_mut!((*token.0).refcnt), 1);
    (*token.0).userns = userns;
    (*token.0).allowed_cmds = (*mnt_opts).delegate_cmds;
    (*token.0).allowed_maps = (*mnt_opts).delegate_maps;
    (*token.0).allowed_progs = (*mnt_opts).delegate_progs;
    (*token.0).allowed_attachs = (*mnt_opts).delegate_attachs;

    let err = lupos_token_security_create(token.0, attr, addr_of!(path));
    if err != 0 { return err; }
    lupos_token_get_user_ns((*token.0).userns);
    (*lupos_token_fd_prepare_file(addr_of!(fdf.0))).private_data = token.0.cast();
    token.0 = null_mut();
    lupos_token_fd_publish(addr_of_mut!(fdf.0))
}

#[no_mangle]
pub unsafe extern "C" fn bpf_token_get_info_by_fd(
    token: *mut bpf_token, attr: *const bpf_attr, uattr: *mut bpf_attr,
) -> c_int {
    let uinfo = lupos_token_u64_to_user_ptr((*attr).info.info);
    let info_len = (*attr).info.info_len.min(size_of::<bpf_token_info>() as u32);
    let mut info = MaybeUninit::<bpf_token_info>::uninit();
    let info_ptr = info.as_mut_ptr();
    // Match memset over the entire C object, including any target padding.
    ptr::write_bytes(info_ptr.cast::<u8>(), 0, size_of::<bpf_token_info>());
    (*info_ptr).allowed_cmds = (*token).allowed_cmds;
    (*info_ptr).allowed_maps = (*token).allowed_maps;
    (*info_ptr).allowed_progs = (*token).allowed_progs;
    (*info_ptr).allowed_attachs = (*token).allowed_attachs;
    // Failed payload copy must not store length; both use checked uaccess.
    if lupos_token_copy_to_user(uinfo, info_ptr.cast::<c_void>(), info_len as usize) != 0
        || lupos_token_put_info_len(info_len, uattr) != 0
    {
        return -(EFAULT as c_int);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_token_get_from_fd(ufd: u32) -> *mut bpf_token {
    let f = FetchedFd(fdget(ufd));
    if lupos_token_fd_empty(addr_of!(f.0)) {
        return lupos_token_err_ptr(-(EBADF as isize)).cast();
    }
    let file = f.file();
    if (*file).f_op != addr_of!(bpf_token_fops) {
        return lupos_token_err_ptr(-(EINVAL as isize)).cast();
    }
    let token = (*file).private_data.cast::<bpf_token>();
    bpf_token_inc(token);
    token
}

#[no_mangle]
pub unsafe extern "C" fn bpf_token_allow_cmd(token: *const bpf_token, cmd: bpf_cmd) -> bool {
    if token.is_null() { return false; }
    // Like C, this entry point receives a valid bpf_cmd from its callers.
    if ((*token).allowed_cmds & (1u64 << cmd)) == 0 { return false; }
    lupos_token_security_cmd(token, cmd) == 0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_token_allow_map_type(token: *const bpf_token, ty: bpf_map_type) -> bool {
    if token.is_null() || ty >= LUPOS_TOKEN_MAX_MAP_TYPE { return false; }
    ((*token).allowed_maps & (1u64 << ty)) != 0
}

#[no_mangle]
pub unsafe extern "C" fn bpf_token_allow_prog_type(
    token: *const bpf_token, prog_type: bpf_prog_type, attach_type: bpf_attach_type,
) -> bool {
    if token.is_null() || prog_type >= LUPOS_TOKEN_MAX_PROG_TYPE
        || attach_type >= LUPOS_TOKEN_MAX_ATTACH_TYPE
    {
        return false;
    }
    ((*token).allowed_progs & (1u64 << prog_type)) != 0
        && ((*token).allowed_attachs & (1u64 << attach_type)) != 0
}
