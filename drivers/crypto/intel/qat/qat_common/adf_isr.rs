// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
/* Copyright(c) 2014 - 2020 Intel Corporation */

// Dependencies are supplied by the surrounding kernel/driver translation.

const ADF_MAX_NUM_VFS: usize = 32;
static mut adf_misc_wq: *mut workqueue_struct = core::ptr::null_mut();

unsafe fn adf_enable_msix(accel_dev: *mut adf_accel_dev) -> i32 {
    let pci_dev_info = &mut (*accel_dev).accel_pci_dev;
    let hw_data = (*accel_dev).hw_device;
    let msix_num_entries: u32 = (*hw_data).num_banks + 1;
    let ret: i32;

    if let Some(set_msix_rttable) = (*hw_data).set_msix_rttable {
        set_msix_rttable(accel_dev);
    }

    ret = pci_alloc_irq_vectors(pci_dev_info.pci_dev, msix_num_entries,
                                 msix_num_entries, PCI_IRQ_MSIX);
    if ret < 0 {
        dev_err(&GET_DEV(accel_dev), "Failed to allocate %d MSI-X vectors\n", msix_num_entries);
        return ret;
    }
    0
}

unsafe fn adf_disable_msix(pci_dev_info: *mut adf_accel_pci) {
    pci_free_irq_vectors((*pci_dev_info).pci_dev);
}

unsafe extern "C" fn adf_msix_isr_bundle(_irq: i32, bank_ptr: *mut core::ffi::c_void) -> irqreturn_t {
    let bank = bank_ptr as *mut adf_etr_bank_data;
    let csr_ops = GET_CSR_OPS((*bank).accel_dev);
    ((*csr_ops).write_csr_int_flag_and_col)((*bank).csr_addr, (*bank).bank_number, 0);
    tasklet_hi_schedule(&mut (*bank).resp_handler);
    IRQ_HANDLED
}

#[cfg(CONFIG_PCI_IOV)]
pub unsafe fn adf_enable_vf2pf_interrupts(accel_dev: *mut adf_accel_dev, vf_mask: u32) {
    let pmisc_addr = adf_get_pmisc_base(accel_dev);
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*accel_dev).pf.vf2pf_ints_lock, &mut flags);
    if !READ_ONCE((*accel_dev).pf.vf2pf_disabled) {
        ((*GET_PFVF_OPS(accel_dev)).enable_vf2pf_interrupts)(pmisc_addr, vf_mask);
    }
    spin_unlock_irqrestore(&mut (*accel_dev).pf.vf2pf_ints_lock, flags);
}

#[cfg(CONFIG_PCI_IOV)]
pub unsafe fn adf_enable_all_vf2pf_interrupts(accel_dev: *mut adf_accel_dev, num_vfs: u32) {
    let pmisc_addr = adf_get_pmisc_base(accel_dev);
    let mut flags: c_ulong = 0;
    let vf_mask = (1u64 << num_vfs) - 1;
    if vf_mask == 0 { return; }
    spin_lock_irqsave(&mut (*accel_dev).pf.vf2pf_ints_lock, &mut flags);
    WRITE_ONCE((*accel_dev).pf.vf2pf_disabled, false);
    ((*GET_PFVF_OPS(accel_dev)).enable_vf2pf_interrupts)(pmisc_addr, vf_mask as u32);
    spin_unlock_irqrestore(&mut (*accel_dev).pf.vf2pf_ints_lock, flags);
}

#[cfg(CONFIG_PCI_IOV)]
pub unsafe fn adf_disable_all_vf2pf_interrupts(accel_dev: *mut adf_accel_dev) {
    let pmisc_addr = adf_get_pmisc_base(accel_dev);
    let mut flags: c_ulong = 0;
    spin_lock_irqsave(&mut (*accel_dev).pf.vf2pf_ints_lock, &mut flags);
    WRITE_ONCE((*accel_dev).pf.vf2pf_disabled, true);
    ((*GET_PFVF_OPS(accel_dev)).disable_all_vf2pf_interrupts)(pmisc_addr);
    spin_unlock_irqrestore(&mut (*accel_dev).pf.vf2pf_ints_lock, flags);
}

#[cfg(CONFIG_PCI_IOV)]
unsafe fn adf_disable_pending_vf2pf_interrupts(accel_dev: *mut adf_accel_dev) -> u32 {
    let pmisc_addr = adf_get_pmisc_base(accel_dev);
    spin_lock(&mut (*accel_dev).pf.vf2pf_ints_lock);
    let pending = ((*GET_PFVF_OPS(accel_dev)).disable_pending_vf2pf_interrupts)(pmisc_addr);
    spin_unlock(&mut (*accel_dev).pf.vf2pf_ints_lock);
    pending
}

#[cfg(CONFIG_PCI_IOV)]
unsafe fn adf_handle_vf2pf_int(accel_dev: *mut adf_accel_dev) -> bool {
    let mut irq_handled = false;
    let vf_mask = adf_disable_pending_vf2pf_interrupts(accel_dev);
    if vf_mask != 0 {
        for i in 0..ADF_MAX_NUM_VFS {
            if (vf_mask & (1u32 << i)) == 0 { continue; }
            let vf_info = (*accel_dev).pf.vf_info.add(i);
            if !__ratelimit(&mut (*vf_info).vf2pf_ratelimit) {
                dev_info(&GET_DEV(accel_dev), "Too many ints from VF%d\n", (*vf_info).vf_nr);
                continue;
            }
            adf_schedule_vf2pf_handler(vf_info);
            irq_handled = true;
        }
    }
    irq_handled
}

unsafe fn adf_handle_pm_int(accel_dev: *mut adf_accel_dev) -> bool {
    let hw_data = (*accel_dev).hw_device;
    (*hw_data).handle_pm_interrupt.map_or(false, |f| f(accel_dev))
}

unsafe fn adf_handle_ras_int(accel_dev: *mut adf_accel_dev) -> bool {
    let ras_ops = &mut (*(*accel_dev).hw_device).ras_ops;
    let mut reset_required = false;
    if let Some(handle_interrupt) = ras_ops.handle_interrupt {
        if handle_interrupt(accel_dev, &mut reset_required) {
            if reset_required {
                dev_err(&GET_DEV(accel_dev), "Fatal error, reset required\n");
                if adf_notify_fatal_error(accel_dev) {
                    dev_err(&GET_DEV(accel_dev), "Failed to notify fatal error\n");
                }
            }
            return true;
        }
    }
    false
}

unsafe extern "C" fn adf_msix_isr_ae(_irq: i32, dev_ptr: *mut core::ffi::c_void) -> irqreturn_t {
    let accel_dev = dev_ptr as *mut adf_accel_dev;
    #[cfg(CONFIG_PCI_IOV)]
    if !(*accel_dev).pf.vf_info.is_null() && adf_handle_vf2pf_int(accel_dev) { return IRQ_HANDLED; }
    if adf_handle_pm_int(accel_dev) { return IRQ_HANDLED; }
    if adf_handle_ras_int(accel_dev) { return IRQ_HANDLED; }
    dev_dbg(&GET_DEV(accel_dev), "qat_dev%d spurious AE interrupt\n", (*accel_dev).accel_id);
    IRQ_NONE
}

pub unsafe fn adf_isr_sync_ae_cluster(accel_dev: *mut adf_accel_dev) {
    let pci_dev_info = &mut (*accel_dev).accel_pci_dev;
    let hw_data = GET_HW_DATA(accel_dev);
    let num_entries = pci_dev_info.msix_entries.num_entries;
    let irqs = pci_dev_info.msix_entries.irqs;
    if !test_bit(ADF_STATUS_IRQ_ALLOCATED, &(*accel_dev).status) || irqs.is_null() { return; }
    let irq_idx = if num_entries > 1 { (*hw_data).num_banks } else { 0 };
    if irq_idx >= num_entries || !(*irqs.add(irq_idx as usize)).enabled { return; }
    let irq = pci_irq_vector(pci_dev_info.pci_dev, (*hw_data).num_banks);
    if irq > 0 { synchronize_irq(irq); }
}

unsafe fn adf_free_irqs(accel_dev: *mut adf_accel_dev) {
    let pci_dev_info = &mut (*accel_dev).accel_pci_dev;
    let hw_data = (*accel_dev).hw_device;
    let irqs = pci_dev_info.msix_entries.irqs;
    let etr_data = (*accel_dev).transport;
    let clust_irq = (*hw_data).num_banks;
    let mut i = 0;
    if pci_dev_info.msix_entries.num_entries > 1 {
        for i0 in 0..(*hw_data).num_banks as usize {
            i = i0;
            if (*irqs.add(i0)).enabled {
                let irq = pci_irq_vector(pci_dev_info.pci_dev, i0 as u32);
                irq_set_affinity_hint(irq, core::ptr::null());
                free_irq(irq, &mut (*etr_data).banks[i0]);
            }
        }
    }
    if (*irqs.add(i)).enabled {
        let irq = pci_irq_vector(pci_dev_info.pci_dev, clust_irq);
        free_irq(irq, accel_dev);
    }
}

// Remaining resource-management routines retain the source ordering and delegate to
// the corresponding kernel/driver dependency declarations.
unsafe fn adf_request_irqs(accel_dev: *mut adf_accel_dev) -> i32 {
    let pci = &mut (*accel_dev).accel_pci_dev;
    let hw = (*accel_dev).hw_device;
    let irqs = pci.msix_entries.irqs;
    let etr = (*accel_dev).transport;
    let clust_irq = (*hw).num_banks;
    let mut i = 0usize;
    if (*accel_dev).pf.vf_info.is_null() {
        for j in 0..(*hw).num_banks as usize {
            i = j;
            let bank = &mut (*etr).banks[j];
            let name = (*irqs.add(j)).name;
            snprintf(name, ADF_MAX_MSIX_VECTOR_NAME, "qat%d-bundle%d", (*accel_dev).accel_id, j);
            let irq = pci_irq_vector(pci.pci_dev, j as u32);
            if irq < 0 { adf_free_irqs(accel_dev); return irq; }
            let ret = request_irq(irq, adf_msix_isr_bundle, 0, name, bank);
            if ret != 0 { adf_free_irqs(accel_dev); return ret; }
            let cpu = (((*accel_dev).accel_id * (*hw).num_banks) + j as u32) % num_online_cpus();
            irq_set_affinity_hint(irq, get_cpu_mask(cpu));
            (*irqs.add(j)).enabled = true;
        }
    }
    let name = (*irqs.add(i)).name;
    snprintf(name, ADF_MAX_MSIX_VECTOR_NAME, "qat%d-ae-cluster", (*accel_dev).accel_id);
    let irq = pci_irq_vector(pci.pci_dev, clust_irq);
    if irq < 0 { adf_free_irqs(accel_dev); return irq; }
    let ret = request_irq(irq, adf_msix_isr_ae, 0, name, accel_dev);
    if ret != 0 { adf_free_irqs(accel_dev); return ret; }
    (*irqs.add(i)).enabled = true;
    ret
}

unsafe fn adf_isr_alloc_msix_vectors_data(accel_dev: *mut adf_accel_dev) -> i32 {
    let hw = (*accel_dev).hw_device;
    let mut n = 1u32;
    if (*accel_dev).pf.vf_info.is_null() { n += (*hw).num_banks; }
    let irqs = kcalloc_node(n as usize, core::mem::size_of::<adf_irq>(), GFP_KERNEL, dev_to_node(&GET_DEV(accel_dev)));
    if irqs.is_null() { return -ENOMEM; }
    (*accel_dev).accel_pci_dev.msix_entries.num_entries = n;
    (*accel_dev).accel_pci_dev.msix_entries.irqs = irqs;
    0
}

unsafe fn adf_isr_free_msix_vectors_data(accel_dev: *mut adf_accel_dev) {
    kfree((*accel_dev).accel_pci_dev.msix_entries.irqs as *mut core::ffi::c_void);
    (*accel_dev).accel_pci_dev.msix_entries.irqs = core::ptr::null_mut();
}

unsafe fn adf_setup_bh(accel_dev: *mut adf_accel_dev) -> i32 {
    let data = (*accel_dev).transport;
    for i in 0..(*(*accel_dev).hw_device).num_banks as usize {
        tasklet_init(&mut (*data).banks[i].resp_handler, adf_response_handler, &mut (*data).banks[i] as *mut _ as usize);
    }
    0
}

unsafe fn adf_cleanup_bh(accel_dev: *mut adf_accel_dev) {
    let data = (*accel_dev).transport;
    for i in 0..(*(*accel_dev).hw_device).num_banks as usize {
        tasklet_disable(&mut (*data).banks[i].resp_handler);
        tasklet_kill(&mut (*data).banks[i].resp_handler);
    }
}

pub unsafe fn adf_isr_resource_free(accel_dev: *mut adf_accel_dev) {
    adf_free_irqs(accel_dev);
    adf_cleanup_bh(accel_dev);
    adf_disable_msix(&mut (*accel_dev).accel_pci_dev);
    adf_isr_free_msix_vectors_data(accel_dev);
}

pub unsafe fn adf_isr_resource_alloc(accel_dev: *mut adf_accel_dev) -> i32 {
    let mut ret = adf_isr_alloc_msix_vectors_data(accel_dev);
    if ret != 0 { return ret; }
    ret = adf_enable_msix(accel_dev);
    if ret != 0 { adf_isr_free_msix_vectors_data(accel_dev); return ret; }
    ret = adf_setup_bh(accel_dev);
    if ret != 0 { adf_disable_msix(&mut (*accel_dev).accel_pci_dev); adf_isr_free_msix_vectors_data(accel_dev); return ret; }
    ret = adf_request_irqs(accel_dev);
    if ret != 0 { adf_cleanup_bh(accel_dev); adf_disable_msix(&mut (*accel_dev).accel_pci_dev); adf_isr_free_msix_vectors_data(accel_dev); return ret; }
    0
}

pub unsafe fn adf_init_misc_wq() -> i32 {
    adf_misc_wq = alloc_workqueue("qat_misc_wq", WQ_MEM_RECLAIM | WQ_PERCPU, 0);
    if adf_misc_wq.is_null() { -ENOMEM } else { 0 }
}

pub unsafe fn adf_exit_misc_wq() {
    if !adf_misc_wq.is_null() { destroy_workqueue(adf_misc_wq); }
    adf_misc_wq = core::ptr::null_mut();
}

pub unsafe fn adf_misc_wq_queue_work(work: *mut work_struct) -> bool { queue_work(adf_misc_wq, work) }
pub unsafe fn adf_misc_wq_queue_delayed_work(work: *mut delayed_work, delay: c_ulong) -> bool { queue_delayed_work(adf_misc_wq, work, delay) }
pub unsafe fn adf_misc_wq_flush() { flush_workqueue(adf_misc_wq); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
