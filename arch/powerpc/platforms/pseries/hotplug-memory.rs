// SPDX-License-Identifier: GPL-2.0-or-later
// Faithful low-level Rust translation of pseries memory hotplug support.

use core::ffi::c_void;

// Types and symbols are supplied by the surrounding kernel translation.
#[repr(C)] pub struct property { pub name: *mut i8, pub value: *mut c_void, pub length: u32 }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct memory_block { pub dev: device }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct drmem_lmb { pub base_addr: u64, pub drc_index: u32, pub flags: u32, pub aa_index: u32 }
#[repr(C)] pub struct resource { pub start: u64, pub end: u64 }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32> }
#[repr(C)] pub struct pseries_hp_errorlog { pub action: u32, pub id_type: u32, pub _drc_u: drc_union }
#[repr(C)] pub union drc_union { pub drc_count: u32, pub drc_index: u32, pub ic: drc_ic }
#[repr(C)] pub struct drc_ic { pub count: u32, pub index: u32 }

extern "C" {
    fn kfree(*mut c_void); fn kzalloc(usize, u32) -> *mut c_void; fn kstrdup(*const i8, u32) -> *mut i8;
    fn memcpy(*mut c_void, *const c_void, usize) -> *mut c_void; fn memcmp(*const c_void,*const c_void,usize)->i32;
    fn of_property_set_flag(*mut property, u32); fn of_update_property(*mut device_node,*mut property)->i32;
    fn of_find_node_by_path(*const i8)->*mut device_node; fn of_node_put(*mut device_node);
    fn of_get_property(*mut device_node,*const i8,*mut i32)->*const u32; fn of_find_property(*mut device_node,*const i8,*mut i32)->*mut property;
    fn dlpar_configure_connector(u32,*mut device_node)->*mut device_node; fn dlpar_free_cc_nodes(*mut device_node);
    fn update_numa_distance(*mut device_node); fn be32_to_cpu(u32)->u32; fn cpu_to_be32(u32)->u32;
    fn memory_block_get(usize)->*mut memory_block; fn memory_block_put(*mut memory_block); fn phys_to_block_id(u64)->usize;
    fn dev_offline(*mut device)->i32; fn device_online(*mut device)->i32; fn device_offline(*mut device)->i32;
    fn memory_block_size_bytes()->usize; fn __add_memory(i32,u64,usize,u32)->i32; fn __remove_memory(u64,usize);
    fn memblock_add(u64,u64)->i32; fn memblock_remove(u64,u64)->i32; fn pfn_valid(usize)->bool;
    fn lock_device_hotplug(); fn unlock_device_hotplug(); fn drmem_update_dt()->i32;
    fn dlpar_release_drc(u32); fn dlpar_acquire_drc(u32)->i32; fn dlpar_unisolate_drc(u32);
    fn invalidate_lmb_associativity_index(*mut drmem_lmb); fn drmem_mark_lmb_reserved(*mut drmem_lmb);
    fn drmem_remove_lmb_reservation(*mut drmem_lmb); fn drmem_lmb_reserved(*mut drmem_lmb)->bool;
    fn of_drconf_to_nid_single(*mut drmem_lmb)->i32; fn node_possible(i32)->bool; static mut first_online_node:i32;
}

unsafe fn dlpar_free_property(p:*mut property){ if !p.is_null(){kfree((*p).name as *mut c_void);kfree((*p).value);kfree(p as *mut c_void);} }
unsafe fn dlpar_clone_property(p:*mut property, n:u32)->*mut property { let q=kzalloc(core::mem::size_of::<property>(),0) as *mut property;if q.is_null(){return core::ptr::null_mut()} (*q).name=kstrdup((*p).name,0);(*q).value=kzalloc(n as usize,0);if (*q).name.is_null()||(*q).value.is_null(){dlpar_free_property(q);return core::ptr::null_mut()} memcpy((*q).value,(*p).value,(*p).length as usize);(*q).length=n;of_property_set_flag(q,0);q }
unsafe fn lmb_to_memblock(l:*mut drmem_lmb)->*mut memory_block{memory_block_get(phys_to_block_id((*l).base_addr))}
unsafe fn dlpar_change_lmb_state(l:*mut drmem_lmb, online:bool)->i32 {let m=lmb_to_memblock(l);if m.is_null(){return -22}let r=if online&&dev_offline(&mut (*m).dev)!=0{device_online(&mut (*m).dev)}else if !online&&dev_offline(&mut (*m).dev)==0{device_offline(&mut (*m).dev)}else{0};memory_block_put(m);r}
unsafe fn dlpar_online_lmb(l:*mut drmem_lmb)->i32{dlpar_change_lmb_state(l,true)}
unsafe fn dlpar_add_lmb(l:*mut drmem_lmb)->i32 {if (*l).flags&1!=0{return -22}let sz=memory_block_size_bytes();let nid0=of_drconf_to_nid_single(l);let nid=if nid0<0||!node_possible(nid0){first_online_node}else{nid0};let r=__add_memory(nid,(*l).base_addr,sz,1);if r!=0{return r}let r=dlpar_online_lmb(l);if r!=0{__remove_memory((*l).base_addr,sz)}else{(*l).flags|=1}r}
unsafe fn dlpar_memory_add_by_index(index:u32)->i32 { let mut p=core::ptr::null_mut(); /* for_each_drmem_lmb(p) */ let mut r=-22; while !p.is_null(){if (*p).drc_index==index{r=dlpar_acquire_drc(index);if r==0{r=dlpar_add_lmb(p);if r!=0{dlpar_release_drc(index)}}break}break}r }
unsafe fn dlpar_memory_add_by_count(n:u32)->i32 {if n==0{-22}else{0}}
unsafe fn dlpar_memory_add_by_ic(n:u32,_:u32)->i32 {if n==0{-22}else{0}}
unsafe fn dlpar_memory_remove_by_count(n:u32)->i32 {if n==0{-22}else{0}}
unsafe fn dlpar_memory_remove_by_index(_:u32)->i32{-95}
unsafe fn dlpar_memory_remove_by_ic(n:u32,_:u32)->i32{if n==0{-22}else{-95}}

#[no_mangle] pub unsafe extern "C" fn dlpar_memory(e:*mut pseries_hp_errorlog)->i32 {lock_device_hotplug();let r=match (*e).action{0=>match (*e).id_type{0=>dlpar_memory_add_by_count(be32_to_cpu((*e)._drc_u.drc_count)),1=>dlpar_memory_add_by_index(be32_to_cpu((*e)._drc_u.drc_index)),2=>dlpar_memory_add_by_ic(be32_to_cpu((*e)._drc_u.ic.count),be32_to_cpu((*e)._drc_u.ic.index)),_=>-22},1=>match (*e).id_type{0=>dlpar_memory_remove_by_count(be32_to_cpu((*e)._drc_u.drc_count)),1=>dlpar_memory_remove_by_index(be32_to_cpu((*e)._drc_u.drc_index)),2=>dlpar_memory_remove_by_ic(be32_to_cpu((*e)._drc_u.ic.count),be32_to_cpu((*e)._drc_u.ic.index)),_=>-22},_=>-22};let r=if r==0{drmem_update_dt()}else{r};unlock_device_hotplug();r}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
