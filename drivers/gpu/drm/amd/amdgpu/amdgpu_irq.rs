/* Translated from amdgpu_irq.c. */

const AMDGPU_WAIT_IDLE_TIMEOUT: u32 = 200;

pub static mut soc15_ih_clientid_name: [&'static str; 32] = [
    "IH", "SDMA2 or ACP", "ATHUB", "BIF", "SDMA3 or DCE", "SDMA4 or ISP",
    "VMC1 or PCIE0", "RLC", "SDMA0", "SDMA1", "SE0SH", "SE1SH", "SE2SH",
    "SE3SH", "VCN1 or UVD1", "THM", "VCN or UVD", "SDMA5 or VCE0", "VMC",
    "SDMA6 or XDMA", "GRBM_CP", "ATS", "ROM_SMUIO", "DF", "SDMA7 or VCE1",
    "PWR", "reserved", "UTCL2", "EA", "UTCL2LOG", "MP0", "MP1",
];

pub static mut soc_v1_0_ih_clientid_name: [&'static str; 32] = [
    "IH", "Reserved", "ATHUB", "BIF", "Reserved", "Reserved", "Reserved", "RLC",
    "Reserved", "Reserved", "GFX", "IMU", "Reserved", "Reserved", "VCN1 or UVD1",
    "THM", "VCN or UVD", "Reserved", "VMC", "Reserved", "GRBM_CP", "GC_AID",
    "ROM_SMUIO", "DF", "Reserved", "PWR", "LSDMA", "GC_UTCL2", "nHT",
    "Reserved", "MP0", "MP1",
];

pub static mut node_id_to_phys_map: [i32; NODEID_MAX as usize] = {
    let mut a = [0i32; NODEID_MAX as usize];
    a[AID0_NODEID as usize] = 0; a[XCD0_NODEID as usize] = 0;
    a[XCD1_NODEID as usize] = 1; a[AID1_NODEID as usize] = 1;
    a[XCD2_NODEID as usize] = 2; a[XCD3_NODEID as usize] = 3;
    a[AID2_NODEID as usize] = 2; a[XCD4_NODEID as usize] = 4;
    a[XCD5_NODEID as usize] = 5; a[AID3_NODEID as usize] = 3;
    a[XCD6_NODEID as usize] = 6; a[XCD7_NODEID as usize] = 7; a
};

pub unsafe fn amdgpu_irq_disable_all(adev: *mut amdgpu_device) {
    let mut irqflags = 0; spin_lock_irqsave(&mut (*adev).irq.lock, &mut irqflags);
    for i in 0..AMDGPU_IRQ_CLIENTID_MAX { if (*adev).irq.client[i].sources.is_null() { continue; }
        for j in 0..AMDGPU_MAX_IRQ_SRC_ID { let src = *(*adev).irq.client[i].sources.add(j);
            if src.is_null() || (*(*src).funcs).set.is_none() || (*src).num_types == 0 { continue; }
            for k in 0..(*src).num_types { let r = ((*(*src).funcs).set.unwrap())(adev, src, k, AMDGPU_IRQ_STATE_DISABLE); if r != 0 { dev_err((*adev).dev, "error disabling interrupt (%d)\\n", r); } }
        }
    } spin_unlock_irqrestore(&mut (*adev).irq.lock, irqflags);
}

unsafe fn amdgpu_irq_handler(_irq: i32, arg: *mut core::ffi::c_void) -> irqreturn_t {
    let dev = arg as *mut drm_device; let adev = drm_to_adev(dev); let ret = amdgpu_ih_process(adev, &mut (*adev).irq.ih);
    if ret == IRQ_HANDLED { pm_runtime_mark_last_busy((*dev).dev); } amdgpu_ras_interrupt_fatal_error_handler(adev); ret
}

unsafe fn amdgpu_irq_handle_ih1(work: *mut work_struct) { let adev = container_of(work, amdgpu_device, irq.ih1_work); amdgpu_ih_process(adev, &mut (*adev).irq.ih1); }
unsafe fn amdgpu_irq_handle_ih2(work: *mut work_struct) { let adev = container_of(work, amdgpu_device, irq.ih2_work); amdgpu_ih_process(adev, &mut (*adev).irq.ih2); }
unsafe fn amdgpu_irq_handle_ih_soft(work: *mut work_struct) { let adev = container_of(work, amdgpu_device, irq.ih_soft_work); amdgpu_ih_process(adev, &mut (*adev).irq.ih_soft); }

unsafe fn amdgpu_msi_ok(_adev: *mut amdgpu_device) -> bool { if amdgpu_msi == 1 { true } else if amdgpu_msi == 0 { false } else { true } }

pub unsafe fn amdgpu_restore_msix(adev: *mut amdgpu_device) { let mut ctrl: u16 = 0; pci_read_config_word((*adev).pdev, (*adev).pdev.msix_cap + PCI_MSIX_FLAGS, &mut ctrl); if ctrl & PCI_MSIX_FLAGS_ENABLE == 0 { return; } ctrl &= !PCI_MSIX_FLAGS_ENABLE; pci_write_config_word((*adev).pdev, (*adev).pdev.msix_cap + PCI_MSIX_FLAGS, ctrl); ctrl |= PCI_MSIX_FLAGS_ENABLE; pci_write_config_word((*adev).pdev).pdev, (*adev).pdev.msix_cap + PCI_MSIX_FLAGS, ctrl); }

pub unsafe fn amdgpu_irq_init(adev: *mut amdgpu_device) -> i32 {
    (*adev).irq.msi_enabled = false; let flags = if !amdgpu_msi_ok(adev) { PCI_IRQ_INTX } else { PCI_IRQ_ALL_TYPES }; let r = pci_alloc_irq_vectors((*adev).pdev, 1, 1, flags); if r < 0 { dev_err((*adev).dev, "Failed to alloc msi vectors\\n"); return r; }
    if amdgpu_msi_ok(adev) { (*adev).irq.msi_enabled = true; dev_dbg((*adev).dev, "using MSI/MSI-X.\\n"); }
    INIT_WORK(&mut (*adev).irq.ih1_work, amdgpu_irq_handle_ih1); INIT_WORK(&mut (*adev).irq.ih2_work, amdgpu_irq_handle_ih2); INIT_WORK(&mut (*adev).irq.ih_soft_work, amdgpu_irq_handle_ih_soft);
    let irq = pci_irq_vector((*adev).pdev, 0); if irq < 0 { if (*adev).irq.msi_enabled { pci_free_irq_vectors((*adev).pdev); } (*adev).irq.msi_enabled = false; return irq; }
    let r = request_irq(irq as u32, amdgpu_irq_handler, IRQF_SHARED, adev_to_drm(adev).driver.name, adev_to_drm(adev)); if r != 0 { if (*adev).irq.msi_enabled { pci_free_irq_vectors((*adev).pdev); } (*adev).irq.msi_enabled = false; return r; }
    (*adev).irq.installed = true; (*adev).irq.irq = irq as u32; adev_to_drm(adev).max_vblank_count = 0x00ffffff; dev_dbg((*adev).dev, "irq initialized.\\n"); 0
}

pub unsafe fn amdgpu_irq_fini_hw(adev: *mut amdgpu_device) { if (*adev).irq.installed { free_irq((*adev).irq.irq, adev_to_drm(adev)); (*adev).irq.installed = false; if (*adev).irq.msi_enabled { pci_free_irq_vectors((*adev).pdev); } } amdgpu_ih_ring_fini(adev, &mut (*adev).irq.ih_soft); amdgpu_ih_ring_fini(adev, &mut (*adev).irq.ih); amdgpu_ih_ring_fini(adev, &mut (*adev).irq.ih1); amdgpu_ih_ring_fini(adev, &mut (*adev).irq.ih2); }

pub unsafe fn amdgpu_irq_fini_sw(adev: *mut amdgpu_device) { for i in 0..AMDGPU_IRQ_CLIENTID_MAX { if (*adev).irq.client[i].sources.is_null() { continue; } for j in 0..AMDGPU_MAX_IRQ_SRC_ID { let src = *(*adev).irq.client[i].sources.add(j); if !src.is_null() { kfree((*src).enabled_types as *mut _); (*src).enabled_types = core::ptr::null_mut(); } } kfree((*adev).irq.client[i].sources as *mut _); (*adev).irq.client[i].sources = core::ptr::null_mut(); } }

pub unsafe fn amdgpu_irq_add_id(adev: *mut amdgpu_device, client_id: u32, src_id: u32, source: *mut amdgpu_irq_src) -> i32 { if client_id >= AMDGPU_IRQ_CLIENTID_MAX || src_id >= AMDGPU_MAX_IRQ_SRC_ID || (*source).funcs.is_null() { return -EINVAL; } if (*adev).irq.client[client_id as usize].sources.is_null() { (*adev).irq.client[client_id as usize].sources = kzalloc_objs(AMDGPU_MAX_IRQ_SRC_ID); if (*adev).irq.client[client_id as usize].sources.is_null() { return -ENOMEM; } } let slot = (*adev).irq.client[client_id as usize].sources.add(src_id as usize); if !(*slot).is_null() { return -EINVAL; } if (*source).num_types != 0 && (*source).enabled_types.is_null() { let types = kzalloc_objs((*source).num_types); if types.is_null() { return -ENOMEM; } (*source).enabled_types = types; } *slot = source; 0 }

pub unsafe fn amdgpu_irq_dispatch(adev: *mut amdgpu_device, ih: *mut amdgpu_ih_ring) { let ring_index = (*ih).rptr >> 2; let mut entry = amdgpu_iv_entry { ih, iv_entry: (*ih).ring.add(ring_index as usize) as *const u32, timestamp: 0, timestamp_src: 0 }; amdgpu_ih_decode_iv(adev, &mut entry); trace_amdgpu_iv(ih.offset_from(&(*adev).irq.ih), &entry); let client_id = entry.client_id; let src_id = entry.src_id; let mut handled = false; if client_id >= AMDGPU_IRQ_CLIENTID_MAX { dev_dbg((*adev).dev, "Invalid client_id in IV: %d\\n", client_id); } else if src_id >= AMDGPU_MAX_IRQ_SRC_ID { dev_dbg((*adev).dev, "Invalid src_id in IV: %d\\n", src_id); } else if (client_id == AMDGPU_IRQ_CLIENTID_LEGACY || client_id == SOC15_IH_CLIENTID_ISP) && (*adev).irq.virq[src_id as usize] != 0 { generic_handle_domain_irq((*adev).irq.domain, src_id); } else if (*adev).irq.client[client_id as usize].sources.is_null() { dev_dbg((*adev).dev, "Unregistered interrupt client_id: %d src_id: %d\\n", client_id, src_id); } else { let src = *(*adev).irq.client[client_id as usize].sources.add(src_id as usize); if !src.is_null() { let r = ((*(*src).funcs).process.unwrap())(adev, src, &mut entry); if r < 0 { dev_err((*adev).dev, "error processing interrupt (%d)\\n", r); } else if r != 0 { handled = true; } } else { dev_dbg((*adev).dev, "Unregistered interrupt src_id: %d of client_id:%d\\n", src_id, client_id); } } if !handled { amdgpu_amdkfd_interrupt(adev, entry.iv_entry); } if amdgpu_ih_ts_after((*ih).processed_timestamp, entry.timestamp) { (*ih).processed_timestamp = entry.timestamp; } }

pub unsafe fn amdgpu_irq_delegate(adev: *mut amdgpu_device, entry: *mut amdgpu_iv_entry, num_dw: u32) { amdgpu_ih_ring_write(adev, &mut (*adev).irq.ih_soft, (*entry).iv_entry, num_dw); queue_work(system_dfl_wq, &mut (*adev).irq.ih_soft_work); }

pub unsafe fn amdgpu_irq_update(adev: *mut amdgpu_device, src: *mut amdgpu_irq_src, type_: u32) -> i32 { let mut flags = 0; spin_lock_irqsave(&mut (*adev).irq.lock, &mut flags); let state = if amdgpu_irq_enabled(adev, src, type_) { AMDGPU_IRQ_STATE_ENABLE } else { AMDGPU_IRQ_STATE_DISABLE }; let r = ((*(*src).funcs).set.unwrap())(adev, src, type_, state); spin_unlock_irqrestore(&mut (*adev).irq.lock, flags); r }

pub unsafe fn amdgpu_irq_gpu_reset_resume_helper(adev: *mut amdgpu_device) { if amdgpu_sriov_vf(adev) || amdgpu_passthrough(adev) { amdgpu_restore_msix(adev); } for i in 0..AMDGPU_IRQ_CLIENTID_MAX { if (*adev).irq.client[i].sources.is_null() { continue; } for j in 0..AMDGPU_MAX_IRQ_SRC_ID { let src = *(*adev).irq.client[i].sources.add(j); if src.is_null() || (*src).funcs.is_null() || (*(*src).funcs).set.is_none() { continue; } for k in 0..(*src).num_types { amdgpu_irq_update(adev, src, k); } } } }

pub unsafe fn amdgpu_irq_get(adev: *mut amdgpu_device, src: *mut amdgpu_irq_src, type_: u32) -> i32 { if !(*adev).irq.installed { return -ENOENT; } if type_ >= (*src).num_types || (*src).enabled_types.is_null() || (*(*src).funcs).set.is_none() { return -EINVAL; } if atomic_inc_return((*src).enabled_types.add(type_ as usize)) == 1 { return amdgpu_irq_update(adev, src, type_); } 0 }
pub unsafe fn amdgpu_irq_put(adev: *mut amdgpu_device, src: *mut amdgpu_irq_src, type_: u32) -> i32 { if amdgpu_ras_is_rma(adev) && !amdgpu_irq_enabled(adev, src, type_) { return -EINVAL; } if !(*adev).irq.installed { return -ENOENT; } if type_ >= (*src).num_types || (*src).enabled_types.is_null() || (*(*src).funcs).set.is_none() { return -EINVAL; } if WARN_ON(!amdgpu_irq_enabled(adev, src, type_)) { return -EINVAL; } if atomic_dec_and_test((*src).enabled_types.add(type_ as usize)) { return amdgpu_irq_update(adev, src, type_); } 0 }
pub unsafe fn amdgpu_irq_enabled(adev: *mut amdgpu_device, src: *mut amdgpu_irq_src, type_: u32) -> bool { if !(*adev).irq.installed || type_ >= (*src).num_types || (*src).enabled_types.is_null() || (*(*src).funcs).set.is_none() { return false; } atomic_read((*src).enabled_types.add(type_ as usize)) != 0 }

unsafe fn amdgpu_irq_mask(_irqd: *mut irq_data) {}
unsafe fn amdgpu_irq_unmask(_irqd: *mut irq_data) {}
static mut amdgpu_irq_chip: irq_chip = irq_chip { name: "amdgpu-ih", irq_mask: Some(amdgpu_irq_mask), irq_unmask: Some(amdgpu_irq_unmask) };
unsafe fn amdgpu_irqdomain_map(_d: *mut irq_domain, irq: u32, hwirq: irq_hw_number_t) -> i32 { if hwirq >= AMDGPU_MAX_IRQ_SRC_ID as _ { return -EPERM; } irq_set_chip_and_handler(irq, &mut amdgpu_irq_chip, handle_simple_irq); 0 }
static mut amdgpu_hw_irqdomain_ops: irq_domain_ops = irq_domain_ops { map: Some(amdgpu_irqdomain_map) };
pub unsafe fn amdgpu_irq_add_domain(adev: *mut amdgpu_device) -> i32 { (*adev).irq.domain = irq_domain_create_linear(core::ptr::null_mut(), AMDGPU_MAX_IRQ_SRC_ID, &mut amdgpu_hw_irqdomain_ops, adev); if (*adev).irq.domain.is_null() { dev_err((*adev).dev, "GPU irq add domain failed\\n"); return -ENODEV; } 0 }
pub unsafe fn amdgpu_irq_remove_domain(adev: *mut amdgpu_device) { if !(*adev).irq.domain.is_null() { irq_domain_remove((*adev).irq.domain); (*adev).irq.domain = core::ptr::null_mut(); } }
pub unsafe fn amdgpu_irq_create_mapping(adev: *mut amdgpu_device, src_id: u32) -> u32 { (*adev).irq.virq[src_id as usize] = irq_create_mapping((*adev).irq.domain, src_id); (*adev).irq.virq[src_id as usize] }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
