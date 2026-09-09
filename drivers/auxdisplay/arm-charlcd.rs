// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for the on-board character LCD found on some ARM reference boards
 * This is basically an Hitachi HD44780 LCD with a custom IP block to drive it
 * https://en.wikipedia.org/wiki/HD44780_Character_LCD
 * Currently it will just display the text "ARM Linux" and the linux version
 *
 * Author: Linus Walleij <triad@df.lth.se>
 */

// Linux kernel dependencies supplied by other files.
use core::ffi::c_void;

type U8 = u8;
type U32 = u32;

#[repr(C)]
struct Device;
#[repr(C)]
struct PlatformDevice { dev: Device }
#[repr(C)]
struct WorkStruct;
#[repr(C)]
struct DelayedWork { work: WorkStruct }
#[repr(C)]
struct Completion;
#[repr(C)]
struct DevPmOps;
#[repr(C)]
struct OfDeviceId;
#[repr(C)]
struct PlatformDriver;

extern "C" {
    fn readl(addr: *mut c_void) -> u32;
    fn writel(value: u32, addr: *mut c_void);
    fn udelay(usecs: u64);
    fn msleep(msecs: u64);
    fn readl_poll_timeout_atomic(addr: *mut c_void, value: *mut u32, condition: u32, delay: u64, timeout: u64) -> i32;
    fn wait_for_completion_interruptible_timeout(completion: *mut Completion, timeout: u64) -> i64;
    fn complete(completion: *mut Completion);
    fn init_completion(completion: *mut Completion);
    fn dev_info(dev: *mut Device, format: *const u8, ...);
    fn dev_err(dev: *mut Device, format: *const u8, ...);
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut c_void;
    fn ptr_err(ptr: *mut c_void) -> i32;
    fn platform_get_irq(pdev: *mut PlatformDevice, index: u32) -> i32;
    fn devm_request_irq(dev: *mut Device, irq: i32, handler: unsafe extern "C" fn(i32, *mut c_void) -> i32, flags: u32, name: *const u8, data: *mut c_void) -> i32;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut c_void);
    fn init_delayed_work(work: *mut DelayedWork, function: unsafe extern "C" fn(*mut WorkStruct));
    fn schedule_delayed_work(work: *mut DelayedWork, delay: u64) -> bool;
    fn dev_get_drvdata(dev: *mut Device) -> *mut c_void;
}

const DRIVERNAME: &[u8] = b"arm-charlcd\0";
const CHARLCD_TIMEOUT: u64 = 1000;
const CHAR_COM: u32 = 0x00;
const CHAR_DAT: u32 = 0x04;
const CHAR_RD: u32 = 0x08;
const CHAR_RAW: u32 = 0x0c;
const CHAR_MASK: u32 = 0x10;
const CHAR_STAT: u32 = 0x14;
const CHAR_RAW_CLEAR: u32 = 0x00000000;
const CHAR_RAW_VALID: u32 = 0x00000100;
const HD_CLEAR: u8 = 0x01;
const HD_HOME: u8 = 0x02;
const HD_ENTRYMODE: u8 = 0x04;
const HD_ENTRYMODE_INCREMENT: u8 = 0x02;
const HD_ENTRYMODE_SHIFT: u8 = 0x01;
const HD_DISPCTRL: u8 = 0x08;
const HD_DISPCTRL_ON: u8 = 0x04;
const HD_DISPCTRL_CURSOR_ON: u8 = 0x02;
const HD_DISPCTRL_CURSOR_BLINK: u8 = 0x01;
const HD_CRSR_SHIFT: u8 = 0x10;
const HD_CRSR_SHIFT_DISPLAY: u8 = 0x08;
const HD_CRSR_SHIFT_DISPLAY_RIGHT: u8 = 0x04;
const HD_FUNCSET: u8 = 0x20;
const HD_FUNCSET_8BIT: u8 = 0x10;
const HD_FUNCSET_2_LINES: u8 = 0x08;
const HD_FUNCSET_FONT_5X10: u8 = 0x04;
const HD_SET_CGRAM: u8 = 0x40;
const HD_SET_DDRAM: u8 = 0x80;
const HD_BUSY_FLAG: u8 = 0x80;

#[repr(C)]
struct Charlcd {
    dev: *mut Device,
    virtbase: *mut c_void,
    irq: i32,
    complete: Completion,
    init_work: DelayedWork,
}

unsafe extern "C" fn charlcd_interrupt(_irq: i32, data: *mut c_void) -> i32 {
    let lcd = data as *mut Charlcd;
    let status = unsafe { readl((*lcd).virtbase.add(CHAR_STAT as usize)) } & 0x01;
    unsafe { writel(CHAR_RAW_CLEAR, (*lcd).virtbase.add(CHAR_RAW as usize)); }
    if status != 0 { unsafe { complete(&mut (*lcd).complete); } }
    else { unsafe { dev_info((*lcd).dev, b"Spurious IRQ (%02x)\n\0".as_ptr(), status); } }
    1
}

unsafe fn charlcd_wait_complete_irq(lcd: *mut Charlcd) {
    let ret = unsafe { wait_for_completion_interruptible_timeout(&mut (*lcd).complete, CHARLCD_TIMEOUT) };
    unsafe { writel(0, (*lcd).virtbase.add(CHAR_MASK as usize)); }
    if ret < 0 { unsafe { dev_err((*lcd).dev, b"wait_for_completion_interruptible_timeout() returned %d waiting for ready\n\0".as_ptr(), ret); } return; }
    if ret == 0 { unsafe { dev_err((*lcd).dev, b"charlcd controller timed out waiting for ready\n\0".as_ptr()); } }
}

unsafe fn charlcd_4bit_read_char(lcd: *mut Charlcd) -> u8 {
    let mut val = 0u32;
    if unsafe { (*lcd).irq } >= 0 { unsafe { charlcd_wait_complete_irq(lcd); } }
    else { unsafe { udelay(100); readl_poll_timeout_atomic((*lcd).virtbase.add(CHAR_RAW as usize), &mut val, val & CHAR_RAW_VALID, 100, 1000); writel(CHAR_RAW_CLEAR, (*lcd).virtbase.add(CHAR_RAW as usize)); } }
    unsafe { msleep(1); }
    let mut data = (unsafe { readl((*lcd).virtbase.add(CHAR_RD as usize)) } & 0xf0) as u8;
    unsafe { udelay(100); readl_poll_timeout_atomic((*lcd).virtbase.add(CHAR_RAW as usize), &mut val, val & CHAR_RAW_VALID, 100, 1000); writel(CHAR_RAW_CLEAR, (*lcd).virtbase.add(CHAR_RAW as usize)); msleep(1); }
    data |= ((unsafe { readl((*lcd).virtbase.add(CHAR_RD as usize)) } >> 4) & 0x0f) as u8;
    data
}

unsafe fn charlcd_4bit_read_bf(lcd: *mut Charlcd) -> bool {
    if unsafe { (*lcd).irq } >= 0 { unsafe { writel(CHAR_RAW_CLEAR, (*lcd).virtbase.add(CHAR_RAW as usize)); init_completion(&mut (*lcd).complete); writel(1, (*lcd).virtbase.add(CHAR_MASK as usize)); } }
    unsafe { readl((*lcd).virtbase.add(CHAR_COM as usize)); }
    unsafe { charlcd_4bit_read_char(lcd) & HD_BUSY_FLAG != 0 }
}

unsafe fn charlcd_4bit_wait_busy(lcd: *mut Charlcd) {
    let mut retries = 50;
    unsafe { udelay(100); while charlcd_4bit_read_bf(lcd) && retries != 0 { retries -= 1; } }
    if retries == 0 { unsafe { dev_err((*lcd).dev, b"timeout waiting for busyflag\n\0".as_ptr()); } }
}

unsafe fn charlcd_4bit_command(lcd: *mut Charlcd, cmd: u8) {
    let cmdlo = ((cmd << 4) & 0xf0) as u32; let cmdhi = (cmd & 0xf0) as u32;
    unsafe { writel(cmdhi, (*lcd).virtbase.add(CHAR_COM as usize)); udelay(10); writel(cmdlo, (*lcd).virtbase.add(CHAR_COM as usize)); charlcd_4bit_wait_busy(lcd); }
}

unsafe fn charlcd_4bit_char(lcd: *mut Charlcd, ch: u8) {
    let chlo = ((ch << 4) & 0xf0) as u32; let chhi = (ch & 0xf0) as u32;
    unsafe { writel(chhi, (*lcd).virtbase.add(CHAR_DAT as usize)); udelay(10); writel(chlo, (*lcd).virtbase.add(CHAR_DAT as usize)); charlcd_4bit_wait_busy(lcd); }
}

unsafe fn charlcd_4bit_print(lcd: *mut Charlcd, line: i32, str_: *const u8) {
    let offset = if line == 0 { 0 } else if line == 1 { 0x28 } else { return };
    unsafe { charlcd_4bit_command(lcd, HD_SET_DDRAM | offset); }
    let mut i = 0; while i < 0x28 { let ch = unsafe { *str_.add(i) }; if ch == 0 { break; } unsafe { charlcd_4bit_char(lcd, ch); } i += 1; }
}

unsafe fn charlcd_4bit_init(lcd: *mut Charlcd) {
    unsafe { writel((HD_FUNCSET | HD_FUNCSET_8BIT) as u32, (*lcd).virtbase.add(CHAR_COM as usize)); msleep(5); writel((HD_FUNCSET | HD_FUNCSET_8BIT) as u32, (*lcd).virtbase.add(CHAR_COM as usize)); udelay(100); writel((HD_FUNCSET | HD_FUNCSET_8BIT) as u32, (*lcd).virtbase.add(CHAR_COM as usize)); udelay(100); writel(HD_FUNCSET as u32, (*lcd).virtbase.add(CHAR_COM as usize)); udelay(100); charlcd_4bit_command(lcd, HD_FUNCSET | HD_FUNCSET_2_LINES); charlcd_4bit_command(lcd, HD_DISPCTRL | HD_DISPCTRL_ON); charlcd_4bit_command(lcd, HD_ENTRYMODE | HD_ENTRYMODE_INCREMENT); charlcd_4bit_command(lcd, HD_CLEAR); charlcd_4bit_command(lcd, HD_HOME); charlcd_4bit_print(lcd, 0, b"ARM Linux\0".as_ptr()); charlcd_4bit_print(lcd, 1, UTS_RELEASE.as_ptr()); }
}

extern "C" { static UTS_RELEASE: [u8; 1]; }

unsafe extern "C" fn charlcd_init_work(work: *mut WorkStruct) {
    let lcd = (work as *mut u8).sub(core::mem::offset_of!(Charlcd, init_work) + core::mem::offset_of!(DelayedWork, work)) as *mut Charlcd;
    unsafe { charlcd_4bit_init(lcd); }
}

unsafe extern "C" fn charlcd_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = unsafe { &mut (*pdev).dev } as *mut Device;
    let lcd = unsafe { devm_kzalloc(dev, core::mem::size_of::<Charlcd>(), 0) } as *mut Charlcd;
    if lcd.is_null() { return -12; }
    unsafe { (*lcd).dev = dev; }
    let virtbase = unsafe { devm_platform_ioremap_resource(pdev, 0) };
    if virtbase as isize == -1 { return unsafe { ptr_err(virtbase) }; }
    unsafe { (*lcd).virtbase = virtbase; (*lcd).irq = platform_get_irq(pdev, 0); }
    if unsafe { (*lcd).irq } >= 0 {
        let ret = unsafe { devm_request_irq(dev, (*lcd).irq, charlcd_interrupt, 0, DRIVERNAME.as_ptr(), lcd as *mut c_void) };
        if ret != 0 { return ret; }
    }
    unsafe { platform_set_drvdata(pdev, lcd as *mut c_void); init_delayed_work(&mut (*lcd).init_work, charlcd_init_work); schedule_delayed_work(&mut (*lcd).init_work, 0); }
    0
}
unsafe extern "C" fn charlcd_suspend(dev: *mut Device) -> i32 { let lcd = unsafe { dev_get_drvdata(dev) } as *mut Charlcd; unsafe { charlcd_4bit_command(lcd, HD_DISPCTRL); } 0 }
unsafe extern "C" fn charlcd_resume(dev: *mut Device) -> i32 { let lcd = unsafe { dev_get_drvdata(dev) } as *mut Charlcd; unsafe { charlcd_4bit_command(lcd, HD_DISPCTRL | HD_DISPCTRL_ON); } 0 }

#[repr(C)]
struct CharLCDPmOps { suspend: unsafe extern "C" fn(*mut Device) -> i32, resume: unsafe extern "C" fn(*mut Device) -> i32 }
static CHARLCD_PM_OPS: CharLCDPmOps = CharLCDPmOps { suspend: charlcd_suspend, resume: charlcd_resume };
static CHARLCD_MATCH: [OfDeviceId; 2] = unsafe { core::mem::zeroed() };
static mut CHARLCD_DRIVER: *mut PlatformDriver = core::ptr::null_mut();
// equivalent of builtin_platform_driver_probe(charlcd_driver, charlcd_probe)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
