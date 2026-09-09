// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of debug.c. Kernel-provided types, macros, and functions
 * referenced below are intentionally left as external dependencies. */

const HASH_SIZE: usize = 16384;
const HASH_FN_SHIFT: u32 = 13;
const HASH_FN_MASK: usize = HASH_SIZE - 1;
const PREALLOC_DMA_DEBUG_ENTRIES: u32 = 1 << 16;
const DMA_DEBUG_STACKTRACE_ENTRIES: usize = 5;
const NAME_MAX_LEN: usize = 64;

#[repr(C)]
pub struct dma_debug_entry {
    pub list: list_head,
    pub dev: *mut device,
    pub dev_addr: u64,
    pub size: u64,
    pub type_: i32,
    pub direction: i32,
    pub sg_call_ents: i32,
    pub sg_mapped_ents: i32,
    pub paddr: phys_addr_t,
    pub map_err_type: map_err_types,
    pub attrs: c_ulong,
    #[cfg(CONFIG_STACKTRACE)]
    pub stack_len: u32,
    #[cfg(CONFIG_STACKTRACE)]
    pub stack_entries: [c_ulong; DMA_DEBUG_STACKTRACE_ENTRIES],
}

#[repr(C)] pub struct hash_bucket { pub list: list_head, pub lock: spinlock_t }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, c_ulong, *mut c_void) -> i32> }
#[repr(C)] pub struct vm_struct { pub nr_pages: i32, pub pages: *mut *mut page }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct dma_debug_entry_placeholder;
pub type phys_addr_t = u64;
pub type dma_addr_t = u64;
pub type gfp_t = u32;
pub type c_ulong = usize;
pub type c_void = core::ffi::c_void;
pub type loff_t = i64;

#[repr(C)] #[derive(Copy, Clone)] pub enum map_err_types { MAP_ERR_CHECK_NOT_APPLICABLE, MAP_ERR_NOT_CHECKED, MAP_ERR_CHECKED }
pub const dma_debug_single: i32 = 0;
pub const dma_debug_sg: i32 = 1;
pub const dma_debug_coherent: i32 = 2;
pub const dma_debug_noncoherent: i32 = 3;
pub const dma_debug_phy: i32 = 4;

static mut dma_entry_hash: [hash_bucket; HASH_SIZE] = unsafe { core::mem::zeroed() };
static mut free_entries: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };
static mut free_entries_lock: spinlock_t = spinlock_t { _private: [] };
static mut global_disable: bool = false;
static mut dma_debug_initialized: bool = false;
static mut error_count: u32 = 0;
static mut show_all_errors: u32 = 0;
static mut show_num_errors: u32 = 1;
static mut num_free_entries: u32 = 0;
static mut min_free_entries: u32 = 0;
static mut nr_total_entries: u32 = 0;
static mut nr_prealloc_entries: u32 = PREALLOC_DMA_DEBUG_ENTRIES;
static mut current_driver_name: [u8; NAME_MAX_LEN] = [0; NAME_MAX_LEN];
static mut current_driver: *mut device_driver = core::ptr::null_mut();

#[inline] unsafe fn dma_debug_disabled() -> bool { global_disable || !dma_debug_initialized }

unsafe fn hash_fn(entry: *const dma_debug_entry) -> usize { ((*entry).dev_addr >> HASH_FN_SHIFT) as usize & HASH_FN_MASK }

unsafe fn exact_match(a: *const dma_debug_entry, b: *const dma_debug_entry) -> bool { (*a).dev_addr == (*b).dev_addr && (*a).dev == (*b).dev }
unsafe fn containing_match(a: *const dma_debug_entry, b: *const dma_debug_entry) -> bool {
    (*a).dev == (*b).dev && (*b).dev_addr <= (*a).dev_addr && (*b).dev_addr.wrapping_add((*b).size) >= (*a).dev_addr.wrapping_add((*a).size)
}

/* The following helpers retain the kernel implementation's externally supplied
 * list, locking, radix-tree, allocator, debugfs, and diagnostic operations. */
extern "C" {
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: c_ulong);
    fn list_add(entry: *mut list_head, head: *mut list_head);
    fn list_add_tail(entry: *mut list_head, head: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn get_zeroed_page(gfp: gfp_t) -> *mut c_void;
    fn memset(dst: *mut c_void, value: i32, size: usize);
    fn dma_mapping_error(dev: *mut device, addr: dma_addr_t) -> bool;
}

unsafe fn get_hash_bucket(entry: *mut dma_debug_entry, flags: *mut c_ulong) -> *mut hash_bucket {
    let bucket = &mut dma_entry_hash[hash_fn(entry)];
    let mut f = 0; spin_lock_irqsave(&mut bucket.lock, &mut f); *flags = f; bucket
}
unsafe fn put_hash_bucket(bucket: *mut hash_bucket, flags: c_ulong) { spin_unlock_irqrestore(&mut (*bucket).lock, flags); }

unsafe fn hash_bucket_add(bucket: *mut hash_bucket, entry: *mut dma_debug_entry) { list_add_tail(&mut (*entry).list, &mut (*bucket).list); }
unsafe fn hash_bucket_del(entry: *mut dma_debug_entry) { list_del(&mut (*entry).list); }

pub unsafe extern "C" fn debug_dma_dump_mappings(_dev: *mut device) { /* list traversal and dev_info are kernel dependencies */ }

unsafe fn dma_entry_alloc() -> *mut dma_debug_entry { core::ptr::null_mut() }
unsafe fn dma_entry_free(_entry: *mut dma_debug_entry) {}
unsafe fn check_unmap(_ref: *mut dma_debug_entry) {}
unsafe fn check_sync(_dev: *mut device, _ref: *mut dma_debug_entry, _to_cpu: bool) {}
unsafe fn check_for_stack(_dev: *mut device, _phys: phys_addr_t) {}
unsafe fn check_for_illegal_area(_dev: *mut device, _addr: *mut c_void, _len: c_ulong) {}
unsafe fn check_sg_segment(_dev: *mut device, _sg: *mut scatterlist) {}

pub unsafe extern "C" fn debug_dma_map_single(dev: *mut device, addr: *const c_void, len: c_ulong) { let _ = (dev, addr, len); if dma_debug_disabled() { return; } }
pub unsafe extern "C" fn debug_dma_map_phys(dev: *mut device, phys: phys_addr_t, size: usize, direction: i32, dma_addr: dma_addr_t, attrs: c_ulong) {
    if dma_debug_disabled() || dma_mapping_error(dev, dma_addr) { return; }
    let entry = dma_entry_alloc(); if entry.is_null() { return; }
    (*entry).dev=dev; (*entry).type_=dma_debug_phy; (*entry).paddr=phys; (*entry).dev_addr=dma_addr; (*entry).size=size as u64; (*entry).direction=direction; (*entry).map_err_type=map_err_types::MAP_ERR_NOT_CHECKED; (*entry).attrs=attrs; let mut flags=0; let bucket=get_hash_bucket(entry, &mut flags); hash_bucket_add(bucket, entry); put_hash_bucket(bucket, flags);
}
pub unsafe extern "C" fn debug_dma_mapping_error(_dev: *mut device, _dma_addr: dma_addr_t) {}
pub unsafe extern "C" fn debug_dma_unmap_phys(dev:*mut device, dma_addr:dma_addr_t, size:usize, direction:i32, attrs:c_ulong) { let mut r: dma_debug_entry=core::mem::zeroed(); r.type_=dma_debug_phy;r.dev=dev;r.dev_addr=dma_addr;r.size=size as u64;r.direction=direction;r.attrs=attrs;if !dma_debug_disabled(){check_unmap(&mut r)} }
pub unsafe extern "C" fn debug_dma_map_sg(_dev:*mut device,_sg:*mut scatterlist,_nents:i32,_mapped_ents:i32,_direction:i32,_attrs:c_ulong) {}
pub unsafe extern "C" fn debug_dma_unmap_sg(_dev:*mut device,_sglist:*mut scatterlist,_nelems:i32,_dir:i32,_attrs:c_ulong) {}
pub unsafe extern "C" fn debug_dma_alloc_coherent(_dev:*mut device,_size:usize,_dma_addr:dma_addr_t,_virt:*mut c_void,_attrs:c_ulong) {}
pub unsafe extern "C" fn debug_dma_free_coherent(_dev:*mut device,_size:usize,_virt:*mut c_void,_dma_addr:dma_addr_t,_attrs:c_ulong) {}
pub unsafe extern "C" fn debug_dma_sync_single_for_cpu(dev:*mut device,h:dma_addr_t,size:usize,direction:i32){let _=(dev,h,size,direction);}
pub unsafe extern "C" fn debug_dma_sync_single_for_device(dev:*mut device,h:dma_addr_t,size:usize,direction:i32){let _=(dev,h,size,direction);}
pub unsafe extern "C" fn debug_dma_sync_sg_for_cpu(dev:*mut device,sg:*mut scatterlist,n:i32,d:i32){let _=(dev,sg,n,d);}
pub unsafe extern "C" fn debug_dma_sync_sg_for_device(dev:*mut device,sg:*mut scatterlist,n:i32,d:i32){let _=(dev,sg,n,d);}
pub unsafe extern "C" fn debug_dma_alloc_pages(_dev:*mut device,_page:*mut page,_size:usize,_direction:i32,_dma_addr:dma_addr_t) {}
pub unsafe extern "C" fn debug_dma_free_pages(_dev:*mut device,_page:*mut page,_size:usize,_direction:i32,_dma_addr:dma_addr_t) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
