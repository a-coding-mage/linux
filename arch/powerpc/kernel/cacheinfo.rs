// SPDX-License-Identifier: GPL-2.0-only
/* Processor cache information made available to userspace via sysfs. */

// C headers and kernel-provided types/functions are supplied by other files.

const CACHE_TYPE_UNIFIED: i32 = 0;
const CACHE_TYPE_UNIFIED_D: i32 = 1;
const CACHE_TYPE_INSTRUCTION: i32 = 2;
const CACHE_TYPE_DATA: i32 = 3;

#[repr(C)]
struct CacheDir { kobj: *mut Kobject, index: *mut CacheIndexDir }

#[repr(C)]
struct CacheIndexDir { kobj: Kobject, next: *mut CacheIndexDir, cache: *mut Cache }

#[repr(C)]
struct CacheTypeInfo {
    name: *const core::ffi::c_char,
    size_prop: *const core::ffi::c_char,
    line_size_props: [*const core::ffi::c_char; 2],
    nr_sets_prop: *const core::ffi::c_char,
}

#[repr(C)]
struct Cache {
    ofnode: *mut DeviceNode,
    shared_cpu_map: Cpumask,
    type_: i32,
    level: i32,
    group_id: i32,
    list: ListHead,
    next_local: *mut Cache,
}

static CACHE_TYPE_INFO: [CacheTypeInfo; 4] = [
    CacheTypeInfo { name: b"Unified\0".as_ptr() as _, size_prop: b"cache-size\0".as_ptr() as _, line_size_props: [b"cache-line-size\0".as_ptr() as _, b"cache-block-size\0".as_ptr() as _], nr_sets_prop: b"cache-sets\0".as_ptr() as _ },
    CacheTypeInfo { name: b"Unified\0".as_ptr() as _, size_prop: b"d-cache-size\0".as_ptr() as _, line_size_props: [b"d-cache-line-size\0".as_ptr() as _, b"d-cache-block-size\0".as_ptr() as _], nr_sets_prop: b"d-cache-sets\0".as_ptr() as _ },
    CacheTypeInfo { name: b"Instruction\0".as_ptr() as _, size_prop: b"i-cache-size\0".as_ptr() as _, line_size_props: [b"i-cache-line-size\0".as_ptr() as _, b"i-cache-block-size\0".as_ptr() as _], nr_sets_prop: b"i-cache-sets\0".as_ptr() as _ },
    CacheTypeInfo { name: b"Data\0".as_ptr() as _, size_prop: b"d-cache-size\0".as_ptr() as _, line_size_props: [b"d-cache-line-size\0".as_ptr() as _, b"d-cache-block-size\0".as_ptr() as _], nr_sets_prop: b"d-cache-sets\0".as_ptr() as _ },
];

static mut CACHE_DIR_PCPU: PerCpu<*mut CacheDir> = PerCpu::new();
static mut CACHE_LIST: ListHead = ListHead::new();

unsafe fn kobj_to_cache_index_dir(k: *mut Kobject) -> *mut CacheIndexDir { container_of!(k, CacheIndexDir, kobj) }
unsafe fn cache_type_string(c: *const Cache) -> *const core::ffi::c_char { CACHE_TYPE_INFO[(*c).type_ as usize].name }

unsafe fn cache_init(c: *mut Cache, type_: i32, level: i32, node: *mut DeviceNode, group_id: i32) {
    (*c).type_ = type_; (*c).level = level; (*c).ofnode = of_node_get(node); (*c).group_id = group_id;
    init_list_head(&mut (*c).list); list_add(&mut (*c).list, &mut CACHE_LIST);
}
unsafe fn new_cache(type_: i32, level: i32, node: *mut DeviceNode, group_id: i32) -> *mut Cache {
    let c = kzalloc::<Cache>(); if !c.is_null() { cache_init(c, type_, level, node, group_id); } c
}
unsafe fn release_cache_debugcheck(c: *mut Cache) { list_for_each_entry!(iter, Cache, &CACHE_LIST, list, { warn_once!((*iter).next_local == c, "cache reference mismatch"); }); }
unsafe fn release_cache(c: *mut Cache) { if c.is_null() { return; } release_cache_debugcheck(c); list_del(&mut (*c).list); of_node_put((*c).ofnode); kfree(c); }

unsafe fn cache_cpu_set(mut c: *mut Cache, cpu: i32) { while !c.is_null() { warn_once!(cpumask_test_cpu(cpu, &(*c).shared_cpu_map), "CPU already accounted"); cpumask_set_cpu(cpu, &mut (*c).shared_cpu_map); c = (*c).next_local; } }
unsafe fn cache_size(c: *const Cache, ret: *mut u32) -> i32 { let p = of_get_property((*c).ofnode, CACHE_TYPE_INFO[(*c).type_ as usize].size_prop, core::ptr::null_mut()); if p.is_null() { return -ENODEV; } *ret = of_read_number(p, 1) as u32; 0 }
unsafe fn cache_size_kb(c: *const Cache, ret: *mut u32) -> i32 { let mut s=0; if cache_size(c,&mut s)!=0 { return -ENODEV; } *ret=s/1024; 0 }
unsafe fn cache_get_line_size(c: *const Cache, ret: *mut u32) -> i32 { let mut p=core::ptr::null(); for prop in CACHE_TYPE_INFO[(*c).type_ as usize].line_size_props { p=of_get_property((*c).ofnode,prop,core::ptr::null_mut()); if !p.is_null(){break;} } if p.is_null(){return -ENODEV;} *ret=of_read_number(p,1) as u32; 0 }
unsafe fn cache_nr_sets(c:*const Cache, ret:*mut u32)->i32 { let p=of_get_property((*c).ofnode,CACHE_TYPE_INFO[(*c).type_ as usize].nr_sets_prop,core::ptr::null_mut()); if p.is_null(){return -ENODEV;} *ret=of_read_number(p,1) as u32; 0 }
unsafe fn cache_associativity(c:*const Cache, ret:*mut u32)->i32 { let mut n=0; if cache_nr_sets(c,&mut n)!=0{return -ENODEV;} if n==1{*ret=0;return 0;} let(mut l,mut s)=(0,0); if cache_get_line_size(c,&mut l)!=0||cache_size(c,&mut s)!=0||n==0||s==0||l==0{return -ENODEV;} *ret=(s/n)/l; 0 }

unsafe fn cache_find_first_sibling(c:*mut Cache)->*mut Cache { if (*c).type_==CACHE_TYPE_UNIFIED||(*c).type_==CACHE_TYPE_UNIFIED_D{return c;} let mut r=c; list_for_each_entry!(i,Cache,&CACHE_LIST,list,{if (*i).ofnode==(*c).ofnode&&(*i).group_id==(*c).group_id&&(*i).next_local==c{r=i;break;}}); r }
unsafe fn cache_lookup_by_node_group(node:*const DeviceNode, group:i32)->*mut Cache { let mut r=core::ptr::null_mut(); list_for_each_entry!(i,Cache,&CACHE_LIST,list,{if (*i).ofnode==node&&(*i).group_id==group{r=cache_find_first_sibling(i);break;}}); r }
unsafe fn cache_node_is_unified(np:*const DeviceNode)->bool { !of_get_property(np,b"cache-unified\0".as_ptr() as _,core::ptr::null_mut()).is_null() }
unsafe fn cache_is_unified_d(np:*const DeviceNode)->i32 { if !of_get_property(np,CACHE_TYPE_INFO[CACHE_TYPE_UNIFIED_D as usize].size_prop,core::ptr::null_mut()).is_null(){CACHE_TYPE_UNIFIED_D}else{CACHE_TYPE_UNIFIED} }
unsafe fn cache_do_one_devnode_unified(n:*mut DeviceNode,g:i32,l:i32)->*mut Cache { new_cache(cache_is_unified_d(n),l,n,g) }
unsafe fn cache_do_one_devnode_split(n:*mut DeviceNode,g:i32,l:i32)->*mut Cache { let d=new_cache(CACHE_TYPE_DATA,l,n,g); let i=new_cache(CACHE_TYPE_INSTRUCTION,l,n,g); if d.is_null()||i.is_null(){release_cache(d);release_cache(i);return core::ptr::null_mut();} (*d).next_local=i; d }
unsafe fn cache_do_one_devnode(n:*mut DeviceNode,g:i32,l:i32)->*mut Cache { if cache_node_is_unified(n){cache_do_one_devnode_unified(n,g,l)}else{cache_do_one_devnode_split(n,g,l)} }
unsafe fn cache_lookup_or_instantiate(n:*mut DeviceNode,g:i32,l:i32)->*mut Cache { let mut c=cache_lookup_by_node_group(n,g); if !c.is_null(){warn_once!((*c).level!=l,"cache level mismatch");} if c.is_null(){c=cache_do_one_devnode(n,g,l);} c }
unsafe fn link_cache_lists(mut s:*mut Cache,b:*mut Cache){while !(*s).next_local.is_null(){if (*s).next_local==b{return;}s=(*s).next_local;}(*s).next_local=b;warn_once!(((*s).level==1&&(*b).level>2)||((*s).level>1&&(*b).level!=(*s).level+1),"cache level skipped");}

// Remaining sysfs and hotplug entry points retain the C control flow and rely on kernel bindings.
unsafe fn get_group_id(cpu:u32, level:i32)->i32 { if HAS_BIG_CORES&&level==1{return cpumask_first(per_cpu(THREAD_GROUP_L1_CACHE_MAP,cpu)) as i32;} if THREAD_GROUP_SHARES_L2&&level==2{return cpumask_first(per_cpu(THREAD_GROUP_L2_CACHE_MAP,cpu)) as i32;} if THREAD_GROUP_SHARES_L3&&level==3{return cpumask_first(per_cpu(THREAD_GROUP_L3_CACHE_MAP,cpu)) as i32;} -1 }

// The following declarations mirror the source's externally supplied kernel objects.
extern "C" { static mut HAS_BIG_CORES: bool; static mut THREAD_GROUP_SHARES_L2: bool; static mut THREAD_GROUP_SHARES_L3: bool; static mut THREAD_GROUP_L1_CACHE_MAP: PerCpu<Cpumask>; static mut THREAD_GROUP_L2_CACHE_MAP: PerCpu<Cpumask>; static mut THREAD_GROUP_L3_CACHE_MAP: PerCpu<Cpumask>; }

unsafe fn do_subsidiary_caches(mut c:*mut Cache,cpu:u32){let mut level=(*c).level;while{let n=of_find_next_cache_node((*c).ofnode);if n.is_null(){false}else{level+=1;let s=cache_lookup_or_instantiate(n,get_group_id(cpu,level),level);of_node_put(n);if s.is_null(){false}else{link_cache_lists(c,s);c=s;true}}}{} }
unsafe fn cache_chain_instantiate(cpu:u32)->*mut Cache{let n=of_get_cpu_node(cpu,core::ptr::null_mut());if n.is_null(){return core::ptr::null_mut();}let c=cache_lookup_or_instantiate(n,get_group_id(cpu,1),1);if !c.is_null(){do_subsidiary_caches(c,cpu);cache_cpu_set(c,cpu);}of_node_put(n);c}

unsafe fn cacheinfo_create_cache_dir(cpu:u32)->*mut CacheDir{let d=get_cpu_device(cpu);if d.is_null(){return core::ptr::null_mut();}let k=kobject_create_and_add(b"cache\0".as_ptr() as _,&mut (*d).kobj);if k.is_null(){return core::ptr::null_mut();}let cd=kzalloc::<CacheDir>();if cd.is_null(){kobject_put(k);return core::ptr::null_mut();}(*cd).kobj=k;per_cpu_set(&mut CACHE_DIR_PCPU,cpu,cd);cd}
unsafe fn cache_index_release(k:*mut Kobject){let i=kobj_to_cache_index_dir(k);kfree(i);}
unsafe fn index_kobj_to_cache(k:*mut Kobject)->*mut Cache{(*kobj_to_cache_index_dir(k)).cache}
unsafe fn size_show(k:*mut Kobject,_:*mut KobjAttribute,b:*mut i8)->isize{let mut v=0;if cache_size_kb(index_kobj_to_cache(k),&mut v)!=0{return -ENODEV as isize;}sysfs_emit(b,b"%uK\n\0".as_ptr() as _,v)}
unsafe fn line_size_show(k:*mut Kobject,_:*mut KobjAttribute,b:*mut i8)->isize{let mut v=0;if cache_get_line_size(index_kobj_to_cache(k),&mut v)!=0{return -ENODEV as isize;}sysfs_emit(b,b"%u\n\0".as_ptr() as _,v)}
unsafe fn nr_sets_show(k:*mut Kobject,_:*mut KobjAttribute,b:*mut i8)->isize{let mut v=0;if cache_nr_sets(index_kobj_to_cache(k),&mut v)!=0{return -ENODEV as isize;}sysfs_emit(b,b"%u\n\0".as_ptr() as _,v)}
unsafe fn associativity_show(k:*mut Kobject,_:*mut KobjAttribute,b:*mut i8)->isize{let mut v=0;if cache_associativity(index_kobj_to_cache(k),&mut v)!=0{return -ENODEV as isize;}sysfs_emit(b,b"%u\n\0".as_ptr() as _,v)}
unsafe fn type_show(k:*mut Kobject,_:*mut KobjAttribute,b:*mut i8)->isize{sysfs_emit(b,b"%s\n\0".as_ptr() as _,cache_type_string(index_kobj_to_cache(k)))}
unsafe fn level_show(k:*mut Kobject,_:*mut KobjAttribute,b:*mut i8)->isize{sysfs_emit(b,b"%d\n\0".as_ptr() as _,(*index_kobj_to_cache(k)).level)}
unsafe fn show_shared_cpumap(k:*mut Kobject,_:*mut KobjAttribute,b:*mut i8,list:bool)->isize{let c=index_kobj_to_cache(k);sysfs_emit(b,if list{b"%*pbl\n\0"}else{b"%*pb\n\0"}.as_ptr() as _,cpumask_pr_args(&(*c).shared_cpu_map))}
unsafe fn shared_cpu_map_show(k:*mut Kobject,a:*mut KobjAttribute,b:*mut i8)->isize{show_shared_cpumap(k,a,b,false)}
unsafe fn shared_cpu_list_show(k:*mut Kobject,a:*mut KobjAttribute,b:*mut i8)->isize{show_shared_cpumap(k,a,b,true)}

unsafe fn cacheinfo_create_index_dir(c:*mut Cache,index:i32,cd:*mut CacheDir){let d=kzalloc::<CacheIndexDir>();if d.is_null(){return;}(*d).cache=c;if kobject_init_and_add(&mut (*d).kobj,&CACHE_INDEX_TYPE,(*cd).kobj,b"index%d\0".as_ptr() as _,index)!=0{kobject_put(&mut (*d).kobj);return;}(*d).next=(*cd).index;(*cd).index=d;}
unsafe fn cacheinfo_sysfs_populate(cpu:u32,mut c:*mut Cache){let d=cacheinfo_create_cache_dir(cpu);if d.is_null(){return;}let mut i=0;while !c.is_null(){cacheinfo_create_index_dir(c,i,d);i+=1;c=(*c).next_local;}}
#[no_mangle] pub unsafe extern "C" fn cacheinfo_cpu_online(cpu:u32){let c=cache_chain_instantiate(cpu);if !c.is_null(){cacheinfo_sysfs_populate(cpu,c);}}

unsafe fn cache_cpu_clear(mut c:*mut Cache,cpu:i32){while !c.is_null(){let n=(*c).next_local;cpumask_clear_cpu(cpu,&mut (*c).shared_cpu_map);if cpumask_empty(&(*c).shared_cpu_map){release_cache(c);}c=n;}}
#[no_mangle] pub unsafe extern "C" fn cacheinfo_cpu_offline(cpu:u32){let d=per_cpu(CACHE_DIR_PCPU,cpu);if !d.is_null(){kfree(d);}per_cpu_set(&mut CACHE_DIR_PCPU,cpu,core::ptr::null_mut());let n=of_get_cpu_node(cpu,core::ptr::null_mut());if !n.is_null(){let c=cache_lookup_by_node_group(n,get_group_id(cpu,1));if !c.is_null(){cache_cpu_clear(c,cpu as i32);}of_node_put(n);}}
#[no_mangle] pub unsafe extern "C" fn cacheinfo_teardown(){for_each_online_cpu!(cpu,{cacheinfo_cpu_offline(cpu);});}
#[no_mangle] pub unsafe extern "C" fn cacheinfo_rebuild(){for_each_online_cpu!(cpu,{cacheinfo_cpu_online(cpu);});}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
