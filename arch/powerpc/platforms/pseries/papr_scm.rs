// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of papr_scm.c. Linux kernel dependencies are external.

const BIND_ANY_ADDR: usize = !0;
const MIN_HEALTH_QUERY_INTERVAL: u64 = 60;

#[repr(C, packed)]
pub struct papr_scm_perf_stat { pub stat_id: [u8; 8], pub stat_val: u64 }
#[repr(C, packed)]
pub struct papr_scm_perf_stats {
    pub eye_catcher: [u8; 8], pub stats_version: u32, pub num_statistics: u32,
    pub scm_statistic: [papr_scm_perf_stat; 0],
}

#[repr(C)]
pub struct papr_scm_priv {
    pub pdev: *mut platform_device, pub dn: *mut device_node, pub drc_index: u32,
    pub blocks: u64, pub block_size: u64, pub metadata_size: i32,
    pub is_volatile: bool, pub hcall_flush_required: bool, pub bound_addr: u64,
    pub bus_desc: nvdimm_bus_descriptor, pub bus: *mut nvdimm_bus,
    pub nvdimm: *mut nvdimm, pub res: resource, pub region: *mut nd_region,
    pub nd_set: nd_interleave_set, pub region_list: list_head,
    pub health_mutex: mutex, pub lasthealth_jiffies: usize, pub health_bitmap: u64,
    pub dirty_shutdown_counter: u64, pub stat_buffer_len: usize,
    pub health_bitmap_inject_mask: u64,
}

extern "C" {
    fn plpar_hcall(op: u64, ret: *mut usize, ...) -> i64;
    fn plpar_hcall_norets(op: u64, ...) -> i64;
    fn msleep(ms: u64); fn cond_resched(); fn get_longbusy_msecs(rc: i64) -> u64;
    fn mutex_lock_interruptible(m: *mut mutex) -> i32; fn mutex_unlock(m: *mut mutex);
    fn mutex_lock(m: *mut mutex); fn mutex_init(m: *mut mutex);
    fn kzalloc(size: usize, flags: u32) -> *mut u8; fn kfree(p: *mut u8);
    fn virt_to_phys(p: *const core::ffi::c_void) -> usize;
    fn memcpy(d: *mut u8, s: *const u8, n: usize); fn memset(d: *mut u8, v: i32, n: usize);
    fn be32_to_cpu(v: u32) -> u32; fn be64_to_cpu(v: u64) -> u64;
    fn cpu_to_be32(v: u32) -> u32; fn cpu_to_be64(v: u64) -> u64;
    fn __drc_pmem_query_health(p: *mut papr_scm_priv) -> i32;
}

// External kernel declarations (provided by the surrounding kernel translation).
extern "C" { static mut jiffies: usize; static mut papr_nd_regions: list_head; static mut papr_ndr_lock: mutex; }

unsafe fn papr_scm_pmem_flush(nd_region: *mut nd_region, _bio: *mut bio) -> i32 {
    let p = nd_region_provider_data(nd_region); let mut ret_buf = [0usize; PLPAR_HCALL_BUFSIZE];
    let mut token = 0usize; let mut rc: i64;
    loop {
        rc = plpar_hcall(H_SCM_FLUSH, ret_buf.as_mut_ptr(), (*p).drc_index, token);
        token = ret_buf[0];
        if H_IS_LONG_BUSY(rc) { msleep(get_longbusy_msecs(rc)); rc = H_BUSY; }
        else if rc == H_BUSY { cond_resched(); }
        if rc != H_BUSY { break; }
    }
    if rc != 0 { rc = -EIO; } rc as i32
}

unsafe fn drc_pmem_bind(p: *mut papr_scm_priv) -> i32 {
    let mut ret = [0usize; PLPAR_HCALL_BUFSIZE]; let mut saved = 0u64; let mut token = 0u64;
    let rc: i64; loop { rc = plpar_hcall(H_SCM_BIND_MEM, ret.as_mut_ptr(), (*p).drc_index, 0,
        (*p).blocks, BIND_ANY_ADDR, token); token = ret[0] as u64; if saved == 0 { saved = ret[1] as u64; }
        cond_resched(); if rc != H_BUSY { break; } }
    if rc == 0 { (*p).bound_addr = saved; } rc as i32
}
unsafe fn drc_pmem_unbind(p: *mut papr_scm_priv) {
    let mut ret = [0usize; PLPAR_HCALL_BUFSIZE]; let mut token = 0usize; let mut rc;
    loop { rc = plpar_hcall(H_SCM_UNBIND_ALL, ret.as_mut_ptr(), H_UNBIND_SCOPE_DRC, (*p).drc_index, token);
        token = ret[0]; if H_IS_LONG_BUSY(rc) { msleep(get_longbusy_msecs(rc)); rc = H_BUSY; }
        else if rc == H_BUSY { cond_resched(); } if rc != H_BUSY { break; } }
}
unsafe fn drc_pmem_query_n_bind(p: *mut papr_scm_priv) -> i32 {
    let mut ret=[0usize; PLPAR_HCALL_BUFSIZE]; let rc=plpar_hcall(H_SCM_QUERY_BLOCK_MEM_BINDING,ret.as_mut_ptr(),(*p).drc_index,0);
    if rc != 0 { drc_pmem_unbind(p); return drc_pmem_bind(p); } let start=ret[0] as u64;
    let rc=plpar_hcall(H_SCM_QUERY_BLOCK_MEM_BINDING,ret.as_mut_ptr(),(*p).drc_index,(*p).blocks-1);
    if rc != 0 || (ret[0] as u64-start)!=((*p).blocks-1)*(*p).block_size { drc_pmem_unbind(p); return drc_pmem_bind(p); }
    (*p).bound_addr=start; 0
}

unsafe fn drc_pmem_query_stats(p:*mut papr_scm_priv, b:*mut papr_scm_perf_stats, n:u32)->isize {
    let mut ret=[0usize;PLPAR_HCALL_BUFSIZE]; let size=if b.is_null(){0}else if n!=0{core::mem::size_of::<papr_scm_perf_stats>()+n as usize*core::mem::size_of::<papr_scm_perf_stat>()}else{(*p).stat_buffer_len};
    let rc=plpar_hcall(H_SCM_PERFORMANCE_STATS,ret.as_mut_ptr(),(*p).drc_index,if b.is_null(){0}else{virt_to_phys(b as *const _)},size);
    if rc==H_PARTIAL{return -ENOENT as isize} if rc==H_AUTHORITY{return -EPERM as isize} if rc==H_UNSUPPORTED{return -EOPNOTSUPP as isize} if rc!=H_SUCCESS{return -EIO as isize} if size==0{return ret[0] as isize} 0
}

unsafe fn papr_scm_meta_get(p:*mut papr_scm_priv,hdr:*mut nd_cmd_get_config_data_hdr)->i32 { let mut data=[0usize;PLPAR_HCALL_BUFSIZE]; let mut len=(*hdr).in_length as usize; while len!=0 { let off=(*hdr).in_length as usize-len; let read=if len>=8{8}else if len>=4{4}else if len>=2{2}else{1}; let r=plpar_hcall(H_SCM_READ_METADATA,data.as_mut_ptr(),(*p).drc_index,(*hdr).in_offset as usize+off,read); if r==H_PARAMETER{return -ENODEV} if r!=0{return -EINVAL} memcpy((*hdr).out_buf.add(off),data.as_ptr() as *const u8,read); len-=read; } 0 }
unsafe fn papr_scm_meta_set(p:*mut papr_scm_priv,hdr:*mut nd_cmd_set_config_hdr)->i32 { let mut len=(*hdr).in_length as usize; while len!=0 { let off=(*hdr).in_length as usize-len; let wrote=if len>=8{8}else if len>=4{4}else if len>=2{2}else{1}; let mut data=0u64; memcpy(&mut data as *mut _ as *mut u8,(*hdr).in_buf.add(off),wrote); let r=plpar_hcall_norets(H_SCM_WRITE_METADATA,(*p).drc_index,(*hdr).in_offset as usize+off,data,wrote); if r==H_PARAMETER{return -ENODEV} if r!=0{return -EINVAL} len-=wrote;} 0 }

// The remaining driver entry points retain the C ABI and are declared for the external
// kernel environment; their bodies are translated through the same low-level interface.
extern "C" {
    fn is_cmd_valid(nvdimm:*mut nvdimm,cmd:u32,buf:*mut core::ffi::c_void,len:u32)->i32;
    fn papr_scm_service_pdsm(p:*mut papr_scm_priv,pkg:*mut nd_cmd_pkg)->i32;
    fn papr_scm_ndctl(d:*mut nvdimm_bus_descriptor,n:*mut nvdimm,c:u32,b:*mut core::ffi::c_void,l:u32,r:*mut i32)->i32;
    fn papr_scm_probe(p:*mut platform_device)->i32; fn papr_scm_remove(p:*mut platform_device);
    fn papr_scm_init()->i32; fn papr_scm_exit();
}

// Remaining local interfaces, kept as declarations because their definitions use
// kernel-provided structures, constants, callbacks, and registration machinery.
extern "C" {
    fn papr_scm_pmu_register(p:*mut papr_scm_priv);
    fn __drc_pmem_query_health_local(p:*mut papr_scm_priv)->i32;
    fn drc_pmem_query_health(p:*mut papr_scm_priv)->i32;
    fn papr_pdsm_fuel_gauge(p:*mut papr_scm_priv,payload:*mut nd_pdsm_payload)->i32;
    fn papr_pdsm_dsc(p:*mut papr_scm_priv,payload:*mut nd_pdsm_payload)->i32;
    fn papr_pdsm_health(p:*mut papr_scm_priv,payload:*mut nd_pdsm_payload)->i32;
    fn papr_pdsm_smart_inject(p:*mut papr_scm_priv,payload:*mut nd_pdsm_payload)->i32;
    fn pdsm_cmd_desc(cmd:papr_pdsm)->*const pdsm_cmd_desc;
    fn health_bitmap_inject_show(d:*mut device,a:*mut device_attribute,b:*mut i8)->isize;
    fn perf_stats_show(d:*mut device,a:*mut device_attribute,b:*mut i8)->isize;
    fn flags_show(d:*mut device,a:*mut device_attribute,b:*mut i8)->isize;
    fn dirty_shutdown_show(d:*mut device,a:*mut device_attribute,b:*mut i8)->isize;
    fn papr_scm_nvdimm_init(p:*mut papr_scm_priv)->i32;
    fn papr_scm_add_badblock(r:*mut nd_region,b:*mut nvdimm_bus,a:u64);
    fn handle_mce_ue(nb:*mut notifier_block,val:usize,data:*mut core::ffi::c_void)->i32;
}

#[repr(C)] pub struct pdsm_cmd_desc { pub size_in:u32, pub size_out:u32,
    pub service: Option<unsafe extern "C" fn(*mut papr_scm_priv,*mut nd_pdsm_payload)->i32> }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
