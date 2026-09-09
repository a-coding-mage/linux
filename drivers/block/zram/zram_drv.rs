#![allow(dead_code, unused_variables, unused_mut, non_camel_case_types)]

/*
 * Compressed RAM block device.  This is a direct low-level translation of
 * zram_drv.c; Linux kernel types and functions referenced by this unit are
 * supplied by the surrounding kernel bindings.
 */

use core::ffi::{c_char, c_int, c_void};

/* C headers and configuration-selected declarations are external kernel
 * dependencies and intentionally remain unresolved here. */
extern "C" {
    static mut zram_index_idr: c_void;
    static mut zram_index_mutex: c_void;
    static mut zram_major: c_int;
    static mut num_devices: u32;
    static mut huge_class_size: usize;
}

const ZRAM_MAX_ALGO_NAME_SZ: usize = 128;

#[repr(C)]
pub struct zram {
    pub table: *mut zram_table_entry,
    pub disksize: u64,
    pub limit_pages: usize,
    pub mem_pool: *mut c_void,
    pub disk: *mut c_void,
    pub stats: zram_stats,
    pub comps: *mut *mut c_void,
    pub comp_algs: *mut *const c_char,
    pub params: *mut zcomp_params,
    pub dev_lock: c_void,
    pub table_lock_map: c_void,
    pub table_lock_key: c_void,
    pub backing_dev: *mut c_void,
    pub bdev: *mut c_void,
    pub bitmap: *mut usize,
    pub nr_pages: usize,
    pub wb_batch_size: u32,
    pub bd_wb_limit: u64,
    pub wb_limit_enable: bool,
    pub compressed_wb: bool,
}
#[repr(C)] pub struct zram_table_entry { pub __lock: usize, pub handle: usize, pub attr: zram_attr }
#[repr(C)] pub struct zram_attr { pub flags: usize, pub ac_time: u32 }
#[repr(C)] pub struct zram_stats { pub max_used_pages: c_void, pub pages_stored: c_void, pub compr_data_size: c_void, pub same_pages: c_void, pub huge_pages: c_void, pub huge_pages_since: c_void, pub failed_reads: c_void, pub failed_writes: c_void, pub notify_free: c_void, pub miss_free: c_void, pub bd_count: c_void, pub bd_reads: c_void, pub bd_writes: c_void }
#[repr(C)] pub struct zcomp_params { pub dict: *mut c_void, pub level: i32, pub dict_sz: isize, pub deflate: deflate_params }
#[repr(C)] pub struct deflate_params { pub winbits: i32 }

#[repr(u32)] pub enum zram_pageflags { ZRAM_SAME=0, ZRAM_WB=1, ZRAM_HUGE=2, ZRAM_IDLE=3, ZRAM_PP_SLOT=4, ZRAM_INCOMPRESSIBLE=5 }
const ZRAM_FLAG_SHIFT: usize = 8;
const ZRAM_COMP_PRIORITY_BIT1: usize = 16;
const ZRAM_COMP_PRIORITY_MASK: usize = 0xff;

extern "C" {
    fn test_and_set_bit_lock(bit: usize, p: *mut usize) -> bool;
    fn clear_and_wake_up_bit(bit: usize, p: *mut usize);
    fn mutex_acquire(map: *mut c_void, subclass: c_int, trylock: c_int, ip: usize);
    fn mutex_release(map: *mut c_void, ip: usize);
    fn wait_on_bit_lock(p: *mut usize, bit: usize, state: c_int);
    fn zs_get_total_pages(pool: *mut c_void) -> usize;
    fn atomic_long_read(v: *const c_void) -> usize;
    fn atomic_long_try_cmpxchg(v: *mut c_void, old: *mut usize, new: usize) -> bool;
}

unsafe fn table(z: *mut zram, index: usize) -> *mut zram_table_entry { (*z).table.add(index) }
unsafe fn test_slot_flag(z: *mut zram, index: usize, flag: zram_pageflags) -> bool { ((*table(z,index)).attr.flags & (1usize << flag as u32)) != 0 }
unsafe fn set_slot_flag(z: *mut zram, index: usize, flag: zram_pageflags) { (*table(z,index)).attr.flags |= 1usize << flag as u32; }
unsafe fn clear_slot_flag(z: *mut zram, index: usize, flag: zram_pageflags) { (*table(z,index)).attr.flags &= !(1usize << flag as u32); }
unsafe fn get_slot_size(z: *mut zram, index: usize) -> usize { (*table(z,index)).attr.flags & ((1usize << ZRAM_FLAG_SHIFT)-1) }
unsafe fn set_slot_size(z: *mut zram, index: usize, size: usize) { let f=(*table(z,index)).attr.flags >> ZRAM_FLAG_SHIFT; (*table(z,index)).attr.flags=(f<<ZRAM_FLAG_SHIFT)|size; }
unsafe fn get_slot_handle(z: *mut zram, index: usize) -> usize { (*table(z,index)).handle }
unsafe fn set_slot_handle(z: *mut zram, index: usize, handle: usize) { (*table(z,index)).handle=handle; }
unsafe fn slot_allocated(z: *mut zram, index: usize) -> bool { get_slot_size(z,index)!=0 || test_slot_flag(z,index,zram_pageflags::ZRAM_SAME) || test_slot_flag(z,index,zram_pageflags::ZRAM_WB) }
unsafe fn set_slot_comp_priority(z: *mut zram,index:usize,mut prio:u32) { prio &= ZRAM_COMP_PRIORITY_MASK as u32; let p=table(z,index); (*p).attr.flags &= !(ZRAM_COMP_PRIORITY_MASK<<ZRAM_COMP_PRIORITY_BIT1); (*p).attr.flags |= (prio as usize)<<ZRAM_COMP_PRIORITY_BIT1; }
unsafe fn get_slot_comp_priority(z:*mut zram,index:usize)->u32 { (((*table(z,index)).attr.flags>>ZRAM_COMP_PRIORITY_BIT1)&ZRAM_COMP_PRIORITY_MASK) as u32 }

unsafe fn slot_trylock(z: *mut zram, index: usize) -> bool { let p=&mut (*table(z,index)).__lock as *mut usize; if !test_and_set_bit_lock(0,p) { mutex_acquire(&mut (*z).table_lock_map as *mut _ as *mut c_void,0,1,0); true } else { false } }
unsafe fn slot_lock(z:*mut zram,index:usize) { let p=&mut (*table(z,index)).__lock as *mut usize; mutex_acquire(&mut (*z).table_lock_map as *mut _ as *mut c_void,0,0,0); wait_on_bit_lock(p,0,0); }
unsafe fn slot_unlock(z:*mut zram,index:usize) { let p=&mut (*table(z,index)).__lock as *mut usize; mutex_release(&mut (*z).table_lock_map as *mut _ as *mut c_void,0); clear_and_wake_up_bit(0,p); }
unsafe fn init_done(z:*mut zram)->bool { (*z).disksize != 0 }
unsafe fn zram_can_store_page(z:*mut zram)->bool { let n=zs_get_total_pages((*z).mem_pool); n <= (*z).limit_pages || (*z).limit_pages==0 }

/* The remaining functions retain the C implementation's externally supplied
 * kernel operations and are declared here so their ABI and interfaces remain
 * visible to the translated unit. */
extern "C" {
    fn slot_free(z: *mut zram, index: usize);
    fn zram_read_page(z: *mut zram, page: *mut c_void, index: usize, parent: *mut c_void) -> c_int;
    fn zram_write_page(z: *mut zram, page: *mut c_void, index: usize) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
