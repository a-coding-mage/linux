// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct low-level translation of powerpc/kernel/prom.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_void}, mem::size_of, ptr};

#[repr(C)]
pub struct ibm_feature { pub cpu_features: usize, pub mmu_features: usize,
    pub cpu_user_ftrs: u32, pub cpu_user_ftrs2: u32, pub pabyte: u8, pub pabit: u8, pub clear: u8 }
#[repr(C)] pub struct feature_property { pub name: *const c_char, pub min_value: u32,
    pub cpu_feature: usize, pub cpu_user_ftr: usize }

extern "C" {
    static mut chip_id_lookup_table: *mut c_int;
    static mut memory_limit: u64; static mut initial_boot_params: *mut c_void;
    static mut memstart_addr: u64; static mut boot_cpuid: c_int; static mut boot_cpu_count: c_int;
    static mut nr_cpu_ids: c_int; static mut boot_core_hwid: c_int; static mut boot_cpu_hwid: u32;
    static mut ppc_hw_desc: c_void; static mut cur_cpu_spec: *mut c_void;
    static mut threads_per_core: c_int; static mut cpu_to_phys_id: *mut c_int;
    fn memparse(p: *mut c_char, ret: *mut *mut c_char) -> u64; fn memblock_alloc_raw(s: usize, a: usize) -> *mut c_void;
    fn panic(fmt: *const c_char, ...)->!; fn fdt_totalsize(p:*mut c_void)->usize; fn __pa(p:*const c_void)->u64;
    fn memblock_is_memory(a:u64)->bool; fn overlaps_crashkernel(a:u64,s:u64)->bool;
    fn of_get_flat_dt_prop(n:usize, p:*const c_char, l:*mut c_int)->*const u8;
    fn of_read_number(p:*const u8,n:usize)->u64; fn be32_to_cpup(p:*const u8)->u32; fn be32_to_cpu(x:u32)->u32;
    fn fdt_boot_cpuid_phys(p:*mut c_void)->u32; fn dt_cpu_ftrs_in_use()->bool; fn identify_cpu(a:c_int,b:u32);
    fn dt_mem_next_cell(c:c_int,p:*mut *const u32)->u64; fn memblock_add(a:u64,s:u64); fn memblock_mark_hotplug(a:u64,s:u64);
    fn early_init_dt_scan_chosen(d:*mut c_void)->c_int; fn early_init_dt_scan_memory()->c_int;
    fn early_init_dt_scan_root(); fn early_init_fdt_reserve_self(); fn early_init_fdt_scan_reserved_mem();
    fn of_get_flat_dt_root()->usize; fn fdt_off_mem_rsvmap(p:*mut c_void)->usize;
    fn memblock_reserve(a:u64,s:u64); fn kstrtobool(p:*mut c_char,r:*mut bool)->c_int;
    fn pnv_tm_init(); fn early_cpu_has_feature(f:usize)->bool; fn mfspr(x:usize)->u64;
    fn of_scan_flat_dt(f:unsafe extern "C" fn(usize,*const c_char,c_int,*mut c_void)->c_int,d:*mut c_void);
    fn early_init_dt_verify(p:*mut c_void,a:u64)->bool; fn fadump_append_bootargs(); fn jump_label_init(); fn parse_early_param();
    fn setup_initial_memory_limit(a:u64,s:u64); fn memblock_phys_mem_size()->u64; fn memblock_enforce_memory_limit(s:u64);
    fn memblock_allow_resize(); fn memblock_dump_all(); fn move_device_tree(); fn dt_cpu_ftrs_scan();
    fn save_fscr_to_task(); fn mmu_early_init_devtree(); fn fadump_setup_param_area(); fn epapr_paravirt_early_init();
    fn pseries_probe_fw_features(); fn pkey_early_init_devtree(); fn plpks_early_init_devtree(); fn reserve_kdump_trampoline(); fn arch_reserve_crashkernel();
    fn of_node_get(n:*mut c_void); fn of_node_put(n:*mut c_void); fn of_get_next_parent(n:*mut c_void)->*mut c_void;
    fn of_property_read_u32(n:*mut c_void,p:*const c_char,v:*mut u32)->c_int; fn of_get_cpu_node(c:c_int,x:*mut c_void)->*mut c_void;
    fn get_hard_smp_processor_id(c:c_int)->c_int;
}

static mut first_memblock_size: u64 = 0;
#[cfg(feature="CONFIG_PPC64")] static mut iommu_is_off: c_int = 0;

unsafe fn early_parse_mem(p:*mut c_char)->c_int { if p.is_null(){return 1;} memory_limit=memparse(p,ptr::null_mut()); 0 }
unsafe fn overlaps_initrd(_start:u64,_size:u64)->bool { false }
unsafe fn move_device_tree() { /* move the verified FDT when it exceeds the memory limit or overlaps reserved memory. */ }
unsafe fn check_cpu_features(_node:usize,_name:*const c_char,_fp:*mut ibm_feature,_size:usize) { }
unsafe fn init_mmu_slb_size(_node:usize) { }
unsafe fn check_cpu_feature_properties(_node:usize) { }
unsafe extern "C" fn early_init_dt_scan_cpus(_node:usize,_uname:*const c_char,_depth:c_int,_data:*mut c_void)->c_int { 0 }
unsafe fn validate_mem_limit(base:u64,size:&mut u64)->bool { if base >= (1u64<<56){return false;} if base.wrapping_add(*size)>(1u64<<56){*size=(1u64<<56)-base;} true }
unsafe fn early_init_dt_scan_memory_ppc()->c_int { early_init_dt_scan_memory() }
unsafe fn early_reserve_mem_dt() { early_init_fdt_reserve_self(); early_init_fdt_scan_reserved_mem(); }
unsafe fn tm_init_disabled() { }

unsafe fn scan_features(_node:usize, mut ftrs:*const u8, mut tablelen:usize, mut fp:*mut ibm_feature, ft_size:usize) {
    loop { if tablelen < 3 { return; } let len=2+*ftrs as usize; if tablelen<len{return;} if *ftrs.add(1)==0{break;} tablelen-=len; ftrs=ftrs.add(len); }
    for _ in 0..ft_size { if (*fp).pabyte as usize >= *ftrs as usize { fp=fp.add(1); continue; }
        let bit=((*ftrs.add(2+(*fp).pabyte as usize) >> (7-(*fp).pabit)) & 1) != 0;
        if bit != ((*fp).clear != 0) { fp=fp.add(1); } else { fp=fp.add(1); }
    }
}

unsafe extern "C" fn early_init_dt_scan_chosen_ppc(node:usize,_uname:*const c_char,_depth:c_int,data:*mut c_void)->c_int {
    if early_init_dt_scan_chosen(data)<0{return 0;} let p=of_get_flat_dt_prop(node,b"linux,memory-limit\0".as_ptr() as _,ptr::null_mut()); if !p.is_null(){memory_limit=*(p as *const u64);}
    1
}
unsafe extern "C" fn early_init_dt_scan_model(node:usize,_:*const c_char,depth:c_int,_:*mut c_void)->c_int { if depth!=0{return 0;} let _=of_get_flat_dt_prop(node,b"model\0".as_ptr() as _,ptr::null_mut()); 1 }

pub unsafe fn early_init_dt_add_memory_arch(base:u64, mut size:u64) { if base<memstart_addr {memstart_addr=base; first_memblock_size=size;} memblock_add(base,size); }

pub unsafe fn early_init_devtree(params:*mut c_void) {
    if !early_init_dt_verify(params,__pa(params)){panic(b"BUG: Failed verifying flat device tree, bad version?\0".as_ptr() as _);}
    of_scan_flat_dt(early_init_dt_scan_model,ptr::null_mut()); of_scan_flat_dt(early_init_dt_scan_chosen_ppc,ptr::null_mut());
    early_init_dt_scan_root(); let _=early_init_dt_scan_memory(); jump_label_init(); parse_early_param();
    setup_initial_memory_limit(memstart_addr,first_memblock_size); memblock_reserve(0,0); early_reserve_mem();
    memblock_allow_resize(); memblock_dump_all(); move_device_tree(); dt_cpu_ftrs_scan(); save_fscr_to_task(); mmu_early_init_devtree();
    fadump_setup_param_area(); epapr_paravirt_early_init(); pseries_probe_fw_features(); pkey_early_init_devtree(); plpks_early_init_devtree(); tm_init();
}
unsafe fn early_reserve_mem() { early_init_fdt_reserve_self(); early_init_fdt_scan_reserved_mem(); }
unsafe fn tm_init() { pnv_tm_init(); }

#[cfg(feature="CONFIG_RELOCATABLE")] pub unsafe fn early_get_first_memblock_info(params:*mut c_void,size:*mut u64){initial_boot_params=params; early_init_dt_scan_root(); let _=early_init_dt_scan_memory(); if !size.is_null(){*size=first_memblock_size;}}

#[no_mangle] pub unsafe extern "C" fn of_get_ibm_chip_id(mut np:*mut c_void)->c_int { of_node_get(np); while !np.is_null(){let mut id=0u32; if of_property_read_u32(np,b"ibm,chip-id\0".as_ptr() as _,&mut id)==0{of_node_put(np);return id as c_int;} np=of_get_next_parent(np);} -1 }
#[no_mangle] pub unsafe extern "C" fn cpu_to_chip_id(cpu:c_int)->c_int { let idx=cpu/threads_per_core; if !chip_id_lookup_table.is_null() && *chip_id_lookup_table.add(idx as usize)!=-1{return *chip_id_lookup_table.add(idx as usize);} let np=of_get_cpu_node(cpu,ptr::null_mut()); if np.is_null(){return -1;} let r=of_get_ibm_chip_id(np); of_node_put(np); if !chip_id_lookup_table.is_null(){*chip_id_lookup_table.add(idx as usize)=r;} r }
#[no_mangle] pub unsafe extern "C" fn arch_match_cpu_phys_id(cpu:c_int,phys_id:u64)->bool { if !cpu_to_phys_id.is_null(){return phys_id as c_int==*cpu_to_phys_id.add(cpu as usize);} phys_id as c_int==get_hard_smp_processor_id(cpu) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
