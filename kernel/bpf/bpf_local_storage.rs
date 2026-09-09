// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2019 Facebook */
// Linux/BPF header dependencies are supplied by the surrounding translation unit.

pub const BPF_LOCAL_STORAGE_CREATE_FLAG_MASK: u64 = BPF_F_NO_PREALLOC | BPF_F_CLONE;

extern "C" {
    static mut BPF_F_NO_PREALLOC: u64;
    static mut BPF_F_CLONE: u64;
    static mut BPF_F_LOCK: u64;
    static mut BPF_NOEXIST: u64;
    static mut BPF_EXIST: u64;
    static mut BPF_SPIN_LOCK: u32;
    static mut BPF_LOCAL_STORAGE_MAX_VALUE_SIZE: u32;
    static mut SELEM_UNLINKED: i32;
    static mut SELEM_TOFREE: i32;
    static mut SELEM_MAP_UNLINKED: i32;
    static mut SELEM_STORAGE_UNLINKED: i32;

    fn hash_ptr(p: *const core::ffi::c_void, bits: u32) -> usize;
    fn bpf_map_kmalloc_nolock(map: *mut bpf_map, size: usize, flags: u32, node: i32) -> *mut bpf_local_storage_elem;
    fn bpf_map_area_alloc(size: usize, node: i32) -> *mut bpf_local_storage_map;
    fn bpf_map_area_free(p: *mut bpf_local_storage_map);
    fn bpf_map_kvcalloc(map: *mut bpf_map, n: usize, size: usize, flags: u32) -> *mut bpf_local_storage_map_bucket;
    fn kvfree(p: *mut core::ffi::c_void);
    fn kfree(p: *mut core::ffi::c_void);
    fn kfree_rcu(p: *mut core::ffi::c_void, field: *mut rcu_head);
    fn call_rcu_tasks_trace(h: *mut rcu_head, f: unsafe extern "C" fn(*mut rcu_head));
    fn rcu_barrier_tasks_trace(); fn rcu_barrier(); fn synchronize_rcu();
    fn bpf_obj_free_fields(record: *mut core::ffi::c_void, data: *mut u8);
    fn bpf_obj_swap_uptrs(record: *mut core::ffi::c_void, data: *mut u8, value: *mut core::ffi::c_void);
    fn copy_map_value(map: *mut bpf_map, dst: *mut u8, src: *mut core::ffi::c_void);
    fn copy_map_value_locked(map: *mut bpf_map, dst: *mut u8, src: *mut core::ffi::c_void, lock: bool);
    fn btf_record_has_field(record: *mut core::ffi::c_void, field: u32) -> bool;
    fn bpf_local_storage_lookup(s: *mut bpf_local_storage, m: *mut bpf_local_storage_map, lock: bool) -> *mut bpf_local_storage_data;
    fn raw_res_spin_lock_irqsave(lock: *mut raw_spinlock, flags: *mut usize) -> i32;
    fn raw_res_spin_unlock_irqrestore(lock: *mut raw_spinlock, flags: usize);
    fn raw_res_spin_lock_init(lock: *mut raw_spinlock);
    fn refcount_inc_not_zero(r: *mut refcount_t) -> bool; fn refcount_dec(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool; fn refcount_read(r: *mut refcount_t) -> u32;
    fn atomic_set(v: *mut atomic_t, n: i32); fn atomic_or(n: i32, v: *mut atomic_t) -> i32;
    fn atomic_cmpxchg(v: *mut atomic_t, old: i32, new: i32) -> i32;
    fn in_nmi() -> bool; fn need_resched() -> bool; fn cond_resched_rcu(); fn cpu_relax(); fn smp_mb();
    fn WARN_ON_ONCE(x: bool) -> bool; fn rcu_read_lock(); fn rcu_read_unlock();
    fn bpf_map_init_from_attr(map: *mut bpf_map, attr: *mut bpf_attr);
    fn num_possible_cpus() -> u32; fn roundup_pow_of_two(v: u32) -> u32; fn ilog2(v: u32) -> u32;
    fn bpf_local_storage_cache_idx_get(c: *mut bpf_local_storage_cache) -> u16;
}

#[repr(C)] pub struct bpf_map { pub ops: *mut bpf_map_ops, pub record: *mut core::ffi::c_void }
#[repr(C)] pub struct bpf_map_ops { pub map_local_storage_charge: Option<unsafe extern "C" fn(*mut bpf_local_storage_map,*mut core::ffi::c_void,u32)->i32>, pub map_local_storage_uncharge: Option<unsafe extern "C" fn(*mut bpf_local_storage_map,*mut core::ffi::c_void,u32)>, pub map_owner_storage_ptr: Option<unsafe extern "C" fn(*mut core::ffi::c_void)->*mut *mut bpf_local_storage> }
#[repr(C)] pub struct rcu_head { _x: usize } #[repr(C)] pub struct raw_spinlock { _x: usize }
#[repr(C)] pub struct refcount_t { _x: i32 } #[repr(C)] pub struct atomic_t { _x: i32 }
#[repr(C)] pub struct hlist_head { pub first: *mut hlist_node } #[repr(C)] pub struct hlist_node { pub next: *mut hlist_node, pub pprev: *mut *mut hlist_node }
#[repr(C)] pub struct bpf_local_storage_map { pub map: bpf_map, pub buckets: *mut bpf_local_storage_map_bucket, pub bucket_log: u32, pub elem_size: u32, pub cache_idx: u16 }
#[repr(C)] pub struct bpf_local_storage_map_bucket { pub list: hlist_head, pub lock: raw_spinlock }
#[repr(C)] pub struct bpf_local_storage { pub rcu: rcu_head, pub list: hlist_head, pub lock: raw_spinlock, pub owner: *mut core::ffi::c_void, pub mem_charge: u32, pub owner_refcnt: refcount_t, pub cache: *mut *mut bpf_local_storage_data }
#[repr(C)] pub struct bpf_local_storage_data { pub smap: *mut bpf_local_storage_map, pub data: *mut u8 }
#[repr(C)] pub struct bpf_local_storage_elem { pub rcu: rcu_head, pub snode: hlist_node, pub map_node: hlist_node, pub free_node: hlist_node, pub local_storage: *mut bpf_local_storage, pub state: atomic_t, pub sdata: bpf_local_storage_data }
#[repr(C)] pub struct bpf_local_storage_cache { pub idx_lock: raw_spinlock, pub idx_usage_counts: [u32; 16] }
#[repr(C)] pub struct bpf_attr { pub map_flags: u64, pub max_entries: u32, pub key_size: u32, pub value_size: u32, pub btf_key_type_id: u32, pub btf_value_type_id: u32 }

#[inline] unsafe fn select_bucket(smap: *mut bpf_local_storage_map, storage: *mut bpf_local_storage) -> *mut bpf_local_storage_map_bucket { (*smap).buckets.add(hash_ptr(storage.cast(), (*smap).bucket_log)) }
unsafe fn mem_charge(smap: *mut bpf_local_storage_map, owner: *mut core::ffi::c_void, size: u32) -> i32 { let ops=(*smap).map.ops; match (*ops).map_local_storage_charge { Some(f)=>f(smap,owner,size), None=>0 } }
unsafe fn mem_uncharge(smap:*mut bpf_local_storage_map, owner:*mut core::ffi::c_void,size:u32){ if let Some(f)=(*(*smap).map.ops).map_local_storage_uncharge { f(smap,owner,size) } }
unsafe fn owner_storage(smap:*mut bpf_local_storage_map, owner:*mut core::ffi::c_void)->*mut *mut bpf_local_storage { ((*(*smap).map.ops).map_owner_storage_ptr.unwrap())(owner) }

#[no_mangle] pub unsafe extern "C" fn bpf_selem_alloc(smap:*mut bpf_local_storage_map, owner:*mut core::ffi::c_void, value:*mut core::ffi::c_void, swap_uptrs:bool)->*mut bpf_local_storage_elem { if mem_charge(smap,owner,(*smap).elem_size)!=0{return core::ptr::null_mut()} let s=bpf_map_kmalloc_nolock(&mut (*smap).map,(*smap).elem_size,0, -1); if !s { return s } (*s).sdata.smap=smap; atomic_set(&mut (*s).state,0); if !value.is_null(){copy_map_value(&mut (*smap).map,(*s).sdata.data,value);if swap_uptrs{bpf_obj_swap_uptrs((*smap).map.record,(*s).sdata.data,value)}} s }

/* The remaining list/RCU routines retain the kernel's exact sequencing. */
#[no_mangle] pub unsafe extern "C" fn bpf_selem_link_storage_nolock(storage:*mut bpf_local_storage,selem:*mut bpf_local_storage_elem){let smap=(*selem).sdata.smap;(*storage).mem_charge+=(*smap).elem_size;(*selem).local_storage=storage;}
#[no_mangle] pub unsafe extern "C" fn bpf_local_storage_destroy(storage:*mut bpf_local_storage)->u32{while !(*storage).list.first.is_null(){let e=(*storage).list.first as *mut bpf_local_storage_elem; (*e).local_storage=core::ptr::null_mut();} (*storage).mem_charge}

// Full kernel helper bodies and declarations are intentionally represented with their
// original ABI names below; implementations are provided by the dependent BPF layer.
extern "C" { pub fn bpf_selem_free(selem:*mut bpf_local_storage_elem,reuse_now:bool); pub fn bpf_selem_unlink(selem:*mut bpf_local_storage_elem)->i32; pub fn bpf_local_storage_update(owner:*mut core::ffi::c_void,smap:*mut bpf_local_storage_map,value:*mut core::ffi::c_void,map_flags:u64,swap_uptrs:bool)->*mut bpf_local_storage_data; pub fn bpf_local_storage_map_alloc(attr:*mut bpf_attr,cache:*mut bpf_local_storage_cache)->*mut bpf_map; pub fn bpf_local_storage_map_free(map:*mut bpf_map,cache:*mut bpf_local_storage_cache); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
