// SPDX-License-Identifier: GPL-2.0
/* XFRM virtual interface. Kernel symbols and types are supplied by other modules. */

use core::ffi::{c_char, c_int, c_uint, c_void};

const XFRMI_HASH_BITS: u32 = 8;
const XFRMI_HASH_SIZE: usize = 1usize << XFRMI_HASH_BITS;

#[repr(C)]
pub struct xfrmi_net {
    pub xfrmi: [*mut xfrm_if; XFRMI_HASH_SIZE],
    pub collect_md_xfrmi: *mut xfrm_if,
}

// External kernel representations (defined by the corresponding kernel headers).
#[repr(C)] pub struct net_device { pub flags: u32, pub ifindex: c_int, pub netdev_ops: *const net_device_ops, pub rtnl_link_ops: *const rtnl_link_ops, pub dev: *mut c_void }
#[repr(C)] pub struct net_device_ops { pub ndo_init: Option<unsafe extern "C" fn(*mut net_device)->c_int>, pub ndo_uninit: Option<unsafe extern "C" fn(*mut net_device)>, pub ndo_start_xmit: Option<unsafe extern "C" fn(*mut sk_buff,*mut net_device)->netdev_tx_t> }
#[repr(C)] pub struct xfrm_if { pub next: *mut xfrm_if, pub p: xfrm_if_parms, pub net: *mut net, pub dev: *mut net_device, pub gro_cells: gro_cells }
#[repr(C)] pub struct xfrm_if_parms { pub link: u32, pub if_id: u32, pub collect_md: bool }
#[repr(C)] pub struct net { _private: [u8;0] }
#[repr(C)] pub struct xfrm_state { pub if_id: u32, pub sel: xfrm_selector, pub inner_mode: xfrm_mode }
#[repr(C)] pub struct xfrm_selector { pub family: u16 }
#[repr(C)] pub struct xfrm_mode { pub family: u16 }
#[repr(C)] pub struct xfrm_if_decode_session_result { pub net: *mut net, pub if_id: u32 }
#[repr(C)] pub struct sk_buff { pub dev: *mut net_device, pub len: usize, pub protocol: u16, pub mark: u32 }
#[repr(C)] pub struct flowi { pub flowi_oif: u32, pub u: [u64; 8] }
#[repr(C)] pub struct dst_entry { pub dev: *mut net_device, pub xfrm: *mut xfrm_state, pub error: c_int }
#[repr(C)] pub struct gro_cells { _private: [u8;0] }
#[repr(C)] pub struct lwtunnel_state { pub r#type: u16 }
#[repr(C)] pub struct nlattr { _private: [u8;0] }
#[repr(C)] pub struct sk_buff_head { _private: [u8;0] }
#[repr(C)] pub struct list_head { _private: [u8;0] }
#[repr(C)] pub struct netlink_ext_ack { _private: [u8;0] }
#[repr(C)] pub struct rtnl_newlink_params { pub data: *mut *mut nlattr, pub link_net: *mut net }
#[repr(C)] pub struct inet6_skb_parm { _private: [u8;0] }
#[repr(C)] pub struct iphdr { pub protocol: u8, pub ihl: u8, pub daddr: u32, pub frag_off: u16 }
#[repr(C)] pub struct ipv6hdr { pub nexthdr: u8, pub saddr: [u8;16], pub daddr: [u8;16] }
pub type netdev_tx_t = c_int;
type __be32 = u32;

extern "C" {
    fn net_generic(net:*mut net, id:u32)->*mut xfrmi_net;
    fn hash_32(v:u32,bits:u32)->u32;
    fn xfrm_input_state(skb:*mut sk_buff)->*mut xfrm_state;
    fn xs_net(x:*mut xfrm_state)->*mut net;
    fn rcu_dereference<T>(p:*mut T)->*mut T;
    fn dev_get_by_index_rcu(net:*mut net, index:c_int)->*mut net_device;
    fn dev_net(dev:*mut net_device)->*mut net;
    fn xfrm_policy_check(net:*mut net, dir:c_int, skb:*mut sk_buff, family:u16)->bool;
    fn skb_sec_path(skb:*mut sk_buff)->*mut c_void;
    fn secpath_exists(skb:*mut sk_buff)->bool;
    fn xfrm_input(skb:*mut sk_buff,nexthdr:c_int,spi:__be32,encap:c_int)->c_int;
    fn kfree_skb(skb:*mut sk_buff);
    fn register_netdevice(dev:*mut net_device)->c_int;
    fn unregister_netdevice_queue(dev:*mut net_device,head:*mut list_head);
    fn gro_cells_destroy(cells:*mut gro_cells); fn gro_cells_init(cells:*mut gro_cells,dev:*mut net_device)->c_int;
    fn xfrm4_protocol_register(p:*mut c_void, proto:c_int)->c_int; fn xfrm4_protocol_deregister(p:*mut c_void,proto:c_int);
    fn xfrm6_protocol_register(p:*mut c_void, proto:c_int)->c_int; fn xfrm6_protocol_deregister(p:*mut c_void,proto:c_int);
    fn register_pernet_device(ops:*mut c_void)->c_int; fn unregister_pernet_device(ops:*mut c_void);
    fn rtnl_link_register(ops:*mut c_void)->c_int; fn rtnl_link_unregister(ops:*mut c_void);
    fn register_xfrm_interface_bpf()->c_int; fn xfrm_if_register_cb(cb:*const c_void); fn xfrm_if_unregister_cb();
    fn lwtunnel_encap_add_ops(ops:*const c_void, typ:c_int); fn lwtunnel_encap_del_ops(ops:*const c_void,typ:c_int);
}

unsafe fn xfrmi_hash(if_id:u32)->usize { hash_32(if_id,XFRMI_HASH_BITS) as usize }

unsafe fn xfrmi_lookup(net:*mut net, x:*mut xfrm_state)->*mut xfrm_if {
    let n=net_generic(net,0); let mut xi=(*n).xfrmi[xfrmi_hash((*x).if_id)];
    while !xi.is_null() { if (*x).if_id==(*xi).p.if_id && (*(*xi).dev).flags & 1 != 0 { return xi; } xi=(*xi).next; }
    xi=(*n).collect_md_xfrmi; if !xi.is_null() && (*(*xi).dev).flags&1!=0 {xi} else {core::ptr::null_mut()}
}

unsafe fn xfrmi_decode_session(skb:*mut sk_buff, family:u16, res:*mut xfrm_if_decode_session_result)->bool {
    if !secpath_exists(skb) || (*skb).dev.is_null() { return false; }
    let dev=(*skb).dev; if (*dev).flags&1==0 {return false;} let xi=dev as *mut xfrm_if;
    (*res).net=(*xi).net; (*res).if_id=(*xi).p.if_id; true
}

unsafe fn xfrmi_link(n:*mut xfrmi_net, xi:*mut xfrm_if) { (*xi).next=(*n).xfrmi[xfrmi_hash((*xi).p.if_id)]; (*n).xfrmi[xfrmi_hash((*xi).p.if_id)]=xi; }
unsafe fn xfrmi_unlink(n:*mut xfrmi_net, xi:*mut xfrm_if) { let slot=&mut (*n).xfrmi[xfrmi_hash((*xi).p.if_id)]; let mut p=*slot; if p==xi {*slot=(*xi).next;return} while !p.is_null(){if (*p).next==xi{(*p).next=(*xi).next;break}p=(*p).next;} }
unsafe fn xfrmi_dev_free(dev:*mut net_device) { gro_cells_destroy(&mut (*(dev as *mut xfrm_if)).gro_cells); }

unsafe fn xfrmi_create(net:*mut net,dev:*mut net_device)->c_int { let xi=dev as *mut xfrm_if; let n=net_generic(net,0); let e=register_netdevice(dev); if e<0{return e} if (*xi).p.collect_md{(*n).collect_md_xfrmi=xi}else{xfrmi_link(n,xi)} 0 }
unsafe fn xfrmi_dev_uninit(dev:*mut net_device) { let xi=dev as *mut xfrm_if; let n=net_generic((*xi).net,0); if (*xi).p.collect_md{(*n).collect_md_xfrmi=core::ptr::null_mut()}else{xfrmi_unlink(n,xi)} }

unsafe fn xfrmi_input(skb:*mut sk_buff,nexthdr:c_int,spi:__be32,encap:c_int,family:u16)->c_int { if !xfrm_policy_check(core::ptr::null_mut(),0,skb,family){kfree_skb(skb);return 0} xfrm_input(skb,nexthdr,spi,encap) }
unsafe fn xfrmi_scrub_packet(skb:*mut sk_buff,_xnet:bool) { (*skb).mark=0; }
unsafe fn xfrmi4_rcv(skb:*mut sk_buff)->c_int{xfrmi_input(skb,0,0,0,2)}
unsafe fn xfrmi6_rcv(skb:*mut sk_buff)->c_int{xfrmi_input(skb,0,0,0,10)}
unsafe fn xfrmi4_input(skb:*mut sk_buff,n:c_int,s:__be32,e:c_int)->c_int{xfrmi_input(skb,n,s,e,2)}
unsafe fn xfrmi6_input(skb:*mut sk_buff,n:c_int,s:__be32,e:c_int)->c_int{xfrmi_input(skb,n,s,e,10)}

unsafe fn xfrmi_rcv_cb(skb:*mut sk_buff,err:c_int)->c_int { if err!=0 && !secpath_exists(skb){return 0} let x=xfrm_input_state(skb); let xi=xfrmi_lookup(xs_net(x),x); if xi.is_null(){return 1} (*skb).dev=(*xi).dev; if err!=0{return 0} 0 }
unsafe fn xfrmi_xmit2(_skb:*mut sk_buff,_dev:*mut net_device,_fl:*mut flowi)->c_int { 0 }
unsafe fn xfrmi_xmit(skb:*mut sk_buff,dev:*mut net_device)->netdev_tx_t { let mut fl=flowi{flowi_oif:(*(dev as *mut xfrm_if)).p.link,u:[0;8]}; let r=xfrmi_xmit2(skb,dev,&mut fl); if r<0{kfree_skb(skb)} 0 }

unsafe fn xfrmi4_err(_skb:*mut sk_buff,_info:u32)->c_int{0}
unsafe fn xfrmi6_err(_skb:*mut sk_buff,_opt:*mut inet6_skb_parm,_typ:u8,_code:u8,_offset:c_int,_info:__be32)->c_int{0}
unsafe fn xfrmi_change(xi:*mut xfrm_if,p:*const xfrm_if_parms)->c_int { if (*xi).p.link!=(*p).link{return -22} (*xi).p.if_id=(*p).if_id;0 }
unsafe fn xfrmi_update(xi:*mut xfrm_if,p:*mut xfrm_if_parms)->c_int { let n=net_generic((*xi).net,0);xfrmi_unlink(n,xi);let e=xfrmi_change(xi,p);xfrmi_link(n,xi);e }
unsafe fn xfrmi_get_iflink(dev:*const net_device)->c_int{(*(dev as *mut xfrm_if)).p.link as c_int}

unsafe fn xfrmi_dev_setup(dev:*mut net_device) { (*dev).netdev_ops=core::ptr::null(); }
unsafe fn xfrmi_dev_init(dev:*mut net_device)->c_int { let xi=dev as *mut xfrm_if; let e=gro_cells_init(&mut (*xi).gro_cells,dev); if e<0{return e} 0 }
unsafe fn xfrmi_validate(_tb:*mut *mut nlattr,_data:*mut *mut nlattr,_extack:*mut netlink_ext_ack)->c_int{0}
unsafe fn xfrmi_netlink_parms(_data:*mut *mut nlattr,p:*mut xfrm_if_parms){*p=xfrm_if_parms{link:0,if_id:0,collect_md:false}}
unsafe fn xfrmi_newlink(dev:*mut net_device,params:*mut rtnl_newlink_params,_extack:*mut netlink_ext_ack)->c_int { let xi=dev as *mut xfrm_if; let net=if !(*params).link_net.is_null(){(*params).link_net}else{(*xi).net}; xfrmi_netlink_parms((*params).data,&mut (*xi).p);(*xi).net=net;(*xi).dev=dev;xfrmi_create(net,dev) }
unsafe fn xfrmi_dellink(dev:*mut net_device,head:*mut list_head){unregister_netdevice_queue(dev,head)}
unsafe fn xfrmi_changelink(dev:*mut net_device,_tb:*mut *mut nlattr,data:*mut *mut nlattr,_extack:*mut netlink_ext_ack)->c_int { let xi=dev as *mut xfrm_if; let mut p=xfrm_if_parms{link:0,if_id:0,collect_md:false};xfrmi_netlink_parms(data,&mut p);xfrmi_update(xi,&mut p) }
unsafe fn xfrmi_get_size(_dev:*const net_device)->usize{16}
unsafe fn xfrmi_fill_info(_skb:*mut sk_buff,_dev:*const net_device)->c_int{0}
unsafe fn xfrmi_get_link_net(dev:*const net_device)->*mut net{(*(dev as *mut xfrm_if)).net}

unsafe fn xfrmi_exit_rtnl(net:*mut net,head:*mut list_head){let n=net_generic(net,0);for i in 0..XFRMI_HASH_SIZE{let mut xi=(*n).xfrmi[i];while !xi.is_null(){unregister_netdevice_queue((*xi).dev,head);xi=(*xi).next}}if !(*n).collect_md_xfrmi.is_null(){unregister_netdevice_queue((*n).collect_md_xfrmi.as_ref().unwrap().dev,head)}}
unsafe fn xfrmi4_init()->c_int{0} unsafe fn xfrmi4_fini(){} unsafe fn xfrmi6_init()->c_int{0} unsafe fn xfrmi6_fini(){}
unsafe fn xfrmi_init()->c_int { let mut e=register_pernet_device(core::ptr::null_mut());if e<0{return e}e=xfrmi4_init();if e<0{unregister_pernet_device(core::ptr::null_mut());return e}e=xfrmi6_init();if e<0{xfrmi4_fini();return e}e=rtnl_link_register(core::ptr::null_mut());if e<0{xfrmi6_fini();xfrmi4_fini();return e}e=register_xfrm_interface_bpf();if e<0{rtnl_link_unregister(core::ptr::null_mut());return e}e }
unsafe fn xfrmi_fini(){xfrm_if_unregister_cb();lwtunnel_encap_del_ops(core::ptr::null(),0);rtnl_link_unregister(core::ptr::null_mut());xfrmi4_fini();xfrmi6_fini();unregister_pernet_device(core::ptr::null_mut())}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
