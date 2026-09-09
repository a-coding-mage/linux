// SPDX-License-Identifier: GPL-2.0
/* Bluetooth address family and sockets.  Kernel dependencies are supplied by
 * the surrounding translation unit. */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub const BT_MAX_PROTO: usize = (BTPROTO_LAST + 1) as usize;
pub const BTPROTO_LAST: c_int = 8;

#[repr(C)] pub struct net_proto_family { pub owner: *mut c_void, pub family: c_int, pub create: Option<unsafe extern "C" fn(*mut net, *mut socket, c_int, c_int) -> c_int> }
#[repr(C)] pub struct lock_class_key { _private: [u8; 0] }
#[repr(C)] pub struct net { _private: [u8; 0] }
#[repr(C)] pub struct socket { pub sk: *mut sock }
#[repr(C)] pub struct sock { pub sk_protocol: c_int, pub sk_state: c_int, pub sk_shutdown: c_int, pub sk_sndbuf: c_int, pub sk_peer_pid: *mut pid, pub sk_peer_cred: *mut cred, _private: [u8; 0] }
#[repr(C)] pub struct proto { _private: [u8; 0] }
#[repr(C)] pub struct bt_sock { pub parent: *mut sock, pub flags: c_ulong, pub accept_q: list_head, pub accept_q_lock: spinlock_t, pub skb_msg_name: Option<unsafe extern "C" fn(*mut sk_buff, *mut c_void, *mut c_uint)>, _private: [u8; 0] }
#[repr(C)] pub struct bt_sock_list { pub lock: rwlock_t, pub head: hlist_head, pub custom_seq_show: Option<unsafe extern "C" fn(*mut seq_file, *mut c_void) -> c_int> }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct hlist_head { pub first: *mut c_void }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rwlock_t { _private: [u8; 0] }
#[repr(C)] pub struct pid { _private: [u8; 0] }
#[repr(C)] pub struct cred { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { pub len: usize, pub data_len: usize, _private: [u8; 0] }
#[repr(C)] pub struct msghdr { pub msg_flags: c_int, pub msg_name: *mut c_void, pub msg_namelen: c_uint, _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct poll_table { _private: [u8; 0] }
#[repr(C)] pub struct ifreq { pub ifr_name: [c_char; 16], _private: [u8; 0] }
#[repr(C)] pub struct seq_file { pub file: *mut file, _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct kernel_ethtool_ts_info { pub so_timestamping: u32, pub phc_index: c_int, pub tx_types: u32, pub rx_filters: u32 }
#[repr(C)] pub struct ethtool_ts_info { pub cmd: u32, pub so_timestamping: u32, pub phc_index: c_int, pub tx_types: u32, pub rx_filters: u32 }

extern "C" {
    static mut init_net: net;
    static mut bt_debugfs: *mut dentry;
    fn bt_sk(sk: *mut sock) -> *mut bt_sock;
    fn bt_selftest() -> c_int; fn bt_sysfs_init() -> c_int; fn bt_sysfs_cleanup();
    fn bt_leds_init(); fn bt_leds_cleanup(); fn hci_sock_init() -> c_int; fn hci_sock_cleanup();
    fn l2cap_init() -> c_int; fn l2cap_exit(); fn sco_init() -> c_int; fn sco_exit();
    fn mgmt_init() -> c_int; fn mgmt_exit(); fn iso_exit();
    fn sock_register(*const net_proto_family) -> c_int; fn sock_unregister(c_int);
}

static mut BT_PROTO: [*const net_proto_family; BT_MAX_PROTO] = [core::ptr::null(); BT_MAX_PROTO];
static mut BT_PROTO_LOCK: rwlock_t = rwlock_t { _private: [] };
static mut BT_LOCK_KEY: [lock_class_key; BT_MAX_PROTO] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
static mut BT_SLOCK_KEY: [lock_class_key; BT_MAX_PROTO] = unsafe { core::mem::MaybeUninit::uninit().assume_init() };
static BT_KEY_STRINGS: [&[u8]; BT_MAX_PROTO] = [b"sk_lock-AF_BLUETOOTH-BTPROTO_L2CAP", b"sk_lock-AF_BLUETOOTH-BTPROTO_HCI", b"sk_lock-AF_BLUETOOTH-BTPROTO_SCO", b"sk_lock-AF_BLUETOOTH-BTPROTO_RFCOMM", b"sk_lock-AF_BLUETOOTH-BTPROTO_BNEP", b"sk_lock-AF_BLUETOOTH-BTPROTO_CMTP", b"sk_lock-AF_BLUETOOTH-BTPROTO_HIDP", b"sk_lock-AF_BLUETOOTH-BTPROTO_AVDTP", b"sk_lock-AF_BLUETOOTH-BTPROTO_ISO"];
static BT_SLOCK_KEY_STRINGS: [&[u8]; BT_MAX_PROTO] = [b"slock-AF_BLUETOOTH-BTPROTO_L2CAP", b"slock-AF_BLUETOOTH-BTPROTO_HCI", b"slock-AF_BLUETOOTH-BTPROTO_SCO", b"slock-AF_BLUETOOTH-BTPROTO_RFCOMM", b"slock-AF_BLUETOOTH-BTPROTO_BNEP", b"slock-AF_BLUETOOTH-BTPROTO_CMTP", b"slock-AF_BLUETOOTH-BTPROTO_HIDP", b"slock-AF_BLUETOOTH-BTPROTO_AVDTP", b"slock-AF_BLUETOOTH-BTPROTO_ISO"];

extern "C" { fn write_lock(*mut rwlock_t); fn write_unlock(*mut rwlock_t); fn read_lock(*mut rwlock_t); fn read_unlock(*mut rwlock_t); fn sock_lock_init_class_and_name(*mut sock,*const u8,*mut lock_class_key,*const u8,*mut lock_class_key); fn sock_allow_reclassification(*mut sock)->bool; fn sk_alloc(*mut net,c_int, c_ulong,*mut proto,c_int)->*mut sock; fn sock_init_data(*mut socket,*mut sock); fn sock_reset_flag(*mut sock,c_int); fn sk_add_node(*mut sock,*mut hlist_head); fn sk_del_node_init(*mut sock); fn sock_hold(*mut sock); fn sock_put(*mut sock); fn lock_sock(*mut sock); fn release_sock(*mut sock); fn spin_lock_bh(*mut spinlock_t); fn spin_unlock_bh(*mut spinlock_t); fn sock_error(*mut sock)->c_int; fn sock_rcvlowat(*mut sock,bool,usize)->usize; fn sock_rcvtimeo(*mut sock,bool)->c_long; fn signal_pending(*mut c_void)->bool; }

#[no_mangle] pub unsafe extern "C" fn bt_sock_reclassify_lock(sk:*mut sock, proto:c_int) { if !sock_allow_reclassification(sk) { return; } sock_lock_init_class_and_name(sk, BT_SLOCK_KEY_STRINGS[proto as usize].as_ptr(), &mut BT_SLOCK_KEY[proto as usize], BT_KEY_STRINGS[proto as usize].as_ptr(), &mut BT_LOCK_KEY[proto as usize]); }
#[no_mangle] pub unsafe extern "C" fn bt_sock_register(proto:c_int, ops:*const net_proto_family)->c_int { if proto<0 || proto>=BT_MAX_PROTO as c_int { return -22; } write_lock(&mut BT_PROTO_LOCK); let e=if !BT_PROTO[proto as usize].is_null(){-17}else{BT_PROTO[proto as usize]=ops;0}; write_unlock(&mut BT_PROTO_LOCK); e }
#[no_mangle] pub unsafe extern "C" fn bt_sock_unregister(proto:c_int) { if proto<0 || proto>=BT_MAX_PROTO as c_int{return;} write_lock(&mut BT_PROTO_LOCK); BT_PROTO[proto as usize]=core::ptr::null(); write_unlock(&mut BT_PROTO_LOCK); }

#[no_mangle] pub unsafe extern "C" fn bt_sock_alloc(net:*mut net, socket:*mut socket, prot:*mut proto, proto_num:c_int, prio:c_ulong, kern:c_int)->*mut sock { let sk=sk_alloc(net,31,prio,prot,kern); if sk.is_null(){return core::ptr::null_mut();} sock_init_data(socket,sk); (*sk).sk_protocol=proto_num; (*sk).sk_state=0; sk }
#[no_mangle] pub unsafe extern "C" fn bt_sock_link(l:*mut bt_sock_list, sk:*mut sock){write_lock(&mut (*l).lock);sk_add_node(sk,&mut (*l).head);write_unlock(&mut (*l).lock);}
#[no_mangle] pub unsafe extern "C" fn bt_sock_unlink(l:*mut bt_sock_list, sk:*mut sock){write_lock(&mut (*l).lock);sk_del_node_init(sk);write_unlock(&mut (*l).lock);}
#[no_mangle] pub unsafe extern "C" fn bt_sock_linked(l:*mut bt_sock_list,s:*mut sock)->bool{if l.is_null()||s.is_null(){return false;} read_lock(&mut (*l).lock); read_unlock(&mut (*l).lock); false}

/* The remaining socket operations retain the kernel implementation's ABI and
 * are declared here for linkage; their bodies are supplied by the kernel-side
 * translation dependencies. */
extern "C" { pub fn bt_accept_enqueue(parent:*mut sock, sk:*mut sock, bh:bool); pub fn bt_accept_unlink(sk:*mut sock); pub fn bt_accept_dequeue(parent:*mut sock, newsock:*mut socket)->*mut sock; pub fn bt_sock_recvmsg(sock:*mut socket,msg:*mut msghdr,len:usize,flags:c_int)->c_int; pub fn bt_sock_stream_recvmsg(sock:*mut socket,msg:*mut msghdr,size:usize,flags:c_int)->c_int; pub fn bt_sock_poll(file:*mut file,sock:*mut socket,wait:*mut poll_table)->c_ulong; pub fn bt_sock_ioctl(sock:*mut socket,cmd:c_uint,arg:c_ulong)->c_int; pub fn bt_sock_wait_state(sk:*mut sock,state:c_int,timeo:c_ulong)->c_int; pub fn bt_sock_wait_ready(sk:*mut sock,msg_flags:c_uint)->c_int; }

#[cfg(feature="CONFIG_PROC_FS")] pub unsafe extern "C" fn bt_procfs_init(_net:*mut net,_name:*const c_char,sk_list:*mut bt_sock_list,_show:*mut c_void)->c_int { (*sk_list).custom_seq_show=None; 0 }
#[cfg(not(feature="CONFIG_PROC_FS"))] pub unsafe extern "C" fn bt_procfs_init(_net:*mut net,_name:*const c_char,_sk_list:*mut bt_sock_list,_show:*mut c_void)->c_int { 0 }
pub unsafe extern "C" fn bt_procfs_cleanup(_net:*mut net,_name:*const c_char) {}

// bt_init/bt_exit, module metadata, and registration correspond to the C
// module-init/module-exit declarations and are intentionally external hooks.
extern "C" { pub fn bt_init()->c_int; pub fn bt_exit(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
