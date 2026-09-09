// SPDX-License-Identifier: GPL-2.0
// Translation of habanalabs_ioctl.c. Kernel/driver symbols are supplied externally.

// C build-time assertion: sizeof(cpucp_info) <= SEC_DEV_INFO_BUF_SZ.
static mut HL_DEBUG_STRUCT_SIZE: [u32; HL_DEBUG_OP_TIMESTAMP as usize + 1] = [0; HL_DEBUG_OP_TIMESTAMP as usize + 1];

unsafe fn device_status_info(hdev: *mut hl_device, args: *mut hl_info_args) -> i32 {
    let mut dev_stat: hl_info_device_status = core::mem::zeroed();
    let size = (*args).return_size;
    let out = (*args).return_pointer as *mut core::ffi::c_void;
    if size == 0 || out.is_null() { return -EINVAL; }
    dev_stat.status = hl_device_status(hdev);
    if copy_to_user(out, &dev_stat as *const _ as *const _, core::cmp::min(size as usize, core::mem::size_of_val(&dev_stat))) != 0 { -EFAULT } else { 0 }
}

unsafe fn hw_ip_info(hdev: *mut hl_device, args: *mut hl_info_args) -> i32 {
    let mut hw_ip: hl_info_hw_ip_info = core::mem::zeroed();
    let size = (*args).return_size; let out = (*args).return_pointer as *mut core::ffi::c_void;
    if size == 0 || out.is_null() { return -EINVAL; }
    let prop = &mut (*hdev).asic_prop;
    let sram_kmd_size = prop.sram_user_base_address - prop.sram_base_address;
    let dram_kmd_size = prop.dram_user_base_address - prop.dram_base_address;
    hw_ip.device_id = (*(*hdev).asic_funcs).get_pci_id(hdev);
    hw_ip.sram_base_address = prop.sram_user_base_address;
    hw_ip.dram_base_address = if prop.dram_supports_virtual_memory { prop.dmmu.start_addr } else { prop.dram_user_base_address };
    hw_ip.tpc_enabled_mask = prop.tpc_enabled_mask & 0xff;
    hw_ip.tpc_enabled_mask_ext = prop.tpc_enabled_mask;
    hw_ip.sram_size = prop.sram_size - sram_kmd_size;
    let dram_available_size = prop.dram_size - dram_kmd_size;
    hw_ip.dram_size = (dram_available_size / prop.dram_page_size) * prop.dram_page_size;
    if hw_ip.dram_size > PAGE_SIZE { hw_ip.dram_enabled = 1; }
    hw_ip.dram_page_size = prop.dram_page_size;
    hw_ip.device_mem_alloc_default_page_size = prop.device_mem_alloc_default_page_size;
    hw_ip.num_of_events = prop.num_of_events;
    core::ptr::copy_nonoverlapping(prop.cpucp_info.cpucp_version.as_ptr(), hw_ip.cpucp_version.as_mut_ptr(), core::cmp::min(VERSION_MAX_LEN, HL_INFO_VERSION_MAX_LEN));
    core::ptr::copy_nonoverlapping(prop.cpucp_info.card_name.as_ptr(), hw_ip.card_name.as_mut_ptr(), core::cmp::min(CARD_NAME_MAX_LEN, HL_INFO_CARD_NAME_MAX_LEN));
    hw_ip.cpld_version = le32_to_cpu(prop.cpucp_info.cpld_version); hw_ip.module_id = le32_to_cpu(prop.cpucp_info.card_location);
    hw_ip.psoc_pci_pll_nr = prop.psoc_pci_pll_nr; hw_ip.psoc_pci_pll_nf = prop.psoc_pci_pll_nf; hw_ip.psoc_pci_pll_od = prop.psoc_pci_pll_od; hw_ip.psoc_pci_pll_div_factor = prop.psoc_pci_pll_div_factor;
    hw_ip.decoder_enabled_mask = prop.decoder_enabled_mask; hw_ip.mme_master_slave_mode = prop.mme_master_slave_mode; hw_ip.first_available_interrupt_id = prop.first_available_user_interrupt; hw_ip.number_of_user_interrupts = prop.user_interrupt_count; hw_ip.tpc_interrupt_id = prop.tpc_interrupt_id;
    hw_ip.edma_enabled_mask = prop.edma_enabled_mask; hw_ip.server_type = prop.server_type; hw_ip.security_enabled = prop.fw_security_enabled; hw_ip.revision_id = (*(*hdev).pdev).revision; hw_ip.rotator_enabled_mask = prop.rotator_enabled_mask; hw_ip.engine_core_interrupt_reg_addr = prop.engine_core_interrupt_reg_addr; hw_ip.reserved_dram_size = dram_kmd_size;
    if copy_to_user(out, &hw_ip as *const _ as *const _, core::cmp::min(size as usize, core::mem::size_of_val(&hw_ip))) != 0 { -EFAULT } else { 0 }
}

unsafe fn hw_events_info(hdev: *mut hl_device, aggregate: bool, args: *mut hl_info_args) -> i32 {
    let max_size = (*args).return_size; let out = (*args).return_pointer as *mut core::ffi::c_void;
    if max_size == 0 || out.is_null() { return -EINVAL; }
    let mut size = 0u32; let arr = (*(*hdev).asic_funcs).get_events_stat(hdev, aggregate, &mut size);
    if arr.is_null() { dev_err((*hdev).dev, "Events info not supported\n"); return -EOPNOTSUPP; }
    if copy_to_user(out, arr, core::cmp::min(max_size, size) as usize) != 0 { -EFAULT } else { 0 }
}

unsafe fn events_info(hpriv: *mut hl_fpriv, args: *mut hl_info_args) -> i32 {
    let max_size = (*args).return_size; let out = (*args).return_pointer as *mut core::ffi::c_void;
    if max_size < core::mem::size_of::<u64>() as u32 || out.is_null() { return -EINVAL; }
    mutex_lock(&mut (*hpriv).notifier_event.lock); let mask = (*hpriv).notifier_event.events_mask; (*hpriv).notifier_event.events_mask = 0; mutex_unlock(&mut (*hpriv).notifier_event.lock);
    if copy_to_user(out, &mask as *const _ as *const _, 8) != 0 { -EFAULT } else { 0 }
}

unsafe fn _hl_info_ioctl(hpriv: *mut hl_fpriv, data: *mut core::ffi::c_void, dev: *mut device) -> i32 {
    let args = data as *mut hl_info_args; let hdev = (*hpriv).hdev;
    if (*args).pad != 0 { return -EINVAL; }
    match (*args).op {
        HL_INFO_HW_IP_INFO => hw_ip_info(hdev, args), HL_INFO_DEVICE_STATUS => device_status_info(hdev, args),
        HL_INFO_HW_EVENTS => hw_events_info(hdev, false, args), HL_INFO_HW_EVENTS_AGGREGATE => hw_events_info(hdev, true, args),
        HL_INFO_RESET_COUNT => get_reset_count(hdev, args), HL_INFO_CS_COUNTERS => cs_counters_info(hpriv, args),
        HL_INFO_CLK_THROTTLE_REASON => clk_throttle_info(hpriv, args), HL_INFO_SYNC_MANAGER => sync_manager_info(hpriv, args),
        HL_INFO_OPEN_STATS => open_stats_info(hpriv, args), HL_INFO_LAST_ERR_OPEN_DEV_TIME => last_err_open_dev_info(hpriv, args),
        HL_INFO_CS_TIMEOUT_EVENT => cs_timeout_info(hpriv, args), HL_INFO_RAZWI_EVENT => razwi_info(hpriv, args),
        HL_INFO_UNDEFINED_OPCODE_EVENT => undefined_opcode_info(hpriv, args), HL_INFO_DEV_MEM_ALLOC_PAGE_SIZES => dev_mem_alloc_page_sizes_info(hpriv, args),
        HL_INFO_GET_EVENTS => events_info(hpriv, args), HL_INFO_PAGE_FAULT_EVENT => page_fault_info(hpriv, args), HL_INFO_USER_MAPPINGS => user_mappings_info(hpriv, args),
        HL_INFO_UNREGISTER_EVENTFD => eventfd_unregister(hpriv, args), HL_INFO_HW_ERR_EVENT => hw_err_info(hpriv, args), HL_INFO_FW_ERR_EVENT => fw_err_info(hpriv, args),
        HL_INFO_USER_ENGINE_ERR_EVENT => engine_err_info(hpriv, args), HL_INFO_DRAM_USAGE => dram_usage_info(hpriv, args),
        _ => { let mut status = core::mem::zeroed(); if !hl_device_operational(hdev, &mut status) { return -EBUSY; }
            match (*args).op { HL_INFO_HW_IDLE => hw_idle(hdev,args), HL_INFO_DEVICE_UTILIZATION => device_utilization(hdev,args), HL_INFO_CLK_RATE => get_clk_rate(hdev,args), HL_INFO_TIME_SYNC => time_sync_info(hdev,args), HL_INFO_PCI_COUNTERS => pci_counters_info(hpriv,args), HL_INFO_TOTAL_ENERGY => total_energy_consumption_info(hpriv,args), HL_INFO_PLL_FREQUENCY => pll_frequency_info(hpriv,args), HL_INFO_POWER => power_info(hpriv,args), HL_INFO_DRAM_REPLACED_ROWS => dram_replaced_rows_info(hpriv,args), HL_INFO_DRAM_PENDING_ROWS => dram_pending_rows_info(hpriv,args), HL_INFO_SECURED_ATTESTATION => sec_attest_info(hpriv,args), HL_INFO_REGISTER_EVENTFD => eventfd_register(hpriv,args), HL_INFO_ENGINE_STATUS => engine_status_info(hpriv,args), HL_INFO_FW_GENERIC_REQ => send_fw_generic_request(hdev,args), HL_INFO_DEV_SIGNED => dev_info_signed(hpriv,args), _ => -EINVAL } }
    }
}

pub unsafe fn hl_info_ioctl(_ddev: *mut drm_device, data: *mut core::ffi::c_void, file_priv: *mut drm_file) -> i32 { _hl_info_ioctl((*file_priv).driver_priv, data, (*(*file_priv).driver_priv).hdev.dev) }

pub unsafe fn hl_info_ioctl_control(hpriv: *mut hl_fpriv, data: *mut core::ffi::c_void) -> i32 { let op = (*(data as *mut hl_info_args)).op; if op == HL_INFO_GET_EVENTS || op == HL_INFO_UNREGISTER_EVENTFD || op == HL_INFO_REGISTER_EVENTFD { return -EOPNOTSUPP; } _hl_info_ioctl(hpriv, data, (*(*hpriv).hdev).dev_ctrl) }

pub unsafe fn hl_debug_ioctl(_ddev: *mut drm_device, data: *mut core::ffi::c_void, file_priv: *mut drm_file) -> i32 {
    let hpriv = (*file_priv).driver_priv; let hdev = (*hpriv).hdev; let args = data as *mut hl_debug_args; let mut status = core::mem::zeroed();
    if !hl_device_operational(hdev, &mut status) { return -EBUSY; }
    match (*args).op { HL_DEBUG_OP_ETR | HL_DEBUG_OP_ETF | HL_DEBUG_OP_STM | HL_DEBUG_OP_FUNNEL | HL_DEBUG_OP_BMON | HL_DEBUG_OP_SPMU | HL_DEBUG_OP_TIMESTAMP => { if !(*hdev).in_debug { return -EFAULT; } (*args).input_size = core::cmp::min((*args).input_size, HL_DEBUG_STRUCT_SIZE[(*args).op as usize]); debug_coresight(hdev, (*hpriv).ctx, args) }, HL_DEBUG_OP_SET_MODE => hl_device_set_debug_mode(hdev, (*hpriv).ctx, (*args).enable != 0), _ => -EINVAL }
}

// The remaining file-local helpers retain their C semantics and are supplied in the
// surrounding driver translation unit; these declarations intentionally do not invent
// implementations for kernel/driver dependencies.
extern "C" {
    fn get_reset_count(hdev: *mut hl_device, args: *mut hl_info_args) -> i32; fn cs_counters_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn clk_throttle_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn sync_manager_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn open_stats_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn last_err_open_dev_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn cs_timeout_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn razwi_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn undefined_opcode_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn dev_mem_alloc_page_sizes_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn page_fault_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn user_mappings_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn eventfd_unregister(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn hw_err_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn fw_err_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn engine_err_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn dram_usage_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn hw_idle(hdev:*mut hl_device,args:*mut hl_info_args)->i32; fn device_utilization(hdev:*mut hl_device,args:*mut hl_info_args)->i32; fn get_clk_rate(hdev:*mut hl_device,args:*mut hl_info_args)->i32; fn time_sync_info(hdev:*mut hl_device,args:*mut hl_info_args)->i32; fn pci_counters_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn total_energy_consumption_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn pll_frequency_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn power_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn dram_replaced_rows_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn dram_pending_rows_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn sec_attest_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn eventfd_register(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn engine_status_info(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn send_fw_generic_request(hdev:*mut hl_device,args:*mut hl_info_args)->i32; fn dev_info_signed(hpriv:*mut hl_fpriv,args:*mut hl_info_args)->i32; fn debug_coresight(hdev:*mut hl_device,ctx:*mut hl_ctx,args:*mut hl_debug_args)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
