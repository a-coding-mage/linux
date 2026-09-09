// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/xtensa/platform/xtavnet/setup.c
 *
 * Rust translation of the original implementation source.
 */

// Kernel and platform dependencies supplied by the surrounding tree are intentionally external.

unsafe fn xtfpga_power_off(_unused: *mut sys_off_data) -> i32 {
    lcd_disp_at_pos(b"POWEROFF\0".as_ptr() as *const i8, 0);
    local_irq_disable();
    loop {
        cpu_relax();
    }
}

unsafe fn xtfpga_restart(_unused: *mut sys_off_data) -> i32 {
    // Try software reset first.
    core::ptr::write_volatile(XTFPGA_SWRST_VADDR as *mut u32, 0xdead);

    // If software reset did not work, flush and reset the mmu,
    // simulate a processor reset, and jump to the reset vector.
    cpu_reset();

    NOTIFY_DONE
}

#[cfg(feature = "CONFIG_XTENSA_CALIBRATE_CCOUNT")]
unsafe fn platform_calibrate_ccount() {
    ccount_freq = core::ptr::read_volatile(XTFPGA_CLKFRQ_VADDR as *const libc::c_long);
}

unsafe fn xtfpga_register_handlers() {
    register_sys_off_handler(
        SYS_OFF_MODE_RESTART,
        SYS_OFF_PRIO_PLATFORM,
        Some(xtfpga_restart),
        core::ptr::null_mut(),
    );
    register_sys_off_handler(
        SYS_OFF_MODE_POWER_OFF,
        SYS_OFF_PRIO_DEFAULT,
        Some(xtfpga_power_off),
        core::ptr::null_mut(),
    );
}

#[cfg(feature = "CONFIG_USE_OF")]
unsafe fn xtfpga_clk_setup(np: *mut device_node) {
    let base = of_iomap(np, 0);
    let freq: u32;

    if base.is_null() {
        pr_err(b"%pOFn: invalid address\n\0".as_ptr() as *const i8, np);
        return;
    }

    freq = __raw_readl(base);
    iounmap(base);
    let clk = clk_register_fixed_rate(
        core::ptr::null_mut(),
        (*np).name,
        core::ptr::null(),
        0,
        freq,
    );

    if IS_ERR(clk) {
        pr_err(b"%pOFn: clk registration failed\n\0".as_ptr() as *const i8, np);
        return;
    }

    if of_clk_add_provider(np, Some(of_clk_src_simple_get), clk) != 0 {
        pr_err(b"%pOFn: clk provider registration failed\n\0".as_ptr() as *const i8, np);
    }
}

#[cfg(feature = "CONFIG_USE_OF")]
const _: () = {
    // CLK_OF_DECLARE(xtfpga_clk, "cdns,xtfpga-clock", xtfpga_clk_setup);
};

#[cfg(feature = "CONFIG_USE_OF")]
const MAC_LEN: usize = 6;

#[cfg(feature = "CONFIG_USE_OF")]
unsafe fn update_local_mac(node: *mut device_node) {
    let mut prop_len = 0;
    let macaddr = of_get_property(node, b"local-mac-address\0".as_ptr() as *const i8, &mut prop_len);
    if macaddr.is_null() || prop_len != MAC_LEN as i32 {
        return;
    }

    let newmac = kzalloc(core::mem::size_of::<property>() + MAC_LEN, GFP_KERNEL) as *mut property;
    if newmac.is_null() {
        return;
    }
    (*newmac).value = newmac.add(1) as *mut core::ffi::c_void;
    (*newmac).length = MAC_LEN as u32;
    (*newmac).name = kstrdup(b"local-mac-address\0".as_ptr() as *const i8, GFP_KERNEL);
    if (*newmac).name.is_null() {
        kfree(newmac as *mut core::ffi::c_void);
        return;
    }

    core::ptr::copy_nonoverlapping(macaddr, (*newmac).value as *mut u8, MAC_LEN);
    *((*newmac).value as *mut u8).add(5) = core::ptr::read_volatile(DIP_SWITCHES_VADDR as *const u32) as u8 & 0x3f;
    of_update_property(node, newmac);
}

#[cfg(feature = "CONFIG_USE_OF")]
unsafe fn machine_setup() -> i32 {
    let mut eth: *mut device_node = core::ptr::null_mut();
    eth = of_find_compatible_node(eth, core::ptr::null_mut(), b"opencores,ethoc\0".as_ptr() as *const i8);
    if !eth.is_null() {
        update_local_mac(eth);
    }
    of_node_put(eth);
    xtfpga_register_handlers();
    0
}

#[cfg(not(feature = "CONFIG_USE_OF"))]
static mut ethoc_res: [resource; 3] = [
    resource { start: OETH_REGS_PADDR, end: OETH_REGS_PADDR + OETH_REGS_SIZE - 1, flags: IORESOURCE_MEM },
    resource { start: OETH_SRAMBUFF_PADDR, end: OETH_SRAMBUFF_PADDR + OETH_SRAMBUFF_SIZE - 1, flags: IORESOURCE_MEM },
    resource { start: XTENSA_PIC_LINUX_IRQ(OETH_IRQ), end: XTENSA_PIC_LINUX_IRQ(OETH_IRQ), flags: IORESOURCE_IRQ },
];

#[cfg(not(feature = "CONFIG_USE_OF"))]
static mut ethoc_pdata: ethoc_platform_data = ethoc_platform_data {
    hwaddr: [0x00, 0x50, 0xc2, 0x13, 0x6f, 0],
    phy_id: -1,
    big_endian: XCHAL_HAVE_BE,
};

#[cfg(not(feature = "CONFIG_USE_OF"))]
static mut c67x00_res: [resource; 2] = [
    resource { start: C67X00_PADDR, end: C67X00_PADDR + C67X00_SIZE - 1, flags: IORESOURCE_MEM },
    resource { start: XTENSA_PIC_LINUX_IRQ(C67X00_IRQ), end: XTENSA_PIC_LINUX_IRQ(C67X00_IRQ), flags: IORESOURCE_IRQ },
];

#[cfg(not(feature = "CONFIG_USE_OF"))]
static mut c67x00_pdata: c67x00_platform_data = c67x00_platform_data {
    sie_config: C67X00_SIE1_HOST | C67X00_SIE2_UNUSED,
    hpi_regstep: 4,
};

#[cfg(not(feature = "CONFIG_USE_OF"))]
static mut serial_resource: resource = resource { start: DUART16552_PADDR, end: DUART16552_PADDR + 0x1f, flags: IORESOURCE_MEM };

#[cfg(not(feature = "CONFIG_USE_OF"))]
static mut serial_platform_data: [plat_serial8250_port; 2] = [
    plat_serial8250_port {
        mapbase: DUART16552_PADDR,
        irq: XTENSA_PIC_LINUX_IRQ(DUART16552_INTNUM),
        flags: UPF_BOOT_AUTOCONF | UPF_SKIP_TEST | UPF_IOREMAP,
        iotype: if XCHAL_HAVE_BE { UPIO_MEM32BE } else { UPIO_MEM32 },
        regshift: 2,
        uartclk: 0,
    },
    plat_serial8250_port { ..Default::default() },
];

#[cfg(not(feature = "CONFIG_USE_OF"))]
unsafe fn xtavnet_init() -> i32 {
    ethoc_pdata.hwaddr[5] = core::ptr::read_volatile(DIP_SWITCHES_VADDR as *const u32) as u8;
    serial_platform_data[0].uartclk = core::ptr::read_volatile(XTFPGA_CLKFRQ_VADDR as *const libc::c_long) as _;
    platform_add_devices(platform_devices.as_mut_ptr(), platform_devices.len());
    pr_info(b"XTFPGA: Ethernet MAC %pM\n\0".as_ptr() as *const i8, ethoc_pdata.hwaddr.as_ptr());
    ethoc_pdata.eth_clkfreq = core::ptr::read_volatile(XTFPGA_CLKFRQ_VADDR as *const libc::c_long) as _;
    xtfpga_register_handlers();
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
