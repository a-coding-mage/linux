// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * PowerPC 476FPE board specific routines
 *
 * Copyright © 2013 Tony Breeds IBM Corporation
 * Copyright © 2013 Alistair Popple IBM Corporation
 *
 * Based on earlier code:
 *    Matt Porter <mporter@kernel.crashing.org>
 *    Copyright 2002-2005 MontaVista Software Inc.
 *
 *    Eugene Surovegin <eugene.surovegin@zultys.com> or <ebs@ebshome.net>
 *    Copyright (c) 2003-2005 Zultys Technologies
 *
 *    Rewritten and ported to the merged powerpc tree:
 *    Copyright 2007 David Gibson <dwg@au1.ibm.com>, IBM Corporation.
 *    Copyright © 2011 David Kliekamp IBM Corporation
 */

// Linux kernel dependencies supplied by the surrounding translation.

#[repr(C)]
struct OfDeviceId { compatible: *const core::ffi::c_char }

static PPC47X_OF_BUS: [OfDeviceId; 5] = [
    OfDeviceId { compatible: c"ibm,plb4".as_ptr() },
    OfDeviceId { compatible: c"ibm,plb6".as_ptr() },
    OfDeviceId { compatible: c"ibm,opb".as_ptr() },
    OfDeviceId { compatible: c"ibm,ebc".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe fn quirk_ppc_currituck_usb_fixup(dev: *mut PciDev) {
    if of_machine_is_compatible(c"ibm,currituck".as_ptr()) {
        pci_write_config_dword(dev, 0xe0, 0x0114231f);
        pci_write_config_dword(dev, 0xe4, 0x00006c40);
    }
}

// DECLARE_PCI_FIXUP_HEADER(0x1033, 0x0035, quirk_ppc_currituck_usb_fixup);

// The EEPROM is missing and the default values are bogus. This forces USB into EHCI mode.
const AVR_PWRCTL_CMD: u8 = 0x26;
const AVR_PWRCTL_PWROFF: i32 = 0x01;
const AVR_PWRCTL_RESET: i32 = 0x02;

static mut AVR_I2C_CLIENT: *mut I2cClient = core::ptr::null_mut();

unsafe fn avr_halt_system(pwrctl_flags: i32) -> ! {
    i2c_smbus_write_byte_data(AVR_I2C_CLIENT, AVR_PWRCTL_CMD, pwrctl_flags);
    loop {}
}

unsafe fn avr_power_off_system() { avr_halt_system(AVR_PWRCTL_PWROFF); }

unsafe fn avr_reset_system(_cmd: *mut core::ffi::c_char) -> ! {
    avr_halt_system(AVR_PWRCTL_RESET);
}

unsafe fn avr_probe(client: *mut I2cClient) -> i32 {
    AVR_I2C_CLIENT = client;
    ppc_md.restart = Some(avr_reset_system);
    pm_power_off = Some(avr_power_off_system);
    0
}

static AVR_ID: [I2cDeviceId; 2] = [
    I2cDeviceId { name: c"akebono-avr".as_ptr() },
    I2cDeviceId { name: core::ptr::null() },
];

static mut AVR_DRIVER: I2cDriver = I2cDriver {
    driver: Driver { name: c"akebono-avr".as_ptr() },
    probe: Some(avr_probe),
    id_table: AVR_ID.as_ptr(),
};

unsafe fn ppc47x_device_probe() -> i32 {
    i2c_add_driver(&raw mut AVR_DRIVER);
    of_platform_bus_probe(core::ptr::null_mut(), PPC47X_OF_BUS.as_ptr(), core::ptr::null_mut());
    0
}

// machine_device_initcall(ppc47x_akebono, ppc47x_device_probe);
// machine_device_initcall(ppc47x_currituck, ppc47x_device_probe);

unsafe fn ppc47x_init_irq() {
    let mut np: *mut DeviceNode = core::ptr::null_mut();
    // for_each_node_with_property(np, "interrupt-controller")
    while let Some(node) = for_each_node_with_property(&mut np, c"interrupt-controller".as_ptr()) {
        if !of_property_present(node, c"interrupts".as_ptr()) { break; }
    }
    if np.is_null() { panic!("Can't find top level interrupt controller"); }
    if of_device_is_compatible(np, c"chrp,open-pic".as_ptr()) {
        let mpic = mpic_alloc(np, 0, MPIC_NO_RESET, 0, 0, c" MPIC     ".as_ptr());
        assert!(!mpic.is_null());
        mpic_init(mpic);
        ppc_md.get_irq = Some(mpic_get_irq);
    } else { panic!("Unrecognized top level interrupt controller"); }
    of_node_put(np);
}

#[cfg(CONFIG_SMP)]
unsafe fn smp_ppc47x_setup_cpu(_cpu: i32) { mpic_setup_this_cpu(); }

#[cfg(CONFIG_SMP)]
unsafe fn smp_ppc47x_kick_cpu(cpu: i32) -> i32 {
    let cpunode = of_get_cpu_node(cpu, core::ptr::null_mut());
    assert!(!cpunode.is_null());
    let spin_table_addr_prop = of_get_property(cpunode, c"cpu-release-addr".as_ptr(), core::ptr::null_mut()) as *const u64;
    if spin_table_addr_prop.is_null() { pr_err!("CPU{}: Can't start, missing cpu-release-addr !\n", cpu); return 1; }
    let spin_table = __va(*spin_table_addr_prop) as *mut u32;
    pr_debug!("CPU{}: Spin table mapped at {:p}\n", cpu, spin_table);
    *spin_table.add(3) = cpu as u32;
    smp_wmb();
    *spin_table.add(1) = __pa(start_secondary_47x as *const ()) as u32;
    mb();
    0
}

#[cfg(CONFIG_SMP)]
static mut PPC47X_SMP_OPS: SmpOps = SmpOps {
    probe: Some(smp_mpic_probe), message_pass: Some(smp_mpic_message_pass),
    setup_cpu: Some(smp_ppc47x_setup_cpu), kick_cpu: Some(smp_ppc47x_kick_cpu),
    give_timebase: Some(smp_generic_give_timebase), take_timebase: Some(smp_generic_take_timebase),
};

unsafe fn ppc47x_smp_init() {
    #[cfg(CONFIG_SMP)] if mmu_has_feature(MMU_FTR_TYPE_47X) { smp_ops = &raw mut PPC47X_SMP_OPS; }
}

unsafe fn ppc47x_setup_arch() { swiotlb_detect_4g(); ppc47x_smp_init(); }

static mut BOARD_REV: i32 = -1;
unsafe fn ppc47x_get_board_rev() -> i32 {
    let (np, reg) = if of_machine_is_compatible(c"ibm,currituck".as_ptr()) {
        (of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"ibm,currituck-fpga".as_ptr()), 0)
    } else if of_machine_is_compatible(c"ibm,akebono".as_ptr()) {
        (of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"ibm,akebono-fpga".as_ptr()), 2)
    } else { (core::ptr::null_mut(), 0) };
    if np.is_null() { pr_info!("{}: Unable to find board revision\n", "ppc47x_get_board_rev"); return 0; }
    let fpga = of_iomap(np, 0);
    of_node_put(np);
    if fpga.is_null() { pr_info!("{}: Unable to find board revision\n", "ppc47x_get_board_rev"); return 0; }
    BOARD_REV = (ioread8(fpga.add(reg)) & 0x03) as i32;
    pr_info!("{}: Found board revision {}\n", "ppc47x_get_board_rev", BOARD_REV);
    iounmap(fpga); 0
}

unsafe fn ppc47x_pci_irq_fixup(dev: *mut PciDev) {
    if (*dev).vendor == 0x1033 && ((*dev).device == 0x0035 || (*dev).device == 0x00e0) {
        if BOARD_REV == 0 { (*dev).irq = irq_create_mapping(core::ptr::null_mut(), 47); }
        else if BOARD_REV == 2 { (*dev).irq = irq_create_mapping(core::ptr::null_mut(), 49); }
        else { pr_alert!("{}: Unknown board revision\n", "ppc47x_pci_irq_fixup"); }
    }
}

// define_machine(ppc47x_akebono) {
// .name = "PowerPC 47x (akebono)", .compatible = "ibm,akebono",
// .progress = udbg_progress, .init_IRQ = ppc47x_init_irq,
// .setup_arch = ppc47x_setup_arch, .restart = ppc4xx_reset_system,
// };
// define_machine(ppc47x_currituck) {
// .name = "PowerPC 47x (currituck)", .compatible = "ibm,currituck",
// .progress = udbg_progress, .init_IRQ = ppc47x_init_irq,
// .pci_irq_fixup = ppc47x_pci_irq_fixup, .setup_arch = ppc47x_setup_arch,
// .restart = ppc4xx_reset_system,
// };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
