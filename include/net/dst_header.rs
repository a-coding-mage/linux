/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of net/dst.h; external kernel types and helpers are supplied elsewhere. */

use core::ffi::c_void;

#[repr(C)] pub struct sk_buff { pub _skb_refdst: usize, pub slow_gro: u8, pub dev: *mut net_device, pub mac_len: u16, pub len: u32, pub sk: *mut sock }
#[repr(C)] pub struct net_device;
#[repr(C)] pub struct dst_ops { pub mtu: Option<unsafe extern "C" fn(*const dst_entry) -> u32>, pub cow_metrics: Option<unsafe extern "C" fn(*mut dst_entry, usize) -> *mut u32>, pub default_advmss: Option<unsafe extern "C" fn(*const dst_entry) -> u32>, pub neigh_lookup: Option<unsafe extern "C" fn(*const dst_entry, *mut sk_buff, *const c_void) -> *mut neighbour>, pub confirm_neigh: Option<unsafe extern "C" fn(*const dst_entry, *const c_void)>, pub link_failure: Option<unsafe extern "C" fn(*mut sk_buff)>, pub update_pmtu: Option<unsafe extern "C" fn(*mut dst_entry, *mut c_void, *mut sk_buff, u32, bool)>, pub check: Option<unsafe extern "C" fn(*mut dst_entry, u32) -> *mut dst_entry>, pub output: Option<unsafe extern "C" fn(*mut net, *mut sock, *mut sk_buff)>, pub input: Option<unsafe extern "C" fn(*mut sk_buff)> }
#[repr(C)] pub struct net; #[repr(C)] pub struct sock; #[repr(C)] pub struct xfrm_state; #[repr(C)] pub struct lwtunnel_state; #[repr(C)] pub struct neighbour; #[repr(C)] pub struct flowi; #[repr(C)] pub struct uncached_list; #[repr(C)] pub struct list_head; #[repr(C)] pub struct rcu_head; #[repr(C)] pub struct refcount_t; #[repr(C)] pub struct rcuref_t; #[repr(C)] pub struct netdevice_tracker;

pub const DST_NOXFRM:u16=0x0002; pub const DST_NOPOLICY:u16=0x0004; pub const DST_NOCOUNT:u16=0x0008; pub const DST_FAKE_RTABLE:u16=0x0010; pub const DST_XFRM_TUNNEL:u16=0x0020; pub const DST_XFRM_QUEUE:u16=0x0040; pub const DST_METADATA:u16=0x0080;
pub const DST_OBSOLETE_NONE:i16=0; pub const DST_OBSOLETE_DEAD:i16=2; pub const DST_OBSOLETE_FORCE_CHK:i16=-1; pub const DST_OBSOLETE_KILL:i16=-2;

#[repr(C)] pub struct dst_entry {
    pub dev: *mut net_device, pub ops: *mut dst_ops, pub _metrics: usize, pub expires: usize,
    pub xfrm: *mut xfrm_state, pub input: Option<unsafe extern "C" fn(*mut sk_buff) -> i32>,
    pub output: Option<unsafe extern "C" fn(*mut net,*mut sock,*mut sk_buff)->i32>, pub flags:u16,
    pub obsolete:i16, pub header_len:u16, pub trailer_len:u16, pub __rcuref:rcuref_t, pub __use:i32,
    pub lastuse:usize, pub rcu_head:rcu_head, pub error:i16, pub __pad:i16, pub tclassid:u32,
    pub lwtstate:*mut lwtunnel_state, pub dev_tracker:netdevice_tracker, pub rt_uncached:list_head,
    pub rt_uncached_list:*mut uncached_list,
}
#[repr(C, align(4))] pub struct dst_metrics { pub metrics:[u32; 16], pub refcnt:refcount_t }
extern "C" { pub static dst_default_metrics: dst_metrics; pub fn dst_cow_metrics_generic(*mut dst_entry,usize)->*mut u32; pub fn __dst_destroy_metrics_generic(*mut dst_entry,usize); pub fn dst_release(*mut dst_entry); pub fn dst_release_immediate(*mut dst_entry); pub fn dst_discard_out(*mut net,*mut sock,*mut sk_buff)->i32; pub fn dst_alloc(*mut dst_ops,*mut net_device,i32,u16)->*mut c_void; pub fn dst_init(*mut dst_entry,*mut dst_ops,*mut net_device,i32,u16); pub fn dst_dev_put(*mut dst_entry); }
pub const DST_METRICS_READ_ONLY:usize=1; pub const DST_METRICS_REFCOUNTED:usize=2; pub const DST_METRICS_FLAGS:usize=3;
pub const DST_FEATURE_ECN_CA:u32=1u32<<31; pub const DST_FEATURE_MASK:u32=DST_FEATURE_ECN_CA;
pub const DST_FEATURE_ECN_MASK:u32=DST_FEATURE_ECN_CA | 0; // RTAX_FEATURE_ECN supplied externally

#[inline] pub unsafe fn dst_metrics_read_only(dst:*const dst_entry)->bool { (*dst)._metrics & DST_METRICS_READ_ONLY != 0 }
#[inline] pub unsafe fn dst_destroy_metrics_generic(dst:*mut dst_entry) { let v=(*dst)._metrics; if v&DST_METRICS_READ_ONLY==0 { __dst_destroy_metrics_generic(dst,v); } }
#[inline] pub unsafe fn dst_metrics_write_ptr(dst:*mut dst_entry)->*mut u32 { let p=(*dst)._metrics; assert!(p!=0); if p&DST_METRICS_READ_ONLY!=0 { ((*dst).ops).as_ref().unwrap().cow_metrics.unwrap()(dst,p) } else { (p&!DST_METRICS_FLAGS) as *mut u32 } }
#[inline] pub unsafe fn dst_init_metrics(dst:*mut dst_entry, src:*const u32, ro:bool) { (*dst)._metrics=src as usize | if ro {DST_METRICS_READ_ONLY} else {0}; }
#[inline] pub unsafe fn dst_metrics_ptr(dst:*mut dst_entry)->*mut u32 { ((*dst)._metrics&!DST_METRICS_FLAGS) as *mut u32 }
#[inline] pub unsafe fn dst_metric_raw(dst:*const dst_entry, metric:i32)->u32 { *dst_metrics_ptr(dst as *mut dst_entry).offset((metric-1) as isize) }
#[inline] pub unsafe fn dst_metric(dst:*const dst_entry, metric:i32)->u32 { dst_metric_raw(dst,metric) }
#[inline] pub unsafe fn dst_metric_advmss(dst:*const dst_entry)->u32 { let mut a=dst_metric_raw(dst,3); if a==0 { a=((*dst).ops).as_ref().unwrap().default_advmss.unwrap()(dst); } a }
#[inline] pub unsafe fn dst_metric_set(dst:*mut dst_entry, metric:i32, val:u32) { let p=dst_metrics_write_ptr(dst); if !p.is_null(){*p.offset((metric-1) as isize)=val;} }
#[inline] pub unsafe fn dst_feature(dst:*const dst_entry, feature:u32)->u32 { dst_metric(dst,1) & feature }
#[inline] pub unsafe fn dst_clone(dst:*mut dst_entry)->*mut dst_entry { dst }
#[inline] pub unsafe fn dst_hold(_dst:*mut dst_entry) {}
#[inline] pub unsafe fn dst_use_noref(dst:*mut dst_entry,time:usize) { if time != (*dst).lastuse { (*dst).__use += 1; (*dst).lastuse=time; } }
#[inline] pub unsafe fn refdst_drop(refdst:usize) { if refdst & 1 == 0 { dst_release((refdst & !1) as *mut dst_entry); } }
#[inline] pub unsafe fn skb_dst_drop(skb:*mut sk_buff) { if (*skb)._skb_refdst != 0 { refdst_drop((*skb)._skb_refdst); (*skb)._skb_refdst=0; } }
#[inline] pub unsafe fn __skb_dst_copy(nskb:*mut sk_buff,refdst:usize) { (*nskb).slow_gro |= (refdst!=0) as u8; (*nskb)._skb_refdst=refdst; }
#[inline] pub unsafe fn skb_dst_copy(nskb:*mut sk_buff,oskb:*const sk_buff) { __skb_dst_copy(nskb,(*oskb)._skb_refdst); }
#[inline] pub unsafe fn dst_hold_safe(_dst:*mut dst_entry)->bool { true }
#[inline] pub unsafe fn skb_dst_force(skb:*mut sk_buff)->bool { (*skb)._skb_refdst != 0 }
#[inline] pub unsafe fn dst_tclassid(_skb:*const sk_buff)->u32 { 0 }
#[inline] pub unsafe fn dst_discard(skb:*mut sk_buff)->i32 { dst_discard_out(core::ptr::null_mut(),(*skb).sk,skb) }
#[inline] pub unsafe fn dst_confirm(_dst:*mut dst_entry) {}
#[inline] pub unsafe fn dst_neigh_lookup(_dst:*const dst_entry,_daddr:*const c_void)->*mut neighbour { core::ptr::null_mut() }
#[inline] pub unsafe fn dst_neigh_lookup_skb(_dst:*const dst_entry,_skb:*mut sk_buff)->*mut neighbour { core::ptr::null_mut() }
#[inline] pub unsafe fn dst_confirm_neigh(_dst:*const dst_entry,_daddr:*const c_void) {}
#[inline] pub unsafe fn dst_link_failure(_skb:*mut sk_buff) {}
#[inline] pub unsafe fn dst_set_expires(dst:*mut dst_entry,timeout:i32) { (*dst).expires=(timeout as usize).max(1); }
#[inline] pub unsafe fn dst_dev_overhead(dst:*mut dst_entry,skb:*mut sk_buff)->u32 { if !dst.is_null(){0}else{(*skb).mac_len as u32} }
#[inline] pub unsafe fn dst_xfrm(_dst:*const dst_entry)->*mut xfrm_state { core::ptr::null_mut() }
#[inline] pub unsafe fn skb_dst_update_pmtu(_skb:*mut sk_buff,_mtu:u32) {}
#[inline] pub unsafe fn skb_dst_update_pmtu_no_confirm(_skb:*mut sk_buff,_mtu:u32) {}
#[inline] pub unsafe fn dst_dev(dst:*const dst_entry)->*mut net_device { (*dst).dev }
#[inline] pub unsafe fn dst_output(_net:*mut net,_sk:*mut sock,_skb:*mut sk_buff)->i32 { 0 }
#[inline] pub unsafe fn dst_input(_skb:*mut sk_buff)->i32 { 0 }
#[inline] pub unsafe fn dst_check(dst:*mut dst_entry,_cookie:u32)->*mut dst_entry { dst }
#[inline] pub unsafe fn skb_dst_dev(skb:*const sk_buff)->*mut net_device { (*skb).dev }

pub const XFRM_LOOKUP_ICMP:i32=1; pub const XFRM_LOOKUP_QUEUE:i32=2; pub const XFRM_LOOKUP_KEEP_DST_REF:i32=4;
extern "C" { pub fn dst_blackhole_check(*mut dst_entry,u32)->*mut dst_entry; pub fn dst_blackhole_update_pmtu(*mut dst_entry,*mut sock,*mut sk_buff,u32,bool); pub fn dst_blackhole_redirect(*mut dst_entry,*mut sock,*mut sk_buff); pub fn dst_blackhole_cow_metrics(*mut dst_entry,usize)->*mut u32; pub fn dst_blackhole_neigh_lookup(*const dst_entry,*mut sk_buff,*const c_void)->*mut neighbour; pub fn dst_blackhole_mtu(*const dst_entry)->u32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
