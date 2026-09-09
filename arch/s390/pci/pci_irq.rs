// SPDX-License-Identifier: GPL-2.0
// Translated from pci_irq.c; Linux includes and build-time definitions are supplied externally.

static mut irq_delivery: i32 = 0; // FLOATING = 0, DIRECTED = 1
static mut zpci_sbv: *mut airq_iv = core::ptr::null_mut();
static mut zpci_ibv: *mut *mut airq_iv = core::ptr::null_mut();

unsafe fn zpci_set_airq(zdev: *mut zpci_dev) -> i32 {
    let req: u64 = ZPCI_CREATE_REQ((*zdev).fh, 0, ZPCI_MOD_FC_REG_INT);
    let mut fib: zpci_fib = core::mem::zeroed();
    let mut status: u8 = 0;
    fib.fmt0.isc = PCI_ISC;
    fib.fmt0.sum = 1;
    fib.fmt0.noi = airq_iv_end((*zdev).aibv);
    fib.fmt0.aibv = virt_to_phys((*zdev).aibv.vector);
    fib.fmt0.aibvo = 0;
    fib.fmt0.aisb = virt_to_phys((*zpci_sbv).vector) + ((*zdev).aisb / 64) * 8;
    fib.fmt0.aisbo = (*zdev).aisb & 63;
    fib.gd = (*zdev).gisa;
    if zpci_mod_fc(req, &mut fib, &mut status) != 0 { -EIO } else { 0 }
}

unsafe fn zpci_clear_airq(zdev: *mut zpci_dev) -> i32 {
    let req: u64 = ZPCI_CREATE_REQ((*zdev).fh, 0, ZPCI_MOD_FC_DEREG_INT);
    let mut fib: zpci_fib = core::mem::zeroed();
    let mut cc: u8 = 0;
    let mut status: u8 = 0;
    fib.gd = (*zdev).gisa;
    cc = zpci_mod_fc(req, &mut fib, &mut status);
    if cc == 3 || (cc == 1 && status == 24) { cc = 0; }
    if cc != 0 { -EIO } else { 0 }
}

unsafe fn zpci_set_directed_irq(zdev: *mut zpci_dev) -> i32 {
    let req: u64 = ZPCI_CREATE_REQ((*zdev).fh, 0, ZPCI_MOD_FC_REG_INT_D);
    let mut fib: zpci_fib = core::mem::zeroed();
    let mut status: u8 = 0;
    fib.fmt = 1;
    fib.fmt1.noi = (*zdev).msi_nr_irqs;
    fib.fmt1.dibvo = (*zdev).msi_first_bit;
    fib.gd = (*zdev).gisa;
    if zpci_mod_fc(req, &mut fib, &mut status) != 0 { -EIO } else { 0 }
}

unsafe fn zpci_clear_directed_irq(zdev: *mut zpci_dev) -> i32 {
    let req: u64 = ZPCI_CREATE_REQ((*zdev).fh, 0, ZPCI_MOD_FC_DEREG_INT_D);
    let mut fib: zpci_fib = core::mem::zeroed();
    let mut cc: u8;
    let mut status: u8 = 0;
    fib.fmt = 1;
    fib.gd = (*zdev).gisa;
    cc = zpci_mod_fc(req, &mut fib, &mut status);
    if cc == 3 || (cc == 1 && status == 24) { cc = 0; }
    if cc != 0 { -EIO } else { 0 }
}

pub unsafe fn zpci_set_irq(zdev: *mut zpci_dev) -> i32 {
    if irq_delivery == DIRECTED { zpci_set_directed_irq(zdev) } else { zpci_set_airq(zdev) }
}

unsafe fn zpci_clear_irq(zdev: *mut zpci_dev) -> i32 {
    if irq_delivery == DIRECTED { zpci_clear_directed_irq(zdev) } else { zpci_clear_airq(zdev) }
}

unsafe fn zpci_set_irq_affinity(data: *mut irq_data, dest: *const cpumask, _force: bool) -> i32 {
    irq_data_update_affinity(data, dest);
    IRQ_SET_MASK_OK
}

#[inline]
unsafe fn zpci_encode_hwirq(devfn: u8, msi_index: u16) -> u32 { ((devfn as u32) << 16) | msi_index as u32 }

#[inline]
unsafe fn zpci_decode_hwirq_msi_index(hwirq: irq_hw_number_t) -> u16 { (hwirq & 0xffff) as u16 }

unsafe fn zpci_compose_msi_msg(data: *mut irq_data, msg: *mut msi_msg) {
    let desc = irq_data_get_msi_desc(data);
    let zdev = to_zpci_dev((*desc).dev);
    if irq_delivery == DIRECTED {
        let cpu = cpumask_first(irq_data_get_affinity_mask(data));
        (*msg).address_lo = (*zdev).msi_addr & 0xff0000ff;
        (*msg).address_lo |= smp_cpu_get_cpu_address(cpu) << 8;
    } else { (*msg).address_lo = (*zdev).msi_addr & 0xffffffff; }
    (*msg).address_hi = (*zdev).msi_addr >> 32;
    (*msg).data = zpci_decode_hwirq_msi_index((*data).hwirq) as u32;
}

static mut zpci_irq_chip: irq_chip = irq_chip { name: b"PCI-MSI\0".as_ptr(), irq_compose_msi_msg: Some(zpci_compose_msi_msg), ..unsafe { core::mem::zeroed() } };

unsafe fn zpci_handle_cpu_local_irq(rescan: bool) {
    let dibv = *zpci_ibv.add(smp_processor_id());
    let mut iib: zpci_sic_iib = core::mem::zeroed();
    let mut irqs_on = 0;
    let mut bit = 0usize;
    loop {
        bit = airq_iv_scan(dibv, bit, airq_iv_end(dibv));
        if bit == usize::MAX {
            if !rescan || irqs_on > 0 { break; }
            irqs_on += 1;
            if zpci_set_irq_ctrl(SIC_IRQ_MODE_D_SINGLE, PCI_ISC, &mut iib) != 0 { break; }
            bit = 0; continue;
        }
        inc_irq_stat(IRQIO_MSI);
        let hwirq = airq_iv_get_data(dibv, bit);
        let msi_domain = airq_iv_get_ptr(dibv, bit) as *mut irq_domain;
        generic_handle_domain_irq(msi_domain, hwirq);
    }
}

#[repr(C)]
struct cpu_irq_data { csd: call_single_data_t, scheduled: atomic_t }
static mut irq_data: cpu_irq_data = unsafe { core::mem::zeroed() };

unsafe fn zpci_handle_remote_irq(data: *mut core::ffi::c_void) {
    let scheduled = data as *mut atomic_t;
    loop { zpci_handle_cpu_local_irq(false); if atomic_dec_return(scheduled) == 0 { break; } }
}

unsafe fn zpci_handle_fallback_irq() {
    let mut iib: zpci_sic_iib = core::mem::zeroed();
    let mut cpu = 0usize; let mut irqs_on = 0;
    loop {
        cpu = airq_iv_scan(zpci_sbv, cpu, airq_iv_end(zpci_sbv));
        if cpu == usize::MAX {
            if irqs_on > 0 { break; } irqs_on += 1;
            if zpci_set_irq_ctrl(SIC_IRQ_MODE_SINGLE, PCI_ISC, &mut iib) != 0 { break; }
            cpu = 0; continue;
        }
        let cpu_data = &mut irq_data;
        if atomic_inc_return(&mut cpu_data.scheduled) > 1 { continue; }
        INIT_CSD(&mut cpu_data.csd, zpci_handle_remote_irq, &mut cpu_data.scheduled as *mut _ as *mut _);
        smp_call_function_single_async(cpu, &mut cpu_data.csd);
    }
}

unsafe fn zpci_directed_irq_handler(_airq: *mut airq_struct, tpi_info: *mut tpi_info) {
    if !(*tpi_info).directed_irq { inc_irq_stat(IRQIO_PCF); zpci_handle_fallback_irq(); }
    else { inc_irq_stat(IRQIO_PCD); zpci_handle_cpu_local_irq(true); }
}

unsafe fn zpci_floating_irq_handler(_airq: *mut airq_struct, _tpi_info: *mut tpi_info) {
    let mut iib: zpci_sic_iib = core::mem::zeroed();
    let mut si = 0usize; let mut irqs_on = 0;
    inc_irq_stat(IRQIO_PCF);
    loop {
        si = airq_iv_scan(zpci_sbv, si, airq_iv_end(zpci_sbv));
        if si == usize::MAX {
            if irqs_on > 0 { break; } irqs_on += 1;
            if zpci_set_irq_ctrl(SIC_IRQ_MODE_SINGLE, PCI_ISC, &mut iib) != 0 { break; }
            si = 0; continue;
        }
        let aibv = *zpci_ibv.add(si); let mut ai = 0usize;
        loop {
            ai = airq_iv_scan(aibv, ai, airq_iv_end(aibv));
            if ai == usize::MAX { break; }
            inc_irq_stat(IRQIO_MSI); airq_iv_lock(aibv, ai);
            let hwirq = airq_iv_get_data(aibv, ai);
            let domain = airq_iv_get_ptr(aibv, ai) as *mut irq_domain;
            generic_handle_domain_irq(domain, hwirq); airq_iv_unlock(aibv, ai);
        }
    }
}

unsafe fn __alloc_airq(zdev: *mut zpci_dev, msi_vecs: i32, bit: *mut usize) -> i32 {
    if irq_delivery == DIRECTED { *bit = airq_iv_alloc(*zpci_ibv, msi_vecs); if *bit == usize::MAX { return -EIO; } }
    else {
        *bit = airq_iv_alloc_bit(zpci_sbv); if *bit == usize::MAX { return -EIO; }
        (*zdev).aisb = *bit; (*zdev).aibv = airq_iv_create(msi_vecs, AIRQ_IV_PTR | AIRQ_IV_DATA | AIRQ_IV_BITLOCK, core::ptr::null_mut());
        if (*zdev).aibv.is_null() { return -ENOMEM; } *zpci_ibv.add(*bit) = (*zdev).aibv; *bit = 0;
    } 0
}

pub unsafe fn arch_restore_msi_irqs(pdev: *mut pci_dev) -> bool { zpci_set_irq(to_zpci(pdev)); true }

unsafe fn zpci_msi_teardown_directed(zdev: *mut zpci_dev) { airq_iv_free(*zpci_ibv, (*zdev).msi_first_bit, (*zdev).max_msi); (*zdev).msi_first_bit = !0; (*zdev).msi_nr_irqs = 0; }
unsafe fn zpci_msi_teardown_floating(zdev: *mut zpci_dev) { airq_iv_release((*zdev).aibv); (*zdev).aibv = core::ptr::null_mut(); airq_iv_free_bit(zpci_sbv, (*zdev).aisb); (*zdev).aisb = usize::MAX; (*zdev).msi_first_bit = !0; (*zdev).msi_nr_irqs = 0; }

pub unsafe fn zpci_create_parent_msi_domain(_zbus: *mut zpci_bus) -> i32 { 0 }
pub unsafe fn zpci_remove_parent_msi_domain(_zbus: *mut zpci_bus) {}
pub unsafe fn zpci_irq_init() -> i32 { 0 }
pub unsafe fn zpci_irq_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
