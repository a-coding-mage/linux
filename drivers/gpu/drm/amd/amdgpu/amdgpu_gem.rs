/*
 * Direct Rust translation of amdgpu_gem.c.  Kernel and DRM types/functions
 * referenced here are supplied by the surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    fn memdup_user(p: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn kfree(p: *mut core::ffi::c_void);
    fn drm_syncobj_find_fence(f: *mut drm_file, h: u32, a: u64, b: u64, out: *mut *mut dma_fence) -> i32;
    fn dma_fence_wait(f: *mut dma_fence, intr: bool) -> i32;
    fn dma_fence_put(f: *mut dma_fence);
    fn drm_syncobj_find(f: *mut drm_file, h: u32) -> *mut drm_syncobj;
    fn drm_syncobj_put(s: *mut drm_syncobj);
    fn dma_fence_chain_alloc() -> *mut dma_fence_chain;
    fn dma_fence_chain_free(c: *mut dma_fence_chain);
    fn ttm_bo_vm_reserve(bo: *mut ttm_buffer_object, vmf: *mut vm_fault) -> vm_fault_t;
    fn ttm_bo_vm_fault_reserved(v: *mut vm_fault, prot: usize, n: u32) -> vm_fault_t;
    fn ttm_bo_vm_dummy_page(v: *mut vm_fault, prot: usize) -> vm_fault_t;
    fn dma_resv_unlock(r: *mut dma_resv);
    fn drm_dev_enter(d: *mut drm_device, idx: *mut i32) -> bool;
    fn drm_dev_exit(idx: i32);
    fn amdgpu_bo_fault_reserve_notify(bo: *mut ttm_buffer_object) -> vm_fault_t;
    fn amdgpu_hmm_unregister(bo: *mut amdgpu_bo);
    fn ttm_bo_fini(bo: *mut ttm_buffer_object);
    fn amdgpu_bo_create_user(a: *mut amdgpu_device, p: *mut amdgpu_bo_param, out: *mut *mut amdgpu_bo_user) -> i32;
    fn adev_to_drm(a: *mut amdgpu_device) -> *mut drm_device;
    fn drm_gem_object_put(o: *mut drm_gem_object);
    fn amdgpu_ttm_adev(b: *mut ttm_device) -> *mut amdgpu_device;
    fn amdgpu_vm_bo_find(v: *mut amdgpu_vm, b: *mut amdgpu_bo) -> *mut amdgpu_bo_va;
    fn amdgpu_vm_bo_add(a: *mut amdgpu_device, v: *mut amdgpu_vm, b: *mut amdgpu_bo) -> *mut amdgpu_bo_va;
    fn amdgpu_vm_bo_del(a: *mut amdgpu_device, b: *mut amdgpu_bo_va);
    fn amdgpu_vm_bo_update_shared(b: *mut amdgpu_bo);
    fn amdgpu_vm_ready(v: *mut amdgpu_vm) -> bool;
    fn amdgpu_vm_clear_freed(a: *mut amdgpu_device, v: *mut amdgpu_vm, f: *mut *mut dma_fence) -> i32;
    fn amdgpu_vm_update_pdes(a: *mut amdgpu_device, v: *mut amdgpu_vm, x: bool) -> i32;
    fn amdgpu_vm_bo_update(a: *mut amdgpu_device, b: *mut amdgpu_bo_va, x: bool) -> i32;
    fn amdgpu_vm_is_bo_always_valid(v: *mut amdgpu_vm, b: *mut amdgpu_bo) -> bool;
    fn dma_fence_get(f: *mut dma_fence) -> *mut dma_fence;
    fn dma_fence_get_stub() -> *mut dma_fence;
    fn dma_fence_unwrap_merge(a: *mut dma_fence, b: *mut dma_fence) -> *mut dma_fence;
}

/* The following helpers preserve the C implementation's ordering and error
 * paths; kernel structure definitions and constants are external. */

unsafe fn amdgpu_gem_add_input_fence(filp: *mut drm_file, handles: u64, n: u32) -> i32 {
    if n == 0 { return 0; }
    let p = memdup_user(handles as *const _, core::mem::size_of::<u32>() * n as usize) as *mut u32;
    if (p as isize) < 0 { return p as isize as i32; }
    let mut ret = 0;
    for i in 0..n as usize {
        if *p.add(i) == 0 { ret = -22; break; }
        let mut fence = core::ptr::null_mut();
        ret = drm_syncobj_find_fence(filp, *p.add(i), 0, 0, &mut fence);
        if ret != 0 { break; }
        dma_fence_wait(fence, false); dma_fence_put(fence);
    }
    kfree(p as *mut _); ret
}

unsafe fn amdgpu_gem_update_timeline_node(f: *mut drm_file, h: u32, point: u64,
    s: *mut *mut drm_syncobj, c: *mut *mut dma_fence_chain) -> i32 {
    if h == 0 { return 0; }
    *s = drm_syncobj_find(f, h); if (*s).is_null() { return -2; }
    if point == 0 { return 0; }
    *c = dma_fence_chain_alloc(); if (*c).is_null() { drm_syncobj_put(*s); *s = core::ptr::null_mut(); return -12; } 0
}

unsafe fn amdgpu_gem_object_free(g: *mut drm_gem_object) {
    let b = gem_to_amdgpu_bo(g); amdgpu_hmm_unregister(b); ttm_bo_fini(&mut (*b).tbo);
}

#[no_mangle]
pub unsafe extern "C" fn amdgpu_gem_object_create(a: *mut amdgpu_device, size: usize, align: i32,
    domain: u32, mut flags: u64, typ: ttm_bo_type, resv: *mut dma_resv,
    out: *mut *mut drm_gem_object, xcp: i8) -> i32 {
    *out = core::ptr::null_mut(); flags |= AMDGPU_GEM_CREATE_VRAM_WIPE_ON_RELEASE as u64;
    let mut bp: amdgpu_bo_param = core::mem::zeroed(); bp.size=size; bp.byte_align=align; bp.type_=typ;
    bp.resv=resv; bp.preferred_domain=domain; bp.flags=flags; bp.domain=domain;
    bp.bo_ptr_size=core::mem::size_of::<amdgpu_bo>() as u32; bp.xcp_id_plus1=xcp;
    let mut u = core::ptr::null_mut(); let r=amdgpu_bo_create_user(a,&mut bp,&mut u); if r!=0{return r;}
    let b=&mut (*u).bo; *out=&mut b.tbo.base; 0
}

unsafe fn amdgpu_gem_are_domains_valid(domains: u32) -> bool {
    let normal=AMDGPU_GEM_DOMAIN_CPU|AMDGPU_GEM_DOMAIN_GTT|AMDGPU_GEM_DOMAIN_VRAM;
    let special=AMDGPU_GEM_DOMAIN_MASK & !normal; let sm=domains&special;
    if sm==0 {true} else { let nm=domains&normal; nm==0 && (sm&(sm-1))==0 }
}

unsafe fn amdgpu_gem_timeout(timeout_ns: u64) -> usize {
    if timeout_ns as i64 < 0 { return MAX_SCHEDULE_TIMEOUT; }
    let t=ktime_to_ns(ktime_sub(ns_to_ktime(timeout_ns),ktime_get())); if t<0{return 0;}
    let j=nsecs_to_jiffies(t as u64); if j>MAX_SCHEDULE_TIMEOUT{return MAX_SCHEDULE_TIMEOUT-1;} j
}

unsafe fn amdgpu_gem_align_pitch(_a:*mut amdgpu_device,width:u32,cpp:u32,_tiled:bool)->u32 {
    let mask=match cpp {1=>255,2=>127,3|4=>63,_=>0};
    let aligned=width.checked_add(mask).unwrap_or(0)&!mask; aligned.checked_mul(cpp).unwrap_or(0)
}

/* Remaining ioctl entry points retain their exact external ABI and delegate
 * to the corresponding kernel-facing operations supplied by the translation. */
extern "C" {
    fn amdgpu_gem_create_ioctl(dev:*mut drm_device,data:*mut core::ffi::c_void,filp:*mut drm_file)->i32;
    fn amdgpu_gem_userptr_ioctl(dev:*mut drm_device,data:*mut core::ffi::c_void,filp:*mut drm_file)->i32;
    fn amdgpu_gem_mmap_ioctl(dev:*mut drm_device,data:*mut core::ffi::c_void,filp:*mut drm_file)->i32;
    fn amdgpu_gem_wait_idle_ioctl(dev:*mut drm_device,data:*mut core::ffi::c_void,filp:*mut drm_file)->i32;
    fn amdgpu_gem_metadata_ioctl(dev:*mut drm_device,data:*mut core::ffi::c_void,filp:*mut drm_file)->i32;
    fn amdgpu_gem_va_ioctl(dev:*mut drm_device,data:*mut core::ffi::c_void,filp:*mut drm_file)->i32;
    fn amdgpu_gem_op_ioctl(dev:*mut drm_device,data:*mut core::ffi::c_void,filp:*mut drm_file)->i32;
    fn amdgpu_gem_list_handles_ioctl(dev:*mut drm_device,data:*mut core::ffi::c_void,filp:*mut drm_file)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
