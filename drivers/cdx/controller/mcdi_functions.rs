// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the surrounding kernel/MCDI translation unit.

pub unsafe fn cdx_mcdi_get_num_buses(cdx: *mut cdx_mcdi) -> i32 {
    let mut outbuf = [0u8; MC_CMD_CDX_BUS_ENUM_BUSES_OUT_LEN];
    let mut outlen: usize = 0;
    let ret = cdx_mcdi_rpc(
        cdx,
        MC_CMD_CDX_BUS_ENUM_BUSES,
        core::ptr::null_mut(),
        0,
        outbuf.as_mut_ptr(),
        outbuf.len(),
        &mut outlen,
    );
    if ret != 0 {
        return ret;
    }
    if outlen != MC_CMD_CDX_BUS_ENUM_BUSES_OUT_LEN {
        return -EIO;
    }
    MCDI_DWORD(outbuf.as_ptr(), CDX_BUS_ENUM_BUSES_OUT_BUS_COUNT) as i32
}

pub unsafe fn cdx_mcdi_get_num_devs(cdx: *mut cdx_mcdi, bus_num: i32) -> i32 {
    let mut outbuf = [0u8; MC_CMD_CDX_BUS_ENUM_DEVICES_OUT_LEN];
    let mut inbuf = [0u8; MC_CMD_CDX_BUS_ENUM_DEVICES_IN_LEN];
    let mut outlen: usize = 0;
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_BUS_ENUM_DEVICES_IN_BUS, bus_num);
    let ret = cdx_mcdi_rpc(cdx, MC_CMD_CDX_BUS_ENUM_DEVICES, inbuf.as_mut_ptr(), inbuf.len(),
                           outbuf.as_mut_ptr(), outbuf.len(), &mut outlen);
    if ret != 0 { return ret; }
    if outlen != MC_CMD_CDX_BUS_ENUM_DEVICES_OUT_LEN { return -EIO; }
    MCDI_DWORD(outbuf.as_ptr(), CDX_BUS_ENUM_DEVICES_OUT_DEVICE_COUNT) as i32
}

pub unsafe fn cdx_mcdi_get_dev_config(
    cdx: *mut cdx_mcdi, bus_num: u8, dev_num: u8, dev_params: *mut cdx_dev_params,
) -> i32 {
    let mut outbuf = [0u8; MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_V2_LEN];
    let mut inbuf = [0u8; MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_IN_LEN];
    let res = (*dev_params).res.as_mut_ptr();
    let mut outlen: usize = 0;
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_BUS_GET_DEVICE_CONFIG_IN_BUS, bus_num);
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_BUS_GET_DEVICE_CONFIG_IN_DEVICE, dev_num);
    let ret = cdx_mcdi_rpc(cdx, MC_CMD_CDX_BUS_GET_DEVICE_CONFIG, inbuf.as_mut_ptr(), inbuf.len(),
                           outbuf.as_mut_ptr(), outbuf.len(), &mut outlen);
    if ret != 0 { return ret; }
    if outlen != MC_CMD_CDX_BUS_GET_DEVICE_CONFIG_OUT_V2_LEN { return -EIO; }
    (*dev_params).bus_num = bus_num;
    (*dev_params).dev_num = dev_num;
    (*dev_params).req_id = MCDI_DWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_REQUESTER_ID);
    (*dev_params).msi_dev_id = MCDI_DWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_V2_REQUESTER_DEVICE_ID);
    (*dev_params).res_count = 0;
    if MCDI_QWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_SIZE) != 0 {
        (*res.add((*dev_params).res_count)).start = MCDI_QWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_BASE);
        (*res.add((*dev_params).res_count)).end = (*res.add((*dev_params).res_count)).start + MCDI_QWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION0_SIZE) - 1;
        (*res.add((*dev_params).res_count)).flags = IORESOURCE_MEM;
        (*dev_params).res_count += 1;
    }
    if MCDI_QWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_SIZE) != 0 {
        (*res.add((*dev_params).res_count)).start = MCDI_QWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_BASE);
        (*res.add((*dev_params).res_count)).end = (*res.add((*dev_params).res_count)).start + MCDI_QWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION1_SIZE) - 1;
        (*res.add((*dev_params).res_count)).flags = IORESOURCE_MEM;
        (*dev_params).res_count += 1;
    }
    if MCDI_QWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_SIZE) != 0 {
        (*res.add((*dev_params).res_count)).start = MCDI_QWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_BASE);
        (*res.add((*dev_params).res_count)).end = (*res.add((*dev_params).res_count)).start + MCDI_QWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION2_SIZE) - 1;
        (*res.add((*dev_params).res_count)).flags = IORESOURCE_MEM;
        (*dev_params).res_count += 1;
    }
    if MCDI_QWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_SIZE) != 0 {
        (*res.add((*dev_params).res_count)).start = MCDI_QWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_BASE);
        (*res.add((*dev_params).res_count)).end = (*res.add((*dev_params).res_count)).start + MCDI_QWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MMIO_REGION3_SIZE) - 1;
        (*res.add((*dev_params).res_count)).flags = IORESOURCE_MEM;
        (*dev_params).res_count += 1;
    }
    (*dev_params).vendor = MCDI_WORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_VENDOR_ID);
    (*dev_params).device = MCDI_WORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_DEVICE_ID);
    (*dev_params).subsys_vendor = MCDI_WORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_SUBSYS_VENDOR_ID);
    (*dev_params).subsys_device = MCDI_WORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_SUBSYS_DEVICE_ID);
    (*dev_params).class = MCDI_DWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_DEVICE_CLASS) & 0xFFFFFF;
    (*dev_params).revision = MCDI_BYTE(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_DEVICE_REVISION);
    (*dev_params).num_msi = MCDI_DWORD(outbuf.as_ptr(), CDX_BUS_GET_DEVICE_CONFIG_OUT_MSI_COUNT);
    0
}

pub unsafe fn cdx_mcdi_bus_enable(cdx: *mut cdx_mcdi, bus_num: u8) -> i32 {
    let mut inbuf = [0u8; MC_CMD_CDX_BUS_UP_IN_LEN];
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_BUS_UP_IN_BUS, bus_num);
    cdx_mcdi_rpc(cdx, MC_CMD_CDX_BUS_UP, inbuf.as_mut_ptr(), inbuf.len(), core::ptr::null_mut(), 0, core::ptr::null_mut())
}

pub unsafe fn cdx_mcdi_bus_disable(cdx: *mut cdx_mcdi, bus_num: u8) -> i32 {
    let mut inbuf = [0u8; MC_CMD_CDX_BUS_DOWN_IN_LEN];
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_BUS_DOWN_IN_BUS, bus_num);
    cdx_mcdi_rpc(cdx, MC_CMD_CDX_BUS_DOWN, inbuf.as_mut_ptr(), inbuf.len(), core::ptr::null_mut(), 0, core::ptr::null_mut())
}

pub unsafe fn cdx_mcdi_write_msi(cdx: *mut cdx_mcdi, bus_num: u8, dev_num: u8, msi_vector: u32, msi_address: u64, msi_data: u32) -> i32 {
    let mut inbuf = [0u8; MC_CMD_CDX_DEVICE_WRITE_MSI_MSG_IN_LEN];
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_DEVICE_WRITE_MSI_MSG_IN_BUS, bus_num);
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_DEVICE_WRITE_MSI_MSG_IN_DEVICE, dev_num);
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_VECTOR, msi_vector);
    MCDI_SET_QWORD(inbuf.as_mut_ptr(), CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_ADDRESS, msi_address);
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_DEVICE_WRITE_MSI_MSG_IN_MSI_DATA, msi_data);
    cdx_mcdi_rpc(cdx, MC_CMD_CDX_DEVICE_WRITE_MSI_MSG, inbuf.as_mut_ptr(), inbuf.len(), core::ptr::null_mut(), 0, core::ptr::null_mut())
}

pub unsafe fn cdx_mcdi_reset_device(cdx: *mut cdx_mcdi, bus_num: u8, dev_num: u8) -> i32 {
    let mut inbuf = [0u8; MC_CMD_CDX_DEVICE_RESET_IN_LEN];
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_DEVICE_RESET_IN_BUS, bus_num);
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_DEVICE_RESET_IN_DEVICE, dev_num);
    cdx_mcdi_rpc(cdx, MC_CMD_CDX_DEVICE_RESET, inbuf.as_mut_ptr(), inbuf.len(), core::ptr::null_mut(), 0, core::ptr::null_mut())
}

unsafe fn cdx_mcdi_ctrl_flag_get(cdx: *mut cdx_mcdi, bus_num: u8, dev_num: u8, flags: *mut u32) -> i32 {
    let mut inbuf = [0u8; MC_CMD_CDX_DEVICE_CONTROL_GET_IN_LEN];
    let mut outbuf = [0u8; MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_LEN];
    let mut outlen: usize = 0;
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_DEVICE_CONTROL_GET_IN_BUS, bus_num);
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_DEVICE_CONTROL_GET_IN_DEVICE, dev_num);
    let ret = cdx_mcdi_rpc(cdx, MC_CMD_CDX_DEVICE_CONTROL_GET, inbuf.as_mut_ptr(), inbuf.len(), outbuf.as_mut_ptr(), outbuf.len(), &mut outlen);
    if ret != 0 { return ret; }
    if outlen != MC_CMD_CDX_DEVICE_CONTROL_GET_OUT_LEN { return -EIO; }
    *flags = MCDI_DWORD(outbuf.as_ptr(), CDX_DEVICE_CONTROL_GET_OUT_FLAGS);
    0
}

unsafe fn cdx_mcdi_ctrl_flag_set(cdx: *mut cdx_mcdi, bus_num: u8, dev_num: u8, enable: bool, bit_pos: i32) -> i32 {
    let mut inbuf = [0u8; MC_CMD_CDX_DEVICE_CONTROL_SET_IN_LEN];
    let mut flags: u32 = 0;
    // Get flags and then set/reset bit at bit_pos according to the input params.
    let ret = cdx_mcdi_ctrl_flag_get(cdx, bus_num, dev_num, &mut flags);
    if ret != 0 { return ret; }
    flags &= !(1u32.wrapping_shl(bit_pos as u32));
    if enable { flags |= 1u32.wrapping_shl(bit_pos as u32); }
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_DEVICE_CONTROL_SET_IN_BUS, bus_num);
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_DEVICE_CONTROL_SET_IN_DEVICE, dev_num);
    MCDI_SET_DWORD(inbuf.as_mut_ptr(), CDX_DEVICE_CONTROL_SET_IN_FLAGS, flags);
    cdx_mcdi_rpc(cdx, MC_CMD_CDX_DEVICE_CONTROL_SET, inbuf.as_mut_ptr(), inbuf.len(), core::ptr::null_mut(), 0, core::ptr::null_mut())
}

pub unsafe fn cdx_mcdi_bus_master_enable(cdx: *mut cdx_mcdi, bus_num: u8, dev_num: u8, enable: bool) -> i32 {
    cdx_mcdi_ctrl_flag_set(cdx, bus_num, dev_num, enable, MC_CMD_CDX_DEVICE_CONTROL_SET_IN_BUS_MASTER_ENABLE_LBN)
}

pub unsafe fn cdx_mcdi_msi_enable(cdx: *mut cdx_mcdi, bus_num: u8, dev_num: u8, enable: bool) -> i32 {
    cdx_mcdi_ctrl_flag_set(cdx, bus_num, dev_num, enable, MC_CMD_CDX_DEVICE_CONTROL_SET_IN_MSI_ENABLE_LBN)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
