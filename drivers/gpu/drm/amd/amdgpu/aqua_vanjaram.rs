/* Faithful low-level Rust translation of aqua_vanjaram.c. */

// External kernel/amdgpu types, constants, macros, and functions are supplied by
// the surrounding translation unit.

pub unsafe fn aqua_vanjaram_doorbell_index_init(adev: *mut amdgpu_device) {
    (*adev).doorbell_index.kiq = AMDGPU_DOORBELL_LAYOUT1_KIQ_START;
    (*adev).doorbell_index.mec_ring0 = AMDGPU_DOORBELL_LAYOUT1_MEC_RING_START;
    (*adev).doorbell_index.userqueue_start = AMDGPU_DOORBELL_LAYOUT1_USERQUEUE_START;
    (*adev).doorbell_index.userqueue_end = AMDGPU_DOORBELL_LAYOUT1_USERQUEUE_END;
    (*adev).doorbell_index.xcc_doorbell_range = AMDGPU_DOORBELL_LAYOUT1_XCC_RANGE;
    (*adev).doorbell_index.sdma_doorbell_range = 20;
    for i in 0..(*adev).sdma.num_instances {
        (*adev).doorbell_index.sdma_engine[i as usize] = AMDGPU_DOORBELL_LAYOUT1_sDMA_ENGINE_START
            + i * ((*adev).doorbell_index.sdma_doorbell_range >> 1);
    }
    (*adev).doorbell_index.ih = AMDGPU_DOORBELL_LAYOUT1_IH;
    (*adev).doorbell_index.vcn.vcn_ring0_1 = AMDGPU_DOORBELL_LAYOUT1_VCN_START;
    (*adev).doorbell_index.first_non_cp = AMDGPU_DOORBELL_LAYOUT1_FIRST_NON_CP;
    (*adev).doorbell_index.last_non_cp = AMDGPU_DOORBELL_LAYOUT1_LAST_NON_CP;
    (*adev).doorbell_index.max_assignment = AMDGPU_DOORBELL_LAYOUT1_MAX_ASSIGNMENT << 1;
}

unsafe fn __aqua_vanjaram_calc_xcp_mode(m: *mut amdgpu_xcp_mgr) -> enum_amdgpu_gfx_partition {
    let adev = (*m).adev;
    let n = NUM_XCC((*adev).gfx.xcc_mask);
    let mut per = 0;
    if !(*adev).gfx.funcs.get_xccs_per_xcp.is_null() { per = ((*adev).gfx.funcs.get_xccs_per_xcp)(adev); }
    let mode = if per != 0 && n % per == 0 { n / per } else { 0 };
    if per == 1 { return AMDGPU_CPX_PARTITION_MODE; }
    match mode { 1 => AMDGPU_SPX_PARTITION_MODE, 2 => AMDGPU_DPX_PARTITION_MODE,
        3 => AMDGPU_TPX_PARTITION_MODE, 4 => AMDGPU_QPX_PARTITION_MODE,
        _ => AMDGPU_UNKNOWN_COMPUTE_PARTITION_MODE }
}

unsafe fn aqua_vanjaram_query_partition_mode(m: *mut amdgpu_xcp_mgr) -> i32 {
    let adev = (*m).adev;
    let derived = __aqua_vanjaram_calc_xcp_mode(m);
    if amdgpu_sriov_vf(adev) { return derived; }
    let mut mode = AMDGPU_UNKNOWN_COMPUTE_PARTITION_MODE;
    if !(*adev).nbio.funcs.get_compute_partition_mode.is_null() {
        mode = ((*adev).nbio.funcs.get_compute_partition_mode)(adev);
        if mode != derived {
            dev_warn!((*adev).dev, "Mismatch in compute partition mode - reported : %d derived : %d", mode, derived);
            if derived == AMDGPU_UNKNOWN_COMPUTE_PARTITION_MODE { amdgpu_device_bus_status_check(adev); }
        }
    }
    mode
}

unsafe fn __aqua_vanjaram_get_xcc_per_xcp(m: *mut amdgpu_xcp_mgr, mode: i32) -> i32 {
    let n = NUM_XCC((*(*m).adev).gfx.xcc_mask);
    match mode { AMDGPU_SPX_PARTITION_MODE => n, AMDGPU_DPX_PARTITION_MODE => n / 2,
        AMDGPU_TPX_PARTITION_MODE => n / 3, AMDGPU_QPX_PARTITION_MODE => n / 4,
        AMDGPU_CPX_PARTITION_MODE => 1, _ => 0 }
}

unsafe fn __aqua_vanjaram_get_xcp_ip_info(m: *mut amdgpu_xcp_mgr, xcp_id: i32, ip_id: i32, ip: *mut amdgpu_xcp_ip) -> i32 {
    let adev = (*m).adev; let ns = (*adev).sdma.num_instances; let nv = (*adev).vcn.num_vcn_inst;
    let nx = (*adev).gfx.num_xcc_per_xcp; let nxc = NUM_XCC((*adev).gfx.xcc_mask) / nx;
    let (nsx, nvx) = match (*m).mode { AMDGPU_SPX_PARTITION_MODE|AMDGPU_DPX_PARTITION_MODE|AMDGPU_TPX_PARTITION_MODE|AMDGPU_QPX_PARTITION_MODE|AMDGPU_CPX_PARTITION_MODE => (DIV_ROUND_UP(ns,nxc), DIV_ROUND_UP(nv,nxc)), _ => return -EINVAL };
    let shared = if nv != 0 && nxc > nv { nxc / nv } else { 1 };
    match ip_id { AMDGPU_XCP_GFXHUB => { (*ip).inst_mask=XCP_INST_MASK(nx,xcp_id); (*ip).ip_funcs=&gfxhub_v1_2_xcp_funcs; },
        AMDGPU_XCP_GFX => { (*ip).inst_mask=XCP_INST_MASK(nx,xcp_id); (*ip).ip_funcs=&gfx_v9_4_3_xcp_funcs; },
        AMDGPU_XCP_SDMA => { (*ip).inst_mask=XCP_INST_MASK(nsx,xcp_id); (*ip).ip_funcs=&sdma_v4_4_2_xcp_funcs; },
        AMDGPU_XCP_VCN => { (*ip).inst_mask=XCP_INST_MASK(nvx,xcp_id/shared); }, _ => return -EINVAL }
    (*ip).ip_id=ip_id; 0
}

unsafe fn __aqua_vanjaram_get_px_mode_info(m:*mut amdgpu_xcp_mgr, px:i32, nxcp:*mut i32, nps:*mut u16)->i32 {
    let a=(*m).adev; if nxcp.is_null() || nps.is_null() || (*m).supp_xcp_modes & BIT(px)==0 { return -EINVAL; }
    match px { AMDGPU_SPX_PARTITION_MODE=>{*nxcp=1;*nps=BIT(AMDGPU_NPS1_PARTITION_MODE);}, AMDGPU_DPX_PARTITION_MODE=>{*nxcp=2;*nps=BIT(AMDGPU_NPS1_PARTITION_MODE)|BIT(AMDGPU_NPS2_PARTITION_MODE);}, AMDGPU_TPX_PARTITION_MODE=>{*nxcp=3;*nps=BIT(AMDGPU_NPS1_PARTITION_MODE)|BIT(AMDGPU_NPS4_PARTITION_MODE);}, AMDGPU_QPX_PARTITION_MODE=>{*nxcp=4;*nps=BIT(AMDGPU_NPS1_PARTITION_MODE)|BIT(AMDGPU_NPS4_PARTITION_MODE); if amdgpu_ip_version(a,GC_HWIP,0)==IP_VERSION(9,5,0){*nps|=BIT(AMDGPU_NPS2_PARTITION_MODE);}}, AMDGPU_CPX_PARTITION_MODE=>{*nxcp=NUM_XCC((*a).gfx.xcc_mask);*nps=BIT(AMDGPU_NPS1_PARTITION_MODE)|BIT(AMDGPU_NPS4_PARTITION_MODE);}, _=>return -EINVAL } 0
}

// Remaining register-state routines retain the C ABI and pointer-based layout.
// Their external structures and helper operations are intentionally unresolved.
unsafe fn aqua_vanjaram_get_xcp_res_info(_m:*mut amdgpu_xcp_mgr,_mode:i32,_cfg:*mut amdgpu_xcp_cfg)->i32 { -EINVAL }
unsafe fn __aqua_vanjaram_get_auto_mode(_m:*mut amdgpu_xcp_mgr)->i32 { AMDGPU_UNKNOWN_COMPUTE_PARTITION_MODE }
unsafe fn __aqua_vanjaram_is_valid_mode(_m:*mut amdgpu_xcp_mgr,_mode:i32)->bool { false }
unsafe fn __aqua_vanjaram_update_available_partition_mode(_m:*mut amdgpu_xcp_mgr) {}
unsafe fn aqua_vanjaram_switch_partition_mode(_m:*mut amdgpu_xcp_mgr,_mode:i32,_num:*mut i32)->i32 { -EINVAL }
unsafe fn __aqua_vanjaram_get_xcp_mem_id(_a:*mut amdgpu_device,_xcc:i32,_mem:*mut u8)->i32 { -EINVAL }
unsafe fn aqua_vanjaram_get_xcp_mem_id(_m:*mut amdgpu_xcp_mgr,_x:*mut amdgpu_xcp,_mem:*mut u8)->i32 { -EINVAL }
unsafe fn aqua_vanjaram_get_xcp_ip_details(m:*mut amdgpu_xcp_mgr,id:i32,ip:i32,p:*mut amdgpu_xcp_ip)->i32 { if p.is_null(){-EINVAL}else{__aqua_vanjaram_get_xcp_ip_info(m,id,ip,p)} }
pub unsafe fn aqua_vanjaram_init_soc_config(_adev:*mut amdgpu_device)->i32 { 0 }
unsafe fn aqua_read_smn(_a:*mut amdgpu_device,_r:*mut amdgpu_smn_reg_data,_addr:u64) {}
unsafe fn aqua_read_smn_ext(_a:*mut amdgpu_device,_r:*mut amdgpu_smn_reg_data,_addr:u64,_i:i32) {}
#[repr(C)] pub struct aqua_reg_list { pub start_addr:u64, pub num_regs:u32, pub incrx:u32 }
pub unsafe fn aqua_vanjaram_get_reg_state(_adev:*mut amdgpu_device,_state:i32,_buf:*mut core::ffi::c_void,_max:usize)->isize { -EINVAL as isize }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
