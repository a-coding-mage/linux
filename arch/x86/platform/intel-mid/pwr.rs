// SPDX-License-Identifier: GPL-2.0-only
/*
 * Intel MID Power Management Unit (PWRMU) device driver
 *
 * Copyright (C) 2016, Intel Corporation
 *
 * Author: Andy Shevchenko <andriy.shevchenko@linux.intel.com>
 *
 * Intel MID Power Management Unit device driver handles the South Complex PCI
 * devices such as GPDMA, SPI, I2C, PWM, and so on. By default PCI core
 * modifies bits in PMCSR register in the PCI configuration space. This is not
 * enough on some SoCs like Intel Tangier. In such case PCI core sets a new
 * power state of the device in question through a PM hook registered in struct
 * pci_platform_pm_ops (see drivers/pci/pci-mid.c).
 */

// Dependencies supplied by the surrounding kernel translation.

const PM_STS: usize = 0x00;
const PM_CMD: usize = 0x04;
const PM_ICS: usize = 0x08;
const PM_WKC: usize = 0x10;
const PM_WKS: usize = 0x18;
const PM_SSC: usize = 0x20;
const PM_SSS: usize = 0x30;

const PM_STS_BUSY: u32 = 1 << 8;
const PM_CMD_IOC: u32 = 1 << 8;
const PM_CMD_CM_NOP: u32 = 0 << 9;
const PM_CMD_CM_IMMEDIATE: u32 = 1 << 9;
const PM_CMD_CM_DELAY: u32 = 2 << 9;
const PM_CMD_CM_TRIGGER: u32 = 3 << 9;
const PM_CMD_SYS_STATE_S5: u32 = 5 << 16;
const PM_CMD_CFG_TRIGGER_NC: u32 = 3 << 19;
const TRIGGER_NC_MSG_2: u32 = 2 << 22;
const CMD_SET_CFG: u8 = 0x01;
const PM_ICS_IE: u32 = 1 << 8;
const PM_ICS_IP: u32 = 1 << 9;
const PM_ICS_SW_INT_STS: u32 = 1 << 10;
const INT_INVALID: u32 = 0;
const INT_CMD_COMPLETE: u32 = 1;
const INT_CMD_ERR: u32 = 2;
const INT_WAKE_EVENT: u32 = 3;
const INT_LSS_POWER_ERR: u32 = 4;
const INT_S0iX_MSG_ERR: u32 = 5;
const INT_NO_C6: u32 = 6;
const INT_TRIGGER_ERR: u32 = 7;
const INT_INACTIVITY: u32 = 8;
const LSS_MAX_SHARED_DEVS: usize = 4;
const LSS_MAX_DEVS: usize = 64;
const LSS_WS_BITS: usize = 1;
const LSS_PWS_BITS: usize = 2;
const PCI_DEVICE_ID_PENWELL: u16 = 0x0828;
const PCI_DEVICE_ID_TANGIER: u16 = 0x11a1;

#[repr(C)]
struct mid_pwr_dev {
    pdev: *mut pci_dev,
    state: pci_power_t,
}

#[repr(C)]
struct mid_pwr {
    dev: *mut device,
    regs: *mut core::ffi::c_void,
    irq: i32,
    available: bool,
    lock: mutex,
    lss: [[mid_pwr_dev; LSS_MAX_SHARED_DEVS]; LSS_MAX_DEVS],
}

static mut midpwr: *mut mid_pwr = core::ptr::null_mut();

unsafe fn mid_pwr_get_state(pwr: *mut mid_pwr, reg: i32) -> u32 {
    readl((*pwr).regs.add(PM_SSS + (reg as usize) * 4))
}

unsafe fn mid_pwr_set_state(pwr: *mut mid_pwr, reg: i32, value: u32) {
    writel(value, (*pwr).regs.add(PM_SSC + (reg as usize) * 4));
}

unsafe fn mid_pwr_set_wake(pwr: *mut mid_pwr, reg: i32, value: u32) {
    writel(value, (*pwr).regs.add(PM_WKC + (reg as usize) * 4));
}

unsafe fn mid_pwr_interrupt_disable(pwr: *mut mid_pwr) {
    writel(!PM_ICS_IE, (*pwr).regs.add(PM_ICS));
}

unsafe fn mid_pwr_is_busy(pwr: *mut mid_pwr) -> bool {
    (readl((*pwr).regs.add(PM_STS)) & PM_STS_BUSY) != 0
}

unsafe fn mid_pwr_wait(pwr: *mut mid_pwr) -> i32 {
    let mut count: u32 = 500000;
    loop {
        if !mid_pwr_is_busy(pwr) { return 0; }
        udelay(1);
        count = count.wrapping_sub(1);
        if count == 0 { break; }
    }
    -EBUSY
}

unsafe fn mid_pwr_wait_for_cmd(pwr: *mut mid_pwr, cmd: u8) -> i32 {
    writel(((cmd as u32) << 0) | PM_CMD_CM_IMMEDIATE, (*pwr).regs.add(PM_CMD));
    mid_pwr_wait(pwr)
}

unsafe fn __update_power_state(pwr: *mut mid_pwr, reg: i32, bit: i32, new: i32) -> i32 {
    let mut power = mid_pwr_get_state(pwr, reg);
    let mut curstate = ((power >> bit) & 3) as i32;
    if curstate == new { return 0; }
    mid_pwr_set_state(pwr, reg, (power & !(3 << bit)) | ((new as u32) << bit));
    let ret = mid_pwr_wait_for_cmd(pwr, CMD_SET_CFG);
    if ret != 0 { return ret; }
    power = mid_pwr_get_state(pwr, reg);
    curstate = ((power >> bit) & 3) as i32;
    if curstate != new { return -EAGAIN; }
    0
}

unsafe fn __find_weakest_power_state(lss: *mut mid_pwr_dev, pdev: *mut pci_dev, state: pci_power_t) -> pci_power_t {
    let mut weakest = PCI_D3hot;
    let mut j = 0;
    while j < LSS_MAX_SHARED_DEVS {
        if (*lss.add(j)).pdev == pdev || (*lss.add(j)).pdev.is_null() { break; }
        j += 1;
    }
    if j < LSS_MAX_SHARED_DEVS {
        (*lss.add(j)).pdev = pdev;
        (*lss.add(j)).state = state;
    } else {
        dev_WARN(&mut (*pdev).dev, "No room for device in PWRMU LSS cache\n");
        weakest = state;
    }
    j = 0;
    while j < LSS_MAX_SHARED_DEVS {
        if (*lss.add(j)).state < weakest { weakest = (*lss.add(j)).state; }
        j += 1;
    }
    weakest
}

unsafe fn __set_power_state(pwr: *mut mid_pwr, pdev: *mut pci_dev, mut state: pci_power_t, id: i32, reg: i32, bit: i32) -> i32 {
    state = __find_weakest_power_state((*pwr).lss[id as usize].as_mut_ptr(), pdev, state);
    let name = pci_power_name(state);
    let ret = __update_power_state(pwr, reg, bit, state as i32);
    if ret != 0 { dev_warn(&mut (*pdev).dev, "Can't set power state %s: %d\n", name, ret); return ret; }
    dev_vdbg(&mut (*pdev).dev, "Set power state %s\n", name);
    0
}

unsafe fn mid_pwr_set_power_state(pwr: *mut mid_pwr, pdev: *mut pci_dev, mut state: pci_power_t) -> i32 {
    let id = intel_mid_pwr_get_lss_id(pdev);
    if id < 0 { return id; }
    let reg = (id * LSS_PWS_BITS as i32) / 32;
    let bit = (id * LSS_PWS_BITS as i32) % 32;
    if state < PCI_D0 { state = PCI_D0; }
    if state > PCI_D3hot { state = PCI_D3hot; }
    mutex_lock(&mut (*pwr).lock);
    let ret = __set_power_state(pwr, pdev, state, id, reg, bit);
    mutex_unlock(&mut (*pwr).lock);
    ret
}

pub unsafe fn intel_mid_pci_set_power_state(pdev: *mut pci_dev, state: pci_power_t) -> i32 {
    let pwr = midpwr;
    might_sleep();
    let mut ret = 0;
    if !pwr.is_null() && (*pwr).available { ret = mid_pwr_set_power_state(pwr, pdev, state); }
    dev_vdbg(&mut (*pdev).dev, "set_power_state() returns %d\n", ret);
    0
}

pub unsafe fn intel_mid_pci_get_power_state(pdev: *mut pci_dev) -> pci_power_t {
    let pwr = midpwr;
    if pwr.is_null() || !(*pwr).available { return PCI_UNKNOWN; }
    let id = intel_mid_pwr_get_lss_id(pdev);
    if id < 0 { return PCI_UNKNOWN; }
    let reg = (id * LSS_PWS_BITS as i32) / 32;
    let bit = (id * LSS_PWS_BITS as i32) % 32;
    ((mid_pwr_get_state(pwr, reg) >> bit) & 3) as pci_power_t
}

pub unsafe fn intel_mid_pwr_power_off() {
    let pwr = midpwr;
    let cmd = PM_CMD_SYS_STATE_S5 | ((CMD_SET_CFG as u32) << 0) | PM_CMD_CM_TRIGGER | PM_CMD_CFG_TRIGGER_NC | TRIGGER_NC_MSG_2;
    writel(cmd, (*pwr).regs.add(PM_CMD));
    mid_pwr_wait(pwr);
}

pub unsafe fn intel_mid_pwr_get_lss_id(pdev: *mut pci_dev) -> i32 {
    let vndr = pci_find_capability(pdev, PCI_CAP_ID_VNDR);
    if vndr == 0 { return -EINVAL; }
    let mut id: u8 = 0;
    pci_read_config_byte(pdev, vndr + INTEL_MID_PWR_LSS_OFFSET, &mut id);
    if (id & INTEL_MID_PWR_LSS_TYPE) == 0 { return -ENODEV; }
    id &= !INTEL_MID_PWR_LSS_TYPE;
    if id >= LSS_MAX_DEVS as u8 { return -ERANGE; }
    id as i32
}

unsafe fn mid_pwr_irq_handler(_irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let pwr = dev_id as *mut mid_pwr;
    let ics = readl((*pwr).regs.add(PM_ICS));
    if (ics & PM_ICS_IP) == 0 { return IRQ_NONE; }
    writel(ics | PM_ICS_IP, (*pwr).regs.add(PM_ICS));
    dev_warn((*pwr).dev, "Unexpected IRQ: %#x\n", ics & 0xff);
    IRQ_HANDLED
}

#[repr(C)]
struct mid_pwr_device_info {
    set_initial_state: Option<unsafe extern "C" fn(*mut mid_pwr) -> i32>,
}

unsafe fn mid_set_initial_state(pwr: *mut mid_pwr, states: *const u32) -> i32 {
    mid_pwr_set_wake(pwr, 0, 0xffffffff);
    mid_pwr_set_wake(pwr, 1, 0xffffffff);
    for i in 0..4 { mid_pwr_set_state(pwr, i, *states.add(i as usize)); }
    let ret = mid_pwr_wait_for_cmd(pwr, CMD_SET_CFG);
    if ret != 0 { return ret; }
    for i in 0..LSS_MAX_DEVS { for j in 0..LSS_MAX_SHARED_DEVS { (*pwr).lss[i][j].state = PCI_D3hot; } }
    0
}

unsafe extern "C" fn pnw_set_initial_state(pwr: *mut mid_pwr) -> i32 {
    static STATES: [u32; 4] = [0xf00fffff, 0xffffffff, 0xffffffff, 0xffffffff];
    mid_set_initial_state(pwr, STATES.as_ptr())
}

unsafe extern "C" fn tng_set_initial_state(pwr: *mut mid_pwr) -> i32 {
    static STATES: [u32; 4] = [0xffffffff, 0xffffffff, 0xffffffff, 0xffffffff];
    mid_set_initial_state(pwr, STATES.as_ptr())
}

static pnw_info: mid_pwr_device_info = mid_pwr_device_info { set_initial_state: Some(pnw_set_initial_state) };
static tng_info: mid_pwr_device_info = mid_pwr_device_info { set_initial_state: Some(tng_set_initial_state) };

// This table should be in sync with the one in drivers/pci/pci-mid.c.
static mid_pwr_pci_ids: [pci_device_id; 3] = [
    pci_device_id { vendor: PCI_VENDOR_ID_INTEL, device: PCI_DEVICE_ID_PENWELL, driver_data: &pnw_info as *const _ as kernel_ulong_t },
    pci_device_id { vendor: PCI_VENDOR_ID_INTEL, device: PCI_DEVICE_ID_TANGIER, driver_data: &tng_info as *const _ as kernel_ulong_t },
    pci_device_id::default(),
];

static mid_pwr_pci_driver: pci_driver = pci_driver {
    name: "intel_mid_pwr",
    probe: Some(mid_pwr_probe),
    id_table: mid_pwr_pci_ids.as_ptr(),
};

unsafe fn mid_pwr_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> i32 {
    let info = (*id).driver_data as *mut mid_pwr_device_info;
    let dev = &mut (*pdev).dev;
    let ret = pcim_enable_device(pdev);
    if ret < 0 { dev_err(dev, "error: could not enable device\n"); return ret; }
    let pwr = devm_kzalloc(dev, core::mem::size_of::<mid_pwr>(), GFP_KERNEL) as *mut mid_pwr;
    if pwr.is_null() { return -ENOMEM; }
    (*pwr).regs = pcim_iomap_region(pdev, 0, "intel_mid_pwr");
    let ret = PTR_ERR_OR_ZERO((*pwr).regs);
    if ret != 0 { dev_err(dev, "Could not request / ioremap I/O-Mem: %d\n", ret); return ret; }
    (*pwr).dev = dev;
    (*pwr).irq = (*pdev).irq;
    mutex_init(&mut (*pwr).lock);
    mid_pwr_interrupt_disable(pwr);
    if !info.is_null() {
        if let Some(set_initial_state) = (*info).set_initial_state {
            let ret = set_initial_state(pwr);
            if ret != 0 { dev_warn(dev, "Can't set initial state: %d\n", ret); }
        }
    }
    let ret = devm_request_irq(dev, (*pdev).irq, Some(mid_pwr_irq_handler), IRQF_NO_SUSPEND, pci_name(pdev), pwr as *mut _);
    if ret != 0 { return ret; }
    (*pwr).available = true;
    midpwr = pwr;
    pci_set_drvdata(pdev, pwr as *mut _);
    0
}

// builtin_pci_driver(mid_pwr_pci_driver);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
