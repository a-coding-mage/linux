// SPDX-License-Identifier: GPL-2.0
/*
 * linux/fs/nfs/nfs4namespace.c
 *
 * Copyright (C) 2005 Trond Myklebust <Trond.Myklebust@netapp.com>
 * - Modified by David Howells <dhowells@redhat.com>
 *
 * NFSv4 namespace
 */

// Kernel headers and local headers from the C translation unit are supplied
// by the surrounding Rust kernel environment.

/* Work out the length that an NFSv4 path would render to as a standard posix
 * path, with a leading slash but no terminating slash.
 */
unsafe fn nfs4_pathname_len(pathname: *const nfs4_pathname) -> isize {
    let mut len: isize = 0;
    let mut i: c_int = 0;

    while i < (*pathname).ncomponents {
        let component: *const nfs4_string = (*pathname).components.add(i as usize);
        if (*component).len > NAME_MAX {
            return -ENAMETOOLONG as isize;
        }
        len += 1 + (*component).len as isize;
        if len > PATH_MAX as isize {
            return -ENAMETOOLONG as isize;
        }
        i += 1;
    }
    len
}

/* Convert the NFSv4 pathname components into a standard posix path. */
unsafe fn nfs4_pathname_string(pathname: *const nfs4_pathname,
                               _len: *mut c_ushort) -> *mut c_char {
    let len = nfs4_pathname_len(pathname);
    if len < 0 {
        return ERR_PTR(len);
    }
    *_len = len as c_ushort;

    let buf = kmalloc((len + 1) as usize, GFP_KERNEL) as *mut c_char;
    if buf.is_null() {
        return ERR_PTR(-ENOMEM as isize);
    }
    let mut p = buf;
    let mut i: c_int = 0;
    while i < (*pathname).ncomponents {
        let component = (*pathname).components.add(i as usize);
        *p = b'/' as c_char;
        p = p.add(1);
        memcpy(p, (*component).data as *const c_void, (*component).len as usize);
        p = p.add((*component).len as usize);
        i += 1;
    }
    *p = 0;
    buf
}

/* Return the path component of "<server>:<path>". */
unsafe fn nfs_path_component(nfspath: *const c_char, end: *const c_char) -> *mut c_char {
    let mut p: *mut c_char;
    if *nfspath == b'[' as c_char {
        p = strchr(nfspath, b']' as c_int);
        if !p.is_null() && p.add(1) < end as *mut c_char && *p.add(1) == b':' as c_char {
            return p.add(2);
        }
    } else {
        p = strchr(nfspath, b':' as c_int);
        if !p.is_null() && p < end as *mut c_char {
            return p.add(1);
        }
    }
    core::ptr::null_mut()
}

/* Determine the mount path as a string. */
unsafe fn nfs4_path(dentry: *mut dentry, buffer: *mut c_char, buflen: isize) -> *mut c_char {
    let mut limit: *mut c_char = core::ptr::null_mut();
    let path = nfs_path(&mut limit, dentry, buffer, buflen, NFS_PATH_CANONICAL);
    if !IS_ERR(path) {
        let path_component = nfs_path_component(path, limit);
        if !path_component.is_null() {
            return path_component;
        }
    }
    path
}

/* Check that fs_locations::fs_root is a prefix for the server path. */
unsafe fn nfs4_validate_fspath(dentry: *mut dentry,
                               locations: *const nfs4_fs_locations,
                               ctx: *mut nfs_fs_context) -> c_int {
    let buf = kmalloc(4096, GFP_KERNEL) as *mut c_char;
    if buf.is_null() { return -ENOMEM; }
    let path = nfs4_path(dentry, buf, 4096);
    if IS_ERR(path) { let e = PTR_ERR(path); kfree(buf as *mut c_void); return e as c_int; }
    let mut len: c_ushort = 0;
    let fs_path = nfs4_pathname_string(&(*locations).fs_path, &mut len);
    if IS_ERR(fs_path) { let e = PTR_ERR(fs_path); kfree(buf as *mut c_void); return e as c_int; }
    let n = strncmp(path, fs_path, len as usize);
    kfree(buf as *mut c_void); kfree(fs_path as *mut c_void);
    if n != 0 { return -ENOENT; }
    0
}

pub unsafe fn nfs_parse_server_name(string: *mut c_char, len: usize,
                                    ss: *mut sockaddr_storage, salen: usize,
                                    net: *mut net, port: c_int) -> usize {
    let sa = ss as *mut sockaddr;
    let mut ret = rpc_pton(net, string, len, sa, salen);
    if ret == 0 {
        ret = rpc_uaddr2sockaddr(net, string, len, sa, salen);
        if ret == 0 { ret = nfs_dns_resolve_name(net, string, len, ss, salen); if ret < 0 { ret = 0; } }
    } else if port != 0 { rpc_set_port(sa, port); }
    ret as usize
}

unsafe fn nfs_find_best_sec(clnt: *mut rpc_clnt, server: *mut nfs_server,
                            flavors: *mut nfs4_secinfo_flavors) -> *mut rpc_clnt {
    let mut i = 0;
    while i < (*flavors).num_flavors {
        let secinfo = (*flavors).flavors.add(i as usize);
        match (*secinfo).flavor { RPC_AUTH_NULL | RPC_AUTH_UNIX | RPC_AUTH_GSS => {
            let pflavor = rpcauth_get_pseudoflavor((*secinfo).flavor, &mut (*secinfo).flavor_info);
            if pflavor != RPC_AUTH_MAXFLAVOR && nfs_auth_info_match(&(*server).auth_info, pflavor) {
                let new = rpc_clone_client_set_auth(clnt, pflavor);
                if IS_ERR(new) { i += 1; continue; }
                let cred = rpcauth_lookupcred((*new).cl_auth, 0);
                if IS_ERR(cred) { rpc_shutdown_client(new); i += 1; continue; }
                put_rpccred(cred); return new;
            }
        }, _ => {} }
        i += 1;
    }
    ERR_PTR(-EPERM as isize)
}

pub unsafe fn nfs4_negotiate_security(clnt: *mut rpc_clnt, inode: *mut inode,
                                      name: *const qstr) -> *mut rpc_clnt {
    let page = alloc_page(GFP_KERNEL);
    if page.is_null() { return ERR_PTR(-ENOMEM as isize); }
    let flavors = page_address(page) as *mut nfs4_secinfo_flavors;
    let err = nfs4_proc_secinfo(inode, name, flavors);
    let new = if err < 0 { ERR_PTR(err as isize) } else { nfs_find_best_sec(clnt, NFS_SERVER(inode), flavors) };
    put_page(page); new
}

unsafe fn try_location(fc: *mut fs_context, location: *const nfs4_fs_location) -> c_int {
    let ctx = nfs_fc2context(fc);
    let mut len: usize = 0;
    let mut s = 0;
    while s < (*location).nservers { let b = (*location).servers.add(s as usize); if (*b).len as usize > len { len = (*b).len as usize; } s += 1; }
    kfree((*ctx).nfs_server.hostname as *mut c_void);
    (*ctx).nfs_server.hostname = kmalloc(len + 1, GFP_KERNEL) as *mut c_char;
    if (*ctx).nfs_server.hostname.is_null() { return -ENOMEM; }
    let export_path = nfs4_pathname_string(&(*location).rootpath, &mut (*ctx).nfs_server.export_path_len);
    if IS_ERR(export_path) { return PTR_ERR(export_path) as c_int; }
    kfree((*ctx).nfs_server.export_path as *mut c_void); (*ctx).nfs_server.export_path = export_path;
    let source = kmalloc(len + 1 + (*ctx).nfs_server.export_path_len as usize + 1, GFP_KERNEL) as *mut c_char;
    if source.is_null() { return -ENOMEM; }
    kfree((*fc).source as *mut c_void); (*fc).source = source;
    s = 0;
    while s < (*location).nservers {
        let b = (*location).servers.add(s as usize);
        if !memchr((*b).data as *const c_void, IPV6_SCOPE_DELIMITER, (*b).len as usize).is_null() { s += 1; continue; }
        (*ctx).nfs_server.addrlen = nfs_parse_server_name((*b).data, (*b).len as usize, &mut (*ctx).nfs_server._address, core::mem::size_of::<sockaddr_storage>(), (*fc).net_ns, 0);
        if (*ctx).nfs_server.addrlen == 0 { s += 1; continue; }
        rpc_set_port(&mut (*ctx).nfs_server.address, NFS_PORT);
        memcpy((*ctx).nfs_server.hostname as *mut c_void, (*b).data as *const c_void, (*b).len as usize); *((*ctx).nfs_server.hostname).add((*b).len as usize) = 0;
        let mut p = source; memcpy(p as *mut c_void, (*b).data as *const c_void, (*b).len as usize); p = p.add((*b).len as usize); *p = b':' as c_char; p = p.add(1); memcpy(p as *mut c_void, (*ctx).nfs_server.export_path as *const c_void, (*ctx).nfs_server.export_path_len as usize); p = p.add((*ctx).nfs_server.export_path_len as usize); *p = 0;
        let ret = nfs4_get_referral_tree(fc); if ret == 0 { return 0; } s += 1;
    }
    -ENOENT
}

unsafe fn nfs_follow_referral(fc: *mut fs_context, locations: *const nfs4_fs_locations) -> c_int {
    if locations.is_null() || (*locations).nlocations <= 0 { return -ENOENT; }
    let ctx = nfs_fc2context(fc); let e = nfs4_validate_fspath((*ctx).clone_data.dentry, locations, ctx); if e < 0 { return e; }
    let mut error = -ENOENT; let mut loc = 0;
    while loc < (*locations).nlocations { let l = (*locations).locations.add(loc as usize); if !l.is_null() && (*l).nservers > 0 && (*l).rootpath.ncomponents != 0 { error = try_location(fc, l); if error == 0 { return 0; } } loc += 1; }
    error
}

unsafe fn nfs_do_refmount(fc: *mut fs_context, client: *mut rpc_clnt) -> c_int {
    let ctx = nfs_fc2context(fc); let page = alloc_page(GFP_KERNEL); if page.is_null() { return -ENOMEM; }
    let fs_locations = kmalloc(core::mem::size_of::<nfs4_fs_locations>(), GFP_KERNEL) as *mut nfs4_fs_locations; if fs_locations.is_null() { __free_page(page); return -ENOMEM; }
    (*fs_locations).fattr = nfs_alloc_fattr(); if (*fs_locations).fattr.is_null() { kfree(fs_locations as *mut c_void); __free_page(page); return -ENOMEM; }
    let dentry = (*ctx).clone_data.dentry; let parent = dget_parent(dentry); let mut err = nfs4_proc_fs_locations(client, d_inode(parent), &(*dentry).d_name, fs_locations, page); dput(parent);
    if err == 0 && (*fs_locations).nlocations > 0 && (*fs_locations).fs_path.ncomponents > 0 { err = nfs_follow_referral(fc, fs_locations); } else if err == 0 { err = -ENOENT; }
    kfree((*fs_locations).fattr as *mut c_void); kfree(fs_locations as *mut c_void); __free_page(page); err
}

pub unsafe fn nfs4_submount(fc: *mut fs_context, _server: *mut nfs_server) -> c_int {
    let ctx = nfs_fc2context(fc); let dentry = (*ctx).clone_data.dentry; let parent = dget_parent(dentry); let dir = d_inode(parent);
    let client = nfs4_proc_lookup_mountpoint(dir, dentry, (*ctx).mntfh, (*ctx).clone_data.fattr); dput(parent); if IS_ERR(client) { return PTR_ERR(client) as c_int; }
    (*ctx).selected_flavor = (*(*client).cl_auth).au_flavor;
    let ret = if (*ctx).clone_data.fattr.valid & NFS_ATTR_FATTR_V4_REFERRAL != 0 { nfs_do_refmount(fc, client) } else { nfs_do_submount(fc) };
    rpc_shutdown_client(client); ret
}

/* Try one location from the fs_locations array. */
unsafe fn nfs4_try_replacing_one_location(server: *mut nfs_server,
                                          location: *const nfs4_fs_location) -> c_int {
    let net = rpc_net_ns((*server).client);
    let sap = kmalloc(core::mem::size_of::<sockaddr_storage>(), GFP_KERNEL) as *mut sockaddr_storage;
    if sap.is_null() { return -ENOMEM; }
    let mut error = -ENOENT;
    let mut s = 0;
    while s < (*location).nservers {
        let buf = (*location).servers.add(s as usize);
        if (*buf).len > 0 && (*buf).len <= PAGE_SIZE && memchr((*buf).data as *const c_void, IPV6_SCOPE_DELIMITER, (*buf).len as usize).is_null() {
            let salen = nfs_parse_server_name((*buf).data, (*buf).len as usize, sap, core::mem::size_of::<sockaddr_storage>(), net, 0);
            if salen != 0 { rpc_set_port(sap as *mut sockaddr, NFS_PORT); let hostname = kmemdup_nul((*buf).data, (*buf).len as usize, GFP_KERNEL); if hostname.is_null() { error = -ENOMEM; break; } error = nfs4_update_server(server, hostname, sap, salen, net); kfree(hostname as *mut c_void); if error == 0 { break; } }
        }
        s += 1;
    }
    kfree(sap as *mut c_void); error
}

pub unsafe fn nfs4_replace_transport(server: *mut nfs_server,
                                      locations: *const nfs4_fs_locations) -> c_int {
    if locations.is_null() || (*locations).nlocations <= 0 { return -ENOENT; }
    let mut error = -ENOENT;
    let mut loc = 0;
    while loc < (*locations).nlocations {
        let location = (*locations).locations.add(loc as usize);
        if !location.is_null() && (*location).nservers > 0 && (*location).rootpath.ncomponents != 0 {
            error = nfs4_try_replacing_one_location(server, location);
            if error == 0 { break; }
        }
        loc += 1;
    }
    error
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
