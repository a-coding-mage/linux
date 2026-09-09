// SPDX-License-Identifier: GPL-2.0-only
/*
 * Memory allocator for buffers shared with the TrustZone.
 *
 * Copyright (C) 2023-2024 Linaro Ltd.
 */

// Dependencies supplied by the kernel and Qualcomm support code are intentionally
// left as external symbols for integration with the surrounding repository.

#[repr(C)]
pub struct QcomTzmemArea {
    pub list: ListHead,
    pub vaddr: *mut core::ffi::c_void,
    pub paddr: DmaAddrT,
    pub size: usize,
    pub priv_: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct QcomTzmemPool {
    pub genpool: *mut GenPool,
    pub areas: ListHead,
    pub policy: QcomTzmemPolicy,
    pub increment: usize,
    pub max_size: usize,
    pub lock: SpinlockT,
}

#[repr(C)]
pub struct QcomTzmemChunk {
    pub size: usize,
    pub owner: *mut QcomTzmemPool,
}

static mut QCOM_TZMEM_DEV: *mut Device = core::ptr::null_mut();
static mut QCOM_TZMEM_USING_SHM_BRIDGE: bool = false;

#[cfg(feature = "qcom_tzmem_mode_generic")]
unsafe fn qcom_tzmem_init() -> i32 { 0 }

#[cfg(feature = "qcom_tzmem_mode_generic")]
unsafe fn qcom_tzmem_init_area(_area: *mut QcomTzmemArea) -> i32 { 0 }

#[cfg(feature = "qcom_tzmem_mode_generic")]
unsafe fn qcom_tzmem_cleanup_area(_area: *mut QcomTzmemArea) {}

#[cfg(feature = "qcom_tzmem_mode_shmbridge")]
const QCOM_SHM_BRIDGE_NUM_VM_SHIFT: u32 = 9;

#[cfg(feature = "qcom_tzmem_mode_shmbridge")]
static QCOM_TZMEM_BLACKLIST: &[&[u8]] = &[
    b"qcom,sc7180\0", b"qcom,sc8180x\0", b"qcom,sdm670\0",
    b"qcom,sdm845\0", b"qcom,sm7150\0", b"qcom,sm8150\0",
];

#[cfg(feature = "qcom_tzmem_mode_shmbridge")]
unsafe fn qcom_tzmem_init() -> i32 {
    for platform in QCOM_TZMEM_BLACKLIST {
        if of_machine_is_compatible(platform.as_ptr() as *const i8) { return 0; }
    }
    let ret = qcom_scm_shm_bridge_enable(QCOM_TZMEM_DEV);
    if ret == -95 { return 0; }
    if ret == 0 { QCOM_TZMEM_USING_SHM_BRIDGE = true; }
    ret
}

#[cfg(feature = "qcom_tzmem_mode_shmbridge")]
pub unsafe fn qcom_tzmem_shm_bridge_create(paddr: PhysAddrT, size: usize, handle: *mut u64) -> i32 {
    if !QCOM_TZMEM_USING_SHM_BRIDGE { return 0; }
    let pfn_and_ns_perm = paddr | QCOM_SCM_PERM_RW;
    let ipfn_and_s_perm = paddr | QCOM_SCM_PERM_RW;
    let size_and_flags = size as u64 | (1u64 << QCOM_SHM_BRIDGE_NUM_VM_SHIFT);
    let ret = qcom_scm_shm_bridge_create(pfn_and_ns_perm, ipfn_and_s_perm,
        size_and_flags, QCOM_SCM_VMID_HLOS, handle);
    if ret { return ret; }
    0
}

#[cfg(feature = "qcom_tzmem_mode_shmbridge")]
pub unsafe fn qcom_tzmem_shm_bridge_delete(handle: u64) {
    if QCOM_TZMEM_USING_SHM_BRIDGE { qcom_scm_shm_bridge_delete(handle); }
}

#[cfg(feature = "qcom_tzmem_mode_shmbridge")]
unsafe fn qcom_tzmem_init_area(area: *mut QcomTzmemArea) -> i32 {
    let handle = Box::into_raw(Box::new(0u64));
    let ret = qcom_tzmem_shm_bridge_create((*area).paddr, (*area).size, handle);
    if ret != 0 { drop(Box::from_raw(handle)); return ret; }
    (*area).priv_ = handle.cast();
    0
}

#[cfg(feature = "qcom_tzmem_mode_shmbridge")]
unsafe fn qcom_tzmem_cleanup_area(area: *mut QcomTzmemArea) {
    let handle = (*area).priv_ as *mut u64;
    qcom_tzmem_shm_bridge_delete(*handle);
    drop(Box::from_raw(handle));
}

unsafe fn qcom_tzmem_pool_add_memory(pool: *mut QcomTzmemPool, size: usize, gfp: GfpT) -> i32 {
    let area = Box::into_raw(Box::new(core::mem::zeroed::<QcomTzmemArea>()));
    (*area).size = page_align(size);
    (*area).vaddr = dma_alloc_coherent(QCOM_TZMEM_DEV, (*area).size, &mut (*area).paddr, gfp);
    if (*area).vaddr.is_null() { drop(Box::from_raw(area)); return -12; }
    let ret = qcom_tzmem_init_area(area);
    if ret != 0 { dma_free_coherent(QCOM_TZMEM_DEV, (*area).size, (*area).vaddr, (*area).paddr); drop(Box::from_raw(area)); return ret; }
    let ret = gen_pool_add_virt((*pool).genpool, (*area).vaddr as usize, (*area).paddr, size, -1);
    if ret != 0 { dma_free_coherent(QCOM_TZMEM_DEV, (*area).size, (*area).vaddr, (*area).paddr); drop(Box::from_raw(area)); return ret; }
    list_add_tail(&mut (*area).list, &mut (*pool).areas);
    core::mem::forget(Box::from_raw(area));
    0
}

pub unsafe fn qcom_tzmem_pool_new(config: *const QcomTzmemPoolConfig) -> *mut QcomTzmemPool {
    match (*config).policy {
        QcomTzmemPolicy::Static if (*config).initial_size == 0 => return err_ptr(-22),
        QcomTzmemPolicy::Multiplier if (*config).increment == 0 => return err_ptr(-22),
        QcomTzmemPolicy::Static | QcomTzmemPolicy::Multiplier | QcomTzmemPolicy::OnDemand => {},
        _ => return err_ptr(-22),
    }
    let pool = Box::into_raw(Box::new(core::mem::zeroed::<QcomTzmemPool>()));
    (*pool).genpool = gen_pool_create(PAGE_SHIFT, -1);
    if (*pool).genpool.is_null() { drop(Box::from_raw(pool)); return err_ptr(-12); }
    gen_pool_set_algo((*pool).genpool, gen_pool_best_fit, core::ptr::null_mut());
    (*pool).policy = (*config).policy; (*pool).increment = (*config).increment; (*pool).max_size = (*config).max_size;
    init_list_head(&mut (*pool).areas); spin_lock_init(&mut (*pool).lock);
    if (*config).initial_size != 0 && qcom_tzmem_pool_add_memory(pool, (*config).initial_size, GFP_KERNEL) != 0 { gen_pool_destroy((*pool).genpool); drop(Box::from_raw(pool)); return err_ptr(-12); }
    pool
}

pub unsafe fn qcom_tzmem_pool_free(pool: *mut QcomTzmemPool) { if pool.is_null() { return; } gen_pool_destroy((*pool).genpool); drop(Box::from_raw(pool)); }
unsafe fn devm_qcom_tzmem_pool_free(data: *mut core::ffi::c_void) { qcom_tzmem_pool_free(data as *mut QcomTzmemPool); }
pub unsafe fn devm_qcom_tzmem_pool_new(dev: *mut Device, config: *const QcomTzmemPoolConfig) -> *mut QcomTzmemPool { let pool = qcom_tzmem_pool_new(config); if is_err(pool) { return pool; } if devm_add_action_or_reset(dev, devm_qcom_tzmem_pool_free, pool.cast()) != 0 { return err_ptr(-12); } pool }

pub unsafe fn qcom_tzmem_alloc(pool: *mut QcomTzmemPool, mut size: usize, gfp: GfpT) -> *mut core::ffi::c_void { if size == 0 { return core::ptr::null_mut(); } size = page_align(size); let chunk = Box::into_raw(Box::new(QcomTzmemChunk { size, owner: pool })); let vaddr = gen_pool_alloc((*pool).genpool, size); if vaddr == 0 { drop(Box::from_raw(chunk)); return core::ptr::null_mut(); } if radix_tree_insert(vaddr, chunk) != 0 { gen_pool_free((*pool).genpool, vaddr, size); drop(Box::from_raw(chunk)); return core::ptr::null_mut(); } let _ = gfp; vaddr as *mut core::ffi::c_void }
pub unsafe fn qcom_tzmem_free(vaddr: *mut core::ffi::c_void) { let chunk = radix_tree_delete_item(vaddr as usize, core::ptr::null_mut()); if chunk.is_null() { return; } gen_pool_free((*(*chunk).owner).genpool, vaddr as usize, (*chunk).size); drop(Box::from_raw(chunk)); }
pub unsafe fn qcom_tzmem_to_phys(vaddr: *mut core::ffi::c_void) -> PhysAddrT { gen_pool_virt_to_phys(core::ptr::null_mut(), vaddr as usize) }
pub unsafe fn qcom_tzmem_enable(dev: *mut Device) -> i32 { static mut RESULT: i32 = 0; QCOM_TZMEM_DEV = dev; RESULT = qcom_tzmem_init(); RESULT }

// External kernel types, constants, and functions are provided by the translated dependencies.
extern "C" {
    static QCOM_SCM_PERM_RW: u64; static QCOM_SCM_VMID_HLOS: u32; static PAGE_SHIFT: u32; static GFP_KERNEL: GfpT;
    fn qcom_scm_shm_bridge_enable(dev: *mut Device) -> i32; fn qcom_scm_shm_bridge_create(a:u64,b:u64,c:u64,d:u32,h:*mut u64)->i32; fn qcom_scm_shm_bridge_delete(h:u64);
    fn of_machine_is_compatible(s:*const i8)->bool; fn dma_alloc_coherent(d:*mut Device,s:usize,p:*mut DmaAddrT,g:GfpT)->*mut core::ffi::c_void; fn dma_free_coherent(d:*mut Device,s:usize,v:*mut core::ffi::c_void,p:DmaAddrT);
    fn gen_pool_create(o:u32,n:i32)->*mut GenPool; fn gen_pool_set_algo(p:*mut GenPool,a:*const core::ffi::c_void,d:*mut core::ffi::c_void); fn gen_pool_best_fit()->(); fn gen_pool_add_virt(p:*mut GenPool,v:usize,pa:PhysAddrT,s:usize,n:i32)->i32; fn gen_pool_destroy(p:*mut GenPool); fn gen_pool_alloc(p:*mut GenPool,s:usize)->usize; fn gen_pool_free(p:*mut GenPool,v:usize,s:usize); fn gen_pool_virt_to_phys(p:*mut GenPool,v:usize)->PhysAddrT;
    fn radix_tree_insert(k:usize,c:*mut QcomTzmemChunk)->i32; fn radix_tree_delete_item(k:usize,x:*mut core::ffi::c_void)->*mut QcomTzmemChunk; fn devm_add_action_or_reset(d:*mut Device,f:unsafe fn(*mut core::ffi::c_void),p:*mut core::ffi::c_void)->i32;
}

pub type DmaAddrT=u64; pub type PhysAddrT=u64; pub type GfpT=u32; #[repr(C)] pub struct Device; #[repr(C)] pub struct GenPool; #[repr(C)] pub struct ListHead{pub next:*mut ListHead,pub prev:*mut ListHead;} #[repr(C)] pub struct SpinlockT; #[repr(C)] pub struct QcomTzmemPoolConfig{pub policy:QcomTzmemPolicy,pub initial_size:usize,pub increment:usize,pub max_size:usize;} #[repr(C)] #[derive(Copy,Clone)] pub enum QcomTzmemPolicy{Static,Multiplier,OnDemand}
fn page_align(x:usize)->usize{x}
fn err_ptr(x:i32)->*mut QcomTzmemPool{x as isize as *mut QcomTzmemPool} fn is_err(p:*mut QcomTzmemPool)->bool{(p as isize)<0}
extern "C"{fn init_list_head(l:*mut ListHead);fn list_add_tail(l:*mut ListHead,h:*mut ListHead);fn spin_lock_init(l:*mut SpinlockT);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
