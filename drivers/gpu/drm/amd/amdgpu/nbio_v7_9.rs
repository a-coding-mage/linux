/* Translated from nbio_v7_9.c. */

const NPS_MODE_MASK: u32 = 0x000000ff;
const MMIO_REG_HOLE_OFFSET: u32 = 0x1a000;

unsafe fn nbio_v7_9_remap_hdp_registers(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_REMAP_HDP_MEM_FLUSH_CNTL,
        (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_MEM_FLUSH_CNTL);
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_REMAP_HDP_REG_FLUSH_CNTL,
        (*adev).rmmio_remap.reg_offset + KFD_MMIO_REMAP_HDP_REG_FLUSH_CNTL);
}

unsafe fn nbio_v7_9_get_rev_id(adev: *mut amdgpu_device) -> u32 {
    if amdgpu_sriov_vf(adev) {
        IP_VERSION_SUBREV!(amdgpu_ip_version_full(adev, NBIO_HWIP, 0))
    } else {
        let mut rev_id = RREG32_SOC15!(NBIO, 0, regRCC_STRAP0_RCC_DEV0_EPF0_STRAP0);
        rev_id = REG_GET_FIELD!(rev_id, RCC_STRAP0_RCC_DEV0_EPF0_STRAP0, STRAP_ATI_REV_ID_DEV0_F0);
        rev_id
    }
}

unsafe fn nbio_v7_9_mc_access_enable(adev: *mut amdgpu_device, enable: bool) {
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_FB_EN, if enable {
        BIF_BX0_BIF_FB_EN__FB_READ_EN_MASK | BIF_BX0_BIF_FB_EN__FB_WRITE_EN_MASK
    } else { 0 });
}

unsafe fn nbio_v7_9_get_memsize(adev: *mut amdgpu_device) -> u32 {
    RREG32_SOC15!(NBIO, 0, regRCC_DEV0_EPF0_RCC_CONFIG_MEMSIZE)
}

unsafe fn nbio_v7_9_sdma_doorbell_range(adev: *mut amdgpu_device, instance: i32, use_doorbell: bool, doorbell_index: i32, doorbell_size: i32) {
    let mut doorbell_range = 0u32;
    let mut doorbell_ctrl = 0u32;
    let dev_inst = GET_INST!(SDMA0, instance);
    let aid_id = (*adev).sdma.instance[instance as usize].aid_id;
    if !use_doorbell { return; }
    doorbell_range = REG_SET_FIELD!(doorbell_range, DOORBELL0_CTRL_ENTRY_0, BIF_DOORBELL0_RANGE_OFFSET_ENTRY, doorbell_index);
    doorbell_range = REG_SET_FIELD!(doorbell_range, DOORBELL0_CTRL_ENTRY_0, BIF_DOORBELL0_RANGE_SIZE_ENTRY, doorbell_size);
    doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_ENABLE, 1);
    doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_SIZE, doorbell_size);
    match dev_inst % (*adev).sdma.num_inst_per_aid {
        0 => {
            WREG32_SOC15_OFFSET!(NBIO, 0, regDOORBELL0_CTRL_ENTRY_1, 4 * aid_id, doorbell_range);
            doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWID, 0xe);
            doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_OFFSET, 0xe);
            doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWADDR_31_28_VALUE, 1);
            WREG32_SOC15_EXT!(NBIO, aid_id, regS2A_DOORBELL_ENTRY_1_CTRL, aid_id, doorbell_ctrl);
        },
        1 => {
            WREG32_SOC15_OFFSET!(NBIO, 0, regDOORBELL0_CTRL_ENTRY_2, 4 * aid_id, doorbell_range);
            doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWID, 8);
            doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_OFFSET, 8);
            doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWADDR_31_28_VALUE, 2);
            WREG32_SOC15_EXT!(NBIO, aid_id, regS2A_DOORBELL_ENTRY_2_CTRL, aid_id, doorbell_ctrl);
        },
        2 => {
            WREG32_SOC15_OFFSET!(NBIO, 0, regDOORBELL0_CTRL_ENTRY_3, 4 * aid_id, doorbell_range);
            doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWID, 9);
            doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_OFFSET, 9);
            doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWADDR_31_28_VALUE, 8);
            WREG32_SOC15_EXT!(NBIO, aid_id, regS2A_DOORBELL_ENTRY_5_CTRL, aid_id, doorbell_ctrl);
        },
        3 => {
            WREG32_SOC15_OFFSET!(NBIO, 0, regDOORBELL0_CTRL_ENTRY_4, 4 * aid_id, doorbell_range);
            doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWID, 0xa);
            doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_OFFSET, 0xa);
            doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWADDR_31_28_VALUE, 9);
            WREG32_SOC15_EXT!(NBIO, aid_id, regS2A_DOORBELL_ENTRY_6_CTRL, aid_id, doorbell_ctrl);
        }, _ => {}
    }
}

unsafe fn nbio_v7_9_vcn_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool, doorbell_index: i32, instance: i32) {
    let mut doorbell_range = 0u32;
    let mut doorbell_ctrl = 0u32;
    let aid_id = instance as u32;
    let range_size = if amdgpu_ip_version(adev, GC_HWIP, 0) == IP_VERSION!(9, 5, 0) { 0xb } else { 9 };
    if use_doorbell {
        doorbell_range = REG_SET_FIELD!(doorbell_range, DOORBELL0_CTRL_ENTRY_0, BIF_DOORBELL0_RANGE_OFFSET_ENTRY, doorbell_index);
        doorbell_range = REG_SET_FIELD!(doorbell_range, DOORBELL0_CTRL_ENTRY_0, BIF_DOORBELL0_RANGE_SIZE_ENTRY, range_size);
        if aid_id != 0 { doorbell_range = REG_SET_FIELD!(doorbell_range, DOORBELL0_CTRL_ENTRY_0, DOORBELL0_FENCE_ENABLE_ENTRY, 4); }
        doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_ENABLE, 1);
        doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWID, 4);
        doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_OFFSET, 4);
        doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_SIZE, range_size);
        doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWADDR_31_28_VALUE, 4);
        WREG32_SOC15_OFFSET!(NBIO, 0, regDOORBELL0_CTRL_ENTRY_17, aid_id, doorbell_range);
        WREG32_SOC15_EXT!(NBIO, aid_id, regS2A_DOORBELL_ENTRY_4_CTRL, aid_id, doorbell_ctrl);
    } else {
        doorbell_range = REG_SET_FIELD!(doorbell_range, DOORBELL0_CTRL_ENTRY_0, BIF_DOORBELL0_RANGE_SIZE_ENTRY, 0);
        doorbell_ctrl = REG_SET_FIELD!(doorbell_ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_SIZE, 0);
        WREG32_SOC15_OFFSET!(NBIO, 0, regDOORBELL0_CTRL_ENTRY_17, aid_id, doorbell_range);
        WREG32_SOC15_EXT!(NBIO, aid_id, regS2A_DOORBELL_ENTRY_4_CTRL, aid_id, doorbell_ctrl);
    }
}

unsafe fn nbio_v7_9_enable_doorbell_aperture(adev: *mut amdgpu_device, enable: bool) {
    WREG32_SOC15!(NBIO, 0, regBIFC_DOORBELL_ACCESS_EN_PF, 0xfffff);
    WREG32_FIELD15_PREREG!(NBIO, 0, RCC_DEV0_EPF0_RCC_DOORBELL_APER_EN, BIF_DOORBELL_APER_EN, if enable { 1 } else { 0 });
}

unsafe fn nbio_v7_9_enable_doorbell_selfring_aperture(adev: *mut amdgpu_device, enable: bool) {
    let mut tmp = 0u32;
    if enable {
        tmp = REG_SET_FIELD!(tmp, BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_EN, 1) |
            REG_SET_FIELD!(tmp, BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_MODE, 1) |
            REG_SET_FIELD!(tmp, BIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, DOORBELL_SELFRING_GPA_APER_SIZE, 0);
        WREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_BASE_LOW, lower_32_bits!((*adev).doorbell.base));
        WREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_BASE_HIGH, upper_32_bits!((*adev).doorbell.base));
    }
    WREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_DOORBELL_SELFRING_GPA_APER_CNTL, tmp);
}

unsafe fn nbio_v7_9_ih_doorbell_range(adev: *mut amdgpu_device, use_doorbell: bool, doorbell_index: i32) {
    let mut range = 0u32; let mut ctrl = 0u32;
    if use_doorbell {
        range = REG_SET_FIELD!(range, DOORBELL0_CTRL_ENTRY_0, BIF_DOORBELL0_RANGE_OFFSET_ENTRY, doorbell_index);
        range = REG_SET_FIELD!(range, DOORBELL0_CTRL_ENTRY_0, BIF_DOORBELL0_RANGE_SIZE_ENTRY, 8);
        ctrl = REG_SET_FIELD!(ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_ENABLE, 1);
        ctrl = REG_SET_FIELD!(ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWID, 0);
        ctrl = REG_SET_FIELD!(ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_OFFSET, 0);
        ctrl = REG_SET_FIELD!(ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_SIZE, 8);
        ctrl = REG_SET_FIELD!(ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_AWADDR_31_28_VALUE, 0);
    } else {
        range = REG_SET_FIELD!(range, DOORBELL0_CTRL_ENTRY_0, BIF_DOORBELL0_RANGE_SIZE_ENTRY, 0);
        ctrl = REG_SET_FIELD!(ctrl, S2A_DOORBELL_ENTRY_1_CTRL, S2A_DOORBELL_PORT1_RANGE_SIZE, 0);
    }
    WREG32_SOC15!(NBIO, 0, regDOORBELL0_CTRL_ENTRY_0, range);
    WREG32_SOC15!(NBIO, 0, regS2A_DOORBELL_ENTRY_3_CTRL, ctrl);
}

unsafe fn nbio_v7_9_update_medium_grain_clock_gating(_: *mut amdgpu_device, _: bool) {}
unsafe fn nbio_v7_9_update_medium_grain_light_sleep(_: *mut amdgpu_device, _: bool) {}
unsafe fn nbio_v7_9_get_clockgating_state(_: *mut amdgpu_device, _: *mut u64) {}

unsafe fn nbio_v7_9_ih_control(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_INTERRUPT_CNTL2, (*adev).dummy_page_addr >> 8);
    let mut c = RREG32_SOC15!(NBIO, 0, regBIF_BX0_INTERRUPT_CNTL);
    c = REG_SET_FIELD!(c, BIF_BX0_INTERRUPT_CNTL, IH_DUMMY_RD_OVERRIDE, 0);
    c = REG_SET_FIELD!(c, BIF_BX0_INTERRUPT_CNTL, IH_REQ_NONSNOOP_EN, 0);
    WREG32_SOC15!(NBIO, 0, regBIF_BX0_INTERRUPT_CNTL, c);
}

unsafe fn nbio_v7_9_get_hdp_flush_req_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_PF0_GPU_HDP_FLUSH_REQ) }
unsafe fn nbio_v7_9_get_hdp_flush_done_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_PF0_GPU_HDP_FLUSH_DONE) }
unsafe fn nbio_v7_9_get_pcie_index_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX0_PCIE_INDEX2) }
unsafe fn nbio_v7_9_get_pcie_data_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX0_PCIE_DATA2) }
unsafe fn nbio_v7_9_get_pcie_index_hi_offset(_: *mut amdgpu_device) -> u32 { SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX0_PCIE_INDEX2_HI) }

pub static mut nbio_v7_9_hdp_flush_reg: nbio_hdp_flush_reg = nbio_hdp_flush_reg {
    ref_and_mask_cp0: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP0_MASK, ref_and_mask_cp1: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP1_MASK,
    ref_and_mask_cp2: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP2_MASK, ref_and_mask_cp3: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP3_MASK,
    ref_and_mask_cp4: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP4_MASK, ref_and_mask_cp5: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP5_MASK,
    ref_and_mask_cp6: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP6_MASK, ref_and_mask_cp7: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP7_MASK,
    ref_and_mask_cp8: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP8_MASK, ref_and_mask_cp9: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__CP9_MASK,
    ref_and_mask_sdma0: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__SDMA0_MASK, ref_and_mask_sdma1: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__SDMA1_MASK,
    ref_and_mask_sdma2: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__RSVD_ENG0_MASK, ref_and_mask_sdma3: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__RSVD_ENG1_MASK,
    ref_and_mask_sdma4: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__RSVD_ENG2_MASK, ref_and_mask_sdma5: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__RSVD_ENG3_MASK,
    ref_and_mask_sdma6: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__RSVD_ENG4_MASK, ref_and_mask_sdma7: BIF_BX_PF0_GPU_HDP_FLUSH_DONE__RSVD_ENG5_MASK,
};

unsafe fn nbio_v7_9_enable_doorbell_interrupt(adev: *mut amdgpu_device, enable: bool) { WREG32_FIELD15_PREREG!(NBIO, 0, BIF_BX0_BIF_DOORBELL_INT_CNTL, DOORBELL_INTERRUPT_DISABLE, if enable { 0 } else { 1 }); }
unsafe fn nbio_v7_9_get_compute_partition_mode(adev: *mut amdgpu_device) -> i32 { let t = RREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_PARTITION_COMPUTE_STATUS); REG_GET_FIELD!(t, BIF_BX_PF0_PARTITION_COMPUTE_STATUS, PARTITION_MODE) as i32 }
unsafe fn nbio_v7_9_is_nps_switch_requested(adev: *mut amdgpu_device) -> bool { let t = RREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_PARTITION_MEM_STATUS); REG_GET_FIELD!(t, BIF_BX_PF0_PARTITION_MEM_STATUS, CHANGE_STATUE) == 8 }
unsafe fn nbio_v7_9_get_memory_partition_mode(adev: *mut amdgpu_device, supp_modes: *mut u32) -> u32 { let t = RREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_PARTITION_MEM_STATUS); let t = REG_GET_FIELD!(t, BIF_BX_PF0_PARTITION_MEM_STATUS, NPS_MODE); if !supp_modes.is_null() { *supp_modes = RREG32_SOC15!(NBIO, 0, regBIF_BX_PF0_PARTITION_MEM_CAP); } ffs!(t) }

unsafe fn nbio_v7_9_init_registers(adev: *mut amdgpu_device) {
    WREG32_SOC15!(NBIO, 0, regXCC_DOORBELL_FENCE, 0xff & !(*adev).gfx.xcc_mask);
    WREG32_SOC15!(NBIO, 0, regBIFC_GFX_INT_MONITOR_MASK, 0x7ff);
    let inst_mask = (*adev).aid_mask & !1u32;
    for_each_inst!(i, inst_mask, { WREG32_SOC15_EXT!(NBIO, i, regXCC_DOORBELL_FENCE, i, XCC_DOORBELL_FENCE__SHUB_SLV_MODE_MASK); });
    if !amdgpu_sriov_vf(adev) { for_each_inst!(i, (*adev).aid_mask, { let mut c = RREG32_SOC15!(NBIO, i, regBIF_BX0_BACO_CNTL); if c & (BIF_BX0_BACO_CNTL__BACO_DUMMY_EN_MASK | BIF_BX0_BACO_CNTL__BACO_EN_MASK) != 0 { c &= !(BIF_BX0_BACO_CNTL__BACO_DUMMY_EN_MASK | BIF_BX0_BACO_CNTL__BACO_EN_MASK); dev_dbg!((*adev).dev, "Unsetting baco dummy mode %x", c); WREG32_SOC15!(NBIO, i, regBIF_BX0_BACO_CNTL, c); } }); }
}

unsafe fn nbio_v7_9_set_reg_remap(adev: *mut amdgpu_device) {
    if !amdgpu_sriov_vf(adev) && PAGE_SIZE <= 4096 { (*adev).rmmio_remap.reg_offset = MMIO_REG_HOLE_OFFSET; (*adev).rmmio_remap.bus_addr = (*adev).rmmio_base + MMIO_REG_HOLE_OFFSET as u64; }
    else { (*adev).rmmio_remap.reg_offset = SOC15_REG_OFFSET!(NBIO, 0, regBIF_BX_DEV0_EPF0_VF0_HDP_MEM_COHERENCY_FLUSH_CNTL) << 2; (*adev).rmmio_remap.bus_addr = 0; }
}

unsafe fn nbio_v7_9_query_ras_error_count(_: *mut amdgpu_device, _: *mut core::ffi::c_void) {}
unsafe fn nbio_v7_9_set_ras_controller_irq_state(_: *mut amdgpu_device, _: *mut amdgpu_irq_src, _: u32, _: amdgpu_interrupt_state) -> i32 { 0 }
unsafe fn nbio_v7_9_process_ras_controller_irq(_: *mut amdgpu_device, _: *mut amdgpu_irq_src, _: *mut amdgpu_iv_entry) -> i32 { 0 }
unsafe fn nbio_v7_9_set_ras_err_event_athub_irq_state(_: *mut amdgpu_device, _: *mut amdgpu_irq_src, _: u32, _: amdgpu_interrupt_state) -> i32 { 0 }
unsafe fn nbio_v7_9_process_err_event_athub_irq(_: *mut amdgpu_device, _: *mut amdgpu_irq_src, _: *mut amdgpu_iv_entry) -> i32 { 0 }

// Function tables and RAS handlers retain the C aggregate layout and external callbacks.
pub static mut nbio_v7_9_funcs: amdgpu_nbio_funcs = amdgpu_nbio_funcs {
    get_hdp_flush_req_offset: Some(nbio_v7_9_get_hdp_flush_req_offset), get_hdp_flush_done_offset: Some(nbio_v7_9_get_hdp_flush_done_offset),
    get_pcie_index_offset: Some(nbio_v7_9_get_pcie_index_offset), get_pcie_data_offset: Some(nbio_v7_9_get_pcie_data_offset), get_pcie_index_hi_offset: Some(nbio_v7_9_get_pcie_index_hi_offset),
    get_rev_id: Some(nbio_v7_9_get_rev_id), mc_access_enable: Some(nbio_v7_9_mc_access_enable), get_memsize: Some(nbio_v7_9_get_memsize),
    sdma_doorbell_range: Some(nbio_v7_9_sdma_doorbell_range), vcn_doorbell_range: Some(nbio_v7_9_vcn_doorbell_range), enable_doorbell_aperture: Some(nbio_v7_9_enable_doorbell_aperture),
    enable_doorbell_selfring_aperture: Some(nbio_v7_9_enable_doorbell_selfring_aperture), ih_doorbell_range: Some(nbio_v7_9_ih_doorbell_range), enable_doorbell_interrupt: Some(nbio_v7_9_enable_doorbell_interrupt),
    update_medium_grain_clock_gating: Some(nbio_v7_9_update_medium_grain_clock_gating), update_medium_grain_light_sleep: Some(nbio_v7_9_update_medium_grain_light_sleep), get_clockgating_state: Some(nbio_v7_9_get_clockgating_state),
    ih_control: Some(nbio_v7_9_ih_control), remap_hdp_registers: Some(nbio_v7_9_remap_hdp_registers), get_compute_partition_mode: Some(nbio_v7_9_get_compute_partition_mode), get_memory_partition_mode: Some(nbio_v7_9_get_memory_partition_mode),
    is_nps_switch_requested: Some(nbio_v7_9_is_nps_switch_requested), init_registers: Some(nbio_v7_9_init_registers), set_reg_remap: Some(nbio_v7_9_set_reg_remap),
};

unsafe fn nbio_v7_9_handle_ras_controller_intr_no_bifring(adev: *mut amdgpu_device) {
    let mut c = RREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_DOORBELL_INT_CNTL);
    if REG_GET_FIELD!(c, BIF_BX0_BIF_DOORBELL_INT_CNTL, RAS_CNTLR_INTERRUPT_STATUS) != 0 {
        c = REG_SET_FIELD!(c, BIF_BX0_BIF_DOORBELL_INT_CNTL, RAS_CNTLR_INTERRUPT_CLEAR, 1);
        WREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_DOORBELL_INT_CNTL, c);
        dev_info!((*adev).dev, "RAS controller interrupt triggered by NBIF error");
    }
}

unsafe fn nbio_v7_9_handle_ras_err_event_athub_intr_no_bifring(adev: *mut amdgpu_device) {
    let mut c = RREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_DOORBELL_INT_CNTL);
    if REG_GET_FIELD!(c, BIF_BX0_BIF_DOORBELL_INT_CNTL, RAS_ATHUB_ERR_EVENT_INTERRUPT_STATUS) != 0 {
        c = REG_SET_FIELD!(c, BIF_BX0_BIF_DOORBELL_INT_CNTL, RAS_ATHUB_ERR_EVENT_INTERRUPT_CLEAR, 1);
        WREG32_SOC15!(NBIO, 0, regBIF_BX0_BIF_DOORBELL_INT_CNTL, c);
        amdgpu_ras_global_ras_isr(adev);
    }
}

unsafe fn nbio_v7_9_init_ras_controller_interrupt(adev: *mut amdgpu_device) -> i32 {
    (*adev).nbio.ras_controller_irq.funcs = &nbio_v7_9_ras_controller_irq_funcs;
    (*adev).nbio.ras_controller_irq.num_types = 1;
    amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_BIF, NBIF_7_4__SRCID__RAS_CONTROLLER_INTERRUPT, &mut (*adev).nbio.ras_controller_irq)
}

unsafe fn nbio_v7_9_init_ras_err_event_athub_interrupt(adev: *mut amdgpu_device) -> i32 {
    (*adev).nbio.ras_err_event_athub_irq.funcs = &nbio_v7_9_ras_err_event_athub_irq_funcs;
    (*adev).nbio.ras_err_event_athub_irq.num_types = 1;
    amdgpu_irq_add_id(adev, SOC15_IH_CLIENTID_BIF, NBIF_7_4__SRCID__ERREVENT_ATHUB_INTERRUPT, &mut (*adev).nbio.ras_err_event_athub_irq)
}

pub static mut nbio_v7_9_ras_controller_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs {
    set: Some(nbio_v7_9_set_ras_controller_irq_state), process: Some(nbio_v7_9_process_ras_controller_irq),
};
pub static mut nbio_v7_9_ras_err_event_athub_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs {
    set: Some(nbio_v7_9_set_ras_err_event_athub_irq_state), process: Some(nbio_v7_9_process_err_event_athub_irq),
};
pub static mut nbio_v7_9_ras_hw_ops: amdgpu_ras_block_hw_ops = amdgpu_ras_block_hw_ops {
    query_ras_error_count: Some(nbio_v7_9_query_ras_error_count),
};

pub static mut nbio_v7_9_ras: amdgpu_nbio_ras = amdgpu_nbio_ras {
    ras_block: amdgpu_ras_block {
        ras_comm: amdgpu_ras_common_if {
            name: b"pcie_bif\0".as_ptr() as *const _,
            block: AMDGPU_RAS_BLOCK__PCIE_BIF,
            type_: AMDGPU_RAS_ERROR__MULTI_UNCORRECTABLE,
        },
        hw_ops: &nbio_v7_9_ras_hw_ops,
        ras_late_init: Some(amdgpu_nbio_ras_late_init),
    },
    handle_ras_controller_intr_no_bifring: Some(nbio_v7_9_handle_ras_controller_intr_no_bifring),
    handle_ras_err_event_athub_intr_no_bifring: Some(nbio_v7_9_handle_ras_err_event_athub_intr_no_bifring),
    init_ras_controller_interrupt: Some(nbio_v7_9_init_ras_controller_interrupt),
    init_ras_err_event_athub_interrupt: Some(nbio_v7_9_init_ras_err_event_athub_interrupt),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
