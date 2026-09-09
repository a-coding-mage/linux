// SPDX-License-Identifier: GPL-2.0-or-later
/* Firmware Assisted dump implementation.  Kernel dependencies are supplied by
 * the surrounding PowerPC kernel translation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

const CRASH_TIMEOUT: u32 = 500;

extern "C" {
    static mut fw_dump: fw_dump;
    static mut fadump_mutex: mutex;
    static mut cpus_in_fadump: atomic_t;
    static mut reserved_mrange_info: fadump_mrange_info;
    fn fadump_reserve_crash_area(base: u64);
}

#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct fadump_memory_range { pub base: u64, pub size: u64 }
#[repr(C)] pub struct fadump_mrange_info {
    pub name: *const c_char, pub mem_ranges: *mut fadump_memory_range,
    pub mem_ranges_sz: u64, pub mem_range_cnt: c_int,
    pub max_mem_ranges: c_int, pub is_static: bool,
}
#[repr(C)] pub struct fadump_ops {
    pub fadump_get_bootmem_min: Option<unsafe extern "C" fn() -> u64>,
    pub fadump_max_boot_mem_rgns: Option<unsafe extern "C" fn() -> c_int>,
    pub fadump_get_metadata_size: Option<unsafe extern "C" fn() -> u64>,
    pub fadump_setup_metadata: Option<unsafe extern "C" fn(*mut fw_dump) -> c_int>,
    pub fadump_trigger: Option<unsafe extern "C" fn(*mut fadump_crash_info_header,*const c_char)>,
    pub fadump_register: Option<unsafe extern "C" fn(*mut fw_dump)->c_int>,
    pub fadump_unregister: Option<unsafe extern "C" fn(*mut fw_dump)>,
    pub fadump_invalidate: Option<unsafe extern "C" fn(*mut fw_dump)>,
    pub fadump_cleanup: Option<unsafe extern "C" fn(*mut fw_dump)>,
    pub fadump_init_mem_struct: Option<unsafe extern "C" fn(*mut fw_dump)>,
    pub fadump_process: Option<unsafe extern "C" fn(*mut fw_dump)->c_int>,
}
#[repr(C)] pub struct fw_dump {
    pub fadump_supported: c_int, pub fadump_enabled: c_int, pub dump_active: c_int,
    pub dump_registered: c_int, pub nocma: c_int, pub reserve_bootvar: u64,
    pub reserve_dump_area_start: u64, pub reserve_dump_area_size: u64,
    pub boot_memory_size: u64, pub boot_mem_top: u64, pub cpu_state_data_size: u64,
    pub hpte_region_size: u64, pub max_copy_size: u64, pub boot_mem_regs_cnt: c_int,
    pub boot_mem_addr: [u64; 64], pub boot_mem_sz: [u64; 64], pub boot_mem_dest_addr: u64,
    pub fadumphdr_addr: u64, pub elfcorehdr_addr: u64, pub elfcorehdr_size: u64,
    pub cpu_notes_buf_vaddr: u64, pub cpu_notes_buf_size: u64, pub param_area: u64,
    pub param_area_supported: c_int, pub ops: *mut fadump_ops,
}
#[repr(C)] pub struct pt_regs { _data: [u8; 0] }
#[repr(C)] pub struct fadump_crash_info_header {
    pub magic_number:u64, pub version:u64, pub crashing_cpu:u32, pub vmcoreinfo_raddr:u64,
    pub vmcoreinfo_size:u64, pub pt_regs_sz:u64, pub cpu_mask_sz:u64,
    pub regs: pt_regs, pub cpu_mask: [u8; 0],
}
#[repr(C)] pub struct elf_phdr { pub p_type:u32,p_flags:u32,p_offset:u64,p_vaddr:u64,p_paddr:u64,p_filesz:u64,p_memsz:u64,p_align:u64 }
#[repr(C)] pub struct elfhdr { pub e_ident:[u8;16],pub e_type:u16,pub e_machine:u16,pub e_version:u32,pub e_entry:u64,pub e_phoff:u64,pub e_shoff:u64,pub e_flags:u32,pub e_ehsize:u16,pub e_phentsize:u16,pub e_phnum:u16,pub e_shentsize:u16,pub e_shnum:u16,pub e_shstrndx:u16 }
extern "C" { fn __va(x:u64)->*mut c_void; fn __pa(x:*const c_void)->u64; fn memblock_phys_mem_size()->u64; fn memblock_end_of_DRAM()->u64; fn memblock_reserve(u64,u64)->c_int; fn paddr_vmcoreinfo_note()->u64; fn virt_to_phys(*const c_void)->u64; fn fadump_get_boot_mem_regions()->c_int; }

pub unsafe fn is_fadump_memory_area(addr:u64,size:u64)->c_int { if fw_dump.dump_registered==0||size==0{return 0}; let s=fw_dump.reserve_dump_area_start; let e=s+fw_dump.reserve_dump_area_size; if addr+size>s&&addr<=e {1} else {(addr<=fw_dump.boot_mem_top) as c_int} }
pub unsafe fn should_fadump_crash()->c_int {(fw_dump.dump_registered!=0&&fw_dump.fadumphdr_addr!=0) as c_int}
pub unsafe fn is_fadump_active()->c_int {fw_dump.dump_active}

pub unsafe fn fadump_calculate_reserve_size()->u64 { let mut base=0; let mut size=0; let r=parse_crashkernel(core::ptr::null(),memblock_phys_mem_size(),&mut size,&mut base); if r==0&&size>0 {fw_dump.reserve_bootvar=size; return size} if fw_dump.reserve_bootvar!=0{return fw_dump.reserve_bootvar} size=memblock_phys_mem_size()/20; size &= !0x0fffffff; let min=fw_dump.ops.as_ref().unwrap().fadump_get_bootmem_min.unwrap()(); if size>min{size}else{min} }
extern "C" { fn parse_crashkernel(*const c_char,u64,*mut u64,*mut u64)->c_int; }
pub unsafe fn get_fadump_area_size()->u64 { let mut s=fw_dump.cpu_state_data_size+fw_dump.hpte_region_size; s=(s+4095)&!4095; s+=fw_dump.boot_memory_size+core::mem::size_of::<fadump_crash_info_header>() as u64; if let Some(f)=(*fw_dump.ops).fadump_get_metadata_size{s+=f()} s }

pub unsafe fn fadump_reserve_mem()->c_int { if fw_dump.fadump_enabled==0{return 0} if fw_dump.fadump_supported==0 {return 0} if fw_dump.dump_active {fadump_reserve_crash_area(fw_dump.boot_mem_top)} else {fw_dump.boot_memory_size=fadump_calculate_reserve_size(); if fadump_get_boot_mem_regions()==0{return 0}; fw_dump.reserve_dump_area_start=fw_dump.boot_mem_top; fw_dump.reserve_dump_area_size=get_fadump_area_size(); if memblock_reserve(fw_dump.reserve_dump_area_start,fw_dump.reserve_dump_area_size)!=0{return 0} } 1 }

pub unsafe fn crash_fadump(_regs:*mut pt_regs,_str:*const c_char) { if should_fadump_crash()==0{return}; let fdh=__va(fw_dump.fadumphdr_addr) as *mut fadump_crash_info_header; if !fdh.is_null(){(*fdh).crashing_cpu=0;} if let Some(f)=(*fw_dump.ops).fadump_trigger{f(fdh,_str)} }
pub unsafe fn fadump_setup_cpu_notes_buf(_n:u32)->i32 {0}
pub unsafe fn fadump_free_cpu_notes_buf(){fw_dump.cpu_notes_buf_vaddr=0;fw_dump.cpu_notes_buf_size=0;}
pub unsafe fn fadump_setup_param_area() { if fw_dump.fadump_enabled!=0&&!fw_dump.dump_active { fw_dump.param_area=0; } }
pub unsafe fn setup_fadump()->c_int { if fw_dump.fadump_supported==0{return 0}; if fw_dump.fadump_enabled==0{return 1}; 1 }
pub unsafe fn early_init_dt_scan_fw_dump(_node:usize,_uname:*const c_char,_depth:c_int,_data:*mut c_void)->c_int {0}
pub unsafe fn fadump_cleanup(){if !fw_dump.ops.is_null(){if let Some(f)=(*fw_dump.ops).fadump_cleanup{f(&mut fw_dump)}}}

unsafe fn fadump_reserve_crash_area_impl(base:u64){let mut i=0;let mut s=0;let mut e=0;while next_mem_range(i,&mut s,&mut e)!=0{i+=1;if e<base{continue}if s<base{s=base}fadump_reserve_range(s,e-s)}}
extern "C" {fn next_mem_range(u64,*mut u64,*mut u64)->c_int;fn fadump_reserve_range(u64,u64);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
