// SPDX-License-Identifier: GPL-2.0+
/*
 * Driver for Loongson-2K BMC IPMI interface
 *
 * Copyright (C) 2024-2025 Loongson Technology Corporation Limited.
 *
 * Authors:
 *	Chong Qiao <qiaochong@loongson.cn>
 *	Binbin Zhou <zhoubinbin@loongson.cn>
 */

// Linux and ipmi_si.h declarations are supplied by the surrounding translation unit.

const LS2K_KCS_FIFO_IBFH: usize = 0x0;
const LS2K_KCS_FIFO_IBFT: usize = 0x1;
const LS2K_KCS_FIFO_OBFH: usize = 0x2;
const LS2K_KCS_FIFO_OBFT: usize = 0x3;

/* KCS registers */
const LS2K_KCS_REG_STS: usize = 0x4;
const LS2K_KCS_REG_DATA_OUT: usize = 0x5;
const LS2K_KCS_REG_DATA_IN: usize = 0x6;
const LS2K_KCS_REG_CMD: usize = 0x8;

const LS2K_KCS_CMD_DATA: usize = 0xa;
const LS2K_KCS_VERSION: usize = 0xb;
const LS2K_KCS_WR_REQ: usize = 0xc;
const LS2K_KCS_WR_ACK: usize = 0x10;

const LS2K_KCS_STS_OBF: u8 = 1 << 0;
const LS2K_KCS_STS_IBF: u8 = 1 << 1;
const LS2K_KCS_STS_SMS_ATN: u8 = 1 << 2;
const LS2K_KCS_STS_CMD: u8 = 1 << 3;
const LS2K_KCS_DATA_MASK: u8 = LS2K_KCS_STS_OBF | LS2K_KCS_STS_IBF | LS2K_KCS_STS_CMD;

static mut ls2k_registered: bool = false;

unsafe fn ls2k_mem_inb_v0(io: *const si_sm_io, offset: u32) -> u8 {
    let addr = (*io).addr;
    let reg_offset: usize;

    if offset & (1 << 0) != 0 {
        reg_offset = LS2K_KCS_REG_STS;
    } else {
        writeb(readb(addr.add(LS2K_KCS_REG_STS)) & !LS2K_KCS_STS_OBF,
               addr.add(LS2K_KCS_REG_STS));
        reg_offset = LS2K_KCS_REG_DATA_OUT;
    }

    readb(addr.add(reg_offset))
}

unsafe fn ls2k_mem_inb_v1(io: *const si_sm_io, offset: u32) -> u8 {
    let addr = (*io).addr;
    let mut inb: u8 = 0;

    let obf = readb(addr.add(LS2K_KCS_FIFO_OBFH)) ^ readb(addr.add(LS2K_KCS_FIFO_OBFT));
    let ibf = readb(addr.add(LS2K_KCS_FIFO_IBFH)) ^ readb(addr.add(LS2K_KCS_FIFO_IBFT));
    let cmd = readb(addr.add(LS2K_KCS_CMD_DATA));

    if offset & (1 << 0) != 0 {
        inb = readb(addr.add(LS2K_KCS_REG_STS)) & !LS2K_KCS_DATA_MASK;
        inb |= (obf & 1) | ((ibf & 1) << 1) | ((cmd & 1) << 3);
    } else {
        inb = readb(addr.add(LS2K_KCS_REG_DATA_OUT));
        writeb(readb(addr.add(LS2K_KCS_FIFO_OBFH)), addr.add(LS2K_KCS_FIFO_OBFT));
    }

    inb
}

unsafe fn ls2k_mem_outb_v0(io: *const si_sm_io, offset: u32, val: u8) {
    let addr = (*io).addr;
    let mut sts = readb(addr.add(LS2K_KCS_REG_STS));
    let reg_offset: usize;

    if sts & LS2K_KCS_STS_IBF != 0 { return; }

    if offset & (1 << 0) != 0 {
        reg_offset = LS2K_KCS_REG_CMD;
        sts |= LS2K_KCS_STS_CMD;
    } else {
        reg_offset = LS2K_KCS_REG_DATA_IN;
        sts &= !LS2K_KCS_STS_CMD;
    }

    writew(val as u16, addr.add(reg_offset));
    writeb(sts | LS2K_KCS_STS_IBF, addr.add(LS2K_KCS_REG_STS));
    writel(readl(addr.add(LS2K_KCS_WR_REQ)).wrapping_add(1), addr.add(LS2K_KCS_WR_REQ));
}

unsafe fn ls2k_mem_outb_v1(io: *const si_sm_io, offset: u32, val: u8) {
    let addr = (*io).addr;
    let ibfh = readb(addr.add(LS2K_KCS_FIFO_IBFH));
    let ibft = readb(addr.add(LS2K_KCS_FIFO_IBFT));
    if ibfh ^ ibft != 0 { return; }

    let reg_offset = if offset & (1 << 0) != 0 { LS2K_KCS_REG_CMD } else { LS2K_KCS_REG_DATA_IN };
    writew(val as u16, addr.add(reg_offset));
    writeb(if offset & (1 << 0) != 0 { 1 } else { 0 }, addr.add(LS2K_KCS_CMD_DATA));
    writeb(if ibft == 0 { 1 } else { 0 }, addr.add(LS2K_KCS_FIFO_IBFH));
    writel(readl(addr.add(LS2K_KCS_WR_REQ)).wrapping_add(1), addr.add(LS2K_KCS_WR_REQ));
}

unsafe fn ls2k_mem_cleanup(io: *mut si_sm_io) {
    if !(*io).addr.is_null() { iounmap((*io).addr); }
}

unsafe fn ipmi_ls2k_mem_setup(io: *mut si_sm_io) -> i32 {
    (*io).addr = ioremap((*io).addr_data, (*io).regspacing);
    if (*io).addr.is_null() { return -EIO; }
    let version = readb((*io).addr.add(LS2K_KCS_VERSION));
    (*io).inputb = if version != 0 { Some(ls2k_mem_inb_v1) } else { Some(ls2k_mem_inb_v0) };
    (*io).outputb = if version != 0 { Some(ls2k_mem_outb_v1) } else { Some(ls2k_mem_outb_v0) };
    (*io).io_cleanup = Some(ls2k_mem_cleanup);
    0
}

unsafe fn ipmi_ls2k_probe(pdev: *mut platform_device) -> i32 {
    let mut io: si_sm_io = core::mem::zeroed();
    io.si_info = &ipmi_kcs_si_info;
    io.io_setup = Some(ipmi_ls2k_mem_setup);
    io.addr_data = (*pdev).resource[0].start;
    io.regspacing = resource_size(&(*pdev).resource[0]);
    io.dev = &(*pdev).dev;
    dev_dbg(&(*pdev).dev, "addr 0x%lx, spacing %d.\n", io.addr_data, io.regspacing);
    ipmi_si_add_smi(&mut io)
}

unsafe fn ipmi_ls2k_remove(pdev: *mut platform_device) {
    ipmi_si_remove_by_dev(&(*pdev).dev);
}

static mut ipmi_ls2k_platform_driver: platform_driver = platform_driver {
    driver: driver { name: "ls2k-ipmi-si" },
    probe: Some(ipmi_ls2k_probe),
    remove: Some(ipmi_ls2k_remove),
};

pub unsafe fn ipmi_si_ls2k_init() {
    platform_driver_register(&mut ipmi_ls2k_platform_driver);
    ls2k_registered = true;
}

pub unsafe fn ipmi_si_ls2k_shutdown() {
    if ls2k_registered { platform_driver_unregister(&mut ipmi_ls2k_platform_driver); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
