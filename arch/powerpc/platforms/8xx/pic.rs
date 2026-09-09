// Translated from powerpc/platforms/8xx/pic.c.
// C header dependencies are supplied by the surrounding kernel translation.

const PIC_VEC_SPURRIOUS: u32 = 15;

static mut mpc8xx_pic_host: *mut irq_domain = core::ptr::null_mut();
static mut mpc8xx_cached_irq_mask: libc::c_ulong = 0;
static mut siu_reg: *mut sysconf8xx_t = core::ptr::null_mut();

#[inline]
unsafe fn mpc8xx_irqd_to_bit(d: *mut irq_data) -> libc::c_ulong {
    0x80000000u32 as libc::c_ulong >> irqd_to_hwirq(d)
}

unsafe fn mpc8xx_unmask_irq(d: *mut irq_data) {
    mpc8xx_cached_irq_mask |= mpc8xx_irqd_to_bit(d);
    out_be32(&mut (*siu_reg).sc_simask, mpc8xx_cached_irq_mask);
}

unsafe fn mpc8xx_mask_irq(d: *mut irq_data) {
    mpc8xx_cached_irq_mask &= !mpc8xx_irqd_to_bit(d);
    out_be32(&mut (*siu_reg).sc_simask, mpc8xx_cached_irq_mask);
}

unsafe fn mpc8xx_ack(d: *mut irq_data) {
    out_be32(&mut (*siu_reg).sc_sipend, mpc8xx_irqd_to_bit(d));
}

unsafe fn mpc8xx_end_irq(d: *mut irq_data) {
    mpc8xx_cached_irq_mask |= mpc8xx_irqd_to_bit(d);
    out_be32(&mut (*siu_reg).sc_simask, mpc8xx_cached_irq_mask);
}

unsafe fn mpc8xx_set_irq_type(d: *mut irq_data, flow_type: libc::c_uint) -> libc::c_int {
    /* only external IRQ senses are programmable */
    if (flow_type & IRQ_TYPE_EDGE_FALLING) != 0 && (irqd_to_hwirq(d) & 1) == 0 {
        let mut siel = in_be32(&(*siu_reg).sc_siel);
        siel |= mpc8xx_irqd_to_bit(d);
        out_be32(&mut (*siu_reg).sc_siel, siel);
        irq_set_handler_locked(d, handle_edge_irq);
    }
    0
}

static mut mpc8xx_pic: irq_chip = irq_chip {
    name: b"8XX SIU\0".as_ptr() as *const libc::c_char,
    irq_unmask: Some(mpc8xx_unmask_irq),
    irq_mask: Some(mpc8xx_mask_irq),
    irq_ack: Some(mpc8xx_ack),
    irq_eoi: Some(mpc8xx_end_irq),
    irq_set_type: Some(mpc8xx_set_irq_type),
};

pub unsafe fn mpc8xx_get_irq() -> libc::c_uint {
    /* For MPC8xx, read the SIVEC register and shift the bits down
     * to get the irq number.
     */
    let irq = in_be32(&(*siu_reg).sc_sivec) >> 26;

    if irq == PIC_VEC_SPURRIOUS {
        return 0;
    }

    irq_find_mapping(mpc8xx_pic_host, irq)
}

unsafe fn mpc8xx_pic_host_map(
    _h: *mut irq_domain,
    virq: libc::c_uint,
    hw: irq_hw_number_t,
) -> libc::c_int {
    pr_debug!("mpc8xx_pic_host_map({}, 0x{:lx})\n", virq, hw);

    /* Set default irq handle */
    irq_set_chip_and_handler(virq, &mut mpc8xx_pic, handle_level_irq);
    0
}

unsafe fn mpc8xx_pic_host_xlate(
    _h: *mut irq_domain,
    _ct: *mut device_node,
    intspec: *const u32,
    intsize: libc::c_uint,
    out_hwirq: *mut irq_hw_number_t,
    out_flags: *mut libc::c_uint,
) -> libc::c_int {
    static map_pic_senses: [libc::c_uint; 4] = [
        IRQ_TYPE_EDGE_RISING,
        IRQ_TYPE_LEVEL_LOW,
        IRQ_TYPE_LEVEL_HIGH,
        IRQ_TYPE_EDGE_FALLING,
    ];

    if *intspec > 0x1f {
        return 0;
    }

    *out_hwirq = *intspec as irq_hw_number_t;
    if intsize > 1 && *intspec.add(1) < 4 {
        *out_flags = map_pic_senses[*intspec.add(1) as usize];
    } else {
        *out_flags = IRQ_TYPE_NONE;
    }

    0
}

static mpc8xx_pic_host_ops: irq_domain_ops = irq_domain_ops {
    map: Some(mpc8xx_pic_host_map),
    xlate: Some(mpc8xx_pic_host_xlate),
};

pub unsafe fn mpc8xx_pic_init() {
    let mut res: resource = core::mem::zeroed();
    let mut np: *mut device_node;
    let ret: libc::c_int;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), b"fsl,pq1-pic\0".as_ptr() as *const libc::c_char);
    if np.is_null() {
        np = of_find_node_by_type(core::ptr::null_mut(), b"mpc8xx-pic\0".as_ptr() as *const libc::c_char);
    }
    if np.is_null() {
        printk!(KERN_ERR "Could not find fsl,pq1-pic node\n");
        return;
    }

    ret = of_address_to_resource(np, 0, &mut res);
    if ret != 0 {
        of_node_put(np);
        return;
    }

    siu_reg = ioremap(res.start, resource_size(&res));
    if siu_reg.is_null() {
        of_node_put(np);
        return;
    }

    mpc8xx_pic_host = irq_domain_create_linear(of_fwnode_handle(np), 64, &mpc8xx_pic_host_ops, core::ptr::null_mut());
    if mpc8xx_pic_host.is_null() {
        printk!(KERN_ERR "MPC8xx PIC: failed to allocate irq host!\n");
    }

    of_node_put(np);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
