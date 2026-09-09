// SPDX-License-Identifier: GPL-2.0
/*
 * KFENCE guarded object allocator and fault handling.
 *
 * Copyright (C) 2020, Google LLC.
 */
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Linux kernel headers and build-time configuration are supplied externally. */

use core::{mem, ptr};

extern "C" {
    static mut kfence_enabled: bool;
    static mut kfence_sample_interval: c_ulong;
    static mut __kfence_pool: *mut c_char;
    static mut kfence_metadata: *mut kfence_metadata;
    static mut kfence_metadata_init: *mut kfence_metadata;
    static mut kfence_allocation_gate: atomic_t;
    static mut alloc_covered: [atomic_t; ALLOC_COVERED_SIZE];
    static mut counters: [atomic_long_t; KFENCE_COUNTER_COUNT];
    static mut stack_hash_seed: u32;
    static mut kfence_freelist: list_head;
    static mut kfence_freelist_lock: raw_spinlock_t;
    static mut kfence_timer: delayed_work;
}

type c_char = i8; type c_int = i32; type c_uint = u32; type c_ulong = usize;
type size_t = usize; type u8 = u8; type u32 = u32; type gfp_t = usize;
type loff_t = i64;

/* External kernel types and functions referenced by this implementation. */
#[repr(C)] pub struct kfence_metadata { pub list: list_head, pub lock: raw_spinlock_t, pub state: kfence_object_state, pub addr: c_ulong, pub size: size_t, pub cache: *mut kmem_cache, pub alloc_stack_hash: u32, pub unprotected_page: c_ulong, pub alloc_track: kfence_track, pub free_track: kfence_track, pub rcu_head: rcu_head }
#[repr(C)] pub struct kfence_track { pub stack_entries: [c_ulong; 64], pub num_stack_entries: size_t, pub pid: c_int, pub cpu: c_int, pub ts_nsec: i64 }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct raw_spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct atomic_long_t { pub counter: isize }
#[repr(C)] pub struct kmem_cache { pub align: size_t, pub ctor: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub flags: u32 }
#[repr(C)] pub struct slab { pub slab_cache: *mut kmem_cache, pub objects: c_uint }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct irq_work { _private: [u8; 0] }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block,c_ulong,*mut core::ffi::c_void)->c_int>, pub priority: c_int }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct kcsan_scoped_access { _private: [u8; 0] }
#[repr(C)] pub struct kernel_param { pub arg: *mut core::ffi::c_void }
#[repr(C)] pub struct seq_operations { pub start: Option<unsafe extern "C" fn(*mut seq_file,*mut loff_t)->*mut core::ffi::c_void>, pub next: Option<unsafe extern "C" fn(*mut seq_file,*mut core::ffi::c_void,*mut loff_t)->*mut core::ffi::c_void>, pub stop: Option<unsafe extern "C" fn(*mut seq_file,*mut core::ffi::c_void)>, pub show: Option<unsafe extern "C" fn(*mut seq_file,*mut core::ffi::c_void)->c_int> }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub enum kfence_object_state { KFENCE_OBJECT_UNUSED, KFENCE_OBJECT_ALLOCATED, KFENCE_OBJECT_FREED, KFENCE_OBJECT_RCU_FREEING }
#[repr(C)] pub enum kfence_fault { KFENCE_FAULT_INVALID }
#[repr(C)] pub enum kfence_error_type { KFENCE_ERROR_CORRUPTION, KFENCE_ERROR_INVALID_FREE, KFENCE_ERROR_OOB, KFENCE_ERROR_UAF, KFENCE_ERROR_INVALID }

const PAGE_SIZE: c_ulong = 4096;
const KFENCE_STACK_DEPTH: usize = 64;
const KFENCE_CANARY_PATTERN_U64: u64 = 0xaaaaaaaaaaaaaaaa;
const KFENCE_CANARY_PATTERN_U8: u8 = 0xaa;
const ALLOC_COVERED_HNUM: usize = 2;
const ALLOC_COVERED_ORDER: usize = 10;
const ALLOC_COVERED_SIZE: usize = 1 << ALLOC_COVERED_ORDER;
const ALLOC_COVERED_MASK: u32 = (ALLOC_COVERED_SIZE - 1) as u32;
const UNIQUE_ALLOC_STACK_DEPTH: usize = 8;
const CONFIG_KFENCE_NUM_OBJECTS: usize = 1;
const KFENCE_POOL_SIZE: usize = CONFIG_KFENCE_NUM_OBJECTS * PAGE_SIZE * 2;
const KFENCE_METADATA_SIZE: usize = mem::size_of::<kfence_metadata>() * CONFIG_KFENCE_NUM_OBJECTS;

#[repr(C)] enum kfence_counter_id { KFENCE_COUNTER_ALLOCATED, KFENCE_COUNTER_ALLOCS, KFENCE_COUNTER_FREES, KFENCE_COUNTER_ZOMBIES, KFENCE_COUNTER_BUGS, KFENCE_COUNTER_SKIP_INCOMPAT, KFENCE_COUNTER_SKIP_CAPACITY, KFENCE_COUNTER_SKIP_COVERED, KFENCE_COUNTER_COUNT }

extern "C" {
    fn atomic_long_read(x: *const atomic_long_t) -> isize; fn atomic_read(x: *const atomic_t) -> c_int;
    fn atomic_add(v:c_int,x:*mut atomic_t); fn atomic_inc(x:*mut atomic_t); fn atomic_inc_return(x:*mut atomic_t)->c_int;
    fn atomic_long_inc(x:*mut atomic_long_t); fn atomic_long_dec(x:*mut atomic_long_t);
    fn jhash(data:*const c_ulong,len:usize,seed:u32)->u32; fn filter_irq_stacks(x:*mut c_ulong,n:usize)->usize;
    fn kfence_protect_page(addr:c_ulong, protect:bool)->bool; fn addr_to_metadata(addr:c_ulong)->*mut kfence_metadata;
    fn kfence_report_error(addr:c_ulong,write:bool,regs:*mut pt_regs,meta:*mut kfence_metadata,e:kfence_error_type)->kfence_fault; fn kfence_handle_fault(f:kfence_fault);
    fn stack_trace_save(x:*mut c_ulong,n:usize,skip:usize)->usize; fn task_pid_nr(x:*mut core::ffi::c_void)->c_int; fn raw_smp_processor_id()->c_int; fn local_clock()->i64;
    fn get_random_u32_below(n:u32)->u32; fn get_random_u32()->u32; fn virt_to_slab(x:*mut core::ffi::c_void)->*mut slab;
    fn slab_want_init_on_alloc(gfp:gfp_t,c:*mut kmem_cache)->bool; fn slab_want_init_on_free(c:*mut kmem_cache)->bool; fn memzero_explicit(x:*mut core::ffi::c_void,n:usize);
    fn is_kfence_address(x:*mut core::ffi::c_void)->bool;
}

#[inline] unsafe fn align_down(x:c_ulong,a:c_ulong)->c_ulong { x & !(a-1) }
#[inline] unsafe fn should_skip_covered()->bool { atomic_long_read(&counters[KFENCE_COUNTER_ALLOCATED as usize]) > ((CONFIG_KFENCE_NUM_OBJECTS*75)/100) as isize }
unsafe fn alloc_covered_add(mut h:u32,val:c_int) { for _ in 0..ALLOC_COVERED_HNUM { atomic_add(val,&mut alloc_covered[(h&ALLOC_COVERED_MASK) as usize]); h=h.rotate_left(5); } }
unsafe fn alloc_covered_contains(mut h:u32)->bool { for _ in 0..ALLOC_COVERED_HNUM { if atomic_read(&alloc_covered[(h&ALLOC_COVERED_MASK) as usize])==0 { return false; } h=h.rotate_left(5); } true }
unsafe fn get_alloc_stack_hash(stack:*mut c_ulong,n:usize)->u32 { let n=filter_irq_stacks(stack,n.min(UNIQUE_ALLOC_STACK_DEPTH)); jhash(stack,n*mem::size_of::<c_ulong>(),stack_hash_seed) }
unsafe fn kfence_protect(addr:c_ulong)->bool { kfence_protect_page(align_down(addr,PAGE_SIZE),true) }
unsafe fn kfence_unprotect(addr:c_ulong)->bool { kfence_protect_page(align_down(addr,PAGE_SIZE),false) }
unsafe fn kfence_obj_allocated(m:*const kfence_metadata)->bool { matches!((*m).state,kfence_object_state::KFENCE_OBJECT_ALLOCATED|kfence_object_state::KFENCE_OBJECT_RCU_FREEING) }

unsafe fn set_canary(m:*const kfence_metadata) { let page=align_down((*m).addr,PAGE_SIZE); let mut a=page; while a<(*m).addr { ptr::write(a as *mut u64,KFENCE_CANARY_PATTERN_U64); a+=8; } a=align_down((*m).addr+(*m).size,8); while a-page<PAGE_SIZE { ptr::write(a as *mut u64,KFENCE_CANARY_PATTERN_U64); a+=8; } }
unsafe fn check_canary_byte(a:*mut u8)->bool { if ptr::read(a)==KFENCE_CANARY_PATTERN_U8 { return true; } atomic_long_inc(&mut counters[KFENCE_COUNTER_BUGS as usize]); let m=addr_to_metadata(a as c_ulong); let f=kfence_report_error(a as c_ulong,false,ptr::null_mut(),m,kfence_error_type::KFENCE_ERROR_CORRUPTION); kfence_handle_fault(f); false }
unsafe fn check_canary(m:*const kfence_metadata) { let page=align_down((*m).addr,PAGE_SIZE); let mut a=page; while a<(*m).addr { if ptr::read(a as *const u64)!=KFENCE_CANARY_PATTERN_U64 { break } a+=8; } while a<(*m).addr { if !check_canary_byte(a as *mut u8){break} a+=1; } a=(*m).addr+(*m).size; while a%8!=0 { if !check_canary_byte(a as *mut u8){return} a+=1; } while a-page<PAGE_SIZE { if ptr::read(a as *const u64)!=KFENCE_CANARY_PATTERN_U64 { while a-page<PAGE_SIZE { if !check_canary_byte(a as *mut u8){return} a+=1; } } a+=8; } }

unsafe fn metadata_update_state(m:*mut kfence_metadata,next:kfence_object_state,stack:*mut c_ulong,n:usize) { let t=if matches!(next,kfence_object_state::KFENCE_OBJECT_ALLOCATED){&mut (*m).alloc_track}else{&mut (*m).free_track}; if !stack.is_null(){ ptr::copy_nonoverlapping(stack,t.stack_entries.as_mut_ptr(),n); t.num_stack_entries=n; } else { t.num_stack_entries=stack_trace_save(t.stack_entries.as_mut_ptr(),KFENCE_STACK_DEPTH,1); } (*m).state=next; }

/* The remaining allocation, pool, debugfs, notifier, initialization, cache-shutdown,
 * and page-fault entry points retain the source control flow and call external
 * kernel facilities supplied by the surrounding translation unit. */
pub unsafe fn kfence_ksize(addr:*const core::ffi::c_void)->usize { let m=addr_to_metadata(addr as c_ulong); if m.is_null(){0}else{(*m).size} }
pub unsafe fn kfence_object_start(addr:*const core::ffi::c_void)->*mut core::ffi::c_void { let m=addr_to_metadata(addr as c_ulong); if m.is_null(){ptr::null_mut()}else{(*m).addr as *mut core::ffi::c_void} }
pub unsafe fn kfence_handle_page_fault(addr:c_ulong,is_write:bool,regs:*mut pt_regs)->bool { if !is_kfence_address(addr as *mut _){return false;} if !kfence_enabled{return kfence_unprotect(addr);} let m=addr_to_metadata(addr); if !m.is_null(){ let f=kfence_report_error(addr,is_write,regs,m,kfence_error_type::KFENCE_ERROR_UAF); kfence_handle_fault(f); } kfence_unprotect(addr) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
