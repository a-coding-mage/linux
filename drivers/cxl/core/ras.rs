// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2025 AMD Corporation. All rights reserved. */

// External kernel and CXL dependencies are supplied by the surrounding crate.

const _: () = assert!(CXL_HEADERLOG_TRACE_SIZE_U32 == 128);

unsafe fn cxl_cper_trace_corr_port_prot_err(
    pdev: *mut pci_dev,
    ras_cap: cxl_ras_capability_regs,
) {
    let status = ras_cap.cor_status & !ras_cap.cor_mask;
    trace_cxl_port_aer_correctable_error(&mut (*pdev).dev, status);
}

unsafe fn cxl_cper_trace_uncorr_port_prot_err(
    pdev: *mut pci_dev,
    ras_cap: cxl_ras_capability_regs,
) {
    let mut hl = [0u32; CXL_HEADERLOG_TRACE_SIZE_U32 as usize];
    let status = ras_cap.uncor_status & !ras_cap.uncor_mask;
    let fe;

    if hweight32(status) > 1 {
        fe = 1u32 << field_get(CXL_RAS_CAP_CONTROL_FE_MASK, ras_cap.cap_control);
    } else {
        fe = status;
    }

    memcpy(
        hl.as_mut_ptr() as *mut c_void,
        ras_cap.header_log.as_ptr() as *const c_void,
        CXL_HEADERLOG_SIZE as usize,
    );
    trace_cxl_port_aer_uncorrectable_error(&mut (*pdev).dev, status, fe, hl.as_ptr());
}

unsafe fn cxl_cper_trace_corr_prot_err(
    cxlmd: *mut cxl_memdev,
    ras_cap: cxl_ras_capability_regs,
) {
    let status = ras_cap.cor_status & !ras_cap.cor_mask;
    trace_cxl_aer_correctable_error(cxlmd, status);
}

unsafe fn cxl_cper_trace_uncorr_prot_err(
    cxlmd: *mut cxl_memdev,
    ras_cap: cxl_ras_capability_regs,
) {
    let mut hl = [0u32; CXL_HEADERLOG_TRACE_SIZE_U32 as usize];
    let status = ras_cap.uncor_status & !ras_cap.uncor_mask;
    let fe;

    if hweight32(status) > 1 {
        fe = 1u32 << field_get(CXL_RAS_CAP_CONTROL_FE_MASK, ras_cap.cap_control);
    } else {
        fe = status;
    }

    /* Copy hardware dwords into the zero-filled trace staging buffer. */
    memcpy(
        hl.as_mut_ptr() as *mut c_void,
        ras_cap.header_log.as_ptr() as *const c_void,
        CXL_HEADERLOG_SIZE as usize,
    );
    trace_cxl_aer_uncorrectable_error(cxlmd, status, fe, hl.as_ptr());
}

unsafe fn match_memdev_by_parent(dev: *mut device, uport: *const c_void) -> i32 {
    if is_cxl_memdev(dev) && (*dev).parent == uport {
        return 1;
    }
    0
}

pub unsafe fn cxl_cper_handle_prot_err(data: *mut cxl_cper_prot_err_work_data) {
    let devfn = pci_devfn((*data).prot_err.agent_addr.device,
                          (*data).prot_err.agent_addr.function);
    let pdev = pci_get_domain_bus_and_slot(
        (*data).prot_err.agent_addr.segment,
        (*data).prot_err.agent_addr.bus,
        devfn,
    );
    if pdev.is_null() {
        return;
    }

    let port_type = pci_pcie_type(pdev);
    if port_type == PCI_EXP_TYPE_ROOT_PORT
        || port_type == PCI_EXP_TYPE_DOWNSTREAM
        || port_type == PCI_EXP_TYPE_UPSTREAM
    {
        if (*data).severity == AER_CORRECTABLE {
            cxl_cper_trace_corr_port_prot_err(pdev, (*data).ras_cap);
        } else {
            cxl_cper_trace_uncorr_port_prot_err(pdev, (*data).ras_cap);
        }
        pci_dev_put(pdev);
        return;
    }

    device_lock(&mut (*pdev).dev);
    if (*pdev).dev.driver.is_null() {
        device_unlock(&mut (*pdev).dev);
        pci_dev_put(pdev);
        return;
    }
    let mem_dev = bus_find_device(&cxl_bus_type, core::ptr::null_mut(), pdev as *mut c_void,
                                  Some(match_memdev_by_parent));
    if !mem_dev.is_null() {
        let cxlmd = to_cxl_memdev(mem_dev);
        if (*data).severity == AER_CORRECTABLE {
            cxl_cper_trace_corr_prot_err(cxlmd, (*data).ras_cap);
        } else {
            cxl_cper_trace_uncorr_prot_err(cxlmd, (*data).ras_cap);
        }
        put_device(mem_dev);
    }
    device_unlock(&mut (*pdev).dev);
    pci_dev_put(pdev);
}

unsafe fn cxl_cper_prot_err_work_fn(work: *mut work_struct) {
    let mut wd = core::mem::MaybeUninit::<cxl_cper_prot_err_work_data>::uninit();
    while cxl_cper_prot_err_kfifo_get(wd.as_mut_ptr()) {
        cxl_cper_handle_prot_err(wd.as_mut_ptr());
    }
}

static mut cxl_cper_prot_err_work: work_struct = DECLARE_WORK(cxl_cper_prot_err_work_fn);

pub unsafe fn cxl_ras_init() {
    cxl_cper_register_prot_err_work(&mut cxl_cper_prot_err_work);
}

pub unsafe fn cxl_ras_exit() {
    cxl_cper_unregister_prot_err_work();
}

unsafe fn cxl_dport_map_ras(dport: *mut cxl_dport) {
    let map = &mut (*dport).reg_map;
    let dev = (*dport).dport_dev;
    if !map.component_map.ras.valid {
        dev_dbg(dev, "RAS registers not found\n");
    } else if cxl_map_component_regs(map, &mut (*dport).regs.component,
                                     1u32 << CXL_CM_CAP_CAP_ID_RAS) {
        dev_dbg(dev, "Failed to map RAS capability.\n");
    }
}

pub unsafe fn devm_cxl_dport_ras_setup(dport: *mut cxl_dport) {
    (*dport).reg_map.host = dport_to_host(dport);
    cxl_dport_map_ras(dport);
}

pub unsafe fn devm_cxl_dport_rch_ras_setup(dport: *mut cxl_dport) {
    if !dev_is_pci((*dport).dport_dev) {
        return;
    }
    devm_cxl_dport_ras_setup(dport);
    let host_bridge = to_pci_host_bridge((*dport).dport_dev);
    if !(*host_bridge).native_aer {
        return;
    }
    cxl_dport_map_rch_aer(dport);
    cxl_disable_rch_root_ints(dport);
}

pub unsafe fn devm_cxl_port_ras_setup(port: *mut cxl_port) {
    let map = &mut (*port).reg_map;
    if !map.component_map.ras.valid {
        dev_dbg(&mut (*port).dev, "RAS registers not found\n");
        return;
    }
    map.host = &mut (*port).dev;
    if cxl_map_component_regs(map, &mut (*port).regs, 1u32 << CXL_CM_CAP_CAP_ID_RAS) {
        dev_dbg(&mut (*port).dev, "Failed to map RAS capability\n");
    }
}

pub unsafe fn cxl_handle_cor_ras(dev: *mut device, ras_base: *mut c_void) {
    if ras_base.is_null() { return; }
    let addr = (ras_base as *mut u8).add(CXL_RAS_CORRECTABLE_STATUS_OFFSET as usize) as *mut c_void;
    let status = readl(addr);
    if status & CXL_RAS_CORRECTABLE_STATUS_MASK != 0 {
        writel(status & CXL_RAS_CORRECTABLE_STATUS_MASK, addr);
        trace_cxl_aer_correctable_error(to_cxl_memdev(dev), status);
    }
}

/* CXL spec rev3.0 8.2.4.16.1 */
unsafe fn header_log_copy(ras_base: *mut c_void, log: *mut u32) {
    let addr = (ras_base as *mut u8).add(CXL_RAS_HEADER_LOG_OFFSET as usize) as *mut c_void;
    for i in 0..CXL_HEADERLOG_SIZE_U32 as usize {
        *log.add(i) = readl(addr.add(i * core::mem::size_of::<u32>()));
    }
}

pub unsafe fn cxl_handle_ras(dev: *mut device, ras_base: *mut c_void) -> bool {
    let mut hl = [0u32; CXL_HEADERLOG_TRACE_SIZE_U32 as usize];
    if ras_base.is_null() { return false; }
    let addr = (ras_base as *mut u8).add(CXL_RAS_UNCORRECTABLE_STATUS_OFFSET as usize) as *mut c_void;
    let status = readl(addr);
    if status & CXL_RAS_UNCORRECTABLE_STATUS_MASK == 0 { return false; }
    let fe = if hweight32(status) > 1 {
        let rcc_addr = (ras_base as *mut u8).add(CXL_RAS_CAP_CONTROL_OFFSET as usize) as *mut c_void;
        1u32 << field_get(CXL_RAS_CAP_CONTROL_FE_MASK, readl(rcc_addr))
    } else { status };
    header_log_copy(ras_base, hl.as_mut_ptr());
    trace_cxl_aer_uncorrectable_error(to_cxl_memdev(dev), status, fe, hl.as_ptr());
    writel(status & CXL_RAS_UNCORRECTABLE_STATUS_MASK, addr);
    true
}

pub unsafe fn cxl_cor_error_detected(pdev: *mut pci_dev) {
    let cxlds = pci_get_drvdata(pdev);
    let cxlmd = (*cxlds).cxlmd;
    let dev = &mut (*cxlmd).dev;
    device_lock(dev);
    if (*dev).driver.is_null() {
        dev_warn(&mut (*pdev).dev, "%s: memdev disabled, abort error handling\n", dev_name(dev));
        device_unlock(dev);
        return;
    }
    if (*cxlds).rcd { cxl_handle_rdport_errors(cxlds); }
    cxl_handle_cor_ras(&mut (*cxlmd).dev, (*(*cxlds).cxlmd).endpoint.regs.ras);
    device_unlock(dev);
}

pub unsafe fn cxl_error_detected(pdev: *mut pci_dev, state: pci_channel_state_t) -> pci_ers_result_t {
    let cxlds = pci_get_drvdata(pdev);
    let cxlmd = (*cxlds).cxlmd;
    let dev = &mut (*cxlmd).dev;
    let ue;
    device_lock(dev);
    if (*dev).driver.is_null() {
        dev_warn(&mut (*pdev).dev, "%s: memdev disabled, abort error handling\n", dev_name(dev));
        device_unlock(dev);
        return PCI_ERS_RESULT_DISCONNECT;
    }
    if (*cxlds).rcd { cxl_handle_rdport_errors(cxlds); }
    ue = cxl_handle_ras(&mut (*cxlmd).dev, (*cxlmd).endpoint.regs.ras);
    device_unlock(dev);
    match state {
        pci_channel_io_normal => if ue { device_release_driver(dev); PCI_ERS_RESULT_NEED_RESET } else { PCI_ERS_RESULT_CAN_RECOVER },
        pci_channel_io_frozen => {
            dev_warn(&mut (*pdev).dev, "%s: frozen state error detected, disable CXL.mem\n", dev_name(dev));
            device_release_driver(dev); PCI_ERS_RESULT_NEED_RESET
        },
        pci_channel_io_perm_failure => {
            dev_warn(&mut (*pdev).dev, "failure state error detected, request disconnect\n");
            PCI_ERS_RESULT_DISCONNECT
        },
        _ => PCI_ERS_RESULT_NEED_RESET,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
