// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/nfs/cache_lib.c
 *
 * Helper routines for the NFS client caches
 *
 * Copyright (c) 2009 Trond Myklebust <Trond.Myklebust@netapp.com>
 */

// C header dependencies are supplied by the surrounding kernel translation.

const NFS_CACHE_UPCALL_PATHLEN: usize = 256;
const NFS_CACHE_UPCALL_TIMEOUT: usize = 15;

static mut nfs_cache_getent_prog: [u8; NFS_CACHE_UPCALL_PATHLEN] = {
    let mut value = [0u8; NFS_CACHE_UPCALL_PATHLEN];
    value[0] = b'/';
    value[1] = b's';
    value[2] = b'b';
    value[3] = b'i';
    value[4] = b'n';
    value[5] = b'/';
    value[6] = b'n';
    value[7] = b'f';
    value[8] = b's';
    value[9] = b'_';
    value[10] = b'c';
    value[11] = b'a';
    value[12] = b'c';
    value[13] = b'h';
    value[14] = b'e';
    value[15] = b'_';
    value[16] = b'g';
    value[17] = b'e';
    value[18] = b't';
    value[19] = b'e';
    value[20] = b'n';
    value
};
static mut nfs_cache_getent_timeout: libc::c_ulong = NFS_CACHE_UPCALL_TIMEOUT as libc::c_ulong;

pub unsafe fn nfs_cache_upcall(cd: *mut cache_detail, entry_name: *mut libc::c_char) -> libc::c_int {
    static mut ENVP: [*mut libc::c_char; 4] = [
        b"HOME=/\0".as_ptr() as *mut libc::c_char,
        b"TERM=linux\0".as_ptr() as *mut libc::c_char,
        b"PATH=/sbin:/usr/sbin:/bin:/usr/bin\0".as_ptr() as *mut libc::c_char,
        core::ptr::null_mut(),
    ];
    let argv: [*mut libc::c_char; 4] = [
        nfs_cache_getent_prog.as_mut_ptr() as *mut libc::c_char,
        (*cd).name,
        entry_name,
        core::ptr::null_mut(),
    ];
    let mut ret: libc::c_int = -EACCES;

    if nfs_cache_getent_prog[0] == b'\0' {
        return if ret > 0 { 0 } else { ret };
    }
    ret = call_usermodehelper(argv[0], argv.as_ptr() as *mut *mut libc::c_char,
        ENVP.as_mut_ptr(), UMH_WAIT_EXEC);
    // Disable the upcall mechanism on ENOENT or EACCES; sysfs may re-enable it.
    if ret == -ENOENT || ret == -EACCES {
        nfs_cache_getent_prog[0] = b'\0';
    }
    if ret > 0 { 0 } else { ret }
}

pub unsafe fn nfs_cache_defer_req_put(dreq: *mut nfs_cache_defer_req) {
    if refcount_dec_and_test(&mut (*dreq).count) {
        kfree(dreq as *mut libc::c_void);
    }
}

unsafe fn nfs_dns_cache_revisit(d: *mut cache_deferred_req, _toomany: libc::c_int) {
    let dreq = container_of!(d, nfs_cache_defer_req, deferred_req);
    complete(&mut (*dreq).completion);
    nfs_cache_defer_req_put(dreq);
}

unsafe fn nfs_dns_cache_defer(req: *mut cache_req) -> *mut cache_deferred_req {
    let dreq = container_of!(req, nfs_cache_defer_req, req);
    (*dreq).deferred_req.revisit = Some(nfs_dns_cache_revisit);
    refcount_inc(&mut (*dreq).count);
    &mut (*dreq).deferred_req
}

pub unsafe fn nfs_cache_defer_req_alloc() -> *mut nfs_cache_defer_req {
    let dreq = kzalloc_obj::<nfs_cache_defer_req>();
    if !dreq.is_null() {
        init_completion(&mut (*dreq).completion);
        refcount_set(&mut (*dreq).count, 1);
        (*dreq).req.defer = Some(nfs_dns_cache_defer);
    }
    dreq
}

pub unsafe fn nfs_cache_wait_for_upcall(dreq: *mut nfs_cache_defer_req) -> libc::c_int {
    if wait_for_completion_timeout(&mut (*dreq).completion,
        nfs_cache_getent_timeout * HZ as libc::c_ulong) == 0 {
        return -ETIMEDOUT;
    }
    0
}

pub unsafe fn nfs_cache_register_sb(sb: *mut super_block, cd: *mut cache_detail) -> libc::c_int {
    let dir = rpc_d_lookup_sb(sb, b"cache\0".as_ptr() as *const libc::c_char);
    let ret = sunrpc_cache_register_pipefs(dir, (*cd).name, 0o600, cd);
    dput(dir);
    ret
}

pub unsafe fn nfs_cache_register_net(net: *mut net, cd: *mut cache_detail) -> libc::c_int {
    let mut ret = 0;
    sunrpc_init_cache_detail(cd);
    let pipefs_sb = rpc_get_sb_net(net);
    if !pipefs_sb.is_null() {
        ret = nfs_cache_register_sb(pipefs_sb, cd);
        rpc_put_sb_net(net);
        if ret != 0 {
            sunrpc_destroy_cache_detail(cd);
        }
    }
    ret
}

pub unsafe fn nfs_cache_unregister_sb(_sb: *mut super_block, cd: *mut cache_detail) {
    sunrpc_cache_unregister_pipefs(cd);
}

pub unsafe fn nfs_cache_unregister_net(net: *mut net, cd: *mut cache_detail) {
    let pipefs_sb = rpc_get_sb_net(net);
    if !pipefs_sb.is_null() {
        nfs_cache_unregister_sb(pipefs_sb, cd);
        rpc_put_sb_net(net);
    }
    sunrpc_destroy_cache_detail(cd);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
