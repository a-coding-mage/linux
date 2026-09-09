// SPDX-License-Identifier: GPL-2.0
// External declarations supplied by the s390 boot environment and headers are intentionally omitted.

#[repr(C)]
pub struct VmLayout { _private: [u8; 0] }
#[repr(C)]
pub struct Page { _private: [u8; 0] }
#[repr(C)]
pub union TodClock { pub tod: u64, _raw: [u8; 16] }
#[repr(C)]
pub struct OldmemData { pub start: usize, pub size: usize }

pub static mut vm_layout: VmLayout = VmLayout { _private: [] };
pub static mut __abs_lowcore: usize = 0;
pub static mut __memcpy_real_area: usize = 0;
pub static mut memcpy_real_ptep: *mut usize = core::ptr::null_mut();
pub static mut VMALLOC_START: usize = 0;
pub static mut VMALLOC_END: usize = 0;
pub static mut vmemmap: *mut Page = core::ptr::null_mut();
pub static mut vmemmap_size: usize = 0;
pub static mut MODULES_VADDR: usize = 0;
pub static mut MODULES_END: usize = 0;
pub static mut max_mappable: usize = 0;
pub static mut page_noexec_mask: usize = 0;
pub static mut segment_noexec_mask: usize = 0;
pub static mut region_noexec_mask: usize = 0;
pub static mut tod_clock_base: TodClock = TodClock { tod: 0 };
pub static mut clock_comparator_max: u64 = usize::MAX as u64;
pub static mut stfle_fac_list: [u64; 16] = [0; 16];
pub static mut oldmem_data: OldmemData = OldmemData { start: 0, size: 0 };

extern "C" {
    static mut sysinfo_page: [u8; PAGE_SIZE];
    static mut cmma_flag: u32;
    static mut ident_map_size: usize;
    static mut memory_limit: usize;
    static mut __kaslr_enabled: i32;
    static mut __kaslr_offset: usize;
    static mut __kaslr_offset_phys: usize;
    static mut vmalloc_size: usize;
    static mut vmalloc_size_set: i32;
    static mut __identity_base: usize;
    static mut parmarea: ParmArea;
    static mut vmlinux: Vmlinux;
    static mut ipl_block_valid: i32;
    static mut __boot_data_start: u8;
    static mut __boot_data_end: u8;
    static mut __boot_data_preserved_start: u8;
    static mut __boot_data_preserved_end: u8;
    static mut _compressed_start: u8;
    static mut _decompressor_end: u8;
    static mut _vmlinux_relocs_64_start: u8;
    static mut _vmlinux_relocs_64_end: u8;
    static mut _vmlinux_info: VmlinuxInfo;
}

#[repr(C)] pub struct ParmArea { pub initrd_size: usize, pub initrd_start: usize, pub oldmem_base: usize, pub oldmem_size: usize }
#[repr(C)] pub struct Vmlinux { pub image_size: usize, pub bss_size: usize, pub bootdata_size: usize, pub bootdata_preserved_size: usize, pub bootdata_off: usize, pub bootdata_preserved_off: usize, pub got_start: usize, pub got_end: usize, pub init_mm_off: usize, pub swapper_pg_dir_off: usize, pub invalid_pg_dir_off: usize, pub alt_instructions: usize, pub alt_instructions_end: usize, pub stack_prot_start: usize, pub stack_prot_end: usize, pub kasan_early_shadow_page_off: usize, pub kasan_early_shadow_pte_off: usize, pub kasan_early_shadow_pmd_off: usize, pub kasan_early_shadow_pud_off: usize, pub kasan_early_shadow_p4d_off: usize, pub amode31_size: usize, pub entry: usize }
#[repr(C)] pub struct VmlinuxInfo { pub alt_instructions: usize, pub alt_instructions_end: usize }

extern "C" {
    fn stsi(_: *mut core::ffi::c_void, _: u32, _: u32, _: u32) -> i32; fn set_machine_feature(_: u32); fn memcmp(_: *const u8, _: *const u8, _: usize) -> i32;
    fn machine_is_vm() -> bool; fn __diag288(_: usize, _: usize, _: usize, _: usize) -> i32; fn stap() -> u32;
    fn store_tod_clock_ext_cc(_: *mut TodClock) -> i32; fn set_tod_clock(_: u64) -> i32; fn disabled_wait() -> !; fn get_lowcore() -> *mut Lowcore;
    fn cpu_has_edat1() -> bool; fn local_ctl_set_bit(_: u32, _: u32); fn cpu_has_nx() -> bool; fn cpu_has_vx() -> bool; fn test_facility(_: u32) -> bool;
    fn lpp(_: *mut u64); fn get_physmem_reserved(_: u32, _: *mut usize, _: *mut usize) -> bool; fn physmem_free(_: u32); fn physmem_alloc_or_die(_: u32, _: usize, _: usize) -> usize;
    fn boot_panic(_: *const u8) -> !; fn boot_debug(_: *const u8, ...); fn adjust_to_uv_max(_: usize) -> usize; fn get_random(_: usize, _: *mut usize) -> i32;
    fn setup_vmem(_: usize, _: usize, _: usize); fn jump_to_kernel(_: *const Psw) -> !;
    fn setup_lpp(); fn store_ipl_parmblock(); fn uv_query_info(); fn setup_boot_command_line(); fn parse_boot_command_line(); fn read_ipl_report(); fn sclp_early_read_info(); fn sclp_early_detect_machine_features(); fn sanitize_prot_virt_host(); fn detect_max_physmem_end() -> usize; fn physmem_reserve(_: u32, _: usize, _: usize); fn physmem_set_usable_limit(_: usize); fn detect_physmem_online_ranges(_: usize); fn save_ipl_cert_comp_list(); fn is_ipl_block_dump() -> bool; fn sclp_early_get_hsa_size(_: *mut usize) -> i32; fn dump_physmem_reserved(); fn __apply_alternatives(_: *mut AltInstr, _: *mut AltInstr, _: u32); fn stack_protector_apply_early(_: usize); fn randomize_within_range(_: usize, _: usize, _: usize, _: usize) -> usize;
}
#[repr(C)] pub struct Lowcore { pub last_update_clock: u64, pub current_pid: u64, pub lpp: u64, pub vmcore_info: usize }
#[repr(C)] pub struct Psw { pub mask: u64, pub addr: u64 }
#[repr(C)] pub struct AltInstr { _private: [u8; 0] }

fn detect_machine_type() { unsafe { let vmms = sysinfo_page.as_mut_ptr(); if stsi(core::ptr::null_mut(),0,0,0) <= 2 { set_machine_feature(MFEATURE_LPAR); return; } if stsi(vmms.cast(),3,2,2) != 0 { return; } /* vmms->count check is supplied by the ABI type. */ } }
fn detect_diag288() { unsafe { static CMD: [u8;5] = [0xc2,0xc5,0xc7,0xc9,0xd5]; let action = if machine_is_vm() { CMD.as_ptr() as usize } else { LPARWDT_RESTART }; let len = if machine_is_vm() { CMD.len() } else { 0 }; if __diag288(WDT_FUNC_INIT,MIN_INTERVAL,action,len) != 0 { return; } __diag288(WDT_FUNC_CANCEL,0,0,0); set_machine_feature(MFEATURE_DIAG288); } }
fn detect_diag9c() { unsafe { let _cpu=stap(); let mut rc=1; core::arch::asm!("diag {0},r0,0x9c", "0:", "1:", out(reg) _cpu, inout(reg) rc, options(nostack)); if rc==0 { set_machine_feature(MFEATURE_DIAG9C); } } }
fn reset_tod_clock() { unsafe { let mut clk=TodClock{tod:0}; if store_tod_clock_ext_cc(&mut clk)==0{return;} if set_tod_clock(TOD_UNIX_EPOCH)!=0 || store_tod_clock_ext_cc(&mut clk)!=0 {disabled_wait();} tod_clock_base=TodClock{tod:0}; tod_clock_base.tod=TOD_UNIX_EPOCH; (*get_lowcore()).last_update_clock=TOD_UNIX_EPOCH; } }
fn detect_facilities() { unsafe { if cpu_has_edat1(){local_ctl_set_bit(0,CR0_EDAT_BIT);} page_noexec_mask=usize::MAX; segment_noexec_mask=usize::MAX; region_noexec_mask=usize::MAX; if !cpu_has_nx(){page_noexec_mask &= !_PAGE_NOEXEC; segment_noexec_mask &= !_SEGMENT_ENTRY_NOEXEC; region_noexec_mask &= !_REGION_ENTRY_NOEXEC;} if test_facility(153){set_machine_feature(MFEATURE_PCI_MIO);} reset_tod_clock(); if test_facility(139)&&((tod_clock_base.tod>>63)!=0){set_machine_feature(MFEATURE_SCC);clock_comparator_max=(usize::MAX as u64)>>1;local_ctl_set_bit(0,CR0_CLOCK_COMPARATOR_SIGN_BIT);} if test_facility(50)&&test_facility(73){set_machine_feature(MFEATURE_TX);local_ctl_set_bit(0,CR0_TRANSACTIONAL_EXECUTION_BIT);} if cpu_has_vx(){local_ctl_set_bit(0,CR0_VECTOR_BIT);} } }
fn cmma_test_essa()->i32 { unsafe { let mut tmp=0usize; let mut rc=1; core::arch::asm!(".insn rrf,0xb9ab0000,{0},{0},{1},0", inout(reg) tmp, const ESSA_GET_STATE, inout(reg) rc); rc } }
fn cmma_init(){unsafe{if cmma_flag==0{return;}if cmma_test_essa()!=0{cmma_flag=0;return;}if test_facility(147){cmma_flag=2;}}}
fn setup_lpp_local(){unsafe{(*get_lowcore()).current_pid=0;(*get_lowcore()).lpp=LPP_MAGIC;if test_facility(40){lpp(&mut (*get_lowcore()).lpp);}}}

fn rescue_initrd(min:usize,max:usize){unsafe{let(mut addr,mut size)=(0,0);if !get_physmem_reserved(RR_INITRD,&mut addr,&mut size){return;}if addr>=min&&addr+size<=max{return;}let old_addr=addr;physmem_free(RR_INITRD);addr=physmem_alloc_or_die(RR_INITRD,size,0);core::ptr::copy(addr as *const u8, old_addr as *mut u8, size);}}
fn copy_bootdata(){unsafe{if (&__boot_data_end as *const _ as usize)-(&__boot_data_start as *const _ as usize)!=vmlinux.bootdata_size{boot_panic(b".boot.data section size mismatch\0".as_ptr());}core::ptr::copy_nonoverlapping(&__boot_data_start as *const _,vmlinux.bootdata_off as *mut u8,vmlinux.bootdata_size);if (&__boot_data_preserved_end as *const _ as usize)-(&__boot_data_preserved_start as *const _ as usize)!=vmlinux.bootdata_preserved_size{boot_panic(b".boot.preserved.data section size mismatch\0".as_ptr());}core::ptr::copy_nonoverlapping(&__boot_data_preserved_start as *const _,vmlinux.bootdata_preserved_off as *mut u8,vmlinux.bootdata_preserved_size);}}
fn kaslr_adjust_relocs(min:usize,max:usize,offset:usize,phys_offset:usize){unsafe{let mut p=&_vmlinux_relocs_64_start as *const _ as *mut i32;let end=&_vmlinux_relocs_64_end as *const _ as *mut i32;while p<end{let loc=(*p as isize as usize).wrapping_add(phys_offset);if loc<min||loc>max{boot_panic(b"64-bit relocation outside of kernel!\0".as_ptr());}*(loc as *mut u64)=(* (loc as *mut u64)).wrapping_add(offset as u64);p=p.add(1);}}}
fn kaslr_adjust_got(offset:usize){unsafe{let mut p=vmlinux.got_start as *mut u64;let end=vmlinux.got_end as *mut u64;while p<end{if *p!=0{*p=(*p).wrapping_add(offset as u64);}p=p.add(1);}}}
fn setup_ident_map_size(max:usize){unsafe{ident_map_size=if memory_limit!=0{core::cmp::min(max,memory_limit)}else{max};ident_map_size=core::cmp::min(ident_map_size,1usize<<MAX_PHYSMEM_BITS);boot_debug(b"Identity map size:   0x%016lx\0".as_ptr(),ident_map_size);}}
fn setup_kernel_memory_layout(_kernel_size:usize)->usize{unsafe{vmemmap_size=SECTION_ALIGN_UP(ident_map_size/PAGE_SIZE)*core::mem::size_of::<Page>();MODULES_END=round_down(__kaslr_offset,_SEGMENT_SIZE);MODULES_VADDR=MODULES_END-MODULES_LEN;VMALLOC_END=MODULES_VADDR;VMALLOC_START=VMALLOC_END-vmalloc_size;__memcpy_real_area=round_down(VMALLOC_START-MEMCPY_REAL_SIZE,PAGE_SIZE);__abs_lowcore=round_down(__memcpy_real_area-ABS_LOWCORE_MAP_SIZE,core::mem::size_of::<Lowcore>());vmemmap=__abs_lowcore as *mut Page;max_mappable=core::cmp::max(ident_map_size,MAX_DCSS_ADDR);_REGION1_SIZE}}
fn mem_safe_offset()->usize{unsafe{&_compressed_start as *const _ as usize}}
fn deploy_kernel(output:*mut core::ffi::c_void){unsafe{let start=&_compressed_start as *const _ as *mut u8;if output==start.cast(){return;}core::ptr::copy(start,output.cast(),vmlinux.image_size);core::ptr::write_bytes(start,0,vmlinux.image_size);}}
fn clear_bss_section(kernel_start:usize){unsafe{core::ptr::write_bytes((kernel_start+vmlinux.image_size) as *mut u8,0,vmlinux.bss_size);}}
fn setup_vmalloc_size(){unsafe{if vmalloc_size_set!=0{return;}vmalloc_size=core::cmp::max(round_up(ident_map_size/8,_SEGMENT_SIZE),vmalloc_size);}}
fn kaslr_adjust_vmlinux_info(offset:isize){unsafe{vmlinux.bootdata_off=(vmlinux.bootdata_off as isize+offset)as usize;vmlinux.bootdata_preserved_off=(vmlinux.bootdata_preserved_off as isize+offset)as usize;vmlinux.got_start=(vmlinux.got_start as isize+offset)as usize;vmlinux.got_end=(vmlinux.got_end as isize+offset)as usize;vmlinux.init_mm_off=(vmlinux.init_mm_off as isize+offset)as usize;vmlinux.swapper_pg_dir_off=(vmlinux.swapper_pg_dir_off as isize+offset)as usize;vmlinux.invalid_pg_dir_off=(vmlinux.invalid_pg_dir_off as isize+offset)as usize;vmlinux.alt_instructions=(vmlinux.alt_instructions as isize+offset)as usize;vmlinux.alt_instructions_end=(vmlinux.alt_instructions_end as isize+offset)as usize;}}

pub unsafe fn startup_kernel(){ let vmlinux_size=vmlinux.image_size+vmlinux.bss_size; let mut text_lma=0usize; let mut amode31_lma=0usize; setup_lpp_local();store_ipl_parmblock();uv_query_info();setup_boot_command_line();parse_boot_command_line();let nokaslr_text_lma=ALIGN(mem_safe_offset(),_SEGMENT_SIZE);let safe_addr=PAGE_ALIGN(nokaslr_text_lma+vmlinux_size);physmem_reserve(RR_DECOMPRESSOR,0,safe_addr);oldmem_data.start=parmarea.oldmem_base;oldmem_data.size=parmarea.oldmem_size;read_ipl_report();sclp_early_read_info();sclp_early_detect_machine_features();detect_facilities();detect_diag9c();detect_machine_type();detect_diag288();cmma_init();sanitize_prot_virt_host();let max_physmem_end=detect_max_physmem_end();setup_ident_map_size(max_physmem_end);setup_vmalloc_size();let asce_limit=setup_kernel_memory_layout(TEXT_OFFSET+vmlinux_size);physmem_set_usable_limit(ident_map_size);detect_physmem_online_ranges(max_physmem_end);save_ipl_cert_comp_list();rescue_initrd(safe_addr,ident_map_size);let large=__kaslr_offset&!_SEGMENT_MASK;if kaslr_enabled(){text_lma=randomize_within_range(vmlinux_size+large,_SEGMENT_SIZE,TEXT_OFFSET,ident_map_size);}if text_lma==0{text_lma=nokaslr_text_lma;}text_lma|=large;__kaslr_offset_phys=text_lma-TEXT_OFFSET;kaslr_adjust_vmlinux_info(__kaslr_offset_phys as isize);physmem_reserve(RR_VMLINUX,text_lma,vmlinux_size);deploy_kernel(text_lma as *mut core::ffi::c_void);physmem_reserve(RR_DECOMPRESSOR,0,&_decompressor_end as *const _ as usize);if kaslr_enabled(){amode31_lma=randomize_within_range(vmlinux.amode31_size,PAGE_SIZE,&_decompressor_end as *const _ as usize,SZ_2G);}if amode31_lma==0{amode31_lma=text_lma-vmlinux.amode31_size;}physmem_reserve(RR_AMODE31,amode31_lma,vmlinux.amode31_size);clear_bss_section(text_lma);kaslr_adjust_relocs(text_lma,text_lma+vmlinux.image_size,__kaslr_offset,__kaslr_offset_phys);kaslr_adjust_got(__kaslr_offset);setup_vmem(__kaslr_offset,__kaslr_offset+TEXT_OFFSET+vmlinux_size,asce_limit);dump_physmem_reserved();copy_bootdata();__apply_alternatives((&_vmlinux_info.alt_instructions as *const _ as *mut AltInstr),(&_vmlinux_info.alt_instructions_end as *const _ as *mut AltInstr),ALT_CTX_EARLY);stack_protector_apply_early(text_lma);(*get_lowcore()).vmcore_info=if __kaslr_offset_phys!=0{__kaslr_offset_phys|1}else{0};let psw=Psw{addr:(__kaslr_offset+vmlinux.entry)as u64,mask:PSW_KERNEL_BITS};jump_to_kernel(&psw); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
