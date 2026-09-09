// SPDX-License-Identifier: GPL-2.0
/*
   BNEP implementation for Linux Bluetooth stack (BlueZ).
   Copyright (C) 2001-2002 Inventel Systemes
   Written 2001-2002 by
	David Libault  <david.libault@inventel.fr>

   Copyright (C) 2002 Maxim Krasnyansky <maxk@qualcomm.com>

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
   OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT OF THIRD PARTY RIGHTS.
   IN NO EVENT SHALL THE COPYRIGHT HOLDER(S) AND AUTHOR(S) BE LIABLE FOR ANY
   CLAIM, OR ANY SPECIAL INDIRECT OR CONSEQUENTIAL DAMAGES, OR ANY DAMAGES
   WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
   ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR
   IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

   ALL LIABILITY, INCLUDING LIABILITY FOR INFRINGEMENT OF ANY PATENTS,
   COPYRIGHTS, TRADEMARKS OR OTHER RIGHTS, RELATING TO USE OF THIS
   SOFTWARE IS DISCLAIMED.
*/

// Linux kernel dependencies are supplied by the surrounding translation.

static mut BNEP_SK_LIST: bt_sock_list = bt_sock_list {
    lock: __RW_LOCK_UNLOCKED!(BNEP_SK_LIST.lock),
};

unsafe fn bnep_sock_release(sock: *mut socket) -> i32 {
    let sk = (*sock).sk;

    BT_DBG!("sock %p sk %p", sock, sk);

    if sk.is_null() {
        return 0;
    }

    bt_sock_unlink(&mut BNEP_SK_LIST, sk);
    sock_orphan(sk);
    sock_put(sk);
    0
}

unsafe fn do_bnep_sock_ioctl(sock: *mut socket, cmd: u32, argp: *mut core::ffi::c_void) -> i32 {
    let mut cl: bnep_connlist_req = core::mem::zeroed();
    let mut ca: bnep_connadd_req = core::mem::zeroed();
    let mut cd: bnep_conndel_req = core::mem::zeroed();
    let mut ci: bnep_conninfo = core::mem::zeroed();
    let mut nsock: *mut socket;
    let supp_feat: u32 = 1u32 << BNEP_SETUP_RESPONSE;
    let mut err: i32;

    BT_DBG!("cmd %x arg %p", cmd, argp);

    match cmd {
        BNEPCONNADD => {
            if !capable(CAP_NET_ADMIN) { return -EPERM; }
            if copy_from_user(&mut ca, argp, core::mem::size_of::<bnep_connadd_req>()) != 0 { return -EFAULT; }
            nsock = sockfd_lookup(ca.sock, &mut err);
            if nsock.is_null() { return err; }
            if (*(*nsock).sk).sk_state != BT_CONNECTED {
                sockfd_put(nsock);
                return -EBADFD;
            }
            ca.device[core::mem::size_of_val(&ca.device) - 1] = 0;
            err = bnep_add_connection(&mut ca, nsock);
            if err == 0 {
                if copy_to_user(argp, &ca, core::mem::size_of::<bnep_connadd_req>()) != 0 { err = -EFAULT; }
            } else { sockfd_put(nsock); }
            err
        }
        BNEPCONNDEL => {
            if !capable(CAP_NET_ADMIN) { return -EPERM; }
            if copy_from_user(&mut cd, argp, core::mem::size_of::<bnep_conndel_req>()) != 0 { return -EFAULT; }
            bnep_del_connection(&mut cd)
        }
        BNEPGETCONNLIST => {
            if copy_from_user(&mut cl, argp, core::mem::size_of::<bnep_connlist_req>()) != 0 { return -EFAULT; }
            if cl.cnum <= 0 { return -EINVAL; }
            err = bnep_get_connlist(&mut cl);
            if err == 0 && copy_to_user(argp, &cl, core::mem::size_of::<bnep_connlist_req>()) != 0 { return -EFAULT; }
            err
        }
        BNEPGETCONNINFO => {
            if copy_from_user(&mut ci, argp, core::mem::size_of::<bnep_conninfo>()) != 0 { return -EFAULT; }
            err = bnep_get_conninfo(&mut ci);
            if err == 0 && copy_to_user(argp, &ci, core::mem::size_of::<bnep_conninfo>()) != 0 { return -EFAULT; }
            err
        }
        BNEPGETSUPPFEAT => {
            if copy_to_user(argp, &supp_feat, core::mem::size_of::<u32>()) != 0 { return -EFAULT; }
            0
        }
        _ => -EINVAL,
    }
}

unsafe fn bnep_sock_ioctl(sock: *mut socket, cmd: u32, arg: usize) -> i32 {
    do_bnep_sock_ioctl(sock, cmd, arg as *mut core::ffi::c_void)
}

#[cfg(CONFIG_COMPAT)]
unsafe fn bnep_sock_compat_ioctl(sock: *mut socket, cmd: u32, arg: usize) -> i32 {
    let argp = compat_ptr(arg);
    if cmd == BNEPGETCONNLIST {
        let mut cl: bnep_connlist_req = core::mem::zeroed();
        let p = argp as *mut u32;
        let mut uci: u32 = 0;
        if get_user(&mut cl.cnum, p) != 0 || get_user(&mut uci, p.add(1)) != 0 { return -EFAULT; }
        cl.ci = compat_ptr(uci as usize);
        if cl.cnum <= 0 { return -EINVAL; }
        let mut err = bnep_get_connlist(&mut cl);
        if err == 0 && put_user(cl.cnum, p) != 0 { err = -EFAULT; }
        return err;
    }
    do_bnep_sock_ioctl(sock, cmd, argp)
}

static mut BNEP_SOCK_OPS: proto_ops = proto_ops {
    family: PF_BLUETOOTH, owner: THIS_MODULE, release: Some(bnep_sock_release), ioctl: Some(bnep_sock_ioctl),
    #[cfg(CONFIG_COMPAT)] compat_ioctl: Some(bnep_sock_compat_ioctl),
    bind: Some(sock_no_bind), getname: Some(sock_no_getname), sendmsg: Some(sock_no_sendmsg),
    recvmsg: Some(sock_no_recvmsg), listen: Some(sock_no_listen), shutdown: Some(sock_no_shutdown),
    connect: Some(sock_no_connect), socketpair: Some(sock_no_socketpair), accept: Some(sock_no_accept),
    mmap: Some(sock_no_mmap),
};

static mut BNEP_PROTO: proto = proto { name: b"BNEP\0".as_ptr() as *const _, owner: THIS_MODULE, obj_size: core::mem::size_of::<bt_sock>() };

unsafe fn bnep_sock_create(net: *mut net, sock: *mut socket, protocol: i32, kern: i32) -> i32 {
    BT_DBG!("sock %p", sock);
    if (*sock).type_ != SOCK_RAW { return -ESOCKTNOSUPPORT; }
    let sk = bt_sock_alloc(net, sock, &mut BNEP_PROTO, protocol, GFP_ATOMIC, kern);
    if sk.is_null() { return -ENOMEM; }
    (*sock).ops = &mut BNEP_SOCK_OPS;
    (*sock).state = SS_UNCONNECTED;
    bt_sock_link(&mut BNEP_SK_LIST, sk);
    0
}

static BNEP_SOCK_FAMILY_OPS: net_proto_family = net_proto_family { family: PF_BLUETOOTH, owner: THIS_MODULE, create: Some(bnep_sock_create) };

pub unsafe fn bnep_sock_init() -> i32 {
    let mut err = proto_register(&mut BNEP_PROTO, 0);
    if err < 0 { return err; }
    err = bt_sock_register(BTPROTO_BNEP, &BNEP_SOCK_FAMILY_OPS);
    if err < 0 { BT_ERR!("Can't register BNEP socket"); proto_unregister(&mut BNEP_PROTO); return err; }
    err = bt_procfs_init(&init_net, b"bnep\0".as_ptr() as *const _, &mut BNEP_SK_LIST, core::ptr::null_mut());
    if err < 0 { BT_ERR!("Failed to create BNEP proc file"); bt_sock_unregister(BTPROTO_BNEP); proto_unregister(&mut BNEP_PROTO); return err; }
    BT_INFO!("BNEP socket layer initialized");
    0
}

pub unsafe fn bnep_sock_cleanup() {
    bt_procfs_cleanup(&init_net, b"bnep\0".as_ptr() as *const _);
    bt_sock_unregister(BTPROTO_BNEP);
    proto_unregister(&mut BNEP_PROTO);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
