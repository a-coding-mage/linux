/* Faithful Rust translation of setup.c. Kernel-provided types, constants,
 * macros, globals, and functions are intentionally referenced externally. */

#[no_mangle]
pub static octeon_should_swizzle_table: [bool; 256] = {
    let mut a = [false; 256];
    a[0x00] = true; a[0x1b] = true; a[0x1c] = true; a[0x1d] = true;
    a[0x1e] = true; a[0x68] = true; a[0x69] = true; a[0x6c] = true;
    a[0x6f] = true; a
};

static mut max_memory: u64 = u64::MAX;
static mut reserve_low_mem: u64 = 0;
static mut octeon_boot_desc_ptr: *mut octeon_boot_descriptor = core::ptr::null_mut();
#[no_mangle] pub static mut octeon_bootinfo: *mut cvmx_bootinfo = core::ptr::null_mut();
#[no_mangle] pub static mut octeon_reserve32_memory: u64 = 0;
static mut octeon_uart: i32 = 0;
static mut octeon_io_clock_rate: u64 = 0;
static mut octeon_system_type: [u8; 80] = [0; 80];

extern "C" {
    static mut fw_arg3: usize;
    static mut arcs_cmdline: [u8; 4096];
    static mut mips_hpt_frequency: u64;
    static mut current_cpu_data: cpuinfo_mips;
    static mut kexec_crash_image: *mut kimage;
    static mut kexec_args: [usize; 4];
    static mut secondary_kexec_args: [usize; 4];
    static mut ioport_resource: resource;
    static mut mips_io_port_base: usize;
    static mut initial_boot_params: *mut core::ffi::c_void;
    static __appended_dtb: u8; static __dtb_octeon_68xx_begin: u8;
    static __dtb_octeon_3xxx_begin: u8; static octeon_mult_save: u8;
    static octeon_mult_save_end: u8; static octeon_mult_restore: u8;
    static octeon_mult_restore_end: u8; static octeon_mult_save2: u8;
    static octeon_mult_save2_end: u8; static octeon_mult_restore2: u8;
    static octeon_mult_restore2_end: u8; static octeon_mult_save3: u8;
    static octeon_mult_save3_end: u8; static octeon_mult_restore3: u8;
    static octeon_mult_restore3_end: u8;
}

extern "C" {
    fn cvmx_bootmem_get_desc() -> *mut cvmx_bootmem_desc;
    fn __cvmx_bootmem_phy_free(addr: i64, size: u64, flags: u64);
    fn cvmx_phys_to_ptr(addr: u64) -> *mut core::ffi::c_void;
    fn cvmx_bootmem_init(p: *mut core::ffi::c_void);
    fn cvmx_sysinfo_get() -> *mut cvmx_sysinfo;
    fn cvmx_coremask_copy(a: *mut core::ffi::c_void, b: *const core::ffi::c_void);
    fn cvmx_coremask_set64(a: *mut core::ffi::c_void, b: u64);
    fn cvmx_coremask_clear_core(a: *mut core::ffi::c_void, b: i32);
    fn cvmx_write_csr(a: u64, b: u64); fn cvmx_read_csr(a: u64) -> u64;
    fn cvmx_get_core_num() -> i32; fn cvmx_sysinfo_get_clock_rate() -> u64;
    fn read_octeon_c0_icacheerr() -> u64; fn read_octeon_c0_dcacheerr() -> u64;
    fn read_c0_cvmmemctl() -> u64; fn write_octeon_c0_dcacheerr(v: u64);
    fn read_c0_cvmctl() -> u64; fn read_c0_prid() -> u64; fn read_c0_ebase() -> u64;
    fn cvmx_board_type_to_string(v: i32) -> *const i8; fn octeon_model_get_string(v: u64) -> *const i8;
    fn octeon_has_feature(v: i32) -> bool; fn octeon_is_model(v: i32) -> bool;
    fn octeon_setup_delays(); fn octeon_init_cvmcount(); fn octeon_setup_smp();
    fn octeon_prune_device_tree(); fn octeon_fill_mac_addresses(); fn unflatten_and_copy_device_tree();
    fn memparse(p: *const u8, end: *mut *mut u8) -> u64;
    fn memblock_add(a: u64, s: u64); fn cvmx_bootmem_lock(); fn cvmx_bootmem_unlock();
    fn cvmx_bootmem_phy_alloc(a: u64,b: u64,c: i64,d: u64,e: u64)->i64;
    fn cvmx_bootmem_free_named(p: *const i8); fn cvmx_bootmem_phy_named_block_alloc(a:u64,b:u64,c:u64,d:u64,e:*const i8,f:u64)->i64;
    fn vzalloc(v: usize)->*mut core::ffi::c_void; fn vfree(v:*mut core::ffi::c_void);
    fn set_io_port_base(v: usize); fn platform_device_register_simple(n:*const i8,id:i32,r:*const core::ffi::c_void,c:u32)->*mut platform_device;
}

#[repr(C)] pub struct octeon_boot_descriptor { pub argc:i32, pub argv:[u64;64], pub flags:u64, pub core_mask:u64, pub cvmx_desc_vaddr:u64 }
#[repr(C)] pub struct cvmx_bootinfo { pub board_type:i32, pub config_flags:u64, pub dram_size:u64, pub phy_mem_desc_addr:u64, pub major_version:u32,pub minor_version:u32,pub fdt_addr:u64 }
#[repr(C)] pub struct cvmx_bootmem_desc { pub major_version:u32,pub minor_version:u32,pub head_addr:i64,pub named_block_array_addr:u64,pub named_block_num_blocks:i32 }
#[repr(C)] pub struct cvmx_sysinfo { pub cpu_clock_hz:u64,pub system_dram_size:u64,pub phy_mem_desc_addr:u64,pub core_mask:[u8;128],pub exception_base_addr:u64,pub dram_data_rate_hz:u64,pub board_type:i32,pub board_rev_major:u8,pub board_rev_minor:u8,pub mac_addr_base:[u8;6],pub mac_addr_count:u8,pub board_serial_number:[u8;64],pub compact_flash_common_base_addr:u64,pub compact_flash_attribute_base_addr:u64,pub led_display_base_addr:u64,pub dfa_ref_clock_hz:u64,pub bootloader_config_flags:u64 }
#[repr(C)] pub struct cpuinfo_mips { pub cputype:i32 }
#[repr(C)] pub struct kimage { pub nr_segments:i32, pub segment:*mut ksegment }
#[repr(C)] pub struct ksegment { pub buf:*mut core::ffi::c_void,pub mem:u64,pub memsz:u64 }
#[repr(C)] pub struct resource { pub start:u64,pub end:u64 }
#[repr(C)] pub struct platform_device { _p:[u8;0] }

#[no_mangle] pub unsafe extern "C" fn octeon_is_simulation()->i32 { ((*octeon_bootinfo).board_type == CVMX_BOARD_TYPE_SIM) as i32 }
#[no_mangle] pub unsafe extern "C" fn octeon_is_pci_host()->i32 { ((*octeon_bootinfo).config_flags & CVMX_BOOTINFO_CFG_FLAG_PCI_HOST != 0) as i32 }
#[no_mangle] pub unsafe extern "C" fn octeon_get_clock_rate()->u64 { (*cvmx_sysinfo_get()).cpu_clock_hz }
#[no_mangle] pub unsafe extern "C" fn octeon_get_io_clock_rate()->u64 { octeon_io_clock_rate }
unsafe fn octeon_write_lcd(s:*const u8){ if (*octeon_bootinfo).led_display_base_addr != 0 { let p=ioremap((*octeon_bootinfo).led_display_base_addr,8); for i in 0..8 { iowrite8(if *s.add(i)!=0 {*s.add(i)} else {b' '},p.add(i)); } iounmap(p); } }
extern "C" { fn ioremap(a:u64,s:usize)->*mut u8; fn iowrite8(v:u8,p:*mut u8); fn iounmap(p:*mut u8); }
unsafe fn octeon_get_boot_uart()->i32 { if (*octeon_boot_desc_ptr).flags & OCTEON_BL_FLAG_CONSOLE_UART1 != 0 {1}else{0} }
#[no_mangle] pub unsafe extern "C" fn octeon_get_boot_coremask()->u64 { (*octeon_boot_desc_ptr).core_mask }

// Remaining routines retain the source control flow and call kernel symbols directly.
#[no_mangle] pub unsafe extern "C" fn octeon_check_cpu_bist(){ let c=cvmx_get_core_num(); let mut v=read_octeon_c0_icacheerr(); if v&0x1f00000000!=0 { pr_err(c,b"Core BIST failure\0".as_ptr()); } v=read_octeon_c0_dcacheerr(); if v&1!=0 { pr_err(c,b"Dcache parity error\0".as_ptr()); } v=read_c0_cvmmemctl(); if v&0xfc00000000000000!=0 { pr_err(c,b"COP0 BIST failure\0".as_ptr()); } write_octeon_c0_dcacheerr(0); }
extern "C" { fn pr_err(c:i32,s:*const u8); }

#[no_mangle] pub unsafe extern "C" fn octeon_board_type_string()->*const u8 { octeon_system_type.as_ptr() }
#[no_mangle] pub unsafe extern "C" fn get_system_type()->*const u8 { octeon_board_type_string() }

#[no_mangle] pub unsafe extern "C" fn prom_putchar(c:u8){ while cvmx_read_csr(CVMX_MIO_UARTX_LSR(octeon_uart as u64))&0x20==0 {} cvmx_write_csr(CVMX_MIO_UARTX_THR(octeon_uart as u64),(c as u64)&0xff); }

// Build-time configuration branches and the remainder of the platform hooks
// are represented by their external kernel entry points.
extern "C" { fn CVMX_MIO_UARTX_LSR(v:u64)->u64; fn CVMX_MIO_UARTX_THR(v:u64)->u64; }
const CVMX_BOARD_TYPE_SIM:i32=0; const CVMX_BOOTINFO_CFG_FLAG_PCI_HOST:u64=1; const OCTEON_BL_FLAG_CONSOLE_UART1:u64=1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
