// SPDX-License-Identifier: GPL-2.0-only

// Translated from the Linux kernel UNIX socket diagnostic implementation.

extern "C" {
    fn smp_load_acquire<T>(ptr: *const T) -> T;
    fn nla_put(nlskb: *mut sk_buff, attr_type: i32, len: usize, data: *const core::ffi::c_void) -> i32;
    fn unix_state_lock(sk: *mut sock);
    fn unix_state_unlock(sk: *mut sock);
    fn d_backing_inode(dentry: *mut dentry) -> *mut inode;
    fn unix_peer_get(sk: *mut sock) -> *mut sock;
    fn sock_i_ino(sk: *mut sock) -> u64;
    fn sock_put(sk: *mut sock);
    fn spin_lock(lock: *mut spinlock);
    fn spin_unlock(lock: *mut spinlock);
    fn nla_reserve(skb: *mut sk_buff, attr_type: i32, len: usize) -> *mut nlattr;
    fn nla_data(attr: *mut nlattr) -> *mut core::ffi::c_void;
    fn sock_diag_save_cookie(sk: *mut sock, cookie: *mut u32);
    fn nlmsg_put(skb: *mut sk_buff, portid: u32, seq: u32, msg_type: u32, len: usize, flags: u16) -> *mut nlmsghdr;
    fn nlmsg_data<T>(nlh: *mut nlmsghdr) -> *mut T;
    fn nlmsg_end(skb: *mut sk_buff, nlh: *mut nlmsghdr);
    fn nlmsg_cancel(skb: *mut sk_buff, nlh: *mut nlmsghdr);
    fn nla_put_u8(skb: *mut sk_buff, attr_type: i32, value: u8) -> i32;
    fn nla_put_u32(skb: *mut sk_buff, attr_type: i32, value: u32) -> i32;
    fn sock_diag_put_meminfo(sk: *mut sock, skb: *mut sk_buff, attr_type: i32) -> i32;
    fn skb_queue_len_lockless(queue: *mut sk_buff_head) -> u32;
    fn unix_inq_len(sk: *mut sock) -> u32;
    fn unix_outq_len(sk: *mut sock) -> u32;
    fn from_kuid_munged(ns: *mut user_namespace, uid: uid_t) -> uid_t;
    fn sk_uid(sk: *mut sock) -> uid_t;
    fn sock_net(sk: *mut sock) -> *mut net;
    fn sk_user_ns(sk: *mut sock) -> *mut user_namespace;
    fn netlink_dump_start(diag_nlsk: *mut sock, skb: *mut sk_buff, h: *mut nlmsghdr, c: *mut netlink_dump_control) -> i32;
    fn nlmsg_len(h: *mut nlmsghdr) -> usize;
    fn nlmsg_new(len: usize, flags: u32) -> *mut sk_buff;
    fn nlmsg_free(skb: *mut sk_buff);
    fn nlmsg_unicast(sk: *mut sock, skb: *mut sk_buff, portid: u32) -> i32;
    fn sock_diag_check_cookie(sk: *mut sock, cookie: *const u32) -> i32;
    fn sock_diag_register(handler: *const sock_diag_handler) -> i32;
    fn sock_diag_unregister(handler: *const sock_diag_handler);
}

#[repr(C)] pub struct sock { pub sk_state: u8, pub sk_type: u16, pub sk_shutdown: u8, pub sk_max_ack_backlog: u32, pub sk_receive_queue: sk_buff_head }
#[repr(C)] pub struct sk_buff { pub sk: *mut sock, pub len: u32 }
#[repr(C)] pub struct sk_buff_head { pub lock: spinlock, pub qlen: u32 }
#[repr(C)] pub struct spinlock;
#[repr(C)] pub struct nlattr;
#[repr(C)] pub struct dentry { pub d_sb: *mut super_block }
#[repr(C)] pub struct super_block { pub s_dev: u64 }
#[repr(C)] pub struct inode { pub i_ino: u64 }
#[repr(C)] pub struct user_namespace;
#[repr(C)] pub struct net { pub unx: unix_table, pub diag_nlsk: *mut sock }
#[repr(C)] pub struct unix_table { pub locks: *mut spinlock, pub buckets: *mut unix_sock_list }
#[repr(C)] pub struct unix_sock_list;
#[repr(C)] pub struct nlmsghdr { pub nlmsg_seq: u32, pub nlmsg_flags: u16 }
#[repr(C)] pub struct netlink_callback { pub nlh: *mut nlmsghdr, pub skb: *mut sk_buff, pub args: [u32; 2] }
#[repr(C)] pub struct netlink_dump_control { pub dump: Option<unsafe extern "C" fn(*mut sk_buff, *mut netlink_callback) -> i32> }
#[repr(C)] pub struct uid_t(pub u32);
#[repr(C)] pub struct unix_diag_req { pub udiag_show: u32, pub udiag_states: u32, pub udiag_ino: u64, pub udiag_cookie: [u32; 2] }
#[repr(C)] pub struct unix_diag_msg { pub udiag_family: u8, pub udiag_type: u8, pub udiag_state: u8, pub pad: u8, pub udiag_ino: u64, pub udiag_cookie: [u32; 2] }
#[repr(C)] pub struct unix_diag_vfs { pub udiag_vfs_ino: u64, pub udiag_vfs_dev: u64 }
#[repr(C)] pub struct unix_diag_rqlen { pub udiag_rqueue: u32, pub udiag_wqueue: u32 }
#[repr(C)] pub struct sock_diag_handler { pub owner: *mut core::ffi::c_void, pub family: u16, pub dump: Option<unsafe extern "C" fn(*mut sk_buff, *mut nlmsghdr) -> i32> }
#[repr(C)] pub struct unix_address { pub len: u32, pub name: *mut sockaddr_un }
#[repr(C)] pub struct sockaddr_un { pub sun_path: [u8; 108] }

const UNIX_DIAG_NAME: i32 = 0;
const UNIX_DIAG_VFS: i32 = 1;
const UNIX_DIAG_PEER: i32 = 2;
const UNIX_DIAG_ICONS: i32 = 3;
const UNIX_DIAG_RQLEN: i32 = 4;
const UNIX_DIAG_MEMINFO: i32 = 5;
const UNIX_DIAG_SHUTDOWN: i32 = 6;
const UNIX_DIAG_UID: i32 = 7;
const UDIAG_SHOW_NAME: u32 = 1;
const UDIAG_SHOW_VFS: u32 = 2;
const UDIAG_SHOW_PEER: u32 = 4;
const UDIAG_SHOW_ICONS: u32 = 8;
const UDIAG_SHOW_RQLEN: u32 = 16;
const UDIAG_SHOW_MEMINFO: u32 = 32;
const UDIAG_SHOW_UID: u32 = 64;
const TCP_LISTEN: u8 = 10;
const UNIX_HASH_SIZE: i32 = 256;
const AF_UNIX: u8 = 1;
const SOCK_DIAG_BY_FAMILY: u32 = 20;
const NLM_F_MULTI: u16 = 2;
const NLM_F_DUMP: u16 = 0x300;
const GFP_KERNEL: u32 = 0x10;
const PAGE_SIZE: usize = 4096;
const EMSGSIZE: i32 = 90;
const EINVAL: i32 = 22;
const ENOENT: i32 = 2;
const ENOMEM: i32 = 12;

#[repr(C)] pub struct unix_sock { pub addr: *mut unix_address, pub path: unix_path }
#[repr(C)] pub struct unix_path { pub dentry: *mut dentry }
extern "C" { fn unix_sk(sk: *mut sock) -> *mut unix_sock; }

unsafe fn sk_diag_dump_name(sk: *mut sock, nlskb: *mut sk_buff) -> i32 {
    /* might or might not have a hash table lock */
    let addr = smp_load_acquire(&(*unix_sk(sk)).addr);
    if addr.is_null() { return 0; }
    nla_put(nlskb, UNIX_DIAG_NAME, ((*addr).len as usize) - 2, (*addr).name.cast::<sockaddr_un>().add(0).as_ref().unwrap().sun_path.as_ptr().cast())
}

unsafe fn sk_diag_dump_vfs(sk: *mut sock, nlskb: *mut sk_buff) -> i32 {
    let mut uv: unix_diag_vfs = core::mem::zeroed();
    unix_state_lock(sk);
    let dentry = (*unix_sk(sk)).path.dentry;
    if !dentry.is_null() {
        uv.udiag_vfs_ino = (*d_backing_inode(dentry)).i_ino;
        uv.udiag_vfs_dev = (*(*dentry).d_sb).s_dev;
    }
    unix_state_unlock(sk);
    if dentry.is_null() { return 0; }
    nla_put(nlskb, UNIX_DIAG_VFS, core::mem::size_of::<unix_diag_vfs>(), (&uv as *const _).cast())
}

unsafe fn sk_diag_dump_peer(sk: *mut sock, nlskb: *mut sk_buff) -> i32 {
    let peer = unix_peer_get(sk);
    if !peer.is_null() { let ino = sock_i_ino(peer); sock_put(peer); return nla_put_u32(nlskb, UNIX_DIAG_PEER, ino as u32); }
    0
}

unsafe fn sk_diag_dump_icons(sk: *mut sock, nlskb: *mut sk_buff) -> i32 {
    if (*sk).sk_state == TCP_LISTEN {
        spin_lock(&mut (*sk).sk_receive_queue.lock);
        let attr = nla_reserve(nlskb, UNIX_DIAG_ICONS, (*sk).sk_receive_queue.qlen as usize * core::mem::size_of::<u32>());
        if attr.is_null() { spin_unlock(&mut (*sk).sk_receive_queue.lock); return -EMSGSIZE; }
        spin_unlock(&mut (*sk).sk_receive_queue.lock);
    }
    0
}

unsafe fn sk_diag_show_rqlen(sk: *mut sock, nlskb: *mut sk_buff) -> i32 {
    let mut rql: unix_diag_rqlen = core::mem::zeroed();
    if (*sk).sk_state == TCP_LISTEN { rql.udiag_rqueue = skb_queue_len_lockless(&mut (*sk).sk_receive_queue); rql.udiag_wqueue = (*sk).sk_max_ack_backlog; }
    else { rql.udiag_rqueue = unix_inq_len(sk); rql.udiag_wqueue = unix_outq_len(sk); }
    nla_put(nlskb, UNIX_DIAG_RQLEN, core::mem::size_of::<unix_diag_rqlen>(), (&rql as *const _).cast())
}

unsafe fn sk_diag_dump_uid(sk: *mut sock, nlskb: *mut sk_buff, user_ns: *mut user_namespace) -> i32 {
    let uid = from_kuid_munged(user_ns, sk_uid(sk));
    nla_put(nlskb, UNIX_DIAG_UID, core::mem::size_of::<uid_t>(), (&uid as *const _).cast())
}

unsafe fn sk_diag_fill(sk: *mut sock, skb: *mut sk_buff, req: *mut unix_diag_req, user_ns: *mut user_namespace, portid: u32, seq: u32, flags: u16, sk_ino: u64) -> i32 {
    let nlh = nlmsg_put(skb, portid, seq, SOCK_DIAG_BY_FAMILY, core::mem::size_of::<unix_diag_msg>(), flags);
    if nlh.is_null() { return -EMSGSIZE; }
    let rep = nlmsg_data::<unix_diag_msg>(nlh);
    (*rep).udiag_family = AF_UNIX;
    (*rep).udiag_type = (*sk).sk_type as u8;
    (*rep).udiag_state = (*sk).sk_state;
    (*rep).pad = 0;
    (*rep).udiag_ino = sk_ino;
    sock_diag_save_cookie(sk, (*rep).udiag_cookie.as_mut_ptr());
    if ((*req).udiag_show & UDIAG_SHOW_NAME) != 0 && sk_diag_dump_name(sk, skb) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    if ((*req).udiag_show & UDIAG_SHOW_VFS) != 0 && sk_diag_dump_vfs(sk, skb) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    if ((*req).udiag_show & UDIAG_SHOW_PEER) != 0 && sk_diag_dump_peer(sk, skb) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    if ((*req).udiag_show & UDIAG_SHOW_ICONS) != 0 && sk_diag_dump_icons(sk, skb) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    if ((*req).udiag_show & UDIAG_SHOW_RQLEN) != 0 && sk_diag_show_rqlen(sk, skb) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    if ((*req).udiag_show & UDIAG_SHOW_MEMINFO) != 0 && sock_diag_put_meminfo(sk, skb, UNIX_DIAG_MEMINFO) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    if nla_put_u8(skb, UNIX_DIAG_SHUTDOWN, (*sk).sk_shutdown) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    if ((*req).udiag_show & UDIAG_SHOW_UID) != 0 && sk_diag_dump_uid(sk, skb, user_ns) != 0 { nlmsg_cancel(skb, nlh); return -EMSGSIZE; }
    nlmsg_end(skb, nlh); 0
}

unsafe fn unix_diag_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> i32 {
    let _net = sock_net((*skb).sk);
    let req = nlmsg_data::<unix_diag_req>((*cb).nlh);
    let _s_slot = (*cb).args[0];
    let _s_num = (*cb).args[1];
    (*cb).args[0] = UNIX_HASH_SIZE as u32;
    (*cb).args[1] = 0;
    (*skb).len as i32
}

unsafe fn unix_lookup_by_ino(_net: *mut net, _ino: u32) -> *mut sock { core::ptr::null_mut() }

unsafe fn unix_diag_get_exact(in_skb: *mut sk_buff, nlh: *const nlmsghdr, req: *mut unix_diag_req) -> i32 {
    if (*req).udiag_ino == 0 { return -EINVAL; }
    let net = sock_net((*in_skb).sk);
    let sk = unix_lookup_by_ino(net, (*req).udiag_ino as u32);
    if sk.is_null() { return -ENOENT; }
    let mut extra_len = 256usize;
    loop {
        let rep = nlmsg_new(core::mem::size_of::<unix_diag_msg>() + extra_len, GFP_KERNEL);
        if rep.is_null() { sock_put(sk); return -ENOMEM; }
        let err = sk_diag_fill(sk, rep, req, sk_user_ns((*in_skb).sk), 0, (*nlh).nlmsg_seq, 0, (*req).udiag_ino);
        if err < 0 { nlmsg_free(rep); extra_len += 256; if extra_len >= PAGE_SIZE { sock_put(sk); return err; } continue; }
        sock_put(sk); return nlmsg_unicast((*net).diag_nlsk, rep, 0);
    }
}

extern "C" { fn sk_user_ns(sk: *mut sock) -> *mut user_namespace; }
unsafe fn unix_diag_handler_dump(skb: *mut sk_buff, h: *mut nlmsghdr) -> i32 {
    if nlmsg_len(h) < core::mem::size_of::<unix_diag_req>() { return -EINVAL; }
    unix_diag_get_exact(skb, h, nlmsg_data::<unix_diag_req>(h))
}

static mut UNIX_DIAG_HANDLER: sock_diag_handler = sock_diag_handler { owner: core::ptr::null_mut(), family: AF_UNIX as u16, dump: Some(unix_diag_handler_dump) };
pub unsafe fn unix_diag_init() -> i32 { sock_diag_register(&UNIX_DIAG_HANDLER) }
pub unsafe fn unix_diag_exit() { sock_diag_unregister(&UNIX_DIAG_HANDLER); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
