// SPDX-License-Identifier: GPL-2.0
// S390 architecture-dependent initialization. C headers and configuration
// symbols are supplied by the surrounding kernel translation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut _samode31: u8; static mut _eamode31: u8;
    static mut _stext_amode31: u8; static mut _etext_amode31: u8;
    static mut _start_amode31_ex_table: exception_table_entry;
    static mut _stop_amode31_ex_table: exception_table_entry;
    static mut _text: u8; static mut _etext: u8; static mut _edata: u8;
    static mut __bss_start: u8; static mut __bss_stop: u8; static mut _end: u8;
    static mut boot_command_line: [u8; COMMAND_LINE_SIZE];
    static mut oldmem_data: oldmem_data;
    static mut physmem_info: physmem_info;
    static mut vm_layout: vm_layout;
    static mut sclp: sclp;
    static mut ipl_block: ipl_block;
    static mut memblock: memblock;
    static mut crashk_res: resource;
    static mut initrd_start: usize; static mut initrd_end: usize;
    static mut high_memory: *mut u8; static mut max_pfn: usize; static mut max_low_pfn: usize;
    static mut ROOT_DEV: u32; static mut elfcorehdr_addr: u64;
    static mut pcpu_devices: u8; static mut init_task: u8;
    static mut early_ipl_comp_list_addr: usize; static mut early_ipl_comp_list_size: usize;
    static mut ipl_cert_list_addr: usize; static mut ipl_cert_list_size: usize;
    static mut restart_stack: *mut core::ffi::c_void;
}

type u32_t = u32; type u64_t = u64; type phys_addr_t = usize;
const COMMAND_LINE_SIZE: usize = 4096;

#[repr(C)] pub struct exception_table_entry { _x: [u8; 0] }
#[repr(C)] pub struct oldmem_data { pub start: usize, pub size: usize }
#[repr(C)] pub struct physmem_info { pub reserved: [reserved_range; 16], pub info_source: i8 }
#[repr(C)] pub struct reserved_range { pub start: usize, pub size: usize }
#[repr(C)] pub struct vm_layout { _x: [u8; 0] }
#[repr(C)] pub struct sclp { pub has_vt220: bool, pub has_linemode: bool, pub has_diag318: bool, pub hsa_size: usize }
#[repr(C)] pub struct ipl_block { pub hdr: ipl_hdr }
#[repr(C)] pub struct ipl_hdr { pub flags: u32 }
#[repr(C)] pub struct memblock { pub memory: memblock_type }
#[repr(C)] pub struct memblock_type { pub total_size: usize, pub regions: *mut memblock_region }
#[repr(C)] pub struct memblock_region { pub size: usize }
#[repr(C)] pub struct resource { pub name: *const u8, pub start: usize, pub end: usize, pub flags: u64 }
#[repr(C)] pub struct lowcore { _x: [u8; 0] }
#[repr(C)] pub struct sysinfo_3_2_2 { pub count: u32, pub vm: [u8; 0] }
#[repr(C)] pub struct ipl_rb_component_entry { pub addr: u64, pub len: u64, pub flags: u32 }
#[repr(C)] pub union ctlreg2 { pub reg: u64, pub ducto: u64 }
#[repr(C)] pub union ctlreg5 { pub reg: u64, pub pasteo: u64 }
#[repr(C)] pub union ctlreg15 { pub reg: u64, pub lsea: u64 }

pub static mut console_mode: u32 = 0;
pub static mut console_devno: u32 = u32::MAX;
pub static mut console_irq: u32 = u32::MAX;
pub static mut __samode31: *mut u8 = unsafe { &_samode31 as *const _ as *mut _ };
pub static mut __eamode31: *mut u8 = unsafe { &_eamode31 as *const _ as *mut _ };
pub static mut __stext_amode31: *mut u8 = unsafe { &_stext_amode31 as *const _ as *mut _ };
pub static mut __etext_amode31: *mut u8 = unsafe { &_etext_amode31 as *const _ as *mut _ };
pub static mut __start_amode31_ex_table: *mut exception_table_entry = unsafe { &_start_amode31_ex_table as *const _ as *mut _ };
pub static mut __stop_amode31_ex_table: *mut exception_table_entry = unsafe { &_stop_amode31_ex_table as *const _ as *mut _ };
static mut __ctl_duct_amode31: [u32; 16] = [0; 16];
static mut __ctl_aste_amode31: [u64; 8] = [0, u64::MAX, 0, 0, 0, 0, 0, 0];
static mut __ctl_duald_amode31: [u32; 32] = [0x80000000,0,0,0, 0x80000000,0,0,0, 0x80000000,0,0,0, 0x80000000,0,0,0, 0x80000000,0,0,0, 0x80000000,0,0,0, 0x80000000,0,0,0, 0x80000000,0,0,0];
static mut __ctl_linkage_stack_amode31: [u32; 8] = [0,0,0x89000000,0,0,0,0x8a000000,0];
static mut __ctl_aste: *mut u64 = unsafe { __ctl_aste_amode31.as_mut_ptr() };
static mut __ctl_duald: *mut u32 = unsafe { __ctl_duald_amode31.as_mut_ptr() };
static mut __ctl_linkage_stack: *mut u32 = unsafe { __ctl_linkage_stack_amode31.as_mut_ptr() };
static mut __ctl_duct: *mut u32 = unsafe { __ctl_duct_amode31.as_mut_ptr() };
pub static mut max_mappable: usize = 0; pub static mut __kaslr_enabled: i32 = 0;
pub static mut zlib_dfltcc_support: u32 = 0; pub static mut stfle_fac_list: [u64;16] = [0;16];
pub static mut boot_rb: [u8; 8192] = [0;8192]; pub static mut boot_earlyprintk: bool = false;
pub static mut boot_rb_off: usize = 0; pub static mut bootdebug_filter: [u8;128] = [0;128];
pub static mut bootdebug: bool = false; pub static mut VMALLOC_START: usize = 0; pub static mut VMALLOC_END: usize = 0;
pub static mut vmemmap: *mut u8 = core::ptr::null_mut(); pub static mut vmemmap_size: usize = 0;
pub static mut MODULES_VADDR: usize = 0; pub static mut MODULES_END: usize = 0;
pub static mut lowcore_ptr: [*mut lowcore; 256] = [core::ptr::null_mut(); 256];
pub static mut mio_wb_bit_mask: usize = 0;

extern "C" {
    fn simple_strtoul(*const u8, *mut *const u8, u32) -> usize; fn strcmp(*const u8,*const u8)->i32;
    fn add_preferred_console(*const u8,u32,*const u8)->i32; fn machine_is_vm()->bool; fn machine_is_kvm()->bool; fn machine_is_lpar()->bool;
    fn cpcmd(*const u8,*mut u8,usize,*mut u8); fn strstr(*const u8,*const u8)->*mut u8; fn str_has_prefix(*const u8,*const u8)->bool;
    fn in_interrupt()->bool; fn in_atomic()->bool; fn console_unblank(); fn _machine_restart(*mut u8); fn _machine_halt(); fn _machine_power_off();
    fn __vmalloc_node(usize,usize,u32,i32,*const u8)->*mut u8; fn kmemleak_not_leak(*mut u8); fn vfree(*mut u8);
    fn memblock_alloc_or_panic(usize,usize)->*mut u8; fn memblock_alloc_low(usize,usize)->*mut lowcore; fn panic(*const u8,...)->!;
    fn get_lowcore()->*mut lowcore; fn get_abs_lowcore()->*mut lowcore; fn put_abs_lowcore(*mut lowcore); fn abs_lowcore_map(i32,*mut lowcore,bool)->i32;
    fn storage_key_init_range(usize,usize); fn psw_set_key(u32); fn memmove(*mut u8,*const u8,usize); fn memset(*mut u8,i32,usize);
    fn stsi(*mut sysinfo_3_2_2,u32,u32,u32)->i32; fn add_device_randomness(*const u8,usize); fn cpacf_query_func(u32,u32)->bool;
    fn diag_stat_inc(u32); fn boot_rb_foreach(fn(*const u8)); fn printk_get_level(*const u8)->i32; fn printk(*const u8,...);
    fn skip_timestamp(*const u8)->*const u8; fn printk_skip_level(*const u8)->*const u8; fn bootdebug_filter_match(*const u8)->bool;
}

pub unsafe fn condev_setup(s: *mut u8) -> i32 { let v = simple_strtoul(s, core::ptr::null_mut(), 0); if v < 65536 { console_devno=v as u32; console_irq=u32::MAX; } 1 }
unsafe fn set_preferred_console() { if CONSOLE_IS_3215 || CONSOLE_IS_SCLP { add_preferred_console(b"ttyS\0".as_ptr(),0,core::ptr::null()); } else if CONSOLE_IS_3270 { add_preferred_console(b"tty3270\0".as_ptr(),0,core::ptr::null()); } else if CONSOLE_IS_VT220 { add_preferred_console(b"ttysclp\0".as_ptr(),0,core::ptr::null()); } else if CONSOLE_IS_HVC { add_preferred_console(b"hvc\0".as_ptr(),0,core::ptr::null()); } }
pub unsafe fn conmode_setup(_s:*mut u8)->i32 { set_preferred_console(); 1 }
pub unsafe fn machine_restart(command:*mut u8) { if ((!in_interrupt()&&!in_atomic())||oops_in_progress) { console_unblank(); } _machine_restart(command); }
pub unsafe fn machine_halt() { if !in_interrupt()||oops_in_progress { console_unblank(); } _machine_halt(); }
pub unsafe fn machine_power_off() { if !in_interrupt()||oops_in_progress { console_unblank(); } _machine_power_off(); }
pub static mut pm_power_off: Option<unsafe extern "C" fn()> = Some(machine_power_off);
pub unsafe fn stack_alloc()->usize { let s=__vmalloc_node(THREAD_SIZE,THREAD_SIZE,THREADINFO_GFP,NUMA_NO_NODE,core::ptr::null()); kmemleak_not_leak(s); s as usize }
pub unsafe fn stack_free(s:usize) { vfree(s as *mut u8); }
unsafe fn stack_alloc_early()->usize { memblock_alloc_or_panic(THREAD_SIZE,THREAD_SIZE) as usize }

// The remaining initialization is kept in direct source order; external kernel
// structures and helper symbols intentionally remain unresolved dependencies.
unsafe fn reserve_pgtables() { let mut start=0; let mut end=0; for_each_physmem_reserved_type_range(RR_VMEM,&mut start,&mut end); memblock_reserve(start,end-start); }
unsafe fn reserve_initrd() { let mut a=0; let mut s=0; if get_physmem_reserved(RR_INITRD,&mut a,&mut s) { initrd_start=__va(a) as usize; initrd_end=initrd_start+s; memblock_reserve(a,s); } }
unsafe fn reserve_certificate_list() { if ipl_cert_list_addr!=0 { memblock_reserve(ipl_cert_list_addr,ipl_cert_list_size); } }
unsafe fn reserve_physmem_info() { let(mut a,mut s)=(0,0); if get_physmem_reserved(RR_MEM_DETECT_EXT,&mut a,&mut s){memblock_reserve(a,s);} }
unsafe fn free_physmem_info() { let(mut a,mut s)=(0,0); if get_physmem_reserved(RR_MEM_DETECT_EXT,&mut a,&mut s){memblock_phys_free(a,s);} }
unsafe fn setup_high_memory() { high_memory=__va(ident_map_size); }
unsafe fn setup_memory_end() { max_pfn=PFN_DOWN(ident_map_size); max_low_pfn=max_pfn; }
unsafe fn setup_memory() { let(mut i,mut s,mut e)=(0,0,0); for_each_mem_range(&mut i,&mut s,&mut e){storage_key_init_range(s,e);} psw_set_key(PAGE_DEFAULT_KEY); }
unsafe fn setup_randomness() { let p=memblock_alloc_or_panic(PAGE_SIZE,PAGE_SIZE) as *mut sysinfo_3_2_2; if stsi(p,3,2,2)==0 && (*p).count!=0 { add_device_randomness((*p).vm.as_ptr(), core::mem::size_of::<u8>()*(*p).count as usize); } memblock_free(p as *mut u8,PAGE_SIZE); }
unsafe fn log_component_list() { if early_ipl_comp_list_addr==0{return;} let mut p=__va(early_ipl_comp_list_addr) as *mut ipl_rb_component_entry; let e=(p as usize+early_ipl_comp_list_size) as *mut ipl_rb_component_entry; while p<e { p=p.add(1); } }
unsafe fn print_rb_entry(buf:*const u8) { let level=printk_get_level(buf); let b=skip_timestamp(printk_skip_level(buf)); if level==KERN_DEBUG && (!bootdebug||!bootdebug_filter_match(b)){return;} printk(b"%s\0".as_ptr(),b); }
pub unsafe fn setup_arch(cmdline_p:*mut *mut u8) { *cmdline_p=boot_command_line.as_mut_ptr(); setup_ipl(); reserve_pgtables(); reserve_initrd(); reserve_certificate_list(); reserve_physmem_info(); memblock_add_physmem_info(); free_physmem_info(); setup_memory_end(); setup_high_memory(); setup_memory(); setup_randomness(); }
pub unsafe fn arch_cpu_finalize_init() { sclp_init(); }

// Configuration-provided constants and functions are intentionally referenced
// rather than implemented here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
