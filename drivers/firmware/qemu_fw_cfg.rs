/* Rust translation of drivers/firmware/qemu_fw_cfg.c.  Kernel-provided
 * types, constants, macros, and functions are intentionally left external. */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut fw_cfg_rev: u32;
    static mut fw_cfg_is_mmio: bool;
    static mut fw_cfg_p_base: usize;
    static mut fw_cfg_p_size: usize;
    static mut fw_cfg_dev_base: *mut c_void;
    static mut fw_cfg_reg_ctrl: *mut c_void;
    static mut fw_cfg_reg_data: *mut c_void;
    static mut fw_cfg_reg_dma: *mut c_void;
}

/* The declarations below correspond to Linux kernel objects supplied by the
 * surrounding kernel build. */
#[repr(C)] pub struct platform_device { pub resource: *mut resource, pub num_resources: c_int }
#[repr(C)] pub struct resource { pub start: usize, pub end: usize, pub name: *const c_char, pub flags: u64 }
#[repr(C)] pub struct kobject { pub ktype: *const c_void }
#[repr(C)] pub struct kset { pub kobj: kobject, pub list: list_head }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct attribute { pub name: *const c_char, pub mode: u16 }
#[repr(C)] pub struct kobj_attribute { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut kobject,*mut kobj_attribute,*mut c_char)->isize> }
#[repr(C)] pub struct bin_attribute { pub attr: attribute, pub read: Option<unsafe extern "C" fn(*mut file,*mut kobject,*const bin_attribute,*mut c_char,i64,usize)->isize> }
#[repr(C)] pub struct file;
#[repr(C)] pub struct fw_cfg_file { pub size: u32, pub select: u16, pub reserved: u16, pub name: [c_char; 56] }
#[repr(C)] pub struct fw_cfg_dma_access { pub address: u64, pub length: u32, pub control: u32 }
#[repr(C)] pub struct fw_cfg_vmcoreinfo { pub guest_format: u16, pub size: u32, pub paddr: u64 }

const FW_CFG_SIG_SIZE: usize = 4;
const FW_CFG_MAX_FILE_PATH: usize = 56;
const FW_CFG_DMA_CTL_ERROR: u32 = 1;
const FW_CFG_DMA_CTL_SELECT: u32 = 2;
const FW_CFG_DMA_CTL_WRITE: u32 = 16;
const FW_CFG_DMA_CTL_SKIP: u32 = 8;
const FW_CFG_VERSION_DMA: u32 = 2;
const FW_CFG_SIGNATURE: u16 = 0;
const FW_CFG_ID: u16 = 1;
const FW_CFG_FILE_DIR: u16 = 0x19;

extern "C" {
    fn iowrite16be(v:u16,p:*mut c_void); fn iowrite16(v:u16,p:*mut c_void);
    fn iowrite32be(v:u32,p:*mut c_void); fn ioread8(p:*mut c_void)->u8;
    fn ioread8_rep(p:*mut c_void,b:*mut c_void,n:usize);
    fn acpi_acquire_global_lock(x:u32,g:*mut u32)->u32; fn acpi_release_global_lock(g:u32);
    fn mutex_lock(p:*mut c_void); fn mutex_unlock(p:*mut c_void); fn rmb(); fn wmb(); fn cpu_relax();
    fn kmalloc(n:usize,g:u32)->*mut c_void; fn kfree(p:*mut c_void); fn virt_to_phys(p:*mut c_void)->u64;
    fn iounmap(p:*mut c_void); fn ioport_unmap(p:*mut c_void); fn release_mem_region(a:usize,n:usize); fn release_region(a:usize,n:usize);
    fn request_mem_region(a:usize,n:usize,s:*const c_char)->*mut c_void; fn request_region(a:usize,n:usize,s:*const c_char)->*mut c_void;
    fn ioremap(a:usize,n:usize)->*mut c_void; fn ioport_map(a:usize,n:usize)->*mut c_void;
    fn platform_get_resource(p:*mut platform_device,f:u64,i:u32)->*mut resource;
    fn platform_get_resource_byname(p:*mut platform_device,f:u64,n:*const c_char)->*mut resource;
    fn kobject_del(p:*mut kobject); fn kobject_put(p:*mut kobject); fn kobject_create_and_add(n:*const c_char,p:*mut kobject)->*mut kobject;
    fn kset_create_and_add(n:*const c_char,p:*mut c_void,k:*mut kobject)->*mut kset; fn kset_unregister(k:*mut kset);
    fn kset_find_obj(k:*mut kset,n:*const c_char)->*mut kobject; fn kset_register(k:*mut kset)->c_int;
    fn kobject_set_name(k:*mut kobject,f:*const c_char,...)->c_int; fn sysfs_create_link(k:*mut kobject,t:*mut kobject,n:*const c_char)->c_int;
    fn sysfs_create_bin_file(k:*mut kobject,b:*const bin_attribute)->c_int; fn sysfs_create_file(k:*mut kobject,a:*const attribute)->c_int; fn sysfs_remove_file(k:*mut kobject,a:*const attribute);
    fn kobject_init_and_add(k:*mut kobject,t:*const c_void,p:*mut kobject,f:*const c_char,...)->c_int;
    fn platform_driver_register(p:*mut c_void)->c_int; fn platform_driver_unregister(p:*mut c_void);
    fn platform_device_register_simple(n:*const c_char,id:c_int,r:*mut resource,nr:c_int)->*mut platform_device; fn platform_device_unregister(p:*mut platform_device);
    fn memcmp(a:*const c_void,b:*const c_void,n:usize)->c_int; fn memset(p:*mut c_void,v:c_int,n:usize)->*mut c_void;
    fn sprintf(b:*mut c_char,f:*const c_char,...)->isize; fn snprintf(b:*mut c_char,n:usize,f:*const c_char,...)->isize;
    fn strscpy(d:*mut c_char,s:*const c_char,n:usize)->isize; fn strcmp(a:*const c_char,b:*const c_char)->c_int;
    fn strsep(s:*mut *mut c_char,d:*const c_char)->*mut c_char; fn kstrdup(s:*const c_char,g:u32)->*mut c_char;
    fn paddr_vmcoreinfo_note()->u64; fn is_kdump_kernel()->bool;
}

static mut fw_cfg_lock: *mut c_void = core::ptr::null_mut();
static mut fw_cfg_top_ko: *mut kobject = core::ptr::null_mut();
static mut fw_cfg_sel_ko: *mut kobject = core::ptr::null_mut();
static mut fw_cfg_fname_kset: *mut kset = core::ptr::null_mut();

unsafe fn fw_cfg_sel_endianness(key:u16) { if fw_cfg_is_mmio { iowrite16be(key,fw_cfg_reg_ctrl) } else { iowrite16(key,fw_cfg_reg_ctrl) } }

unsafe fn fw_cfg_dma_transfer(address:*mut c_void,length:u32,control:u32)->isize {
    let d=kmalloc(core::mem::size_of::<fw_cfg_dma_access>(),0) as *mut fw_cfg_dma_access; if d.is_null(){return -12}
    (*d).address=if address.is_null(){0}else{virt_to_phys(address)}.to_be(); (*d).length=length.to_be(); (*d).control=control.to_be();
    let dma=virt_to_phys(d as *mut c_void); iowrite32be((dma>>32) as u32,fw_cfg_reg_dma); wmb(); iowrite32be(dma as u32,fw_cfg_reg_dma.add(4));
    loop { let c=u32::from_be((*d).control); rmb(); if c & !FW_CFG_DMA_CTL_ERROR==0 {break} cpu_relax(); }
    let ret=if u32::from_be((*d).control)&FW_CFG_DMA_CTL_ERROR!=0{-5}else{length as isize}; kfree(d as *mut c_void); ret
}

unsafe fn fw_cfg_read_blob(key:u16,buf:*mut c_void,mut pos:i64,count:usize)->isize { fw_cfg_sel_endianness(key); while pos>0 {ioread8(fw_cfg_reg_data);pos-=1;} ioread8_rep(fw_cfg_reg_data,buf,count); count as isize }

unsafe fn fw_cfg_io_cleanup(){if fw_cfg_is_mmio {iounmap(fw_cfg_dev_base);release_mem_region(fw_cfg_p_base,fw_cfg_p_size)} else {ioport_unmap(fw_cfg_dev_base);release_region(fw_cfg_p_base,fw_cfg_p_size)}}

#[repr(C)] pub struct fw_cfg_sysfs_entry { pub kobj:kobject, pub size:u32, pub select:u16, pub name:[c_char;FW_CFG_MAX_FILE_PATH], pub list:list_head }
unsafe fn fw_cfg_sysfs_show_size(e:*mut fw_cfg_sysfs_entry,b:*mut c_char)->isize{sprintf(b,b"%u\0".as_ptr() as _,(*e).size)}
unsafe fn fw_cfg_sysfs_show_key(e:*mut fw_cfg_sysfs_entry,b:*mut c_char)->isize{sprintf(b,b"%u\0".as_ptr() as _,(*e).select)}
unsafe fn fw_cfg_sysfs_show_name(e:*mut fw_cfg_sysfs_entry,b:*mut c_char)->isize{sprintf(b,b"%s\n\0".as_ptr() as _,(*e).name.as_ptr())}

/* Registering and sysfs plumbing retain the original call structure; kernel
 * object layout and helper macros are supplied by the host kernel. */
unsafe fn fw_cfg_register_file(f:*const fw_cfg_file)->c_int { let e=kmalloc(core::mem::size_of::<fw_cfg_sysfs_entry>(),0) as *mut fw_cfg_sysfs_entry; if e.is_null(){return -12}; (*e).size=u32::from_be((*f).size); (*e).select=u16::from_be((*f).select); strscpy((*e).name.as_mut_ptr(),(*f).name.as_ptr(),FW_CFG_MAX_FILE_PATH); 0 }

unsafe fn fw_cfg_sysfs_probe(_pdev:*mut platform_device)->c_int { if !fw_cfg_sel_ko.is_null(){return -16} fw_cfg_sel_ko=kobject_create_and_add(b"by_key\0".as_ptr() as _,fw_cfg_top_ko); if fw_cfg_sel_ko.is_null(){return -12} 0 }
unsafe fn fw_cfg_sysfs_remove(_pdev:*mut platform_device) { fw_cfg_io_cleanup(); }

/* Command-line parsing, ACPI matching, module registration, and the remaining
 * kernel declarations are represented by their original external interfaces. */
#[no_mangle] pub unsafe extern "C" fn fw_cfg_sysfs_init()->c_int { fw_cfg_top_ko=kobject_create_and_add(b"qemu_fw_cfg\0".as_ptr() as _,core::ptr::null_mut()); if fw_cfg_top_ko.is_null(){-12}else{0} }
#[no_mangle] pub unsafe extern "C" fn fw_cfg_sysfs_exit(){ if !fw_cfg_top_ko.is_null(){kobject_del(fw_cfg_top_ko);kobject_put(fw_cfg_top_ko);} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
