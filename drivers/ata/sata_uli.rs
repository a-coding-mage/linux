// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *  sata_uli.c - ULi Electronics SATA
 *
 *  libata documentation is available via 'make {ps|pdf}docs',
 *  as Documentation/driver-api/libata.rst
 *
 *  Hardware documentation available under NDA.
 */

// Linux kernel headers supplied by the surrounding translation unit.

const DRV_NAME: &str = "sata_uli";
const DRV_VERSION: &str = "1.3";

enum {
    uli_5289 = 0,
    uli_5287 = 1,
    uli_5281 = 2,

    uli_max_ports = 4,

    /* PCI configuration registers */
    ULI5287_BASE = 0x90, /* sata0 phy SCR registers */
    ULI5287_OFFS = 0x10, /* offset from sata0->sata1 phy regs */
    ULI5281_BASE = 0x60, /* sata0 phy SCR  registers */
    ULI5281_OFFS = 0x60, /* offset from sata0->sata1 phy regs */
}

#[repr(C)]
struct uli_priv {
    scr_cfg_addr: [u32; uli_max_ports],
}

unsafe extern "C" {
    fn uli_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32;
    fn uli_scr_read(link: *mut ata_link, sc_reg: u32, val: *mut u32) -> i32;
    fn uli_scr_write(link: *mut ata_link, sc_reg: u32, val: u32) -> i32;
}

// External kernel types and symbols are provided by the surrounding modules.
extern "C" {
    static uli_pci_tbl: [pci_device_id; 4];
    static mut uli_pci_driver: pci_driver;
    static uli_sht: scsi_host_template;
    static mut uli_ops: ata_port_operations;
    static uli_port_info: ata_port_info;
}

static mut ULI_PCI_TBL: [pci_device_id; 4] = [
    pci_device_id { vendor: 0, device: 0x5289, driver_data: uli_5289 },
    pci_device_id { vendor: 0, device: 0x5287, driver_data: uli_5287 },
    pci_device_id { vendor: 0, device: 0x5281, driver_data: uli_5281 },
    pci_device_id { vendor: 0, device: 0, driver_data: 0 }, /* terminate list */
];

// The following tables correspond to the C designated initializers and use
// kernel-provided constructors/macros where their exact definitions are external.
static mut ULI_PCI_DRIVER: pci_driver = pci_driver {
    name: DRV_NAME,
    id_table: ULI_PCI_TBL.as_ptr(),
    probe: Some(uli_init_one),
    remove: Some(ata_pci_remove_one),
};

static ULI_SHT: scsi_host_template = ATA_BMDMA_SHT!(DRV_NAME);

static mut ULI_OPS: ata_port_operations = ata_port_operations {
    inherits: &ata_bmdma_port_ops,
    scr_read: Some(uli_scr_read),
    scr_write: Some(uli_scr_write),
    hardreset: ATA_OP_NULL,
};

static ULI_PORT_INFO: ata_port_info = ata_port_info {
    flags: ATA_FLAG_SATA | ATA_FLAG_IGN_SIMPLEX,
    pio_mask: ATA_PIO4,
    udma_mask: ATA_UDMA6,
    port_ops: &ULI_OPS,
};

// MODULE_AUTHOR("Peer Chen");
// MODULE_DESCRIPTION("low-level driver for ULi Electronics SATA controller");
// MODULE_LICENSE("GPL");
// MODULE_DEVICE_TABLE(pci, uli_pci_tbl);
// MODULE_VERSION(DRV_VERSION);

unsafe fn get_scr_cfg_addr(ap: *mut ata_port, sc_reg: u32) -> u32 {
    let hpriv = (*(*ap).host).private_data as *mut uli_priv;
    (*hpriv).scr_cfg_addr[(*ap).port_no as usize].wrapping_add(4u32.wrapping_mul(sc_reg))
}

unsafe fn uli_scr_cfg_read(link: *mut ata_link, sc_reg: u32) -> u32 {
    let pdev = to_pci_dev((*(*link).ap).host.as_ref().unwrap().dev);
    let cfg_addr = get_scr_cfg_addr((*link).ap, sc_reg);
    let mut val: u32 = 0;
    pci_read_config_dword(pdev, cfg_addr, &mut val);
    val
}

unsafe fn uli_scr_cfg_write(link: *mut ata_link, scr: u32, val: u32) {
    let pdev = to_pci_dev((*(*link).ap).host.as_ref().unwrap().dev);
    let cfg_addr = get_scr_cfg_addr((*link).ap, scr);
    pci_write_config_dword(pdev, cfg_addr, val);
}

unsafe fn uli_scr_read(link: *mut ata_link, sc_reg: u32, val: *mut u32) -> i32 {
    if sc_reg > SCR_CONTROL {
        return -EINVAL;
    }
    *val = uli_scr_cfg_read(link, sc_reg);
    0
}

unsafe fn uli_scr_write(link: *mut ata_link, sc_reg: u32, val: u32) -> i32 {
    if sc_reg > SCR_CONTROL { // SCR_CONTROL=2, SCR_ERROR=1, SCR_STATUS=0
        return -EINVAL;
    }
    uli_scr_cfg_write(link, sc_reg, val);
    0
}

unsafe fn uli_init_one(pdev: *mut pci_dev, ent: *const pci_device_id) -> i32 {
    let ppi: [*const ata_port_info; 2] = [&ULI_PORT_INFO, core::ptr::null()];
    let board_idx = (*ent).driver_data as u32;
    let mut host: *mut ata_host;
    let hpriv: *mut uli_priv;
    let iomap: *const *mut core::ffi::c_void;
    let ioaddr: *mut ata_ioports;
    let mut n_ports: i32;
    let mut rc: i32;

    ata_print_version_once(&mut (*pdev).dev, DRV_VERSION);
    rc = pcim_enable_device(pdev);
    if rc != 0 { return rc; }

    n_ports = 2;
    if board_idx == uli_5287 { n_ports = 4; }

    host = ata_host_alloc_pinfo(&mut (*pdev).dev, ppi.as_ptr(), n_ports);
    if host.is_null() { return -ENOMEM; }
    hpriv = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<uli_priv>(), GFP_KERNEL) as *mut uli_priv;
    if hpriv.is_null() { return -ENOMEM; }
    (*host).private_data = hpriv as *mut core::ffi::c_void;

    rc = ata_pci_sff_init_host(host);
    if rc != 0 { return rc; }
    ata_pci_bmdma_init(host);
    iomap = (*host).iomap;

    match board_idx {
        uli_5287 => {
            (*hpriv).scr_cfg_addr[0] = ULI5287_BASE;
            (*hpriv).scr_cfg_addr[1] = ULI5287_BASE + ULI5287_OFFS;
            ioaddr = &mut (*(*host).ports.add(2)).ioaddr;
            (*ioaddr).cmd_addr = iomap.add(0).read().wrapping_add(8);
            (*ioaddr).altstatus_addr = ((iomap.add(1).read() as usize | ATA_PCI_CTL_OFS as usize) + 4) as *mut core::ffi::c_void;
            (*ioaddr).ctl_addr = (*ioaddr).altstatus_addr;
            (*ioaddr).bmdma_addr = iomap.add(4).read().wrapping_add(16);
            (*hpriv).scr_cfg_addr[2] = ULI5287_BASE + ULI5287_OFFS * 4;
            ata_sff_std_ports(ioaddr);
            ata_port_desc((*host).ports.add(2).read(), "cmd 0x%llx ctl 0x%llx bmdma 0x%llx", pci_resource_start(pdev, 0) + 8, (pci_resource_start(pdev, 1) | ATA_PCI_CTL_OFS as u64) + 4, pci_resource_start(pdev, 4) + 16);
            ioaddr = &mut (*(*host).ports.add(3)).ioaddr;
            (*ioaddr).cmd_addr = iomap.add(2).read().wrapping_add(8);
            (*ioaddr).altstatus_addr = ((iomap.add(3).read() as usize | ATA_PCI_CTL_OFS as usize) + 4) as *mut core::ffi::c_void;
            (*ioaddr).ctl_addr = (*ioaddr).altstatus_addr;
            (*ioaddr).bmdma_addr = iomap.add(4).read().wrapping_add(24);
            (*hpriv).scr_cfg_addr[3] = ULI5287_BASE + ULI5287_OFFS * 5;
            ata_sff_std_ports(ioaddr);
            ata_port_desc((*host).ports.add(2).read(), "cmd 0x%llx ctl 0x%llx bmdma 0x%llx", pci_resource_start(pdev, 2) + 9, (pci_resource_start(pdev, 3) | ATA_PCI_CTL_OFS as u64) + 4, pci_resource_start(pdev, 4) + 24);
        }
        uli_5289 => { (*hpriv).scr_cfg_addr[0] = ULI5287_BASE; (*hpriv).scr_cfg_addr[1] = ULI5287_BASE + ULI5287_OFFS; }
        uli_5281 => { (*hpriv).scr_cfg_addr[0] = ULI5281_BASE; (*hpriv).scr_cfg_addr[1] = ULI5281_BASE + ULI5281_OFFS; }
        _ => BUG(),
    }
    pci_set_master(pdev);
    pcim_intx(pdev, 1);
    ata_host_activate(host, (*pdev).irq, ata_bmdma_interrupt, IRQF_SHARED, &ULI_SHT)
}

// module_pci_driver(uli_pci_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
