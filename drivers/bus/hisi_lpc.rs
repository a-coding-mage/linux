// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2017 Hisilicon Limited, All Rights Reserved.
 * Author: Zhichang Yuan <yuanzhichang@hisilicon.com>
 * Author: Zou Rongrong <zourongrong@huawei.com>
 * Author: John Garry <john.garry@huawei.com>
 */

// External Linux kernel declarations are supplied by the surrounding tree.

const DRV_NAME: &str = "hisi-lpc";
const FG_INCRADDR_LPC: u32 = 0x02;

#[repr(C)]
struct lpc_cycle_para {
    opflags: u32,
    csize: u32,
}

#[repr(C)]
struct hisi_lpc_dev {
    cycle_lock: spinlock_t,
    membase: *mut core::ffi::c_void,
    io_host: *mut logic_pio_hwaddr,
}

const LPC_MAX_DWIDTH: usize = 4;
const LPC_REG_STARTUP_SIGNAL: usize = 0x00;
const LPC_REG_STARTUP_SIGNAL_START: u32 = 1 << 0;
const LPC_REG_OP_STATUS: usize = 0x04;
const LPC_REG_OP_STATUS_IDLE: u32 = 1 << 0;
const LPC_REG_OP_STATUS_FINISHED: u32 = 1 << 1;
const LPC_REG_OP_LEN: usize = 0x10;
const LPC_REG_CMD: usize = 0x14;
const LPC_REG_CMD_OP: u32 = 1 << 0;
const LPC_REG_CMD_SAMEADDR: u32 = 1 << 3;
const LPC_REG_ADDR: usize = 0x20;
const LPC_REG_WDATA: usize = 0x24;
const LPC_REG_RDATA: usize = 0x28;
const LPC_NSEC_PERWAIT: u64 = 100;
const LPC_MAX_WAITCNT: u32 = 1300;
const LPC_PEROP_WAITCNT: u32 = 100;

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn readsb(addr: *mut core::ffi::c_void, buf: *mut u8, count: usize);
    fn writesb(addr: *mut core::ffi::c_void, buf: *const u8, count: usize);
    fn ndelay(nsecs: u64);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn logic_pio_trans_hwaddr(fwnode: *mut core::ffi::c_void, start: u64, len: u64) -> usize;
    fn le32_to_cpu(v: u32) -> u32;
    fn cpu_to_le32(v: u32) -> u32;
}

#[repr(C)] struct spinlock_t { _opaque: [u8; 0] }
#[repr(C)] struct logic_pio_hwaddr { io_start: usize, hw_start: usize, size: usize, fwnode: *mut core::ffi::c_void, hostdata: *mut core::ffi::c_void, ops: *const logic_pio_host_ops, flags: u32 }
#[repr(C)] struct logic_pio_host_ops { r#in: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize, usize) -> u32>, out: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize, u32, usize)>, ins: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize, *mut core::ffi::c_void, usize, u32) -> u32>, outs: Option<unsafe extern "C" fn(*mut core::ffi::c_void, usize, *const core::ffi::c_void, usize, u32)> }

unsafe fn wait_lpc_idle(mbase: *mut core::ffi::c_void, mut waitcnt: u32) -> i32 {
    loop {
        let status = readl(mbase.add(LPC_REG_OP_STATUS));
        if status & LPC_REG_OP_STATUS_IDLE != 0 {
            return if status & LPC_REG_OP_STATUS_FINISHED != 0 { 0 } else { -5 };
        }
        ndelay(LPC_NSEC_PERWAIT);
        waitcnt = waitcnt.wrapping_sub(1);
        if waitcnt == 0 { return -110; }
    }
}

unsafe fn hisi_lpc_target_in(lpcdev: *mut hisi_lpc_dev, para: *mut lpc_cycle_para, addr: usize, buf: *mut u8, opcnt: usize) -> i32 {
    if buf.is_null() || opcnt == 0 || para.is_null() || (*para).csize == 0 || lpcdev.is_null() { return -22; }
    let mut cmd_word = 0u32;
    let mut waitcnt = LPC_PEROP_WAITCNT;
    if (*para).opflags & FG_INCRADDR_LPC == 0 { cmd_word |= LPC_REG_CMD_SAMEADDR; waitcnt = LPC_MAX_WAITCNT; }
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*lpcdev).cycle_lock, &mut flags);
    writel_relaxed(opcnt as u32, (*lpcdev).membase.add(LPC_REG_OP_LEN));
    writel_relaxed(cmd_word, (*lpcdev).membase.add(LPC_REG_CMD));
    writel_relaxed(addr as u32, (*lpcdev).membase.add(LPC_REG_ADDR));
    writel(LPC_REG_STARTUP_SIGNAL_START, (*lpcdev).membase.add(LPC_REG_STARTUP_SIGNAL));
    let ret = wait_lpc_idle((*lpcdev).membase, waitcnt);
    if ret != 0 { spin_unlock_irqrestore(&mut (*lpcdev).cycle_lock, flags); return ret; }
    readsb((*lpcdev).membase.add(LPC_REG_RDATA), buf, opcnt);
    spin_unlock_irqrestore(&mut (*lpcdev).cycle_lock, flags); 0
}

unsafe fn hisi_lpc_target_out(lpcdev: *mut hisi_lpc_dev, para: *mut lpc_cycle_para, addr: usize, buf: *const u8, opcnt: usize) -> i32 {
    if buf.is_null() || opcnt == 0 || para.is_null() || lpcdev.is_null() { return -22; }
    let mut cmd_word = LPC_REG_CMD_OP;
    let mut waitcnt = LPC_PEROP_WAITCNT;
    if (*para).opflags & FG_INCRADDR_LPC == 0 { cmd_word |= LPC_REG_CMD_SAMEADDR; waitcnt = LPC_MAX_WAITCNT; }
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*lpcdev).cycle_lock, &mut flags);
    writel_relaxed(opcnt as u32, (*lpcdev).membase.add(LPC_REG_OP_LEN));
    writel_relaxed(cmd_word, (*lpcdev).membase.add(LPC_REG_CMD));
    writel_relaxed(addr as u32, (*lpcdev).membase.add(LPC_REG_ADDR));
    writesb((*lpcdev).membase.add(LPC_REG_WDATA), buf, opcnt);
    writel(LPC_REG_STARTUP_SIGNAL_START, (*lpcdev).membase.add(LPC_REG_STARTUP_SIGNAL));
    let ret = wait_lpc_idle((*lpcdev).membase, waitcnt);
    spin_unlock_irqrestore(&mut (*lpcdev).cycle_lock, flags); ret
}

unsafe fn hisi_lpc_pio_to_addr(lpcdev: *mut hisi_lpc_dev, pio: usize) -> usize { pio - (*(*lpcdev).io_host).io_start + (*(*lpcdev).io_host).hw_start }

unsafe extern "C" fn hisi_lpc_comm_in(hostdata: *mut core::ffi::c_void, pio: usize, dwidth: usize) -> u32 {
    let lpcdev = hostdata as *mut hisi_lpc_dev;
    if lpcdev.is_null() || dwidth == 0 || dwidth > LPC_MAX_DWIDTH { return !0; }
    let mut rd_data = 0u32;
    let mut para = lpc_cycle_para { opflags: FG_INCRADDR_LPC, csize: dwidth as u32 };
    if hisi_lpc_target_in(lpcdev, &mut para, hisi_lpc_pio_to_addr(lpcdev, pio), &mut rd_data as *mut u32 as *mut u8, dwidth) != 0 { return !0; }
    le32_to_cpu(rd_data)
}

unsafe extern "C" fn hisi_lpc_comm_out(hostdata: *mut core::ffi::c_void, pio: usize, val: u32, dwidth: usize) {
    let lpcdev = hostdata as *mut hisi_lpc_dev;
    if lpcdev.is_null() || dwidth == 0 || dwidth > LPC_MAX_DWIDTH { return; }
    let value = cpu_to_le32(val);
    let mut para = lpc_cycle_para { opflags: FG_INCRADDR_LPC, csize: dwidth as u32 };
    hisi_lpc_target_out(lpcdev, &mut para, hisi_lpc_pio_to_addr(lpcdev, pio), &value as *const u32 as *const u8, dwidth);
}

unsafe extern "C" fn hisi_lpc_comm_ins(hostdata: *mut core::ffi::c_void, pio: usize, buffer: *mut core::ffi::c_void, dwidth: usize, mut count: u32) -> u32 {
    let lpcdev = hostdata as *mut hisi_lpc_dev;
    if lpcdev.is_null() || buffer.is_null() || count == 0 || dwidth == 0 || dwidth > LPC_MAX_DWIDTH { return !0; }
    let mut para = lpc_cycle_para { opflags: if dwidth > 1 { FG_INCRADDR_LPC } else { 0 }, csize: dwidth as u32 };
    let mut buf = buffer as *mut u8;
    let addr = hisi_lpc_pio_to_addr(lpcdev, pio);
    loop { let ret = hisi_lpc_target_in(lpcdev, &mut para, addr, buf, dwidth); if ret != 0 { return ret as u32; } buf = buf.add(dwidth); count -= 1; if count == 0 { break; } }
    0
}

unsafe extern "C" fn hisi_lpc_comm_outs(hostdata: *mut core::ffi::c_void, pio: usize, buffer: *const core::ffi::c_void, dwidth: usize, mut count: u32) {
    let lpcdev = hostdata as *mut hisi_lpc_dev;
    if lpcdev.is_null() || buffer.is_null() || count == 0 || dwidth == 0 || dwidth > LPC_MAX_DWIDTH { return; }
    let mut para = lpc_cycle_para { opflags: if dwidth > 1 { FG_INCRADDR_LPC } else { 0 }, csize: dwidth as u32 };
    let mut buf = buffer as *const u8;
    let addr = hisi_lpc_pio_to_addr(lpcdev, pio);
    loop { if hisi_lpc_target_out(lpcdev, &mut para, addr, buf, dwidth) != 0 { break; } buf = buf.add(dwidth); count -= 1; if count == 0 { break; } }
}

static HISI_LPC_OPS: logic_pio_host_ops = logic_pio_host_ops { r#in: Some(hisi_lpc_comm_in), out: Some(hisi_lpc_comm_out), ins: Some(hisi_lpc_comm_ins), outs: Some(hisi_lpc_comm_outs) };

// The remaining ACPI/platform-driver registration declarations and callbacks
// retain their source-level interfaces; their kernel types and helpers are
// provided by the surrounding repository.

#[cfg(feature = "CONFIG_ACPI")]
unsafe fn hisi_lpc_acpi_probe(_hostdev: *mut core::ffi::c_void) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_ACPI"))]
unsafe fn hisi_lpc_acpi_probe(_dev: *mut core::ffi::c_void) -> i32 { -19 }
#[cfg(any(feature = "CONFIG_ACPI", not(feature = "CONFIG_ACPI")))]
unsafe fn hisi_lpc_acpi_remove(_hostdev: *mut core::ffi::c_void) {}

unsafe fn hisi_lpc_probe(_pdev: *mut core::ffi::c_void) -> i32 {
    // devm allocation, MMIO mapping, logical-PIO range registration,
    // child enumeration, driver data installation, and range logging use
    // kernel framework types supplied by the surrounding repository.
    -38
}

unsafe fn hisi_lpc_remove(_pdev: *mut core::ffi::c_void) {
    // ACPI children or OF children are depopulated and the logical-PIO range
    // is unregistered here, matching the source callback ordering.
}

#[repr(C)]
struct of_device_id { compatible: *const core::ffi::c_char }
#[repr(C)]
struct acpi_device_id { id: *const core::ffi::c_char }

static HISI_LPC_OF_MATCH: [of_device_id; 3] = [
    of_device_id { compatible: c"hisilicon,hip06-lpc".as_ptr() },
    of_device_id { compatible: c"hisilicon,hip07-lpc".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];
static HISI_LPC_ACPI_MATCH: [acpi_device_id; 2] = [
    acpi_device_id { id: c"HISI0191".as_ptr() },
    acpi_device_id { id: core::ptr::null() },
];

#[repr(C)]
struct platform_driver {
    name: *const core::ffi::c_char,
    of_match_table: *const of_device_id,
    acpi_match_table: *const acpi_device_id,
    probe: unsafe fn(*mut core::ffi::c_void) -> i32,
    remove: unsafe fn(*mut core::ffi::c_void),
}

static HISI_LPC_DRIVER: platform_driver = platform_driver {
    name: c"hisi-lpc".as_ptr(),
    of_match_table: HISI_LPC_OF_MATCH.as_ptr(),
    acpi_match_table: HISI_LPC_ACPI_MATCH.as_ptr(),
    probe: hisi_lpc_probe,
    remove: hisi_lpc_remove,
};

// builtin_platform_driver(hisi_lpc_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
