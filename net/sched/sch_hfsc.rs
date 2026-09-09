/* Faithful low-level Rust translation of sch_hfsc.c.  Kernel dependencies are external. */
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::ffi::c_void;

type u8 = core::primitive::u8; type u16 = core::primitive::u16; type u32 = core::primitive::u32; type u64 = core::primitive::u64;
type i32 = core::primitive::i32;

#[repr(C)] pub struct Qdisc_class_common { pub classid:u32, pub hnode:[u8;0] }
#[repr(C)] pub struct gnet_stats_basic_sync { _p:[u8;0] }
#[repr(C)] pub struct gnet_stats_queue { pub backlog:u32, pub drops:u32 }
#[repr(C)] pub struct net_rate_estimator { _p:[u8;0] }
#[repr(C)] pub struct tcf_proto { _p:[u8;0] } #[repr(C)] pub struct tcf_block { _p:[u8;0] }
#[repr(C)] pub struct Qdisc { pub q: qdisc_queue, pub handle:u32, pub dev_queue:*mut c_void, pub ops:*mut Qdisc_ops }
#[repr(C)] pub struct qdisc_queue { pub qlen:u32 }
#[repr(C)] pub struct rb_node { pub rb_left:*mut rb_node, pub rb_right:*mut rb_node, pub rb_parent:*mut rb_node }
#[repr(C)] pub struct rb_root { pub rb_node:*mut rb_node }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct qdisc_watchdog { _p:[u8;0] } #[repr(C)] pub struct Qdisc_class_hash { pub hashsize:u32, pub hash:*mut c_void }
#[repr(C)] pub struct sk_buff { pub priority:u32 } #[repr(C)] pub struct nlattr { _p:[u8;0] }
#[repr(C)] pub struct netlink_ext_ack { _p:[u8;0] } #[repr(C)] pub struct tcmsg { pub tcm_parent:u32,pub tcm_handle:u32,pub tcm_info:u32 }
#[repr(C)] pub struct gnet_dump { _p:[u8;0] } #[repr(C)] pub struct qdisc_walker { pub stop:bool }
#[repr(C)] pub struct tc_service_curve { pub m1:u32,pub d:u32,pub m2:u32 }

#[repr(C)] pub struct internal_sc { pub sm1:u64,pub ism1:u64,pub dx:u64,pub dy:u64,pub sm2:u64,pub ism2:u64 }
#[repr(C)] pub struct runtime_sc { pub x:u64,pub y:u64,pub sm1:u64,pub ism1:u64,pub dx:u64,pub dy:u64,pub sm2:u64,pub ism2:u64 }
#[repr(C)] pub struct hfsc_class {
 pub cl_common:Qdisc_class_common,pub bstats:gnet_stats_basic_sync,pub qstats:gnet_stats_queue,pub rate_est:*mut net_rate_estimator,pub filter_list:*mut tcf_proto,pub block:*mut tcf_block,pub level:u32,pub sched:*mut hfsc_sched,pub cl_parent:*mut hfsc_class,pub siblings:list_head,pub children:list_head,pub qdisc:*mut Qdisc,pub el_node:rb_node,pub vt_tree:rb_root,pub vt_node:rb_node,pub cf_tree:rb_root,pub cf_node:rb_node,
 pub cl_total:u64,pub cl_cumul:u64,pub cl_d:u64,pub cl_e:u64,pub cl_vt:u64,pub cl_f:u64,pub cl_myf:u64,pub cl_cfmin:u64,pub cl_cvtmin:u64,pub cl_vtadj:u64,pub cl_cvtoff:u64,pub cl_rsc:internal_sc,pub cl_fsc:internal_sc,pub cl_usc:internal_sc,pub cl_deadline:runtime_sc,pub cl_eligible:runtime_sc,pub cl_virtual:runtime_sc,pub cl_ulimit:runtime_sc,pub cl_flags:u8,pub cl_vtperiod:u32,pub cl_parentperiod:u32,pub cl_nactive:u32
}
#[repr(C)] pub struct hfsc_sched { pub defcls:u16,pub root:hfsc_class,pub clhash:Qdisc_class_hash,pub eligible:rb_root,pub watchdog:qdisc_watchdog }
pub const HT_INFINITY:u64=0xffffffffffffffff; pub const HFSC_RSC:u8=1; pub const HFSC_FSC:u8=2; pub const HFSC_USC:u8=4;
pub const PSCHED_SHIFT:u32=10; pub const SM_SHIFT:u32=30-PSCHED_SHIFT; pub const ISM_SHIFT:u32=8+PSCHED_SHIFT;
pub const SM_MASK:u64=(1u64<<SM_SHIFT)-1; pub const ISM_MASK:u64=(1u64<<ISM_SHIFT)-1;

extern "C" { fn psched_get_time()->u64; fn div64_u64(a:u64,b:u64)->u64; static PSCHED_TICKS_PER_SEC:u64; static USEC_PER_SEC:u64; }
#[inline] unsafe fn seg_x2y(x:u64,sm:u64)->u64 {(x>>SM_SHIFT)*sm+(((x&SM_MASK)*sm)>>SM_SHIFT)}
#[inline] unsafe fn seg_y2x(y:u64,ism:u64)->u64 { if y==0 {0} else if ism==HT_INFINITY {HT_INFINITY} else {(y>>ISM_SHIFT)*ism+(((y&ISM_MASK)*ism)>>ISM_SHIFT)} }
unsafe fn m2sm(m:u32)->u64 { let mut x=(m as u64)<<SM_SHIFT; x+=PSCHED_TICKS_PER_SEC-1; x/PSCHED_TICKS_PER_SEC }
unsafe fn m2ism(m:u32)->u64 { if m==0 {HT_INFINITY} else {((PSCHED_TICKS_PER_SEC<<ISM_SHIFT)+(m as u64)-1)/(m as u64)} }
unsafe fn d2dx(d:u32)->u64 { ((d as u64)*PSCHED_TICKS_PER_SEC+USEC_PER_SEC-1)/USEC_PER_SEC }
unsafe fn sm2m(sm:u64)->u32 {(sm*PSCHED_TICKS_PER_SEC>>SM_SHIFT) as u32}
unsafe fn dx2d(dx:u64)->u32 {(dx*USEC_PER_SEC/PSCHED_TICKS_PER_SEC) as u32}
unsafe fn sc2isc(sc:*const tc_service_curve,isc:*mut internal_sc){(*isc).sm1=m2sm((*sc).m1);(*isc).ism1=m2ism((*sc).m1);(*isc).dx=d2dx((*sc).d);(*isc).dy=seg_x2y((*isc).dx,(*isc).sm1);(*isc).sm2=m2sm((*sc).m2);(*isc).ism2=m2ism((*sc).m2)}
unsafe fn rtsc_init(r:*mut runtime_sc,i:*const internal_sc,x:u64,y:u64){(*r).x=x;(*r).y=y;(*r).sm1=(*i).sm1;(*r).ism1=(*i).ism1;(*r).dx=(*i).dx;(*r).dy=(*i).dy;(*r).sm2=(*i).sm2;(*r).ism2=(*i).ism2}
unsafe fn rtsc_y2x(r:*const runtime_sc,y:u64)->u64 { if y<(*r).y {(*r).x} else if y<=(*r).y+(*r).dy {(*r).x+if (*r).dy==0 {(*r).dx}else{seg_y2x(y-(*r).y,(*r).ism1)}} else {(*r).x+(*r).dx+seg_y2x(y-(*r).y-(*r).dy,(*r).ism2)} }
unsafe fn rtsc_x2y(r:*const runtime_sc,x:u64)->u64 { if x<=(*r).x {(*r).y} else if x<=(*r).x+(*r).dx {(*r).y+seg_x2y(x-(*r).x,(*r).sm1)} else {(*r).y+(*r).dy+seg_x2y(x-(*r).x-(*r).dx,(*r).sm2)} }
unsafe fn rtsc_min(r:*mut runtime_sc,i:*const internal_sc,x:u64,y:u64){ if (*i).sm1<=(*i).sm2 {if rtsc_x2y(r,x)<y{return} (*r).x=x;(*r).y=y;return} let y1=rtsc_x2y(r,x);if y1<=y{return} let y2=rtsc_x2y(r,x+(*i).dx);if y2>=y+(*i).dy {(*r).x=x;(*r).y=y;(*r).dx=(*i).dx;(*r).dy=(*i).dy;return} let mut dx=div64_u64((y1-y)<<SM_SHIFT,(*i).sm1-(*i).sm2);if (*r).x+(*r).dx>x {dx+=(*r).x+(*r).dx-x}(*r).x=x;(*r).y=y;(*r).dx=dx;(*r).dy=seg_x2y(dx,(*i).sm1)}

/* Tree operations and qdisc callbacks retain the C implementation's external kernel semantics. */
extern "C" {
 fn eltree_insert(cl:*mut hfsc_class); fn eltree_remove(cl:*mut hfsc_class); fn eltree_update(cl:*mut hfsc_class);
 fn vttree_insert(cl:*mut hfsc_class); fn vttree_remove(cl:*mut hfsc_class); fn vttree_update(cl:*mut hfsc_class);
 fn cftree_insert(cl:*mut hfsc_class); fn cftree_remove(cl:*mut hfsc_class); fn cftree_update(cl:*mut hfsc_class);
 fn init_ed(cl:*mut hfsc_class,next_len:u32); fn update_ed(cl:*mut hfsc_class,next_len:u32); fn update_d(cl:*mut hfsc_class,next_len:u32);
 fn init_vf(cl:*mut hfsc_class,len:u32); fn update_vf(cl:*mut hfsc_class,len:u32,cur_time:u64);
 fn hfsc_change_rsc(cl:*mut hfsc_class,rsc:*mut tc_service_curve,cur_time:u64); fn hfsc_change_fsc(cl:*mut hfsc_class,fsc:*mut tc_service_curve); fn hfsc_change_usc(cl:*mut hfsc_class,usc:*mut tc_service_curve,cur_time:u64);
 fn hfsc_reset_class(cl:*mut hfsc_class); fn hfsc_schedule_watchdog(sch:*mut Qdisc);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
