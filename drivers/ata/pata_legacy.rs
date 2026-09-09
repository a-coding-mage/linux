// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   pata-legacy.c - Legacy port PATA/SATA controller driver.
 *   Copyright 2005/2006 Red Hat, all rights reserved.
 *
 *   An ATA driver for the legacy ATA ports.
 *
 *  This driver handles legacy (that is "ISA side") IDE ports found
 *  on PC class systems. There are three hybrid devices that are exceptions:
 *  The Cyrix 5510/5520 where a pre SFF ATA device is on the bridge and
 *  the MPIIX where the tuning is PCI side but the IDE is "ISA side".
 */

// C dependencies supplied by the surrounding kernel translation.

const DRV_NAME: &str = "pata_legacy";
const DRV_VERSION: &str = "0.6.5";
const NR_HOST: usize = 6;

static mut all: i32 = 0;
static mut probe_all: i32 = 0;
static mut probe_mask: i32 = !0;
static mut autospeed: i32 = 0;
static mut pio_mask: i32 = ATA_PIO4;
static mut iordy_mask: i32 = 0xFFFF_FFFFu32 as i32;

#[repr(i32)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum controller {
    BIOS = 0,
    SNOOP = 1,
    UNKNOWN = -1,
}

#[repr(C)]
struct legacy_data {
    timing: c_ulong,
    clock: [u8; 2],
    last: u8,
    fast: i32,
    type_: controller,
    platform_dev: *mut platform_device,
}

#[repr(C)]
struct legacy_probe {
    name: *mut u8,
    port: c_ulong,
    irq: u32,
    slot: u32,
    type_: controller,
    private: c_ulong,
}

#[repr(C)]
struct legacy_controller {
    name: *const c_char,
    ops: *mut ata_port_operations,
    pio_mask: u32,
    flags: u32,
    pflags: u32,
    setup: Option<unsafe extern "C" fn(*mut platform_device, *mut legacy_probe, *mut legacy_data) -> i32>,
}

static mut legacy_port: [c_ulong; NR_HOST] = [0x1f0, 0x170, 0x1e8, 0x168, 0x1e0, 0x160];
static mut probe_list: [legacy_probe; NR_HOST] = [unsafe { core::mem::zeroed() }; NR_HOST];
static mut legacy_data: [legacy_data; NR_HOST] = [unsafe { core::mem::zeroed() }; NR_HOST];
static mut legacy_host: [*mut ata_host; NR_HOST] = [core::ptr::null_mut(); NR_HOST];

unsafe fn legacy_probe_add(port: c_ulong, irq: u32, type_: controller, private: c_ulong) -> i32 {
    let mut free: *mut legacy_probe = core::ptr::null_mut();
    for i in 0..NR_HOST {
        let lp = &mut probe_list[i] as *mut legacy_probe;
        if (*lp).port == 0 && free.is_null() { free = lp; }
        if (*lp).port == port || legacy_port[i] == port {
            if (probe_mask & (1 << i)) == 0 { return -1; }
            free = lp;
            break;
        }
    }
    if free.is_null() { printk(KERN_ERR, "pata_legacy: Too many interfaces.\n"); return -1; }
    (*free).port = port;
    (*free).irq = irq;
    (*free).type_ = type_;
    (*free).private = private;
    0
}

unsafe extern "C" fn legacy_set_mode(link: *mut ata_link, _unused: *mut *mut ata_device) -> i32 {
    let mut dev: *mut ata_device;
    ata_for_each_dev!(dev, link, ENABLED, {
        ata_dev_info!(dev, "configured for PIO\n");
        (*dev).pio_mode = XFER_PIO_0;
        (*dev).xfer_mode = XFER_PIO_0;
        (*dev).xfer_shift = ATA_SHIFT_PIO;
        (*dev).flags |= ATA_DFLAG_PIO;
    });
    0
}

static legacy_sht: scsi_host_template = scsi_host_template { ATA_PIO_SHT!(DRV_NAME) };
static legacy_base_port_ops: ata_port_operations = ata_port_operations {
    inherits: &ata_sff_port_ops,
    cable_detect: Some(ata_cable_40wire),
    ..unsafe { core::mem::zeroed() }
};
static mut simple_port_ops: ata_port_operations = ata_port_operations {
    inherits: &legacy_base_port_ops,
    sff_data_xfer: Some(ata_sff_data_xfer32),
    ..unsafe { core::mem::zeroed() }
};
static mut legacy_port_ops: ata_port_operations = ata_port_operations {
    inherits: &legacy_base_port_ops,
    sff_data_xfer: Some(ata_sff_data_xfer32),
    set_mode: Some(legacy_set_mode),
    ..unsafe { core::mem::zeroed() }
};
static mut controllers: [legacy_controller; 2] = [
    legacy_controller { name: b"BIOS\0".as_ptr() as *const c_char, ops: &mut legacy_port_ops, pio_mask: ATA_PIO4, flags: ATA_FLAG_NO_IORDY, pflags: 0, setup: None },
    legacy_controller { name: b"Snooping\0".as_ptr() as *const c_char, ops: &mut simple_port_ops, pio_mask: ATA_PIO4, flags: 0, pflags: 0, setup: None },
];

unsafe fn probe_chip_type(probe: *mut legacy_probe) -> controller {
    if (autospeed & (1 << (*probe).slot)) != 0 { controller::SNOOP } else { controller::BIOS }
}

unsafe fn legacy_init_one(probe: *mut legacy_probe) -> i32 {
    let controller = &controllers[(*probe).type_ as usize];
    let pio_modes = controller.pio_mask;
    let io = (*probe).port;
    let mask = 1u32 << (*probe).slot;
    let ops = controller.ops;
    let ld = &mut legacy_data[(*probe).slot as usize];
    let mut host: *mut ata_host = core::ptr::null_mut();
    let mut ap: *mut ata_port;
    let mut pdev: *mut platform_device;
    let mut dev: *mut ata_device;
    let iordy = if (iordy_mask & mask as i32) != 0 { 0 } else { ATA_FLAG_NO_IORDY } | controller.flags;
    pdev = platform_device_register_simple(DRV_NAME, (*probe).slot as i32, core::ptr::null(), 0);
    if IS_ERR(pdev) { return PTR_ERR(pdev); }
    let mut ret = -EBUSY;
    if devm_request_region(&mut (*pdev).dev, io, 8, b"pata_legacy\0".as_ptr() as *const c_char).is_null()
        || devm_request_region(&mut (*pdev).dev, io + 0x0206, 1, b"pata_legacy\0".as_ptr() as *const c_char).is_null() { platform_device_unregister(pdev); return ret; }
    ret = -ENOMEM;
    let io_addr = devm_ioport_map(&mut (*pdev).dev, io, 8);
    let ctrl_addr = devm_ioport_map(&mut (*pdev).dev, io + 0x0206, 1);
    if io_addr.is_null() || ctrl_addr.is_null() { platform_device_unregister(pdev); return ret; }
    ld.type_ = (*probe).type_;
    if let Some(setup) = controller.setup { if setup(pdev, probe, ld) < 0 { platform_device_unregister(pdev); return ret; } }
    host = ata_host_alloc(&mut (*pdev).dev, 1);
    if host.is_null() { platform_device_unregister(pdev); return ret; }
    ap = (*host).ports[0];
    (*ap).ops = ops;
    (*ap).pio_mask = pio_modes;
    (*ap).flags |= ATA_FLAG_SLAVE_POSS | iordy;
    (*ap).pflags |= controller.pflags;
    (*ap).ioaddr.cmd_addr = io_addr;
    (*ap).ioaddr.altstatus_addr = ctrl_addr;
    (*ap).ioaddr.ctl_addr = ctrl_addr;
    ata_sff_std_ports(&mut (*ap).ioaddr);
    (*ap).host.private_data = ld as *mut legacy_data as *mut c_void;
    ata_port_desc!(ap, "cmd 0x%lx ctl 0x%lx", io, io + 0x0206);
    ret = ata_host_activate(host, (*probe).irq, Some(ata_sff_interrupt), 0, &legacy_sht);
    if ret != 0 { platform_device_unregister(pdev); return ret; }
    async_synchronize_full();
    ld.platform_dev = pdev;
    ret = -ENODEV;
    ata_for_each_dev!(dev, &mut (*ap).link, ALL, {
        if !ata_dev_absent(dev) {
            legacy_host[(*probe).slot as usize] = host;
            ld.platform_dev = pdev;
            return 0;
        }
    });
    ata_host_detach(host);
    platform_device_unregister(pdev);
    return ret;
}

unsafe fn legacy_check_special_cases(p: *mut pci_dev, primary: *mut i32, secondary: *mut i32) {
    if (*p).vendor == 0x1078 && ((*p).device == 0x0000 || (*p).device == 0x0002) {
        *primary = 1; *secondary = 1; return;
    }
    if (*p).vendor == 0x8086 && (*p).device == 0x1234 {
        let mut r: u16 = 0;
        pci_read_config_word(p, 0x6C, &mut r);
        if r & 0x8000 != 0 {
            if r & 0x4000 != 0 { *secondary = 1; } else { *primary = 1; }
        }
    }
}

unsafe fn legacy_init() -> i32 {
    let mut primary = 0;
    let mut secondary = 0;
    let mut pci_present = 0;
    let mut p: *mut pci_dev = core::ptr::null_mut();
    for_each_pci_dev!(p, {
        for r in 0..6 {
            if pci_resource_start(p, r) == 0x1f0 { primary = 1; }
            if pci_resource_start(p, r) == 0x170 { secondary = 1; }
        }
        legacy_check_special_cases(p, &mut primary, &mut secondary);
        pci_present = 1;
    });
    if primary == 0 || all != 0 { legacy_probe_add(0x1F0, 14, controller::UNKNOWN, 0); }
    if secondary == 0 || all != 0 { legacy_probe_add(0x170, 15, controller::UNKNOWN, 0); }
    if probe_all != 0 || pci_present == 0 {
        legacy_probe_add(0x1E8, 11, controller::UNKNOWN, 0);
        legacy_probe_add(0x168, 10, controller::UNKNOWN, 0);
        legacy_probe_add(0x1E0, 8, controller::UNKNOWN, 0);
        legacy_probe_add(0x160, 12, controller::UNKNOWN, 0);
    }
    let mut ct = 0;
    let mut slot = 0;
    for i in 0..NR_HOST {
        let pl = &mut probe_list[i];
        if pl.port == 0 { continue; }
        if pl.type_ == controller::UNKNOWN { pl.type_ = probe_chip_type(pl); }
        pl.slot = slot; slot += 1;
        if legacy_init_one(pl) == 0 { ct += 1; }
    }
    if ct != 0 { 0 } else { -ENODEV }
}

unsafe fn legacy_exit() {
    for i in 0..NR_HOST {
        if !legacy_host[i].is_null() { ata_host_detach(legacy_host[i]); }
        platform_device_unregister(legacy_data[i].platform_dev);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
