/* Copyright 2022 Advanced Micro Devices, Inc. */
/* Permission is hereby granted ... (the Software). */

// C dependencies: linux/firmware.h, drm/drm_drv.h, and the AMDGPU headers.

const VPE_THREAD1_UCODE_OFFSET: u32 = 0x8000;
const regVPEC_COLLABORATE_CNTL: u32 = 0x0013;
const regVPEC_COLLABORATE_CNTL_BASE_IDX: u32 = 0;
const VPEC_COLLABORATE_CNTL__COLLABORATE_MODE_EN__SHIFT: u32 = 0;
const VPEC_COLLABORATE_CNTL__COLLABORATE_MODE_EN_MASK: u32 = 0x00000001;
const regVPEC_COLLABORATE_CFG: u32 = 0x0014;
const regVPEC_COLLABORATE_CFG_BASE_IDX: u32 = 0;
const VPEC_COLLABORATE_CFG__MASTER_ID__SHIFT: u32 = 0;
const VPEC_COLLABORATE_CFG__MASTER_EN__SHIFT: u32 = 0x3;
const VPEC_COLLABORATE_CFG__SLAVE0_ID__SHIFT: u32 = 0x4;
const VPEC_COLLABORATE_CFG__SLAVE0_EN__SHIFT: u32 = 0x7;
const VPEC_COLLABORATE_CFG__MASTER_ID_MASK: u32 = 0x00000007;
const VPEC_COLLABORATE_CFG__MASTER_EN_MASK: u32 = 0x00000008;
const VPEC_COLLABORATE_CFG__SLAVE0_ID_MASK: u32 = 0x00000070;
const VPEC_COLLABORATE_CFG__SLAVE0_EN_MASK: u32 = 0x00000080;
const regVPEC_CNTL_6_1_1: u32 = 0x0016;
const regVPEC_CNTL_6_1_1_BASE_IDX: u32 = 0;
const regVPEC_QUEUE_RESET_REQ_6_1_1: u32 = 0x002c;
const regVPEC_QUEUE_RESET_REQ_6_1_1_BASE_IDX: u32 = 0;
const regVPEC_PUB_DUMMY2_6_1_1: u32 = 0x004c;
const regVPEC_PUB_DUMMY2_6_1_1_BASE_IDX: u32 = 0;

unsafe fn vpe_v6_1_get_reg_offset(vpe: *mut amdgpu_vpe, inst: u32, offset: u32) -> u32 {
    (*(*vpe).ring.adev).reg_offset[VPE_HWIP][inst as usize][0] + offset
}

unsafe fn vpe_v6_1_halt(vpe: *mut amdgpu_vpe, halt: bool) {
    let adev = (*vpe).ring.adev;
    for i in 0..(*vpe).num_instances {
        let off = vpe_get_reg_offset(vpe, i, regVPEC_F32_CNTL);
        let mut f32_cntl = RREG32(off);
        f32_cntl = REG_SET_FIELD!(f32_cntl, VPEC_F32_CNTL, HALT, if halt { 1 } else { 0 });
        f32_cntl = REG_SET_FIELD!(f32_cntl, VPEC_F32_CNTL, TH1_RESET, if halt { 1 } else { 0 });
        WREG32(off, f32_cntl);
    }
}

unsafe fn vpe_v6_1_irq_init(vpe: *mut amdgpu_vpe) -> i32 {
    let adev = container_of!(vpe, amdgpu_device, vpe);
    let ret = amdgpu_irq_add_id(adev, SOC21_IH_CLIENTID_VPE, VPE_6_1_SRCID__VPE_TRAP, &mut (*adev).vpe.trap_irq);
    if ret != 0 { return ret; }
    0
}

unsafe fn vpe_v6_1_set_collaborate_mode(vpe: *mut amdgpu_vpe, enable: bool) {
    let _adev = (*vpe).ring.adev;
    if !(*vpe).collaborate_mode { return; }
    for i in 0..(*vpe).num_instances {
        let mut cntl = RREG32(vpe_get_reg_offset(vpe, i, regVPEC_COLLABORATE_CNTL));
        cntl = REG_SET_FIELD!(cntl, VPEC_COLLABORATE_CNTL, COLLABORATE_MODE_EN, if enable {1} else {0});
        WREG32(vpe_get_reg_offset(vpe, i, regVPEC_COLLABORATE_CNTL), cntl);
        let mut cfg = RREG32(vpe_get_reg_offset(vpe, i, regVPEC_COLLABORATE_CFG));
        cfg = REG_SET_FIELD!(cfg, VPEC_COLLABORATE_CFG, MASTER_ID, 0);
        cfg = REG_SET_FIELD!(cfg, VPEC_COLLABORATE_CFG, MASTER_EN, if enable {1} else {0});
        cfg = REG_SET_FIELD!(cfg, VPEC_COLLABORATE_CFG, SLAVE0_ID, 1);
        cfg = REG_SET_FIELD!(cfg, VPEC_COLLABORATE_CFG, SLAVE0_EN, if enable {1} else {0});
        WREG32(vpe_get_reg_offset(vpe, i, regVPEC_COLLABORATE_CFG), cfg);
    }
}

unsafe fn vpe_v6_1_load_microcode(vpe: *mut amdgpu_vpe) -> i32 {
    let adev = (*vpe).ring.adev;
    for j in 0..(*vpe).num_instances {
        let off = if amdgpu_ip_version(adev,VPE_HWIP,0)==IP_VERSION!(6,1,1) { regVPEC_CNTL_6_1_1 } else { regVPEC_CNTL };
        let mut ret = RREG32(vpe_get_reg_offset(vpe,j,off));
        ret = REG_SET_FIELD!(ret,VPEC_CNTL,UMSCH_INT_ENABLE,0);
        WREG32(vpe_get_reg_offset(vpe,j,off),ret);
    }
    vpe_v6_1_set_collaborate_mode(vpe,true);
    if amdgpu_vpe_configure_dpm(vpe) != 0 { dev_warn!((*adev).dev, "VPE failed to enable DPM\n"); }
    if (*adev).firmware.load_type == AMDGPU_FW_LOAD_PSP {
        let off = vpe_get_reg_offset(vpe,0,regVPEC_F32_CNTL);
        let mut cntl = RREG32(off);
        cntl = REG_SET_FIELD!(cntl,VPEC_F32_CNTL,HALT,0);
        cntl = REG_SET_FIELD!(cntl,VPEC_F32_CNTL,TH1_RESET,0);
        (*adev).vpe.cmdbuf_cpu_addr[0]=off; (*adev).vpe.cmdbuf_cpu_addr[1]=cntl;
        return amdgpu_vpe_psp_update_sram(adev);
    }
    let hdr = (*adev).vpe.fw.data as *const vpe_firmware_header_v1_0;
    let offsets = [(*hdr).header.ucode_array_offset_bytes.to_cpu(), (*hdr).ctl_ucode_offset.to_cpu()];
    let sizes = [(*hdr).ctx_ucode_size_bytes.to_cpu(), (*hdr).ctl_ucode_size_bytes.to_cpu()];
    vpe_v6_1_halt(vpe,true);
    for j in 0..(*vpe).num_instances { for i in 0..2 {
        WREG32(vpe_get_reg_offset(vpe,j,regVPEC_UCODE_ADDR), if i>0 {VPE_THREAD1_UCODE_OFFSET} else {0});
        let mut data = ((*adev).vpe.fw.data.add(offsets[i] as usize)) as *const __le32;
        let mut n = sizes[i] / core::mem::size_of::<__le32>() as u32;
        while n != 0 { n -= 1; if amdgpu_emu_mode && n % 500 == 0 { msleep(1); } WREG32(vpe_get_reg_offset(vpe,j,regVPEC_UCODE_DATA), le32_to_cpup(data)); data=data.add(1); }
    }}
    vpe_v6_1_halt(vpe,false); 0
}

unsafe fn vpe_v6_1_ring_start(vpe: *mut amdgpu_vpe) -> i32 {
    let ring=&mut (*vpe).ring; let adev=ring.adev;
    for i in 0..(*vpe).num_instances {
        let mut c=RREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_RB_CNTL));
        c=REG_SET_FIELD!(c,VPEC_QUEUE0_RB_CNTL,RB_SIZE,order_base_2(ring.ring_size/4)); c=REG_SET_FIELD!(c,VPEC_QUEUE0_RB_CNTL,RB_PRIV,1); c=REG_SET_FIELD!(c,VPEC_QUEUE0_RB_CNTL,RB_VMID,0); WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_RB_CNTL),c);
        for r in [regVPEC_QUEUE0_RB_RPTR,regVPEC_QUEUE0_RB_RPTR_HI,regVPEC_QUEUE0_RB_WPTR,regVPEC_QUEUE0_RB_WPTR_HI] { WREG32(vpe_get_reg_offset(vpe,i,r),0); }
        WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_RB_RPTR_ADDR_LO),lower_32_bits(ring.rptr_gpu_addr)&0xfffffffc); WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_RB_RPTR_ADDR_HI),upper_32_bits(ring.rptr_gpu_addr));
        c=REG_SET_FIELD!(c,VPEC_QUEUE0_RB_CNTL,RPTR_WRITEBACK_ENABLE,1); WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_RB_BASE),ring.gpu_addr>>8); WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_RB_BASE_HI),ring.gpu_addr>>40); ring.wptr=0;
        WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_MINOR_PTR_UPDATE),1); WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_RB_WPTR),lower_32_bits(ring.wptr)<<2); WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_RB_WPTR_HI),upper_32_bits(ring.wptr)<<2); WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_MINOR_PTR_UPDATE),0);
        let mut d=RREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_DOORBELL_OFFSET)); d=REG_SET_FIELD!(d,VPEC_QUEUE0_DOORBELL_OFFSET,OFFSET,ring.doorbell_index+i*4); WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_DOORBELL_OFFSET),d);
        d=RREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_DOORBELL)); d=REG_SET_FIELD!(d,VPEC_QUEUE0_DOORBELL,ENABLE,if ring.use_doorbell{1}else{0}); WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_DOORBELL),d); (*adev).nbio.funcs.vpe_doorbell_range(adev,i,ring.use_doorbell,ring.doorbell_index+i*4,4);
        c=REG_SET_FIELD!(c,VPEC_QUEUE0_RB_CNTL,RB_ENABLE,1); WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_RB_CNTL),c); let mut ib=RREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_IB_CNTL)); ib=REG_SET_FIELD!(ib,VPEC_QUEUE0_IB_CNTL,IB_ENABLE,1); WREG32(vpe_get_reg_offset(vpe,i,regVPEC_QUEUE0_IB_CNTL),ib);
    } amdgpu_ring_test_helper(ring)
}
unsafe fn vpe_v_6_1_ring_stop(vpe:*mut amdgpu_vpe)->i32 { (*vpe).ring.sched.ready=false; 0 }
unsafe fn vpe_v6_1_set_trap_irq_state(_adev:*mut amdgpu_device,_source:*mut amdgpu_irq_src,_type:u32,_state:enum_amdgpu_interrupt_state)->i32 { 0 }
unsafe fn vpe_v6_1_process_trap_irq(adev:*mut amdgpu_device,_source:*mut amdgpu_irq_src,entry:*mut amdgpu_iv_entry)->i32 { if (*entry).client_id==SOC21_IH_CLIENTID_VPE { amdgpu_fence_process(&mut (*adev).vpe.ring); } 0 }
unsafe fn vpe_v6_1_set_regs(vpe:*mut amdgpu_vpe)->i32 { (*vpe).regs.queue0_rb_rptr_lo=regVPEC_QUEUE0_RB_RPTR; (*vpe).regs.queue0_rb_rptr_hi=regVPEC_QUEUE0_RB_RPTR_HI; (*vpe).regs.queue0_rb_wptr_lo=regVPEC_QUEUE0_RB_WPTR; (*vpe).regs.queue0_rb_wptr_hi=regVPEC_QUEUE0_RB_WPTR_HI; (*vpe).regs.queue0_preempt=regVPEC_QUEUE0_PREEMPT; 0 }
static vpe_v6_1_funcs: vpe_funcs = vpe_funcs { get_reg_offset:vpe_v6_1_get_reg_offset,set_regs:vpe_v6_1_set_regs,irq_init:vpe_v6_1_irq_init,init_microcode:amdgpu_vpe_init_microcode,load_microcode:vpe_v6_1_load_microcode,ring_init:amdgpu_vpe_ring_init,ring_start:vpe_v6_1_ring_start,ring_stop:vpe_v_6_1_ring_stop,ring_fini:amdgpu_vpe_ring_fini };
static vpe_v6_1_trap_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { set:vpe_v6_1_set_trap_irq_state, process:vpe_v6_1_process_trap_irq };
unsafe fn vpe_v6_1_set_funcs(vpe: *mut amdgpu_vpe) { (*vpe).funcs=&vpe_v6_1_funcs; (*vpe).trap_irq.funcs=&vpe_v6_1_trap_irq_funcs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
