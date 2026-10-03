// SPDX-License-Identifier: GPL-2.0-or-later
// Source reconstruction of vfs_cache.c at 2099a6c20c54c99ae1689eb59f5240c54cc2774e.
// Original algorithms remain in Rust; configured headers own every ABI layout.
#![allow(missing_docs, unsafe_op_in_unsafe_fn, non_upper_case_globals)]

#[allow(clippy::all, dead_code, non_camel_case_types, non_snake_case,
         non_upper_case_globals, improper_ctypes, unreachable_pub)]
mod bindings {
    use kernel::ffi;
    type __kernel_size_t = usize;
    type __kernel_ssize_t = isize;
    type __kernel_ptrdiff_t = isize;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/ksmbd_vfs_cache_generated.rs"));
}
use bindings::*;
use kernel::ffi::{c_char, c_int, c_uint, c_ulong, c_long, c_void};
use core::{
    mem::{size_of, offset_of},
    ptr::{addr_of, addr_of_mut, null_mut},
};

// Bind the configured C expression; bindgen need not translate INT_MAX macros.
const KSMBD_NO_FID: c_uint = RVC_NO_FID;
const S_DEL_PENDING: c_uint = 1;
const S_DEL_ON_CLS: c_uint = 2;
const S_DEL_ON_CLS_STREAM: c_uint = 8;
#[link_section = ".data..read_mostly"]
static mut inode_hash_mask: c_uint = 0;
#[link_section = ".data..read_mostly"]
static mut inode_hash_shift: c_uint = 0;
#[link_section = ".data..read_mostly"]
static mut inode_hashtable: *mut hlist_head = null_mut();
// Correspond to C static zero-initialized storage; initialized by original init APIs.
static mut global_ft: ksmbd_file_table = unsafe { core::mem::zeroed() };
static mut fd_limit: atomic_long_t = unsafe { core::mem::zeroed() };
static mut filp_cache: *mut kmem_cache = null_mut();
static mut durable_scavenger_running: bool = false;
static mut dh_wq: wait_queue_head_t = unsafe { core::mem::zeroed() };

unsafe fn has_file_id(id: u64) -> bool { id < KSMBD_NO_FID as u64 }
unsafe fn ksmbd_stream_fd(fp: *mut ksmbd_file) -> bool { !(*fp).stream.name.is_null() }
// fs.h's first anonymous union owns f_path; do not invent a struct file prefix.
unsafe fn file_path(f: *mut file) -> *mut path {
    addr_of_mut!((*f).__bindgen_anon_1.f_path)
}
unsafe fn file_dentry(f: *mut file) -> *mut dentry { (*file_path(f)).dentry }

#[cfg(CONFIG_PROC_FS)]
const fn const_name(value: c_uint, name: &'static core::ffi::CStr) -> ksmbd_const_name {
    ksmbd_const_name { const_value: value, name: name.as_ptr().cast() }
}
#[cfg(CONFIG_PROC_FS)]
static mut lease_names: [ksmbd_const_name; 8] = [
    const_name(RVC_LEASE_NONE, c"LEASE_NONE"),
    const_name(RVC_LEASE_R, c"LEASE_R"),
    const_name(RVC_LEASE_H, c"LEASE_H"),
    const_name(RVC_LEASE_W, c"LEASE_W"),
    const_name(RVC_LEASE_R | RVC_LEASE_H, c"LEASE_RH"),
    const_name(RVC_LEASE_R | RVC_LEASE_W, c"LEASE_RW"),
    const_name(RVC_LEASE_H | RVC_LEASE_W, c"LEASE_WH"),
    const_name(RVC_LEASE_R | RVC_LEASE_H | RVC_LEASE_W, c"LEASE_RWH"),
];
#[cfg(CONFIG_PROC_FS)]
static mut oplock_names: [ksmbd_const_name; 4] = [
    const_name(SMB2_OPLOCK_LEVEL_NONE, c"OPLOCK_NONE"),
    const_name(SMB2_OPLOCK_LEVEL_II, c"OPLOCK_II"),
    const_name(SMB2_OPLOCK_LEVEL_EXCLUSIVE, c"OPLOCK_EXCLUSIVE"),
    const_name(SMB2_OPLOCK_LEVEL_BATCH, c"OPLOCK_BATCH"),
];
#[cfg(CONFIG_PROC_FS)]
static mut state_names: [ksmbd_const_name; 3] = [
    const_name(FP_NEW, c"new"), const_name(FP_INITED, c"open"),
    const_name(FP_CLOSED, c"closed"),
];
#[cfg(CONFIG_PROC_FS)]
static mut flag_names: [ksmbd_const_name; 7] = [
    const_name(1 << 0, c"durable"), const_name(1 << 1, c"persistent"),
    const_name(1 << 2, c"resilient"), const_name(1 << 3, c"delete-on-close"),
    const_name(1 << 4, c"stream"), const_name(1 << 5, c"posix"),
    const_name(1 << 6, c"attrib-only"),
];
#[cfg(CONFIG_PROC_FS)]
unsafe fn ksmbd_proc_file_flags(fp: *mut ksmbd_file) -> c_uint {
    let mut flags = 0;
    if (*fp).is_durable { flags |= 1 << 0; }
    if (*fp).is_persistent { flags |= 1 << 1; }
    if (*fp).is_resilient { flags |= 1 << 2; }
    if (*fp).coption & RVC_DELETE_ON_CLOSE != 0 { flags |= 1 << 3; }
    if !(*fp).stream.name.is_null() { flags |= 1 << 4; }
    if (*fp).is_posix_ctxt { flags |= 1 << 5; }
    if (*fp).attrib_only { flags |= 1 << 6; }
    flags
}
#[cfg(CONFIG_PROC_FS)]
unsafe extern "C" fn proc_show_files(m: *mut seq_file, _v: *mut c_void) -> c_int {
    rvc_read_lock(addr_of_mut!(global_ft.lock));
    let mut id: c_uint = 0;
    loop {
        let fp = idr_get_next(global_ft.idr, addr_of_mut!(id).cast()).cast::<ksmbd_file>();
        if fp.is_null() { break; }
        let tree_id = if (*fp).tcon.is_null() { 0 } else { (*(*fp).tcon).id };
        seq_printf(m, c"tree_id:\t0x%x\n".as_ptr().cast(), tree_id);
        seq_printf(m, c"persistent_id:\t0x%llx\n".as_ptr().cast(), (*fp).persistent_id);
        seq_printf(m, c"volatile_id:\t0x%llx\n".as_ptr().cast(), (*fp).volatile_id);
        seq_printf(m, c"refcount:\t%d\n".as_ptr().cast(), rvc_atomic_read(addr_of!((*fp).refcount)));
        rvc_rcu_read_lock();
        let opinfo = rvc_opinfo_dereference(fp);
        if !opinfo.is_null() {
            let (table, count, level) = if (*opinfo).is_lease {
                (addr_of!(lease_names).cast(), 8,
                 u32::from_le((*(*opinfo).o_lease).state))
            } else {
                (addr_of!(oplock_names).cast(), 4, (*opinfo).level as c_uint)
            };
            rvc_rcu_read_unlock();
            let name = ksmbd_proc_const_name(table, count, level);
            if !name.is_null() { seq_printf(m, c"oplock:\t%s\n".as_ptr().cast(), name); }
            else { seq_printf(m, c"oplock:\t0x%x\n".as_ptr().cast(), level); }
        } else {
            rvc_rcu_read_unlock();
            rvc_seq_puts(m, c"oplock:\tnone\n".as_ptr().cast());
        }
        seq_printf(m, c"state:\t%s\n".as_ptr().cast(),
                   ksmbd_proc_const_name(addr_of!(state_names).cast(), 3, (*fp).f_state));
        seq_printf(m, c"durable_timeout:\t%u\n".as_ptr().cast(), (*fp).durable_timeout);
        seq_printf(m, c"create_options:\t0x%08x\n".as_ptr().cast(), u32::from_le((*fp).coption));
        seq_printf(m, c"desired_access:\t0x%08x\n".as_ptr().cast(), u32::from_le((*fp).daccess));
        seq_printf(m, c"share_access:\t0x%08x\n".as_ptr().cast(), u32::from_le((*fp).saccess));
        rvc_seq_puts(m, c"flags:\t".as_ptr().cast());
        ksmbd_proc_show_flag_names(m, addr_of!(flag_names).cast(), 7, ksmbd_proc_file_flags(fp));
        seq_printf(m, c"\nname:\t%s\n\n".as_ptr().cast(), (*file_dentry((*fp).filp)).__bindgen_anon_1.d_name.name);
        id = id.wrapping_add(1);
    }
    rvc_read_unlock(addr_of_mut!(global_ft.lock));
    0
}
#[cfg(CONFIG_PROC_FS)]
unsafe fn create_proc_files() -> c_int {
    if ksmbd_proc_create(c"files".as_ptr().cast(), Some(proc_show_files), null_mut()).is_null() {
        -(ENOMEM as c_int)
    } else { 0 }
}
#[cfg(not(CONFIG_PROC_FS))]
unsafe fn create_proc_files() -> c_int { 0 }

#[no_mangle]
pub unsafe extern "C" fn ksmbd_durable_scavenger_active() -> bool {
    rvc_mutex_lock(addr_of_mut!(rvc_durable_scavenger_lock));
    let active = durable_scavenger_running;
    rvc_mutex_unlock(addr_of_mut!(rvc_durable_scavenger_lock));
    active
}
#[no_mangle]
pub unsafe extern "C" fn ksmbd_set_fd_limit(limit: c_ulong) {
    rvc_atomic_long_set(addr_of_mut!(fd_limit), core::cmp::min(limit, get_max_files()) as c_long);
}
unsafe fn fd_limit_depleted() -> bool {
    if rvc_atomic_long_dec_return(addr_of_mut!(fd_limit)) >= 0 { return false; }
    rvc_atomic_long_inc(addr_of_mut!(fd_limit));
    true
}
unsafe fn fd_limit_close() { rvc_atomic_long_inc(addr_of_mut!(fd_limit)); }

unsafe fn inode_hash(sb: *mut super_block, hashval: c_ulong) -> c_ulong {
    let mut tmp = hashval.wrapping_mul(sb as c_ulong)
        ^ RVC_GOLDEN_RATIO_PRIME.wrapping_add(hashval) / RVC_CACHE_BYTES;
    tmp ^= (tmp ^ RVC_GOLDEN_RATIO_PRIME) >> inode_hash_shift;
    tmp & inode_hash_mask as c_ulong
}
unsafe fn __ksmbd_inode_lookup(de: *mut dentry) -> *mut ksmbd_inode {
    let head = inode_hashtable.add(inode_hash((*rvc_d_inode(de)).i_sb, de as c_ulong) as usize);
    let mut node = (*head).first;
    while !node.is_null() {
        let ci = node.byte_sub(offset_of!(ksmbd_inode, m_hash)).cast::<ksmbd_inode>();
        if (*ci).m_de == de {
            return if rvc_atomic_inc_not_zero(addr_of_mut!((*ci).m_count)) { ci } else { null_mut() };
        }
        node = (*node).next;
    }
    null_mut()
}
unsafe fn ksmbd_inode_lookup(fp: *mut ksmbd_file) -> *mut ksmbd_inode {
    __ksmbd_inode_lookup(file_dentry((*fp).filp))
}
#[no_mangle]
pub unsafe extern "C" fn ksmbd_inode_lookup_lock(d: *mut dentry) -> *mut ksmbd_inode {
    rvc_read_lock(addr_of_mut!(rvc_inode_hash_lock));
    let ci = __ksmbd_inode_lookup(d);
    rvc_read_unlock(addr_of_mut!(rvc_inode_hash_lock));
    ci
}
#[no_mangle]
pub unsafe extern "C" fn ksmbd_query_inode_status(d: *mut dentry) -> c_int {
    let ci = ksmbd_inode_lookup_lock(d);
    if ci.is_null() { return RVC_INODE_STATUS_UNKNOWN as c_int; }
    down_read(addr_of_mut!((*ci).m_lock));
    let ret = if (*ci).m_flags & S_DEL_PENDING != 0 { RVC_INODE_STATUS_PENDING_DELETE }
        else { RVC_INODE_STATUS_OK };
    up_read(addr_of_mut!((*ci).m_lock));
    ksmbd_inode_put(ci);
    ret as c_int
}
#[no_mangle]
pub unsafe extern "C" fn ksmbd_inode_pending_delete(fp: *mut ksmbd_file) -> bool {
    let ci = (*fp).f_ci;
    down_read(addr_of_mut!((*ci).m_lock));
    let mut ret = (*ci).m_flags & S_DEL_PENDING != 0;
    up_read(addr_of_mut!((*ci).m_lock));
    if ret || !ksmbd_stream_fd(fp) { return ret; }
    rvc_spin_lock(addr_of_mut!((*fp).f_lock));
    ret = (*fp).stream_del_pending;
    rvc_spin_unlock(addr_of_mut!((*fp).f_lock));
    ret
}
#[no_mangle]
pub unsafe extern "C" fn ksmbd_set_inode_pending_delete(fp: *mut ksmbd_file) {
    let ci = (*fp).f_ci;
    down_write(addr_of_mut!((*ci).m_lock));
    (*ci).m_flags |= S_DEL_PENDING;
    up_write(addr_of_mut!((*ci).m_lock));
}
#[no_mangle]
pub unsafe extern "C" fn ksmbd_clear_inode_pending_delete(fp: *mut ksmbd_file) {
    let ci = (*fp).f_ci;
    down_write(addr_of_mut!((*ci).m_lock));
    (*ci).m_flags &= !S_DEL_PENDING;
    up_write(addr_of_mut!((*ci).m_lock));
}
#[no_mangle]
pub unsafe extern "C" fn ksmbd_has_stream_without_delete_share(fp: *mut ksmbd_file) -> bool {
    if ksmbd_stream_fd(fp) { return false; }
    let ci = (*fp).f_ci;
    let mut ret = false;
    down_read(addr_of_mut!((*ci).m_lock));
    let head = addr_of_mut!((*ci).m_fp_list);
    let mut node = (*head).next;
    while node != head {
        let prev = node.byte_sub(offset_of!(ksmbd_file, node)).cast::<ksmbd_file>();
        node = (*node).next;
        if prev == fp || !ksmbd_stream_fd(prev) { continue; }
        if rvc_file_inode((*fp).filp) != rvc_file_inode((*prev).filp) { continue; }
        if (*prev).saccess & RVC_SHARE_DELETE == 0 { ret = true; break; }
    }
    up_read(addr_of_mut!((*ci).m_lock));
    ret
}
#[no_mangle]
pub unsafe extern "C" fn ksmbd_fd_set_delete_on_close(fp: *mut ksmbd_file, _file_info: c_int) {
    let ci = (*fp).f_ci;
    down_write(addr_of_mut!((*ci).m_lock));
    (*ci).m_flags |= if ksmbd_stream_fd(fp) { S_DEL_ON_CLS_STREAM } else { S_DEL_ON_CLS };
    up_write(addr_of_mut!((*ci).m_lock));
}
#[no_mangle]
pub unsafe extern "C" fn ksmbd_fd_set_delete_pending(fp: *mut ksmbd_file) {
    if ksmbd_stream_fd(fp) {
        rvc_spin_lock(addr_of_mut!((*fp).f_lock));
        (*fp).stream_del_pending = true;
        rvc_spin_unlock(addr_of_mut!((*fp).f_lock));
    } else { ksmbd_set_inode_pending_delete(fp); }
}
#[no_mangle]
pub unsafe extern "C" fn ksmbd_fd_clear_delete_pending(fp: *mut ksmbd_file) {
    if ksmbd_stream_fd(fp) {
        rvc_spin_lock(addr_of_mut!((*fp).f_lock));
        (*fp).stream_del_pending = false;
        rvc_spin_unlock(addr_of_mut!((*fp).f_lock));
    } else { ksmbd_clear_inode_pending_delete(fp); }
}
unsafe fn ksmbd_inode_hash(ci: *mut ksmbd_inode) {
    let de = (*ci).m_de;
    let bucket = inode_hashtable.add(inode_hash((*rvc_d_inode(de)).i_sb, de as c_ulong) as usize);
    rvc_hlist_add_head(addr_of_mut!((*ci).m_hash), bucket);
}
unsafe fn ksmbd_inode_unhash(ci: *mut ksmbd_inode) {
    rvc_write_lock(addr_of_mut!(rvc_inode_hash_lock));
    rvc_hlist_del_init(addr_of_mut!((*ci).m_hash));
    rvc_write_unlock(addr_of_mut!(rvc_inode_hash_lock));
}
unsafe fn ksmbd_inode_init(ci: *mut ksmbd_inode, fp: *mut ksmbd_file) -> c_int {
    rvc_atomic_set(addr_of_mut!((*ci).m_count), 1);
    rvc_atomic_set(addr_of_mut!((*ci).op_count), 0);
    rvc_atomic_set(addr_of_mut!((*ci).sop_count), 0);
    (*ci).m_flags = 0;
    (*ci).m_fattr = 0;
    rvc_list_init(addr_of_mut!((*ci).m_fp_list));
    rvc_list_init(addr_of_mut!((*ci).m_op_list));
    rvc_init_rwsem(addr_of_mut!((*ci).m_lock));
    (*ci).m_de = file_dentry((*fp).filp);
    0
}
unsafe fn ksmbd_inode_get(fp: *mut ksmbd_file) -> *mut ksmbd_inode {
    rvc_read_lock(addr_of_mut!(rvc_inode_hash_lock));
    let mut ci = ksmbd_inode_lookup(fp);
    rvc_read_unlock(addr_of_mut!(rvc_inode_hash_lock));
    if !ci.is_null() { return ci; }
    ci = rvc_kmalloc(size_of::<ksmbd_inode>(), RVC_DEFAULT_GFP).cast();
    if ci.is_null() { return null_mut(); }
    if ksmbd_inode_init(ci, fp) != 0 {
        rvc_err_inode_init();
        kfree(ci.cast());
        return null_mut();
    }
    rvc_write_lock(addr_of_mut!(rvc_inode_hash_lock));
    let existing = ksmbd_inode_lookup(fp);
    if existing.is_null() { ksmbd_inode_hash(ci); }
    else { kfree(ci.cast()); ci = existing; }
    rvc_write_unlock(addr_of_mut!(rvc_inode_hash_lock));
    ci
}
unsafe fn ksmbd_inode_free(ci: *mut ksmbd_inode) { ksmbd_inode_unhash(ci); kfree(ci.cast()); }
#[no_mangle]
pub unsafe extern "C" fn ksmbd_inode_put(ci: *mut ksmbd_inode) {
    if rvc_atomic_dec_and_test(addr_of_mut!((*ci).m_count)) { ksmbd_inode_free(ci); }
}
#[no_mangle]
#[link_section = ".init.text"]
pub unsafe extern "C" fn ksmbd_inode_hash_init() -> c_int {
    let numentries: c_ulong = 16384;
    inode_hash_shift = numentries.ilog2();
    inode_hash_mask = (1_u32 << inode_hash_shift) - 1;
    let size = (size_of::<hlist_head>() as c_ulong) << inode_hash_shift;
    inode_hashtable = rvc_vmalloc(size).cast();
    if inode_hashtable.is_null() { return -(ENOMEM as c_int); }
    for index in 0..(1_u32 << inode_hash_shift) { rvc_hlist_init(inode_hashtable.add(index as usize)); }
    0
}
#[no_mangle]
pub unsafe extern "C" fn ksmbd_release_inode_hash() { vfree(inode_hashtable.cast()); }

unsafe fn __ksmbd_inode_close(fp: *mut ksmbd_file) {
    let ci = (*fp).f_ci;
    let filp = (*fp).filp;
    if ksmbd_stream_fd(fp) {
        let mut remove_stream_xattr = false;
        down_write(addr_of_mut!((*ci).m_lock));
        if (*ci).m_flags & S_DEL_ON_CLS_STREAM != 0 {
            (*ci).m_flags &= !S_DEL_ON_CLS_STREAM;
            remove_stream_xattr = true;
        }
        up_write(addr_of_mut!((*ci).m_lock));
        rvc_spin_lock(addr_of_mut!((*fp).f_lock));
        if (*fp).stream_del_pending {
            (*fp).stream_del_pending = false;
            remove_stream_xattr = true;
        }
        rvc_spin_unlock(addr_of_mut!((*fp).f_lock));
        if remove_stream_xattr {
            let saved_cred = rvc_override_creds((*filp).f_cred);
            let err = ksmbd_vfs_remove_xattr(rvc_file_mnt_idmap(filp), file_path(filp),
                                           (*fp).stream.name, true);
            rvc_revert_creds(saved_cred);
            if err != 0 { rvc_err_xattr((*fp).stream.name); }
        }
    }
    down_write(addr_of_mut!((*ci).m_lock));
    if (*ci).m_flags & S_DEL_ON_CLS != 0 {
        (*ci).m_flags &= !S_DEL_ON_CLS;
        (*ci).m_flags |= S_DEL_PENDING;
    }
    up_write(addr_of_mut!((*ci).m_lock));
    if rvc_atomic_dec_and_test(addr_of_mut!((*ci).m_count)) {
        let mut do_unlink = false;
        down_write(addr_of_mut!((*ci).m_lock));
        if (*ci).m_flags & S_DEL_PENDING != 0 {
            (*ci).m_flags &= !S_DEL_PENDING;
            do_unlink = true;
        }
        up_write(addr_of_mut!((*ci).m_lock));
        if do_unlink { ksmbd_vfs_unlink(filp); }
        ksmbd_inode_free(ci);
    }
}

include!("vfs_cache_close.rs");
include!("vfs_cache_lifecycle.rs");
