// SPDX-License-Identifier: GPL-2.0
/*
 * Interrupt controller for the
 * Communication Processor Module.
 * Copyright (c) 1997 Dan error_act (dmalek@jlc.net)
 */
// Dependencies supplied by the kernel and architecture headers are external.

#[repr(C)]
struct CpmPicData {
    reg: *mut Cpic8xx,
    host: *mut IrqDomain,
}

unsafe fn cpm_mask_irq(d: *mut IrqData) {
    let data = irq_data_get_irq_chip_data(d);
    let cpm_vec = irqd_to_hwirq(d) as u32;
    clrbits32(unsafe { &mut (*(*data).reg).cpic_cimr }, 1u32 << cpm_vec);
}

unsafe fn cpm_unmask_irq(d: *mut IrqData) {
    let data = irq_data_get_irq_chip_data(d);
    let cpm_vec = irqd_to_hwirq(d) as u32;
    setbits32(unsafe { &mut (*(*data).reg).cpic_cimr }, 1u32 << cpm_vec);
}

unsafe fn cpm_end_irq(d: *mut IrqData) {
    let data = irq_data_get_irq_chip_data(d);
    let cpm_vec = irqd_to_hwirq(d) as u32;
    out_be32(unsafe { &mut (*(*data).reg).cpic_cisr }, 1u32 << cpm_vec);
}

static mut CPM_PIC: IrqChip = IrqChip {
    name: b"CPM PIC\0".as_ptr() as *const i8,
    irq_mask: Some(cpm_mask_irq),
    irq_unmask: Some(cpm_unmask_irq),
    irq_eoi: Some(cpm_end_irq),
};

unsafe fn cpm_get_irq(desc: *mut IrqDesc) -> i32 {
    let data = irq_desc_get_handler_data(desc);
    let mut cpm_vec: i32;

    /*
     * Get the vector by setting the ACK bit and then reading
     * the register.
     */
    out_be16(&mut (*(*data).reg).cpic_civr, 1);
    cpm_vec = in_be16(&(*(*data).reg).cpic_civr) as i32;
    cpm_vec >>= 11;

    irq_find_mapping((*data).host, cpm_vec as u32)
}

unsafe fn cpm_cascade(desc: *mut IrqDesc) {
    generic_handle_irq(cpm_get_irq(desc) as u32);
}

unsafe fn cpm_pic_host_map(h: *mut IrqDomain, virq: u32, _hw: IrqHwNumber) -> i32 {
    irq_set_chip_data(virq, (*h).host_data);
    irq_set_status_flags(virq, IRQ_LEVEL);
    irq_set_chip_and_handler(virq, &mut CPM_PIC, handle_fasteoi_irq);
    0
}

static CPM_PIC_HOST_OPS: IrqDomainOps = IrqDomainOps { map: Some(cpm_pic_host_map) };

unsafe fn cpm_pic_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev;
    let mut res: *mut Resource;
    let irq: i32;
    let data: *mut CpmPicData;

    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if res.is_null() {
        return -ENODEV;
    }

    data = devm_kzalloc(dev, core::mem::size_of::<CpmPicData>(), GFP_KERNEL) as *mut CpmPicData;
    if data.is_null() {
        return -ENOMEM;
    }

    (*data).reg = devm_ioremap(dev, (*res).start, resource_size(res)) as *mut Cpic8xx;
    if (*data).reg.is_null() {
        return -ENODEV;
    }

    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    /* Initialize the CPM interrupt controller. */
    out_be32(&mut (*(*data).reg).cpic_cicr,
        (CICR_SCD_SCC4 | CICR_SCC_SCC3 | CICR_SCB_SCC2 | CICR_SCA_SCC1)
            | ((virq_to_hw(irq as u32) / 2) << 13) | CICR_HP_MASK);
    out_be32(&mut (*(*data).reg).cpic_cimr, 0);

    (*data).host = irq_domain_create_linear(dev_fwnode(dev), 64, &CPM_PIC_HOST_OPS, data);
    if (*data).host.is_null() {
        return -ENODEV;
    }

    irq_set_handler_data(irq as u32, data);
    irq_set_chained_handler(irq as u32, cpm_cascade);
    setbits32(&mut (*(*data).reg).cpic_cicr, CICR_IEN);
    0
}

static CPM_PIC_MATCH: [OfDeviceId; 3] = [
    OfDeviceId { compatible: b"fsl,cpm1-pic\0".as_ptr() as *const i8, ..OfDeviceId::default() },
    OfDeviceId { type_: b"cpm-pic\0".as_ptr() as *const i8, compatible: b"CPM\0".as_ptr() as *const i8, ..OfDeviceId::default() },
    OfDeviceId::default(),
];

static mut CPM_PIC_DRIVER: PlatformDriver = PlatformDriver {
    driver: DeviceDriver { name: b"cpm-pic\0".as_ptr() as *const i8, of_match_table: CPM_PIC_MATCH.as_ptr() },
    probe: Some(cpm_pic_probe),
};

unsafe fn cpm_pic_init() -> i32 { platform_driver_register(&mut CPM_PIC_DRIVER) }
// arch_initcall(cpm_pic_init)

/*
 * The CPM can generate the error interrupt when there is a race condition
 * between generating and masking interrupts.  All we have to do is ACK it
 * and return.  This is a no-op function so we don't need any special
 * tests in the interrupt handler.
 */
unsafe fn cpm_error_interrupt(_irq: i32, _dev: *mut core::ffi::c_void) -> IrqReturn {
    IRQ_HANDLED
}

unsafe fn cpm_error_probe(pdev: *mut PlatformDevice) -> i32 {
    let irq = platform_get_irq(pdev, 0);
    if irq < 0 { return irq; }
    request_irq(irq as u32, Some(cpm_error_interrupt), IRQF_NO_THREAD, b"error\0".as_ptr() as *const i8, core::ptr::null_mut())
}

static CPM_ERROR_IDS: [OfDeviceId; 3] = [
    OfDeviceId { compatible: b"fsl,cpm1\0".as_ptr() as *const i8, ..OfDeviceId::default() },
    OfDeviceId { type_: b"cpm\0".as_ptr() as *const i8, ..OfDeviceId::default() },
    OfDeviceId::default(),
];

static mut CPM_ERROR_DRIVER: PlatformDriver = PlatformDriver {
    driver: DeviceDriver { name: b"cpm-error\0".as_ptr() as *const i8, of_match_table: CPM_ERROR_IDS.as_ptr() },
    probe: Some(cpm_error_probe),
};

unsafe fn cpm_error_init() -> i32 { platform_driver_register(&mut CPM_ERROR_DRIVER) }
// subsys_initcall(cpm_error_init)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
