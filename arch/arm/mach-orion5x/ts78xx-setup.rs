// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-orion5x/ts78xx-setup.c
 *
 * Maintainer: Alexander Clouter <alex@digriz.org.uk>
 */

// Dependency declarations supplied by the surrounding kernel translation.

const TS78XX_FPGA_REGS_PHYS_BASE: usize = 0xe8000000;
const TS78XX_FPGA_REGS_VIRT_BASE: usize = 0xff900000;
const TS78XX_FPGA_REGS_SIZE: usize = SZ_1M;

static mut ts78xx_fpga: ts78xx_fpga_data = ts78xx_fpga_data {
    id: 0,
    state: 1,
    // supports is populated by ts78xx_fpga_supports().
};

static mut ts78xx_io_desc: [map_desc; 1] = [map_desc {
    virtual_: TS78XX_FPGA_REGS_VIRT_BASE,
    pfn: __phys_to_pfn(TS78XX_FPGA_REGS_PHYS_BASE),
    length: TS78XX_FPGA_REGS_SIZE,
    type_: MT_DEVICE,
}];

unsafe fn ts78xx_map_io() {
    orion5x_map_io();
    iotable_init(ts78xx_io_desc.as_ptr(), ARRAY_SIZE(ts78xx_io_desc));
}

static mut ts78xx_eth_data: mv643xx_eth_platform_data = mv643xx_eth_platform_data {
    phy_addr: MV643XX_ETH_PHY_ADDR(0),
};

static mut ts78xx_sata_data: mv_sata_platform_data = mv_sata_platform_data {
    n_ports: 2,
};

const TS_RTC_CTRL: usize = TS78XX_FPGA_REGS_PHYS_BASE + 0x808;
const TS_RTC_DATA: usize = TS78XX_FPGA_REGS_PHYS_BASE + 0x80c;

static mut ts78xx_ts_rtc_resources: [resource; 2] = [
    DEFINE_RES_MEM(TS_RTC_CTRL, 0x01),
    DEFINE_RES_MEM(TS_RTC_DATA, 0x01),
];

static mut ts78xx_ts_rtc_device: platform_device = platform_device {
    name: c"rtc-m48t86",
    id: -1,
    resource: ts78xx_ts_rtc_resources.as_mut_ptr(),
    num_resources: ARRAY_SIZE(ts78xx_ts_rtc_resources),
};

unsafe fn ts78xx_ts_rtc_load() -> c_int {
    let rc;
    if ts78xx_fpga.supports.ts_rtc.init == 0 {
        rc = platform_device_register(&raw mut ts78xx_ts_rtc_device);
        if rc == 0 { ts78xx_fpga.supports.ts_rtc.init = 1; }
    } else {
        rc = platform_device_add(&raw mut ts78xx_ts_rtc_device);
    }
    if rc != 0 { pr_info!("RTC could not be registered: {}\n", rc); }
    rc
}

unsafe fn ts78xx_ts_rtc_unload() { platform_device_del(&raw mut ts78xx_ts_rtc_device); }

const TS_NAND_CTRL: usize = TS78XX_FPGA_REGS_VIRT_BASE + 0x800;
const TS_NAND_DATA: usize = TS78XX_FPGA_REGS_PHYS_BASE + 0x804;

unsafe fn ts78xx_ts_nand_cmd_ctrl(this: *mut nand_chip, cmd: c_int, ctrl: c_uint) {
    if ctrl & NAND_CTRL_CHANGE != 0 {
        let mut bits: u8 = ((ctrl & NAND_NCE) << 2) as u8;
        bits |= (ctrl & NAND_CLE) as u8;
        bits |= ((ctrl & NAND_ALE) >> 2) as u8;
        writeb((readb(TS_NAND_CTRL) & !0x7) | bits, TS_NAND_CTRL);
    }
    if cmd != NAND_CMD_NONE { writeb(cmd as u8, (*this).legacy.IO_ADDR_W); }
}

unsafe fn ts78xx_ts_nand_dev_ready(_chip: *mut nand_chip) -> c_int { (readb(TS_NAND_CTRL) & 0x20) as c_int }

unsafe fn ts78xx_ts_nand_write_buf(chip: *mut nand_chip, mut buf: *const u8, mut len: c_int) {
    let io_base = (*chip).legacy.IO_ADDR_W;
    let off = (buf as usize) & 3;
    if off != 0 { let sz = core::cmp::min(4 - off, len as usize); writesb(io_base, buf, sz as c_int); buf = buf.add(sz); len -= sz as c_int; }
    let sz = len >> 2;
    if sz != 0 { writesl(io_base, buf as *const u32, sz); buf = buf.add((sz << 2) as usize); len -= sz << 2; }
    if len != 0 { writesb(io_base, buf, len); }
}

unsafe fn ts78xx_ts_nand_read_buf(chip: *mut nand_chip, mut buf: *mut u8, mut len: c_int) {
    let io_base = (*chip).legacy.IO_ADDR_R;
    let off = (buf as usize) & 3;
    if off != 0 { let sz = core::cmp::min(4 - off, len as usize); readsb(io_base, buf, sz as c_int); buf = buf.add(sz); len -= sz as c_int; }
    let sz = len >> 2;
    if sz != 0 { readsl(io_base, buf as *mut u32, sz); buf = buf.add((sz << 2) as usize); len -= sz << 2; }
    if len != 0 { readsb(io_base, buf, len); }
}

static mut ts78xx_ts_nand_parts: [mtd_partition; 4] = [
    mtd_partition { name: c"mbr", offset: 0, size: SZ_128K, mask_flags: MTD_WRITEABLE },
    mtd_partition { name: c"kernel", offset: MTDPART_OFS_APPEND, size: SZ_4M, mask_flags: 0 },
    mtd_partition { name: c"initrd", offset: MTDPART_OFS_APPEND, size: SZ_4M, mask_flags: 0 },
    mtd_partition { name: c"rootfs", offset: MTDPART_OFS_APPEND, size: MTDPART_SIZ_FULL, mask_flags: 0 },
];

static mut ts78xx_ts_nand_data: platform_nand_data = platform_nand_data {
    chip: platform_nand_chip { nr_chips: 1, partitions: ts78xx_ts_nand_parts.as_mut_ptr(), nr_partitions: ARRAY_SIZE(ts78xx_ts_nand_parts), chip_delay: 15, bbt_options: NAND_BBT_USE_FLASH },
    ctrl: platform_nand_ctrl { cmd_ctrl: Some(ts78xx_ts_nand_cmd_ctrl), dev_ready: Some(ts78xx_ts_nand_dev_ready), write_buf: Some(ts78xx_ts_nand_write_buf), read_buf: Some(ts78xx_ts_nand_read_buf) },
};

static mut ts78xx_ts_nand_resources: resource = DEFINE_RES_MEM(TS_NAND_DATA, 4);
static mut ts78xx_ts_nand_device: platform_device = platform_device { name: c"gen_nand", id: -1, dev: device { platform_data: &raw mut ts78xx_ts_nand_data as *mut _ }, resource: &raw mut ts78xx_ts_nand_resources, num_resources: 1 };

unsafe fn ts78xx_ts_nand_load() -> c_int { let rc; if ts78xx_fpga.supports.ts_nand.init == 0 { rc = platform_device_register(&raw mut ts78xx_ts_nand_device); if rc == 0 { ts78xx_fpga.supports.ts_nand.init = 1; } } else { rc = platform_device_add(&raw mut ts78xx_ts_nand_device); } if rc != 0 { pr_info!("NAND could not be registered: {}\n", rc); } rc }
unsafe fn ts78xx_ts_nand_unload() { platform_device_del(&raw mut ts78xx_ts_nand_device); }

const TS_RNG_DATA: usize = TS78XX_FPGA_REGS_PHYS_BASE | 0x044;
static mut ts78xx_ts_rng_resource: resource = DEFINE_RES_MEM(TS_RNG_DATA, 4);
static mut ts78xx_ts_rng_data: timeriomem_rng_data = timeriomem_rng_data { period: 1000000 };
static mut ts78xx_ts_rng_device: platform_device = platform_device { name: c"timeriomem_rng", id: -1, dev: device { platform_data: &raw mut ts78xx_ts_rng_data as *mut _ }, resource: &raw mut ts78xx_ts_rng_resource, num_resources: 1 };
unsafe fn ts78xx_ts_rng_load() -> c_int { let rc; if ts78xx_fpga.supports.ts_rng.init == 0 { rc = platform_device_register(&raw mut ts78xx_ts_rng_device); if rc == 0 { ts78xx_fpga.supports.ts_rng.init = 1; } } else { rc = platform_device_add(&raw mut ts78xx_ts_rng_device); } if rc != 0 { pr_info!("RNG could not be registered: {}\n", rc); } rc }
unsafe fn ts78xx_ts_rng_unload() { platform_device_del(&raw mut ts78xx_ts_rng_device); }

unsafe fn ts78xx_fpga_devices_zero_init() { ts78xx_fpga.supports.ts_rtc.init = 0; ts78xx_fpga.supports.ts_nand.init = 0; ts78xx_fpga.supports.ts_rng.init = 0; }

unsafe fn ts78xx_fpga_supports() {
    // TODO: put this 'table' into ts78xx-fpga.h
    match ts78xx_fpga.id {
        TS7800_REV_1..=TS7800_REV_9 => { ts78xx_fpga.supports.ts_rtc.present = 1; ts78xx_fpga.supports.ts_nand.present = 1; ts78xx_fpga.supports.ts_rng.present = 1; }
        _ => match (ts78xx_fpga.id >> 8) & 0xffffff {
            TS7800_FPGA_MAGIC => { pr_warn!("unrecognised FPGA revision 0x{:02x}\n", ts78xx_fpga.id & 0xff); ts78xx_fpga.supports.ts_rtc.present = 1; ts78xx_fpga.supports.ts_nand.present = 1; ts78xx_fpga.supports.ts_rng.present = 1; }
            _ => { ts78xx_fpga.supports.ts_rtc.present = 0; ts78xx_fpga.supports.ts_nand.present = 0; ts78xx_fpga.supports.ts_rng.present = 0; }
        }
    }
}

unsafe fn ts78xx_fpga_load_devices() -> c_int { let mut ret = 0; if ts78xx_fpga.supports.ts_rtc.present == 1 { let tmp = ts78xx_ts_rtc_load(); if tmp != 0 { ts78xx_fpga.supports.ts_rtc.present = 0; } ret |= tmp; } if ts78xx_fpga.supports.ts_nand.present == 1 { let tmp = ts78xx_ts_nand_load(); if tmp != 0 { ts78xx_fpga.supports.ts_nand.present = 0; } ret |= tmp; } if ts78xx_fpga.supports.ts_rng.present == 1 { let tmp = ts78xx_ts_rng_load(); if tmp != 0 { ts78xx_fpga.supports.ts_rng.present = 0; } ret |= tmp; } ret }
unsafe fn ts78xx_fpga_unload_devices() -> c_int { if ts78xx_fpga.supports.ts_rtc.present == 1 { ts78xx_ts_rtc_unload(); } if ts78xx_fpga.supports.ts_nand.present == 1 { ts78xx_ts_nand_unload(); } if ts78xx_fpga.supports.ts_rng.present == 1 { ts78xx_ts_rng_unload(); } 0 }

unsafe fn ts78xx_fpga_load() -> c_int { ts78xx_fpga.id = readl(TS78XX_FPGA_REGS_VIRT_BASE); pr_info!("FPGA magic=0x{:06x}, rev=0x{:02x}\n", (ts78xx_fpga.id >> 8) & 0xffffff, ts78xx_fpga.id & 0xff); ts78xx_fpga_supports(); if ts78xx_fpga_load_devices() != 0 { ts78xx_fpga.state = -1; return -EBUSY; } 0 }
unsafe fn ts78xx_fpga_unload() -> c_int { let fpga_id = readl(TS78XX_FPGA_REGS_VIRT_BASE); if ts78xx_fpga.id != fpga_id { pr_err!("FPGA magic/rev mismatch\nTS-78xx FPGA: was 0x{:06x}/{:02x} but now 0x{:06x}/{:02x}\n", (ts78xx_fpga.id >> 8) & 0xffffff, ts78xx_fpga.id & 0xff, (fpga_id >> 8) & 0xffffff, fpga_id & 0xff); ts78xx_fpga.state = -1; return -EBUSY; } if ts78xx_fpga_unload_devices() != 0 { ts78xx_fpga.state = -1; return -EBUSY; } 0 }

unsafe fn ts78xx_fpga_show(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *mut c_char) -> ssize_t { if ts78xx_fpga.state < 0 { return sprintf(buf, c"borked\n"); } sprintf(buf, c"%s\n", if ts78xx_fpga.state != 0 { c"online" } else { c"offline" }) }
unsafe fn ts78xx_fpga_store(_kobj: *mut kobject, _attr: *mut kobj_attribute, buf: *const c_char, n: size_t) -> ssize_t { if ts78xx_fpga.state < 0 { pr_err!("FPGA borked, you must powercycle ASAP\n"); return -EBUSY as ssize_t; } let value = if strncmp(buf, c"online", 6) == 0 { 1 } else if strncmp(buf, c"offline", 7) == 0 { 0 } else { return -EINVAL as ssize_t; }; if ts78xx_fpga.state == value { return n as ssize_t; } let ret = if ts78xx_fpga.state == 0 { ts78xx_fpga_load() } else { ts78xx_fpga_unload() }; if ret >= 0 { ts78xx_fpga.state = value; } n as ssize_t }

static mut ts78xx_fpga_attr: kobj_attribute = __ATTR(ts78xx_fpga, 0o644, ts78xx_fpga_show, ts78xx_fpga_store);

static mut ts78xx_mpp_modes: [u32; 27] = [MPP0_UNUSED, MPP1_GPIO, MPP2_GPIO, MPP3_GPIO, MPP4_GPIO, MPP5_GPIO, MPP6_GPIO, MPP7_GPIO, MPP8_UNUSED, MPP9_UNUSED, MPP10_UNUSED, MPP11_UNUSED, MPP12_UNUSED, MPP13_UNUSED, MPP14_UNUSED, MPP15_UNUSED, MPP16_UART, MPP17_UART, MPP18_UART, MPP19_UART, 0, 0, 0, 0, 0, 0, 0];

unsafe fn ts78xx_init() {
    orion5x_init();
    orion5x_mpp_conf(ts78xx_mpp_modes.as_ptr());
    orion5x_ehci0_init(); orion5x_ehci1_init(); orion5x_eth_init(&raw mut ts78xx_eth_data); orion5x_sata_init(&raw mut ts78xx_sata_data); orion5x_uart0_init(); orion5x_uart1_init(); orion5x_xor_init();
    ts78xx_fpga_devices_zero_init();
    let ret = ts78xx_fpga_load();
    let ret = sysfs_create_file(firmware_kobj, &raw mut ts78xx_fpga_attr.attr);
    if ret != 0 { pr_err!("sysfs_create_file failed: {}\n", ret); }
}

MACHINE_START!(TS78XX, "Technologic Systems TS-78xx SBC", {
    // Maintainer: Alexander Clouter <alex@digriz.org.uk>
    atag_offset: 0x100,
    nr_irqs: ORION5X_NR_IRQS,
    init_machine: ts78xx_init,
    map_io: ts78xx_map_io,
    init_early: orion5x_init_early,
    init_irq: orion5x_init_irq,
    init_time: orion5x_timer_init,
    restart: orion5x_restart,
});

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
