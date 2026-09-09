/*
 *  Copyright (c) 2006,2007 The Regents of the University of Michigan.
 *  All rights reserved.
 *
 *  Andy Adamson <andros@citi.umich.edu>
 *  Fred Isaman <iisaman@umich.edu>
 *
 * Permission and warranty notice retained from the original source.
 */

// Translated dependencies: linux/module.h, linux/blkdev.h, and blocklayout.h.

const NFSDBG_FACILITY: u32 = NFSDBG_PNFS_LD;

unsafe fn nfs4_encode_simple(mut p: *mut __be32, b: *mut pnfs_block_volume) {
    let mut i: c_int = 0;
    *p = cpu_to_be32(1);
    p = p.add(1);
    *p = cpu_to_be32((*b).type_);
    p = p.add(1);
    *p = cpu_to_be32((*b).simple.nr_sigs);
    p = p.add(1);
    while i < (*b).simple.nr_sigs {
        p = xdr_encode_hyper(p, (*b).simple.sigs[i as usize].offset);
        p = xdr_encode_opaque(
            p,
            (*b).simple.sigs[i as usize].sig,
            (*b).simple.sigs[i as usize].sig_len,
        );
        i += 1;
    }
}

pub unsafe fn bl_resolve_deviceid(
    server: *mut nfs_server,
    b: *mut pnfs_block_volume,
    gfp_mask: gfp_t,
) -> dev_t {
    let net = (*(*server).nfs_client).cl_net;
    let nn = net_generic(net, nfs_net_id);
    let reply = &mut (*nn).bl_mount_reply;
    let mut bl_pipe_msg: bl_pipe_msg = core::mem::zeroed();
    let msg: *mut rpc_pipe_msg = &mut bl_pipe_msg.msg;
    let mut bl_msg: *mut bl_msg_hdr;
    let mut wq: wait_queue_entry_t = DECLARE_WAITQUEUE(current);
    let mut dev: dev_t = 0;
    let mut rc: c_int;

    dprintk!("%s CREATING PIPEFS MESSAGE\n", __func__);

    mutex_lock(&mut (*nn).bl_mutex);
    bl_pipe_msg.bl_wq = &mut (*nn).bl_wq;

    (*b).simple.len += 4;
    if (*b).simple.len > PAGE_SIZE {
        mutex_unlock(&mut (*nn).bl_mutex);
        return dev;
    }

    core::ptr::write_bytes(msg, 0, 1);
    (*msg).len = core::mem::size_of::<bl_msg_hdr>() + (*b).simple.len as usize;
    (*msg).data = kzalloc((*msg).len, gfp_mask);
    if (*msg).data.is_null() {
        mutex_unlock(&mut (*nn).bl_mutex);
        return dev;
    }

    bl_msg = (*msg).data as *mut bl_msg_hdr;
    (*bl_msg).type_ = BL_DEVICE_MOUNT;
    (*bl_msg).totallen = (*b).simple.len;
    nfs4_encode_simple(
        ((*msg).data as *mut u8).add(core::mem::size_of::<bl_msg_hdr>()) as *mut __be32,
        b,
    );

    dprintk!("%s CALLING USERSPACE DAEMON\n", __func__);
    add_wait_queue(&mut (*nn).bl_wq, &mut wq);
    rc = rpc_queue_upcall((*nn).bl_device_pipe, msg);
    if rc < 0 {
        remove_wait_queue(&mut (*nn).bl_wq, &mut wq);
        kfree((*msg).data);
        mutex_unlock(&mut (*nn).bl_mutex);
        return dev;
    }

    set_current_state(TASK_UNINTERRUPTIBLE);
    schedule();
    remove_wait_queue(&mut (*nn).bl_wq, &mut wq);

    if reply.status != BL_DEVICE_REQUEST_PROC {
        printk!(KERN_WARNING, "%s failed to decode device: %d\n", __func__, reply.status);
        kfree((*msg).data);
        mutex_unlock(&mut (*nn).bl_mutex);
        return dev;
    }

    dev = MKDEV(reply.major, reply.minor);
    kfree((*msg).data);
    mutex_unlock(&mut (*nn).bl_mutex);
    dev
}

unsafe fn bl_pipe_downcall(filp: *mut file, src: *const c_char, mlen: usize) -> ssize_t {
    let nn = net_generic((*file_inode(filp)).i_sb.s_fs_info, nfs_net_id);
    if mlen != core::mem::size_of::<bl_dev_msg>() { return -EINVAL as ssize_t; }
    if copy_from_user(&mut (*nn).bl_mount_reply as *mut _ as *mut c_void, src as *const c_void, mlen) != 0 {
        return -EFAULT as ssize_t;
    }
    wake_up(&mut (*nn).bl_wq);
    mlen as ssize_t
}

unsafe fn bl_pipe_destroy_msg(msg: *mut rpc_pipe_msg) {
    let bl_pipe_msg = container_of!(msg, bl_pipe_msg, msg);
    if (*msg).errno >= 0 { return; }
    wake_up((*bl_pipe_msg).bl_wq);
}

static bl_upcall_ops: rpc_pipe_ops = rpc_pipe_ops {
    upcall: Some(rpc_pipe_generic_upcall),
    downcall: Some(bl_pipe_downcall),
    destroy_msg: Some(bl_pipe_destroy_msg),
};

unsafe fn nfs4blocklayout_register_sb(sb: *mut super_block, pipe: *mut rpc_pipe) -> c_int {
    let dir = rpc_d_lookup_sb(sb, NFS_PIPE_DIRNAME);
    if dir.is_null() { return -ENOENT; }
    let err = rpc_mkpipe_dentry(dir, b"blocklayout\0".as_ptr() as *const c_char, core::ptr::null_mut(), pipe);
    dput(dir);
    err
}

unsafe fn rpc_pipefs_event(_nb: *mut notifier_block, event: c_ulong, ptr: *mut c_void) -> c_int {
    let sb = ptr as *mut super_block;
    let net = (*sb).s_fs_info as *mut net;
    let nn = net_generic(net, nfs_net_id);
    let mut ret = 0;
    if !try_module_get(THIS_MODULE) { return 0; }
    if (*nn).bl_device_pipe.is_null() { module_put(THIS_MODULE); return 0; }
    match event {
        RPC_PIPEFS_MOUNT => ret = nfs4blocklayout_register_sb(sb, (*nn).bl_device_pipe),
        RPC_PIPEFS_UMOUNT => rpc_unlink((*nn).bl_device_pipe),
        _ => ret = -ENOTSUPP,
    }
    module_put(THIS_MODULE);
    ret
}

static mut nfs4blocklayout_block: notifier_block = notifier_block { notifier_call: Some(rpc_pipefs_event) };

unsafe fn nfs4blocklayout_register_net(net: *mut net, pipe: *mut rpc_pipe) -> c_int {
    let pipefs_sb = rpc_get_sb_net(net);
    if pipefs_sb.is_null() { return 0; }
    let ret = nfs4blocklayout_register_sb(pipefs_sb, pipe);
    rpc_put_sb_net(net);
    ret
}

unsafe fn nfs4blocklayout_unregister_net(net: *mut net, pipe: *mut rpc_pipe) {
    let pipefs_sb = rpc_get_sb_net(net);
    if !pipefs_sb.is_null() { rpc_unlink(pipe); rpc_put_sb_net(net); }
}

unsafe fn nfs4blocklayout_net_init(net: *mut net) -> c_int {
    let nn = net_generic(net, nfs_net_id);
    mutex_init(&mut (*nn).bl_mutex);
    init_waitqueue_head(&mut (*nn).bl_wq);
    (*nn).bl_device_pipe = rpc_mkpipe_data(&bl_upcall_ops, 0);
    if IS_ERR((*nn).bl_device_pipe) { return PTR_ERR((*nn).bl_device_pipe); }
    let err = nfs4blocklayout_register_net(net, (*nn).bl_device_pipe);
    if unlikely(err != 0) { rpc_destroy_pipe_data((*nn).bl_device_pipe); }
    err
}

unsafe fn nfs4blocklayout_net_exit(net: *mut net) {
    let nn = net_generic(net, nfs_net_id);
    nfs4blocklayout_unregister_net(net, (*nn).bl_device_pipe);
    rpc_destroy_pipe_data((*nn).bl_device_pipe);
    (*nn).bl_device_pipe = core::ptr::null_mut();
}

static mut nfs4blocklayout_net_ops: pernet_operations = pernet_operations {
    init: Some(nfs4blocklayout_net_init),
    exit: Some(nfs4blocklayout_net_exit),
};

pub unsafe fn bl_init_pipefs() -> c_int {
    let mut ret = rpc_pipefs_notifier_register(&mut nfs4blocklayout_block);
    if ret != 0 { return ret; }
    ret = register_pernet_subsys(&mut nfs4blocklayout_net_ops);
    if ret != 0 { rpc_pipefs_notifier_unregister(&mut nfs4blocklayout_block); }
    ret
}

pub unsafe fn bl_cleanup_pipefs() {
    rpc_pipefs_notifier_unregister(&mut nfs4blocklayout_block);
    unregister_pernet_subsys(&mut nfs4blocklayout_net_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
