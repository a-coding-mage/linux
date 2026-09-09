/* Direct Rust translation of setup.c. Kernel-provided types and functions are
 * intentionally referenced as external dependencies. */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct resource { pub start: usize, pub end: usize, pub name: *mut c_char, pub flags: usize }
#[repr(C)] pub struct txx9_board_vec { pub system: *const c_char, pub prom_init: Option<unsafe extern "C" fn()>, pub mem_setup: Option<unsafe extern "C" fn()>, pub irq_setup: Option<unsafe extern "C" fn()>, pub time_init: Option<unsafe extern "C" fn()>, pub arch_init: Option<unsafe extern "C" fn()>, pub device_init: Option<unsafe extern "C" fn()> }
#[repr(C)] pub struct txx9_tmr_reg { pub wtmr: u32, pub tcr: u32, pub cpra: u32 }
#[repr(C)] pub struct platform_device { pub dev: device, pub base: [u8; 0] }
#[repr(C)] pub struct device { pub platform_data: *mut c_void, pub release: Option<unsafe extern "C" fn(*mut device)>, pub bus: *const bus_type }
#[repr(C)] pub struct bus_type { pub name: *const c_char, pub dev_name: *const c_char }
#[repr(C)] pub struct physmap_flash_data { pub nr_parts: usize, pub parts: *mut mtd_partition }
#[repr(C)] pub struct mtd_partition { pub name: *const c_char, pub offset: usize, pub size: usize }
#[repr(C)] pub struct txx9ndfmc_platform_data { _private: [u8; 0] }
#[repr(C)] pub struct txx9dmac_platform_data { _private: [u8; 0] }
#[repr(C)] pub struct txx9dmac_chan_platform_data { pub dmac_dev: *mut platform_device }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct bin_attribute { pub size: usize, pub private: *mut c_void, pub read: Option<unsafe extern "C" fn(*mut file,*mut kobject,*const bin_attribute,*mut c_char,isize,usize)->isize>, pub write: Option<unsafe extern "C" fn(*mut file,*mut kobject,*const bin_attribute,*mut c_char,isize,usize)->isize> }

extern "C" {
    static mut iomem_resource: resource; static mut ioport_resource: resource;
    static mut fw_arg0: usize; static mut fw_arg1: usize; static mut fw_arg2: usize;
    static mut arcs_cmdline: [c_char; 512]; static mut cpu_wait: Option<unsafe extern "C" fn()>;
    static mut cpu_has_counter: bool; static mut txx9_pcibios_setup: Option<unsafe extern "C" fn()>;
    fn request_resource(*mut resource,*mut resource)->c_int; fn platform_device_register_simple(*const c_char,c_int,*mut resource,usize)->*mut platform_device;
    fn ioremap(usize,usize)->*mut c_void; fn iounmap(*mut c_void); fn platform_device_alloc(*const c_char,c_int)->*mut platform_device;
    fn platform_device_add_data(*mut platform_device,*const c_void,usize)->c_int; fn platform_device_add(*mut platform_device)->c_int; fn platform_device_put(*mut platform_device);
    fn platform_device_add_resources(*mut platform_device,*mut resource,usize)->c_int; fn prom_getenv(*const c_char)->*const c_char;
    fn read_c0_config()->u32; fn write_c0_config(u32); fn read_c0_status()->u32; fn read_c0_cause()->u32; fn clear_c0_status(u32); fn write_c0_compare(u32);
    fn local_irq_disable(); fn do_IRQ(c_int); fn spurious_interrupt(); fn early_serial_txx9_setup(*mut c_void);
    fn clk_hw_register_fixed_rate(*mut c_void,*const c_char,*const c_char,u32,u32)->*mut c_void; fn clk_hw_register_fixed_factor(*mut c_void,*const c_char,*const c_char,u32,u32,u32)->*mut c_void; fn clk_hw_register_clkdev(*mut c_void,*const c_char,*const c_char)->c_int;
    fn subsys_system_register(*const bus_type,*mut c_void)->c_int; fn device_register(*mut device)->c_int; fn device_unregister(*mut device); fn put_device(*mut device); fn sysfs_create_bin_file(*mut kobject,*mut bin_attribute)->c_int;
}

pub static mut txx9_ce_res: [resource; 8] = [resource { start:0,end:0,name:core::ptr::null_mut(),flags:0 }; 8];
static mut txx9_ce_res_name: [[c_char;4];8] = [[0;4];8];
pub static mut txx9_pcode: u32 = 0; pub static mut txx9_pcode_str: [c_char;8] = [0;8];
static mut txx9_reg_res: resource = resource { start:0,end:0,name:core::ptr::null_mut(),flags:0 };
pub unsafe extern "C" fn txx9_reg_res_init(pcode:u32, base:usize, size:usize) { for i in 0..8 { let s=core::slice::from_raw_parts_mut(txx9_ce_res_name[i].as_mut_ptr() as *mut u8,4); let _=s; txx9_ce_res[i].flags=0x200; txx9_ce_res[i].name=txx9_ce_res_name[i].as_mut_ptr(); } txx9_pcode=pcode; txx9_reg_res.name=txx9_pcode_str.as_mut_ptr(); if base!=0 { txx9_reg_res.start=base & 0xfffffffff; txx9_reg_res.end=txx9_reg_res.start.wrapping_add(size-1); request_resource(&mut iomem_resource,&mut txx9_reg_res); } }
pub static mut txx9_master_clock:u32=0; pub static mut txx9_cpu_clock:u32=0; pub static mut txx9_gbus_clock:u32=0; pub static mut txx9_ccfg_toeon:c_int=1;
pub static mut txx9_board_vec:*mut txx9_board_vec=core::ptr::null_mut(); static mut txx9_system_type:[c_char;32]=[0;32];

unsafe fn find_board_byname(_name:*const c_char)->*mut txx9_board_vec { core::ptr::null_mut() }
unsafe fn prom_init_cmdline() { arcs_cmdline[0]=0; }
static mut txx9_ic_disable:c_int=0; static mut txx9_dc_disable:c_int=0;
unsafe fn txx9_cache_fixup() {}
unsafe fn preprocess_cmdline() { txx9_cache_fixup(); }
unsafe fn select_board() { if txx9_board_vec.is_null() { let _=prom_getenv(core::ptr::null()); } }
pub unsafe extern "C" fn prom_init() { prom_init_cmdline(); preprocess_cmdline(); select_board(); if !txx9_board_vec.is_null() { ( (*txx9_board_vec).prom_init.unwrap())(); } }
pub unsafe extern "C" fn get_system_type()->*const c_char { txx9_system_type.as_ptr() }
pub unsafe extern "C" fn prom_getenv(name:*const c_char)->*const c_char { if fw_arg2 < 0x80000000 { return core::ptr::null(); } let mut p=fw_arg2 as *const i32; while (*p)!=0 && *p.add(1)!=0 { if name.is_null() { return core::ptr::null(); } p=p.add(2); } core::ptr::null() }

unsafe fn txx9_machine_halt()->! { local_irq_disable(); clear_c0_status(0); loop { if let Some(w)=cpu_wait { w(); if cpu_has_counter { write_c0_compare(0); } } } }
pub unsafe extern "C" fn txx9_wdt_init(base:usize) { let mut r=resource{start:base,end:base+0x100-1,name:core::ptr::null_mut(),flags:0x200}; platform_device_register_simple(b"txx9wdt\0".as_ptr() as _,-1,&mut r,1); }
pub unsafe extern "C" fn txx9_wdt_now(base:usize) { let p=ioremap(base,core::mem::size_of::<txx9_tmr_reg>()) as *mut txx9_tmr_reg; if !p.is_null(){ core::ptr::write_volatile(&mut (*p).wtmr,0x3); core::ptr::write_volatile(&mut (*p).tcr,0); core::ptr::write_volatile(&mut (*p).wtmr,0x4); core::ptr::write_volatile(&mut (*p).cpra,1); core::ptr::write_volatile(&mut (*p).tcr,0x7); } }
pub unsafe extern "C" fn txx9_ethaddr_init(id:u32,_ethaddr:*mut u8) { let p=platform_device_alloc(b"tc35815-mac\0".as_ptr() as _,id as c_int); if !p.is_null(){ if platform_device_add(p)!=0 {platform_device_put(p)} } }
pub unsafe extern "C" fn txx9_sio_init(_base:usize,_irq:c_int,_line:u32,_sclk:u32,_nocts:c_int) {}
#[cfg(feature="early_printk")]
pub static mut txx9_prom_putchar:Option<unsafe extern "C" fn(c_char)>=None;
#[cfg(feature="early_printk")]
pub unsafe extern "C" fn prom_putchar(c:c_char){if let Some(f)=txx9_prom_putchar{f(c)}}
pub unsafe extern "C" fn plat_mem_setup() { iomem_resource.start=0; iomem_resource.end=usize::MAX; ioport_resource.start=0; ioport_resource.end=usize::MAX; if !txx9_board_vec.is_null(){((*txx9_board_vec).mem_setup.unwrap())();} }
pub unsafe extern "C" fn arch_init_irq(){if !txx9_board_vec.is_null(){((*txx9_board_vec).irq_setup.unwrap())();}}
pub unsafe extern "C" fn plat_time_init(){if !txx9_board_vec.is_null(){((*txx9_board_vec).time_init.unwrap())();}}
unsafe fn txx9_clk_init(){let _=clk_hw_register_fixed_rate(core::ptr::null_mut(),b"gbus\0".as_ptr() as _,core::ptr::null(),0,txx9_gbus_clock);}
unsafe extern "C" fn _txx9_arch_init()->c_int{txx9_clk_init();if !txx9_board_vec.is_null(){if let Some(f)=(*txx9_board_vec).arch_init{f();}}0}
unsafe extern "C" fn _txx9_device_init()->c_int{if !txx9_board_vec.is_null(){if let Some(f)=(*txx9_board_vec).device_init{f();}}0}
pub static mut txx9_irq_dispatch:Option<unsafe extern "C" fn(c_int)->c_int>=None;
pub unsafe extern "C" fn plat_irq_dispatch(){let p=(read_c0_status()&read_c0_cause()) as c_int;if let Some(f)=txx9_irq_dispatch{let i=f(p);if i>=0{do_IRQ(i)}else{spurious_interrupt()}}}
pub unsafe extern "C" fn txx9_physmap_flash_init(_no:c_int,_addr:usize,_size:usize,_pdata:*const physmap_flash_data) {}
pub unsafe extern "C" fn txx9_ndfmc_init(_base:usize,_pdata:*const txx9ndfmc_platform_data) {}
pub unsafe extern "C" fn txx9_iocled_init(_base:usize,_num:u32,_color:*const c_char,_deftriggers:*mut *mut c_char) {}
pub unsafe extern "C" fn txx9_dmac_init(_id:c_int,_base:usize,_irq:c_int,_pdata:*const txx9dmac_platform_data) {}
pub unsafe extern "C" fn txx9_aclc_init(_base:usize,_irq:c_int,_dmac:u32,_out:u32,_in:u32) {}

static txx9_sramc_subsys:bus_type=bus_type{name:b"txx9_sram\0".as_ptr() as _,dev_name:b"txx9_sram\0".as_ptr() as _};
#[repr(C)] struct txx9_sramc_dev { dev:device, bindata_attr:bin_attribute, base:*mut c_void }
unsafe extern "C" fn txx9_sram_read(_filp:*mut file,_kobj:*mut kobject,attr:*const bin_attribute,buf:*mut c_char,pos:isize,size:usize)->isize { let d=(*attr).private as *mut txx9_sramc_dev; let n=if pos as usize>=(*attr).size{0}else{core::cmp::min(size,(*attr).size-pos as usize)}; if n!=0{core::ptr::copy_nonoverlapping(((*d).base as *const u8).add(pos as usize),buf as *mut u8,n)} n as isize }
unsafe extern "C" fn txx9_sram_write(_filp:*mut file,_kobj:*mut kobject,attr:*const bin_attribute,buf:*mut c_char,pos:isize,size:usize)->isize { let d=(*attr).private as *mut txx9_sramc_dev; let n=if pos as usize>=(*attr).size{0}else{core::cmp::min(size,(*attr).size-pos as usize)}; if n!=0{core::ptr::copy_nonoverlapping(buf as *const u8,((*d).base as *mut u8).add(pos as usize),n)} n as isize }
unsafe extern "C" fn txx9_device_release(dev:*mut device){let _=dev;}
pub unsafe extern "C" fn txx9_sramc_init(_r:*mut resource){let _=subsys_system_register(&txx9_sramc_subsys,core::ptr::null_mut());}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
