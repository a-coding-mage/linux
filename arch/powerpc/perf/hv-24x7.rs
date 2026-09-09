// SPDX-License-Identifier: GPL-2.0-or-later
/* Hypervisor supplied "24x7" performance counter support. */

// Linux/PowerPC headers and macro-generated declarations are supplied by the
// surrounding kernel translation unit.

static mut INTERFACE_VERSION: i32 = 0;
static mut AGGREGATE_RESULT_ELEMENTS: bool = false;
static mut HV_24X7_CPUMASK: cpumask_t = cpumask_t { _private: 0 };
static mut PHYS_SOCKETS: u32 = 0;
static mut PHYS_CHIPSPERSOCKET: u32 = 0;
static mut PHYS_CORESPERCHIP: u32 = 0;

extern "C" {
    static mut hv_page_cache: *mut kmem_cache;
}

#[repr(C)] pub struct cpumask_t { pub _private: usize }
#[repr(C)] pub struct kmem_cache { pub _private: usize }
#[repr(C)] pub struct attribute { pub _private: usize }
#[repr(C)] pub struct attribute_group { pub name: *const u8, pub attrs: *mut *mut attribute }
#[repr(C)] pub struct device { pub _private: usize }
#[repr(C)] pub struct device_attribute { pub _private: usize }
#[repr(C)] pub struct file { pub _private: usize }
#[repr(C)] pub struct kobject { pub _private: usize }
#[repr(C)] pub struct bin_attribute { pub _private: usize }
#[repr(C)] pub struct perf_event { pub _private: usize }
#[repr(C)] pub struct pmu { pub _private: usize }
#[repr(C)] pub struct hv_perf_caps { pub collect_privileged: bool }

extern "C" {
    fn domain_is_valid_external(d: u32) -> bool;
    fn is_physical_domain_external(d: u32) -> bool;
    fn papr_sysparm_buf_alloc() -> *mut papr_sysparm_buf;
    fn papr_sysparm_buf_free(p: *mut papr_sysparm_buf);
    fn papr_sysparm_get(t: u32, p: *mut papr_sysparm_buf) -> i32;
    fn be16_to_cpu(v: u16) -> u16;
    fn be16_to_cpup(v: *const u16) -> u16;
    fn be32_to_cpu(v: u32) -> u32;
    fn be64_to_cpu(v: u64) -> u64;
    fn plpar_hcall_norets(n: u64, a: u64, b: u64, c: u64) -> i64;
    fn virt_to_phys(p: *const u8) -> u64;
    fn vmalloc(n: usize) -> *mut u8;
    fn vfree(p: *mut u8);
    fn vmalloc_to_phys(p: *const u8) -> u64;
    fn memcpy(dst: *mut u8, src: *const u8, n: usize);
    fn strncmp(a: *const i8, b: *const i8, n: usize) -> i32;
    fn hv_perf_caps_get(c: *mut hv_perf_caps) -> i64;
    fn perf_pmu_migrate_context(p: *mut pmu, from: u32, to: u32);
}

#[repr(C)] pub struct papr_sysparm_buf { pub len: u16, pub val: [u8; 256] }
#[repr(C)] pub struct hv_24x7_event_data {
    pub length: u16, pub domain: u8, pub event_group_record_len: u8,
    pub event_name_len: u16, pub event_counter_offs: u16,
    pub event_group_record_offs: u16, pub remainder: [u8; 0],
}
#[repr(C)] pub struct hv_24x7_request { pub performance_domain:u8, pub data_size:u16, pub data_offset:u32, pub starting_lpar_ix:u16, pub max_num_lpars:u16, pub starting_ix:u16, pub max_ix:u16, pub starting_thread_group_ix:u16, pub max_num_thread_groups:u16 }
#[repr(C)] pub struct hv_24x7_request_buffer { pub interface_version:u32, pub num_requests:u32, pub requests:[hv_24x7_request; 0] }
#[repr(C)] pub struct hv_24x7_result { pub result_ix:u8, pub num_elements_returned:u16, pub result_element_data_size:u16, pub elements:[u8; 0] }
#[repr(C)] pub struct hv_24x7_data_result_buffer { pub interface_version:u32, pub num_results:u32, pub detailed_rc:u32, pub failing_request_ix:u32, pub results:[hv_24x7_result; 0] }

unsafe fn domain_is_valid(domain: u32) -> bool { domain_is_valid_external(domain) }
unsafe fn is_physical_domain(domain: u32) -> bool { is_physical_domain_external(domain) }

pub unsafe fn read_24x7_sys_info() {
    PHYS_SOCKETS=1; PHYS_CHIPSPERSOCKET=1; PHYS_CORESPERCHIP=1;
    let b=papr_sysparm_buf_alloc(); if b.is_null(){return;}
    if papr_sysparm_get(43,b)==0 { let n=be16_to_cpup((*b).val.as_ptr() as *const u16); let l=be16_to_cpu((*b).len); if l>=8 && n!=0 { PHYS_SOCKETS=be16_to_cpup((*b).val.as_ptr().add(2) as *const u16) as u32; PHYS_CHIPSPERSOCKET=be16_to_cpup((*b).val.as_ptr().add(4) as *const u16) as u32; PHYS_CORESPERCHIP=be16_to_cpup((*b).val.as_ptr().add(6) as *const u16) as u32; }}
    papr_sysparm_buf_free(b);
}

unsafe fn domain_needs_aggregation(domain:u32)->bool { AGGREGATE_RESULT_ELEMENTS && (domain==3 || (domain>=4 && domain<=6)) }
unsafe fn domain_name(domain:u32)->*const u8 { if !domain_is_valid(domain){return core::ptr::null();} match domain {2=>b"Physical Chip\0".as_ptr(),3=>b"Physical Core\0".as_ptr(),4=>b"VCPU Home Core\0".as_ptr(),5=>b"VCPU Home Chip\0".as_ptr(),6=>b"VCPU Home Node\0".as_ptr(),7=>b"VCPU Remote Node\0".as_ptr(),_=>core::ptr::null()} }
unsafe fn catalog_entry_domain_is_valid(domain:u32)->bool { if INTERFACE_VERSION==1 {is_physical_domain(domain)} else {domain_is_valid(domain)} }

unsafe fn event_name(ev:*mut hv_24x7_event_data, len:*mut i32)->*mut i8 { *len=be16_to_cpu((*ev).event_name_len) as i32-2; (*ev).remainder.as_mut_ptr() as *mut i8 }
unsafe fn event_desc(ev:*mut hv_24x7_event_data, len:*mut i32)->*mut i8 { let n=be16_to_cpu((*ev).event_name_len) as usize; let p=(*ev).remainder.as_mut_ptr().add(n-2) as *const u16; *len=be16_to_cpu(*p) as i32-2; (*ev).remainder.as_mut_ptr().add(n) as *mut i8 }
unsafe fn event_long_desc(ev:*mut hv_24x7_event_data, len:*mut i32)->*mut i8 { let n=be16_to_cpu((*ev).event_name_len) as usize; let p=(*ev).remainder.as_mut_ptr().add(n-2) as *const u16; let d=be16_to_cpu(*p) as usize; let q=(*ev).remainder.as_mut_ptr().add(n+d-2) as *const u16; *len=be16_to_cpu(*q) as i32-2; (*ev).remainder.as_mut_ptr().add(n+d) as *mut i8 }
unsafe fn event_fixed_portion_is_within(ev:*mut hv_24x7_event_data,end:*mut u8)->bool { (ev as *mut u8).add(core::mem::offset_of!(hv_24x7_event_data,remainder)) < end }

unsafe fn event_end(ev:*mut hv_24x7_event_data,end:*mut u8)->*mut u8 { let s=ev as *mut u8; let n=be16_to_cpu((*ev).event_name_len) as usize; if n<2 || s.add(n)>end{return core::ptr::null_mut();} let d=be16_to_cpu(*(s.add(n-2) as *const u16)) as usize; if d<2 || s.add(n+d)>end{return core::ptr::null_mut();} let l=be16_to_cpu(*(s.add(n+d-2) as *const u16)) as usize; if l<2 || s.add(n+d+l)>end{return core::ptr::null_mut();} s.add(n+d+l) }
unsafe fn ignore_event(name:*const i8)->bool { strncmp(name,b"RESERVED\0".as_ptr() as *const i8,8)==0 }

unsafe fn max_num_requests(v:i32)->u32 { ((4096-core::mem::size_of::<hv_24x7_request_buffer>()) / (16+v as usize*8)) as u32 }
unsafe fn h_get_24x7_catalog_page_(p:u64,v:u64,i:u64)->i64 { plpar_hcall_norets(0xF000, p,v,i) }
unsafe fn h_get_24x7_catalog_page(p:*mut u8,v:u64,i:u32)->i64 { h_get_24x7_catalog_page_(virt_to_phys(p),v,i as u64) }

// The remaining sysfs/catalog and PMU glue is represented with the same
// externally supplied kernel interfaces; the local behavior is kept literal.
pub unsafe fn hv_24x7_init()->i32 { read_24x7_sys_info(); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
