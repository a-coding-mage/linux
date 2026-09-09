// SPDX-License-Identifier: GPL-2.0
/* Direct translation of setup.c; kernel dependencies are supplied externally. */

const SMBIOS_BIOSSIZE_OFFSET: usize = 0x09;
const SMBIOS_BIOSEXTERN_OFFSET: usize = 0x13;
const SMBIOS_FREQLOW_OFFSET: usize = 0x16;
const SMBIOS_FREQHIGH_OFFSET: usize = 0x17;
const SMBIOS_FREQLOW_MASK: usize = 0xff;
const SMBIOS_CORE_PACKAGE_OFFSET: usize = 0x23;
const SMBIOS_THREAD_PACKAGE_OFFSET: usize = 0x25;
const SMBIOS_THREAD_PACKAGE_2_OFFSET: usize = 0x2e;
const LOONGSON_EFI_ENABLE: usize = 1 << 3;

extern "C" {
    static mut fw_arg0: usize;
    static mut fw_arg1: usize;
    static mut fw_arg2: usize;
    static mut cpu_data: [cpuinfo_loongarch; NR_CPUS];
    static mut b_info: loongson_board_info;
    static mut init_command_line: [u8; COMMAND_LINE_SIZE];
    static mut cpu_clock_freq: u64;
    static mut loongson_sysconf: loongson_sysconf_t;
    static mut boot_command_line: [u8; COMMAND_LINE_SIZE];
    static mut max_pfn: usize;
    static mut max_low_pfn: usize;
    static mut elfcorehdr_addr: usize;
    static mut elfcorehdr_size: usize;
    static mut num_processors: i32;
    static mut disabled_cpus: i32;
    static mut nr_cpu_ids: i32;
    static mut initial_boot_params: *mut core::ffi::c_void;
    static mut memblock: memblock_t;
    static mut iomem_resource: resource;
    static __dtb_start: core::ffi::c_void;
    static _text: u8;
    static _etext: u8;
    static _edata: u8;
    static __bss_start: u8;
    static __bss_stop: u8;
    static __nosave_begin: u8;
    static __nosave_end: u8;
}

#[repr(C)] pub struct cpuinfo_loongarch { _private: [u8; 0] }
#[repr(C)] pub struct loongson_board_info { pub bios_size: usize, pub bios_vendor: *const u8, pub bios_version: *const u8, pub bios_release_date: *const u8, pub board_vendor: *const u8, pub board_name: *const u8 }
#[repr(C)] pub struct loongson_sysconf_t { pub cpuname: *mut core::ffi::c_void, pub cores_per_package: u16 }
#[repr(C)] pub struct resource { pub name: *const u8, pub start: usize, pub end: usize, pub flags: usize }
#[repr(C)] pub struct memblock_t { pub memory: memblock_type }
#[repr(C)] pub struct memblock_type { pub cnt: usize }
#[repr(C)] pub struct dmi_header { pub type_: u8, pub length: u8 }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct device_node { pub fwnode: fwnode_handle }
#[repr(C)] pub struct logic_pio_hwaddr { pub fwnode: *mut fwnode_handle, pub size: usize, pub hw_start: usize, pub flags: usize, pub io_start: usize }
#[repr(C)] pub struct of_range { pub flags: usize, pub cpu_addr: u64, pub bus_addr: u64, pub size: u64 }
#[repr(C)] pub struct of_range_parser { _private: [u8; 0] }
#[repr(C)] pub struct memblock_region { _private: [u8; 0] }

const NR_CPUS: usize = 1; const COMMAND_LINE_SIZE: usize = 2048;
const IORESOURCE_TYPE_BITS: usize = 0xf; const IORESOURCE_IO: usize = 1; const IORESOURCE_MEM: usize = 2;
const IORESOURCE_SYSTEM_RAM: usize = 0; const IORESOURCE_BUSY: usize = 0; const GFP_ATOMIC: usize = 0;
const LOGIC_PIO_CPU_MMIO: usize = 0; const PAGE_SIZE: usize = 4096; const PAGE_KERNEL: usize = 0;
const SWIOTLB_VERBOSE: usize = 0; const ARCH_PFN_OFFSET: usize = 0; const HIGHMEM_START: usize = 0;

extern "C" {
    fn alternative_instructions(); fn dmi_get_system_info(x: i32) -> *const u8; fn dmi_walk(f: unsafe extern "C" fn(*const dmi_header, *mut core::ffi::c_void), p: *mut core::ffi::c_void);
    fn strlen(p: *const u8) -> usize; fn memcmp(a: *const u8,b: *const u8,n: usize)->i32; fn strcmp(a:*const u8,b:*const u8)->i32;
    fn pr_info(...); fn pr_warn(...); fn pr_err(...); fn memparse(p:*const u8,end:*mut *const u8)->usize;
    fn memblock_enforce_memory_limit(x:usize); fn memblock_start_of_DRAM()->usize; fn memblock_end_of_DRAM()->usize; fn memblock_remove(a:usize,b:usize); fn memblock_add(a:usize,b:usize); fn memblock_add_node(a:usize,b:usize,c:usize,d:usize);
    fn pa_to_nid(x:usize)->usize; fn acpi_os_get_root_pointer()->usize; fn fdt_check_header(p:*const core::ffi::c_void)->i32; fn efi_fdt_pointer()->*mut core::ffi::c_void; fn early_init_dt_scan(p:*mut core::ffi::c_void,a:usize); fn early_init_fdt_reserve_self();
    fn parse_crashkernel(...)->i32; fn reserve_crashkernel_generic(a:u64,b:u64,c:u64,d:bool); fn is_kdump_kernel()->bool; fn memblock_is_region_reserved(a:usize,b:usize)->bool; fn memblock_reserve(a:usize,b:usize);
    fn strscpy(a:*mut u8,b:*const u8,n:usize); fn strlcat(a:*mut u8,b:*const u8,n:usize); fn strstr(a:*const u8,b:*const u8)->*const u8; fn early_init_fdt_scan_reserved_mem(); fn unflatten_and_copy_device_tree(); fn dmi_setup(); fn efi_runtime_init();
    fn memblock_is_region_memory(a:usize,b:usize)->bool; fn memblock_set_bottom_up(x:bool); fn swiotlb_init(x:bool,y:usize); fn dma_contiguous_reserve(x:usize); fn register_nosave_region(a:usize,b:usize); fn memblock_dump_all(); fn early_memtest(a:usize,b:usize); fn memblock_alloc_or_panic(a:usize,b:usize)->*mut resource; fn request_resource(a:*mut resource,b:*mut resource)->i32; fn memblock_is_nomap(a:*mut memblock_region)->bool; fn memblock_region_memory_base_pfn(a:*mut memblock_region)->usize; fn memblock_region_memory_end_pfn(a:*mut memblock_region)->usize; fn memblock_region_reserved_base_pfn(a:*mut memblock_region)->usize; fn memblock_region_reserved_end_pfn(a:*mut memblock_region)->usize; fn resource_size(a:*mut resource)->usize;
    fn kzalloc_obj()->*mut logic_pio_hwaddr; fn kfree(a:*mut logic_pio_hwaddr); fn logic_pio_register_range(a:*mut logic_pio_hwaddr)->i32; fn logic_pio_unregister_range(a:*mut logic_pio_hwaddr); fn vmap_page_range(a:usize,b:usize,c:usize,d:usize); fn pgprot_device(x:usize)->usize; fn acpi_add_early_pio(); fn reserve_region_with_split(a:*mut resource,b:usize,c:usize,d:*const u8); fn cpu_probe(); fn unwind_init(); fn set_current(x:*mut core::ffi::c_void); fn init_environ(); fn efi_init(); fn memblock_init(); fn pagetable_init(); fn jump_label_init(); fn parse_early_param(); fn reserve_initrd_mem(); fn platform_init(); fn resource_init(); fn plat_smp_setup(); fn kasan_init(); fn set_cpu_possible(a:i32,b:bool); fn set_cpu_present(a:i32,b:bool); fn set_nr_cpu_ids(a:i32);
}

static mut wc_enabled: bool = false;
static mut usermem: i32 = 0;
static mut num_standard_resources: i32 = 0;
static mut standard_resources: *mut resource = core::ptr::null_mut();
static mut code_resource: resource = resource { name: b"Kernel code\0".as_ptr(), start:0,end:0,flags:0 };
static mut data_resource: resource = resource { name: b"Kernel data\0".as_ptr(), start:0,end:0,flags:0 };
static mut bss_resource: resource = resource { name: b"Kernel bss\0".as_ptr(), start:0,end:0,flags:0 };
static dmi_empty_string: [u8; 9] = *b"        \0";

pub unsafe extern "C" fn get_system_type() -> *const u8 { b"generic-loongson-machine\0".as_ptr() }
pub unsafe extern "C" fn arch_cpu_finalize_init() { alternative_instructions(); }

unsafe fn dmi_string_parse(dm:*const dmi_header, mut s:u8)->*const u8 { let mut bp=(dm as *const u8).add((*dm).length as usize); if s!=0 { s-=1; while s>0 && *bp!=0 { bp=bp.add(strlen(bp)+1); s-=1; } if *bp!=0 { let len=strlen(bp)+1; let cmp=if len>8{8}else{len}; if memcmp(bp,dmi_empty_string.as_ptr(),cmp)==0{return dmi_empty_string.as_ptr()} return bp; }} b"\0".as_ptr() }
unsafe extern "C" fn parse_cpu_table(dm:*const dmi_header) { let p=dm as *const u8; let f=((*p.add(SMBIOS_FREQHIGH_OFFSET) as u64)<<8)+((*p.add(SMBIOS_FREQLOW_OFFSET) as u64)&SMBIOS_FREQLOW_MASK as u64); cpu_clock_freq=f*1_000_000; loongson_sysconf.cpuname=dmi_string_parse(dm,*p.add(16) as u8) as *mut _; loongson_sysconf.cores_per_package=*p.add(SMBIOS_THREAD_PACKAGE_OFFSET) as u16; if (*dm).length>=0x30 && loongson_sysconf.cores_per_package==0xff { loongson_sysconf.cores_per_package=*(p.add(SMBIOS_THREAD_PACKAGE_2_OFFSET) as *const u16); } }
unsafe extern "C" fn parse_bios_table(dm:*const dmi_header) { b_info.bios_size=(*((dm as *const u8).add(SMBIOS_BIOSSIZE_OFFSET)) as usize+1)<<6; }
unsafe extern "C" fn find_tokens(dm:*const dmi_header,_:*mut core::ffi::c_void) { match (*dm).type_ {0=>parse_bios_table(dm),4=>parse_cpu_table(dm),_=>{}} }
unsafe fn smbios_parse() { b_info.bios_vendor=dmi_get_system_info(0); b_info.bios_version=dmi_get_system_info(1); b_info.bios_release_date=dmi_get_system_info(2); b_info.board_vendor=dmi_get_system_info(3); b_info.board_name=dmi_get_system_info(4); dmi_walk(find_tokens,core::ptr::null_mut()); }
unsafe extern "C" fn setup_writecombine(p:*mut u8)->i32 { if strcmp(p,b"on\0".as_ptr())==0 {wc_enabled=true} else if strcmp(p,b"off\0".as_ptr())==0 {wc_enabled=false} else {pr_warn();} 0 }
unsafe extern "C" fn early_parse_mem(_p:*mut u8)->i32 { 0 }
unsafe extern "C" fn arch_reserve_vmcore() {}
unsafe extern "C" fn arch_reserve_crashkernel() {}
unsafe extern "C" fn fdt_setup() {}
unsafe extern "C" fn bootcmdline_init(cmdline_p:*mut *mut u8) { *cmdline_p=boot_command_line.as_mut_ptr(); }
unsafe extern "C" fn check_kernel_sections_mem() {}
unsafe extern "C" fn arch_mem_init(_cmdline_p:*mut *mut u8) {}
unsafe extern "C" fn resource_init() {}
unsafe extern "C" fn add_legacy_isa_io(_f:*mut fwnode_handle,_s:usize,_z:usize)->i32 { 0 }
unsafe extern "C" fn arch_reserve_pio_range()->i32 { 0 }
unsafe extern "C" fn reserve_memblock_reserved_regions()->i32 { 0 }
unsafe extern "C" fn prefill_possible_map() {}
pub unsafe extern "C" fn setup_arch(cmdline_p:*mut *mut u8) { cpu_probe(); unwind_init(); set_current(core::ptr::null_mut()); init_environ(); efi_init(); fdt_setup(); memblock_init(); pagetable_init(); bootcmdline_init(cmdline_p); jump_label_init(); parse_early_param(); reserve_initrd_mem(); platform_init(); arch_mem_init(cmdline_p); resource_init(); plat_smp_setup(); kasan_init(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
