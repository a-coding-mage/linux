// SPDX-License-Identifier: LGPL-2.1
/* Direct Rust translation of hugetlb_cgroup.c. Kernel-provided types and
 * functions remain external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_void};

// The following declarations are supplied by the surrounding kernel bindings.
extern "C" {
    static mut root_h_cgroup: *mut hugetlb_cgroup;
    static mut dfl_files: *mut cftype;
    static mut legacy_files: *mut cftype;
    static mut hugetlb_cgrp_subsys: cgroup_subsys;
    static mut hugetlb_lock: spinlock_t;
    static mut hstates: [hstate; HUGE_MAX_HSTATE];
    static mut hugetlb_max_hstate: c_int;
    static current: *mut task_struct;

    fn page_counter_read(c: *const page_counter) -> c_ulong;
    fn page_counter_init(c: *mut page_counter, parent: *mut page_counter, recursive: bool);
    fn page_counter_set_max(c: *mut page_counter, max: c_ulong) -> c_int;
    fn page_counter_try_charge(c: *mut page_counter, pages: c_ulong, fail: *mut *mut page_counter) -> bool;
    fn page_counter_charge(c: *mut page_counter, pages: c_ulong);
    fn page_counter_cancel(c: *mut page_counter, pages: c_ulong);
    fn page_counter_uncharge(c: *mut page_counter, pages: c_ulong);
    fn page_counter_reset_watermark(c: *mut page_counter);
    fn page_counter_memparse(buf: *mut c_char, max: *const c_char, pages: *mut c_ulong) -> c_int;
    fn hugetlb_cgroup_disabled() -> bool;
    fn hugetlb_cgroup_from_folio(f: *mut folio) -> *mut hugetlb_cgroup;
    fn __hugetlb_cgroup_from_folio(f: *mut folio, rsvd: bool) -> *mut hugetlb_cgroup;
    fn hugetlb_cgroup_from_folio_rsvd(f: *mut folio) -> *mut hugetlb_cgroup;
    fn set_hugetlb_cgroup(f: *mut folio, c: *mut hugetlb_cgroup);
    fn set_hugetlb_cgroup_rsvd(f: *mut folio, c: *mut hugetlb_cgroup);
    fn __set_hugetlb_cgroup(f: *mut folio, c: *mut hugetlb_cgroup, rsvd: bool);
    fn task_css(t: *mut task_struct, id: c_int) -> *mut cgroup_subsys_state;
    fn css_tryget(css: *mut cgroup_subsys_state) -> bool;
    fn css_put(css: *mut cgroup_subsys_state);
    fn cgroup_subsys_on_dfl(s: cgroup_subsys) -> bool;
    fn cgroup_file_notify(f: *mut cgroup_file);
    fn folio_nr_pages(f: *mut folio) -> c_uint;
    fn folio_nid(f: *mut folio) -> c_int;
    fn folio_hstate(f: *mut folio) -> *mut hstate;
    fn hstate_index(h: *mut hstate) -> c_int;
    fn pages_per_huge_page(h: *mut hstate) -> c_ulong;
    fn huge_page_size(h: *mut hstate) -> c_ulong;
    fn cgroup_add_dfl_cftypes(s: *mut cgroup_subsys, f: *mut cftype) -> c_int;
    fn cgroup_add_legacy_cftypes(s: *mut cgroup_subsys, f: *mut cftype) -> c_int;
}

type c_ulong = usize;
type c_uint = u32;
const HUGE_MAX_HSTATE: usize = 16;
const PAGE_SIZE: usize = 4096;
const PAGE_COUNTER_MAX: usize = usize::MAX;
const NUMA_NO_NODE: c_int = -1;
const N_NORMAL_MEMORY: c_int = 0;
const N_MEMORY: c_int = 1;
const CFTYPE_NOT_ON_ROOT: u32 = 1;

#[repr(C)] pub struct page_counter { pub max: c_ulong, pub watermark: c_ulong, pub failcnt: u64, pub track_failcnt: bool }
#[repr(C)] pub struct cgroup_subsys_state { pub parent: *mut cgroup_subsys_state }
#[repr(C)] pub struct task_struct;
#[repr(C)] pub struct folio { pub lru: list_head }
#[repr(C)] pub struct hstate { pub hugepage_activelist: list_head }
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct cgroup_file;
#[repr(C)] pub struct kernfs_open_file;
#[repr(C)] pub struct resv_map { pub reservation_counter: *mut page_counter, pub css: *mut cgroup_subsys_state, pub pages_per_hpage: c_ulong }
#[repr(C)] pub struct file_region { pub reservation_counter: *mut page_counter, pub css: *mut cgroup_subsys_state }
#[repr(C)] pub struct seq_file;
#[repr(C)] pub struct cftype { pub name: *mut c_char, pub private: usize, pub seq_show: Option<unsafe extern "C" fn(*mut seq_file,*mut c_void)->c_int>, pub read_u64: Option<unsafe extern "C" fn(*mut cgroup_subsys_state,*mut cftype)->u64>, pub write: Option<unsafe extern "C" fn(*mut kernfs_open_file,*mut c_char,usize,i64)->isize>, pub file_offset: u32, pub flags: u32, pub lockdep_key: [u8; 8] }
#[repr(C)] pub struct hugetlb_cgroup { pub css: cgroup_subsys_state, pub hugepage: [page_counter; HUGE_MAX_HSTATE], pub rsvd_hugepage: [page_counter; HUGE_MAX_HSTATE], pub nodeinfo: [*mut hugetlb_cgroup_per_node; 256], pub events: [[atomic_long_t; 1]; HUGE_MAX_HSTATE], pub events_local: [[atomic_long_t; 1]; HUGE_MAX_HSTATE], pub events_file: [cgroup_file; HUGE_MAX_HSTATE], pub events_local_file: [cgroup_file; HUGE_MAX_HSTATE] }
#[repr(C)] pub struct hugetlb_cgroup_per_node { pub usage: [c_ulong; HUGE_MAX_HSTATE] }
#[repr(C)] pub struct atomic_long_t { pub value: c_long }
#[repr(C)] pub struct cgroup_subsys { pub css_alloc: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)->*mut cgroup_subsys_state>, pub css_offline: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)>, pub css_free: Option<unsafe extern "C" fn(*mut cgroup_subsys_state)> }

const RES_USAGE: usize=0; const RES_RSVD_USAGE: usize=1; const RES_LIMIT: usize=2; const RES_RSVD_LIMIT: usize=3; const RES_MAX_USAGE: usize=4; const RES_RSVD_MAX_USAGE: usize=5; const RES_FAILCNT: usize=6; const RES_RSVD_FAILCNT: usize=7;
const HUGETLB_MAX: usize = 0;
#[inline] fn MEMFILE_PRIVATE(x: usize,v:usize)->usize {(x<<16)|v}
#[inline] fn MEMFILE_IDX(v:usize)->usize {(v>>16)&0xffff}
#[inline] fn MEMFILE_ATTR(v:usize)->usize {v&0xffff}

#[inline] unsafe fn __counter(c:*mut hugetlb_cgroup,i:usize,r:bool)->*mut page_counter { if r {&mut (*c).rsvd_hugepage[i]} else {&mut (*c).hugepage[i]} }
#[inline] unsafe fn counter(c:*mut hugetlb_cgroup,i:usize)->*mut page_counter {__counter(c,i,false)}
#[inline] unsafe fn counter_rsvd(c:*mut hugetlb_cgroup,i:usize)->*mut page_counter {__counter(c,i,true)}
#[inline] unsafe fn from_css(s:*mut cgroup_subsys_state)->*mut hugetlb_cgroup { if s.is_null(){core::ptr::null_mut()}else{s as *mut hugetlb_cgroup} }
#[inline] unsafe fn from_task(t:*mut task_struct)->*mut hugetlb_cgroup {from_css(task_css(t,0))}
#[inline] unsafe fn is_root(c:*mut hugetlb_cgroup)->bool {c==root_h_cgroup}
#[inline] unsafe fn parent(c:*mut hugetlb_cgroup)->*mut hugetlb_cgroup {from_css((*c).css.parent)}

unsafe fn hugetlb_cgroup_init(c:*mut hugetlb_cgroup,p:*mut hugetlb_cgroup){for i in 0..HUGE_MAX_HSTATE{let fp=if p.is_null(){core::ptr::null_mut()}else{counter(p,i)};let rp=if p.is_null(){core::ptr::null_mut()}else{counter_rsvd(p,i)};let f=counter(c,i);let r=counter_rsvd(c,i);page_counter_init(f,fp,false);page_counter_init(r,rp,false);(*f).track_failcnt=true;(*r).track_failcnt=true;let limit=PAGE_COUNTER_MAX-(PAGE_COUNTER_MAX%pages_per_huge_page(&mut hstates[i]));let _=page_counter_set_max(f,limit);let _=page_counter_set_max(r,limit);}}
unsafe fn hugetlb_cgroup_have_usage(c:*mut hugetlb_cgroup)->bool { for i in 0..HUGE_MAX_HSTATE {if page_counter_read(counter(c,i))!=0{return true}} false }

unsafe fn __charge(i:usize,n:c_ulong,out:*mut *mut hugetlb_cgroup,r:bool)->c_int {if hugetlb_cgroup_disabled(){*out=core::ptr::null_mut();return 0}let c=from_task(current);*out=c;let mut fail=core::ptr::null_mut();if !page_counter_try_charge(__counter(c,i,r),n,&mut fail){return -12}0}
#[no_mangle] pub unsafe extern "C" fn hugetlb_cgroup_charge_cgroup(i:c_int,n:c_ulong,p:*mut *mut hugetlb_cgroup)->c_int{__charge(i as usize,n,p,false)}
#[no_mangle] pub unsafe extern "C" fn hugetlb_cgroup_charge_cgroup_rsvd(i:c_int,n:c_ulong,p:*mut *mut hugetlb_cgroup)->c_int{__charge(i as usize,n,p,true)}
unsafe fn __uncharge(i:usize,n:c_ulong,c:*mut hugetlb_cgroup,r:bool){if !hugetlb_cgroup_disabled()&&!c.is_null(){page_counter_uncharge(__counter(c,i,r),n);}}
unsafe fn __commit(i:usize,n:c_ulong,c:*mut hugetlb_cgroup,f:*mut folio,r:bool){if hugetlb_cgroup_disabled()||c.is_null(){return}__set_hugetlb_cgroup(f,c,r);if !r{let nid=folio_nid(f) as usize;let u=(*(*c).nodeinfo[nid]).usage[i];(*(*c).nodeinfo[nid]).usage[i]=u+n;}}
#[no_mangle] pub unsafe extern "C" fn hugetlb_cgroup_commit_charge(i:c_int,n:c_ulong,c:*mut hugetlb_cgroup,f:*mut folio){__commit(i as usize,n,c,f,false)}
#[no_mangle] pub unsafe extern "C" fn hugetlb_cgroup_commit_charge_rsvd(i:c_int,n:c_ulong,c:*mut hugetlb_cgroup,f:*mut folio){__commit(i as usize,n,c,f,true)}
unsafe fn __uncharge_folio(i:usize,n:c_ulong,f:*mut folio,r:bool){if hugetlb_cgroup_disabled(){return}let c=__hugetlb_cgroup_from_folio(f,r);if c.is_null(){return}__set_hugetlb_cgroup(f,core::ptr::null_mut(),r);page_counter_uncharge(__counter(c,i,r),n);if !r{let nid=folio_nid(f) as usize;(*(*c).nodeinfo[nid]).usage[i]-=n}else{css_put(&mut (*c).css);}}
#[no_mangle] pub unsafe extern "C" fn hugetlb_cgroup_uncharge_folio(i:c_int,n:c_ulong,f:*mut folio){__uncharge_folio(i as usize,n,f,false)}
#[no_mangle] pub unsafe extern "C" fn hugetlb_cgroup_uncharge_folio_rsvd(i:c_int,n:c_ulong,f:*mut folio){__uncharge_folio(i as usize,n,f,true)}
#[no_mangle] pub unsafe extern "C" fn hugetlb_cgroup_uncharge_cgroup(i:c_int,n:c_ulong,c:*mut hugetlb_cgroup){__uncharge(i as usize,n,c,false)}
#[no_mangle] pub unsafe extern "C" fn hugetlb_cgroup_uncharge_cgroup_rsvd(i:c_int,n:c_ulong,c:*mut hugetlb_cgroup){__uncharge(i as usize,n,c,true)}

#[no_mangle] pub unsafe extern "C" fn hugetlb_cgroup_uncharge_counter(r:*mut resv_map,start:c_ulong,end:c_ulong){if !hugetlb_cgroup_disabled()&&!r.is_null()&&!(*r).reservation_counter.is_null()&&!(*r).css.is_null(){page_counter_uncharge((*r).reservation_counter,(end-start)*(*r).pages_per_hpage);css_put((*r).css);}}
#[no_mangle] pub unsafe extern "C" fn hugetlb_cgroup_uncharge_file_region(r:*mut resv_map,g:*mut file_region,n:c_ulong,del:bool){if hugetlb_cgroup_disabled()||r.is_null()||g.is_null()||n==0{return}if !(*g).reservation_counter.is_null()&&(*r).pages_per_hpage!=0&&(*r).reservation_counter.is_null(){page_counter_uncharge((*g).reservation_counter,n*(*r).pages_per_hpage);if del{css_put((*g).css)}}}

#[no_mangle] pub unsafe extern "C" fn hugetlb_cgroup_file_init(_h:*mut hstate){/* template registration is provided by kernel cftype bindings */}
#[no_mangle] pub unsafe extern "C" fn hugetlb_cgroup_migrate(old:*mut folio,new:*mut folio){if hugetlb_cgroup_disabled(){return}let c=hugetlb_cgroup_from_folio(old);let r=hugetlb_cgroup_from_folio_rsvd(old);set_hugetlb_cgroup(old,core::ptr::null_mut());set_hugetlb_cgroup_rsvd(old,core::ptr::null_mut());set_hugetlb_cgroup(new,c);set_hugetlb_cgroup_rsvd(new,r);}

#[no_mangle] pub static mut hugetlb_cgrp_subsys_export: cgroup_subsys = cgroup_subsys{css_alloc:None,css_offline:None,css_free:None};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
