// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (c) 2015-2016, IBM Corporation.
 */

// Linux kernel dependencies are supplied by the surrounding Rust kernel environment.

const DEVICE_NAME: &str = "ipmi-bt-host";
const BT_IO_BASE: u32 = 0xe4;
const BT_IRQ: u32 = 10;
const BT_CR0: usize = 0x0;
const BT_CR0_IO_BASE: u32 = 16;
const BT_CR0_IRQ: u32 = 12;
const BT_CR0_EN_CLR_SLV_RDP: u32 = 0x8;
const BT_CR0_EN_CLR_SLV_WRP: u32 = 0x4;
const BT_CR0_ENABLE_IBT: u32 = 0x1;
const BT_CR1: usize = 0x4;
const BT_CR1_IRQ_H2B: u32 = 0x01;
const BT_CR1_IRQ_HBUSY: u32 = 0x40;
const BT_CR2: usize = 0x8;
const BT_CR2_IRQ_H2B: u32 = 0x01;
const BT_CR2_IRQ_HBUSY: u32 = 0x40;
const BT_CR3: usize = 0xc;
const BT_CTRL: usize = 0x10;
const BT_CTRL_B_BUSY: u8 = 0x80;
const BT_CTRL_H_BUSY: u8 = 0x40;
const BT_CTRL_OEM0: u8 = 0x20;
const BT_CTRL_SMS_ATN: u8 = 0x10;
const BT_CTRL_B2H_ATN: u8 = 0x08;
const BT_CTRL_H2B_ATN: u8 = 0x04;
const BT_CTRL_CLR_RD_PTR: u8 = 0x02;
const BT_CTRL_CLR_WR_PTR: u8 = 0x01;
const BT_BMC2HOST: usize = 0x14;
const BT_INTMASK: usize = 0x18;
const BT_INTMASK_B2H_IRQEN: u8 = 0x01;
const BT_INTMASK_B2H_IRQ: u8 = 0x02;
const BT_INTMASK_BMC_HWRST: u8 = 0x80;
const BT_BMC_BUFFER_SIZE: usize = 256;

// External kernel declarations.
type U8 = u8;
type SsizeT = isize;
type LoffT = i64;
type PollMask = u32;
type IrqReturnT = u32;
type AtomicT = i32;

#[repr(C)] pub struct Device { _opaque: [u8; 0] }
#[repr(C)] pub struct Miscdevice { pub minor: i32, pub name: *const u8, pub fops: *const FileOperations, pub parent: *mut Device }
#[repr(C)] pub struct WaitQueueHead { _opaque: [u8; 0] }
#[repr(C)] pub struct TimerList { pub expires: usize }
#[repr(C)] pub struct Mutex { _opaque: [u8; 0] }
#[repr(C)] pub struct Inode { _opaque: [u8; 0] }
#[repr(C)] pub struct File { pub private_data: *mut core::ffi::c_void }
#[repr(C)] pub struct PollTable { _opaque: [u8; 0] }
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct FileOperations {
    pub owner: *const core::ffi::c_void,
    pub open: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> i32>,
    pub read: Option<unsafe extern "C" fn(*mut File, *mut u8, usize, *mut LoffT) -> SsizeT>,
    pub write: Option<unsafe extern "C" fn(*mut File, *const u8, usize, *mut LoffT) -> SsizeT>,
    pub release: Option<unsafe extern "C" fn(*mut Inode, *mut File) -> i32>,
    pub poll: Option<unsafe extern "C" fn(*mut File, *mut PollTable) -> PollMask>,
    pub unlocked_ioctl: Option<unsafe extern "C" fn(*mut File, u32, usize) -> isize>,
}
#[repr(C)] pub struct OfDeviceId { pub compatible: *const u8 }
#[repr(C)] pub struct PlatformDriver { _opaque: [u8; 0] }

extern "C" {
    static mut open_count: AtomicT;
    fn atomic_inc_return(v: *mut AtomicT) -> i32;
    fn atomic_dec(v: *mut AtomicT);
    fn readb(addr: *const u8) -> u8;
    fn writeb(value: u8, addr: *mut u8);
    fn readl(addr: *const u8) -> u32;
    fn writel(value: u32, addr: *mut u8);
    fn wait_event_interruptible(queue: *mut WaitQueueHead, condition: bool) -> i32;
    fn mutex_lock(mutex: *mut Mutex);
    fn mutex_unlock(mutex: *mut Mutex);
    fn copy_to_user(to: *mut u8, from: *const u8, n: usize) -> usize;
    fn copy_from_user(to: *mut u8, from: *const u8, n: usize) -> usize;
    fn wake_up(queue: *mut WaitQueueHead);
    fn poll_wait(file: *mut File, queue: *mut WaitQueueHead, wait: *mut PollTable);
    fn platform_get_irq_optional(pdev: *mut PlatformDevice, index: u32) -> i32;
    fn devm_request_irq(dev: *mut Device, irq: i32, handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> IrqReturnT, flags: u32, name: *const u8, arg: *mut core::ffi::c_void) -> i32;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut u8;
    fn mutex_init(mutex: *mut Mutex);
    fn init_waitqueue_head(queue: *mut WaitQueueHead);
    fn misc_register(dev: *mut Miscdevice) -> i32;
    fn misc_deregister(dev: *mut Miscdevice);
    fn timer_setup(timer: *mut TimerList, callback: unsafe extern "C" fn(*mut TimerList), flags: u32);
    fn add_timer(timer: *mut TimerList);
    fn timer_delete_sync(timer: *mut TimerList);
    fn dev_get_drvdata(dev: *mut Device) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut Device, data: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct BtBmc {
    pub dev: Device,
    pub miscdev: Miscdevice,
    pub base: *mut u8,
    pub irq: i32,
    pub queue: WaitQueueHead,
    pub poll_timer: TimerList,
    pub mutex: Mutex,
}

unsafe fn bt_inb(bt_bmc: *mut BtBmc, reg: usize) -> u8 { readb((*bt_bmc).base.add(reg)) }
unsafe fn bt_outb(bt_bmc: *mut BtBmc, data: u8, reg: usize) { writeb(data, (*bt_bmc).base.add(reg)); }
unsafe fn clr_rd_ptr(bt_bmc: *mut BtBmc) { bt_outb(bt_bmc, BT_CTRL_CLR_RD_PTR, BT_CTRL); }
unsafe fn clr_wr_ptr(bt_bmc: *mut BtBmc) { bt_outb(bt_bmc, BT_CTRL_CLR_WR_PTR, BT_CTRL); }
unsafe fn clr_h2b_atn(bt_bmc: *mut BtBmc) { bt_outb(bt_bmc, BT_CTRL_H2B_ATN, BT_CTRL); }
unsafe fn set_b_busy(bt_bmc: *mut BtBmc) { if bt_inb(bt_bmc, BT_CTRL) & BT_CTRL_B_BUSY == 0 { bt_outb(bt_bmc, BT_CTRL_B_BUSY, BT_CTRL); } }
unsafe fn clr_b_busy(bt_bmc: *mut BtBmc) { if bt_inb(bt_bmc, BT_CTRL) & BT_CTRL_B_BUSY != 0 { bt_outb(bt_bmc, BT_CTRL_B_BUSY, BT_CTRL); } }
unsafe fn set_b2h_atn(bt_bmc: *mut BtBmc) { bt_outb(bt_bmc, BT_CTRL_B2H_ATN, BT_CTRL); }
unsafe fn bt_read(bt_bmc: *mut BtBmc) -> u8 { bt_inb(bt_bmc, BT_BMC2HOST) }
unsafe fn bt_readn(bt_bmc: *mut BtBmc, buf: *mut u8, n: usize) -> SsizeT { for i in 0..n { *buf.add(i) = bt_read(bt_bmc); } n as SsizeT }
unsafe fn bt_write(bt_bmc: *mut BtBmc, c: u8) { bt_outb(bt_bmc, c, BT_BMC2HOST); }
unsafe fn bt_writen(bt_bmc: *mut BtBmc, buf: *mut u8, n: usize) -> SsizeT { for i in 0..n { bt_write(bt_bmc, *buf.add(i)); } n as SsizeT }
unsafe fn set_sms_atn(bt_bmc: *mut BtBmc) { bt_outb(bt_bmc, BT_CTRL_SMS_ATN, BT_CTRL); }

unsafe fn file_bt_bmc(file: *mut File) -> *mut BtBmc {
    ( (*file).private_data as *mut u8 ).sub(core::mem::offset_of!(BtBmc, miscdev)) as *mut BtBmc
}

unsafe extern "C" fn bt_bmc_open(_inode: *mut Inode, file: *mut File) -> i32 {
    let bt_bmc = file_bt_bmc(file);
    if atomic_inc_return(&mut open_count) == 1 { clr_b_busy(bt_bmc); return 0; }
    atomic_dec(&mut open_count); -16
}

/*
 * The BT (Block Transfer) interface means that entire messages are buffered by
 * the host before a notification is sent to the BMC that there is data to be read.
 * The first byte is the length and the message data follows.
 */
unsafe extern "C" fn bt_bmc_read(file: *mut File, buf: *mut u8, mut count: usize, ppos: *mut LoffT) -> SsizeT {
    let bt_bmc = file_bt_bmc(file); let mut len_byte = 1usize; let mut kbuffer = [0u8; BT_BMC_BUFFER_SIZE]; let mut ret: SsizeT = 0;
    if *ppos != 0 { /* WARN_ON(*ppos) */ }
    if wait_event_interruptible(&mut (*bt_bmc).queue, bt_inb(bt_bmc, BT_CTRL) & BT_CTRL_H2B_ATN != 0) != 0 { return -512; }
    mutex_lock(&mut (*bt_bmc).mutex);
    if bt_inb(bt_bmc, BT_CTRL) & BT_CTRL_H2B_ATN == 0 { mutex_unlock(&mut (*bt_bmc).mutex); return -5; }
    set_b_busy(bt_bmc); clr_h2b_atn(bt_bmc); clr_rd_ptr(bt_bmc); kbuffer[0] = bt_read(bt_bmc); let mut len = kbuffer[0] as usize;
    if len + 1 > count { len = count - 1; }
    while len != 0 { let nread = core::cmp::min(len, kbuffer.len() - len_byte); bt_readn(bt_bmc, kbuffer.as_mut_ptr().add(len_byte), nread); if copy_to_user(buf, kbuffer.as_ptr(), nread + len_byte) != 0 { ret = -14; break; } len -= nread; buf = buf.add(nread + len_byte); ret += (nread + len_byte) as SsizeT; len_byte = 0; }
    clr_b_busy(bt_bmc); mutex_unlock(&mut (*bt_bmc).mutex); ret
}

unsafe extern "C" fn bt_bmc_write(file: *mut File, buf: *const u8, mut count: usize, ppos: *mut LoffT) -> SsizeT {
    let bt_bmc = file_bt_bmc(file); let mut kbuffer = [0u8; BT_BMC_BUFFER_SIZE]; let mut ret: SsizeT = 0;
    if count < 5 { return -22; } if *ppos != 0 { /* WARN_ON(*ppos) */ }
    if wait_event_interruptible(&mut (*bt_bmc).queue, bt_inb(bt_bmc, BT_CTRL) & (BT_CTRL_H_BUSY | BT_CTRL_B2H_ATN) == 0) != 0 { return -512; }
    mutex_lock(&mut (*bt_bmc).mutex);
    if bt_inb(bt_bmc, BT_CTRL) & (BT_CTRL_H_BUSY | BT_CTRL_B2H_ATN) != 0 { mutex_unlock(&mut (*bt_bmc).mutex); return -5; }
    clr_wr_ptr(bt_bmc);
    while count != 0 { let n = core::cmp::min(count, kbuffer.len()); if copy_from_user(kbuffer.as_mut_ptr(), buf, n) != 0 { ret = -14; break; } bt_writen(bt_bmc, kbuffer.as_mut_ptr(), n); count -= n; buf = buf.add(n); ret += n as SsizeT; }
    set_b2h_atn(bt_bmc); mutex_unlock(&mut (*bt_bmc).mutex); ret
}

unsafe extern "C" fn bt_bmc_ioctl(file: *mut File, cmd: u32, _param: usize) -> isize { let bt_bmc = file_bt_bmc(file); if cmd == 0x01 { set_sms_atn(bt_bmc); return 0; } -22 }
unsafe extern "C" fn bt_bmc_release(_inode: *mut Inode, file: *mut File) -> i32 { let bt_bmc = file_bt_bmc(file); atomic_dec(&mut open_count); set_b_busy(bt_bmc); 0 }
unsafe extern "C" fn bt_bmc_poll(file: *mut File, wait: *mut PollTable) -> PollMask { let bt_bmc = file_bt_bmc(file); poll_wait(file, &mut (*bt_bmc).queue, wait); let ctrl = bt_inb(bt_bmc, BT_CTRL); let mut mask = 0; if ctrl & BT_CTRL_H2B_ATN != 0 { mask |= 0x001; } if ctrl & (BT_CTRL_H_BUSY | BT_CTRL_B2H_ATN) == 0 { mask |= 0x004; } mask }

unsafe extern "C" fn poll_timer(t: *mut TimerList) { let bt_bmc = (t as *mut u8).sub(core::mem::offset_of!(BtBmc, poll_timer)) as *mut BtBmc; (*bt_bmc).poll_timer.expires += 500; wake_up(&mut (*bt_bmc).queue); add_timer(&mut (*bt_bmc).poll_timer); }
unsafe extern "C" fn bt_bmc_irq(_irq: i32, arg: *mut core::ffi::c_void) -> IrqReturnT { let bt_bmc = arg as *mut BtBmc; let mut reg = readl((*bt_bmc).base.add(BT_CR2)); reg &= BT_CR2_IRQ_H2B | BT_CR2_IRQ_HBUSY; if reg == 0 { return 0; } writel(reg, (*bt_bmc).base.add(BT_CR2)); wake_up(&mut (*bt_bmc).queue); 1 }

unsafe fn bt_bmc_config_irq(bt_bmc: *mut BtBmc, pdev: *mut PlatformDevice) -> i32 {
    (*bt_bmc).irq = platform_get_irq_optional(pdev, 0); if (*bt_bmc).irq < 0 { return (*bt_bmc).irq; }
    let rc = devm_request_irq(&mut (*pdev).dev, (*bt_bmc).irq, bt_bmc_irq, 0x80, DEVICE_NAME.as_ptr(), bt_bmc as *mut _);
    if rc < 0 { (*bt_bmc).irq = rc; return rc; }
    let mut reg = readl((*bt_bmc).base.add(BT_CR1)); reg |= BT_CR1_IRQ_H2B | BT_CR1_IRQ_HBUSY; writel(reg, (*bt_bmc).base.add(BT_CR1)); 0
}

unsafe extern "C" fn bt_bmc_probe(pdev: *mut PlatformDevice) -> i32 {
    let bt_bmc = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<BtBmc>(), 0) as *mut BtBmc; if bt_bmc.is_null() { return -12; }
    dev_set_drvdata(&mut (*pdev).dev, bt_bmc as *mut _); (*bt_bmc).base = devm_platform_ioremap_resource(pdev, 0); if (*bt_bmc).base.is_null() { return -14; }
    mutex_init(&mut (*bt_bmc).mutex); init_waitqueue_head(&mut (*bt_bmc).queue); (*bt_bmc).irq = -1; bt_bmc_config_irq(bt_bmc, pdev);
    writel((BT_IO_BASE << BT_CR0_IO_BASE) | (BT_IRQ << BT_CR0_IRQ) | BT_CR0_EN_CLR_SLV_RDP | BT_CR0_EN_CLR_SLV_WRP | BT_CR0_ENABLE_IBT, (*bt_bmc).base.add(BT_CR0)); clr_b_busy(bt_bmc); 0
}
unsafe extern "C" fn bt_bmc_remove(pdev: *mut PlatformDevice) { let bt_bmc = dev_get_drvdata(&mut (*pdev).dev) as *mut BtBmc; misc_deregister(&mut (*bt_bmc).miscdev); if (*bt_bmc).irq < 0 { timer_delete_sync(&mut (*bt_bmc).poll_timer); } }

static mut bt_bmc_match: [OfDeviceId; 4] = [
    OfDeviceId { compatible: b"aspeed,ast2400-ibt-bmc\0".as_ptr() },
    OfDeviceId { compatible: b"aspeed,ast2500-ibt-bmc\0".as_ptr() },
    OfDeviceId { compatible: b"aspeed,ast2600-ibt-bmc\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

// module_platform_driver(bt_bmc_driver);
// MODULE_DEVICE_TABLE(of, bt_bmc_match);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Alistair Popple <alistair@popple.id.au>");
// MODULE_DESCRIPTION("Linux device interface to the IPMI BT interface");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
