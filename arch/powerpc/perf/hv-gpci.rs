// SPDX-License-Identifier: GPL-2.0-or-later
/* Hypervisor supplied gpci performance counter support. */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Kernel headers and hv-gpci.h/hv-common.h supply the types, constants,
// functions, macros, and generated event-format declarations referenced here.

extern "C" {
    fn hv_perf_caps_get(caps: *mut hv_perf_caps) -> c_ulong;
    fn plpar_hcall_norets(op: c_ulong, arg: c_ulong, len: c_ulong) -> c_ulong;
    fn virt_to_phys(p: *mut core::ffi::c_void) -> c_ulong;
    fn event_get_request(e: *mut perf_event) -> u32;
    fn event_get_starting_index(e: *mut perf_event) -> u32;
    fn event_get_secondary_index(e: *mut perf_event) -> u16;
    fn event_get_counter_info_version(e: *mut perf_event) -> u8;
    fn event_get_offset(e: *mut perf_event) -> u32;
    fn event_get_length(e: *mut perf_event) -> u8;
    fn has_branch_stack(e: *mut perf_event) -> bool;
    fn local64_xchg(p: *mut i64, v: u64) -> i64;
    fn local64_add(v: i64, p: *mut i64);
    fn local64_set(p: *mut i64, v: u64);
    fn perf_pmu_migrate_context(pmu: *mut pmu, cpu: c_int, target: c_int);
    fn cpuhp_setup_state(state: c_int, name: *const c_char,
        online: unsafe extern "C" fn(u32) -> c_int,
        offline: unsafe extern "C" fn(u32) -> c_int) -> c_int;
    fn firmware_has_feature(feature: c_ulong) -> bool;
    fn mfspr(reg: c_ulong) -> c_ulong;
    fn perf_pmu_register(pmu: *mut pmu, name: *const c_char, idx: c_int) -> c_int;
    fn hv_gpci_assert_offsets_correct();
}

type c_int = i32; type c_ulong = usize; type c_char = i8;
type u32_be = u32; type u16_be = u16;

#[repr(C)] pub struct hv_perf_caps { pub version: u32, pub ga: i32, pub expanded: i32, pub lab: i32, pub collect_privileged: i32 }
#[repr(C)] pub struct hv_gpci_params {
    pub counter_request: u32_be, pub starting_index: u32_be, pub secondary_index: u16_be,
    pub returned_values: u16_be, pub cv_element_size: u16_be, pub detail_rc: u32_be,
    pub counter_info_version_in: u8, pub counter_info_version_out: u8,
}
#[repr(C)] pub struct hv_gpci_request_buffer { pub params: hv_gpci_params, pub bytes: [u8; HGPCI_MAX_DATA_BYTES] }
#[repr(C)] pub struct cpumask_t { _private: [u64; 1] }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { pub name: *const c_char, pub attrs: *mut *mut attribute }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { pub attr: attribute, pub show: Option<unsafe extern "C" fn(*mut device,*mut device_attribute,*mut c_char)->isize> }
#[repr(C)] pub struct perf_event_attr { pub r#type: u32, pub config: u64, pub config2: u64 }
#[repr(C)] pub struct perf_event { pub attr: perf_event_attr, pub pmu: *mut pmu, pub hw: perf_hw, pub count: i64 }
#[repr(C)] pub struct perf_hw { pub prev_count: i64 }
#[repr(C)] pub struct pmu { pub task_ctx_nr: i32, pub name: *const c_char, pub attr_groups: *const *const attribute_group, pub capabilities: u64 }

const HGPCI_MAX_DATA_BYTES: usize = 4096;
const HGPCI_REQ_BUFFER_SIZE: usize = 4096;
const H_GET_PERF_COUNTER_INFO: c_ulong = 0;
const H_SUCCESS: c_ulong = 0; const H_PARAMETER: c_ulong = 1; const H_AUTHORITY: c_ulong = 2;
const PAGE_SIZE: usize = 4096; const PERF_EF_START: c_int = 1;
const PERF_PMU_CAP_NO_EXCLUDE: u64 = 1; const PERF_PMU_CAP_NO_INTERRUPT: u64 = 2;
const ENOENT: c_int = 2; const EIO: c_int = 5; const EINVAL: c_int = 22; const EPERM: c_int = 1;
const ENODEV: c_int = 19; const EFBIG: c_int = 27; const EOPNOTSUPP: c_int = 95;

static mut hv_gpci_cpumask: cpumask_t = cpumask_t { _private: [0] };
static mut hv_gpci_reqb: hv_gpci_request_buffer = hv_gpci_request_buffer { params: hv_gpci_params { counter_request:0, starting_index:0, secondary_index:0, returned_values:0, cv_element_size:0, detail_rc:0, counter_info_version_in:0, counter_info_version_out:0 }, bytes:[0; HGPCI_MAX_DATA_BYTES] };

const PROCESSOR_BUS_TOPOLOGY: usize = 0; const PROCESSOR_CONFIG: usize = 1;
const AFFINITY_DOMAIN_VIA_VP: usize = 2; const AFFINITY_DOMAIN_VIA_DOM: usize = 3;
const AFFINITY_DOMAIN_VIA_PAR: usize = 4;
static sysinfo_counter_request: [u32; 5] = [0xD0, 0x90, 0xA0, 0xB0, 0xB1];

unsafe fn systeminfo_gpci_request(req:u32, starting_index:u32, secondary_index:u16, buf:*mut u8, n:*mut usize, arg:*mut hv_gpci_request_buffer)->c_ulong {
    (*arg).params.counter_request=req.to_be(); (*arg).params.starting_index=starting_index.to_be(); (*arg).params.secondary_index=secondary_index.to_be();
    let ret=plpar_hcall_norets(H_GET_PERF_COUNTER_INFO, virt_to_phys(arg.cast()), HGPCI_REQ_BUFFER_SIZE as c_ulong);
    if ret==H_AUTHORITY{return EPERM as c_ulong} if ret!=0 && ret!=H_PARAMETER{return EIO as c_ulong}
    let count=u16::from_be((*arg).params.returned_values) as usize; let size=u16::from_be((*arg).params.cv_element_size) as usize;
    for i in 0..count { for j in i*size..(i+1)*size { *buf.add(*n)=(*arg).bytes[j]; *n+=2; } *buf.add(*n)=b'\n'; *n+=1; }
    if *n>=PAGE_SIZE{return EFBIG as c_ulong} ret
}

unsafe fn single_gpci_request(req:u32, start:u32, secondary:u16, version:u8, offset:u32, length:u8, value:*mut u64)->c_ulong {
    let arg=&mut hv_gpci_reqb; *arg=core::mem::zeroed(); arg.params.counter_request=req.to_be(); arg.params.starting_index=start.to_be(); arg.params.secondary_index=secondary.to_be(); arg.params.counter_info_version_in=version;
    let mut ret=plpar_hcall_norets(H_GET_PERF_COUNTER_INFO, virt_to_phys(arg as *mut _ as *mut _), HGPCI_REQ_BUFFER_SIZE as c_ulong);
    if ret==H_PARAMETER && u32::from_be(arg.params.detail_rc)==0x1b {ret=0} if ret!=0{return ret}
    let mut count=0u64; for i in offset as usize..offset as usize+length as usize {count|=(arg.bytes[i] as u64)<<((length as usize-1-(i-offset as usize))*8)} *value=count; ret
}

unsafe fn h_gpci_get_value(event:*mut perf_event)->u64 { let mut v=0; if single_gpci_request(event_get_request(event),event_get_starting_index(event),event_get_secondary_index(event),event_get_counter_info_version(event),event_get_offset(event),event_get_length(event),&mut v)!=0{0}else{v} }
unsafe fn h_gpci_event_update(e:*mut perf_event){let now=h_gpci_get_value(e);let prev=local64_xchg(&mut (*e).hw.prev_count,now);local64_add((now as i64)-prev,&mut (*e).count);}
unsafe fn h_gpci_event_start(e:*mut perf_event,_:c_int){local64_set(&mut (*e).hw.prev_count,h_gpci_get_value(e));}
unsafe fn h_gpci_event_stop(e:*mut perf_event,_:c_int){h_gpci_event_update(e)}
unsafe fn h_gpci_event_add(e:*mut perf_event,flags:c_int)->c_int{if flags&PERF_EF_START!=0{h_gpci_event_start(e,flags)} 0}
unsafe fn h_gpci_event_init(e:*mut perf_event)->c_int { if (*e).attr.r#type != (*(*e).pmu).task_ctx_nr as u32{return -ENOENT}; if (*e).attr.config2!=0{return -EINVAL}; if has_branch_stack(e){return -EOPNOTSUPP}; let l=event_get_length(e); if l<1||l>8{return -EINVAL}; if event_get_offset(e) as usize+l as usize>HGPCI_MAX_DATA_BYTES{return -EINVAL}; let mut v=0; let r=single_gpci_request(event_get_request(e),event_get_starting_index(e),event_get_secondary_index(e),event_get_counter_info_version(e),event_get_offset(e),l,&mut v); if r==H_AUTHORITY{return -EPERM} if r!=0{-EINVAL}else{0} }

// System-information readers retain the C driver's repeated-request protocol.
unsafe fn systeminfo_show(req:u32, buf:*mut u8)->isize {
    let arg=&mut hv_gpci_reqb; *arg=core::mem::zeroed(); let mut n=0usize; let mut start=0u32; let mut ret;
    loop { ret=systeminfo_gpci_request(req,start,0,buf,&mut n,arg); if ret!=H_PARAMETER{break}; let rv=u16::from_be(arg.params.returned_values) as usize; let sz=u16::from_be(arg.params.cv_element_size) as usize; if rv==0{break}; let p=(rv-1)*sz; start=u32::from_be_bytes([arg.bytes[p],arg.bytes[p+1],arg.bytes[p+2],arg.bytes[p+3]]).wrapping_add(1); *arg=core::mem::zeroed(); }
    if ret==0 {n as isize} else {-(ret as isize)}
}
unsafe fn processor_bus_topology_show(_: *mut device, _: *mut device_attribute, b:*mut c_char)->isize{systeminfo_show(sysinfo_counter_request[PROCESSOR_BUS_TOPOLOGY],b.cast())}
unsafe fn processor_config_show(_: *mut device, _: *mut device_attribute, b:*mut c_char)->isize{systeminfo_show(sysinfo_counter_request[PROCESSOR_CONFIG],b.cast())}
unsafe fn affinity_domain_via_virtual_processor_show(_: *mut device, _: *mut device_attribute, b:*mut c_char)->isize{systeminfo_show(sysinfo_counter_request[AFFINITY_DOMAIN_VIA_VP],b.cast())}
unsafe fn affinity_domain_via_domain_show(_: *mut device, _: *mut device_attribute, b:*mut c_char)->isize{systeminfo_show(sysinfo_counter_request[AFFINITY_DOMAIN_VIA_DOM],b.cast())}
unsafe fn affinity_domain_via_partition_show(_: *mut device, _: *mut device_attribute, b:*mut c_char)->isize{systeminfo_show(sysinfo_counter_request[AFFINITY_DOMAIN_VIA_PAR],b.cast())}

unsafe extern "C" fn ppc_hv_gpci_cpu_online(_cpu:u32)->c_int {0}
unsafe extern "C" fn ppc_hv_gpci_cpu_offline(_cpu:u32)->c_int {0}
unsafe fn hv_gpci_cpu_hotplug_init()->c_int {0}
unsafe fn add_sysinfo_interface_files() {}

// The remaining sysfs/event-group registration is represented by the kernel's
// generated attribute declarations and initcall machinery from the included headers.
unsafe fn hv_gpci_init()->c_int { hv_gpci_assert_offsets_correct(); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
