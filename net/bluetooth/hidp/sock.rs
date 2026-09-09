// SPDX-License-Identifier: GPL-2.0
/*
   HIDP implementation for Linux Bluetooth stack (BlueZ).
   Copyright (C) 2003-2004 Marcel Holtmann <marcel@holtmann.org>

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

// Dependencies supplied by the surrounding kernel and HIDP implementation.

static mut hidp_sk_list: bt_sock_list = bt_sock_list {
    lock: __RW_LOCK_UNLOCKED!(hidp_sk_list.lock),
};

unsafe fn hidp_sock_release(sock: *mut socket) -> c_int {
    let sk = (*sock).sk;

    BT_DBG!("sock %p sk %p", sock, sk);

    if sk.is_null() {
        return 0;
    }

    bt_sock_unlink(&mut hidp_sk_list, sk);
    sock_orphan(sk);
    sock_put(sk);
    0
}

unsafe fn do_hidp_sock_ioctl(
    sock: *mut socket,
    cmd: c_uint,
    argp: *mut c_void,
) -> c_int {
    let mut ca: hidp_connadd_req = core::mem::zeroed();
    let mut cd: hidp_conndel_req = core::mem::zeroed();
    let mut cl: hidp_connlist_req = core::mem::zeroed();
    let mut ci: hidp_conninfo = core::mem::zeroed();
    let mut err: c_int;

    BT_DBG!("cmd %x arg %p", cmd, argp);

    match cmd {
        HIDPCONNADD => {
            if !capable(CAP_NET_ADMIN) { return -EPERM; }
            if copy_from_user(&mut ca, argp, core::mem::size_of::<hidp_connadd_req>()) != 0 { return -EFAULT; }

            let csock = sockfd_lookup(ca.ctrl_sock, &mut err);
            if csock.is_null() { return err; }
            let isock = sockfd_lookup(ca.intr_sock, &mut err);
            if isock.is_null() {
                sockfd_put(csock);
                return err;
            }
            ca.name[core::mem::size_of_val(&ca.name) - 1] = 0;
            err = hidp_connection_add(&mut ca, csock, isock);
            if err == 0 && copy_to_user(argp, &ca, core::mem::size_of::<hidp_connadd_req>()) != 0 { err = -EFAULT; }
            sockfd_put(csock);
            sockfd_put(isock);
            err
        }
        HIDPCONNDEL => {
            if !capable(CAP_NET_ADMIN) { return -EPERM; }
            if copy_from_user(&mut cd, argp, core::mem::size_of::<hidp_conndel_req>()) != 0 { return -EFAULT; }
            hidp_connection_del(&mut cd)
        }
        HIDPGETCONNLIST => {
            if copy_from_user(&mut cl, argp, core::mem::size_of::<hidp_connlist_req>()) != 0 { return -EFAULT; }
            if cl.cnum <= 0 { return -EINVAL; }
            err = hidp_get_connlist(&mut cl);
            if err == 0 && copy_to_user(argp, &cl, core::mem::size_of::<hidp_connlist_req>()) != 0 { return -EFAULT; }
            err
        }
        HIDPGETCONNINFO => {
            if copy_from_user(&mut ci, argp, core::mem::size_of::<hidp_conninfo>()) != 0 { return -EFAULT; }
            err = hidp_get_conninfo(&mut ci);
            if err == 0 && copy_to_user(argp, &ci, core::mem::size_of::<hidp_conninfo>()) != 0 { return -EFAULT; }
            err
        }
        _ => -EINVAL,
    }
}

unsafe fn hidp_sock_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int {
    do_hidp_sock_ioctl(sock, cmd, arg as *mut c_void)
}

#[cfg(CONFIG_COMPAT)]
#[repr(C)]
struct compat_hidp_connadd_req {
    ctrl_sock: c_int,
    intr_sock: c_int,
    parser: u16,
    rd_size: u16,
    rd_data: compat_uptr_t,
    country: u8,
    subclass: u8,
    vendor: u16,
    product: u16,
    version: u16,
    flags: u32,
    idle_to: u32,
    name: [c_char; 128],
}

#[cfg(CONFIG_COMPAT)]
unsafe fn hidp_sock_compat_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int {
    let argp = compat_ptr(arg) as *mut c_void;
    let mut err: c_int;

    if cmd == HIDPGETCONNLIST {
        let mut cl: hidp_connlist_req = core::mem::zeroed();
        let p = argp as *mut u32;
        let mut uci: u32 = 0;
        if get_user(&mut cl.cnum, p) != 0 || get_user(&mut uci, p.add(1)) != 0 { return -EFAULT; }
        cl.ci = compat_ptr(uci as c_ulong);
        if cl.cnum <= 0 { return -EINVAL; }
        err = hidp_get_connlist(&mut cl);
        if err == 0 && put_user(cl.cnum, p) != 0 { err = -EFAULT; }
        return err;
    } else if cmd == HIDPCONNADD {
        let mut ca32: compat_hidp_connadd_req = core::mem::zeroed();
        let mut ca: hidp_connadd_req = core::mem::zeroed();
        if !capable(CAP_NET_ADMIN) { return -EPERM; }
        if copy_from_user(&mut ca32, arg as *mut c_void, core::mem::size_of::<compat_hidp_connadd_req>()) != 0 { return -EFAULT; }
        ca.ctrl_sock = ca32.ctrl_sock; ca.intr_sock = ca32.intr_sock; ca.parser = ca32.parser; ca.rd_size = ca32.rd_size;
        ca.rd_data = compat_ptr(ca32.rd_data as c_ulong); ca.country = ca32.country; ca.subclass = ca32.subclass;
        ca.vendor = ca32.vendor; ca.product = ca32.product; ca.version = ca32.version; ca.flags = ca32.flags; ca.idle_to = ca32.idle_to;
        ca32.name[core::mem::size_of_val(&ca32.name) - 1] = 0;
        core::ptr::copy_nonoverlapping(ca32.name.as_ptr(), ca.name.as_mut_ptr(), 128);
        let csock = sockfd_lookup(ca.ctrl_sock, &mut err); if csock.is_null() { return err; }
        let isock = sockfd_lookup(ca.intr_sock, &mut err); if isock.is_null() { sockfd_put(csock); return err; }
        err = hidp_connection_add(&mut ca, csock, isock);
        if err == 0 && copy_to_user(argp, &ca32, core::mem::size_of::<compat_hidp_connadd_req>()) != 0 { err = -EFAULT; }
        sockfd_put(csock); sockfd_put(isock); return err;
    }
    hidp_sock_ioctl(sock, cmd, arg)
}

static hidp_sock_ops: proto_ops = proto_ops {
    family: PF_BLUETOOTH,
    owner: THIS_MODULE,
    release: Some(hidp_sock_release),
    ioctl: Some(hidp_sock_ioctl),
    #[cfg(CONFIG_COMPAT)]
    compat_ioctl: Some(hidp_sock_compat_ioctl),
    bind: Some(sock_no_bind), getname: Some(sock_no_getname),
    sendmsg: Some(sock_no_sendmsg), recvmsg: Some(sock_no_recvmsg),
    listen: Some(sock_no_listen), shutdown: Some(sock_no_shutdown),
    connect: Some(sock_no_connect), socketpair: Some(sock_no_socketpair),
    accept: Some(sock_no_accept), mmap: Some(sock_no_mmap),
};

static hidp_proto: proto = proto {
    name: "HIDP",
    owner: THIS_MODULE,
    obj_size: core::mem::size_of::<bt_sock>(),
};

static hidp_sock_family_ops: net_proto_family = net_proto_family {
    family: PF_BLUETOOTH,
    owner: THIS_MODULE,
    create: Some(hidp_sock_create),
};
unsafe fn hidp_sock_create(net: *mut net, sock: *mut socket, protocol: c_int, kern: c_int) -> c_int {
    BT_DBG!("sock %p", sock);
    if (*sock).type_ != SOCK_RAW { return -ESOCKTNOSUPPORT; }
    let sk = bt_sock_alloc(net, sock, &hidp_proto, protocol, GFP_ATOMIC, kern);
    if sk.is_null() { return -ENOMEM; }
    (*sock).ops = &hidp_sock_ops;
    (*sock).state = SS_UNCONNECTED;
    bt_sock_link(&mut hidp_sk_list, sk);
    0
}

unsafe fn hidp_init_sockets() -> c_int {
    let mut err = proto_register(&hidp_proto, 0);
    if err < 0 { return err; }
    err = bt_sock_register(BTPROTO_HIDP, &hidp_sock_family_ops);
    if err < 0 { BT_ERR!("Can't register HIDP socket"); proto_unregister(&hidp_proto); return err; }
    err = bt_procfs_init(&init_net, "hidp", &mut hidp_sk_list, core::ptr::null_mut());
    if err < 0 { BT_ERR!("Failed to create HIDP proc file"); bt_sock_unregister(BTPROTO_HIDP); proto_unregister(&hidp_proto); return err; }
    BT_INFO!("HIDP socket layer initialized");
    0
}

unsafe fn hidp_cleanup_sockets() {
    bt_procfs_cleanup(&init_net, "hidp");
    bt_sock_unregister(BTPROTO_HIDP);
    proto_unregister(&hidp_proto);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
