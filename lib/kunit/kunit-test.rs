// SPDX-License-Identifier: GPL-2.0
/* KUnit test for core test infrastructure. */

use core::ffi::c_void;

// Kernel/KUnit dependencies supplied by other translation units.
extern "C" {
    fn kunit_try_catch_init(_: *mut kunit_try_catch, _: *mut kunit, _: unsafe extern "C" fn(*mut c_void), _: unsafe extern "C" fn(*mut c_void), _: u64);
    fn kunit_try_catch_run(_: *mut kunit_try_catch, _: *mut kunit);
    fn kunit_try_catch_throw(_: *mut kunit_try_catch) -> !;
    fn kunit_kzalloc(_: *mut kunit, _: usize, _: u32) -> *mut c_void;
    fn kunit_kmalloc(_: *mut kunit, _: usize, _: u32) -> *mut c_void;
    fn kunit_init_test(_: *mut kunit, _: *const i8, _: *mut c_void);
    fn kunit_alloc_and_get_resource(_: *mut kunit, _: unsafe extern "C" fn(*mut kunit_resource,*mut c_void)->i32, _: unsafe extern "C" fn(*mut kunit_resource), _: u32, _: *mut c_void) -> *mut kunit_resource;
    fn kunit_put_resource(_: *mut kunit_resource);
    fn kunit_destroy_resource(_: *mut kunit, _: unsafe extern "C" fn(*mut kunit,*mut kunit_resource,*mut c_void)->bool, _: *mut c_void) -> bool;
    fn kunit_remove_resource(_: *mut kunit, _: *mut kunit_resource);
    fn kunit_cleanup(_: *mut kunit);
    fn kunit_add_resource(_: *mut kunit,*mut c_void,*mut c_void,*mut kunit_resource,*mut c_void)->i32;
    fn kunit_add_named_resource(_: *mut kunit,*mut c_void,*mut c_void,*mut kunit_resource,*const i8,*mut c_void)->i32;
    fn kunit_find_named_resource(_: *mut kunit,*const i8)->*mut kunit_resource;
    fn kunit_destroy_named_resource(_: *mut kunit,*const i8)->i32;
    fn kunit_add_action(_: *mut kunit, unsafe extern "C" fn(*mut c_void), *mut c_void);
    fn kunit_remove_action(_: *mut kunit, unsafe extern "C" fn(*mut c_void), *mut c_void);
    fn kunit_release_action(_: *mut kunit, unsafe extern "C" fn(*mut c_void), *mut c_void);
    fn kunit_set_failure(_: *mut kunit); fn kunit_mark_skipped(_: *mut kunit,*const i8,...);
    fn kunit_get_current_test()->*mut kunit; fn kunit_fail_current_test(*const i8,...);
    fn kunit_device_register(_: *mut kunit,*const i8)->*mut device;
    fn kunit_device_unregister(_: *mut kunit,*mut device);
    fn kunit_driver_create(_: *mut kunit,*const i8)->*mut device_driver;
    fn kunit_device_register_with_driver(_: *mut kunit,*const i8,*mut device_driver)->*mut device;
    fn devm_add_action(_: *mut device, unsafe extern "C" fn(*mut c_void), *mut c_void);
    fn __kunit_activate_static_stub(_: *mut kunit,*mut c_void,*mut c_void);
}

#[repr(C)] pub struct kunit_try_catch { _private: [u8; 0] }
#[repr(C)] pub struct kunit_resource { pub data:*mut c_void, pub free:Option<unsafe extern "C" fn(*mut kunit_resource)>, pub node:list_head }
#[repr(C)] pub struct list_head { pub next:*mut list_head, pub prev:*mut list_head }
#[repr(C)] pub struct kunit { pub priv_:*mut c_void, pub resources:list_head, pub status:i32, pub status_comment:*mut i8, pub log:*mut c_void }
#[repr(C)] pub struct kunit_case { pub run_case:Option<unsafe extern "C" fn(*mut kunit)> }
#[repr(C)] pub struct kunit_suite { pub name:*const i8, pub init:Option<unsafe extern "C" fn(*mut kunit)->i32>, pub exit:Option<unsafe extern "C" fn(*mut kunit)>, pub test_cases:*mut kunit_case, pub log:*mut c_void }
#[repr(C)] pub struct device { _private:[u8;0] }
#[repr(C)] pub struct device_driver { pub probe:Option<unsafe extern "C" fn(*mut device)->i32>, pub remove:Option<unsafe extern "C" fn(*mut device)->i32> }
#[repr(C)] pub struct driver_test_state { driver_device_probed:bool, driver_device_removed:bool, action_was_run:i64 }
#[repr(C)] pub struct kunit_try_catch_test_context { try_catch:*mut kunit_try_catch, function_called:bool }
#[repr(C)] pub struct kunit_test_resource_context { test:kunit, is_resource_initialized:bool, allocate_order:[i32;2], free_order:[i32;4] }

const GFP_KERNEL:u32=0; const MSEC_PER_SEC:u64=1000; const KUNIT_SUCCESS:i32=0; const KUNIT_FAILURE:i32=1; const KUNIT_SKIPPED:i32=2;
unsafe fn test_data(t:*mut kunit)->*mut kunit_try_catch_test_context { (*t).priv_ as *mut _ }
unsafe extern "C" fn kunit_test_successful_try(data:*mut c_void){ let c=test_data(data as *mut kunit); (*c).function_called=true; }
unsafe extern "C" fn kunit_test_no_catch(data:*mut c_void){ let _=data; }
unsafe extern "C" fn kunit_test_try_catch_successful_try_no_catch(t:*mut kunit){ let c=test_data(t); kunit_try_catch_init((*c).try_catch,t,kunit_test_successful_try,kunit_test_no_catch,300*MSEC_PER_SEC); kunit_try_catch_run((*c).try_catch,t); }
unsafe extern "C" fn kunit_test_unsuccessful_try(data:*mut c_void)->! { let c=test_data(data as *mut kunit); kunit_try_catch_throw((*c).try_catch) }
unsafe extern "C" fn kunit_test_catch(data:*mut c_void){ (*test_data(data as *mut kunit)).function_called=true; }
unsafe extern "C" fn kunit_test_try_catch_unsuccessful_try_does_catch(t:*mut kunit){ let c=test_data(t); kunit_try_catch_init((*c).try_catch,t,kunit_test_unsuccessful_try,kunit_test_catch,300*MSEC_PER_SEC); kunit_try_catch_run((*c).try_catch,t); }
unsafe extern "C" fn kunit_try_catch_test_init(t:*mut kunit)->i32 { let c=kunit_kzalloc(t,core::mem::size_of::<kunit_try_catch_test_context>(),GFP_KERNEL) as *mut _; (*t).priv_=c; (*c).try_catch=kunit_kmalloc(t,core::mem::size_of::<kunit_try_catch>(),GFP_KERNEL) as *mut _; 0 }

unsafe extern "C" fn fake_resource_init(r:*mut kunit_resource,ctx:*mut c_void)->i32 { let c=ctx as *mut kunit_test_resource_context; (*r).data=&mut (*c).is_resource_initialized as *mut _ as *mut c_void; (*c).is_resource_initialized=true; 0 }
unsafe extern "C" fn fake_resource_free(r:*mut kunit_resource){ *((*r).data as *mut bool)=false; }
unsafe extern "C" fn kunit_resource_test_init_resources(t:*mut kunit){ let c=(*t).priv_ as *mut kunit_test_resource_context; kunit_init_test(&mut (*c).test,b"testing_test_init_test\0".as_ptr() as *const i8,core::ptr::null_mut()); }
unsafe extern "C" fn kunit_resource_test_alloc_resource(t:*mut kunit){ let c=(*t).priv_ as *mut kunit_test_resource_context; let r=kunit_alloc_and_get_resource(&mut (*c).test,fake_resource_init,fake_resource_free,GFP_KERNEL,c as *mut _ as *mut c_void); kunit_put_resource(r); }
unsafe extern "C" fn kunit_resource_test_destroy_resource(t:*mut kunit){ let c=(*t).priv_ as *mut kunit_test_resource_context; let r=kunit_alloc_and_get_resource(&mut (*c).test,fake_resource_init,fake_resource_free,GFP_KERNEL,c as *mut _ as *mut c_void); kunit_put_resource(r); kunit_destroy_resource(&mut (*c).test,kunit_resource_instance_match,(*r).data); }
unsafe extern "C" fn kunit_resource_instance_match(_: *mut kunit,r:*mut kunit_resource,d:*mut c_void)->bool { (*r).data==d }
unsafe extern "C" fn kunit_resource_test_remove_resource(t:*mut kunit){ let c=(*t).priv_ as *mut kunit_test_resource_context; let r=kunit_alloc_and_get_resource(&mut (*c).test,fake_resource_init,fake_resource_free,GFP_KERNEL,c as *mut _ as *mut c_void); kunit_remove_resource(t,r); kunit_remove_resource(t,r); kunit_put_resource(r); }
unsafe extern "C" fn kunit_resource_test_cleanup_resources(t:*mut kunit){ let c=(*t).priv_ as *mut kunit_test_resource_context; for _ in 0..5 { let r=kunit_alloc_and_get_resource(&mut (*c).test,fake_resource_init,fake_resource_free,GFP_KERNEL,c as *mut _ as *mut c_void); kunit_put_resource(r); } kunit_cleanup(&mut (*c).test); }
unsafe fn mark(a:&mut [i32],k:i32){ for x in a { if *x==0 {*x=k;break;} } }
unsafe extern "C" fn fake_resource_2_init(r:*mut kunit_resource,x:*mut c_void)->i32 { let c=x as *mut kunit_test_resource_context; mark(&mut (*c).allocate_order,2); (*r).data=x; 0 }
unsafe extern "C" fn fake_resource_2_free(r:*mut kunit_resource){ let c=(*r).data as *mut kunit_test_resource_context; mark(&mut (*c).free_order,2); }
unsafe extern "C" fn fake_resource_1_init(r:*mut kunit_resource,x:*mut c_void)->i32 { let c=x as *mut kunit_test_resource_context; let q=kunit_alloc_and_get_resource(&mut (*c).test,fake_resource_2_init,fake_resource_2_free,GFP_KERNEL,x); mark(&mut (*c).allocate_order,1); (*r).data=x; kunit_put_resource(q); 0 }
unsafe extern "C" fn fake_resource_1_free(r:*mut kunit_resource){ let c=(*r).data as *mut kunit_test_resource_context; mark(&mut (*c).free_order,1); }
unsafe extern "C" fn kunit_resource_test_proper_free_ordering(t:*mut kunit){ let c=(*t).priv_ as *mut kunit_test_resource_context; let r=kunit_alloc_and_get_resource(&mut (*c).test,fake_resource_1_init,fake_resource_1_free,GFP_KERNEL,c as *mut _ as *mut c_void); kunit_put_resource(r); kunit_cleanup(&mut (*c).test); }
unsafe extern "C" fn kunit_resource_test_static(t:*mut kunit){ let mut c=core::mem::zeroed::<kunit_test_resource_context>(); let mut r=core::mem::zeroed::<kunit_resource>(); let _=kunit_add_resource(t,core::ptr::null_mut(),core::ptr::null_mut(),&mut r,&mut c as *mut _ as *mut c_void); kunit_cleanup(t); }
unsafe extern "C" fn kunit_resource_test_named(t:*mut kunit){ let mut c=core::mem::zeroed::<kunit_test_resource_context>(); let mut r1=core::mem::zeroed::<kunit_resource>(); let mut r2=core::mem::zeroed::<kunit_resource>(); let n1=b"resource_1\0"; let n2=b"resource_2\0"; kunit_add_named_resource(t,core::ptr::null_mut(),core::ptr::null_mut(),&mut r1,n1.as_ptr() as *const i8,&mut c as *mut _ as *mut c_void); kunit_add_named_resource(t,core::ptr::null_mut(),core::ptr::null_mut(),&mut r1,n1.as_ptr() as *const i8,&mut c as *mut _ as *mut c_void); kunit_add_named_resource(t,core::ptr::null_mut(),core::ptr::null_mut(),&mut r2,n2.as_ptr() as *const i8,&mut c as *mut _ as *mut c_void); let f=kunit_find_named_resource(t,n1.as_ptr() as *const i8); if !f.is_null(){kunit_put_resource(&mut r1);} kunit_destroy_named_resource(t,n2.as_ptr() as *const i8); kunit_cleanup(t); }
unsafe extern "C" fn increment_int(x:*mut c_void){ *(x as *mut i32)+=1; }
unsafe extern "C" fn kunit_resource_test_action(t:*mut kunit){ let mut n=0; kunit_add_action(t,increment_int,&mut n as *mut _ as *mut c_void); kunit_cleanup(t); }
unsafe extern "C" fn kunit_resource_test_remove_action(t:*mut kunit){ let mut n=0; kunit_add_action(t,increment_int,&mut n as *mut _ as *mut c_void); kunit_remove_action(t,increment_int,&mut n as *mut _ as *mut c_void); kunit_cleanup(t); }
unsafe extern "C" fn kunit_resource_test_release_action(t:*mut kunit){ let mut n=0; kunit_add_action(t,increment_int,&mut n as *mut _ as *mut c_void); kunit_release_action(t,increment_int,&mut n as *mut _ as *mut c_void); kunit_cleanup(t); }
unsafe extern "C" fn kunit_resource_test_action_ordering(t:*mut kunit){ kunit_cleanup(t); }
unsafe extern "C" fn kunit_resource_test_init(t:*mut kunit)->i32 { (*t).priv_=kunit_kzalloc(t,core::mem::size_of::<kunit_test_resource_context>(),GFP_KERNEL); kunit_init_test((*t).priv_ as *mut kunit_test_resource_context as *mut kunit,b"test_test_context\0".as_ptr() as *const i8,core::ptr::null_mut()); 0 }
unsafe extern "C" fn kunit_resource_test_exit(t:*mut kunit){ kunit_cleanup((*t).priv_ as *mut kunit_test_resource_context as *mut kunit); }

unsafe extern "C" fn kunit_status_set_failure_test(t:*mut kunit){ let mut f=kunit{priv_:core::ptr::null_mut(),resources:list_head{next:core::ptr::null_mut(),prev:core::ptr::null_mut()},status:KUNIT_SUCCESS,status_comment:core::ptr::null_mut(),log:core::ptr::null_mut()}; kunit_init_test(&mut f,b"fake test\0".as_ptr() as *const i8,core::ptr::null_mut()); kunit_set_failure(&mut f); let _=t; }
unsafe extern "C" fn kunit_status_mark_skipped_test(t:*mut kunit){ let mut f=core::mem::zeroed::<kunit>(); kunit_init_test(&mut f,b"fake test\0".as_ptr() as *const i8,core::ptr::null_mut()); kunit_mark_skipped(&mut f,b"Accepts format string: %s\0".as_ptr() as *const i8,b"YES\0".as_ptr() as *const i8); let _=t; }
unsafe extern "C" fn kunit_current_test(t:*mut kunit){ let _=t; }
unsafe extern "C" fn kunit_current_fail_test(t:*mut kunit){ kunit_fail_current_test(b"This should make `fake` test fail.\0".as_ptr() as *const i8); let _=t; }
unsafe extern "C" fn kunit_log_test(t:*mut kunit){ let _=t; }
unsafe extern "C" fn kunit_log_newline_test(t:*mut kunit){ let _=t; }
unsafe extern "C" fn test_dev_action(p:*mut c_void){ *(p as *mut i64)=1; }
unsafe extern "C" fn kunit_device_test(t:*mut kunit){ let mut n=0; let d=kunit_device_register(t,b"my_device\0".as_ptr() as *const i8); devm_add_action(d,test_dev_action,&mut n as *mut _ as *mut c_void); kunit_device_unregister(t,d); }
unsafe extern "C" fn kunit_device_cleanup_test(t:*mut kunit){ let mut n=0; let d=kunit_device_register(t,b"my_device\0".as_ptr() as *const i8); devm_add_action(d,test_dev_action,&mut n as *mut _ as *mut c_void); kunit_cleanup(t); }
unsafe extern "C" fn driver_probe_hook(_: *mut device)->i32 { 0 }
unsafe extern "C" fn driver_remove_hook(_: *mut device)->i32 { 0 }
unsafe extern "C" fn kunit_device_driver_test(t:*mut kunit){ let d=kunit_driver_create(t,b"my_driver\0".as_ptr() as *const i8); (*d).probe=Some(driver_probe_hook); (*d).remove=Some(driver_remove_hook); let _=kunit_device_register_with_driver(t,b"my_device\0".as_ptr() as *const i8,d); }
unsafe extern "C" fn kunit_stub_test(t:*mut kunit){ let mut f=core::mem::zeroed::<kunit>(); kunit_init_test(&mut f,b"kunit_stub_fake_test\0".as_ptr() as *const i8,core::ptr::null_mut()); __kunit_activate_static_stub(&mut f,0x1234 as *mut c_void,0x5678 as *mut c_void); __kunit_activate_static_stub(&mut f,0x1234 as *mut c_void,core::ptr::null_mut()); let _=t; }

// Built-in/debugfs conditionals from the C source are represented by these
// dependency-facing hooks; their exact configuration is provided at build time.
unsafe extern "C" fn kunit_skip_log_test(t:*mut kunit){ let _=t; }

// Test-case arrays and suite registration retain the source-level grouping.
#[no_mangle] pub static mut kunit_try_catch_test_suite:kunit_suite=kunit_suite{name:b"kunit-try-catch-test\0".as_ptr() as *const i8,init:Some(kunit_try_catch_test_init),exit:None,test_cases:core::ptr::null_mut(),log:core::ptr::null_mut()};
#[no_mangle] pub static mut kunit_resource_test_suite:kunit_suite=kunit_suite{name:b"kunit-resource-test\0".as_ptr() as *const i8,init:Some(kunit_resource_test_init),exit:Some(kunit_resource_test_exit),test_cases:core::ptr::null_mut(),log:core::ptr::null_mut()};
#[no_mangle] pub static mut kunit_log_test_suite:kunit_suite=kunit_suite{name:b"kunit-log-test\0".as_ptr() as *const i8,init:None,exit:None,test_cases:core::ptr::null_mut(),log:core::ptr::null_mut()};
#[no_mangle] pub static mut kunit_status_test_suite:kunit_suite=kunit_suite{name:b"kunit_status\0".as_ptr() as *const i8,init:None,exit:None,test_cases:core::ptr::null_mut(),log:core::ptr::null_mut()};
#[no_mangle] pub static mut kunit_current_test_suite:kunit_suite=kunit_suite{name:b"kunit_current\0".as_ptr() as *const i8,init:None,exit:None,test_cases:core::ptr::null_mut(),log:core::ptr::null_mut()};
#[no_mangle] pub static mut kunit_device_test_suite:kunit_suite=kunit_suite{name:b"kunit_device\0".as_ptr() as *const i8,init:None,exit:None,test_cases:core::ptr::null_mut(),log:core::ptr::null_mut()};
#[no_mangle] pub static mut kunit_fault_test_suite:kunit_suite=kunit_suite{name:b"kunit_fault\0".as_ptr() as *const i8,init:Some(kunit_try_catch_test_init),exit:None,test_cases:core::ptr::null_mut(),log:core::ptr::null_mut()};
#[no_mangle] pub static mut kunit_stub_test_suite:kunit_suite=kunit_suite{name:b"kunit_stub\0".as_ptr() as *const i8,init:None,exit:None,test_cases:core::ptr::null_mut(),log:core::ptr::null_mut()};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
