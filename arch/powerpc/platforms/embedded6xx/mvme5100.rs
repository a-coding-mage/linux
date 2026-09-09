// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Board setup routines for the Motorola/Emerson MVME5100.
 *
 * Copyright 2013 CSC Australia Pty. Ltd.
 *
 * Based on earlier code by:
 *
 *    Matt Porter, MontaVista Software Inc.
 *    Copyright 2001 MontaVista Software Inc.
 *
 * Author: Stephen Chivers <schivers@csc.com>
 */

// Dependencies supplied by the surrounding kernel translation.

const HAWK_MPIC_SIZE: u32 = 0x0004_0000;
const MVME5100_PCI_MEM_OFFSET: usize = 0x0000_0000;

// Board register addresses.
const BOARD_STATUS_REG: usize = 0xfef8_8080;
const BOARD_MODFAIL_REG: usize = 0xfef8_8090;
const BOARD_MODRST_REG: usize = 0xfef8_80a0;
const BOARD_TBEN_REG: usize = 0xfef8_80c0;
const BOARD_SW_READ_REG: usize = 0xfef8_80e0;
const BOARD_GEO_ADDR_REG: usize = 0xfef8_80e8;
const BOARD_EXT_FEATURE1_REG: usize = 0xfef8_80f0;
const BOARD_EXT_FEATURE2_REG: usize = 0xfef8_8100;

static mut pci_membase: phys_addr_t = 0;
static mut restart: *mut u8 = core::ptr::null_mut();

unsafe extern "C" fn mvme5100_8259_cascade(desc: *mut irq_desc) {
    let chip: *mut irq_chip = irq_desc_get_chip(desc);
    let cascade_irq: c_uint = i8259_irq();

    if cascade_irq != 0 {
        generic_handle_irq(cascade_irq);
    }

    ((*chip).irq_eoi)(&mut (*desc).irq_data);
}

unsafe extern "C" fn mvme5100_pic_init() {
    let mut mpic: *mut mpic = core::ptr::null_mut();
    let mut np: *mut device_node;
    let mut cp: *mut device_node = core::ptr::null_mut();
    let mut cirq: c_uint;
    let mut intack: c_ulong = 0;
    let mut prop: *const u32 = core::ptr::null();

    np = of_find_node_by_type(core::ptr::null_mut(), c"open-pic".as_ptr());
    if np.is_null() {
        pr_err!("Could not find open-pic node\n");
        return;
    }

    mpic = mpic_alloc(np, pci_membase, 0, 16, 256, c" OpenPIC  ".as_ptr());
    BUG_ON!(mpic.is_null());
    of_node_put(np);

    mpic_assign_isu(mpic, 0, pci_membase + 0x10000);
    mpic_init(mpic);

    cp = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null(), c"chrp,iic".as_ptr());
    if cp.is_null() {
        pr_warn!("mvme5100_pic_init: couldn't find i8259\n");
        return;
    }

    cirq = irq_of_parse_and_map(cp, 0);
    if cirq == 0 {
        pr_warn!("mvme5100_pic_init: no cascade interrupt?\n");
        return;
    }

    np = of_find_compatible_node(core::ptr::null_mut(), c"pci".as_ptr(), c"mpc10x-pci".as_ptr());
    if !np.is_null() {
        prop = of_get_property(np, c"8259-interrupt-acknowledge".as_ptr(), core::ptr::null_mut());
        if !prop.is_null() {
            intack = *prop as c_ulong;
        }
        of_node_put(np);
    }

    if intack != 0 {
        pr_debug!("mvme5100_pic_init: PCI 8259 intack at 0x%016lx\n", intack);
    }

    i8259_init(cp, intack);
    of_node_put(cp);
    irq_set_chained_handler(cirq, Some(mvme5100_8259_cascade));
}

unsafe extern "C" fn mvme5100_add_bridge(dev: *mut device_node) -> c_int {
    let bus_range: *const c_int;
    let mut len: c_int = 0;
    let hose: *mut pci_controller;
    let mut devid: u16 = 0;

    pr_info!("Adding PCI host bridge %pOF\n", dev);
    bus_range = of_get_property(dev, c"bus-range".as_ptr(), &mut len);
    hose = pcibios_alloc_controller(dev);
    if hose.is_null() {
        return -ENOMEM;
    }

    (*hose).first_busno = if !bus_range.is_null() { *bus_range as u8 } else { 0 };
    (*hose).last_busno = if !bus_range.is_null() { *bus_range.add(1) as u8 } else { 0xff };
    setup_indirect_pci(hose, 0xfe000cf8, 0xfe000cfc, 0);
    pci_process_bridge_OF_ranges(hose, dev, 1);
    early_read_config_word(hose, 0, 0, PCI_DEVICE_ID, &mut devid);

    if devid != PCI_DEVICE_ID_MOTOROLA_HAWK {
        pr_err!("HAWK PHB not present?\n");
        return 0;
    }
    early_read_config_dword(hose, 0, 0, PCI_BASE_ADDRESS_1, &mut pci_membase);
    if pci_membase == 0 {
        pr_err!("HAWK PHB mibar not correctly set?\n");
        return 0;
    }
    pr_info!("mvme5100_pic_init: pci_membase: %x\n", pci_membase);
    0
}

static mvme5100_of_bus_ids: [of_device_id; 2] = [
    of_device_id { compatible: c"hawk-bridge".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

/* Setup the architecture */
unsafe extern "C" fn mvme5100_setup_arch() {
    if ppc_md.progress.is_some() {
        ppc_md.progress.unwrap()(c"mvme5100_setup_arch()".as_ptr(), 0);
    }
    restart = ioremap(BOARD_MODRST_REG, 4) as *mut u8;
}

unsafe extern "C" fn mvme5100_setup_pci() {
    let mut np: *mut device_node = core::ptr::null_mut();
    for_each_compatible_node!(np, c"pci", c"hawk-pci") {
        mvme5100_add_bridge(np);
    }
}

unsafe extern "C" fn mvme5100_show_cpuinfo(m: *mut seq_file) {
    seq_puts(m, c"Vendor\t\t: Motorola/Emerson\n".as_ptr());
    seq_puts(m, c"Machine\t\t: MVME5100\n".as_ptr());
}

unsafe extern "C" fn mvme5100_restart(_cmd: *mut c_char) -> ! {
    local_irq_disable();
    mtmsr(mfmsr() | MSR_IP);
    out_8(restart, 0x01);
    loop {}
}

unsafe extern "C" fn probe_of_platform_devices() -> c_int {
    of_platform_bus_probe(core::ptr::null_mut(), mvme5100_of_bus_ids.as_ptr(), core::ptr::null_mut());
    0
}

machine_device_initcall!(mvme5100, probe_of_platform_devices);

define_machine!(mvme5100 {
    .name = c"MVME5100",
    .compatible = c"MVME5100",
    .setup_arch = mvme5100_setup_arch,
    .discover_phbs = mvme5100_setup_pci,
    .init_IRQ = mvme5100_pic_init,
    .show_cpuinfo = mvme5100_show_cpuinfo,
    .get_irq = mpic_get_irq,
    .restart = mvme5100_restart,
    .progress = udbg_progress,
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
