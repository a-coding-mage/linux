// SPDX-License-Identifier: GPL-2.0
// Translated from slub_kunit.c. Kernel declarations are supplied externally.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct kunit_resource { _private: [u8; 0] }
#[repr(C)]
pub struct kunit { _private: [u8; 0] }
#[repr(C)]
pub struct kmem_cache { pub offset: usize, pub flags: slab_flags_t }
#[repr(C)]
pub struct rcu_head { _private: [u8; 0] }
#[repr(C)]
pub struct kvfree_rcu_head { _private: [u8; 0] }
#[repr(C)]
pub struct work_struct { _private: [u8; 0] }
#[repr(C)]
pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)]
pub struct perf_event { pub overflow_handler_context: *mut c_void }
#[repr(C)]
pub struct perf_sample_data { _private: [u8; 0] }
#[repr(C)]
pub struct pt_regs { _private: [u8; 0] }
#[repr(C)]
pub struct kprobe { pub symbol_name: *const c_char, pub pre_handler: Option<unsafe extern "C" fn(*mut kprobe, *mut pt_regs) -> c_int> }
#[repr(C)]
pub struct perf_event_attr { pub type_: u32, pub config: u64, pub size: u32, pub pinned: u32, pub disabled: u32, pub freq: u32, pub sample_freq: u64 }
pub type slab_flags_t = usize;
pub type gfp_t = usize;
pub type u8 = core::ffi::c_uchar;

extern "C" {
    static mut resource: kunit_resource;
    static mut slab_errors: c_int;
    fn kmem_cache_create(name: *const c_char, size: usize, align: usize, flags: slab_flags_t, ctor: *mut c_void) -> *mut kmem_cache;
    fn kmem_cache_alloc(s: *mut kmem_cache, gfp: gfp_t) -> *mut u8;
    fn kmem_cache_free(s: *mut kmem_cache, p: *mut u8);
    fn kmem_cache_destroy(s: *mut kmem_cache);
    fn validate_slab_cache(s: *mut kmem_cache);
    fn kasan_disable_current();
    fn kasan_enable_current();
    fn alloc_hooks(p: *mut u8) -> *mut u8;
    fn __kmalloc_cache_noprof(s: *mut kmem_cache, gfp: gfp_t, size: usize) -> *mut u8;
    fn krealloc(p: *mut u8, size: usize, gfp: gfp_t) -> *mut u8;
    fn kfree(p: *mut u8);
    fn memset(p: *mut u8, value: c_int, size: usize) -> *mut c_void;
    fn kunit_skip(test: *mut kunit, msg: *const c_char) -> !;
    fn kunit_add_named_resource(test: *mut kunit, a: *mut c_void, b: *mut c_void, r: *mut kunit_resource, name: *const c_char, data: *mut c_void) -> c_int;
    fn kunit_info(test: *mut kunit, fmt: *const c_char, ...);
    fn kfree_rcu_nolock(p: *mut test_kfree_rcu_struct, field: *mut kvfree_rcu_head);
    fn kfree_nolock(p: *mut test_kfree_rcu_struct);
    fn kmalloc_nolock(size: usize, gfp: gfp_t, node: c_int) -> *mut test_kfree_rcu_struct;
    fn perf_event_create_kernel_counter(attr: *mut perf_event_attr, cpu: c_int, current: *mut c_void, handler: Option<unsafe extern "C" fn(*mut perf_event, *mut perf_sample_data, *mut pt_regs)>, ctx: *mut c_void) -> *mut perf_event;
    fn perf_event_enable(event: *mut perf_event);
    fn perf_event_disable(event: *mut perf_event);
    fn perf_event_release_kernel(event: *mut perf_event);
    fn register_kprobe(p: *mut kprobe) -> c_int;
    fn unregister_kprobe(p: *mut kprobe);
    fn alloc_workqueue(name: *const c_char, flags: usize, max_active: c_int) -> *mut workqueue_struct;
    fn destroy_workqueue(wq: *mut workqueue_struct);
    fn queue_work(wq: *mut workqueue_struct, work: *mut work_struct) -> bool;
    fn flush_work(work: *mut work_struct);
    fn msleep(delay: u32);
    fn get_random_u8() -> u8;
    static mut current: *mut c_void;
}

const SLAB_NO_USER_FLAGS: slab_flags_t = 0;
const SLAB_SKIP_KFENCE: slab_flags_t = 0;
const SLAB_RED_ZONE: slab_flags_t = 0;
const SLAB_POISON: slab_flags_t = 0;
const SLAB_KMALLOC: slab_flags_t = 0;
const SLAB_STORE_USER: slab_flags_t = 0;
const SLAB_NO_MERGE: slab_flags_t = 0;
const GFP_KERNEL: gfp_t = 0;
const GFP_KERNEL_ACCOUNT: gfp_t = 0;
const __GFP_ZERO: gfp_t = 0;
const __GFP_ACCOUNT: gfp_t = 0;
const NUMA_NO_NODE: c_int = -1;
const SLUB_RED_ACTIVE: u8 = 0;

static mut RESOURCE: kunit_resource = kunit_resource { _private: [] };
static mut SLAB_ERRORS: c_int = 0;

unsafe fn test_kmem_cache_create(name: *const c_char, size: u32, flags: slab_flags_t) -> *mut kmem_cache {
    let s = kmem_cache_create(name, size as usize, 0, flags | SLAB_NO_USER_FLAGS, core::ptr::null_mut());
    (*s).flags |= SLAB_SKIP_KFENCE;
    s
}

unsafe fn test_clobber_zone(test: *mut kunit) {
    let s = test_kmem_cache_create(c"TestSlub_RZ_alloc".as_ptr(), 64, SLAB_RED_ZONE);
    let p = kmem_cache_alloc(s, GFP_KERNEL);
    kasan_disable_current(); *p.add(64) = 0x12; validate_slab_cache(s);
    KUNIT_EXPECT_EQ(test, 2, SLAB_ERRORS); kasan_enable_current();
    kmem_cache_free(s, p); kmem_cache_destroy(s);
}

#[cfg(not(CONFIG_KASAN))]
unsafe fn test_next_pointer(test: *mut kunit) {
    let s = test_kmem_cache_create(c"TestSlub_next_ptr_free".as_ptr(), 64, SLAB_POISON);
    let p = kmem_cache_alloc(s, GFP_KERNEL); kmem_cache_free(s, p);
    let ptr_addr = p.add((*s).offset) as *mut usize; let tmp = *ptr_addr;
    *p.add((*s).offset) = !*p.add((*s).offset); validate_slab_cache(s); KUNIT_EXPECT_EQ(test, 3, SLAB_ERRORS);
    *ptr_addr = tmp; SLAB_ERRORS = 0; validate_slab_cache(s); KUNIT_EXPECT_EQ(test, 2, SLAB_ERRORS);
    SLAB_ERRORS = 0; validate_slab_cache(s); KUNIT_EXPECT_EQ(test, 0, SLAB_ERRORS); kmem_cache_destroy(s);
}

#[cfg(not(CONFIG_KASAN))]
unsafe fn test_first_word(test: *mut kunit) {
    let s = test_kmem_cache_create(c"TestSlub_1th_word_free".as_ptr(), 64, SLAB_POISON); let p = kmem_cache_alloc(s, GFP_KERNEL);
    kmem_cache_free(s, p); *p = 0x78; validate_slab_cache(s); KUNIT_EXPECT_EQ(test, 2, SLAB_ERRORS); kmem_cache_destroy(s);
}

#[cfg(not(CONFIG_KASAN))]
unsafe fn test_clobber_50th_byte(test: *mut kunit) {
    let s = test_kmem_cache_create(c"TestSlub_50th_word_free".as_ptr(), 64, SLAB_POISON); let p = kmem_cache_alloc(s, GFP_KERNEL);
    kmem_cache_free(s, p); *p.add(50) = 0x9a; validate_slab_cache(s); KUNIT_EXPECT_EQ(test, 2, SLAB_ERRORS); kmem_cache_destroy(s);
}

unsafe fn test_clobber_redzone_free(test: *mut kunit) {
    let s = test_kmem_cache_create(c"TestSlub_RZ_free".as_ptr(), 64, SLAB_RED_ZONE); let p = kmem_cache_alloc(s, GFP_KERNEL);
    kasan_disable_current(); kmem_cache_free(s, p); *p.add(64) = 0xab; validate_slab_cache(s); KUNIT_EXPECT_EQ(test, 2, SLAB_ERRORS);
    kasan_enable_current(); kmem_cache_destroy(s);
}

unsafe fn test_kmalloc_redzone_access(test: *mut kunit) {
    let s = test_kmem_cache_create(c"TestSlub_RZ_kmalloc".as_ptr(), 32, SLAB_KMALLOC | SLAB_STORE_USER | SLAB_RED_ZONE);
    let p = alloc_hooks(__kmalloc_cache_noprof(s, GFP_KERNEL, 18)); kasan_disable_current(); *p.add(18)=0xab; *p.add(19)=0xab;
    validate_slab_cache(s); KUNIT_EXPECT_EQ(test, 2, SLAB_ERRORS); kasan_enable_current(); kmem_cache_free(s,p); kmem_cache_destroy(s);
}

#[repr(C)]
pub union test_kfree_rcu_struct { pub rcu: rcu_head, pub kvrcu: kvfree_rcu_head }

unsafe fn test_kfree_rcu(test: *mut kunit) {
    if IS_BUILTIN(CONFIG_SLUB_KUNIT_TEST) { kunit_skip(test, c"can't do kfree_rcu() when test is built-in".as_ptr()); }
    let s = test_kmem_cache_create(c"TestSlub_kfree_rcu".as_ptr(), core::mem::size_of::<test_kfree_rcu_struct>() as u32, SLAB_NO_MERGE);
    let p = kmem_cache_alloc(s, GFP_KERNEL) as *mut test_kfree_rcu_struct; kfree_rcu(p, rcu); kmem_cache_destroy(s); KUNIT_EXPECT_EQ(test, 0, SLAB_ERRORS);
}

#[repr(C)] pub struct cache_destroy_work { pub work: work_struct, pub s: *mut kmem_cache }
unsafe fn cache_destroy_workfn(w: *mut work_struct) { let cdw = (w as *mut u8).sub(0) as *mut cache_destroy_work; kmem_cache_destroy((*cdw).s); }
const KMEM_CACHE_DESTROY_NR: c_int = 10;

unsafe fn test_leak_destroy(test: *mut kunit) { let s=test_kmem_cache_create(c"TestSlub_leak_destroy".as_ptr(),64,SLAB_NO_MERGE); kmem_cache_alloc(s,GFP_KERNEL); kmem_cache_destroy(s); KUNIT_EXPECT_EQ(test,2,SLAB_ERRORS); }

unsafe fn test_krealloc_redzone_zeroing(test: *mut kunit) {
    let s=test_kmem_cache_create(c"TestSlub_krealloc".as_ptr(),64,SLAB_KMALLOC|SLAB_STORE_USER|SLAB_RED_ZONE); let mut p=alloc_hooks(__kmalloc_cache_noprof(s,GFP_KERNEL,48)); memset(p,0xff,48); kasan_disable_current();
    p=krealloc(p,40,GFP_KERNEL|__GFP_ZERO); for i in 40..64 { KUNIT_EXPECT_EQ(test,*p.add(i),SLUB_RED_ACTIVE); }
    p=krealloc(p,56,GFP_KERNEL|__GFP_ZERO); for i in 40..56 { KUNIT_EXPECT_EQ(test,*p.add(i),0); } for i in 56..64 { KUNIT_EXPECT_EQ(test,*p.add(i),SLUB_RED_ACTIVE); }
    validate_slab_cache(s); KUNIT_EXPECT_EQ(test,0,SLAB_ERRORS); memset(p,0xff,56); p=krealloc(p,112,GFP_KERNEL|__GFP_ZERO);
    for i in 0..56 { KUNIT_EXPECT_EQ(test,*p.add(i),0xff); } for i in 56..112 { KUNIT_EXPECT_EQ(test,*p.add(i),0); } kfree(p); kasan_enable_current(); kmem_cache_destroy(s);
}

unsafe fn test_init(test: *mut kunit) -> c_int { SLAB_ERRORS=0; kunit_add_named_resource(test,core::ptr::null_mut(),core::ptr::null_mut(),&mut RESOURCE,c"slab_errors".as_ptr(),&mut SLAB_ERRORS as *mut _ as *mut c_void); 0 }

unsafe fn test_kfree_rcu_wq_destroy(test: *mut kunit) {
    if IS_BUILTIN(CONFIG_SLUB_KUNIT_TEST) { kunit_skip(test,c"can't do kfree_rcu() when test is built-in".as_ptr()); }
    let mut cdw=cache_destroy_work { work: work_struct{_private:[]}, s:core::ptr::null_mut() };
    let wq=alloc_workqueue(c"test_kfree_rcu_destroy_wq".as_ptr(),WQ_HIGHPRI|WQ_UNBOUND|WQ_MEM_RECLAIM,0);
    if wq.is_null() { kunit_skip(test,c"failed to alloc wq".as_ptr()); }
    for _ in 0..KMEM_CACHE_DESTROY_NR { let s=test_kmem_cache_create(c"TestSlub_kfree_rcu_wq_destroy".as_ptr(),core::mem::size_of::<test_kfree_rcu_struct>() as u32,SLAB_NO_MERGE); if s.is_null(){kunit_skip(test,c"failed to create cache".as_ptr());} let p=kmem_cache_alloc(s,GFP_KERNEL) as *mut test_kfree_rcu_struct; kfree_rcu(p,rcu); cdw.s=s; msleep(get_random_u8() as u32); queue_work(wq,&mut cdw.work); flush_work(&mut cdw.work); }
    destroy_workqueue(wq); KUNIT_EXPECT_EQ(test,0,SLAB_ERRORS);
}

#[cfg(any(CONFIG_PERF_EVENTS, all(CONFIG_KPROBES, CONFIG_SMP)))]
unsafe fn test_kmalloc_and_friends() { let mut objects:[*mut test_kfree_rcu_struct;1000]=[core::ptr::null_mut();1000]; for i in 0..1000 { for j in 0..1000 { objects[j]=kmalloc_nolock(core::mem::size_of::<test_kfree_rcu_struct>(),if i&1!=0{GFP_KERNEL}else{GFP_KERNEL_ACCOUNT},NUMA_NO_NODE); if objects[j].is_null(){for k in 0..j{kfree(objects[k] as *mut u8);} return;} } for j in 0..1000 { kfree(objects[j] as *mut u8); } } }
#[cfg(any(CONFIG_PERF_EVENTS, all(CONFIG_KPROBES, CONFIG_SMP)))]
unsafe fn test_nolock(_ctx:*mut c_void) { }
#[cfg(CONFIG_PERF_EVENTS)] unsafe fn test_kmalloc_nolock_and_friends_perf(test:*mut kunit) { test_kmalloc_and_friends(); KUNIT_EXPECT_EQ(test,0,SLAB_ERRORS); }
#[cfg(all(CONFIG_KPROBES, CONFIG_SMP))] unsafe fn test_kmalloc_nolock_and_friends_kprobe(test:*mut kunit) { test_kmalloc_and_friends(); KUNIT_EXPECT_EQ(test,0,SLAB_ERRORS); }

// KUNIT_CASE registration and kernel module metadata are represented by the external framework.
extern "C" { fn KUNIT_EXPECT_EQ(test: *mut kunit, expected: i32, actual: i32); fn IS_BUILTIN(config: c_int) -> bool; fn kfree_rcu(p: *mut test_kfree_rcu_struct, field: *mut rcu_head); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
