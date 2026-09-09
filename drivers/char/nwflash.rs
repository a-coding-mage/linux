// SPDX-License-Identifier: GPL-2.0-only
/*
 * Flash memory interface rev.5 driver for the Intel
 * Flash chips used on the NetWinder.
 *
 * 20/08/2000 RMK use __ioremap to map flash into virtual memory
 * 22/05/2001 RMK - Lock read against write
 *                 - merge printk level changes (with mods) from Alan Cox.
 *                 - use *ppos as the file position, not file->f_pos.
 *                 - fix check for out of range pos and r/w size
 *
 * Please note that we are tampering with the only flash chip in the
 * machine, which contains the bootup code. We therefore have the
 * power to convert these machines into doorstops...
 */

// Kernel and architecture dependencies are supplied by the surrounding crate.

use core::ffi::c_void;

const NWFLASH_VERSION: &[u8] = b"6.4\0";
const KFLASH_SIZE: usize = 1024 * 1024;
const KFLASH_SIZE4: usize = 4 * 1024 * 1024;
const KFLASH_ID: u32 = 0x89A6;
const KFLASH_ID4: u32 = 0xB0D4;

static mut flashdebug: bool = false;
static mut gbWriteEnable: i32 = 0;
static mut gbWriteBase64Enable: i32 = 0;
static mut FLASH_BASE: *mut u8 = core::ptr::null_mut();
static mut gbFlashSize: usize = KFLASH_SIZE;

#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct file_operations { _private: [u8; 0] }
#[repr(C)] pub struct miscdevice { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
extern "C" {
    static mut flash_mutex: mutex;
    static mut nwflash_mutex: mutex;
    static mut nw_gpio_lock: u64;
    static mut CSR_ROMWRITEREG: *mut u32;
    fn inb(port: u16) -> u8;
    fn udelay(usecs: u32);
    fn msleep(msecs: u32);
    fn printk(fmt: *const u8, ...);
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn mutex_lock_interruptible(lock: *mut mutex) -> i32;
    fn raw_spin_lock_irqsave(lock: *mut u64, flags: *mut usize);
    fn raw_spin_unlock_irqrestore(lock: *mut u64, flags: usize);
    fn nw_cpld_modify(mask: u32, value: u32);
    fn ioremap(addr: usize, size: usize) -> *mut u8;
    fn iounmap(addr: *mut c_void);
    fn machine_is_netwinder() -> bool;
    fn misc_register(dev: *mut miscdevice) -> i32;
    fn misc_deregister(dev: *mut miscdevice);
    fn access_ok(ptr: *const u8, count: usize) -> bool;
    fn simple_read_from_buffer(to: *mut u8, count: usize, pos: *mut i64, from: *const c_void, available: usize) -> isize;
    fn no_seek_end_llseek_size(file: *mut file, offset: i64, orig: i32, size: usize) -> i64;
    fn __get_user<T>(to: *mut T, from: *const T) -> i32;
    static jiffies: usize;
    fn time_before(a: usize, b: usize) -> bool;
    fn time_after_eq(a: usize, b: usize) -> bool;
}

const EINVAL: i32 = 22;
const EFAULT: i32 = 14;
const ENXIO: i32 = 6;
const ERESTARTSYS: i32 = 512;
const HZ: usize = 100;
const DC21285_FLASH: usize = 0;
const CPLD_FLASH_WR_ENABLE: u32 = 0;

unsafe fn get_flash_id() -> u32 {
    kick_open(); let _ = inb(0x80);
    FLASH_BASE.add(0x8000).write_volatile(0x90); udelay(15);
    let c1 = FLASH_BASE.read_volatile() as u32; let _ = inb(0x80);
    let c2 = if c1 == 0xB0 { FLASH_BASE.add(2).read_volatile() } else { FLASH_BASE.add(1).read_volatile() } as u32;
    let id = c2 + (c1 << 8); FLASH_BASE.add(0x8000).write_volatile(0xFF);
    if id == KFLASH_ID4 { gbFlashSize = KFLASH_SIZE4; } id
}

unsafe fn flash_ioctl(_filep: *mut file, cmd: u32, _arg: usize) -> i64 {
    mutex_lock(&mut flash_mutex); match cmd {
        0 => { gbWriteBase64Enable = 0; gbWriteEnable = 0; }
        1 => gbWriteEnable = 1,
        2 => gbWriteBase64Enable = 1,
        _ => { gbWriteBase64Enable = 0; gbWriteEnable = 0; mutex_unlock(&mut flash_mutex); return -(EINVAL as i64); }
    } mutex_unlock(&mut flash_mutex); 0
}

unsafe fn flash_read(_file: *mut file, buf: *mut u8, size: usize, ppos: *mut i64) -> isize {
    if mutex_lock_interruptible(&mut nwflash_mutex) != 0 { return -(ERESTARTSYS as isize); }
    let ret = simple_read_from_buffer(buf, size, ppos, FLASH_BASE as *const c_void, gbFlashSize);
    mutex_unlock(&mut nwflash_mutex); ret
}

unsafe fn flash_write(_file: *mut file, mut buf: *const u8, size: usize, ppos: *mut i64) -> isize {
    let mut p = *ppos as usize; let mut count = size; if gbWriteEnable == 0 || (p < 64*1024 && gbWriteBase64Enable == 0) { return -(EINVAL as isize); }
    if p >= gbFlashSize { return if count != 0 { -(ENXIO as isize) } else { 0 }; }
    if count > gbFlashSize-p { count = gbFlashSize-p; } if !access_ok(buf,count) { return -(EFAULT as isize); }
    if mutex_lock_interruptible(&mut nwflash_mutex) != 0 { return -(ERESTARTSYS as isize); }
    let mut written = 0usize; let mut block = (p >> 16) as i32;
    let mut temp = (((p+count)>>16) as i32)-block+1; if ((p+count)&0xffff)==0 { temp-=1; }
    while temp != 0 { let mut i=0; let mut rc; loop { rc=erase_block(block); i+=1; if rc==0 || i>=10 {break;} } if rc != 0 { break; }
        let mut j=0; 'retry: loop { rc=write_block(p,buf,(count-written) as i32); j+=1; if rc==0 && j<10 {continue 'retry;} if rc==0 {rc=-1;} break; }
        if rc<0 {break;} p+=rc as usize; buf=buf.add(rc as usize); written+=rc as usize; *ppos+=rc as i64; temp-=1; block+=1;
    } mutex_unlock(&mut nwflash_mutex); written as isize
}

unsafe fn erase_block(n: i32) -> i32 {
    CSR_ROMWRITEREG.write(0); let _ = FLASH_BASE.add(0x8000).read_volatile(); kick_open(); FLASH_BASE.add(0x8000).write_volatile(0x50);
    let ptr=FLASH_BASE.add(0x8000+(n as usize<<16)); let _=ptr.read_volatile(); kick_open(); ptr.write_volatile(0x20); ptr.write_volatile(0xD0); msleep(10);
    let timeout=jiffies+10*HZ; let mut c1=0u32; while c1&0x80==0 && time_before(jiffies,timeout) {msleep(10); c1=ptr.read_volatile() as u32;}
    kick_open(); ptr.write_volatile(0xff); if c1&0x20!=0 {FLASH_BASE.add(0x8000).write_volatile(0x50); return -2;} msleep(10);
    let mut p=FLASH_BASE.add(n as usize<<16); for _ in 0..(16*1024) {if p.cast::<u32>().read_volatile()!=0xffffffff{return -1;} p=p.add(4);} 0
}

unsafe fn write_block(mut p: usize, mut buf: *const u8, mut count: i32) -> i32 {
    let mut ptr=FLASH_BASE.add(p); let off=p&0xffff; if off+count as usize>0x10000 {count=0x10000-off as i32;} let timeout=jiffies+30*HZ;
    for o in 0..count { let addr=ptr as usize&0xfffffffc; let c2=buf.add(o as usize).read(); let mut c1;
        loop { let _=FLASH_BASE.add(0x8000).read_volatile(); kick_open(); CSR_ROMWRITEREG.write(ptr as u32&3); (addr as *mut u8).write_volatile(0x40); (addr as *mut u8).write_volatile(c2); FLASH_BASE.add(0x10000).write_volatile(0x70); let t=jiffies+HZ; c1=0; while c1&0x80==0&&time_before(jiffies,t){c1=FLASH_BASE.add(0x8000).read_volatile() as u32;} if time_after_eq(jiffies,t){kick_open();FLASH_BASE.add(0x8000).write_volatile(0x50);continue;} kick_open();FLASH_BASE.add(0x8000).write_volatile(0xff); if c1&0x10!=0&&time_before(jiffies,timeout){msleep(10);continue;} if c1&0x10!=0{return -2;} break; } ptr=ptr.add(1); }
    msleep(10); ptr=FLASH_BASE.add(p); for o in 0..count {if ptr.read_volatile()!=buf.add(o as usize).read(){return 0;}ptr=ptr.add(1);} count
}

unsafe fn kick_open() { let mut flags=0usize; raw_spin_lock_irqsave(&mut nw_gpio_lock,&mut flags); nw_cpld_modify(CPLD_FLASH_WR_ENABLE,CPLD_FLASH_WR_ENABLE); raw_spin_unlock_irqrestore(&mut nw_gpio_lock,flags); udelay(25); }

unsafe fn nwflash_init() -> i32 { let mut ret=-ENXIO; if machine_is_netwinder(){FLASH_BASE=ioremap(DC21285_FLASH,KFLASH_SIZE4);if !FLASH_BASE.is_null(){let id=get_flash_id();if id==KFLASH_ID||id==KFLASH_ID4{ret=0;}else{iounmap(FLASH_BASE as *mut c_void);}}}ret }
unsafe fn nwflash_exit(){misc_deregister(core::ptr::null_mut());iounmap(FLASH_BASE as *mut c_void);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
