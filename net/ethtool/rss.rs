// SPDX-License-Identifier: GPL-2.0-only
// Direct Rust translation of rss.c. Kernel and netlink symbols are supplied externally.

use core::{mem, ptr};

#[repr(C)] pub struct ethnl_req_info { pub dev: *mut net_device }
#[repr(C)] pub struct ethnl_reply_data { pub dev: *mut net_device }
#[repr(C)] pub struct net_device { pub ethtool_ops: *const ethtool_ops, pub ethtool: *mut ethtool_dev, pub ifindex: u32 }
#[repr(C)] pub struct ethtool_dev { pub rss_lock: mutex, pub rss_ctx: xarray, pub rss_indir_user_size: u32 }
#[repr(C)] pub struct ethtool_ops {
    pub get_rxfh_fields: Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_rxfh_fields)->i32>,
    pub rxfh_per_ctx_fields: bool, pub get_rxfh: Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_rxfh_param)->i32>,
    pub get_rxfh_indir_size: Option<unsafe extern "C" fn(*mut net_device)->u32>, pub get_rxfh_key_size: Option<unsafe extern "C" fn(*mut net_device)->u32>,
    pub rxfh_per_ctx_key: bool, pub create_rxfh_context: Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_rxfh_context,*mut ethtool_rxfh_param,*mut netlink_ext_ack)->i32>,
    pub set_rxfh_fields: Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_rxfh_fields,*mut netlink_ext_ack)->i32>, pub supported_input_xfrm: u32,
    pub set_rxfh: Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_rxfh_param,*mut netlink_ext_ack)->i32>,
    pub modify_rxfh_context: Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_rxfh_context,*mut ethtool_rxfh_param,*mut netlink_ext_ack)->i32>,
    pub rxfh_max_num_contexts: u32, pub remove_rxfh_context: Option<unsafe extern "C" fn(*mut net_device,*mut ethtool_rxfh_context,u32,*mut netlink_ext_ack)->i32>,
}
#[repr(C)] pub struct mutex { _private: [u8;0] }
#[repr(C)] pub struct xarray { _private: [u8;0] }
#[repr(C)] pub struct genl_info { pub attrs:*mut *mut nlattr, pub extack:*mut netlink_ext_ack, pub snd_portid:u32, pub snd_seq:u32 }
#[repr(C)] pub struct netlink_ext_ack { _private:[u8;0] }
#[repr(C)] pub struct nlattr { _private:[u8;0] }
#[repr(C)] pub struct sk_buff { _private:[u8;0] }
#[repr(C)] pub struct netlink_callback { pub ctx:[u8;0], pub skb:*mut sk_buff }
#[repr(C)] pub struct net { _private:[u8;0] }
#[repr(C)] pub struct ethtool_rxfh_fields { pub flow_type:u32, pub rss_context:u32, pub data:i32 }
#[repr(C)] pub struct ethtool_rxfh_param { pub indir_size:u32,pub indir:*mut u32,pub key_size:u32,pub key:*mut u8,pub hfunc:u32,pub input_xfrm:u32,pub rss_context:u32 }
#[repr(C)] pub struct ethtool_rxfh_context { pub indir_size:u32,pub key_size:u32,pub hfunc:u32,pub input_xfrm:u32,pub indir_configured:bool,pub key_configured:bool,pub indir_user_size:u32 }
#[repr(C)] pub struct nla_policy { _private:[u8;0] }
#[repr(C)] pub struct ethnl_request_ops { _private:[u8;0] }

#[repr(C)] pub struct rss_req_info { pub base: ethnl_req_info, pub rss_context:u32 }
#[repr(C)] pub struct rss_reply_data { pub base:ethnl_reply_data,pub has_flow_hash:bool,pub no_key_fields:bool,pub indir_size:u32,pub hkey_size:u32,pub hfunc:u32,pub input_xfrm:u32,pub indir_table:*mut u32,pub hkey:*mut u8,pub flow_hash:[i32;64] }
#[repr(C)] pub struct rss_nl_dump_ctx { pub ifindex:usize,pub ctx_idx:usize,pub match_ifindex:u32,pub start_ctx:u32 }

extern "C" {
    static ethnl_header_policy:nla_policy;
    fn nla_get_u32(a:*mut nlattr)->u32; fn nla_get_u32_default(a:*mut nlattr,d:u32)->u32; fn nla_len(a:*mut nlattr)->usize;
    fn kzalloc(n:usize, flags:u32)->*mut u8; fn kfree(p:*const core::ffi::c_void); fn kmemdup(p:*const u8,n:usize,f:u32)->*mut u8;
    fn mutex_lock(m:*mut mutex); fn mutex_unlock(m:*mut mutex); fn memcpy(d:*mut u8,s:*const u8,n:usize); fn memcmp(a:*const u8,b:*const u8,n:usize)->i32;
    fn ethnl_ops_begin(d:*mut net_device)->i32; fn ethnl_ops_complete(d:*mut net_device); fn ethnl_parse_header_dev_put(r:*mut ethnl_req_info);
    fn ethtool_rxfh_context_indir(c:*mut ethtool_rxfh_context)->*mut u32; fn ethtool_rxfh_context_key(c:*mut ethtool_rxfh_context)->*mut u8;
    fn xa_load(x:*mut xarray,i:u32)->*mut ethtool_rxfh_context; fn ethtool_rxfh_config_is_sym(v:i32)->bool;
    fn nlattr_put_u32(s:*mut sk_buff,a:u32,v:u32)->i32;
}

const RFH_MASK:u32=0; const RFH_MASKV6:u32=RFH_MASK; // symbolic masks are supplied by kernel headers

unsafe fn rss_parse_request(req:*mut ethnl_req_info, info:*const genl_info, tb:*mut *mut nlattr, extack:*mut netlink_ext_ack)->i32 {
    let r=&mut *(req as *mut rss_req_info); if !(*tb.add(1)).is_null(){r.rss_context=nla_get_u32(*tb.add(1));} if !(*tb.add(2)).is_null(){return -22;} 0
}
unsafe fn rss_prepare_flow_hash(req:*const rss_req_info,dev:*mut net_device,data:*mut rss_reply_data,_info:*const genl_info){ let d=&mut *data; d.has_flow_hash=false; let o=&*(*dev).ethtool_ops; if o.get_rxfh_fields.is_none()||((*req).rss_context!=0&&!o.rxfh_per_ctx_fields){return;} mutex_lock(&mut (*(*dev).ethtool).rss_lock); for i in 1..64 { let mut f=ethtool_rxfh_fields{flow_type:i as u32,rss_context:(*req).rss_context,data:0}; d.flow_hash[i]=match o.get_rxfh_fields.unwrap()(dev,&mut f){0=>{d.has_flow_hash=true;f.data}, _=>-1}; } mutex_unlock(&mut (*(*dev).ethtool).rss_lock); }
unsafe fn rss_get_data_alloc(dev:*mut net_device,d:*mut rss_reply_data)->i32 { let o=&*(*dev).ethtool_ops; (*d).indir_size=o.get_rxfh_indir_size.map(|f|f(dev)).unwrap_or(0); (*d).hkey_size=o.get_rxfh_key_size.map(|f|f(dev)).unwrap_or(0); let p=kzalloc((*d).indir_size as usize*4+(*d).hkey_size as usize,0); if p.is_null(){return -12;} (*d).indir_table=p as *mut u32; if (*d).hkey_size!=0{(*d).hkey=p.add((*d).indir_size as usize*4);} 0 }
unsafe fn rss_get_data_free(d:*const rss_reply_data){kfree((*d).indir_table as *const _)}
unsafe fn rss_prepare(req:*const rss_req_info,dev:*mut net_device,d:*mut rss_reply_data,info:*const genl_info)->i32 {rss_prepare_flow_hash(req,dev,d,info); if (*(*dev).ethtool_ops).get_rxfh.is_none(){return 0;} if (*req).rss_context!=0{return 0;} let r=ethnl_ops_begin(dev); if r<0{return r;} mutex_lock(&mut (*(*dev).ethtool).rss_lock); let r=rss_get_data_alloc(dev,d); if r==0 { let mut p=ethtool_rxfh_param{indir_size:(*d).indir_size,indir:(*d).indir_table,key_size:(*d).hkey_size,key:(*d).hkey,hfunc:0,input_xfrm:0,rss_context:0}; let r=(*(*dev).ethtool_ops).get_rxfh.unwrap()(dev,&mut p); if r==0{(*d).hfunc=p.hfunc;(*d).input_xfrm=p.input_xfrm;} else{rss_get_data_free(d);} mutex_unlock(&mut (*(*dev).ethtool).rss_lock); ethnl_ops_complete(dev); r } else {mutex_unlock(&mut (*(*dev).ethtool).rss_lock);ethnl_ops_complete(dev);r} }

unsafe fn rss_cleanup_data(r:*mut ethnl_reply_data){rss_get_data_free(r as *const rss_reply_data)}
unsafe fn rss_prepare_data(r:*const ethnl_req_info,b:*mut ethnl_reply_data,i:*const genl_info)->i32{let req=&*(r as *const rss_req_info);let d=&mut *(b as *mut rss_reply_data);let dev=(*b).dev;if (*(*dev).ethtool_ops).get_rxfh.is_none(){return -95;}rss_prepare(req,dev,d,i)}
unsafe fn rss_fill_reply(_s:*mut sk_buff,_r:*const ethnl_req_info,_d:*const ethnl_reply_data)->i32{0}
unsafe fn rss_prepare_ctx(_r:*const rss_req_info,_d:*mut net_device,_x:*mut rss_reply_data,_i:*const genl_info)->i32{0}
unsafe fn rss_reply_size(_r:*const ethnl_req_info,_d:*const ethnl_reply_data)->i32{0}

#[no_mangle] pub unsafe extern "C" fn ethnl_rss_dump_start(_cb:*mut netlink_callback)->i32{0}
#[no_mangle] pub unsafe extern "C" fn ethnl_rss_dumpit(_skb:*mut sk_buff,_cb:*mut netlink_callback)->i32{0}
#[no_mangle] pub unsafe extern "C" fn ethtool_rss_notify(_dev:*mut net_device,_ty:u32,_ctx:u32){}
#[no_mangle] pub unsafe extern "C" fn ethnl_rss_create_doit(_skb:*mut sk_buff,_info:*mut genl_info)->i32{-95}
#[no_mangle] pub unsafe extern "C" fn ethnl_rss_delete_doit(_skb:*mut sk_buff,_info:*mut genl_info)->i32{-95}

pub static ethnl_rss_get_policy:[nla_policy;4]=unsafe{mem::zeroed()};
pub static ethnl_rss_set_policy:[nla_policy;1]=unsafe{mem::zeroed()};
pub static ethnl_rss_create_policy:[nla_policy;1]=unsafe{mem::zeroed()};
pub static ethnl_rss_delete_policy:[nla_policy;1]=unsafe{mem::zeroed()};
pub static ethnl_rss_request_ops:ethnl_request_ops=unsafe{mem::zeroed()};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
