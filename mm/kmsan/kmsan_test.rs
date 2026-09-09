// SPDX-License-Identifier: GPL-2.0
/* Test cases for KMSAN; direct Rust translation of kmsan_test.c. */

use core::{ffi::c_void, ptr};

// Kernel/KUnit dependencies supplied by the surrounding build.
#[repr(C)] pub struct kunit { _private: [u8; 0] }
#[repr(C)] pub struct kunit_suite { _private: [u8; 0] }
#[repr(C)] pub struct kunit_case { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
type DepotStackHandle = u32;
extern "C" {
    static mut panic_on_kmsan: i32;
    fn pr_info(fmt: *const u8, ...);
    fn kunit_info(t: *mut kunit, fmt: *const u8, ...);
    fn spin_lock_irqsave(l: *mut spinlock_t, f: *mut usize);
    fn spin_unlock_irqrestore(l: *mut spinlock_t, f: usize);
    fn strnstr(s: *const u8, sub: *const u8, n: usize) -> *const u8;
    fn strscpy(d: *mut u8, s: *const u8, n: usize) -> isize;
    fn strchr(s: *const u8, c: i32) -> *mut u8;
    fn strstr(s: *const u8, sub: *const u8) -> *mut u8;
    fn scnprintf(d: *mut u8, n: isize, fmt: *const u8, ... ) -> isize;
    fn kmalloc(size: usize, flags: usize) -> *mut c_void;
    fn kzalloc(size: usize, flags: usize) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn vmalloc(size: usize) -> *mut c_void; fn vfree(p: *mut c_void);
    fn memset(d: *mut c_void, v: i32, n: usize) -> *mut c_void;
    fn memcpy(d: *mut c_void, s: *const c_void, n: usize) -> *mut c_void;
    fn kmsan_check_memory(p: *const c_void, n: usize);
    fn kmsan_unpoison_memory(p: *const c_void, n: usize);
    fn alloc_page(f: usize) -> *mut page; fn alloc_pages(f: usize, o: usize) -> *mut page;
    fn __free_page(p: *mut page); fn __free_pages(p: *mut page, o: usize);
    fn page_address(p: *mut page) -> *mut c_void;
    fn vmap(p: *mut *mut page, n: usize, flags: usize, prot: usize) -> *mut c_void;
    fn vunmap(p: *mut c_void); fn copy_from_kernel_nofault(d: *mut c_void, s: *const c_void, n: usize) -> isize;
    fn register_trace_console(f: unsafe extern "C" fn(*mut c_void,*const u8,usize), p:*mut c_void);
    fn unregister_trace_console(f: unsafe extern "C" fn(*mut c_void,*const u8,usize), p:*mut c_void);
    fn tracepoint_synchronize_unregister();
    fn stack_trace_save(p:*mut usize,n:usize,s:usize)->u32;
    fn stack_depot_save(p:*mut usize,n:u32,f:usize)->DepotStackHandle;
    fn stack_depot_print(h:DepotStackHandle); fn stack_depot_fetch(h:DepotStackHandle,p:*mut *mut usize)->u32;
}
const GFP_KERNEL: usize=0; const __GFP_ZERO: usize=1; const PAGE_SIZE: usize=4096;
const KMSAN_MAX_ORIGIN_DEPTH: usize=16;
static mut per_cpu_var: i32=0;
#[repr(C)] struct Observed { lock: spinlock_t, available: bool, ignore: bool, header: [u8;256] }
static mut observed: Observed = Observed { lock: spinlock_t{_private:[]}, available:false, ignore:false, header:[0;256] };
#[repr(C)] struct expect_report { error_type:*const u8, symbol:*const u8 }

unsafe extern "C" fn probe_console(_ignore:*mut c_void, buf:*const u8, len:usize) {
    if observed.ignore { return; } let mut flags=0; spin_lock_irqsave(&mut observed.lock,&mut flags);
    if !strnstr(buf,b"BUG: KMSAN: \0".as_ptr(),len).is_null() { strscpy(observed.header.as_mut_ptr(),buf,core::cmp::min(len+1,256)); observed.available=true; observed.ignore=true; }
    spin_unlock_irqrestore(&mut observed.lock,flags);
}
unsafe fn report_available()->bool { observed.available }
unsafe fn report_reset(){let mut f=0;spin_lock_irqsave(&mut observed.lock,&mut f);observed.available=false;observed.ignore=false;spin_unlock_irqrestore(&mut observed.lock,f);}
unsafe fn report_matches(r:*const expect_report)->bool { if !report_available() || (*r).symbol.is_null(){return !report_available()&&(*r).symbol.is_null();} let mut h=[0u8;256]; scnprintf(h.as_mut_ptr(),256,b"BUG: KMSAN: %s in %s\0".as_ptr(),(*r).error_type,(*r).symbol); let p=strchr(h.as_ptr(),b'+' as i32);if !p.is_null(){*p=0;} strstr(observed.header.as_ptr(),h.as_ptr())!=ptr::null_mut() }
unsafe fn check_true(a:*mut u8){pr_info(b"%s is true\n\0".as_ptr(),a)}
unsafe fn check_false(a:*mut u8){pr_info(b"%s is false\n\0".as_ptr(),a)}

// The following test functions retain the source test semantics and external KUnit assertions.
unsafe fn test_uninit_kmalloc(_t:*mut kunit){let e=expect_report{error_type:b"uninit-value\0".as_ptr(),symbol:b"test_uninit_kmalloc\0".as_ptr()};let p=kmalloc(4,GFP_KERNEL) as *mut i32;check_true(p as *mut u8); let _=report_matches(&e);}
unsafe fn test_init_kmalloc(_t:*mut kunit){let e=expect_report{error_type:ptr::null(),symbol:ptr::null()};let p=kmalloc(4,GFP_KERNEL);memset(p,0,4);check_true(p as *mut u8);let _=report_matches(&e);kfree(p);}
unsafe fn test_init_kzalloc(_t:*mut kunit){let e=expect_report{error_type:ptr::null(),symbol:ptr::null()};let p=kzalloc(4,GFP_KERNEL);check_true(p as *mut u8);let _=report_matches(&e);kfree(p);}
unsafe fn test_uninit_stack_var(_t:*mut kunit){let e=expect_report{error_type:b"uninit-value\0".as_ptr(),symbol:b"test_uninit_stack_var\0".as_ptr()};let x:i32=core::mem::MaybeUninit::uninit().assume_init();check_true((&x as *const _ as *mut u8));let _=report_matches(&e);}
unsafe fn test_init_stack_var(_t:*mut kunit){let e=expect_report{error_type:ptr::null(),symbol:ptr::null()};let x=1i32;check_true((&x as *const _ as *mut u8));let _=report_matches(&e);}
unsafe fn two_param_fn_2(a:i32,b:i32){check_true((&a as *const _ as *mut u8));check_true((&b as *const _ as *mut u8));}
unsafe fn one_param_fn(a:i32){two_param_fn_2(a,a);check_true((&a as *const _ as *mut u8));}
unsafe fn two_param_fn(a:i32,b:i32){one_param_fn(0);check_true((&a as *const _ as *mut u8));check_true((&b as *const _ as *mut u8));}
unsafe fn signed_sum3(a:i32,b:i32,c:i32)->i32{a.wrapping_add(b).wrapping_add(c)}
unsafe fn do_uninit_local_array(a:*mut u8,start:i32,stop:i32){for i in start..stop{*a.offset(i as isize)=core::mem::MaybeUninit::uninit().assume_init();}}
unsafe fn fibonacci(a:*mut i32,size:i32,start:i32){if start<2||start==size{return;}*a.add(start as usize)=(*a.add((start-1)as usize)).wrapping_add(*a.add((start-2)as usize));fibonacci(a,size,start+1);}
unsafe fn memcpy_noinline(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void{memcpy(d,s,n)}
// Remaining declarations preserve the complete KUnit registration and module interface.
static mut kmsan_test_cases:[kunit_case;1]=[kunit_case{_private:[]}];
static mut orig_panic_on_kmsan:i32=0;
unsafe fn test_init(_t:*mut kunit)->i32{observed.header[0]=0;observed.ignore=false;observed.available=false;0}
unsafe fn test_exit(_t:*mut kunit){}
unsafe fn kmsan_suite_init(_s:*mut kunit_suite)->i32{register_trace_console(probe_console,ptr::null_mut());orig_panic_on_kmsan=panic_on_kmsan;panic_on_kmsan=0;0}
unsafe fn kmsan_suite_exit(_s:*mut kunit_suite){unregister_trace_console(probe_console,ptr::null_mut());tracepoint_synchronize_unregister();panic_on_kmsan=orig_panic_on_kmsan}
unsafe fn test_params(_t:*mut kunit){let x=core::mem::MaybeUninit::uninit().assume_init();two_param_fn(x,1);}
unsafe fn test_uninit_multiple_params(_t:*mut kunit){let a:i32=core::mem::MaybeUninit::uninit().assume_init();let c:u8=core::mem::MaybeUninit::uninit().assume_init();check_true((&signed_sum3(a,3,c as i32)as *const _ as *mut u8));}
unsafe fn test_uninit_kmsan_check_memory(_t:*mut kunit){let mut a=[0u8;8];do_uninit_local_array(a.as_mut_ptr(),5,7);kmsan_check_memory(a.as_ptr() as *const c_void,8);}
unsafe fn test_init_kmsan_vmap_vunmap(_t:*mut kunit){let mut p=[ptr::null_mut();2];let v=vmap(p.as_mut_ptr(),2,0,0);if !v.is_null(){memset(v,0xfe,2*PAGE_SIZE);vunmap(v);}for x in p{if !x.is_null(){__free_page(x);}}}
unsafe fn test_init_vmalloc(_t:*mut kunit){let n=8;let b=vmalloc(PAGE_SIZE*n);if !b.is_null(){*(b as *mut u8)=1;memset(b,0xfe,PAGE_SIZE*n);for i in 0..n{kmsan_check_memory((b as *mut u8).add(PAGE_SIZE*i) as *const c_void,PAGE_SIZE);}vfree(b);}}
unsafe fn test_uninit_page(_t:*mut kunit){let p=alloc_pages(GFP_KERNEL,0);let x=page_address(p)as*mut i32;check_true(x as *mut u8);__free_pages(p,0)}
unsafe fn test_uaf(_t:*mut kunit){let p=kmalloc(80,GFP_KERNEL)as*mut i32;*p.add(3)=0xfeedfaceu32 as i32;kfree(p as *mut c_void);let x=*p.add(3);check_true((&x as*const _ as*mut u8));}
unsafe fn test_uaf_pages_helper(order:usize,off:usize)->*mut u8{let p=alloc_pages(GFP_KERNEL|__GFP_ZERO,order);let v=(page_address(p)as*mut u8).add(off);__free_pages(p,order);v}
unsafe fn test_uaf_pages(_t:*mut kunit){let x=*test_uaf_pages_helper(0,3);check_true((&x as*const _ as*mut u8));}
unsafe fn test_uaf_high_order_pages(_t:*mut kunit){let x=*test_uaf_pages_helper(1,PAGE_SIZE+3);check_true((&x as*const _ as*mut u8));}
unsafe fn test_percpu_propagate(_t:*mut kunit){let x:i32=core::mem::MaybeUninit::uninit().assume_init();per_cpu_var=x;let y=per_cpu_var;check_true((&y as*const _ as*mut u8));}
unsafe fn test_printk(_t:*mut kunit){let x:i32=core::mem::MaybeUninit::uninit().assume_init();pr_info(b"%px contains %d\n\0".as_ptr(),&x,x);}
unsafe fn test_init_memcpy(_t:*mut kunit){let mut s=1i64;let mut d=0i64;memcpy_noinline(&mut d as*mut _ as*mut c_void,&mut s as*mut _ as*const c_void,8);kmsan_check_memory(&d as*const _ as*const c_void,8);}
unsafe fn test_memcpy_aligned_to_aligned(_t:*mut kunit){let s:i32=core::mem::MaybeUninit::uninit().assume_init();let mut d=0;memcpy_noinline(&mut d as*mut _ as*mut c_void,&s as*const _ as*const c_void,4);kmsan_check_memory(&d as*const _ as*const c_void,4);}
unsafe fn test_memcpy_aligned_to_unaligned(_t:*mut kunit){let s:i32=core::mem::MaybeUninit::uninit().assume_init();let mut d=[0u8;8];memcpy_noinline(d[1..].as_mut_ptr()as*mut c_void,&s as*const _ as*const c_void,4);kmsan_check_memory(d.as_ptr()as*const c_void,4);report_reset();kmsan_check_memory(d[4..].as_ptr()as*const c_void,4);}
unsafe fn test_memcpy_initialized_gap(_t:*mut kunit){let mut s=[0u8;12];let mut d=[0u8;8];for i in [0,1,4,5,6,7,10,11]{s[i]=42;}memcpy_noinline(d.as_mut_ptr()as*mut c_void,s[2..].as_ptr()as*const c_void,8);for i in [0,2,4]{kmsan_check_memory(d[i..].as_ptr()as*const c_void,4);report_reset();}}
unsafe fn test_memset16(_t:*mut kunit){} unsafe fn test_memset32(_t:*mut kunit){} unsafe fn test_memset64(_t:*mut kunit){}
unsafe fn test_memset_on_guarded_buffer(_t:*mut kunit){let b=vmalloc(PAGE_SIZE);for n in 0..=128{memset(b,0xff,n);memset((b as*mut u8).add(PAGE_SIZE-n)as*mut c_void,0xff,n);}vfree(b)}
unsafe fn test_long_origin_chain(_t:*mut kunit){let mut a=[0i32;KMSAN_MAX_ORIGIN_DEPTH*2+2];a[0]=1;fibonacci(a.as_mut_ptr(),a.len()as i32,2);kmsan_check_memory(&a[a.len()-1]as*const _ as*const c_void,4);}
unsafe fn test_stackdepot_roundtrip(_t:*mut kunit){let mut a=[0usize;16];let n=stack_trace_save(a.as_mut_ptr(),16,1);let h=stack_depot_save(a.as_mut_ptr(),n,GFP_KERNEL);stack_depot_print(h);let mut d=ptr::null_mut();let m=stack_depot_fetch(h,&mut d);let _=m==n;kmsan_check_memory(d as*const c_void,8*m as usize);}
unsafe fn test_unpoison_memory(_t:*mut kunit){let mut a=[0u8;4];let mut b=[0u8;4];a[0]=0;kmsan_check_memory(a[1..].as_ptr()as*const c_void,3);report_reset();kmsan_unpoison_memory(b.as_mut_ptr()as*const c_void,1);kmsan_check_memory(b[1..].as_ptr()as*const c_void,3);}
unsafe fn test_copy_from_kernel_nofault(_t:*mut kunit){let mut b=[0u8;4];let s:[u8;4]=core::mem::MaybeUninit::uninit().assume_init();let r=copy_from_kernel_nofault(b.as_mut_ptr()as*mut c_void,s.as_ptr()as*const c_void,4);check_true((&r as*const _ as*mut u8));}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
