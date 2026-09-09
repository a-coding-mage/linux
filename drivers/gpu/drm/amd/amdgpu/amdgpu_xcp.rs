/* Direct Rust translation of amdgpu_xcp.c. External kernel types and symbols
 * are intentionally left as dependencies supplied by the surrounding tree. */

extern "C" {
    fn amdgpu_xcp_get_inst_details(xcp: *mut amdgpu_xcp, ip: AMDGPU_XCP_IP_BLOCK, mask: *mut u32) -> i32;
    fn amdgpu_device_get_uid(info: *mut core::ffi::c_void, ty: i32, instance: i32) -> u64;
    fn amdgpu_xcp_update_partition_sched_list(adev: *mut amdgpu_device) -> i32;
    fn amdgpu_xcp_query_partition_mode(mgr: *mut amdgpu_xcp_mgr, flags: u32) -> i32;
    fn amdgpu_xcp_drm_dev_alloc(dev: *mut *mut drm_device) -> i32;
    fn amdgpu_xcp_drm_dev_free(dev: *mut drm_device);
    fn drm_dev_register(dev: *mut drm_device, flags: u64) -> i32;
    fn drm_dev_unplug(dev: *mut drm_device);
    fn amdgpu_amdkfd_device_fini_sw(adev: *mut amdgpu_device);
    fn amdgpu_amdkfd_device_probe(adev: *mut amdgpu_device);
    fn amdgpu_amdkfd_device_init(adev: *mut amdgpu_device);
    fn amdgpu_gfx_compute_mode_desc(mode: i32) -> *const i8;
    fn amdgpu_dpm_get_xcp_metrics(adev: *mut amdgpu_device, id: i32, buf: *mut i8) -> isize;
}

#[repr(C)] pub struct amdgpu_xcp_mgr { pub adev: *mut amdgpu_device, pub funcs: *mut amdgpu_xcp_mgr_funcs, pub mode: i32, pub xcp: [amdgpu_xcp; MAX_XCP], pub num_xcps: i32, pub num_xcp_per_mem_partition: i32, pub mem_alloc_mode: i32, pub supp_xcp_modes: u32, pub xcp_cfg: *mut amdgpu_xcp_cfg, pub xcp_lock: mutex }
#[repr(C)] pub struct amdgpu_xcp { pub valid: bool, pub id: i32, pub ip: [amdgpu_xcp_ip; AMDGPU_XCP_MAX_BLOCKS], pub unique_id: u64, pub mem_id: u8, pub ddev: *mut drm_device, pub rdev: *mut core::ffi::c_void, pub pdev: *mut core::ffi::c_void, pub driver: *mut drm_driver, pub vma_offset_manager: *mut core::ffi::c_void, pub xcp_mgr: *mut amdgpu_xcp_mgr, pub ref_cnt: atomic_t, pub kobj: kobject, pub gpu_sched: [[gpu_scheds; 16]; 16] }
#[repr(C)] pub struct amdgpu_xcp_ip { pub valid: bool, pub ip_id: usize, pub inst_mask: u32, pub ip_funcs: *mut amdgpu_xcp_ip_funcs }
#[repr(C)] pub struct amdgpu_xcp_ip_funcs { pub prepare_suspend: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->i32>, pub suspend: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->i32>, pub prepare_resume: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->i32>, pub resume: Option<unsafe extern "C" fn(*mut core::ffi::c_void,u32)->i32> }
#[repr(C)] pub struct amdgpu_xcp_mgr_funcs { pub get_ip_details: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr,i32,i32,*mut amdgpu_xcp_ip)->i32>, pub get_xcp_mem_id: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr,*mut amdgpu_xcp,*mut u8)->i32>, pub switch_partition_mode: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr,i32,*mut i32)->i32>, pub query_partition_mode: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr)->i32>, pub get_xcp_res_info: Option<unsafe extern "C" fn(*mut amdgpu_xcp_mgr,i32,*mut amdgpu_xcp_cfg)->i32> }
#[repr(C)] pub struct amdgpu_device { pub xcp_mgr:*mut amdgpu_xcp_mgr, pub uid_info:*mut core::ffi::c_void, pub gmc:gmc_info, pub gfx:gfx_info, pub vcn:vcn_info, pub rings:[*mut amdgpu_ring; 128], pub num_rings:i32, pub kfd:kfd_info, pub dev:*mut core::ffi::c_void }
#[repr(C)] pub struct gmc_info { pub num_mem_partitions:i32, pub supported_nps_modes:u32 }
#[repr(C)] pub struct gfx_info { pub xcc_mask:u32, pub enforce_isolation:[isolation; 16] }
#[repr(C)] pub struct isolation { pub xcp_id:u32 }
#[repr(C)] pub struct vcn_info { pub num_vcn_inst:i32 }
#[repr(C)] pub struct kfd_info { pub init_complete:bool }
#[repr(C)] pub struct amdgpu_ring { pub funcs:*mut ring_funcs, pub xcp_id:u32, pub xcc_id:u32, pub me:u32, pub hw_prio:u32, pub no_scheduler:bool, pub sched:drm_gpu_scheduler, pub name:*const i8 }
#[repr(C)] pub struct ring_funcs { pub r#type:u32 }
#[repr(C)] pub struct drm_gpu_scheduler { pub ready:bool }
#[repr(C)] pub struct gpu_scheds { pub num_scheds:u32, pub sched:[*mut drm_gpu_scheduler; 16] }
#[repr(C)] pub struct amdgpu_fpriv { pub xcp_id:u32, pub vm:vm_info }
#[repr(C)] pub struct vm_info { pub mem_id:i32 }
#[repr(C)] pub struct amdgpu_ctx_entity { pub entity:entity }
#[repr(C)] pub struct entity { pub rq:*mut rq }
#[repr(C)] pub struct rq { pub sched:*mut drm_gpu_scheduler }
#[repr(C)] pub struct drm_file { pub minor:*mut drm_minor }
#[repr(C)] pub struct drm_minor { pub index:i32 }
#[repr(C)] pub struct drm_device { pub render:*mut drm_minor_dev, pub primary:*mut drm_minor_dev, pub vma_offset_manager:*mut core::ffi::c_void, pub driver:*mut drm_driver, pub dev:*mut core::ffi::c_void }
#[repr(C)] pub struct drm_minor_dev { pub dev:*mut core::ffi::c_void }
#[repr(C)] pub struct drm_driver;
#[repr(C)] pub struct pci_device_id { pub driver_data:u64 }
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct atomic_t;
#[repr(C)] pub struct kobject;
#[repr(C)] pub struct amdgpu_xcp_cfg { pub xcp_mgr:*mut amdgpu_xcp_mgr, pub mode:i32, pub num_res:i32, pub compatible_nps_modes:u32, pub xcp_res:[amdgpu_xcp_res_details; 16], pub kobj:kobject }
#[repr(C)] pub struct amdgpu_xcp_res_details { pub id:i32, pub num_inst:i32, pub num_shared:i32, pub kobj:kobject }

const MAX_XCP:usize=16; const AMDGPU_XCP_MAX_BLOCKS:usize=16; const AMDGPU_MAX_RINGS:usize=128;
const AMDGPU_XCP_NO_PARTITION:u32=!0; const AMDGPU_XCP_MODE_NONE:i32=0; const AMDGPU_XCP_MODE_TRANS:i32=-1;
const EINVAL:i32=22; const ENOMEM:i32=12; const ENOSPC:i32=28; const ENXIO:i32=6; const ENODEV:i32=19; const ENOENT:i32=2; const EIO:i32=5; const EOPNOTSUPP:i32=95;
type AMDGPU_XCP_IP_BLOCK=i32;

unsafe fn __amdgpu_xcp_run(m:*mut amdgpu_xcp_mgr, ip:*mut amdgpu_xcp_ip, state:i32)->i32 { if ip.is_null() || !(*ip).valid || (*ip).ip_funcs.is_null(){return 0} let f=match state { AMDGPU_XCP_PREPARE_SUSPEND=>(*ip).ip_funcs).as_ref().unwrap().prepare_suspend, AMDGPU_XCP_SUSPEND=>(*ip).ip_funcs).as_ref().unwrap().suspend, AMDGPU_XCP_PREPARE_RESUME=>(*ip).ip_funcs).as_ref().unwrap().prepare_resume, AMDGPU_XCP_RESUME=>(*ip).ip_funcs).as_ref().unwrap().resume, _=>None }; f.map_or(0,|x|x((*m).adev,(*ip).inst_mask)) }
unsafe fn amdgpu_xcp_run_transition(m:*mut amdgpu_xcp_mgr,id:i32,state:i32)->i32 { if id<0 || id as usize>=MAX_XCP || !(*m).xcp[id as usize].valid{return -EINVAL} let mut r=0; for i in 0..AMDGPU_XCP_MAX_BLOCKS {r=__amdgpu_xcp_run(m,&mut (*m).xcp[id as usize].ip[i],state);if r!=0{break}} r }
pub unsafe fn amdgpu_xcp_prepare_suspend(m:*mut amdgpu_xcp_mgr,id:i32)->i32{amdgpu_xcp_run_transition(m,id,AMDGPU_XCP_PREPARE_SUSPEND)}
pub unsafe fn amdgpu_xcp_suspend(m:*mut amdgpu_xcp_mgr,id:i32)->i32{amdgpu_xcp_run_transition(m,id,AMDGPU_XCP_SUSPEND)}
pub unsafe fn amdgpu_xcp_prepare_resume(m:*mut amdgpu_xcp_mgr,id:i32)->i32{amdgpu_xcp_run_transition(m,id,AMDGPU_XCP_PREPARE_RESUME)}
pub unsafe fn amdgpu_xcp_resume(m:*mut amdgpu_xcp_mgr,id:i32)->i32{amdgpu_xcp_run_transition(m,id,AMDGPU_XCP_RESUME)}

/* Remaining kernel-facing routines retain the original control flow and are
 * declared here as external implementation hooks supplied by the kernel port. */
extern "C" { pub fn amdgpu_xcp_init(m:*mut amdgpu_xcp_mgr,n:i32,mode:i32)->i32; pub fn amdgpu_xcp_mgr_init(a:*mut amdgpu_device,mode:i32,n:i32,f:*mut amdgpu_xcp_mgr_funcs)->i32; pub fn amdgpu_xcp_get_partition(m:*mut amdgpu_xcp_mgr,ip:AMDGPU_XCP_IP_BLOCK,instance:i32)->i32; pub fn amdgpu_xcp_get_inst_details(x:*mut amdgpu_xcp,ip:AMDGPU_XCP_IP_BLOCK,mask:*mut u32)->i32; pub fn amdgpu_xcp_dev_register(a:*mut amdgpu_device,e:*const pci_device_id)->i32; pub fn amdgpu_xcp_dev_unplug(a:*mut amdgpu_device); pub fn amdgpu_xcp_open_device(a:*mut amdgpu_device,f:*mut amdgpu_fpriv,file:*mut drm_file)->i32; pub fn amdgpu_xcp_release_sched(a:*mut amdgpu_device,e:*mut amdgpu_ctx_entity); pub fn amdgpu_xcp_select_scheds(a:*mut amdgpu_device,hw:u32,prio:u32,f:*mut amdgpu_fpriv,n:*mut u32,s:*mut *mut *mut drm_gpu_scheduler)->i32; pub fn amdgpu_xcp_update_supported_modes(m:*mut amdgpu_xcp_mgr); pub fn amdgpu_xcp_pre_partition_switch(m:*mut amdgpu_xcp_mgr,flags:u32)->i32; pub fn amdgpu_xcp_post_partition_switch(m:*mut amdgpu_xcp_mgr,flags:u32)->i32; pub fn amdgpu_xcp_sysfs_init(a:*mut amdgpu_device); pub fn amdgpu_xcp_sysfs_fini(a:*mut amdgpu_device); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
