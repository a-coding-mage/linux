/* SPDX-License-Identifier: GPL-2.0-or-later */
/* NFS filesystem cache interface definitions */

// C header dependencies are supplied by the surrounding translation unit.

#[cfg(feature = "CONFIG_NFS_FSCACHE")]
#[repr(C)]
pub struct nfs_fscache_inode_auxdata {
    pub mtime_sec: i64,
    pub mtime_nsec: i64,
    pub ctime_sec: i64,
    pub ctime_nsec: i64,
    pub change_attr: u64,
}

#[cfg(feature = "CONFIG_NFS_FSCACHE")]
#[repr(C)]
pub struct nfs_netfs_io_data {
    pub refcount: refcount_t,
    pub sreq: *mut netfs_io_subrequest,
    pub transferred: atomic64_t,
    pub error: i32,
}

#[cfg(feature = "CONFIG_NFS_FSCACHE")]
#[inline]
pub unsafe fn nfs_netfs_get(netfs: *mut nfs_netfs_io_data) {
    refcount_inc(&mut (*netfs).refcount);
}

#[cfg(feature = "CONFIG_NFS_FSCACHE")]
#[inline]
pub unsafe fn nfs_netfs_put(netfs: *mut nfs_netfs_io_data) {
    /* Only the last RPC completion should call netfs_subreq_terminated() */
    if !refcount_dec_and_test(&mut (*netfs).refcount) {
        return;
    }
    /* Correct the final length to avoid netfs's "Subreq overread" warning. */
    (*(*netfs).sreq).transferred = core::cmp::min(
        (*(*netfs).sreq).len,
        atomic64_read(&(*netfs).transferred),
    );
    (*(*netfs).sreq).error = (*netfs).error;
    netfs_read_subreq_terminated((*netfs).sreq);
    kfree(netfs);
}

#[cfg(feature = "CONFIG_NFS_FSCACHE")]
#[inline]
pub unsafe fn nfs_netfs_inode_init(nfsi: *mut nfs_inode) {
    netfs_inode_init(&mut (*nfsi).netfs, &nfs_netfs_ops, false);
}

#[cfg(feature = "CONFIG_NFS_FSCACHE")]
extern "C" {
    pub fn nfs_netfs_initiate_read(hdr: *mut nfs_pgio_header);
    pub fn nfs_netfs_read_completion(hdr: *mut nfs_pgio_header);
    pub fn nfs_netfs_folio_unlock(folio: *mut folio) -> i32;
    pub fn nfs_fscache_get_super_cookie(sb: *mut super_block, name: *const i8, len: i32) -> i32;
    pub fn nfs_fscache_release_super_cookie(sb: *mut super_block);
    pub fn nfs_fscache_init_inode(inode: *mut inode);
    pub fn nfs_fscache_clear_inode(inode: *mut inode);
    pub fn nfs_fscache_open_file(inode: *mut inode, file: *mut file);
    pub fn nfs_fscache_release_file(inode: *mut inode, file: *mut file);
    pub fn nfs_netfs_readahead(ractl: *mut readahead_control) -> i32;
    pub fn nfs_netfs_read_folio(file: *mut file, folio: *mut folio) -> i32;
}

#[cfg(feature = "CONFIG_NFS_FSCACHE")]
#[inline]
pub unsafe fn nfs_fscache_release_folio(folio: *mut folio, gfp: gfp_t) -> bool {
    if folio_test_private_2(folio) {
        if current_is_kswapd() || (gfp & __GFP_FS) == 0 { return false; }
        folio_wait_private_2(folio);
    }
    fscache_note_page_release(netfs_i_cookie(netfs_inode((*(*folio).mapping).host)));
    true
}

#[cfg(feature = "CONFIG_NFS_FSCACHE")]
#[inline]
pub unsafe fn nfs_fscache_update_auxdata(auxdata: *mut nfs_fscache_inode_auxdata, inode: *mut inode) {
    core::ptr::write_bytes(auxdata, 0, 1);
    (*auxdata).mtime_sec = inode_get_mtime(inode).tv_sec;
    (*auxdata).mtime_nsec = inode_get_mtime(inode).tv_nsec;
    (*auxdata).ctime_sec = inode_get_ctime(inode).tv_sec;
    (*auxdata).ctime_nsec = inode_get_ctime(inode).tv_nsec;
    if (*(*NFS_SERVER(inode)).nfs_client).rpc_ops.version == 4 {
        (*auxdata).change_attr = inode_peek_iversion_raw(inode);
    }
}

#[cfg(feature = "CONFIG_NFS_FSCACHE")]
#[inline]
pub unsafe fn nfs_fscache_invalidate(inode: *mut inode, flags: i32) {
    let mut auxdata = core::mem::MaybeUninit::<nfs_fscache_inode_auxdata>::uninit();
    let cookie = netfs_i_cookie(&mut (*NFS_I(inode)).netfs);
    nfs_fscache_update_auxdata(auxdata.as_mut_ptr(), inode);
    fscache_invalidate(cookie, auxdata.as_ptr(), i_size_read(inode), flags);
}

#[cfg(feature = "CONFIG_NFS_FSCACHE")]
#[inline]
pub unsafe fn nfs_server_fscache_state(server: *mut nfs_server) -> *const i8 {
    if (*server).fscache { b"yes\0".as_ptr() as *const i8 } else { b"no \0".as_ptr() as *const i8 }
}

#[cfg(feature = "CONFIG_NFS_FSCACHE")]
#[inline] pub unsafe fn nfs_netfs_set_pgio_header(hdr: *mut nfs_pgio_header, desc: *mut nfs_pageio_descriptor) { (*hdr).netfs = (*desc).pg_netfs; }
#[cfg(feature = "CONFIG_NFS_FSCACHE")]
#[inline] pub unsafe fn nfs_netfs_set_pageio_descriptor(desc: *mut nfs_pageio_descriptor, hdr: *mut nfs_pgio_header) { (*desc).pg_netfs = (*hdr).netfs; }
#[cfg(feature = "CONFIG_NFS_FSCACHE")]
#[inline] pub unsafe fn nfs_netfs_reset_pageio_descriptor(desc: *mut nfs_pageio_descriptor) { (*desc).pg_netfs = core::ptr::null_mut(); }

#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_netfs_inode_init(_: *mut nfs_inode) {}
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_netfs_initiate_read(_: *mut nfs_pgio_header) {}
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_netfs_read_completion(_: *mut nfs_pgio_header) {}
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_netfs_folio_unlock(_: *mut folio) -> i32 { 1 }
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_fscache_release_super_cookie(_: *mut super_block) {}
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_fscache_init_inode(_: *mut inode) {}
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_fscache_clear_inode(_: *mut inode) {}
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_fscache_open_file(_: *mut inode, _: *mut file) {}
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_fscache_release_file(_: *mut inode, _: *mut file) {}
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_netfs_readahead(_: *mut readahead_control) -> i32 { -ENOBUFS }
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_netfs_read_folio(_: *mut file, _: *mut folio) -> i32 { -ENOBUFS }
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_fscache_release_folio(_: *mut folio, _: gfp_t) -> bool { true }
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_fscache_invalidate(_: *mut inode, _: i32) {}
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_server_fscache_state(_: *mut nfs_server) -> *const i8 { b"no \0".as_ptr() as *const i8 }
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_netfs_set_pgio_header(_: *mut nfs_pgio_header, _: *mut nfs_pageio_descriptor) {}
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_netfs_set_pageio_descriptor(_: *mut nfs_pageio_descriptor, _: *mut nfs_pgio_header) {}
#[cfg(not(feature = "CONFIG_NFS_FSCACHE"))]
#[inline] pub unsafe fn nfs_netfs_reset_pageio_descriptor(_: *mut nfs_pageio_descriptor) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
