/*
 * Broadcom specific AMBA
 * System on Chip (SoC) Host
 *
 * Licensed under the GNU/GPL. See COPYING for details.
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn bcma_host_soc_read8(core: *mut bcma_device, offset: u16) -> u8 {
    readb((*core).io_addr.add(offset as usize))
}

unsafe fn bcma_host_soc_read16(core: *mut bcma_device, offset: u16) -> u16 {
    readw((*core).io_addr.add(offset as usize))
}

unsafe fn bcma_host_soc_read32(core: *mut bcma_device, offset: u16) -> u32 {
    readl((*core).io_addr.add(offset as usize))
}

unsafe fn bcma_host_soc_write8(core: *mut bcma_device, offset: u16, value: u8) {
    writeb(value, (*core).io_addr.add(offset as usize));
}

unsafe fn bcma_host_soc_write16(core: *mut bcma_device, offset: u16, value: u16) {
    writew(value, (*core).io_addr.add(offset as usize));
}

unsafe fn bcma_host_soc_write32(core: *mut bcma_device, offset: u16, value: u32) {
    writel(value, (*core).io_addr.add(offset as usize));
}

#[cfg(CONFIG_BCMA_BLOCKIO)]
unsafe fn bcma_host_soc_block_read(
    core: *mut bcma_device,
    buffer: *mut core::ffi::c_void,
    mut count: usize,
    offset: u16,
    reg_width: u8,
) {
    let addr = (*core).io_addr.add(offset as usize);

    match reg_width as usize {
        1 => {
            let mut buf = buffer as *mut u8;
            while count != 0 {
                *buf = __raw_readb(addr);
                buf = buf.add(1);
                count -= 1;
            }
        }
        2 => {
            let mut buf = buffer as *mut u16;
            WARN_ON(count & 1 != 0);
            while count != 0 {
                *buf = __raw_readw(addr).to_le();
                buf = buf.add(1);
                count -= 2;
            }
        }
        4 => {
            let mut buf = buffer as *mut u32;
            WARN_ON(count & 3 != 0);
            while count != 0 {
                *buf = __raw_readl(addr).to_le();
                buf = buf.add(1);
                count -= 4;
            }
        }
        _ => {
            WARN_ON(true);
        }
    }
}

#[cfg(CONFIG_BCMA_BLOCKIO)]
unsafe fn bcma_host_soc_block_write(
    core: *mut bcma_device,
    buffer: *const core::ffi::c_void,
    mut count: usize,
    offset: u16,
    reg_width: u8,
) {
    let addr = (*core).io_addr.add(offset as usize);

    match reg_width as usize {
        1 => {
            let mut buf = buffer as *const u8;
            while count != 0 {
                __raw_writeb(*buf, addr);
                buf = buf.add(1);
                count -= 1;
            }
        }
        2 => {
            let mut buf = buffer as *const u16;
            WARN_ON(count & 1 != 0);
            while count != 0 {
                __raw_writew((*buf).to_le(), addr);
                buf = buf.add(1);
                count -= 2;
            }
        }
        4 => {
            let mut buf = buffer as *const u32;
            WARN_ON(count & 3 != 0);
            while count != 0 {
                __raw_writel((*buf).to_le(), addr);
                buf = buf.add(1);
                count -= 4;
            }
        }
        _ => {
            WARN_ON(true);
        }
    }
}

unsafe fn bcma_host_soc_aread32(core: *mut bcma_device, offset: u16) -> u32 {
    if WARN_ONCE((*core).io_wrap.is_null(), "Accessed core has no wrapper/agent\n") {
        return !0;
    }
    readl((*core).io_wrap.add(offset as usize))
}

unsafe fn bcma_host_soc_awrite32(core: *mut bcma_device, offset: u16, value: u32) {
    if WARN_ONCE((*core).io_wrap.is_null(), "Accessed core has no wrapper/agent\n") {
        return;
    }
    writel(value, (*core).io_wrap.add(offset as usize));
}

static bcma_host_soc_ops: bcma_host_ops = bcma_host_ops {
    read8: Some(bcma_host_soc_read8),
    read16: Some(bcma_host_soc_read16),
    read32: Some(bcma_host_soc_read32),
    write8: Some(bcma_host_soc_write8),
    write16: Some(bcma_host_soc_write16),
    write32: Some(bcma_host_soc_write32),
    #[cfg(CONFIG_BCMA_BLOCKIO)]
    block_read: Some(bcma_host_soc_block_read),
    #[cfg(CONFIG_BCMA_BLOCKIO)]
    block_write: Some(bcma_host_soc_block_write),
    aread32: Some(bcma_host_soc_aread32),
    awrite32: Some(bcma_host_soc_awrite32),
};

unsafe fn bcma_host_soc_register(soc: *mut bcma_soc) -> i32 {
    let bus = &mut (*soc).bus;

    /* iomap only first core. We have to read some register on this core
     * to scan the bus.
     */
    bus.mmio = ioremap(BCMA_ADDR_BASE, BCMA_CORE_SIZE * 1);
    if bus.mmio.is_null() {
        return -ENOMEM;
    }

    /* Host specific */
    bus.hosttype = BCMA_HOSTTYPE_SOC;
    bus.ops = &bcma_host_soc_ops;

    /* Initialize struct, detect chip */
    bcma_init_bus(bus);

    0
}

unsafe fn bcma_host_soc_init(soc: *mut bcma_soc) -> i32 {
    let bus = &mut (*soc).bus;
    let err = bcma_bus_early_register(bus);
    if err != 0 {
        iounmap(bus.mmio);
    }
    err
}

#[cfg(CONFIG_OF)]
unsafe fn bcma_host_soc_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let np = (*dev).of_node;
    let bus = devm_kzalloc(dev, core::mem::size_of::<bcma_bus>(), GFP_KERNEL) as *mut bcma_bus;
    if bus.is_null() {
        return -ENOMEM;
    }

    (*bus).dev = dev;
    (*bus).mmio = of_iomap(np, 0);
    if (*bus).mmio.is_null() {
        return -ENOMEM;
    }

    (*bus).hosttype = BCMA_HOSTTYPE_SOC;
    (*bus).ops = &bcma_host_soc_ops;
    bcma_init_bus(bus);

    let err = bcma_bus_register(bus);
    if err != 0 {
        iounmap((*bus).mmio);
        return err;
    }
    platform_set_drvdata(pdev, bus as *mut core::ffi::c_void);
    err
}

#[cfg(CONFIG_OF)]
unsafe fn bcma_host_soc_remove(pdev: *mut platform_device) {
    let bus = platform_get_drvdata(pdev) as *mut bcma_bus;
    bcma_bus_unregister(bus);
    iounmap((*bus).mmio);
    platform_set_drvdata(pdev, core::ptr::null_mut());
}

#[cfg(CONFIG_OF)]
static bcma_host_soc_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "brcm,bus-axi" },
    of_device_id { ..of_device_id::default() },
];

#[cfg(CONFIG_OF)]
static mut bcma_host_soc_driver: platform_driver = platform_driver {
    driver: driver {
        name: "bcma-host-soc",
        of_match_table: bcma_host_soc_of_match.as_ptr(),
    },
    probe: Some(bcma_host_soc_probe),
    remove: Some(bcma_host_soc_remove),
};

#[cfg(CONFIG_OF)]
unsafe fn bcma_host_soc_register_driver() -> i32 {
    platform_driver_register(&mut bcma_host_soc_driver)
}

#[cfg(CONFIG_OF)]
unsafe fn bcma_host_soc_unregister_driver() {
    platform_driver_unregister(&mut bcma_host_soc_driver);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
