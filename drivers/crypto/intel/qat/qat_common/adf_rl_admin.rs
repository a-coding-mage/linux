// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2023 Intel Corporation */

// Linux DMA, PCI, and QAT declarations are supplied by the surrounding
// translation unit.

unsafe fn prep_admin_req_msg(
    sla: *const rl_sla,
    dma_addr: dma_addr_t,
    fw_params: *mut icp_qat_fw_init_admin_sla_config_params,
    req: *mut icp_qat_fw_init_admin_req,
    is_update: bool,
) {
    (*req).cmd_id = if is_update {
        ICP_QAT_FW_RL_UPDATE
    } else {
        ICP_QAT_FW_RL_ADD
    };
    (*req).init_cfg_ptr = dma_addr;
    (*req).init_cfg_sz = core::mem::size_of_val(&*fw_params);
    (*req).node_id = (*sla).node_id;
    (*req).node_type = (*sla).type_;
    (*req).rp_count = (*sla).ring_pairs_cnt;
    (*req).svc_type = (*sla).srv;
}

unsafe fn prep_admin_req_params(
    accel_dev: *mut adf_accel_dev,
    sla: *const rl_sla,
    fw_params: *mut icp_qat_fw_init_admin_sla_config_params,
) {
    (*fw_params).pcie_in_cir =
        adf_rl_calculate_pci_bw(accel_dev, (*sla).cir, (*sla).srv, false);
    (*fw_params).pcie_in_pir =
        adf_rl_calculate_pci_bw(accel_dev, (*sla).pir, (*sla).srv, false);
    (*fw_params).pcie_out_cir =
        adf_rl_calculate_pci_bw(accel_dev, (*sla).cir, (*sla).srv, true);
    (*fw_params).pcie_out_pir =
        adf_rl_calculate_pci_bw(accel_dev, (*sla).pir, (*sla).srv, true);

    (*fw_params).slice_util_cir =
        adf_rl_calculate_slice_tokens(accel_dev, (*sla).cir, (*sla).srv);
    (*fw_params).slice_util_pir =
        adf_rl_calculate_slice_tokens(accel_dev, (*sla).pir, (*sla).srv);

    (*fw_params).ae_util_cir =
        adf_rl_calculate_ae_cycles(accel_dev, (*sla).cir, (*sla).srv);
    (*fw_params).ae_util_pir =
        adf_rl_calculate_ae_cycles(accel_dev, (*sla).pir, (*sla).srv);

    core::ptr::copy_nonoverlapping(
        (*sla).ring_pairs_ids.as_ptr(),
        (*fw_params).rp_ids.as_mut_ptr(),
        core::mem::size_of_val(&(*sla).ring_pairs_ids),
    );
}

unsafe fn adf_rl_send_admin_init_msg(
    accel_dev: *mut adf_accel_dev,
    slices_int: *mut rl_slice_cnt,
) -> i32 {
    let mut slices_resp: icp_qat_fw_init_admin_slice_cnt = core::mem::zeroed();
    let ret = adf_send_admin_rl_init(accel_dev, &mut slices_resp);
    if ret != 0 {
        return ret;
    }

    (*slices_int).dcpr_cnt = slices_resp.dcpr_cnt;
    (*slices_int).pke_cnt = slices_resp.pke_cnt;
    // For symmetric crypto, slice tokens are relative to the UCS slice
    (*slices_int).cph_cnt = slices_resp.ucs_cnt;
    (*slices_int).cpr_cnt = slices_resp.cpr_cnt;

    0
}

unsafe fn adf_rl_send_admin_add_update_msg(
    accel_dev: *mut adf_accel_dev,
    sla: *mut rl_sla,
    is_update: bool,
) -> i32 {
    let mut fw_params: *mut icp_qat_fw_init_admin_sla_config_params;
    let mut req: icp_qat_fw_init_admin_req = core::mem::zeroed();
    let mut dma_addr: dma_addr_t = core::mem::zeroed();
    let ret: i32;

    fw_params = dma_alloc_coherent(
        &mut GET_DEV(accel_dev),
        core::mem::size_of::<icp_qat_fw_init_admin_sla_config_params>(),
        &mut dma_addr,
        GFP_KERNEL,
    );
    if fw_params.is_null() {
        return -ENOMEM;
    }

    prep_admin_req_params(accel_dev, sla, fw_params);
    prep_admin_req_msg(sla, dma_addr, fw_params, &mut req, is_update);
    ret = adf_send_admin_rl_add_update(accel_dev, &req);

    dma_free_coherent(
        &mut GET_DEV(accel_dev),
        core::mem::size_of::<icp_qat_fw_init_admin_sla_config_params>(),
        fw_params,
        dma_addr,
    );

    ret
}

unsafe fn adf_rl_send_admin_delete_msg(
    accel_dev: *mut adf_accel_dev,
    node_id: u16,
    node_type: u8,
) -> i32 {
    adf_send_admin_rl_delete(accel_dev, node_id, node_type)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
