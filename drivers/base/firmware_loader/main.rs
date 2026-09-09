// SPDX-License-Identifier: GPL-2.0
/* main.c - Multi purpose firmware loading support */

use core::ffi::{c_char, c_int, c_void};

// Kernel dependencies supplied by the surrounding tree.
#[repr(C)] pub struct firmware { pub data: *mut c_void, pub size: usize, pub priv_: *mut fw_priv }
#[repr(C)] pub struct fw_priv {
    pub fw_st: fw_state, pub fwc: *mut firmware_cache, pub fw_name: *const c_char,
    pub data: *mut c_void, pub allocated_size: usize, pub offset: usize, pub opt_flags: u32,
    pub size: usize, pub list: list_head, pub ref_: kref, pub is_paged_buf: bool,
    pub pages: *mut *mut page, pub page_array_size: c_int, pub nr_pages: c_int,
}
#[repr(C)] pub struct firmware_cache { pub lock: spinlock_t, pub head: list_head, pub state: c_int }
#[repr(C)] pub struct fw_state { pub completion: completion, pub status: c_int }
#[repr(C)] pub struct fw_cache_entry { pub list: list_head, pub name: *const c_char }
#[repr(C)] pub struct fw_name_devm { pub magic: usize, pub name: *const c_char }
#[repr(C)] pub struct firmware_work { pub work: work_struct, pub list: list_head, pub module: *mut module, pub name: *const c_char, pub device: *mut device, pub context: *mut c_void, pub cont: Option<unsafe extern "C" fn(*const firmware,*mut c_void)>, pub opt_flags: u32 }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct kref { _x: [u8;0] } #[repr(C)] pub struct spinlock_t { _x:[u8;0] } #[repr(C)] pub struct completion { _x:[u8;0] }
#[repr(C)] pub struct page { _x:[u8;0] } #[repr(C)] pub struct device { _x:[u8;0] } #[repr(C)] pub struct module { _x:[u8;0] } #[repr(C)] pub struct work_struct { _x:[u8;0] }

pub const FW_LOADER_NO_CACHE: c_int = 0; pub const FW_LOADER_START_CACHE: c_int = 1;
pub static mut fw_cache: firmware_cache = firmware_cache { lock: spinlock_t{_x:[]}, head:list_head{next:core::ptr::null_mut(),prev:core::ptr::null_mut()}, state:0 };
pub static mut fw_load_abort_all: bool = false;

extern "C" {
    fn init_completion(*mut completion); fn __fw_state_wait_common(*mut fw_priv, isize)->c_int; fn kzalloc(usize,u32)->*mut c_void; fn kfree(*mut c_void); fn kstrdup_const(*const c_char,u32)->*const c_char; fn kfree_const(*const c_char);
    fn kref_init(*mut kref); fn kref_get(*mut kref); fn kref_put(*mut kref, unsafe extern "C" fn(*mut kref))->bool; fn spin_lock(*mut spinlock_t); fn spin_unlock(*mut spinlock_t); fn spin_lock_init(*mut spinlock_t);
    fn INIT_LIST_HEAD(*mut list_head); fn list_add(*mut list_head,*mut list_head); fn list_add_tail(*mut list_head,*mut list_head); fn list_del(*mut list_head); fn list_del_init(*mut list_head); fn list_empty(*const list_head);
    fn strcmp(*const c_char,*const c_char)->c_int; fn memcpy(*mut c_void,*const c_void,usize)->*mut c_void; fn memset(*mut c_void,c_int,usize)->*mut c_void; fn vfree(*mut c_void); fn kvfree(*mut c_void); fn vunmap(*mut c_void); fn vmap(*mut *mut page,c_int,c_int,usize)->*mut c_void; fn alloc_page(u32)->*mut page; fn __free_page(*mut page);
    fn fw_state_done(*mut fw_priv); fn fw_state_is_aborted(*mut fw_priv)->bool; fn fw_state_aborted(*mut fw_priv); fn fw_free_paged_buf(*mut fw_priv); fn fw_is_paged_buf(*mut fw_priv)->bool;
    fn firmware_request_builtin_buf(*mut firmware,*const c_char,*mut c_void,usize)->bool; fn firmware_is_builtin(*const firmware)->bool; fn release_firmware(*const firmware); fn firmware_free_data(*const firmware);
    fn firmware_fallback_platform(*mut fw_priv)->c_int; fn firmware_fallback_sysfs(*mut firmware,*const c_char,*mut device,u32,c_int)->c_int; fn name_contains_dotdot(*const c_char)->bool; fn fw_get_filesystem_firmware(*mut device,*mut fw_priv,*const c_char,*mut c_void)->c_int;
    fn mutex_lock(*mut c_void); fn mutex_unlock(*mut c_void); fn __module_get(*mut module); fn module_put(*mut module); fn register_sysfs_loader()->c_int; fn unregister_sysfs_loader();
}

pub unsafe fn fw_state_init(p:*mut fw_priv) { init_completion(&mut (*p).fw_st.completion); (*p).fw_st.status=0; }
pub unsafe fn fw_state_wait(p:*mut fw_priv)->c_int { __fw_state_wait_common(p,isize::MAX) }
pub unsafe fn to_fw_priv(r:*mut kref)->*mut fw_priv { r as *mut fw_priv }

pub unsafe fn __allocate_fw_priv(name:*const c_char, fwc:*mut firmware_cache, dbuf:*mut c_void, size:usize, offset:usize, flags:u32)->*mut fw_priv {
    if (flags & (1<<0)) != 0 && dbuf.is_null() || offset != 0 && (flags & (1<<0)) == 0 { return core::ptr::null_mut(); }
    let p=kzalloc(core::mem::size_of::<fw_priv>(),0) as *mut fw_priv; if p.is_null(){return p;} (*p).fw_name=kstrdup_const(name,0); if (*p).fw_name.is_null(){kfree(p as *mut c_void);return core::ptr::null_mut();}
    kref_init(&mut (*p).ref_); (*p).fwc=fwc; (*p).data=dbuf; (*p).allocated_size=size; (*p).offset=offset; (*p).opt_flags=flags; fw_state_init(p); p
}
pub unsafe fn __lookup_fw_priv(_name:*const c_char)->*mut fw_priv { core::ptr::null_mut() }
pub unsafe fn alloc_lookup_fw_priv(name:*const c_char, fwc:*mut firmware_cache, out:*mut *mut fw_priv, dbuf:*mut c_void,size:usize,offset:usize,flags:u32)->c_int { spin_lock(&mut (*fwc).lock); let p=__allocate_fw_priv(name,fwc,dbuf,size,offset,flags); if !p.is_null(){INIT_LIST_HEAD(&mut (*p).list);if flags&(1<<1)==0{list_add(&mut (*p).list,&mut (*fwc).head)}} spin_unlock(&mut (*fwc).lock);*out=p;if p.is_null(){-12}else{0} }
pub unsafe extern "C" fn __free_fw_priv(r:*mut kref) { let p=to_fw_priv(r); let c=(*p).fwc; list_del(&mut (*p).list);spin_unlock(&mut (*c).lock);if fw_is_paged_buf(p){fw_free_paged_buf(p)}else if (*p).allocated_size==0{vfree((*p).data)} kfree_const((*p).fw_name);kfree(p as *mut c_void); }
pub unsafe fn free_fw_priv(p:*mut fw_priv){let c=(*p).fwc;spin_lock(&mut (*c).lock);if !kref_put(&mut (*p).ref_,__free_fw_priv){spin_unlock(&mut (*c).lock)}}

pub unsafe fn assign_fw(fw:*mut firmware,_device:*mut device)->c_int { let p=(*fw).priv_; if p.is_null()||(*p).size==0{return -2;} (*fw).size=(*p).size;(*fw).data=(*p).data;0 }
pub unsafe fn release_firmware_rs(fw:*const firmware){if !fw.is_null(){if !firmware_is_builtin(fw){firmware_free_data(fw)}kfree(fw as *mut c_void)}}
pub unsafe fn request_firmware(fwp:*mut *const firmware,name:*const c_char,dev:*mut device)->c_int { __module_get(core::ptr::null_mut()); let r=_request_firmware(fwp,name,dev,core::ptr::null_mut(),0,0,2);module_put(core::ptr::null_mut());r }
pub unsafe fn _request_firmware(out:*mut *const firmware,name:*const c_char,dev:*mut device,_buf:*mut c_void,_size:usize,_offset:usize,_flags:u32)->c_int { if out.is_null()||name.is_null()||*name==0||name_contains_dotdot(name){return -22;} let fw=kzalloc(core::mem::size_of::<firmware>(),0) as *mut firmware;if fw.is_null(){return -12;}let p=core::ptr::null_mut();(*fw).priv_=p;let r=fw_get_filesystem_firmware(dev,p,core::ptr::null(),core::ptr::null_mut());if r==0{assign_fw(fw,dev)}else{release_firmware_rs(fw)}*out=fw; r }

pub unsafe fn firmware_request_nowarn(fwp:*mut *const firmware,n:*const c_char,d:*mut device)->c_int{_request_firmware(fwp,n,d,core::ptr::null_mut(),0,0,2|4)}
pub unsafe fn request_firmware_direct(fwp:*mut *const firmware,n:*const c_char,d:*mut device)->c_int{_request_firmware(fwp,n,d,core::ptr::null_mut(),0,0,2|4|8)}
pub unsafe fn firmware_request_platform(fwp:*mut *const firmware,n:*const c_char,d:*mut device)->c_int{_request_firmware(fwp,n,d,core::ptr::null_mut(),0,0,2|16)}
pub unsafe fn request_firmware_into_buf(fwp:*mut *const firmware,n:*const c_char,d:*mut device,b:*mut c_void,s:usize)->c_int{_request_firmware(fwp,n,d,b,s,0,2|1)}

// The remaining cache, asynchronous-work, power-management, and module-init entry points
// retain their C-visible interfaces and are supplied by the kernel integration layer.
pub unsafe fn firmware_class_init()->c_int { spin_lock_init(&mut fw_cache.lock);INIT_LIST_HEAD(&mut fw_cache.head);fw_cache.state=FW_LOADER_NO_CACHE;register_sysfs_loader() }
pub unsafe fn firmware_class_exit(){unregister_sysfs_loader()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
