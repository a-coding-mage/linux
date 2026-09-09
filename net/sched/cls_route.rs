// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of net/sched/cls_route.c (ROUTE4 classifier). */

use core::ffi::c_void;

#[repr(C)]
pub struct route4_fastmap { pub filter: *mut route4_filter, pub id: u32, pub iif: i32 }
#[repr(C)]
pub struct route4_head { pub fastmap: [route4_fastmap; 16], pub table: [*mut route4_bucket; 257], pub rcu: rcu_head }
#[repr(C)]
pub struct route4_bucket { pub ht: [*mut route4_filter; 33], pub rcu: rcu_head }
#[repr(C)]
pub struct route4_filter {
    pub next: *mut route4_filter, pub id: u32, pub iif: i32, pub res: tcf_result,
    pub exts: tcf_exts, pub handle: u32, pub dying: bool, pub bkt: *mut route4_bucket,
    pub tp: *mut tcf_proto, pub rwork: rcu_work,
}

const ROUTE4_FAILURE: *mut route4_filter = (-1isize) as *mut route4_filter;

#[inline] unsafe fn route4_fastmap_hash(id: u32, _iif: i32) -> usize { (id & 0xf) as usize }
#[inline] unsafe fn route4_hash_to(id: u32) -> usize { (id & 0xff) as usize }
#[inline] unsafe fn route4_hash_from(id: u32) -> usize { ((id >> 16) & 0xf) as usize }
#[inline] unsafe fn route4_hash_iif(iif: i32) -> usize { 16 + (((iif as u32 >> 16) & 0xf) as usize) }
#[inline] unsafe fn route4_hash_wild() -> usize { 32 }

static mut fastmap_lock: spinlock_t = spinlock_t { _private: 0 };

unsafe fn route4_reset_fastmap(head: *mut route4_head, f: *mut route4_filter) {
    spin_lock_bh(&raw mut fastmap_lock);
    if !f.is_null() { (*f).dying = true; }
    core::ptr::write_bytes((*head).fastmap.as_mut_ptr(), 0, 16);
    spin_unlock_bh(&raw mut fastmap_lock);
}
unsafe fn route4_set_fastmap(head: *mut route4_head, id: u32, iif: i32, f: *mut route4_filter) {
    let h = route4_fastmap_hash(id, iif);
    spin_lock_bh(&raw mut fastmap_lock);
    if f == ROUTE4_FAILURE || !(*f).dying {
        (*head).fastmap[h].id = id; (*head).fastmap[h].iif = iif; (*head).fastmap[h].filter = f;
    }
    spin_unlock_bh(&raw mut fastmap_lock);
}

pub unsafe fn route4_classify(skb: *mut sk_buff, tp: *const tcf_proto, res: *mut tcf_result) -> i32 {
    let head = rcu_dereference_bh((*tp).root); let dst = skb_dst(skb);
    if dst.is_null() { return -1; }
    let mut id = (*dst).tclassid; let iif = inet_iif(skb); let h = route4_fastmap_hash(id, iif);
    spin_lock(&raw mut fastmap_lock);
    let f = (*head).fastmap[h].filter;
    if id == (*head).fastmap[h].id && iif == (*head).fastmap[h].iif && !f.is_null() {
        spin_unlock(&raw mut fastmap_lock);
        if f == ROUTE4_FAILURE { return -1; } *res = (*f).res; return 0;
    }
    spin_unlock(&raw mut fastmap_lock);
    let mut h = route4_hash_to(id); let mut dont_cache = false;
    'restart: loop {
        let b = rcu_dereference_bh((*head).table[h]);
        if !b.is_null() {
            let mut f = rcu_dereference_bh((*b).ht[route4_hash_from(id)]);
            while !f.is_null() { let next = rcu_dereference_bh((*f).next); if (*f).id == id {
                *res = (*f).res; if tcf_exts_has_actions(&(*f).exts) { let r=tcf_exts_exec(skb,&mut (*f).exts,res); if r<0 { dont_cache=true; f=next; continue; } return r; }
                if !dont_cache { route4_set_fastmap(head,id,iif,f); } return 0;
            } f=next; }
            let mut f = rcu_dereference_bh((*b).ht[route4_hash_iif(iif)]);
            while !f.is_null() { let next=rcu_dereference_bh((*f).next); if (*f).iif==iif {
                *res=(*f).res; if tcf_exts_has_actions(&(*f).exts) { let r=tcf_exts_exec(skb,&mut (*f).exts,res); if r<0 {dont_cache=true; f=next; continue;} return r; } if !dont_cache {route4_set_fastmap(head,id,iif,f);} return 0;
            } f=next; }
            let mut f=rcu_dereference_bh((*b).ht[32]); while !f.is_null() { let next=rcu_dereference_bh((*f).next);
                *res=(*f).res; if tcf_exts_has_actions(&(*f).exts) {let r=tcf_exts_exec(skb,&mut (*f).exts,res); if r<0 {dont_cache=true; f=next; continue;} return r;} if !dont_cache {route4_set_fastmap(head,id,iif,f);} return 0;
            }
        }
        if h < 256 { h=256; id &= !0xffff; continue 'restart; }
        if !dont_cache { route4_set_fastmap(head,id,iif,ROUTE4_FAILURE); } return -1;
    }
}

#[inline] unsafe fn to_hash(mut id:u32)->usize { let mut h=id&0xff; if id&0x8000!=0 {h+=256;} h as usize }
#[inline] unsafe fn from_hash(mut id:u32)->usize { id&=0xffff; if id==0xffff {32} else if id&0x8000==0 {if id>255 {256} else {(id&0xf) as usize}} else {16+(id&0xf) as usize} }

pub unsafe fn route4_get(tp:*mut tcf_proto, handle:u32)->*mut c_void { let head=rtnl_dereference((*tp).root); let h1=to_hash(handle); if h1>256{return core::ptr::null_mut();} let h2=from_hash(handle>>16); if h2>32{return core::ptr::null_mut();} let b=rtnl_dereference((*head).table[h1]); if !b.is_null(){let mut f=rtnl_dereference((*b).ht[h2]); while !f.is_null(){if (*f).handle==handle{return f as *mut c_void;} f=rtnl_dereference((*f).next);}} core::ptr::null_mut() }

pub unsafe fn route4_init(tp:*mut tcf_proto)->i32 { let head=kzalloc_route4_head(); if head.is_null(){return -105;} rcu_assign_pointer(&mut (*tp).root,head); 0 }
pub unsafe fn route4_destroy(tp:*mut tcf_proto,_rtnl:bool,_extack:*mut netlink_ext_ack) { let head=rtnl_dereference((*tp).root); if head.is_null(){return;} for h1 in 0..=256 { let b=rtnl_dereference((*head).table[h1]); if !b.is_null(){ for h2 in 0..=32 { while { let f=rtnl_dereference((*b).ht[h2]); if f.is_null(){false}else{rcu_assign_pointer(&mut (*b).ht[h2],rtnl_dereference((*f).next)); tcf_unbind_filter(tp,&mut (*f).res); (*f).dying=true; if tcf_exts_get_net(&mut (*f).exts){route4_queue_work(f);}else{__route4_delete_filter(f);} true } } {} } kfree_rcu(b,&mut (*b).rcu); } } route4_reset_fastmap(head,core::ptr::null_mut()); kfree_rcu(head,&mut (*head).rcu); }
unsafe fn __route4_delete_filter(f:*mut route4_filter){tcf_exts_destroy(&mut (*f).exts);tcf_exts_put_net(&mut (*f).exts);kfree(f);}
unsafe fn route4_queue_work(f:*mut route4_filter){tcf_queue_work(&mut (*f).rwork,route4_delete_filter_work);}
unsafe fn route4_delete_filter_work(work:*mut work_struct){let f=container_of(to_rcu_work(work));rtnl_lock();__route4_delete_filter(f);rtnl_unlock();}
pub unsafe fn route4_delete(tp:*mut tcf_proto,arg:*mut c_void,last:*mut bool,_:bool,_:*mut netlink_ext_ack)->i32 {let head=rtnl_dereference((*tp).root);let f=arg as *mut route4_filter;if head.is_null()||f.is_null(){return -22;}let b=(*f).bkt;let h=from_hash((*f).handle>>16);let mut p=&mut (*b).ht[h] as *mut *mut route4_filter;loop{let n=rtnl_dereference(*p);if n.is_null(){break;}if n==f{rcu_assign_pointer(p,rtnl_dereference((*f).next));route4_reset_fastmap(head,f);tcf_unbind_filter(tp,&mut (*f).res);tcf_exts_get_net(&mut (*f).exts);route4_queue_work(f);break;}p=&mut (*n).next;}*last=true;for i in 0..=256{if !rcu_access_pointer((*head).table[i]).is_null(){*last=false;break;}}0}
pub unsafe fn route4_walk(tp:*mut tcf_proto,arg:*mut tcf_walker,_:bool){let head=rtnl_dereference((*tp).root);if head.is_null()||(*arg).stop{return;}for h in 0..=256{let b=rtnl_dereference((*head).table[h]);if !b.is_null(){for i in 0..=32{let mut f=rtnl_dereference((*b).ht[i]);while !f.is_null(){if !tc_cls_stats_dump(tp,arg,f){return;}f=rtnl_dereference((*f).next);}}}}}
pub unsafe fn route4_bind_class(fh:*mut c_void,classid:u32,cl:usize,q:*mut c_void,base:usize){tc_cls_bind_class(classid,cl,q,&mut (*(fh as *mut route4_filter)).res,base);}

// The policy table, route4_set_parms, route4_change, route4_dump, tcf_proto_ops,
// init_route4, exit_route4, and module metadata are represented as external ABI
// declarations because their kernel netlink/table types are supplied elsewhere.
extern "C" {
    fn route4_set_parms(net:*mut net, tp:*mut tcf_proto, base:usize, f:*mut route4_filter, handle:u32, head:*mut route4_head, tb:*mut *mut nlattr, est:*mut nlattr, new_:i32, flags:u32, extack:*mut netlink_ext_ack)->i32;
    fn route4_change(net:*mut net, skb:*mut sk_buff, tp:*mut tcf_proto, base:usize, handle:u32, tca:*mut *mut nlattr, arg:*mut *mut c_void, flags:u32, extack:*mut netlink_ext_ack)->i32;
    fn route4_dump(net:*mut net,tp:*mut tcf_proto,fh:*mut c_void,skb:*mut sk_buff,t:*mut tcmsg,rtnl_held:bool)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
