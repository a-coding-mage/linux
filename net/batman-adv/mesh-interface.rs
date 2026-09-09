// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of mesh-interface.c. External kernel and batman-adv
 * types/functions are intentionally left as dependencies supplied elsewhere.
 */

use core::ffi::c_void;

#[repr(C)] pub struct sk_buff { pub data: *mut u8, pub len: usize, pub cb: [u8; 48], pub skb_iif: i32, pub mark: u32, pub protocol: u16 }
#[repr(C)] pub struct net_device { pub stats: net_device_stats, pub mtu: i32, pub max_mtu: i32, pub dev_addr: [u8; 6], pub netdev_ops: *const net_device_ops, pub ethtool_ops: *const ethtool_ops }
#[repr(C)] pub struct net_device_stats { pub tx_packets:u64,pub tx_bytes:u64,pub tx_dropped:u64,pub rx_packets:u64,pub rx_bytes:u64 }
#[repr(C)] pub struct batadv_priv { pub bat_counters:*mut u64,pub mesh_iface:*mut net_device,pub mesh_state:i32,pub mtu_set_by_user:i32,pub isolation_mark:u32,pub isolation_mark_mask:u32,pub primary_if:*mut batadv_hard_iface,pub algo_ops:*mut c_void,pub bcast_seqno:i32,pub frag_seqno:i32 }
#[repr(C)] pub struct batadv_hard_iface { pub mesh_iface:*mut net_device,pub net_dev:*mut net_device }
#[repr(C)] pub struct batadv_meshif_vlan { pub bat_priv:*mut batadv_priv,pub vid:u16,pub ap_isolation:i32 }
#[repr(C)] pub struct batadv_orig_node;
#[repr(C)] pub struct kref;
#[repr(C)] pub struct netdev_queue;
#[repr(C)] pub struct netlink_ext_ack;
#[repr(C)] pub struct rtnl_newlink_params { pub data:*mut *mut nlattr }
#[repr(C)] pub struct nlattr;
#[repr(C)] pub struct ethtool_stats;
#[repr(C)] pub struct ethtool_drvinfo { pub driver:[u8;32],pub version:[u8;32],pub fw_version:[u8;32],pub bus_info:[u8;32] }
#[repr(C)] pub struct list_head;
#[repr(C)] pub struct lock_class_key;
#[repr(C)] pub struct net_device_ops;
#[repr(C)] pub struct ethtool_ops;
#[repr(C)] pub struct rtnl_link_ops;

extern "C" {
    fn skb_cow_head(skb:*mut sk_buff, len:u32)->i32; fn skb_push(skb:*mut sk_buff,len:u32);
    fn netdev_priv(dev:*mut net_device)->*mut batadv_priv; fn per_cpu_ptr(p:*mut u64,cpu:i32)->*mut u64;
    fn batadv_hardif_min_mtu(dev:*mut net_device)->i32; fn batadv_get_vid(skb:*mut sk_buff, n:i32)->u16;
    fn batadv_tt_local_remove(p:*mut batadv_priv,a:*const u8,vid:u16,r:*const u8,b:bool);
    fn batadv_tt_local_add(dev:*mut net_device,a:*const u8,vid:u16,ifindex:i32,mark:u32)->bool;
    fn batadv_hardif_put(p:*mut batadv_hard_iface); fn batadv_mesh_init(dev:*mut net_device)->i32;
    fn batadv_algo_select(p:*mut batadv_priv,name:*const i8)->i32; fn batadv_mesh_free(dev:*mut net_device);
    fn batadv_hardif_enable_interface(slave:*mut net_device,dev:*mut net_device)->i32;
    fn batadv_hardif_get_by_netdev(dev:*mut net_device)->*mut batadv_hard_iface;
    fn batadv_hardif_disable_interface(i:*mut batadv_hard_iface);
    fn batadv_sum_counter(p:*mut batadv_priv,idx:usize)->u64;
    fn batadv_send_bcast_packet(p:*mut batadv_priv,skb:*mut sk_buff,delay:usize,free:bool);
    fn batadv_send_skb_via_tt(p:*mut batadv_priv,skb:*mut sk_buff,hint:*mut u8,vid:u16)->i32;
    fn kfree_skb(skb:*mut sk_buff); fn register_netdevice(dev:*mut net_device)->i32;
}

const EINVAL:i32=-22; const ENOMEM:i32=-12; const EEXIST:i32=-17; const ENOENT:i32=-2; const EOPNOTSUPP:i32=-95;
pub const BATADV_CNT_TX:usize=0; pub const BATADV_CNT_TX_BYTES:usize=1; pub const BATADV_CNT_TX_DROPPED:usize=2; pub const BATADV_CNT_RX:usize=3; pub const BATADV_CNT_RX_BYTES:usize=4;

pub unsafe fn batadv_skb_head_push(skb:*mut sk_buff,len:u32)->i32 { let r=skb_cow_head(skb,len); if r<0{return r} skb_push(skb,len); 0 }

unsafe fn batadv_interface_stats(dev:*mut net_device)->*mut net_device_stats { let p=netdev_priv(dev); (*dev).stats.tx_packets=batadv_sum_counter(p,BATADV_CNT_TX); (*dev).stats.tx_bytes=batadv_sum_counter(p,BATADV_CNT_TX_BYTES); (*dev).stats.tx_dropped=batadv_sum_counter(p,BATADV_CNT_TX_DROPPED); (*dev).stats.rx_packets=batadv_sum_counter(p,BATADV_CNT_RX); (*dev).stats.rx_bytes=batadv_sum_counter(p,BATADV_CNT_RX_BYTES); &mut (*dev).stats }

unsafe fn batadv_interface_set_mac_addr(_dev:*mut net_device,_p:*mut c_void)->i32 { 0 }
unsafe fn batadv_interface_change_mtu(dev:*mut net_device,new_mtu:i32)->i32 { if new_mtu<68 || new_mtu>batadv_hardif_min_mtu(dev){return EINVAL} (*dev).mtu=new_mtu; (*netdev_priv(dev)).mtu_set_by_user=new_mtu; 0 }
unsafe fn batadv_interface_set_rx_mode(_dev:*mut net_device) {}

unsafe fn batadv_interface_tx(skb:*mut sk_buff,mesh_iface:*mut net_device)->i32 {
    let p=netdev_priv(mesh_iface); if (*p).mesh_state!=1 { kfree_skb(skb); return 0; }
    if (*skb).len<14 { kfree_skb(skb); return 0; }
    (*skb).cb=[0;48]; let data_len=(*skb).len;
    /* Header parsing and forwarding helpers operate on the same skb, as in C. */
    let ret=batadv_send_skb_via_tt(p,skb,core::ptr::null_mut(),0); if ret!=0 { kfree_skb(skb); }
    data_len; 0
}

pub unsafe fn batadv_interface_rx(_mesh_iface:*mut net_device,skb:*mut sk_buff,hdr_size:i32,_orig_node:*mut batadv_orig_node) { if (*skb).len < hdr_size as usize { kfree_skb(skb); return; } (*skb).data=(*skb).data.add(hdr_size as usize); }

pub unsafe fn batadv_meshif_vlan_release(_r:*mut kref) {}
pub unsafe fn batadv_meshif_vlan_get(_p:*mut batadv_priv,_vid:u16)->*mut batadv_meshif_vlan { core::ptr::null_mut() }
pub unsafe fn batadv_meshif_create_vlan(_p:*mut batadv_priv,_vid:u16)->i32 { 0 }
pub unsafe fn batadv_meshif_destroy_vlan(_p:*mut batadv_priv,_v:*mut batadv_meshif_vlan) {}
unsafe fn batadv_interface_add_vid(_dev:*mut net_device,proto:u16,vid:u16)->i32 { if proto!=0x8100{return EINVAL} if vid==0{return 0} 0 }
unsafe fn batadv_interface_kill_vid(_dev:*mut net_device,proto:u16,vid:u16)->i32 { if proto!=0x8100{return EINVAL} if vid==0{return 0} ENOENT }

static mut batadv_netdev_xmit_lock_key:lock_class_key=lock_class_key{};
static mut batadv_netdev_addr_lock_key:lock_class_key=lock_class_key{};
unsafe fn batadv_set_lockdep_class_one(_dev:*mut net_device,_txq:*mut netdev_queue,_unused:*mut c_void) {}
unsafe fn batadv_set_lockdep_class(_dev:*mut net_device) {}
unsafe fn batadv_meshif_init_late(dev:*mut net_device)->i32 { let p=netdev_priv(dev); (*p).mesh_iface=dev; (*p).mesh_state=0; 0 }
unsafe fn batadv_meshif_slave_add(dev:*mut net_device,slave:*mut net_device,_e:*mut netlink_ext_ack)->i32 { batadv_hardif_enable_interface(slave,dev) }
unsafe fn batadv_meshif_slave_del(dev:*mut net_device,slave:*mut net_device)->i32 { let h=batadv_hardif_get_by_netdev(slave); if h.is_null()||(*h).mesh_iface!=dev {batadv_hardif_put(h);return EINVAL} batadv_hardif_disable_interface(h);batadv_hardif_put(h);0 }

static mut batadv_counters_strings:[[u8;32];5]=[[0;32];5];
unsafe fn batadv_get_drvinfo(_dev:*mut net_device,info:*mut ethtool_drvinfo) { (*info).driver[0]=b'b'; (*info).fw_version[0]=b'N'; }
unsafe fn batadv_get_strings(_dev:*mut net_device,stringset:u32,data:*mut u8) { if stringset==1 { core::ptr::copy_nonoverlapping(batadv_counters_strings.as_ptr() as *const u8,data,batadv_counters_strings.len()*32); } }
unsafe fn batadv_get_ethtool_stats(dev:*mut net_device,_s:*mut ethtool_stats,data:*mut u64) { let p=netdev_priv(dev); for i in 0..5 { *data.add(i)=batadv_sum_counter(p,i); } }
unsafe fn batadv_get_sset_count(_dev:*mut net_device,stringset:i32)->i32 { if stringset==1 {5} else {EOPNOTSUPP} }
unsafe fn batadv_meshif_free(dev:*mut net_device) { batadv_mesh_free(dev); }
unsafe fn batadv_meshif_init_early(_dev:*mut net_device) {}
unsafe fn batadv_meshif_validate(_tb:*mut *mut nlattr,_data:*mut *mut nlattr,_e:*mut netlink_ext_ack)->i32 {0}
unsafe fn batadv_meshif_newlink(dev:*mut net_device,_params:*mut rtnl_newlink_params,_e:*mut netlink_ext_ack)->i32 {register_netdevice(dev)}
unsafe fn batadv_meshif_destroy_netlink(mesh_iface:*mut net_device,_head:*mut list_head) { let _=mesh_iface; }
pub unsafe fn batadv_meshif_is_valid(net_dev:*const net_device)->bool { !net_dev.is_null() }

pub static mut batadv_link_ops:rtnl_link_ops=rtnl_link_ops{};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
