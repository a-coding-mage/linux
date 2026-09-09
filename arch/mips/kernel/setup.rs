/* Rust translation of setup.c. External kernel symbols are intentionally left
 * as dependencies supplied by the surrounding kernel translation. */

#[cfg(CONFIG_MIPS_ELF_APPENDED_DTB)]
#[link_section = ".appended_dtb"]
pub static mut __appended_dtb: [u8; 0x100000] = [0; 0x100000];

pub static mut cpu_data: [cpuinfo_mips; NR_CPUS] = [unsafe { core::mem::zeroed() }; NR_CPUS];
pub static mut mips_machtype: c_ulong = MACH_UNKNOWN;
static mut command_line: [c_char; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];
pub static mut arcs_cmdline: [c_char; COMMAND_LINE_SIZE] = [0; COMMAND_LINE_SIZE];

#[cfg(CONFIG_CMDLINE_BOOL)]
static builtin_cmdline: &[u8] = CONFIG_CMDLINE;
#[cfg(not(CONFIG_CMDLINE_BOOL))]
static builtin_cmdline: &[u8] = b"\0";

pub static mut mips_io_port_base: c_ulong = !0;
static mut code_resource: resource = resource { name: b"Kernel code\0".as_ptr() as *const c_char, ..unsafe { core::mem::zeroed() } };
static mut data_resource: resource = resource { name: b"Kernel data\0".as_ptr() as *const c_char, ..unsafe { core::mem::zeroed() } };
static mut bss_resource: resource = resource { name: b"Kernel bss\0".as_ptr() as *const c_char, ..unsafe { core::mem::zeroed() } };
pub static mut __kaslr_offset: c_ulong = 0;
static mut detect_magic: *mut core::ffi::c_void = detect_memory_region as *mut _;

#[cfg(CONFIG_MIPS_AUTO_PFN_OFFSET)]
pub static mut ARCH_PFN_OFFSET: c_ulong = 0;

pub unsafe fn detect_memory_region(start: phys_addr_t, sz_min: phys_addr_t, sz_max: phys_addr_t) {
    let dm = &mut detect_magic as *mut _ as *mut u8;
    let mut size = sz_min;
    while size < sz_max {
        if core::slice::from_raw_parts(dm, core::mem::size_of::<*mut core::ffi::c_void>()) ==
           core::slice::from_raw_parts(dm.add(size as usize), core::mem::size_of::<*mut core::ffi::c_void>()) { break; }
        size <<= 1;
    }
    pr_debug!("Memory: {}MB of RAM detected at 0x{:x} (min: {}MB, max: {}MB)\n", size / SZ_1M, start, sz_min / SZ_1M, sz_max / SZ_1M);
    memblock_add(start, size);
}

#[cfg(CONFIG_BLK_DEV_INITRD)]
unsafe fn rd_start_early(mut p: *mut c_char) -> c_int { let mut q = p; let start = memparse(p, &mut q); #[cfg(CONFIG_64BIT)] if start < XKPHYS { initrd_start = start as c_int as c_ulong; } initrd_start = start; initrd_end += start; 0 }
#[cfg(CONFIG_BLK_DEV_INITRD)]
unsafe fn rd_size_early(mut p: *mut c_char) -> c_int { initrd_end += memparse(p, &mut p); 0 }

#[cfg(CONFIG_BLK_DEV_INITRD)]
unsafe fn init_initrd() -> c_ulong {
    let mut end;
    if initrd_start == 0 || initrd_end <= initrd_start { initrd_start=0; initrd_end=0; return 0; }
    if initrd_start & !PAGE_MASK != 0 { pr_err!("initrd start must be page aligned\n"); initrd_start=0; initrd_end=0; return 0; }
    end = __pa(initrd_end); initrd_end = __va(end) as c_ulong; initrd_start = __va(__pa(initrd_start)) as c_ulong;
    if initrd_start < PAGE_OFFSET { pr_err!("initrd start < PAGE_OFFSET\n"); initrd_start=0; initrd_end=0; return 0; }
    ROOT_DEV = Root_RAM0; PFN_UP(end)
}
#[cfg(not(CONFIG_BLK_DEV_INITRD))]
unsafe fn init_initrd() -> c_ulong { 0 }

#[cfg(CONFIG_BLK_DEV_INITRD)]
unsafe fn maybe_bswap_initrd() {
    #[cfg(CONFIG_CPU_CAVIUM_OCTEON)] { let mut buf: u64; if !memcmp(initrd_start as *const _, b"070701".as_ptr() as *const _, 6) { return; } if decompress_method(initrd_start as *const u8, 8, core::ptr::null_mut()) != 0 { return; } buf=swab64p(initrd_start as *const u64); if !memcmp(&buf as *const _ as *const _, b"070701".as_ptr() as *const _, 6) || decompress_method(&buf as *const _ as *const u8, 8, core::ptr::null_mut()) != 0 { pr_info!("Byteswapped initrd detected\n"); let mut i=initrd_start; while i < ALIGN(initrd_end,8) { swab64s(i as *mut u64); i+=8; } } }
}
#[cfg(CONFIG_BLK_DEV_INITRD)]
unsafe fn finalize_initrd() { let size=initrd_end-initrd_start; if size==0 || __pa(initrd_end)>PFN_PHYS(max_low_pfn) { initrd_start=0; initrd_end=0; return; } maybe_bswap_initrd(); memblock_reserve(__pa(initrd_start),size); initrd_below_start_ok=1; }

unsafe fn bootmem_init() { init_initrd(); #[cfg(CONFIG_BLK_DEV_INITRD)] finalize_initrd(); }

static mut usermem: c_int = 0;
unsafe fn early_parse_mem(mut p: *mut c_char) -> c_int { if p.is_null(){pr_err!("mem parameter is empty, do nothing\n");return -EINVAL;} if usermem==0 {usermem=1; memblock_remove(memblock_start_of_DRAM(),memblock_end_of_DRAM()-memblock_start_of_DRAM());} let mut start=0; let size=memparse(p,&mut p); if *p as u8==b'@'{start=memparse(p.add(1),&mut p);} if IS_ENABLED(CONFIG_NUMA){memblock_add_node(start,size,pa_to_nid(start),MEMBLOCK_NONE)}else{memblock_add(start,size)} 0 }
unsafe fn early_parse_memmap(mut p:*mut c_char)->c_int { if p.is_null(){return -EINVAL;} if !strncmp(p,b"exactmap".as_ptr() as *const _,8){pr_err!("\"memmap=exactmap\" invalid on MIPS\n");return 0;} let old=p; let size=memparse(p,&mut p); if p==old{return -EINVAL;} let mut at=0; if *p as u8==b'@'{at=memparse(p.add(1),&mut p);memblock_add(at,size)}else if *p as u8==b'$'{at=memparse(p.add(1),&mut p);memblock_add(at,size);memblock_reserve(at,size)}else{return -EINVAL} if *p==0 {usermem=1;0}else{-EINVAL} }

unsafe fn check_kernel_sections_mem(){let start=__pa_symbol(& _text);let size=__pa_symbol(&_end)-start;if !memblock_is_region_memory(start,size){memblock_add(start,size);}}
unsafe fn bootcmdline_append(s:*const c_char,max:usize){if *s==0||max==0{return;}if boot_command_line[0]!=0{strlcat(boot_command_line.as_mut_ptr(),b" \0".as_ptr() as *const _,COMMAND_LINE_SIZE);}strlcat(boot_command_line.as_mut_ptr(),s,max);}
unsafe fn bootcmdline_init(){let mut dt=false;if IS_ENABLED(CONFIG_CMDLINE_OVERRIDE){strscpy(boot_command_line.as_mut_ptr(),builtin_cmdline.as_ptr() as *const _,COMMAND_LINE_SIZE);return;}if IS_ENABLED(CONFIG_MIPS_CMDLINE_BUILTIN_EXTEND){strscpy(boot_command_line.as_mut_ptr(),builtin_cmdline.as_ptr() as *const _,COMMAND_LINE_SIZE)}else{boot_command_line[0]=0;}if IS_ENABLED(CONFIG_MIPS_CMDLINE_DTB_EXTEND)||!dt{bootcmdline_append(arcs_cmdline.as_ptr(),COMMAND_LINE_SIZE);}if IS_ENABLED(CONFIG_CMDLINE_BOOL)&&!IS_ENABLED(CONFIG_MIPS_CMDLINE_BUILTIN_EXTEND){bootcmdline_append(builtin_cmdline.as_ptr() as *const _,COMMAND_LINE_SIZE);}}

unsafe fn arch_mem_init(cmdline_p:*mut *mut c_char){plat_mem_setup();memblock_set_bottom_up(true);bootcmdline_init();strscpy(command_line.as_mut_ptr(),boot_command_line.as_ptr(),COMMAND_LINE_SIZE);*cmdline_p=command_line.as_mut_ptr();parse_early_param();check_kernel_sections_mem();early_init_fdt_reserve_self();early_init_fdt_scan_reserved_mem();bootmem_init();memblock_set_current_limit(PFN_PHYS(max_low_pfn));mips_reserve_vmcore();mips_parse_crashkernel();device_tree_init();plat_swiotlb_setup();dma_contiguous_reserve(PFN_PHYS(max_low_pfn));memblock_reserve(__pa_symbol(&__nosave_begin),__pa_symbol(&__nosave_end)-__pa_symbol(&__nosave_begin));early_memtest(PFN_PHYS(ARCH_PFN_OFFSET),PFN_PHYS(max_low_pfn));}

pub unsafe fn setup_arch(cmdline_p:*mut *mut c_char){cpu_probe();mips_cm_probe();prom_init();setup_early_fdc_console();#[cfg(CONFIG_EARLY_PRINTK)] setup_early_printk();cpu_report();if IS_ENABLED(CONFIG_CPU_R4X00_BUGS64){check_bugs64_early();}arch_mem_init(cmdline_p);dmi_setup();resource_init();plat_smp_setup();prefill_possible_map();cpu_cache_init();pagetable_init();memblock_dump_all();setup_rng_seed();}
pub static mut kernelsp:[c_ulong;NR_CPUS]=[0;NR_CPUS];pub static mut fw_arg0:c_ulong=0;pub static mut fw_arg1:c_ulong=0;pub static mut fw_arg2:c_ulong=0;pub static mut fw_arg3:c_ulong=0;
unsafe fn resource_init(){if UNCAC_BASE!=IO_BASE{return;}code_resource.start=__pa_symbol(&_text);code_resource.end=__pa_symbol(&_etext)-1;data_resource.start=__pa_symbol(&_etext);data_resource.end=__pa_symbol(&_edata)-1;bss_resource.start=__pa_symbol(&__bss_start);bss_resource.end=__pa_symbol(&__bss_stop)-1;let mut i=0;let(mut start,mut end)=(0,0);while for_each_mem_range(i,&mut start,&mut end){let res=memblock_alloc_or_panic(core::mem::size_of::<resource>(),SMP_CACHE_BYTES);(*res).start=start;(*res).end=end-1;(*res).flags=IORESOURCE_SYSTEM_RAM|IORESOURCE_BUSY;(*res).name=b"System RAM\0".as_ptr() as *const _;request_resource(&mut iomem_resource,res);request_resource(res,&mut code_resource);request_resource(res,&mut data_resource);request_resource(res,&mut bss_resource);request_crashkernel(res);i+=1;}}
unsafe fn prefill_possible_map(){let mut possible=num_possible_cpus();if possible>nr_cpu_ids{possible=nr_cpu_ids;}for i in 0..possible{set_cpu_possible(i,true);}for i in possible..NR_CPUS{set_cpu_possible(i,false);}set_nr_cpu_ids(possible);}
unsafe fn setup_rng_seed(){let rng=fw_getenv(b"rngseed\0".as_ptr() as *const _);if rng.is_null(){return;}let mut seed=[0u8;512];let len=core::cmp::min(seed.len(),strlen(rng)/2);if hex2bin(seed.as_mut_ptr(),rng,len)!=0{return;}add_bootloader_randomness(seed.as_ptr(),len);memzero_explicit(seed.as_mut_ptr(),len);memzero_explicit(rng as *mut _,len*2);}
unsafe fn mips_reserve_vmcore(){#[cfg(CONFIG_PROC_VMCORE)]{let(mut start,mut end)=(0,0);let mut i=0;if elfcorehdr_size==0{while for_each_mem_range(i,&mut start,&mut end){if elfcorehdr_addr>=start&&elfcorehdr_addr<end{elfcorehdr_size=end-elfcorehdr_addr;break;}i+=1;}}memblock_reserve(elfcorehdr_addr,elfcorehdr_size);}}
unsafe fn mips_parse_crashkernel(){if !IS_ENABLED(CONFIG_CRASH_RESERVE){return;}let(mut size,mut base)=(0,0);if parse_crashkernel(boot_command_line.as_ptr(),memblock_phys_mem_size(),&mut size,&mut base,core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut())!=0||size<=0{return;}if base<=0{base=memblock_phys_alloc_range(size,SZ_64M,SZ_64M,SZ_512M);if base==0{return;}}else if memblock_phys_alloc_range(size,1,base,base+size)!=base{return;}crashk_res.start=base;crashk_res.end=base+size-1;}
unsafe fn request_crashkernel(res:*mut resource){if !IS_ENABLED(CONFIG_CRASH_RESERVE)||crashk_res.start==crashk_res.end{return;}if request_resource(res,&mut crashk_res)==0{pr_info!("Reserving crashkernel\n");}}
pub unsafe fn arch_cpu_finalize_init(){let cpu=smp_processor_id();cpu_data[cpu].udelay_val=loops_per_jiffy;check_bugs32();if IS_ENABLED(CONFIG_CPU_R4X00_BUGS64){check_bugs64();}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
