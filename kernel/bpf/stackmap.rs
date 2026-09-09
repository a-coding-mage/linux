// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2016 Facebook */
// External Linux/BPF declarations are supplied by the surrounding kernel translation.

const STACK_CREATE_FLAG_MASK: u64 = BPF_F_NUMA_NODE | BPF_F_RDONLY | BPF_F_WRONLY | BPF_F_STACK_BUILD_ID;

#[repr(C)]
pub struct stack_map_bucket { pub fnode: pcpu_freelist_node, pub hash: u32, pub nr: u32, pub data: [u64; 0] }
#[repr(C)]
pub struct bpf_stack_map { pub map: bpf_map, pub elems: *mut core::ffi::c_void, pub freelist: pcpu_freelist, pub n_buckets: u32, pub buckets: [*mut stack_map_bucket; 0] }

unsafe fn stack_map_use_build_id(map: *mut bpf_map) -> bool { ((*map).map_flags & BPF_F_STACK_BUILD_ID) != 0 }
unsafe fn stack_map_data_size(map: *mut bpf_map) -> usize { if stack_map_use_build_id(map) { core::mem::size_of::<bpf_stack_build_id>() } else { 8 } }
unsafe fn stack_map_calculate_max_depth(size: u32, elem_size: u32, flags: u64) -> u32 {
    let skip = (flags & BPF_F_SKIP_FIELD_MASK) as u32;
    let mut depth = size / elem_size + skip;
    let limit = READ_ONCE(sysctl_perf_event_max_stack);
    if depth > limit { depth = limit; } depth
}

unsafe fn prealloc_elems_and_freelist(smap: *mut bpf_stack_map) -> i32 {
    let elem_size = core::mem::size_of::<stack_map_bucket>() as u64 + (*smap).map.value_size as u64;
    (*smap).elems = bpf_map_area_alloc(elem_size * (*smap).map.max_entries as u64, (*smap).map.numa_node);
    if (*smap).elems.is_null() { return -ENOMEM; }
    let err = pcpu_freelist_init(&mut (*smap).freelist);
    if err != 0 { bpf_map_area_free((*smap).elems); return err; }
    pcpu_freelist_populate(&mut (*smap).freelist, (*smap).elems, elem_size, (*smap).map.max_entries); 0
}

#[repr(C)] pub struct stack_map_cached_vma { pub vm_start: usize, pub vm_end: usize, pub vm_pgoff: usize, pub file: *mut file, pub build_id: *const u8 }
#[repr(C)] pub struct stack_map_build_id_cache { pub resolved: stack_map_cached_vma, pub unresolved: stack_map_cached_vma }
#[repr(C)] pub struct stack_map_vma_lock { pub vma: *mut vm_area_struct, pub mm: *mut mm_struct }
#[repr(C)] pub struct stackid { pub bucket: *mut stack_map_bucket, pub ips: *const u64, pub nr: u32, pub len: u32, pub hash: u32, pub id: u32, pub hash_matches: bool }

unsafe fn fetch_build_id(vma: *mut vm_area_struct, id: *mut u8, may_fault: bool) -> i32 { if may_fault { build_id_parse(vma,id,core::ptr::null_mut()) } else { build_id_parse_nofault(vma,id,core::ptr::null_mut()) } }
unsafe fn stack_map_build_id_set_ip(id: *mut bpf_stack_build_id) { (*id).status=BPF_STACK_BUILD_ID_IP; memset((*id).build_id.as_mut_ptr() as *mut _,0,BUILD_ID_SIZE_MAX); }
unsafe fn stack_map_build_id_offset(pgoff: usize, start: usize, ip: u64) -> u64 { ((pgoff<<PAGE_SHIFT) as u64).wrapping_add(ip).wrapping_sub(start as u64) }
unsafe fn stack_map_build_id_set_valid(id: *mut bpf_stack_build_id, off: u64, bid: *const u8) { (*id).status=BPF_STACK_BUILD_ID_VALID; (*id).offset=off; memcpy((*id).build_id.as_mut_ptr() as *mut _,bid as *const _,BUILD_ID_SIZE_MAX); }
unsafe fn stack_map_build_id_set_from_cache(c: *mut stack_map_build_id_cache,id:*mut bpf_stack_build_id,ip:u64)->i32 {
    let r=&(*c).resolved; if r.vm_end!=0 && ip>=r.vm_start as u64 && ip<r.vm_end as u64 { stack_map_build_id_set_valid(id,stack_map_build_id_offset(r.vm_pgoff,r.vm_start,ip),r.build_id); return 0; }
    let u=&(*c).unresolved; if u.vm_end!=0 && ip>=u.vm_start as u64 && ip<u.vm_end as u64 { stack_map_build_id_set_ip(id); return 0; } -ENOENT
}

unsafe fn stackid_init(s:*mut stackid,map:*mut bpf_map,trace:*const perf_callchain_entry,n:u32,flags:u64)->i32 {
    let smap=container_of!(map,bpf_stack_map,map); let skip=(flags&BPF_F_SKIP_FIELD_MASK) as u32; if n<=skip{return -EFAULT;}
    let max=stack_map_calculate_max_depth((*map).value_size,stack_map_data_size(map) as u32,flags); (*s).nr=core::cmp::min(n-skip,max-skip); (*s).len=(*s).nr*8; (*s).ips=(*trace).ip.as_ptr().add(skip as usize); (*s).hash=jhash2((*s).ips as *const u32,(*s).len/4,0); (*s).id=(*s).hash&((*smap).n_buckets-1); (*s).bucket=READ_ONCE((*smap).buckets.as_ptr().add((*s).id as usize).read()); (*s).hash_matches=!(*s).bucket.is_null()&&(*(*s).bucket).hash==(*s).hash; 0
}
unsafe fn stackid_fastpath(s:*mut stackid,map:*mut bpf_map,t:*const perf_callchain_entry,n:u32,f:u64)->i32 { let e=stackid_init(s,map,t,n,f); if e!=0{return e;} if (*s).hash_matches&&f&BPF_F_FAST_STACK_CMP!=0{return (*s).id as i32;} if stack_map_use_build_id(map){return -ENOENT;} if !(*s).bucket.is_null()&&f&BPF_F_REUSE_STACKID==0{-EEXIST}else{-ENOENT} }

pub unsafe fn stack_map_lookup_elem(_: *mut bpf_map, _: *mut core::ffi::c_void)->*mut core::ffi::c_void { ERR_PTR(-EOPNOTSUPP) }
pub unsafe fn stack_map_update_elem(_: *mut bpf_map, _: *mut core::ffi::c_void, _: *mut core::ffi::c_void, _: u64)->i64 { -EINVAL as i64 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
