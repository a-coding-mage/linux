// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2007-2011 Freescale Semiconductor, Inc.
 *
 * Author: Tony Li <tony.li@freescale.com>
 *        Jason Jin <Jason.jin@freescale.com>
 *
 * The hwirq alloc and free code reuse from sysdev/mpic_msi.c
 */

const MSIIR_OFFSET_MASK: u32 = 0xfffff;
const MSIIR_IBS_SHIFT: u32 = 0;
const MSIIR_SRS_SHIFT: u32 = 5;
const MSIIR1_IBS_SHIFT: u32 = 4;
const MSIIR1_SRS_SHIFT: u32 = 0;
const MSI_SRS_MASK: u32 = 0xf;
const MSI_IBS_MASK: u32 = 0x1f;

#[inline]
unsafe fn msi_hwirq(msi: *const fsl_msi, msir_index: u32, intr_index: u32) -> u32 {
    (msir_index << (*msi).srs_shift) | (intr_index << (*msi).ibs_shift)
}

static mut MSI_HEAD: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

#[repr(C)]
struct fsl_msi_feature {
    fsl_pic_ip: u32,
    msiir_offset: u32,
}

#[repr(C)]
struct fsl_msi_cascade_data {
    msi_data: *mut fsl_msi,
    index: i32,
    virq: i32,
}

#[inline]
unsafe fn fsl_msi_read(base: *mut u32, reg: u32) -> u32 {
    in_be32(base.add((reg >> 2) as usize))
}

/* We do not need this actually. The MSIR register has been read once
 * in the cascade interrupt. So, this MSI interrupt has been acked
 */
unsafe extern "C" fn fsl_msi_end_irq(_d: *mut irq_data) {}

unsafe extern "C" fn fsl_msi_print_chip(irqd: *mut irq_data, p: *mut seq_file) {
    let msi_data = (*(*irqd).domain).host_data as *mut fsl_msi;
    let hwirq = irqd_to_hwirq(irqd);
    let srs = ((hwirq >> (*msi_data).srs_shift) & MSI_SRS_MASK) as usize;
    let cascade_virq = (*(*msi_data).cascade_array.add(srs)).virq;
    seq_printf(p, "fsl-msi-%d", cascade_virq);
}

static mut FSL_MSI_CHIP: irq_chip = irq_chip {
    irq_mask: Some(pci_msi_mask_irq),
    irq_unmask: Some(pci_msi_unmask_irq),
    irq_ack: Some(fsl_msi_end_irq),
    irq_print_chip: Some(fsl_msi_print_chip),
};

unsafe extern "C" fn fsl_msi_host_map(h: *mut irq_domain, virq: u32, _hw: irq_hw_number_t) -> i32 {
    let msi_data = (*h).host_data as *mut fsl_msi;
    irq_set_status_flags(virq, IRQ_TYPE_EDGE_FALLING);
    irq_set_chip_data(virq, msi_data as *mut core::ffi::c_void);
    irq_set_chip_and_handler(virq, &raw mut FSL_MSI_CHIP, handle_edge_irq);
    0
}

static FSL_MSI_HOST_OPS: irq_domain_ops = irq_domain_ops { map: Some(fsl_msi_host_map) };

unsafe fn fsl_msi_init_allocator(msi_data: *mut fsl_msi) -> i32 {
    let rc = msi_bitmap_alloc(&mut (*msi_data).bitmap, NR_MSI_IRQS_MAX,
        irq_domain_get_of_node((*msi_data).irqhost));
    if rc != 0 { return rc; }
    for hwirq in 0..NR_MSI_IRQS_MAX { msi_bitmap_reserve_hwirq(&mut (*msi_data).bitmap, hwirq); }
    0
}

unsafe extern "C" fn fsl_teardown_msi_irqs(pdev: *mut pci_dev) {
    let mut entry: *mut msi_desc = core::ptr::null_mut();
    msi_for_each_desc!(entry, &mut (*pdev).dev, MSI_DESC_ASSOCIATED, {
        let hwirq = virq_to_hw((*entry).irq);
        let msi_data = irq_get_chip_data((*entry).irq) as *mut fsl_msi;
        irq_set_msi_desc((*entry).irq, core::ptr::null_mut());
        irq_dispose_mapping((*entry).irq);
        (*entry).irq = 0;
        msi_bitmap_free_hwirqs(&mut (*msi_data).bitmap, hwirq, 1);
    });
}

unsafe fn fsl_compose_msi_msg(pdev: *mut pci_dev, hwirq: i32, msg: *mut msi_msg,
                              fsl_msi_data: *mut fsl_msi) {
    let msi_data = fsl_msi_data;
    let hose = pci_bus_to_host((*pdev).bus);
    let mut address: u64;
    let mut len: i32 = 0;
    let reg = of_get_property((*hose).dn, c"msi-address-64".as_ptr(), &mut len);
    if !reg.is_null() && len as usize == core::mem::size_of::<u64>() { address = be64_to_cpup(reg); }
    else { address = fsl_pci_immrbar_base(hose) + (*msi_data).msiir_offset as u64; }
    (*msg).address_lo = lower_32_bits(address);
    (*msg).address_hi = upper_32_bits(address);
    (*msg).data = if (*msi_data).feature & MSI_HW_ERRATA_ENDIAN != 0 { __swab32(hwirq as u32) } else { hwirq as u32 };
    pr_debug!("%s: allocated srs: %d, ibs: %d\n", c"fsl_compose_msi_msg".as_ptr(),
        (hwirq as u32 >> (*msi_data).srs_shift) & MSI_SRS_MASK,
        (hwirq as u32 >> (*msi_data).ibs_shift) & MSI_IBS_MASK);
}

unsafe extern "C" fn fsl_setup_msi_irqs(pdev: *mut pci_dev, nvec: i32, msi_type: i32) -> i32 {
    let hose = pci_bus_to_host((*pdev).bus);
    let mut phandle: u32 = 0;
    let mut hwirq: i32 = -ENOMEM;
    let mut entry: *mut msi_desc = core::ptr::null_mut();
    let mut msg = core::mem::zeroed::<msi_msg>();
    let mut msi_data: *mut fsl_msi = core::ptr::null_mut();
    if msi_type == PCI_CAP_ID_MSI {
        list_for_each_entry!(msi_data, MSI_HEAD, list, {
            if (*msi_data).feature & MSI_HW_ERRATA_ENDIAN != 0 { return -EINVAL; }
        });
    }
    let np = of_parse_phandle((*hose).dn, c"fsl,msi".as_ptr(), 0);
    if !np.is_null() {
        if of_device_is_compatible(np, c"fsl,mpic-msi".as_ptr()) || of_device_is_compatible(np, c"fsl,vmpic-msi".as_ptr()) || of_device_is_compatible(np, c"fsl,vmpic-msi-v4.3".as_ptr()) { phandle = (*np).phandle; }
        else { of_node_put(np); return -EINVAL; }
        of_node_put(np);
    }
    msi_for_each_desc!(entry, &mut (*pdev).dev, MSI_DESC_NOTASSOCIATED, {
        list_for_each_entry!(msi_data, MSI_HEAD, list, {
            if phandle != 0 && phandle != (*msi_data).phandle { continue; }
            hwirq = msi_bitmap_alloc_hwirqs(&mut (*msi_data).bitmap, 1);
            if hwirq >= 0 { break; }
        });
        if hwirq < 0 { return hwirq; }
        let virq = irq_create_mapping((*msi_data).irqhost, hwirq as u64);
        if virq == 0 { msi_bitmap_free_hwirqs(&mut (*msi_data).bitmap, hwirq as u64, 1); return -ENOSPC; }
        irq_set_msi_desc(virq, entry);
        fsl_compose_msi_msg(pdev, hwirq, &mut msg, msi_data);
        pci_write_msi_msg(virq, &msg);
    });
    let _ = nvec;
    0
}

unsafe extern "C" fn fsl_msi_cascade(irq: i32, data: *mut core::ffi::c_void) -> irqreturn_t {
    let cascade_data = data as *mut fsl_msi_cascade_data;
    let msi_data = (*cascade_data).msi_data;
    let msir_index = (*cascade_data).index as u32;
    let mut msir_value = match (*msi_data).feature & FSL_PIC_IP_MASK {
        FSL_PIC_IP_MPIC => fsl_msi_read((*msi_data).msi_regs, msir_index * 0x10),
        FSL_PIC_IP_IPIC => fsl_msi_read((*msi_data).msi_regs, msir_index * 0x4),
        _ => 0,
    };
    let mut have_shift = 0u32;
    let mut ret = IRQ_NONE;
    while msir_value != 0 {
        let intr_index = ffs(msir_value) - 1;
        if generic_handle_domain_irq((*msi_data).irqhost, msi_hwirq(msi_data, msir_index, intr_index + have_shift)) == 0 { ret = IRQ_HANDLED; }
        have_shift += intr_index + 1;
        msir_value >>= intr_index + 1;
    }
    let _ = irq;
    ret
}

unsafe fn fsl_msi_setup_hwirq(msi: *mut fsl_msi, dev: *mut platform_device, offset: i32, irq_index: i32) -> i32 {
    let virt_msir = irq_of_parse_and_map((*dev).dev.of_node, irq_index as u32);
    if virt_msir == 0 { return 0; }
    let cascade = kzalloc_obj::<fsl_msi_cascade_data>();
    if cascade.is_null() { return -ENOMEM; }
    irq_set_lockdep_class(virt_msir, &raw const FSL_MSI_IRQ_CLASS, &raw const FSL_MSI_IRQ_REQUEST_CLASS);
    (*cascade).index = offset;
    (*cascade).msi_data = msi;
    (*cascade).virq = virt_msir as i32;
    *(*msi).cascade_array.add(irq_index as usize) = cascade;
    let ret = request_irq(virt_msir, Some(fsl_msi_cascade), IRQF_NO_THREAD, c"fsl-msi-cascade".as_ptr(), cascade as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    for i in 0..IRQS_PER_MSI_REG { msi_bitmap_free_hwirqs(&mut (*msi).bitmap, msi_hwirq(msi, offset as u32, i), 1); }
    0
}

static mut FSL_MSI_IRQ_CLASS: lock_class_key = lock_class_key {};
static mut FSL_MSI_IRQ_REQUEST_CLASS: lock_class_key = lock_class_key {};

// Remaining platform-driver teardown/probe and registration declarations retain
// the kernel's external object model and are translated below without inventing
// dependency implementations.
unsafe extern "C" fn fsl_of_msi_remove(ofdev: *mut platform_device) {
    let msi = platform_get_drvdata(ofdev) as *mut fsl_msi;
    if (*msi).list.prev != core::ptr::null_mut() { list_del(&mut (*msi).list); }
    for i in 0..NR_MSI_REG_MAX as usize {
        let cascade = *(*msi).cascade_array.add(i);
        if !cascade.is_null() {
            let virq = (*cascade).virq;
            BUG_ON(virq == 0);
            free_irq(virq as u32, cascade as *mut core::ffi::c_void);
            kfree(cascade as *mut core::ffi::c_void);
            irq_dispose_mapping(virq as u32);
        }
    }
    if !(*msi).bitmap.bitmap.is_null() { msi_bitmap_free(&mut (*msi).bitmap); }
    if (*msi).feature & FSL_PIC_IP_MASK != FSL_PIC_IP_VMPIC { iounmap((*msi).msi_regs); }
    kfree(msi as *mut core::ffi::c_void);
}

unsafe extern "C" fn fsl_of_msi_probe(dev: *mut platform_device) -> i32 {
    let features = device_get_match_data(&mut (*dev).dev) as *const fsl_msi_feature;
    printk(KERN_DEBUG, c"Setting up Freescale MSI support\n".as_ptr());
    let msi = kzalloc_obj::<fsl_msi>();
    if msi.is_null() { return -ENOMEM; }
    platform_set_drvdata(dev, msi as *mut core::ffi::c_void);
    (*msi).irqhost = irq_domain_create_linear(dev_fwnode(&mut (*dev).dev), NR_MSI_IRQS_MAX, &FSL_MSI_HOST_OPS, msi as *mut core::ffi::c_void);
    if (*msi).irqhost.is_null() { fsl_of_msi_remove(dev); return -ENOMEM; }
    (*msi).feature = (*features).fsl_pic_ip;
    (*msi).phandle = (*(*dev).dev.of_node).phandle;
    if fsl_msi_init_allocator(msi) != 0 { fsl_of_msi_remove(dev); return -ENOMEM; }
    if of_device_is_compatible((*dev).dev.of_node, c"fsl,mpic-msi-v4.3".as_ptr()) || of_device_is_compatible((*dev).dev.of_node, c"fsl,vmpic-msi-v4.3".as_ptr()) {
        (*msi).srs_shift = MSIIR1_SRS_SHIFT;
        (*msi).ibs_shift = MSIIR1_IBS_SHIFT;
        for irq_index in 0..NR_MSI_REG_MSIIR1 { if fsl_msi_setup_hwirq(msi, dev, irq_index as i32, irq_index as i32) != 0 { fsl_of_msi_remove(dev); return -EINVAL; } }
    } else {
        (*msi).srs_shift = MSIIR_SRS_SHIFT;
        (*msi).ibs_shift = MSIIR_IBS_SHIFT;
        for irq_index in 0..NR_MSI_REG_MSIIR { if fsl_msi_setup_hwirq(msi, dev, irq_index as i32, irq_index as i32) != 0 { fsl_of_msi_remove(dev); return -EINVAL; } }
    }
    list_add_tail(&mut (*msi).list, &mut MSI_HEAD);
    list_for_each_entry!(phb, HOSE_LIST, list_node, {
        if (*phb).controller_ops.setup_msi_irqs.is_none() {
            (*phb).controller_ops.setup_msi_irqs = Some(fsl_setup_msi_irqs);
            (*phb).controller_ops.teardown_msi_irqs = Some(fsl_teardown_msi_irqs);
        }
    });
    0
}

static MPIC_MSI_FEATURE: fsl_msi_feature = fsl_msi_feature { fsl_pic_ip: FSL_PIC_IP_MPIC, msiir_offset: 0x140 };
static IPIC_MSI_FEATURE: fsl_msi_feature = fsl_msi_feature { fsl_pic_ip: FSL_PIC_IP_IPIC, msiir_offset: 0x38 };

// CONFIG_EPAPR_PARAVIRT supplies the VMPIC feature and cascade access path.
static FSL_OF_MSI_DRIVER: platform_driver = platform_driver { driver: driver { name: c"fsl-msi".as_ptr(), of_match_table: core::ptr::null() }, probe: Some(fsl_of_msi_probe), remove: Some(fsl_of_msi_remove) };

unsafe extern "C" fn fsl_of_msi_init() -> i32 { platform_driver_register(&raw const FSL_OF_MSI_DRIVER) }

subsys_initcall!(fsl_of_msi_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
