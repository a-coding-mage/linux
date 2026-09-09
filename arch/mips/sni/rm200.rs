/*
 * RM200 specific code
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2006,2007 Thomas Bogendoerfer (tsbogend@alpha.franken.de)
 *
 * i8259 parts ripped out of arch/mips/kernel/i8259.c
 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

const RM200_I8259A_IRQ_BASE: c_uint = 32;
const PIC_CMD: isize = 0x00;
const PIC_IMR: isize = 0x01;
const PIC_ISR: isize = PIC_CMD;
const PIC_CASCADE_IR: c_uint = 2;
const MASTER_ICW4_DEFAULT: u8 = 0x01;
const SLAVE_ICW4_DEFAULT: u8 = 0x01;
const SNI_RM200_INT_START: c_uint = 24;
const SNI_RM200_INT_END: c_uint = 28;

#[repr(C)]
pub struct PlatSerial8250Port {
    pub mapbase: c_ulong,
    pub irq: c_uint,
    pub uartclk: c_uint,
    pub iotype: c_uint,
    pub flags: c_ulong,
}
#[repr(C)] pub struct Device { pub platform_data: *mut c_void }
#[repr(C)] pub struct PlatformDevice { pub name: *const c_char, pub id: c_int, pub dev: Device, pub num_resources: usize, pub resource: *mut Resource }
#[repr(C)] pub struct Resource { pub name: *const c_char, pub start: c_ulong, pub end: c_ulong, pub flags: c_ulong }
#[repr(C)] pub struct IrqData { pub irq: c_uint }
#[repr(C)] pub struct IrqChip { pub name: *const c_char, pub irq_mask: Option<unsafe extern "C" fn(*mut IrqData)>, pub irq_unmask: Option<unsafe extern "C" fn(*mut IrqData)>, pub irq_mask_ack: Option<unsafe extern "C" fn(*mut IrqData)> }
#[repr(C)] pub struct RawSpinLock { _private: [u8; 0] }

extern "C" {
    static mut sni_brd_type: c_int;
    static mut irq_err_count: c_uint;
    static mut iomem_resource: Resource;
    static mut sni_hwint: Option<unsafe extern "C" fn()>;
    static sni_rm200_i8259A_lock: RawSpinLock;
    fn platform_device_register(dev: *mut PlatformDevice) -> c_int;
    fn sni_eisa_root_init();
    fn raw_spin_lock_irqsave(lock: *const RawSpinLock, flags: *mut c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *const RawSpinLock, flags: c_ulong);
    fn raw_spin_lock(lock: *const RawSpinLock);
    fn raw_spin_unlock(lock: *const RawSpinLock);
    fn writeb(value: u8, addr: *mut u8);
    fn readb(addr: *mut u8) -> u8;
    fn ioremap(addr: c_ulong, size: usize) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn udelay(usecs: c_uint);
    fn insert_resource(root: *mut Resource, new: *mut Resource) -> c_int;
    fn irq_set_chip_and_handler(irq: c_uint, chip: *mut IrqChip, handler: unsafe extern "C" fn());
    fn request_irq(irq: c_uint, handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> c_uint>, flags: c_ulong, name: *const c_char, dev: *mut c_void) -> c_int;
    fn do_IRQ(irq: c_int);
    fn mips_cpu_irq_init();
    fn sni_isa_irq_handler(dummy: c_int, p: *mut c_void) -> c_uint;
    fn read_c0_cause() -> u32;
    fn read_c0_status() -> u32;
    fn clear_c0_status(mask: u32);
    fn set_c0_status(mask: u32);
    fn change_c0_status(mask: u32, value: u32);
    fn ffs(value: c_int) -> c_int;
    fn printk(fmt: *const c_char, ...);
    fn atomic_inc(value: *mut c_uint);
    fn pr_err(fmt: *const c_char, ...);
}

static mut RM200_DATA: [PlatSerial8250Port; 3] = [
    PlatSerial8250Port { mapbase: 0x160003f8, irq: RM200_I8259A_IRQ_BASE + 4, uartclk: 1843200, iotype: 0, flags: 0 },
    PlatSerial8250Port { mapbase: 0x160002f8, irq: RM200_I8259A_IRQ_BASE + 3, uartclk: 1843200, iotype: 0, flags: 0 },
    PlatSerial8250Port { mapbase: 0, irq: 0, uartclk: 0, iotype: 0, flags: 0 },
];
static mut RM200_SERIAL8250_DEVICE: PlatformDevice = PlatformDevice { name: b"serial8250\0".as_ptr() as *const c_char, id: 0, dev: Device { platform_data: core::ptr::addr_of_mut!(RM200_DATA) as *mut c_void }, num_resources: 0, resource: core::ptr::null_mut() };
static mut RM200_DS1216_RSRC: [Resource; 1] = [Resource { name: core::ptr::null(), start: 0x1cd41ffc, end: 0x1cd41fff, flags: 0x00000200 }];
static mut RM200_DS1216_DEVICE: PlatformDevice = PlatformDevice { name: b"rtc-ds1216\0".as_ptr() as *const c_char, id: -1, dev: Device { platform_data: core::ptr::null_mut() }, num_resources: 1, resource: core::ptr::addr_of_mut!(RM200_DS1216_RSRC) };
static mut PIC_MASTER: *mut u8 = core::ptr::null_mut();
static mut PIC_SLAVE: *mut u8 = core::ptr::null_mut();
static mut CACHED_IRQ_MASK: c_uint = 0xffff;

unsafe extern "C" fn disable_irq(d: *mut IrqData) { let irq = (*d).irq - RM200_I8259A_IRQ_BASE; let mut flags = 0; CACHED_IRQ_MASK |= 1 << irq; raw_spin_lock_irqsave(&sni_rm200_i8259A_lock, &mut flags); if irq & 8 != 0 { writeb((CACHED_IRQ_MASK >> 8) as u8, PIC_SLAVE.offset(PIC_IMR)); } else { writeb(CACHED_IRQ_MASK as u8, PIC_MASTER.offset(PIC_IMR)); } raw_spin_unlock_irqrestore(&sni_rm200_i8259A_lock, flags); }
unsafe extern "C" fn enable_irq(d: *mut IrqData) { let irq = (*d).irq - RM200_I8259A_IRQ_BASE; let mut flags = 0; CACHED_IRQ_MASK &= !(1 << irq); raw_spin_lock_irqsave(&sni_rm200_i8259A_lock, &mut flags); if irq & 8 != 0 { writeb((CACHED_IRQ_MASK >> 8) as u8, PIC_SLAVE.offset(PIC_IMR)); } else { writeb(CACHED_IRQ_MASK as u8, PIC_MASTER.offset(PIC_IMR)); } raw_spin_unlock_irqrestore(&sni_rm200_i8259A_lock, flags); }
unsafe fn irq_real(irq: c_uint) -> c_int { let mask = 1 << irq; let pic = if irq < 8 { PIC_MASTER } else { PIC_SLAVE }; writeb(0x0b, pic.offset(PIC_CMD)); let v = readb(pic.offset(PIC_CMD)) as c_int & if irq < 8 { mask as c_int } else { (mask >> 8) as c_int }; writeb(0x0a, pic.offset(PIC_CMD)); v }
pub unsafe extern "C" fn sni_rm200_mask_and_ack_8259A(d: *mut IrqData) { let irq = (*d).irq - RM200_I8259A_IRQ_BASE; let irqmask = 1 << irq; let mut flags = 0; raw_spin_lock_irqsave(&sni_rm200_i8259A_lock, &mut flags); if CACHED_IRQ_MASK & irqmask != 0 { if irq_real(irq) == 0 { atomic_inc(&mut irq_err_count); } } CACHED_IRQ_MASK |= irqmask; if irq & 8 != 0 { readb(PIC_SLAVE.offset(PIC_IMR)); writeb((CACHED_IRQ_MASK >> 8) as u8, PIC_SLAVE.offset(PIC_IMR)); writeb(0x60 + (irq & 7) as u8, PIC_SLAVE.offset(PIC_CMD)); writeb(0x62, PIC_MASTER.offset(PIC_CMD)); } else { readb(PIC_MASTER.offset(PIC_IMR)); writeb(CACHED_IRQ_MASK as u8, PIC_MASTER.offset(PIC_IMR)); writeb(0x60 + irq as u8, PIC_MASTER.offset(PIC_CMD)); } raw_spin_unlock_irqrestore(&sni_rm200_i8259A_lock, flags); }
static mut IRQ_CHIP: IrqChip = IrqChip { name: b"RM200-XT-PIC\0".as_ptr() as *const c_char, irq_mask: Some(disable_irq), irq_unmask: Some(enable_irq), irq_mask_ack: Some(sni_rm200_mask_and_ack_8259A) };

pub unsafe extern "C" fn sni_rm200_init_8259A() { let mut flags = 0; raw_spin_lock_irqsave(&sni_rm200_i8259A_lock, &mut flags); writeb(0xff, PIC_MASTER.offset(PIC_IMR)); writeb(0xff, PIC_SLAVE.offset(PIC_IMR)); writeb(0x11, PIC_MASTER); writeb(0, PIC_MASTER.offset(PIC_IMR)); writeb(1 << PIC_CASCADE_IR, PIC_MASTER.offset(PIC_IMR)); writeb(MASTER_ICW4_DEFAULT, PIC_MASTER.offset(PIC_IMR)); writeb(0x11, PIC_SLAVE); writeb(8, PIC_SLAVE.offset(PIC_IMR)); writeb(PIC_CASCADE_IR as u8, PIC_SLAVE.offset(PIC_IMR)); writeb(SLAVE_ICW4_DEFAULT, PIC_SLAVE.offset(PIC_IMR)); udelay(100); writeb(CACHED_IRQ_MASK as u8, PIC_MASTER.offset(PIC_IMR)); writeb((CACHED_IRQ_MASK >> 8) as u8, PIC_SLAVE.offset(PIC_IMR)); raw_spin_unlock_irqrestore(&sni_rm200_i8259A_lock, flags); }

pub unsafe extern "C" fn sni_rm200_i8259_irqs() { PIC_MASTER = ioremap(0x16000020, 4); if PIC_MASTER.is_null() { return; } PIC_SLAVE = ioremap(0x160000a0, 4); if PIC_SLAVE.is_null() { iounmap(PIC_MASTER); return; } sni_rm200_init_8259A(); for i in RM200_I8259A_IRQ_BASE..RM200_I8259A_IRQ_BASE + 16 { irq_set_chip_and_handler(i, &mut IRQ_CHIP, handle_level_irq); } }
unsafe extern "C" fn handle_level_irq() {}

unsafe extern "C" fn sni_rm200_i8259_irq_handler(_: c_int, _: *mut c_void) -> c_uint { let irq = sni_rm200_i8259_irq(); if irq < 0 { return 0; } do_IRQ(irq); 1 }
unsafe fn sni_rm200_i8259_irq() -> c_int { raw_spin_lock(&sni_rm200_i8259A_lock); writeb(0x0c, PIC_MASTER); let mut irq = (readb(PIC_MASTER) & 7) as c_int; if irq == PIC_CASCADE_IR as c_int { writeb(0x0c, PIC_SLAVE); irq = (readb(PIC_SLAVE) & 7) as c_int + 8; } if irq == 7 { writeb(0x0b, PIC_MASTER); if readb(PIC_MASTER) & 0x80 == 0 { irq = -1; } } raw_spin_unlock(&sni_rm200_i8259A_lock); if irq >= 0 { irq + RM200_I8259A_IRQ_BASE as c_int } else { irq } }
unsafe extern "C" fn enable_rm200_irq(d: *mut IrqData) { let mask = 1 << ((*d).irq - SNI_RM200_INT_START); let p = 0xbc080000 as *mut u8; core::ptr::write_volatile(p, core::ptr::read_volatile(p) & !(mask as u8)); }
unsafe extern "C" fn disable_rm200_irq(d: *mut IrqData) { let mask = 1 << ((*d).irq - SNI_RM200_INT_START); let p = 0xbc080000 as *mut u8; core::ptr::write_volatile(p, core::ptr::read_volatile(p) | mask as u8); }
static mut RM200_IRQ_TYPE: IrqChip = IrqChip { name: b"RM200\0".as_ptr() as *const c_char, irq_mask: Some(disable_rm200_irq), irq_unmask: Some(enable_rm200_irq), irq_mask_ack: None };
unsafe fn sni_rm200_hwint() { let pending = read_c0_cause() & read_c0_status(); if pending & (1 << 15) != 0 { do_IRQ(7 + 0); } else if pending & 1 != 0 { clear_c0_status(1); let mask = core::ptr::read_volatile(0xbc080000 as *const u8) ^ 0x1f; let stat = core::ptr::read_volatile(0xbc000000 as *const u8) ^ 0x14; let irq = ffs((stat & mask & 0x1f) as c_int); if irq > 0 { do_IRQ(irq + SNI_RM200_INT_START as c_int - 1); } set_c0_status(1); } }
#[no_mangle] pub unsafe extern "C" fn snirm_setup_devinit() -> c_int { if sni_brd_type == 0 { platform_device_register(&mut RM200_SERIAL8250_DEVICE); platform_device_register(&mut RM200_DS1216_DEVICE); sni_eisa_root_init(); } 0 }
#[no_mangle] pub unsafe extern "C" fn sni_rm200_irq_init() { sni_rm200_i8259_irqs(); mips_cpu_irq_init(); for i in SNI_RM200_INT_START..=SNI_RM200_INT_END { irq_set_chip_and_handler(i, &mut RM200_IRQ_TYPE, handle_level_irq); } sni_hwint = Some(sni_rm200_hwint); change_c0_status(0xff00, 1); }
#[no_mangle] pub unsafe extern "C" fn sni_rm200_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
