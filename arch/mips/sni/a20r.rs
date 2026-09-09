/*
 * A20R specific code
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2006 Thomas Bogendoerfer (tsbogend@alpha.franken.de)
 */

// External Linux kernel and MIPS definitions are supplied by other translation units.

const fn port(base: u16, irq: i32) -> PlatSerial8250Port {
    PlatSerial8250Port { iobase: base, irq, uartclk: 1_843_200, iotype: UPIO_PORT, flags: UPF_BOOT_AUTOCONF }
}

static mut A20R_DATA: [PlatSerial8250Port; 3] = [
    port(0x3f8, 4),
    port(0x2f8, 3),
    PlatSerial8250Port { iobase: 0, irq: 0, uartclk: 0, iotype: 0, flags: 0 },
];

static mut A20R_SERIAL8250_DEVICE: PlatformDevice = PlatformDevice {
    name: "serial8250",
    id: PLAT8250_DEV_PLATFORM,
    dev: Device { platform_data: unsafe { &raw mut A20R_DATA as *mut _ } },
    num_resources: 0,
    resource: core::ptr::null_mut(),
};

static mut A20R_DS1216_RSRC: [Resource; 1] = [Resource { start: 0x1c081ffc, end: 0x1c081fff, flags: IORESOURCE_MEM }];

static mut A20R_DS1216_DEVICE: PlatformDevice = PlatformDevice {
    name: "rtc-ds1216", id: 0, dev: Device { platform_data: core::ptr::null_mut() },
    num_resources: 1, resource: unsafe { A20R_DS1216_RSRC.as_mut_ptr() },
};

static mut SNIRM_82596_RSRC: [Resource; 5] = [
    Resource { start: 0x18000000, end: 0x18000004, flags: IORESOURCE_MEM },
    Resource { start: 0x18010000, end: 0x18010004, flags: IORESOURCE_MEM },
    Resource { start: 0x1ff00000, end: 0x1ff00020, flags: IORESOURCE_MEM },
    Resource { start: 22, end: 22, flags: IORESOURCE_IRQ },
    Resource { start: 0, end: 0, flags: 0x01 }, // 16bit mpu port access
];

static mut SNIRM_82596_PDEV: PlatformDevice = PlatformDevice {
    name: "snirm_82596", id: 0, dev: Device { platform_data: core::ptr::null_mut() },
    num_resources: 5, resource: unsafe { SNIRM_82596_RSRC.as_mut_ptr() },
};

static mut SNIRM_53C710_RSRC: [Resource; 2] = [
    Resource { start: 0x19000000, end: 0x190fffff, flags: IORESOURCE_MEM },
    Resource { start: 19, end: 19, flags: IORESOURCE_IRQ },
];

static mut SNIRM_53C710_PDEV: PlatformDevice = PlatformDevice {
    name: "snirm_53c710", id: 0, dev: Device { platform_data: core::ptr::null_mut() },
    num_resources: 2, resource: unsafe { SNIRM_53C710_RSRC.as_mut_ptr() },
};

static mut SC26XX_RSRC: [Resource; 2] = [
    Resource { start: 0x1c070000, end: 0x1c0700ff, flags: IORESOURCE_MEM },
    Resource { start: 20, end: 20, flags: IORESOURCE_IRQ },
];

static mut SCCNXP_DATA: SccnxpPdata = SccnxpPdata {
    reg_shift: 2,
    mctrl_cfg: [
        MCTRL_SIG(DTR_OP, LINE_OP7) | MCTRL_SIG(RTS_OP, LINE_OP3) |
        MCTRL_SIG(DSR_IP, LINE_IP5) | MCTRL_SIG(DCD_IP, LINE_IP6),
        MCTRL_SIG(DTR_OP, LINE_OP2) | MCTRL_SIG(RTS_OP, LINE_OP1) |
        MCTRL_SIG(DSR_IP, LINE_IP0) | MCTRL_SIG(CTS_IP, LINE_IP1) |
        MCTRL_SIG(DCD_IP, LINE_IP2) | MCTRL_SIG(RNG_IP, LINE_IP3),
    ],
};

static mut SC26XX_PDEV: PlatformDevice = PlatformDevice {
    name: "sc2681", id: 0,
    dev: Device { platform_data: unsafe { &raw mut SCCNXP_DATA as *mut _ } },
    num_resources: 2, resource: unsafe { SC26XX_RSRC.as_mut_ptr() },
};

unsafe fn a20r_update_cause_ip() -> u32 {
    let status = read_c0_status();
    write_c0_status(status | 0x00010000);
    // Original MIPS volatile assembly: updates PCIMT_UCONF and 0xbc000000,
    // performs chipset load/store, synchronization, and the delay loop.
    // The exact register constraints are architecture/toolchain-specific.
    write_c0_status(status);
    status
}

unsafe fn unmask_a20r_irq(d: *mut IrqData) {
    set_c0_status(0x100 << ((*d).irq - SNI_A20R_IRQ_BASE));
    irq_enable_hazard();
}

unsafe fn mask_a20r_irq(d: *mut IrqData) {
    clear_c0_status(0x100 << ((*d).irq - SNI_A20R_IRQ_BASE));
    irq_disable_hazard();
}

static mut A20R_IRQ_TYPE: IrqChip = IrqChip { name: "A20R", irq_mask: Some(mask_a20r_irq), irq_unmask: Some(unmask_a20r_irq) };

unsafe fn a20r_hwint() {
    let mut cause: u32;
    let status: u32;
    let irq: i32;
    clear_c0_status(IE_IRQ0);
    status = a20r_update_cause_ip();
    cause = read_c0_cause();
    irq = ffs(((cause & status) >> 8) & 0xf8);
    if irq > 0 { do_IRQ(SNI_A20R_IRQ_BASE + irq - 1); }
    a20r_update_cause_ip();
    set_c0_status(IE_IRQ0);
}

pub unsafe fn sni_a20r_irq_init() {
    let mut i = SNI_A20R_IRQ_BASE + 2;
    while i < SNI_A20R_IRQ_BASE + 8 {
        irq_set_chip_and_handler(i, &raw mut A20R_IRQ_TYPE, handle_level_irq);
        i += 1;
    }
    sni_hwint = Some(a20r_hwint);
    change_c0_status(ST0_IM, IE_IRQ0);
    if request_irq(SNI_A20R_IRQ_BASE + 3, sni_isa_irq_handler, IRQF_SHARED, "ISA", sni_isa_irq_handler) != 0 {
        pr_err("Failed to register ISA interrupt\n");
    }
}

pub unsafe fn sni_a20r_init() {
    /* FIXME, remove if not needed */
}

unsafe fn snirm_a20r_setup_devinit() -> i32 {
    match sni_brd_type {
        SNI_BRD_TOWER_OASIC | SNI_BRD_MINITOWER => {
            platform_device_register(&raw mut SNIRM_82596_PDEV);
            platform_device_register(&raw mut SNIRM_53C710_PDEV);
            platform_device_register(&raw mut SC26XX_PDEV);
            platform_device_register(&raw mut A20R_SERIAL8250_DEVICE);
            platform_device_register(&raw mut A20R_DS1216_DEVICE);
            sni_eisa_root_init();
        }
        _ => {}
    }
    0
}

// device_initcall(snirm_a20r_setup_devinit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
