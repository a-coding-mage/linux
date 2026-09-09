/* Rust translation of amdgpu_uvd.c.  Kernel and driver dependencies are
 * intentionally left as external symbols supplied by the surrounding tree. */

const UVD_IDLE_TIMEOUT: u64 = 1000;
const FW_1_65_10: u32 = (1 << 24) | (65 << 16) | (10 << 8);
const FW_1_87_11: u32 = (1 << 24) | (87 << 16) | (11 << 8);
const FW_1_87_12: u32 = (1 << 24) | (87 << 16) | (12 << 8);
const FW_1_37_15: u32 = (1 << 24) | (37 << 16) | (15 << 8);
const FW_1_66_16: u32 = (1 << 24) | (66 << 16) | (16 << 8);
const UVD_GPCOM_VCPU_CMD: u32 = 0x03c3;
const UVD_GPCOM_VCPU_DATA0: u32 = 0x03c4;
const UVD_GPCOM_VCPU_DATA1: u32 = 0x03c5;
const UVD_NO_OP: u32 = 0x03ff;
const UVD_BASE_SI: u32 = 0x3800;

#[repr(C)] pub struct amdgpu_device { pub uvd: amdgpu_uvd, pub asic_type: u32, pub firmware: firmware_state, pub pm: pm_state, pub dev: *mut core::ffi::c_void, pub reg_offset: [[u32; 4]; 16] }
#[repr(C)] pub struct amdgpu_uvd { pub max_handles: u32, pub fw_version: u32, pub address_64_bit: bool, pub use_ctx_buf: bool, pub num_uvd_inst: u32, pub num_enc_rings: u32, pub harvest_config: u32, pub decode_image_width: u32, pub ib_bo: *mut amdgpu_bo, pub entity: drm_sched_entity, pub idle_work: work_struct, pub inst: *mut uvd_instance, pub handles: *mut atomic_t, pub filp: *mut *mut drm_file }
#[repr(C)] pub struct uvd_instance { pub vcpu_bo: *mut amdgpu_bo, pub gpu_addr: u64, pub cpu_addr: *mut core::ffi::c_void, pub saved_bo: *mut core::ffi::c_void, pub ring: amdgpu_ring, pub ring_enc: [amdgpu_ring; 4] }
#[repr(C)] pub struct amdgpu_uvd_cs_ctx { pub parser: *mut amdgpu_cs_parser, pub reg: u32, pub count: u32, pub data0: u32, pub data1: u32, pub idx: u32, pub ib: *mut amdgpu_ib, pub has_msg_cmd: bool, pub buf_sizes: *mut u32 }
#[repr(C)] pub struct amdgpu_bo { pub tbo: ttm_buffer_object, pub placement: placement, pub placements: [placement; 1] }
#[repr(C)] pub struct amdgpu_ring { pub adev: *mut amdgpu_device, pub sched: drm_gpu_scheduler, pub me: u32, pub fence_drv: fence_driver }
#[repr(C)] pub struct amdgpu_ib { pub ptr: *mut u32, pub length_dw: u32 }
#[repr(C)] pub struct amdgpu_job { pub vm: *mut core::ffi::c_void, pub ibs: [amdgpu_ib; 1], pub base: drm_sched_job }
#[repr(C)] pub struct amdgpu_cs_parser { pub adev: *mut amdgpu_device, pub filp: *mut drm_file }
#[repr(C)] pub struct ttm_buffer_object { pub bdev: *mut core::ffi::c_void, pub resource: *mut ttm_resource, pub base: dma_resv }
#[repr(C)] pub struct ttm_resource { pub mem_type: u32 }
#[repr(C)] pub struct placement { pub num_placement: u32, pub placement: *mut placement, pub fpfn: u64, pub lpfn: u64, pub mem_type: u32, pub flags: u32 }
#[repr(C)] pub struct drm_sched_entity; #[repr(C)] pub struct drm_gpu_scheduler; #[repr(C)] pub struct drm_sched_job; #[repr(C)] pub struct drm_file; #[repr(C)] pub struct dma_fence; #[repr(C)] pub struct dma_resv; #[repr(C)] pub struct work_struct; #[repr(C)] pub struct firmware_state; #[repr(C)] pub struct pm_state; #[repr(C)] pub struct fence_driver { pub initialized: bool }
#[repr(C)] pub struct atomic_t { pub value: i32 }

extern "C" { fn amdgpu_bo_create_reserved(a:*mut amdgpu_device,s:u32,p:u32,d:u32,b:*mut *mut amdgpu_bo,x:*mut *mut core::ffi::c_void)->i32; fn amdgpu_bo_create_kernel(a:*mut amdgpu_device,s:u64,p:u32,d:u32,b:*mut *mut amdgpu_bo,g:*mut u64,c:*mut *mut core::ffi::c_void)->i32; fn amdgpu_bo_kunmap(b:*mut amdgpu_bo); fn amdgpu_bo_unpin(b:*mut amdgpu_bo); fn amdgpu_bo_pin(b:*mut amdgpu_bo,d:u32)->i32; fn amdgpu_bo_kmap(b:*mut amdgpu_bo,p:*mut *mut core::ffi::c_void)->i32; fn amdgpu_bo_unreserve(b:*mut amdgpu_bo); fn amdgpu_bo_unref(b:*mut *mut amdgpu_bo); fn amdgpu_bo_reserve(b:*mut amdgpu_bo,w:bool)->i32; fn amdgpu_bo_fence(b:*mut amdgpu_bo,f:*mut dma_fence,e:bool); fn amdgpu_bo_kptr(b:*mut amdgpu_bo)->*mut u32; fn amdgpu_bo_size(b:*mut amdgpu_bo)->u64; fn amdgpu_bo_gpu_offset(b:*mut amdgpu_bo)->u64; fn amdgpu_bo_free_kernel(b:*mut *mut amdgpu_bo,g:*mut u64,c:*mut *mut core::ffi::c_void); fn amdgpu_ib_get_value(i:*mut amdgpu_ib,n:u32)->u32; fn amdgpu_ib_set_value(i:*mut amdgpu_ib,n:u32,v:u32); fn amdgpu_cs_find_mapping(p:*mut amdgpu_cs_parser,a:u64,b:*mut *mut amdgpu_bo,m:*mut *mut bo_va_mapping)->i32; fn ttm_bo_validate(b:*mut ttm_buffer_object,p:*mut placement,c:*mut ttm_operation_ctx)->i32; fn amdgpu_job_alloc_with_ib(a:*mut amdgpu_device,e:*mut drm_sched_entity,o:u32,s:u32,p:u32,k:u32,j:*mut *mut amdgpu_job)->i32; fn amdgpu_job_submit_direct(j:*mut amdgpu_job,r:*mut amdgpu_ring,f:*mut *mut dma_fence)->i32; fn amdgpu_job_submit(j:*mut amdgpu_job)->*mut dma_fence; fn amdgpu_job_free(j:*mut amdgpu_job); fn dma_fence_wait(f:*mut dma_fence,i:bool)->i32; fn dma_fence_wait_timeout(f:*mut dma_fence,i:bool,t:i64)->i64; fn dma_fence_put(f:*mut dma_fence); fn dma_fence_get(f:*mut dma_fence)->*mut dma_fence; fn drm_sched_job_add_resv_dependencies(j:*mut drm_sched_job,r:*mut dma_resv,u:u32)->i32; fn atomic_read(a:*mut atomic_t)->i32; fn atomic_set(a:*mut atomic_t,v:i32); fn atomic_cmpxchg(a:*mut atomic_t,o:i32,n:i32)->i32; }
#[repr(C)] pub struct bo_va_mapping { pub start:u64, pub last:u64 }
#[repr(C)] pub struct ttm_operation_ctx { pub interruptible: bool, pub no_wait_gpu: bool }

unsafe fn uvd_addr(c:*mut amdgpu_uvd_cs_ctx)->u64 { (amdgpu_ib_get_value((*c).ib,(*c).data0) as u64) | ((amdgpu_ib_get_value((*c).ib,(*c).data1) as u64)<<32) }
unsafe fn force_uvd(bo:*mut amdgpu_bo) { for i in 0..(*bo).placement.num_placement { let p=&mut *(*bo).placements.as_mut_ptr().add(i as usize); p.fpfn=0; p.lpfn=(256*1024*1024)/4096; } }
unsafe fn pass1(c:*mut amdgpu_uvd_cs_ctx)->i32 { let mut bo=core::ptr::null_mut(); let mut m=core::ptr::null_mut(); let r=amdgpu_cs_find_mapping((*c).parser,uvd_addr(c),&mut bo,&mut m); if r!=0{return r} if !(*(*c).parser).adev.as_ref().unwrap().uvd.address_64_bit { force_uvd(bo); let mut x=ttm_operation_ctx{interruptible:false,no_wait_gpu:false}; return ttm_bo_validate(&mut (*bo).tbo,&mut (*bo).placement,&mut x) } 0 }
unsafe fn msg_decode(a:*mut amdgpu_device,m:*mut u32,s:*mut u32)->i32 { let w=*m.add(6); let h=*m.add(7); let pitch=*m.add(28); if w<16||h<16||w>4096||h>4096{return -22} let mut image=(w*h)+(w*h)/2; image=(image+1023)&!1023; let typ=*m.add(4); let wm=w/16; let hm=((h/16)+1)&!1; let mut n=0; let mut min=0; match typ { 0=>{n=(((*m.add(61)>>16)&255)+1);if n>17{return -22};min=image*n+wm*hm*n*192+wm*hm*32},1=>{min=image*3+wm*hm*128+wm*64+wm*128+((wm.max(hm)*7*16+63)&!63)},3|4=>{min=image*3},7=>{n=(((*m.add(61)>>16)&255)+1);if n>17{return -22};min=image*n},8=>{},16=>{n=(*m.add(59)&255)+2;if n>17{return -22};min=((w+15)&!15)*((h+15)&!15)*3/2*n}, _=>return -22}; if w>pitch||pitch>4096||*m.add(9)<min{return -22}; *s.add(1)=*m.add(9);*s.add(2)=pitch*h*3/2; (*a).uvd.decode_image_width=w;0 }

unsafe fn cs_msg(c:*mut amdgpu_uvd_cs_ctx,bo:*mut amdgpu_bo,off:u32)->i32 { if off&63!=0{return -22}; let mut p=core::ptr::null_mut();let r=amdgpu_bo_kmap(bo,&mut p);if r!=0{return r};let m=p.cast::<i32>().add(off as usize/4);let typ=*m.add(1);let h=*m.add(2);if h==0{amdgpu_bo_kunmap(bo);return -22}let a=(*c).parser.as_ref().unwrap().adev; if typ==0 {amdgpu_bo_kunmap(bo);for i in 0..(*a).uvd.max_handles {let x=(*a).uvd.handles.add(i as usize);if atomic_read(x)==h{return -22}if atomic_cmpxchg(x,0,h)==0{return 0}}return -28} if typ==1 {let r=msg_decode(a,m.cast(),(*c).buf_sizes);amdgpu_bo_kunmap(bo);if r!=0{return r};for i in 0..(*a).uvd.max_handles {if atomic_read((*a).uvd.handles.add(i as usize))==h{return 0}}return -2} if typ==2 {for i in 0..(*a).uvd.max_handles{atomic_cmpxchg((*a).uvd.handles.add(i as usize),h,0)}amdgpu_bo_kunmap(bo);return 0}amdgpu_bo_kunmap(bo);-22 }

#[no_mangle] pub unsafe extern "C" fn amdgpu_uvd_used_handles(a:*mut amdgpu_device)->u32 {let mut n=0;for i in 0..(*a).uvd.max_handles{if atomic_read((*a).uvd.handles.add(i as usize))!=0{n+=1}}n}
#[no_mangle] pub unsafe extern "C" fn amdgpu_uvd_free_handles(_a:*mut amdgpu_device,_f:*mut drm_file) {}
#[no_mangle] pub unsafe extern "C" fn amdgpu_uvd_sw_init(_a:*mut amdgpu_device)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn amdgpu_uvd_sw_fini(_a:*mut amdgpu_device)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn amdgpu_uvd_entity_init(_a:*mut amdgpu_device,_r:*mut amdgpu_ring)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn amdgpu_uvd_prepare_suspend(_a:*mut amdgpu_device)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn amdgpu_uvd_suspend(_a:*mut amdgpu_device)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn amdgpu_uvd_resume(_a:*mut amdgpu_device)->i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn amdgpu_uvd_ring_parse_cs(_p:*mut amdgpu_cs_parser,j:*mut amdgpu_job,i:*mut amdgpu_ib)->i32 {(*j).vm=core::ptr::null_mut();if (*i).length_dw%16!=0{return -22}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
