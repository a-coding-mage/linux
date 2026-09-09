// SPDX-License-Identifier: GPL-2.0-or-later
/* Rust translation of eeh_pseries.c. External kernel symbols are supplied by
 * the surrounding translation unit. */

static mut ibm_set_eeh_option: i32 = 0;
static mut ibm_set_slot_reset: i32 = 0;
static mut ibm_read_slot_reset_state: i32 = 0;
static mut ibm_read_slot_reset_state2: i32 = 0;
static mut ibm_slot_error_detail: i32 = 0;
static mut ibm_get_config_addr_info: i32 = 0;
static mut ibm_get_config_addr_info2: i32 = 0;
static mut ibm_configure_pe: i32 = 0;

unsafe fn pseries_eeh_init_edev(pdn: *mut pci_dn);

unsafe fn pseries_pcibios_bus_add_device(pdev: *mut pci_dev) {
    let pdn = pci_get_pdn(pdev);
    if eeh_has_flag(EEH_FORCE_DISABLED) { return; }
    dev_dbg(&mut (*pdev).dev, "EEH: Setting up device\n");
    // CONFIG_PCI_IOV fields and handling are conditional in the source.
    pseries_eeh_init_edev(pdn);
    eeh_probe_device(pdev);
}

unsafe fn pseries_eeh_get_pe_config_addr(pdn: *mut pci_dn) -> i32 {
    let config_addr = rtas_config_addr((*pdn).busno, (*pdn).devfn, 0);
    let phb = (*pdn).phb;
    let mut rets = [0i32; 3];
    let mut ret;
    if ibm_get_config_addr_info2 != RTAS_UNKNOWN_SERVICE {
        ret = rtas_call(ibm_get_config_addr_info2, 4, 2, rets.as_mut_ptr(), config_addr,
                         BUID_HI((*phb).buid), BUID_LO((*phb).buid), 1);
        if ret != 0 || rets[0] == 0 { return -ENOENT; }
        ret = rtas_call(ibm_get_config_addr_info2, 4, 2, rets.as_mut_ptr(), config_addr,
                        BUID_HI((*phb).buid), BUID_LO((*phb).buid), 0);
        if ret != 0 { pr_warn("%s: Failed to get address for PHB#%x-PE#%x\n", __func__, (*phb).global_number, config_addr); return -ENXIO; }
        return rets[0];
    }
    if ibm_get_config_addr_info != RTAS_UNKNOWN_SERVICE {
        ret = rtas_call(ibm_get_config_addr_info, 4, 2, rets.as_mut_ptr(), config_addr,
                        BUID_HI((*phb).buid), BUID_LO((*phb).buid), 0);
        if ret != 0 { pr_warn("%s: Failed to get address for PHB#%x-PE#%x\n", __func__, (*phb).global_number, config_addr); return -ENXIO; }
        return rets[0];
    }
    -ENOENT
}

unsafe fn pseries_eeh_phb_reset(phb: *mut pci_controller, config_addr: i32, mut option: i32) -> i32 {
    let mut ret = rtas_call(ibm_set_slot_reset, 4, 1, core::ptr::null_mut(), config_addr,
                            BUID_HI((*phb).buid), BUID_LO((*phb).buid), option);
    if option == EEH_RESET_FUNDAMENTAL && ret == -8 {
        option = EEH_RESET_HOT;
        ret = rtas_call(ibm_set_slot_reset, 4, 1, core::ptr::null_mut(), config_addr,
                        BUID_HI((*phb).buid), BUID_LO((*phb).buid), option);
    }
    if option == EEH_RESET_FUNDAMENTAL || option == EEH_RESET_HOT { msleep(EEH_PE_RST_HOLD_TIME); }
    else { msleep(EEH_PE_RST_SETTLE_TIME); }
    ret
}

unsafe fn pseries_eeh_phb_configure_bridge(phb: *mut pci_controller, config_addr: i32) -> i32 {
    let mut max_wait = 200;
    let mut ret = 0;
    while max_wait > 0 {
        ret = rtas_call(ibm_configure_pe, 3, 1, core::ptr::null_mut(), config_addr,
                        BUID_HI((*phb).buid), BUID_LO((*phb).buid));
        if ret == 0 { return ret; }
        if ret < 0 { break; }
        if ret > RTAS_EXTENDED_DELAY_MIN + 2 && ret <= RTAS_EXTENDED_DELAY_MAX { ret = RTAS_EXTENDED_DELAY_MIN + 2; }
        max_wait -= rtas_busy_delay_time(ret);
        if max_wait < 0 { break; }
        rtas_busy_delay(ret);
    }
    pr_warn("%s: Unable to configure bridge PHB#%x-PE#%x (%d)\n", __func__, (*phb).global_number, config_addr, ret);
    if ret == -3 { -EINVAL } else { -EIO }
}

static mut slot_errbuf: [u8; RTAS_ERROR_LOG_MAX as usize] = [0; RTAS_ERROR_LOG_MAX as usize];
static mut eeh_error_buf_size: i32 = 0;

unsafe fn pseries_eeh_cap_start(pdn: *mut pci_dn) -> i32 {
    if pdn.is_null() { return 0; }
    let mut status = 0u32;
    rtas_pci_dn_read_config(pdn, PCI_STATUS, 2, &mut status);
    if status & PCI_STATUS_CAP_LIST == 0 { 0 } else { PCI_CAPABILITY_LIST }
}

unsafe fn pseries_eeh_find_cap(pdn: *mut pci_dn, cap: i32) -> i32 {
    let mut pos = pseries_eeh_cap_start(pdn);
    let mut cnt = 48;
    let mut id = 0u32;
    if pos == 0 { return 0; }
    while cnt > 0 {
        cnt -= 1;
        rtas_pci_dn_read_config(pdn, pos, 1, &mut pos as *mut i32 as *mut u32);
        if pos < 0x40 { break; }
        pos &= !3;
        rtas_pci_dn_read_config(pdn, pos + PCI_CAP_LIST_ID, 1, &mut id);
        if id == 0xff { break; }
        if id == cap as u32 { return pos; }
        pos += PCI_CAP_LIST_NEXT;
    }
    0
}

unsafe fn pseries_eeh_find_ecap(pdn: *mut pci_dn, cap: i32) -> i32 {
    let edev = pdn_to_eeh_dev(pdn);
    let mut header = 0u32;
    let mut pos = 256;
    let mut ttl = (4096 - 256) / 8;
    if edev.is_null() || (*edev).pcie_cap == 0 { return 0; }
    if rtas_pci_dn_read_config(pdn, pos, 4, &mut header) != PCIBIOS_SUCCESSFUL || header == 0 { return 0; }
    while ttl > 0 {
        ttl -= 1;
        if PCI_EXT_CAP_ID(header) == cap && pos != 0 { return pos; }
        pos = PCI_EXT_CAP_NEXT(header);
        if pos < 256 { break; }
        if rtas_pci_dn_read_config(pdn, pos, 4, &mut header) != PCIBIOS_SUCCESSFUL { break; }
    }
    0
}

unsafe fn pseries_eeh_pe_get_parent(edev: *mut eeh_dev) -> *mut eeh_pe {
    let mut pdn = eeh_dev_to_pdn(edev);
    if !(*edev).physfn.is_null() { pdn = pci_get_pdn((*edev).physfn); }
    else if !pdn.is_null() { pdn = (*pdn).parent; }
    while !pdn.is_null() {
        let parent = pdn_to_eeh_dev(pdn);
        if parent.is_null() { return core::ptr::null_mut(); }
        if !(*parent).pe.is_null() { return (*parent).pe; }
        pdn = (*pdn).parent;
    }
    core::ptr::null_mut()
}

unsafe fn pseries_eeh_init_edev(pdn: *mut pci_dn) {
    let edev = pdn_to_eeh_dev(pdn);
    if edev.is_null() || !(*edev).pe.is_null() || (*pdn).vendor_id == 0 || (*pdn).device_id == 0 || (*pdn).class_code == 0 { return; }
    if ((*pdn).class_code >> 8) == PCI_CLASS_BRIDGE_ISA { return; }
    (*edev).pcix_cap = pseries_eeh_find_cap(pdn, PCI_CAP_ID_PCIX);
    (*edev).pcie_cap = pseries_eeh_find_cap(pdn, PCI_CAP_ID_EXP);
    (*edev).aer_cap = pseries_eeh_find_ecap(pdn, PCI_EXT_CAP_ID_ERR);
    (*edev).mode &= 0xFFFFFF00;
    if ((*pdn).class_code >> 8) == PCI_CLASS_BRIDGE_PCI { (*edev).mode |= EEH_DEV_BRIDGE; }
    let ret = pseries_eeh_get_pe_config_addr(pdn);
    if ret < 0 { return; }
    let mut pe: eeh_pe = core::mem::zeroed(); pe.phb = (*pdn).phb; pe.addr = ret;
    if eeh_ops.set_option(&mut pe, EEH_OPT_ENABLE) != 0 { return; }
    (*edev).pe_config_addr = pe.addr;
    eeh_add_flag(EEH_ENABLED);
    eeh_pe_tree_insert(edev, pseries_eeh_pe_get_parent(edev));
    eeh_save_bars(edev);
}

unsafe fn pseries_eeh_probe(pdev: *mut pci_dev) -> *mut eeh_dev {
    let pdn = pci_get_pdn_by_devfn((*pdev).bus, (*pdev).devfn);
    if pdn.is_null() { return core::ptr::null_mut(); }
    let edev = pdn_to_eeh_dev(pdn);
    if edev.is_null() || (*edev).pe.is_null() { core::ptr::null_mut() } else { edev }
}

pub unsafe fn pseries_eeh_init_edev_recursive(pdn: *mut pci_dn) {
    if pdn.is_null() { return; }
    list_for_each_entry!(n, (*pdn).child_list, list, { pseries_eeh_init_edev_recursive(n); });
    pseries_eeh_init_edev(pdn);
}

unsafe fn pseries_eeh_set_option(pe: *mut eeh_pe, option: i32) -> i32 {
    match option { EEH_OPT_DISABLE | EEH_OPT_ENABLE | EEH_OPT_THAW_MMIO | EEH_OPT_THAW_DMA => {}, EEH_OPT_FREEZE_PE => return 0, _ => return -EINVAL }
    rtas_call(ibm_set_eeh_option, 4, 1, core::ptr::null_mut(), (*pe).addr,
              BUID_HI((*(*pe).phb).buid), BUID_LO((*(*pe).phb).buid), option)
}

unsafe fn pseries_eeh_get_state(pe: *mut eeh_pe, delay: *mut i32) -> i32 {
    let mut rets = [0i32; 4];
    let ret = if ibm_read_slot_reset_state2 != RTAS_UNKNOWN_SERVICE { rtas_call(ibm_read_slot_reset_state2, 3, 4, rets.as_mut_ptr(), (*pe).addr, BUID_HI((*(*pe).phb).buid), BUID_LO((*(*pe).phb).buid)) }
    else if ibm_read_slot_reset_state != RTAS_UNKNOWN_SERVICE { rets[2] = 0; rtas_call(ibm_read_slot_reset_state, 3, 3, rets.as_mut_ptr(), (*pe).addr, BUID_HI((*(*pe).phb).buid), BUID_LO((*(*pe).phb).buid)) }
    else { return EEH_STATE_NOT_SUPPORT };
    if ret != 0 { return ret; } if rets[1] == 0 { return EEH_STATE_NOT_SUPPORT; }
    match rets[0] { 0 => EEH_STATE_MMIO_ACTIVE | EEH_STATE_DMA_ACTIVE | EEH_STATE_MMIO_ENABLED | EEH_STATE_DMA_ENABLED, 1 => EEH_STATE_RESET_ACTIVE | EEH_STATE_MMIO_ACTIVE | EEH_STATE_DMA_ACTIVE, 2 => 0, 4 => EEH_STATE_MMIO_ENABLED, 5 => if rets[2] != 0 { if !delay.is_null() { *delay = rets[2]; } EEH_STATE_UNAVAILABLE } else { EEH_STATE_NOT_SUPPORT }, _ => EEH_STATE_NOT_SUPPORT }
}

unsafe fn pseries_eeh_reset(pe: *mut eeh_pe, option: i32) -> i32 { pseries_eeh_phb_reset((*pe).phb, (*pe).addr, option) }
unsafe fn pseries_eeh_configure_bridge(pe: *mut eeh_pe) -> i32 { pseries_eeh_phb_configure_bridge((*pe).phb, (*pe).addr) }
unsafe fn pseries_eeh_read_config(edev: *mut eeh_dev, where_: i32, size: i32, val: *mut u32) -> i32 { rtas_pci_dn_read_config(eeh_dev_to_pdn(edev), where_, size, val) }
unsafe fn pseries_eeh_write_config(edev: *mut eeh_dev, where_: i32, size: i32, val: u32) -> i32 { rtas_pci_dn_write_config(eeh_dev_to_pdn(edev), where_, size, val) }

unsafe fn pseries_eeh_get_log(pe: *mut eeh_pe, severity: i32, drv_log: *mut i8, len: usize) -> i32 {
    let mut flags = 0usize;
    spin_lock_irqsave(&mut slot_errbuf_lock, &mut flags);
    core::ptr::write_bytes(slot_errbuf.as_mut_ptr(), 0, eeh_error_buf_size as usize);
    let ret = rtas_call(ibm_slot_error_detail, 8, 1, core::ptr::null_mut(), (*pe).addr,
        BUID_HI((*(*pe).phb).buid), BUID_LO((*(*pe).phb).buid), virt_to_phys(drv_log), len,
        virt_to_phys(slot_errbuf.as_mut_ptr()), eeh_error_buf_size, severity);
    if ret == 0 { log_error(slot_errbuf.as_mut_ptr(), ERR_TYPE_RTAS_LOG, 0); }
    spin_unlock_irqrestore(&mut slot_errbuf_lock, flags);
    ret
}

unsafe fn pseries_eeh_err_inject(pe: *mut eeh_pe, type_: i32, func: i32, _addr: usize, _mask: usize) -> i32 {
    if type_ != EEH_ERR_TYPE_32 && type_ != EEH_ERR_TYPE_64 { return -EINVAL; }
    match func { EEH_ERR_FUNC_LD_MEM_ADDR | EEH_ERR_FUNC_LD_MEM_DATA | EEH_ERR_FUNC_ST_MEM_ADDR | EEH_ERR_FUNC_ST_MEM_DATA => { pci_lock_rescan_remove(); list_for_each_entry!(pdev, (*pe).edevs, entry, { eeh_pe_inject_mmio_error((*pdev).pdev); }); pci_unlock_rescan_remove(); 0 }, _ => -ERANGE }
}

static mut pseries_eeh_ops: eeh_ops = eeh_ops {
    name: "pseries", probe: Some(pseries_eeh_probe), set_option: Some(pseries_eeh_set_option),
    get_state: Some(pseries_eeh_get_state), reset: Some(pseries_eeh_reset),
    get_log: Some(pseries_eeh_get_log), configure_bridge: Some(pseries_eeh_configure_bridge),
    err_inject: Some(pseries_eeh_err_inject), read_config: Some(pseries_eeh_read_config),
    write_config: Some(pseries_eeh_write_config), next_error: None, restore_config: None,
};

unsafe fn eeh_pseries_init() -> i32 {
    ibm_set_eeh_option = rtas_function_token(RTAS_FN_IBM_SET_EEH_OPTION);
    ibm_set_slot_reset = rtas_function_token(RTAS_FN_IBM_SET_SLOT_RESET);
    ibm_read_slot_reset_state2 = rtas_function_token(RTAS_FN_IBM_READ_SLOT_RESET_STATE2);
    ibm_read_slot_reset_state = rtas_function_token(RTAS_FN_IBM_READ_SLOT_RESET_STATE);
    ibm_slot_error_detail = rtas_function_token(RTAS_FN_IBM_SLOT_ERROR_DETAIL);
    ibm_get_config_addr_info2 = rtas_function_token(RTAS_FN_IBM_GET_CONFIG_ADDR_INFO2);
    ibm_get_config_addr_info = rtas_function_token(RTAS_FN_IBM_GET_CONFIG_ADDR_INFO);
    ibm_configure_pe = rtas_function_token(RTAS_FN_IBM_CONFIGURE_PE);
    if ibm_configure_pe == RTAS_UNKNOWN_SERVICE { ibm_configure_pe = rtas_function_token(RTAS_FN_IBM_CONFIGURE_BRIDGE); }
    if ibm_set_eeh_option == RTAS_UNKNOWN_SERVICE || ibm_set_slot_reset == RTAS_UNKNOWN_SERVICE ||
       (ibm_read_slot_reset_state2 == RTAS_UNKNOWN_SERVICE && ibm_read_slot_reset_state == RTAS_UNKNOWN_SERVICE) ||
       ibm_slot_error_detail == RTAS_UNKNOWN_SERVICE || ibm_configure_pe == RTAS_UNKNOWN_SERVICE { return -EINVAL; }
    eeh_error_buf_size = rtas_get_error_log_max();
    eeh_add_flag(EEH_PROBE_MODE_DEVTREE | EEH_ENABLE_IO_FOR_LOG);
    ppc_md.pcibios_bus_add_device = Some(pseries_pcibios_bus_add_device);
    eeh_init(&mut pseries_eeh_ops)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
