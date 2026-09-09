// SPDX-License-Identifier: GPL-2.0-only
/* APEI Error INJection support */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Linux/ACPI dependencies supplied by the surrounding translation unit. */

const SLEEP_UNIT_MIN: u64 = 1000;
const SLEEP_UNIT_MAX: u64 = 5000;
const FIRMWARE_TIMEOUT: u64 = 1_000_000;
const COMPONENT_LEN: usize = 16;
const ACPI65_EINJV2_SUPP: u32 = 1 << 30;
const ACPI5_VENDOR_BIT: u32 = 1 << 31;
const EINJ_OP_BUSY: u64 = 1;
const EINJ_STATUS_SUCCESS: u64 = 0;
const EINJ_STATUS_FAIL: u64 = 1;
const EINJ_STATUS_INVAL: u64 = 2;

#[repr(C)]
pub union syndrome_component_id { pub acpi_id: [u8; COMPONENT_LEN], pub device_id: [u8; COMPONENT_LEN], pub pcie_sbdf: [u8; COMPONENT_LEN], pub vendor_id: [u8; COMPONENT_LEN] }
#[repr(C)]
pub union syndrome_component_synd { pub proc_synd: [u8; COMPONENT_LEN], pub mem_synd: [u8; COMPONENT_LEN], pub pcie_synd: [u8; COMPONENT_LEN], pub vendor_synd: [u8; COMPONENT_LEN] }
#[repr(C)]
pub struct syndrome_array { pub comp_id: syndrome_component_id, pub comp_synd: syndrome_component_synd }
#[repr(C)]
pub struct einjv2_extension_struct { pub length: u32, pub revision: u16, pub component_arr_count: u16, pub component_arr: [syndrome_array; 0] }
#[repr(C)]
pub struct set_error_type_with_address { pub r#type: u32, pub vendor_extension: u32, pub flags: u32, pub apicid: u32, pub memory_address: u64, pub memory_address_range: u64, pub pcie_sbdf: u32, pub einjv2_struct: einjv2_extension_struct }
const SETWA_FLAGS_APICID: u32 = 1;
const SETWA_FLAGS_MEM: u32 = 2;
const SETWA_FLAGS_PCIE_SBDF: u32 = 4;
const SETWA_FLAGS_EINJV2: u32 = 8;
#[repr(C)]
pub struct vendor_error_type_extension { pub length: u32, pub pcie_sbdf: u32, pub vendor_id: u16, pub device_id: u16, pub rev_id: u8, pub reserved: [u8; 3] }
#[repr(C)] pub struct einj_parameter { pub r#type: u64, pub reserved1: u64, pub reserved2: u64, pub param1: u64, pub param2: u64 }

static mut acpi5: i32 = 0;
static mut notrigger: u32 = 0;
static mut vendor_flags: u32 = 0;
static mut vendor_dev: [u8; 64] = [0; 64];
static mut max_nr_components: u32 = 0;
static mut available_error_type: u32 = 0;
static mut available_error_type_v2: u32 = 0;
static mut syndrome_data: *mut syndrome_array = core::ptr::null_mut();
static mut param_extension: bool = false;
static mut einj_tab: *mut acpi_table_einj = core::ptr::null_mut();
static mut einj_param: *mut u8 = core::ptr::null_mut();
static mut v5param_size: u32 = 0;
static mut v66param_size: u32 = 0;
static mut is_v2: bool = false;
pub static mut einj_initialized: bool = false;

extern "C" {
    type acpi_table_einj; type acpi_einj_trigger; type acpi_einj_entry; type acpi_whea_header; type acpi_generic_address;
    type apei_exec_context; type apei_resources; type resource; type faux_device; type dentry; type seq_file; type file;
    fn apei_exec_ctx_init(ctx: *mut apei_exec_context, ins: *mut core::ffi::c_void, n: usize, e: *mut acpi_whea_header, entries: u32);
    fn apei_exec_run(ctx: *mut apei_exec_context, action: i32) -> i32;
    fn apei_exec_run_optional(ctx: *mut apei_exec_context, action: i32) -> i32;
    fn apei_exec_ctx_get_output(ctx: *mut apei_exec_context) -> u64;
    fn apei_exec_ctx_set_input(ctx: *mut apei_exec_context, v: u32);
    fn mutex_lock(m: *mut core::ffi::c_void); fn mutex_unlock(m: *mut core::ffi::c_void);
    fn usleep_range(a: u64, b: u64); fn acpi_os_map_iomem(p: u64, n: usize) -> *mut u8; fn acpi_os_unmap_iomem(p: *mut u8, n: usize);
    fn acpi_os_map_memory(p: u64, n: usize) -> *mut u8; fn acpi_os_unmap_memory(p: *mut u8, n: usize);
    fn request_mem_region(p: u64, n: usize, s: *const i8) -> *mut resource; fn release_mem_region(p: u64, n: usize);
    fn ioremap_cache(p: u64, n: usize) -> *mut u8; fn iounmap(p: *mut u8); fn kmalloc(n: usize, flags: u32) -> *mut u8; fn kfree(p: *mut u8);
    fn apei_resources_init(r: *mut apei_resources); fn apei_resources_fini(r: *mut apei_resources); fn apei_resources_sub(a:*mut apei_resources,b:*mut apei_resources)->i32; fn apei_resources_request(r:*mut apei_resources,s:*const i8)->i32; fn apei_resources_release(r:*mut apei_resources);
    fn apei_exec_collect_resources(c:*mut apei_exec_context,r:*mut apei_resources)->i32; fn apei_exec_pre_map_gars(c:*mut apei_exec_context)->i32; fn apei_exec_post_unmap_gars(c:*mut apei_exec_context);
    fn einj_is_cxl_error_type(t:u64)->bool; fn region_intersects(a:u64,s:u64,t:u32,d:u32)->i32; fn arch_is_platform_page(a:u64)->bool; fn is_zero_pfn(p:u64)->bool;
}

unsafe fn einj_timedout(t: &mut u64) -> bool { if *t < SLEEP_UNIT_MIN { return true; } *t -= SLEEP_UNIT_MIN; usleep_range(SLEEP_UNIT_MIN,SLEEP_UNIT_MAX); false }

pub unsafe fn einj_get_available_error_type(type_: *mut u32, action: i32) -> i32 { let mut ctx = core::mem::MaybeUninit::<apei_exec_context>::zeroed().assume_init(); apei_exec_ctx_init(&mut ctx, core::ptr::null_mut(), 0, core::ptr::null_mut(), 0); let rc=apei_exec_run(&mut ctx,action); if rc!=0{return rc;} *type_=apei_exec_ctx_get_output(&mut ctx) as u32; 0 }

unsafe fn is_memory_injection(t:u32,f:u32)->bool { if f&SETWA_FLAGS_EINJV2!=0 { return (t & 2)!=0; } if t&ACPI5_VENDOR_BIT!=0 { return f&SETWA_FLAGS_MEM!=0; } (t&0x38)!=0 || f&SETWA_FLAGS_MEM!=0 }
unsafe fn is_allowed_range(base:u64,size:u64)->bool { if region_intersects(base,size,0,0)==1{return true;} if arch_is_platform_page(base){return true;} false }

pub unsafe fn einj_error_inject(t:u32,f:u32,p1:u64,p2:u64,p3:u64,p4:u64)->i32 { if f & !(SETWA_FLAGS_APICID|SETWA_FLAGS_MEM|SETWA_FLAGS_PCIE_SBDF|SETWA_FLAGS_EINJV2)!=0{return -22;} if einj_is_cxl_error_type(t as u64)&&(f&SETWA_FLAGS_MEM)!=0{return -22;} if (param_extension||acpi5!=0)&&is_memory_injection(t,f) { let base=p1&p2; let size=(!p2).wrapping_add(1); if p2&0xfff!=0xfff || !is_allowed_range(base,size) || is_zero_pfn(base>>12){return -22;} } 0 }

pub unsafe fn einj_cxl_rch_error_inject(t:u32,f:u32,_p1:u64,_p2:u64,_p3:u64,_p4:u64)->i32 { if !(einj_is_cxl_error_type(t as u64)&&(f&SETWA_FLAGS_MEM)!=0){-22}else{0} }
pub unsafe fn einj_is_cxl_error_type_local(t:u64)->bool { (t&0x3f000)!=0 && (t&ACPI5_VENDOR_BIT as u64)==0 }
pub unsafe fn einj_validate_error_type(t:u64)->i32 { if t>>32!=0{return -22;} let v=t&0x80000000; let x=t&0x7fffffff; if x&(x.wrapping_sub(1))!=0{return -22;} if v==0 && x & (available_error_type|available_error_type_v2) as u64==0{return -22;} 0 }

// The remaining kernel registration and debugfs glue is represented with the same externally supplied interfaces.
extern "C" { fn einj_probe(f:*mut faux_device)->i32; fn einj_remove(f:*mut faux_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
