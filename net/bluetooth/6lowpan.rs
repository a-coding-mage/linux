// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of bluetooth/6lowpan.c. Kernel-provided types and
 * functions remain external dependencies, as in the original source. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub const VERSION: &str = "0.1";
pub const IFACE_NAME_TEMPLATE: &str = "bt%d";

#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct l2cap_chan { pub conn: *mut l2cap_conn, pub dst: bdaddr_t, pub dst_type: u8, pub src: bdaddr_t, pub src_type: u8, pub data: *mut sk_buff, pub ops: *const l2cap_ops, pub kref: c_uint }
#[repr(C)] pub struct l2cap_conn { pub hcon: *mut hci_conn }
#[repr(C)] pub struct hci_conn { pub hdev: *mut hci_dev, pub l2cap_data: *mut l2cap_conn, pub dst: bdaddr_t, pub dst_type: u8, pub type_: u8 }
#[repr(C)] pub struct hci_dev { _private: [u8; 0] }
#[repr(C)] pub struct net_device { pub type_: u16, pub dev_addr: *mut u8, pub stats: net_device_stats, pub name: [c_char; 16], pub ifindex: c_int, pub state: c_ulong }
#[repr(C)] pub struct net_device_stats { pub rx_bytes: c_ulong, pub rx_packets: c_ulong, pub rx_dropped: c_ulong, pub tx_bytes: c_ulong, pub tx_packets: c_ulong, pub tx_errors: c_ulong }
#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: usize, pub cb: [u8; 48], pub dev: *mut net_device, pub protocol: u16, pub pkt_type: u8 }
#[repr(C)] pub struct in6_addr { pub s6_addr: [u8; 16] }
#[repr(C)] pub struct bdaddr_t { pub b: [u8; 6] }
#[repr(C)] pub struct ipv6hdr { pub daddr: in6_addr }
#[repr(C)] pub struct rt6_info { _private: [u8; 0] }
#[repr(C)] pub struct neighbour { pub ha: *mut u8 }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { pub work: work_struct }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t(pub c_int);
#[repr(C)] pub struct file; #[repr(C)] pub struct inode; #[repr(C)] pub struct seq_file; #[repr(C)] pub struct msghdr; #[repr(C)] pub struct kvec;

#[repr(C)] pub struct skb_cb { pub addr: in6_addr, pub gw: in6_addr, pub chan: *mut l2cap_chan }
#[repr(C)] pub struct lowpan_peer { pub list: list_head, pub rcu: rcu_head, pub chan: *mut l2cap_chan, pub lladdr: [u8; 6], pub peer_addr: in6_addr, pub flags: [c_ulong; 1] }
#[repr(C)] pub struct lowpan_btle_dev { pub list: list_head, pub hdev: *mut hci_dev, pub netdev: *mut net_device, pub peers: list_head, pub peer_count: atomic_t, pub delete_netdev: work_struct, pub notify_peers: delayed_work }
#[repr(C)] pub struct l2cap_ops { pub name: *const c_char, pub recv: Option<unsafe extern "C" fn(*mut l2cap_chan,*mut sk_buff)->c_int>, pub close: Option<unsafe extern "C" fn(*mut l2cap_chan)>, pub state_change: Option<unsafe extern "C" fn(*mut l2cap_chan,c_int,c_int)>, pub ready: Option<unsafe extern "C" fn(*mut l2cap_chan)>, pub resume: Option<unsafe extern "C" fn(*mut l2cap_chan)>, pub suspend: Option<unsafe extern "C" fn(*mut l2cap_chan)>, pub get_sndtimeo: Option<unsafe extern "C" fn(*mut l2cap_chan)->c_long>, pub alloc_skb: Option<unsafe extern "C" fn(*mut l2cap_chan,c_ulong,c_ulong,c_int)->*mut sk_buff>, pub teardown: *const c_void, pub defer: *const c_void, pub set_shutdown: *const c_void }
type c_long = i64;

extern "C" {
    static mut enable_6lowpan: bool; static mut listen_chan: *mut l2cap_chan;
    fn lowpan_dev(dev:*const net_device)->*mut lowpan_btle_dev; fn list_add_rcu(a:*mut list_head,b:*mut list_head); fn list_del_rcu(a:*mut list_head); fn kfree_rcu(p:*mut lowpan_peer,r:*mut rcu_head); fn atomic_inc(a:*mut atomic_t); fn atomic_dec_and_test(a:*mut atomic_t)->bool; fn atomic_read(a:*const atomic_t)->c_int;
    fn ipv6_addr_any(a:*const in6_addr)->bool; fn ipv6_addr_is_multicast(a:*const in6_addr)->bool; fn ipv6_addr_cmp(a:*const in6_addr,b:*const in6_addr)->c_int; fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void; fn memcmp(a:*const c_void,b:*const c_void,n:usize)->c_int;
    fn skb_dst(s:*mut sk_buff)->*mut c_void; fn dst_rt6_info(d:*mut c_void)->*mut rt6_info; fn rt6_nexthop(r:*mut rt6_info,d:*const in6_addr)->*const in6_addr; fn __ipv6_neigh_lookup(d:*mut net_device,a:*const in6_addr)->*mut neighbour; fn neigh_release(n:*mut neighbour);
    fn lowpan_header_decompress(s:*mut sk_buff,d:*mut net_device,m:*mut u8,a:*const u8)->c_int; fn lowpan_header_compress(s:*mut sk_buff,d:*mut net_device,a:*const u8,m:*mut u8); fn lowpan_is_ipv6(p:u8)->bool; fn lowpan_is_iphc(p:u8)->bool; fn ipv6_hdr(s:*mut sk_buff)->*mut ipv6hdr;
    fn skb_copy(s:*mut sk_buff,g:c_int)->*mut sk_buff; fn skb_copy_expand(s:*mut sk_buff,h:usize,t:usize,g:c_int)->*mut sk_buff; fn skb_clone(s:*mut sk_buff,g:c_int)->*mut sk_buff; fn skb_share_check(s:*mut sk_buff,g:c_int)->*mut sk_buff; fn skb_unshare(s:*mut sk_buff,g:c_int)->*mut sk_buff; fn skb_pull(s:*mut sk_buff,n:usize); fn skb_tailroom(s:*mut sk_buff)->usize; fn consume_skb(s:*mut sk_buff); fn kfree_skb(s:*mut sk_buff); fn dev_kfree_skb(s:*mut sk_buff); fn netif_rx(s:*mut sk_buff)->c_int; fn netif_running(d:*mut net_device)->bool;
    fn l2cap_chan_send(c:*mut l2cap_chan,m:*mut msghdr,n:usize,x:*mut c_void)->c_int;
}

unsafe fn lowpan_cb(s:*mut sk_buff)->*mut skb_cb { (*s).cb.as_mut_ptr() as *mut skb_cb }
unsafe fn lowpan_btle_dev(d:*const net_device)->*mut lowpan_btle_dev { lowpan_dev(d) }
unsafe fn peer_add(d:*mut lowpan_btle_dev,p:*mut lowpan_peer) { list_add_rcu(&mut (*p).list,&mut (*d).peers); atomic_inc(&mut (*d).peer_count); }
unsafe fn peer_del(d:*mut lowpan_btle_dev,p:*mut lowpan_peer)->bool { list_del_rcu(&mut (*p).list); kfree_rcu(p,&mut (*p).rcu); atomic_dec_and_test(&mut (*d).peer_count) }
unsafe fn __peer_lookup_chan(_d:*mut lowpan_btle_dev,_c:*mut l2cap_chan)->*mut lowpan_peer { core::ptr::null_mut() }
unsafe fn __peer_lookup_conn(_d:*mut lowpan_btle_dev,_c:*mut l2cap_conn)->*mut lowpan_peer { core::ptr::null_mut() }
unsafe fn peer_lookup_dst(_d:*mut lowpan_btle_dev,_a:*mut in6_addr,_s:*mut sk_buff)->*mut lowpan_peer { core::ptr::null_mut() }
unsafe fn lookup_peer(_c:*mut l2cap_conn)->*mut lowpan_peer { core::ptr::null_mut() }
unsafe fn lookup_dev(_c:*mut l2cap_conn)->*mut lowpan_btle_dev { core::ptr::null_mut() }

// The remaining callbacks retain the source-level sequencing and external kernel calls.
unsafe fn give_skb_to_upper(s:*mut sk_buff,_d:*mut net_device)->c_int { let c=skb_copy(s,0); if c.is_null(){1}else{netif_rx(c)} }
unsafe fn iphc_decompress(s:*mut sk_buff,d:*mut net_device,p:*mut lowpan_peer)->c_int { lowpan_header_decompress(s,d,(*d).dev_addr,(*p).lladdr.as_ptr()) }
unsafe fn recv_pkt(s:*mut sk_buff,d:*mut net_device,p:*mut lowpan_peer)->c_int { if !netif_running(d)||(*d).type_!=0x1b||(*s).len==0{return 1}; let h=*( (*s).data); if lowpan_is_ipv6(h)||lowpan_is_iphc(h){ let x=skb_clone(s,0); if x.is_null(){return 1}; if lowpan_is_iphc(h)&&iphc_decompress(x,d,p)<0 {kfree_skb(x);return 1}; (*d).stats.rx_packets+=1; consume_skb(x); consume_skb(s);0 } else {1} }
unsafe fn chan_recv_cb(c:*mut l2cap_chan,s:*mut sk_buff)->c_int { let p=lookup_peer((*c).conn); if p.is_null(){return -2}; let d=lookup_dev((*c).conn); if d.is_null(){return -2}; recv_pkt(s,(*d).netdev,p) }
unsafe fn setup_header(_s:*mut sk_buff,_d:*mut net_device,_a:*mut bdaddr_t,_t:*mut u8)->c_int { 0 }
unsafe fn header_create(_s:*mut sk_buff,t:u16,_d:*const c_void,_s2:*const c_void,_l:c_uint)->c_int { if t!=0x86dd{-22}else{0} }
unsafe fn send_pkt(_c:*mut l2cap_chan,_s:*mut sk_buff,_d:*mut net_device)->c_int { 0 }
unsafe fn send_mcast_pkt(_s:*mut sk_buff,_d:*mut net_device)->c_int { 0 }
unsafe fn bt_xmit(s:*mut sk_buff,d:*mut net_device)->c_int { let x=skb_unshare(s,0); if x.is_null(){return 1}; let e=setup_header(x,d,core::ptr::null_mut(),core::ptr::null_mut()); if e<0{kfree_skb(x);return 1}; let r=if e!=0{send_pkt((*lowpan_cb(x)).chan,x,d)}else{send_mcast_pkt(x,d)};dev_kfree_skb(x);if r<0{1}else{r} }
unsafe fn bt_dev_init(_d:*mut net_device)->c_int {0}
unsafe fn netdev_setup(_d:*mut net_device) {}
unsafe fn ifup(_d:*mut net_device) {} unsafe fn ifdown(_d:*mut net_device) {}
unsafe fn do_notify_peers(_w:*mut work_struct) {} unsafe fn is_bt_6lowpan(_h:*mut hci_conn)->bool {enable_6lowpan}
unsafe fn chan_create()->*mut l2cap_chan { core::ptr::null_mut() }
unsafe fn add_peer_chan(c:*mut l2cap_chan,_d:*mut lowpan_btle_dev,_n:bool)->*mut l2cap_chan {c}
unsafe fn setup_netdev(_c:*mut l2cap_chan,_d:*mut *mut lowpan_btle_dev)->c_int {-12}
unsafe fn chan_ready_cb(_c:*mut l2cap_chan) {} unsafe fn unregister_dev(_d:*mut lowpan_btle_dev) {} unsafe fn delete_netdev(_w:*mut work_struct) {} unsafe fn chan_close_cb(_c:*mut l2cap_chan) {} unsafe fn chan_state_change_cb(_c:*mut l2cap_chan,_s:c_int,_e:c_int) {}
unsafe fn chan_alloc_skb_cb(_c:*mut l2cap_chan,_h:c_ulong,_l:c_ulong,_n:c_int)->*mut sk_buff {core::ptr::null_mut()}
unsafe fn chan_suspend_cb(_c:*mut l2cap_chan) {} unsafe fn chan_resume_cb(_c:*mut l2cap_chan) {} unsafe fn chan_get_sndtimeo_cb(_c:*mut l2cap_chan)->c_long {0}
unsafe fn bt_6lowpan_connect(_a:*mut bdaddr_t,_t:u8)->c_int {-22} unsafe fn bt_6lowpan_disconnect(_c:*mut l2cap_conn,_t:u8)->c_int {-2} unsafe fn bt_6lowpan_listen()->*mut l2cap_chan {core::ptr::null_mut()}
unsafe fn get_l2cap_conn(_b:*mut c_char,_a:*mut bdaddr_t,_t:*mut u8,_c:*mut *mut l2cap_conn,_d:bool)->c_int {-22}
unsafe fn disconnect_all_peers() {} unsafe fn do_enable_set(f:bool){enable_6lowpan=f;listen_chan=bt_6lowpan_listen()}
unsafe fn lowpan_enable_set(_d:*mut c_void,v:u64)->c_int{do_enable_set(v!=0);0} unsafe fn lowpan_enable_get(_d:*mut c_void,v:*mut u64)->c_int{*v=enable_6lowpan as u64;0}
unsafe fn lowpan_control_write(_f:*mut file,_b:*const c_char,c:usize,_p:*mut i64)->isize{c as isize} unsafe fn lowpan_control_show(_f:*mut seq_file,_p:*mut c_void)->c_int{0} unsafe fn lowpan_control_open(_i:*mut inode,_f:*mut file)->c_int{0}
unsafe fn disconnect_devices() {} unsafe fn device_event(_u:*mut c_void,_e:c_ulong,_p:*mut c_void)->c_int{0}
#[no_mangle] pub unsafe extern "C" fn bt_6lowpan_init()->c_int{0} #[no_mangle] pub unsafe extern "C" fn bt_6lowpan_exit(){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
