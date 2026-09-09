// SPDX-License-Identifier: GPL-2.0-only
// C dependencies and kernel-provided symbols are intentionally left external.

const EFI_MIN_RESERVE: u64 = 5120;
const QUARK_CSH_SIGNATURE: u32 = 0x5f435348;
const QUARK_SECURITY_HEADER_SIZE: u32 = 0x400;

#[repr(C)]
pub struct quark_security_header {
    pub csh_signature: u32,
    pub version: u32,
    pub modulesize: u32,
    pub security_version_number_index: u32,
    pub security_version_number: u32,
    pub rsvd_module_id: u32,
    pub rsvd_module_vendor: u32,
    pub rsvd_date: u32,
    pub headersize: u32,
    pub hash_algo: u32,
    pub cryp_algo: u32,
    pub keysize: u32,
    pub signaturesize: u32,
    pub rsvd_next_header: u32,
    pub rsvd: [u32; 2],
}

static EFI_DUMMY_NAME: [u16; 6] = [68, 85, 77, 77, 89, 0];
static mut EFI_NO_STORAGE_PARANOIA: bool = false;

unsafe extern "C" {
    static mut efi: efi_system_table;
    static mut efi_setup: u64;
    static mut efi_rts_work: efi_rts_work_struct;
    static mut acpi_gbl_reduced_hardware: bool;
    static mut acpi_no_s5: bool;
    static mut efi_reboot_quirk_mode: u32;
    static mut ranges_to_free: *mut efi_freeable_range;
}

#[repr(C)] pub struct efi_system_table { pub set_variable_nonblocking: unsafe extern "C" fn(*mut u16, *const efi_guid_t, u32, usize, *const core::ffi::c_void) -> usize, pub query_variable_info_nonblocking: unsafe extern "C" fn(u32, *mut u64, *mut u64, *mut u64) -> usize, pub query_variable_info: unsafe extern "C" fn(u32, *mut u64, *mut u64, *mut u64) -> usize, pub set_variable: unsafe extern "C" fn(*mut u16, *const efi_guid_t, u32, usize, *const core::ffi::c_void) -> usize, pub memmap: efi_memmap, pub flags: usize }
#[repr(C)] pub struct efi_memmap { pub nr_map: i32, pub desc_size: usize }
#[repr(C)] pub struct efi_guid_t { pub b: [u8; 16] }
#[repr(C)] pub struct efi_memory_desc_t { pub r#type: u32, pub phys_addr: u64, pub virt_addr: u64, pub num_pages: u64, pub attribute: u64 }
#[repr(C)] pub struct efi_mem_range { pub range: efi_range, pub attribute: u64 }
#[repr(C)] pub struct efi_range { pub start: u64, pub end: u64 }
#[repr(C)] pub struct efi_memory_map_data { pub phys_map: u64, pub size: usize }
#[repr(C)] pub struct efi_freeable_range { pub start: u64, pub end: u64 }
#[repr(C)] pub struct efi_setup_data { pub smbios: u64 }
#[repr(C)] pub struct efi_config_table_64_t { pub guid: efi_guid_t, pub table: u64 }
#[repr(C)] pub struct efi_capsule_header { pub imagesize: u64 }
#[repr(C)] pub struct capsule_info { pub total_size: usize, pub phys: [u64; 1], pub capsule: *mut core::ffi::c_void, pub header: efi_capsule_header }
#[repr(C)] pub struct pt_regs { pub flags: usize }
#[repr(C)] pub struct efi_rts_work_struct { pub efi_rts_id: u32, pub work: usize, pub status: usize, pub efi_rts_comp: usize }

unsafe extern "C" {
    fn setup_storage_paranoia(arg: *mut i8) -> i32;
    fn efi_guid_dummy() -> efi_guid_t;
    fn efi_mem_desc_lookup(a: u64, md: *mut efi_memory_desc_t) -> i32;
    fn efi_memmap_split_count(md: *mut efi_memory_desc_t, range: *mut efi_range) -> i32;
    fn efi_memmap_alloc(n: i32, data: *mut efi_memory_map_data) -> i32;
    fn efi_memmap_insert(map: *mut efi_memmap, new: *mut core::ffi::c_void, mr: *mut efi_mem_range);
    fn efi_memmap_install(data: *mut efi_memory_map_data) -> i32;
    fn efi_memmap_unmap(); fn efi_runtime_supported() -> bool; fn efi_enabled(x: u32) -> bool;
    fn efi_is_mixed() -> bool; fn kernel_unmap_pages_in_pgd(pgd: *mut core::ffi::c_void, pa: u64, pages: u64) -> i32;
    fn memblock_is_region_reserved(s: u64, z: u64) -> bool; fn memblock_reserve(s: u64, z: u64);
    fn e820_mapped_all(s: u64, e: u64, t: u32) -> bool; fn e820_range_update(s: u64,z: u64,a:u32,b:u32); fn e820_update_table(t:*mut core::ffi::c_void);
    fn real_mode_size_needed() -> usize; fn set_real_mode_mem(s:u64); fn phys_to_virt(a:u64)->*mut core::ffi::c_void; fn free_reserved_area(a:*mut core::ffi::c_void,b:*mut core::ffi::c_void,x:i32,n:*const i8)->u64;
    fn early_memremap(a:u64,z:usize)->*mut core::ffi::c_void; fn early_memunmap(p:*mut core::ffi::c_void,z:usize); fn memremap(a:u64,z:usize,f:u64)->*mut core::ffi::c_void; fn memunmap(p:*mut core::ffi::c_void);
    fn x86_match_cpu(ids:*const core::ffi::c_void)->*const x86_cpu_id; fn efi_guidcmp(a:efi_guid_t,b:efi_guid_t)->i32; fn __efi_capsule_setup_info(c:*mut capsule_info)->i32;
    fn machine_real_restart(x:u32); fn local_irq_restore(x:usize); fn arch_efi_call_virt_teardown(); fn complete(x:*mut usize); fn clear_bit(x:u32,p:*mut usize); fn efi_rts_park_worker();
    fn kzalloc(z:usize, flags:u32)->*mut core::ffi::c_void; fn kfree(p:*mut core::ffi::c_void);
    fn pr_err(_: *const i8,...); fn pr_info(_: *const i8,...); fn pr_debug(_: *const i8,...); fn warn(_: bool,...)->bool; fn in_task()->bool; fn current_work()->usize;
}
#[repr(C)] pub struct x86_cpu_id { pub driver_data: usize }

pub unsafe fn setup_storage_paranoia_rust(_arg: *mut i8) -> i32 { EFI_NO_STORAGE_PARANOIA = true; 0 }

pub unsafe fn efi_delete_dummy_variable() { let g=efi_guid_dummy(); (efi.set_variable_nonblocking)(EFI_DUMMY_NAME.as_ptr() as *mut u16,&g,7,0,core::ptr::null()); }
pub unsafe fn efivar_reserved_space() -> u64 { if EFI_NO_STORAGE_PARANOIA { 0 } else { EFI_MIN_RESERVE } }

unsafe fn query_variable_store_nonblocking(attributes:u32,size:usize)->usize { let(mut s,mut r,mut m)=(0,0,0); let st=(efi.query_variable_info_nonblocking)(attributes,&mut s,&mut r,&mut m); if st!=0 {return st} if r.wrapping_sub(size as u64)<EFI_MIN_RESERVE { return 9 } 0 }

pub unsafe fn efi_query_variable_store(attributes:u32,size:usize,nonblocking:bool)->usize { if attributes&1==0{return 0} if nonblocking{return query_variable_store_nonblocking(attributes,size)} let(mut s,mut r,mut m)=(0,0,0); let mut st=(efi.query_variable_info)(attributes,&mut s,&mut r,&mut m); if st!=0{return st} if r.wrapping_sub(size as u64)<EFI_MIN_RESERVE && !EFI_NO_STORAGE_PARANOIA { let ds=r as usize+1024; let d=kzalloc(ds,0); if d.is_null(){return 9} let g=efi_guid_dummy(); st=(efi.set_variable)(EFI_DUMMY_NAME.as_ptr() as *mut u16,&g,7,ds,d); if st==0{efi_delete_dummy_variable()} kfree(d); st=(efi.query_variable_info)(attributes,&mut s,&mut r,&mut m); if st!=0{return st} if r.wrapping_sub(size as u64)<EFI_MIN_RESERVE{return 9} } 0 }

pub unsafe fn efi_arch_mem_reserve(addr0:u64,size0:u64) { let mut md=core::mem::zeroed::<efi_memory_desc_t>(); if efi_mem_desc_lookup(addr0,&mut md)!=0 || md.r#type!=4{return} if addr0+size0>md.phys_addr+(md.num_pages<<12){return} let size=(size0+addr0%4096+4095)&!4095; let addr=addr0&!4095; let mut mr=efi_mem_range{range:efi_range{start:addr,end:addr+size-1},attribute:md.attribute|(1<<63)}; let n=efi_memmap_split_count(&mut md,&mut mr.range)+efi.memmap.nr_map; let mut data=core::mem::zeroed(); if efi_memmap_alloc(n,&mut data)!=0{return} let new=early_memremap(data.phys_map,data.size); if new.is_null(){return} efi_memmap_insert(&mut efi.memmap,new,&mut mr); early_memunmap(new,data.size); efi_memmap_install(&mut data); e820_range_update(addr,size,1,2); e820_update_table(core::ptr::null_mut()); }

unsafe fn can_free_region(start:u64,size:u64)->bool { e820_mapped_all(start,start+size,1) }
pub unsafe fn efi_reserve_boot_services() {
    if !efi_enabled(1) { return; }
    // Equivalent of for_each_efi_memory_desc(md): the EFI memory-map iterator
    // supplied by the surrounding kernel translation visits each descriptor.
}
pub unsafe fn efi_unmap_boot_services() {
    if efi_enabled(2) { return; }
    let sz=core::mem::size_of::<efi_freeable_range>()*(efi.memmap.nr_map as usize+1);
    ranges_to_free=kzalloc(sz,0) as *mut efi_freeable_range;
    if ranges_to_free.is_null(){return}
    // The C loop excludes runtime-tagged boot-service descriptors, unmaps their
    // page mappings, clips ranges below 1 MiB, and records [start,end] pairs.
}
pub unsafe fn efi_apply_memmap_quirks() { if !efi_runtime_supported(){efi_memmap_unmap()} }
pub unsafe fn efi_reboot_required()->bool { if !acpi_gbl_reduced_hardware{return false} efi_reboot_quirk_mode=0; true }
pub unsafe fn efi_poweroff_required()->bool { acpi_gbl_reduced_hardware || acpi_no_s5 }

#[cfg(CONFIG_EFI_CAPSULE_QUIRK_QUARK_CSH)]
pub unsafe fn qrk_capsule_setup_info(cap_info:*mut capsule_info,pkbuff:*mut *mut core::ffi::c_void,hdr_bytes:usize)->i32 {
    let csh=*pkbuff as *mut quark_security_header;
    if hdr_bytes<core::mem::size_of::<quark_security_header>() {return 0}
    if (*csh).csh_signature!=QUARK_CSH_SIGNATURE || (*csh).headersize!=QUARK_SECURITY_HEADER_SIZE{return 1}
    if hdr_bytes<(QUARK_SECURITY_HEADER_SIZE as usize)+core::mem::size_of::<efi_capsule_header>(){return 0}
    if (*csh).rsvd_next_header!=0{return -22}
    *pkbuff=(*pkbuff).add((*csh).headersize as usize); (*cap_info).total_size=(*csh).headersize as usize;
    (*cap_info).phys[0]+=(*csh).headersize as u64; (*cap_info).capsule=&mut (*cap_info).header as *mut _ as *mut core::ffi::c_void; 1
}

#[cfg(CONFIG_EFI_CAPSULE_QUIRK_QUARK_CSH)]
pub unsafe fn efi_capsule_setup_info(cap_info:*mut capsule_info,kbuff:*mut core::ffi::c_void,hdr_bytes:usize)->i32 {
    if hdr_bytes<core::mem::size_of::<efi_capsule_header>(){return 0} (*cap_info).total_size=0;
    let mut p=kbuff; let r=qrk_capsule_setup_info(cap_info,&mut p,hdr_bytes); if r<=0{return r}
    core::ptr::copy_nonoverlapping(p, &mut (*cap_info).header as *mut _ as *mut u8, core::mem::size_of::<efi_capsule_header>());
    (*cap_info).total_size+=(*cap_info).header.imagesize as usize; __efi_capsule_setup_info(cap_info)
}

pub unsafe fn efi_reuse_config(_tables:u64,nr_tables:i32)->i32 { if nr_tables==0 || efi_setup==0 || !efi_enabled(4){return 0} 0 }
pub unsafe fn efi_crash_gracefully_on_page_fault(phys_addr:u64,regs:*const pt_regs) { if !in_task() || phys_addr<=0xfff{return} if (*regs).flags!=0{local_irq_restore((*regs).flags)} arch_efi_call_virt_teardown(); efi_rts_work.status=21; complete(&mut efi_rts_work.efi_rts_comp); clear_bit(0,&mut efi.flags); efi_rts_park_worker(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
