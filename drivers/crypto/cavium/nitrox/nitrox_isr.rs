// SPDX-License-Identifier: GPL-2.0

// Kernel and project headers from the C translation unit provide the external
// types, constants, macros, and functions referenced below.

const NR_RING_VECTORS: i32 = 3;
const NR_NON_RING_VECTORS: i32 = 1;
const PKT_RING_MSIX_BASE: i32 = 0;
const NON_RING_MSIX_BASE: i32 = 192;

unsafe extern "C" {
    fn nps_pkt_slc_isr(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t;
}

unsafe fn nps_pkt_slc_isr_impl(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let qvec = data as *mut nitrox_q_vector;
    let mut slc_cnts: nps_pkt_slc_cnts = core::mem::zeroed();
    let cmdq = (*qvec).cmdq;

    slc_cnts.value = readq((*cmdq).compl_cnt_csr_addr);
    // New packet on SLC output port
    if slc_cnts.s.slc_int != 0 {
        tasklet_hi_schedule(&mut (*qvec).resp_tasklet);
    }

    IRQ_HANDLED
}

unsafe fn clear_nps_core_err_intr(ndev: *mut nitrox_device) {
    // Write 1 to clear
    let value = nitrox_read_csr(ndev, NPS_CORE_INT);
    nitrox_write_csr(ndev, NPS_CORE_INT, value);
    dev_err_ratelimited(DEV(ndev), "NSP_CORE_INT  0x%016llx\n", value);
}

unsafe fn clear_nps_pkt_err_intr(ndev: *mut nitrox_device) {
    let mut pkt_int: nps_pkt_int = core::mem::zeroed();
    let mut value: usize;
    let mut offset: usize;
    pkt_int.value = nitrox_read_csr(ndev, NPS_PKT_INT);
    dev_err_ratelimited(DEV(ndev), "NPS_PKT_INT  0x%016llx\n", pkt_int.value);

    if pkt_int.s.slc_err != 0 {
        offset = NPS_PKT_SLC_ERR_TYPE;
        value = nitrox_read_csr(ndev, offset);
        nitrox_write_csr(ndev, offset, value);
        dev_err_ratelimited(DEV(ndev), "NPS_PKT_SLC_ERR_TYPE  0x%016lx\n", value);
        offset = NPS_PKT_SLC_RERR_LO;
        value = nitrox_read_csr(ndev, offset);
        nitrox_write_csr(ndev, offset, value);
        // enable the solicit ports
        for i in 0..BITS_PER_LONG {
            if (value & (1usize << i)) != 0 { enable_pkt_solicit_port(ndev, i as i32); }
        }
        dev_err_ratelimited(DEV(ndev), "NPS_PKT_SLC_RERR_LO  0x%016lx\n", value);
        offset = NPS_PKT_SLC_RERR_HI;
        value = nitrox_read_csr(ndev, offset);
        nitrox_write_csr(ndev, offset, value);
        dev_err_ratelimited(DEV(ndev), "NPS_PKT_SLC_RERR_HI  0x%016lx\n", value);
    }
    if pkt_int.s.in_err != 0 {
        offset = NPS_PKT_IN_ERR_TYPE;
        value = nitrox_read_csr(ndev, offset);
        nitrox_write_csr(ndev, offset, value);
        dev_err_ratelimited(DEV(ndev), "NPS_PKT_IN_ERR_TYPE  0x%016lx\n", value);
        offset = NPS_PKT_IN_RERR_LO;
        value = nitrox_read_csr(ndev, offset);
        nitrox_write_csr(ndev, offset, value);
        // enable the input ring
        for i in 0..BITS_PER_LONG {
            if (value & (1usize << i)) != 0 { enable_pkt_input_ring(ndev, i as i32); }
        }
        dev_err_ratelimited(DEV(ndev), "NPS_PKT_IN_RERR_LO  0x%016lx\n", value);
        offset = NPS_PKT_IN_RERR_HI;
        value = nitrox_read_csr(ndev, offset);
        nitrox_write_csr(ndev, offset, value);
        dev_err_ratelimited(DEV(ndev), "NPS_PKT_IN_RERR_HI  0x%016lx\n", value);
    }
}

unsafe fn clear_pom_err_intr(ndev: *mut nitrox_device) {
    let value = nitrox_read_csr(ndev, POM_INT);
    nitrox_write_csr(ndev, POM_INT, value);
    dev_err_ratelimited(DEV(ndev), "POM_INT  0x%016llx\n", value);
}

unsafe fn clear_pem_err_intr(ndev: *mut nitrox_device) {
    let value = nitrox_read_csr(ndev, PEM0_INT);
    nitrox_write_csr(ndev, PEM0_INT, value);
    dev_err_ratelimited(DEV(ndev), "PEM(0)_INT  0x%016llx\n", value);
}

unsafe fn clear_lbc_err_intr(ndev: *mut nitrox_device) {
    let mut lbc_int: lbc_int = core::mem::zeroed();
    let mut value: u64;
    let mut offset: u64;
    lbc_int.value = nitrox_read_csr(ndev, LBC_INT);
    dev_err_ratelimited(DEV(ndev), "LBC_INT  0x%016llx\n", lbc_int.value);
    if lbc_int.s.dma_rd_err != 0 {
        for i in 0..NR_CLUSTERS { offset = EFL_CORE_VF_ERR_INT0X(i); value = nitrox_read_csr(ndev, offset); nitrox_write_csr(ndev, offset, value); offset = EFL_CORE_VF_ERR_INT1X(i); value = nitrox_read_csr(ndev, offset); nitrox_write_csr(ndev, offset, value); }
    }
    if lbc_int.s.cam_soft_err != 0 { dev_err_ratelimited(DEV(ndev), "CAM_SOFT_ERR, invalidating LBC\n"); invalidate_lbc(ndev); }
    if lbc_int.s.pref_dat_len_mismatch_err != 0 { offset = LBC_PLM_VF1_64_INT; value = nitrox_read_csr(ndev, offset); nitrox_write_csr(ndev, offset, value); offset = LBC_PLM_VF65_128_INT; value = nitrox_read_csr(ndev, offset); nitrox_write_csr(ndev, offset, value); }
    if lbc_int.s.rd_dat_len_mismatch_err != 0 { offset = LBC_ELM_VF1_64_INT; value = nitrox_read_csr(ndev, offset); nitrox_write_csr(ndev, offset, value); offset = LBC_ELM_VF65_128_INT; value = nitrox_read_csr(ndev, offset); nitrox_write_csr(ndev, offset, value); }
    nitrox_write_csr(ndev, LBC_INT, lbc_int.value);
}

unsafe fn clear_efl_err_intr(ndev: *mut nitrox_device) {
    for i in 0..NR_CLUSTERS { let mut core_int: efl_core_int = core::mem::zeroed(); let offset = EFL_CORE_INTX(i); core_int.value = nitrox_read_csr(ndev, offset); nitrox_write_csr(ndev, offset, core_int.value); dev_err_ratelimited(DEV(ndev), "ELF_CORE(%d)_INT  0x%016llx\n", i, core_int.value); if core_int.s.se_err != 0 { let offset = EFL_CORE_SE_ERR_INTX(i); let value = nitrox_read_csr(ndev, offset); nitrox_write_csr(ndev, offset, value); } }
}

unsafe fn clear_bmi_err_intr(ndev: *mut nitrox_device) { let value = nitrox_read_csr(ndev, BMI_INT); nitrox_write_csr(ndev, BMI_INT, value); dev_err_ratelimited(DEV(ndev), "BMI_INT  0x%016llx\n", value); }

unsafe fn nps_core_int_tasklet(data: usize) {
    let qvec = data as *mut nitrox_q_vector;
    let ndev = (*qvec).ndev;
    // if pf mode do queue recovery
    if (*ndev).mode == __NDEV_MODE_PF {
    } else {
        // if VF(s) enabled communicate the error information to VF(s)
    }
}

unsafe fn nps_core_int_isr(_irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let qvec = data as *mut nitrox_q_vector;
    let ndev = (*qvec).ndev;
    let mut core_int: nps_core_int_active = core::mem::zeroed();
    core_int.value = nitrox_read_csr(ndev, NPS_CORE_INT_ACTIVE);
    if core_int.s.nps_core != 0 { clear_nps_core_err_intr(ndev); }
    if core_int.s.nps_pkt != 0 { clear_nps_pkt_err_intr(ndev); }
    if core_int.s.pom != 0 { clear_pom_err_intr(ndev); }
    if core_int.s.pem != 0 { clear_pem_err_intr(ndev); }
    if core_int.s.lbc != 0 { clear_lbc_err_intr(ndev); }
    if core_int.s.efl != 0 { clear_efl_err_intr(ndev); }
    if core_int.s.bmi != 0 { clear_bmi_err_intr(ndev); }
    // Mailbox interrupt
    if core_int.s.mbox != 0 { nitrox_pf2vf_mbox_handler(ndev); }
    // If more work callback the ISR, set resend
    core_int.s.resend = 1;
    nitrox_write_csr(ndev, NPS_CORE_INT_ACTIVE, core_int.value);
    IRQ_HANDLED
}

// The registration/unregistration routines retain the C kernel API calls and
// structure field accesses directly; declarations are supplied by included
// kernel/project bindings.
pub unsafe fn nitrox_unregister_interrupts(ndev: *mut nitrox_device) {
    let pdev = (*ndev).pdev;
    for i in 0..(*ndev).num_vecs { let qvec = (*ndev).qvec.add(i as usize); if !(*qvec).valid { continue; } let vec = pci_irq_vector(pdev, i); irq_set_affinity_hint(vec, core::ptr::null_mut()); free_irq(vec, qvec as *mut core::ffi::c_void); tasklet_disable(&mut (*qvec).resp_tasklet); tasklet_kill(&mut (*qvec).resp_tasklet); (*qvec).valid = false; }
    kfree((*ndev).qvec as *mut core::ffi::c_void); (*ndev).qvec = core::ptr::null_mut(); pci_free_irq_vectors(pdev);
}

pub unsafe fn nitrox_register_interrupts(ndev: *mut nitrox_device) -> i32 {
    let pdev = (*ndev).pdev;
    let nr_vecs = pci_msix_vec_count(pdev); if nr_vecs < 0 { dev_err(DEV(ndev), "Error in getting vec count %d\n", nr_vecs); return nr_vecs; }
    let ret = pci_alloc_irq_vectors(pdev, nr_vecs, nr_vecs, PCI_IRQ_MSIX); if ret < 0 { dev_err(DEV(ndev), "msix vectors %d alloc failed\n", nr_vecs); return ret; }
    (*ndev).num_vecs = nr_vecs; (*ndev).qvec = kzalloc_objs(nr_vecs); if (*ndev).qvec.is_null() { pci_free_irq_vectors(pdev); return -ENOMEM; }
    let mut i = PKT_RING_MSIX_BASE; while i < nr_vecs - 1 { let qvec = (*ndev).qvec.add(i as usize); (*qvec).ring = i / NR_RING_VECTORS; if (*qvec).ring >= (*ndev).nr_queues { break; } (*qvec).cmdq = (*ndev).pkt_inq.add((*qvec).ring as usize); snprintf((*qvec).name.as_mut_ptr(), IRQ_NAMESZ, "nitrox-pkt%d", (*qvec).ring); let vec = pci_irq_vector(pdev, i); let ret = request_irq(vec, nps_pkt_slc_isr_impl, 0, (*qvec).name.as_ptr(), qvec as *mut core::ffi::c_void); if ret != 0 { dev_err(DEV(ndev), "irq failed for pkt ring/port%d\n", (*qvec).ring); nitrox_unregister_interrupts(ndev); return ret; } let cpu = (*qvec).ring % num_online_cpus(); irq_set_affinity_hint(vec, get_cpu_mask(cpu)); tasklet_init(&mut (*qvec).resp_tasklet, pkt_slc_resp_tasklet, qvec as usize); (*qvec).valid = true; i += NR_RING_VECTORS; }
    let i = NON_RING_MSIX_BASE; let qvec = (*ndev).qvec.add(i as usize); (*qvec).ndev = ndev; snprintf((*qvec).name.as_mut_ptr(), IRQ_NAMESZ, "nitrox-core-int%d", i); let vec = pci_irq_vector(pdev, i); let ret = request_irq(vec, nps_core_int_isr, 0, (*qvec).name.as_ptr(), qvec as *mut core::ffi::c_void); if ret != 0 { nitrox_unregister_interrupts(ndev); return ret; } let cpu = num_online_cpus(); irq_set_affinity_hint(vec, get_cpu_mask(cpu)); tasklet_init(&mut (*qvec).resp_tasklet, nps_core_int_tasklet, qvec as usize); (*qvec).valid = true; 0
}

// SR-IOV interrupt routines mirror the C implementation and use the same
// externally supplied PCI, IRQ, tasklet, and allocation interfaces.
pub unsafe fn nitrox_sriov_unregister_interrupts(ndev: *mut nitrox_device) { let pdev = (*ndev).pdev; for i in 0..(*ndev).num_vecs { let qvec = (*ndev).qvec.add(i as usize); if !(*qvec).valid { continue; } let vec = (*ndev).iov.msix.vector; irq_set_affinity_hint(vec, core::ptr::null_mut()); free_irq(vec, qvec as *mut core::ffi::c_void); tasklet_disable(&mut (*qvec).resp_tasklet); tasklet_kill(&mut (*qvec).resp_tasklet); (*qvec).valid = false; } kfree((*ndev).qvec as *mut core::ffi::c_void); (*ndev).qvec = core::ptr::null_mut(); pci_disable_msix(pdev); }

pub unsafe fn nitrox_sriov_register_interupts(ndev: *mut nitrox_device) -> i32 { let pdev = (*ndev).pdev; (*ndev).iov.msix.entry = NON_RING_MSIX_BASE; let ret = pci_enable_msix_exact(pdev, &mut (*ndev).iov.msix, NR_NON_RING_VECTORS); if ret != 0 { return ret; } let qvec = kzalloc_objs(NR_NON_RING_VECTORS); if qvec.is_null() { pci_disable_msix(pdev); return -ENOMEM; } (*qvec).ndev = ndev; (*ndev).qvec = qvec; (*ndev).num_vecs = NR_NON_RING_VECTORS; snprintf((*qvec).name.as_mut_ptr(), IRQ_NAMESZ, "nitrox-core-int%d", NON_RING_MSIX_BASE); let vec = (*ndev).iov.msix.vector; let ret = request_irq(vec, nps_core_int_isr, 0, (*qvec).name.as_ptr(), qvec as *mut core::ffi::c_void); if ret != 0 { nitrox_sriov_unregister_interrupts(ndev); return ret; } let cpu = num_online_cpus(); irq_set_affinity_hint(vec, get_cpu_mask(cpu)); tasklet_init(&mut (*qvec).resp_tasklet, nps_core_int_tasklet, qvec as usize); (*qvec).valid = true; 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
