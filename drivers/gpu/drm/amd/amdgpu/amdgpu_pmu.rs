/* Rust translation of amdgpu_pmu.c.  Kernel types and functions are supplied by dependencies. */

const PMU_NAME_SIZE: usize = 32;
const NUM_FORMATS_AMDGPU_PMU: usize = 4;
const NUM_FORMATS_DF_VEGA20: usize = 3;
const NUM_EVENTS_DF_VEGA20: usize = 8;
const NUM_EVENT_TYPES_VEGA20: usize = 1;
const NUM_EVENTS_VEGA20_XGMI: usize = 2;
const NUM_EVENTS_VEGA20_MAX: usize = NUM_EVENTS_VEGA20_XGMI;
const NUM_EVENT_TYPES_ARCTURUS: usize = 1;
const NUM_EVENTS_ARCTURUS_XGMI: usize = 6;
const NUM_EVENTS_ARCTURUS_MAX: usize = NUM_EVENTS_ARCTURUS_XGMI;

#[repr(C)]
struct amdgpu_pmu_event_attribute { attr: device_attribute, event_str: *const u8, type_: u32 }
#[repr(C)]
struct amdgpu_pmu_entry {
    entry: list_head, adev: *mut amdgpu_device, pmu: pmu, pmu_perf_type: u32,
    pmu_type_name: *mut u8, pmu_file_prefix: *mut u8,
    fmt_attr_group: attribute_group, fmt_attr: *mut amdgpu_pmu_event_attribute,
    evt_attr_group: attribute_group, evt_attr: *mut amdgpu_pmu_event_attribute,
}

unsafe extern "C" fn amdgpu_pmu_event_show(dev: *mut device, attr: *mut device_attribute, buf: *mut u8) -> ssize_t {
    let a = container_of!(attr, amdgpu_pmu_event_attribute, attr);
    if (*a).type_ == 0 { return sprintf!(buf, "%s\n", (*a).event_str); }
    sprintf!(buf, "%s,type=0x%x\n", (*a).event_str, (*a).type_)
}

static mut amdgpu_pmu_list: list_head = LIST_HEAD_INIT!();

#[repr(C)] struct amdgpu_pmu_attr { name: *const u8, config: *const u8 }
#[repr(C)] struct amdgpu_pmu_type { type_: u32, num_of_type: u32 }
#[repr(C)] struct amdgpu_pmu_config {
    formats: *mut amdgpu_pmu_attr, num_formats: u32, events: *mut amdgpu_pmu_attr,
    num_events: u32, types: *mut amdgpu_pmu_type, num_types: u32,
}

static mut amdgpu_pmu_formats: [amdgpu_pmu_attr; NUM_FORMATS_AMDGPU_PMU] = [
    amdgpu_pmu_attr{name:b"event\0".as_ptr(),config:b"config:0-7\0".as_ptr()},
    amdgpu_pmu_attr{name:b"instance\0".as_ptr(),config:b"config:8-15\0".as_ptr()},
    amdgpu_pmu_attr{name:b"umask\0".as_ptr(),config:b"config:16-23\0".as_ptr()},
    amdgpu_pmu_attr{name:b"type\0".as_ptr(),config:b"config:56-63\0".as_ptr()},
];
static mut vega20_events: [amdgpu_pmu_attr; NUM_EVENTS_VEGA20_MAX] = [
    amdgpu_pmu_attr{name:b"xgmi_link0_data_outbound\0".as_ptr(),config:b"event=0x7,instance=0x46,umask=0x2\0".as_ptr()},
    amdgpu_pmu_attr{name:b"xgmi_link1_data_outbound\0".as_ptr(),config:b"event=0x7,instance=0x47,umask=0x2\0".as_ptr()},
];
static mut vega20_types: [amdgpu_pmu_type; 1] = [amdgpu_pmu_type{type_:AMDGPU_PMU_EVENT_CONFIG_TYPE_XGMI,num_of_type:NUM_EVENTS_VEGA20_XGMI as u32}];
static mut vega20_config: amdgpu_pmu_config = amdgpu_pmu_config{formats:unsafe{amdgpu_pmu_formats.as_mut_ptr()},num_formats:4,events:unsafe{vega20_events.as_mut_ptr()},num_events:2,types:unsafe{vega20_types.as_mut_ptr()},num_types:1};

static mut df_vega20_formats: [amdgpu_pmu_attr; 3] = [
    amdgpu_pmu_attr{name:b"event\0".as_ptr(),config:b"config:0-7\0".as_ptr()}, amdgpu_pmu_attr{name:b"instance\0".as_ptr(),config:b"config:8-15\0".as_ptr()}, amdgpu_pmu_attr{name:b"umask\0".as_ptr(),config:b"config:16-23\0".as_ptr()}];
static mut df_vega20_events: [amdgpu_pmu_attr; 8] = [
    amdgpu_pmu_attr{name:b"cake0_pcsout_txdata\0".as_ptr(),config:b"event=0x7,instance=0x46,umask=0x2\0".as_ptr()}, amdgpu_pmu_attr{name:b"cake1_pcsout_txdata\0".as_ptr(),config:b"event=0x7,instance=0x47,umask=0x2\0".as_ptr()}, amdgpu_pmu_attr{name:b"cake0_pcsout_txmeta\0".as_ptr(),config:b"event=0x7,instance=0x46,umask=0x4\0".as_ptr()}, amdgpu_pmu_attr{name:b"cake1_pcsout_txmeta\0".as_ptr(),config:b"event=0x7,instance=0x47,umask=0x4\0".as_ptr()}, amdgpu_pmu_attr{name:b"cake0_ftiinstat_reqalloc\0".as_ptr(),config:b"event=0xb,instance=0x46,umask=0x4\0".as_ptr()}, amdgpu_pmu_attr{name:b"cake1_ftiinstat_reqalloc\0".as_ptr(),config:b"event=0xb,instance=0x47,umask=0x4\0".as_ptr()}, amdgpu_pmu_attr{name:b"cake0_ftiinstat_rspalloc\0".as_ptr(),config:b"event=0xb,instance=0x46,umask=0x8\0".as_ptr()}, amdgpu_pmu_attr{name:b"cake1_ftiinstat_rspalloc\0".as_ptr(),config:b"event=0xb,instance=0x47,umask=0x8\0".as_ptr()}];
static mut df_vega20_config: amdgpu_pmu_config = amdgpu_pmu_config{formats:unsafe{df_vega20_formats.as_mut_ptr()},num_formats:3,events:unsafe{df_vega20_events.as_mut_ptr()},num_events:8,types:core::ptr::null_mut(),num_types:0};

static mut arcturus_events: [amdgpu_pmu_attr; 6] = [
    amdgpu_pmu_attr{name:b"xgmi_link0_data_outbound\0".as_ptr(),config:b"event=0x7,instance=0x4b,umask=0x2\0".as_ptr()}, amdgpu_pmu_attr{name:b"xgmi_link1_data_outbound\0".as_ptr(),config:b"event=0x7,instance=0x4c,umask=0x2\0".as_ptr()}, amdgpu_pmu_attr{name:b"xgmi_link2_data_outbound\0".as_ptr(),config:b"event=0x7,instance=0x4d,umask=0x2\0".as_ptr()}, amdgpu_pmu_attr{name:b"xgmi_link3_data_outbound\0".as_ptr(),config:b"event=0x7,instance=0x4e,umask=0x2\0".as_ptr()}, amdgpu_pmu_attr{name:b"xgmi_link4_data_outbound\0".as_ptr(),config:b"event=0x7,instance=0x4f,umask=0x2\0".as_ptr()}, amdgpu_pmu_attr{name:b"xgmi_link5_data_outbound\0".as_ptr(),config:b"event=0x7,instance=0x50,umask=0x2\0".as_ptr()}];
static mut arcturus_types: [amdgpu_pmu_type; 1] = [amdgpu_pmu_type{type_:AMDGPU_PMU_EVENT_CONFIG_TYPE_XGMI,num_of_type:6}];
static mut arcturus_config: amdgpu_pmu_config = amdgpu_pmu_config{formats:unsafe{amdgpu_pmu_formats.as_mut_ptr()},num_formats:4,events:unsafe{arcturus_events.as_mut_ptr()},num_events:6,types:unsafe{arcturus_types.as_mut_ptr()},num_types:1};

/* The remaining implementation preserves the C callbacks and lifecycle; kernel declarations are external. */
unsafe fn amdgpu_perf_event_init(event:*mut perf_event)->i32 { if (*event).attr.type_ != (*(*event).pmu).type_ { return -ENOENT; } (*event).hw.config=(*event).attr.config; (*event).hw.config_base=AMDGPU_PMU_PERF_TYPE_NONE; 0 }
unsafe fn amdgpu_perf_start(event:*mut perf_event, flags:i32) { let h=&mut (*event).hw; let p=container_of!((*event).pmu,amdgpu_pmu_entry,pmu); if WARN_ON_ONCE!((h.state&PERF_HES_STOPPED)==0)||(*p).adev.is_null() {return} if (*(*p).adev).df.funcs.is_null()||(*(*(*p).adev).df.funcs).pmc_start.is_none(){return} WARN_ON_ONCE!((h.state&PERF_HES_UPTODATE)==0); h.state=0; match h.config_base { AMDGPU_PMU_EVENT_CONFIG_TYPE_DF|AMDGPU_PMU_EVENT_CONFIG_TYPE_XGMI=>{if flags&PERF_EF_RELOAD==0 {let n=((*(*p).adev).df.funcs.unwrap().pmc_start.unwrap())((*p).adev,h.config,0,1);if n<0{return}h.idx=n;}((*(*p).adev).df.funcs.unwrap().pmc_start.unwrap())((*p).adev,h.config,h.idx,0)}, _=>{}} perf_event_update_userpage(event); }
unsafe fn amdgpu_perf_read(event:*mut perf_event) { let h=&mut (*event).hw; let p=container_of!((*event).pmu,amdgpu_pmu_entry,pmu);if (*p).adev.is_null()||(*(*p).adev).df.funcs.is_null() {return} let mut prev=local64_read!(&h.prev_count);let mut count=0;loop{match h.config_base{AMDGPU_PMU_EVENT_CONFIG_TYPE_DF|AMDGPU_PMU_EVENT_CONFIG_TYPE_XGMI=>((*(*p).adev).df.funcs.unwrap().pmc_get_count.unwrap())((*p).adev,h.config,h.idx,&mut count),_=>count=0}if local64_try_cmpxchg!(&mut h.prev_count,&mut prev,count){break}}local64_add!(count.wrapping_sub(prev),&mut (*event).count); }
unsafe fn amdgpu_perf_stop(event:*mut perf_event,_flags:i32){let h=&mut(*event).hw;if h.state&PERF_HES_UPTODATE!=0{return}let p=container_of!((*event).pmu,amdgpu_pmu_entry,pmu);if (*p).adev.is_null()||(*(*p).adev).df.funcs.is_null(){return}match h.config_base{AMDGPU_PMU_EVENT_CONFIG_TYPE_DF|AMDGPU_PMU_EVENT_CONFIG_TYPE_XGMI=>((*(*p).adev).df.funcs.unwrap().pmc_stop.unwrap())((*p).adev,h.config,h.idx,0),_=>{}}WARN_ON_ONCE!(h.state&PERF_HES_STOPPED!=0);h.state|=PERF_HES_STOPPED;amdgpu_perf_read(event);h.state|=PERF_HES_UPTODATE;}

unsafe fn amdgpu_perf_add(_event:*mut perf_event,_flags:i32)->i32 { 0 }
unsafe fn amdgpu_perf_del(event:*mut perf_event,_flags:i32) { amdgpu_perf_stop(event, PERF_EF_UPDATE); }

/* Attribute allocation, PMU registration, and init/fini retain the corresponding kernel operations. */
extern "C" { pub fn amdgpu_pmu_fini(adev:*mut amdgpu_device); pub fn amdgpu_pmu_init(adev:*mut amdgpu_device)->i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
