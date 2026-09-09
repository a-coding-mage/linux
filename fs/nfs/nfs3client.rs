// SPDX-License-Identifier: GPL-2.0-only
// Dependencies supplied by the Linux NFS/RPC headers and local sources are
// intentionally left as external Rust items.

#[cfg(feature = "CONFIG_NFS_V3_ACL")]
static mut NFSACL_RPCSTAT: rpc_stat = rpc_stat { program: &nfsacl_program };

#[cfg(feature = "CONFIG_NFS_V3_ACL")]
static NFSACL_VERSION: [*const rpc_version; 4] = [
    core::ptr::null(),
    core::ptr::null(),
    core::ptr::null(),
    &nfsacl_version3,
];

#[cfg(feature = "CONFIG_NFS_V3_ACL")]
#[no_mangle]
pub static nfsacl_program: rpc_program = rpc_program {
    name: "nfsacl",
    number: NFS_ACL_PROGRAM,
    nrvers: NFSACL_VERSION.len(),
    version: NFSACL_VERSION.as_ptr(),
    stats: unsafe { &NFSACL_RPCSTAT },
};

// Initialise an NFSv3 ACL client connection
#[cfg(feature = "CONFIG_NFS_V3_ACL")]
unsafe fn nfs_init_server_aclclient(server: *mut nfs_server) {
    if (*server).flags & NFS_MOUNT_NOACL != 0 {
        return;
    }

    (*server).client_acl = rpc_bind_new_program((*server).client, &nfsacl_program, 3);
    if IS_ERR((*server).client_acl) {
        (*server).caps &= !NFS_CAP_ACLS;
        return;
    }

    nfs_sysfs_link_rpc_client(server, (*server).client_acl, core::ptr::null_mut());
    // No errors! Assume that Sun nfsacls are supported
    (*server).caps |= NFS_CAP_ACLS;
}

#[cfg(not(feature = "CONFIG_NFS_V3_ACL"))]
unsafe fn nfs_init_server_aclclient(server: *mut nfs_server) {
    (*server).flags &= !NFS_MOUNT_NOACL;
    (*server).caps &= !NFS_CAP_ACLS;
}

#[no_mangle]
pub unsafe fn nfs3_create_server(fc: *mut fs_context) -> *mut nfs_server {
    let server = nfs_create_server(fc);

    // Create a client RPC handle for the NFS v3 ACL management interface
    if !IS_ERR(server) {
        nfs_init_server_aclclient(server);
    }
    server
}

#[no_mangle]
pub unsafe fn nfs3_clone_server(
    source: *mut nfs_server,
    fh: *mut nfs_fh,
    fattr: *mut nfs_fattr,
    flavor: rpc_authflavor_t,
) -> *mut nfs_server {
    let server = nfs_clone_server(source, fh, fattr, flavor);
    if !IS_ERR(server) && !IS_ERR((*source).client_acl) {
        nfs_init_server_aclclient(server);
    }
    server
}

// Set up a pNFS Data Server client over NFSv3.
//
// Return any existing nfs_client that matches server address,port,version
// and minorversion.
//
// For a new nfs_client, use a soft mount (default), a low retrans and a
// low timeout interval so that if a connection is lost, we retry through
// the MDS.
#[no_mangle]
pub unsafe fn nfs3_set_ds_client(
    mds_srv: *mut nfs_server,
    ds_addr: *const sockaddr_storage,
    ds_addrlen: i32,
    mut ds_proto: i32,
    ds_timeo: u32,
    ds_retrans: u32,
) -> *mut nfs_client {
    let mut ds_timeout: rpc_timeout = core::mem::zeroed();
    let connect_timeout = ds_timeo as c_ulong
        .wrapping_mul((ds_retrans + 1) as c_ulong)
        .wrapping_mul(HZ as c_ulong)
        / 10;
    let mds_clp = (*mds_srv).nfs_client;
    let mut cl_init: nfs_client_initdata = nfs_client_initdata {
        addr: ds_addr,
        addrlen: ds_addrlen,
        nodename: (*(*mds_clp).cl_rpcclient).cl_nodename,
        ip_addr: (*mds_clp).cl_ipaddr,
        nfs_mod: &nfs_v3,
        proto: ds_proto,
        net: (*mds_clp).cl_net,
        timeparms: &mut ds_timeout,
        cred: (*mds_srv).cred,
        xprtsec: rpc_xprtsec {
            policy: RPC_XPRTSEC_NONE,
            cert_serial: TLS_NO_CERT,
            privkey_serial: TLS_NO_PRIVKEY,
        },
        connect_timeout,
        reconnect_timeout: connect_timeout,
        ..core::mem::zeroed()
    };
    let mut clp: *mut nfs_client;
    let mut buf = [0i8; INET6_ADDRSTRLEN + 1];

    // fake a hostname because lockd wants it
    if rpc_ntop(ds_addr as *const sockaddr, buf.as_mut_ptr(), buf.len()) <= 0 {
        return ERR_PTR(-EINVAL);
    }
    cl_init.hostname = buf.as_mut_ptr();

    match ds_proto {
        XPRT_TRANSPORT_TCP_TLS => {
            if (*mds_clp).cl_xprtsec.policy != RPC_XPRTSEC_NONE {
                cl_init.xprtsec = (*mds_clp).cl_xprtsec;
            } else {
                ds_proto = XPRT_TRANSPORT_TCP;
            }
            // fallthrough
            if (*mds_clp).cl_nconnect > 1 {
                cl_init.nconnect = (*mds_clp).cl_nconnect;
            }
        }
        XPRT_TRANSPORT_RDMA | XPRT_TRANSPORT_TCP => {
            if (*mds_clp).cl_nconnect > 1 {
                cl_init.nconnect = (*mds_clp).cl_nconnect;
            }
        }
        _ => {}
    }

    if (*mds_srv).flags & NFS_MOUNT_NORESVPORT != 0 {
        __set_bit(NFS_CS_NORESVPORT, &mut cl_init.init_flags);
    }
    if test_bit(NFS_CS_NETUNREACH_FATAL, &(*mds_clp).cl_flags) {
        __set_bit(NFS_CS_NETUNREACH_FATAL, &mut cl_init.init_flags);
    }
    __set_bit(NFS_CS_DS, &mut cl_init.init_flags);

    // Use the MDS nfs_client cl_ipaddr.
    nfs_init_timeout_values(&mut ds_timeout, ds_proto, ds_timeo, ds_retrans);
    clp = nfs_get_client(&mut cl_init);
    clp
}

// EXPORT_SYMBOL_GPL(nfs3_set_ds_client);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
