// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * i8259 interrupt controller driver.
 */

// C dependencies from the kernel headers are supplied externally.

static mut pci_intack: *mut core::ffi::c_void = core::ptr::null_mut(); /* RO, gives us the irq vector */

static mut cached_8259: [u8; 2] = [0xff, 0xff];

static mut i8259_lock: raw_spinlock_t = raw_spinlock_t {};
static mut i8259_host: *mut irq_domain = core::ptr::null_mut();

extern "C" {
    fn raw_spin_lock(lock: *mut raw_spinlock_t);
    fn raw_spin_unlock(lock: *mut raw_spinlock_t);
    fn raw_spin_lock_irqsave(lock: *mut raw_spinlock_t, flags: *mut c_ulong);
    fn raw_spin_unlock_irqrestore(lock: *mut raw_spinlock_t, flags: c_ulong);
    fn readb(addr: *const core::ffi::c_void) -> c_int;
    fn inb(port: u16) -> c_int;
    fn outb(value: u8, port: u16);
    fn udelay(usecs: u64);
    fn irq_set_status_flags(virq: c_uint, flags: c_uint);
    fn irq_set_chip_and_handler(virq: c_uint, chip: *mut irq_chip, handler: unsafe extern "C" fn());
    fn irq_domain_get_of_node(h: *mut irq_domain) -> *mut device_node;
    fn irq_domain_create_legacy(fwnode: *mut fwnode_handle, size: c_uint, first_hwirq: c_uint,
                                hwirq: c_uint, ops: *const irq_domain_ops, host_data: *mut core::ffi::c_void) -> *mut irq_domain;
    fn of_fwnode_handle(node: *mut device_node) -> *mut fwnode_handle;
    fn request_resource(parent: *mut resource, child: *mut resource) -> c_int;
    fn ioremap(addr: c_ulong, size: usize) -> *mut core::ffi::c_void;
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn pr_debug(fmt: *const core::ffi::c_char, ...);
}

type c_int = i32;
type c_uint = u32;
type c_ulong = usize;
type irq_hw_number_t = c_ulong;

#[repr(C)] pub struct raw_spinlock_t {}
#[repr(C)] pub struct irq_domain {}
#[repr(C)] pub struct device_node {}
#[repr(C)] pub struct fwnode_handle {}
#[repr(C)] pub struct irq_data { pub irq: c_uint }
#[repr(C)] pub struct irq_chip {
    pub name: *const core::ffi::c_char,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_disable: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
}
#[repr(C)] pub struct resource { pub name: *const core::ffi::c_char, pub start: c_ulong, pub end: c_ulong, pub flags: c_ulong }
#[repr(C)] pub struct irq_domain_ops {
    pub match_: Option<unsafe extern "C" fn(*mut irq_domain, *mut device_node, c_uint) -> c_int>,
    pub map: Option<unsafe extern "C" fn(*mut irq_domain, c_uint, irq_hw_number_t) -> c_int>,
    pub xlate: Option<unsafe extern "C" fn(*mut irq_domain, *mut device_node, *const u32, c_uint, *mut irq_hw_number_t, *mut c_uint) -> c_int>,
}

const IRQ_NOREQUEST: c_uint = 0x0001;
const IRQ_LEVEL: c_uint = 0x0002;
const IRQ_TYPE_LEVEL_LOW: u8 = 8;
const IRQ_TYPE_LEVEL_HIGH: u8 = 4;
const IRQ_TYPE_EDGE_FALLING: u8 = 2;
const IRQ_TYPE_EDGE_RISING: u8 = 1;
const IRQ_TYPE_NONE: u8 = 0;
const IORESOURCE_IO: c_ulong = 0x00000100;
const IORESOURCE_BUSY: c_ulong = 0x80000000;
const NR_IRQS_LEGACY: c_uint = 16;

extern "C" { fn handle_level_irq(); }

#[inline] unsafe fn cached_a1() -> u8 { cached_8259[0] }
#[inline] unsafe fn cached_21() -> u8 { cached_8259[1] }

#[no_mangle]
pub unsafe extern "C" fn i8259_irq() -> c_uint {
    let mut irq: c_int;
    let mut lock = 0;
    if !pci_intack.is_null() { irq = readb(pci_intack); } else {
        raw_spin_lock(&mut i8259_lock); lock = 1;
        outb(0x0c, 0x20); irq = inb(0x20) & 7;
        if irq == 2 { outb(0x0c, 0xa0); irq = (inb(0xa0) & 7) + 8; }
    }
    if irq == 7 {
        if pci_intack.is_null() { outb(0x0b, 0x20); }
        if (!inb(0x20) & 0x80) != 0 { irq = 0; }
    } else if irq == 0xff { irq = 0; }
    if lock != 0 { raw_spin_unlock(&mut i8259_lock); }
    irq as c_uint
}

unsafe extern "C" fn i8259_mask_and_ack_irq(d: *mut irq_data) {
    let mut flags = 0; raw_spin_lock_irqsave(&mut i8259_lock, &mut flags);
    if (*d).irq > 7 { cached_8259[0] |= 1 << ((*d).irq - 8); inb(0xa1); outb(cached_a1(), 0xa1); outb(0x20, 0xa0); outb(0x20, 0x20); }
    else { cached_8259[1] |= 1 << (*d).irq; inb(0x21); outb(cached_21(), 0x21); outb(0x20, 0x20); }
    raw_spin_unlock_irqrestore(&mut i8259_lock, flags);
}

unsafe fn i8259_set_irq_mask(_irq_nr: c_int) { outb(cached_a1(), 0xa1); outb(cached_21(), 0x21); }

unsafe extern "C" fn i8259_mask_irq(d: *mut irq_data) {
    let mut flags = 0; pr_debug(b"i8259_mask_irq(%d)\0".as_ptr() as _, (*d).irq as c_int);
    raw_spin_lock_irqsave(&mut i8259_lock, &mut flags);
    if (*d).irq < 8 { cached_8259[1] |= 1 << (*d).irq; } else { cached_8259[0] |= 1 << ((*d).irq - 8); }
    i8259_set_irq_mask((*d).irq as c_int); raw_spin_unlock_irqrestore(&mut i8259_lock, flags);
}

unsafe extern "C" fn i8259_unmask_irq(d: *mut irq_data) {
    let mut flags = 0; pr_debug(b"i8259_unmask_irq(%d)\0".as_ptr() as _, (*d).irq as c_int);
    raw_spin_lock_irqsave(&mut i8259_lock, &mut flags);
    if (*d).irq < 8 { cached_8259[1] &= !(1 << (*d).irq); } else { cached_8259[0] &= !(1 << ((*d).irq - 8)); }
    i8259_set_irq_mask((*d).irq as c_int); raw_spin_unlock_irqrestore(&mut i8259_lock, flags);
}

static mut i8259_pic: irq_chip = irq_chip { name: b"i8259\0".as_ptr() as _, irq_mask: Some(i8259_mask_irq), irq_disable: Some(i8259_mask_irq), irq_unmask: Some(i8259_unmask_irq), irq_mask_ack: Some(i8259_mask_and_ack_irq) };

static mut pic1_iores: resource = resource { name: b"8259 (master)\0".as_ptr() as _, start: 0x20, end: 0x21, flags: IORESOURCE_IO | IORESOURCE_BUSY };
static mut pic2_iores: resource = resource { name: b"8259 (slave)\0".as_ptr() as _, start: 0xa0, end: 0xa1, flags: IORESOURCE_IO | IORESOURCE_BUSY };
static mut pic_edgectrl_iores: resource = resource { name: b"8259 edge control\0".as_ptr() as _, start: 0x4d0, end: 0x4d1, flags: IORESOURCE_IO | IORESOURCE_BUSY };

unsafe extern "C" fn i8259_host_match(h: *mut irq_domain, node: *mut device_node, _bus_token: c_uint) -> c_int { let of_node = irq_domain_get_of_node(h); (of_node.is_null() || of_node == node) as c_int }
unsafe extern "C" fn i8259_host_map(_h: *mut irq_domain, virq: c_uint, hw: irq_hw_number_t) -> c_int {
    pr_debug(b"i8259_host_map(%d, 0x%lx)\0".as_ptr() as _, virq as c_int, hw);
    if hw == 2 { irq_set_status_flags(virq, IRQ_NOREQUEST); } irq_set_status_flags(virq, IRQ_LEVEL); irq_set_chip_and_handler(virq, &mut i8259_pic, handle_level_irq); 0
}
unsafe extern "C" fn i8259_host_xlate(_h: *mut irq_domain, _ct: *mut device_node, intspec: *const u32, intsize: c_uint, out_hwirq: *mut irq_hw_number_t, out_flags: *mut c_uint) -> c_int {
    let map = [IRQ_TYPE_LEVEL_LOW as c_uint, IRQ_TYPE_LEVEL_HIGH as c_uint, IRQ_TYPE_EDGE_FALLING as c_uint, IRQ_TYPE_EDGE_RISING as c_uint]; *out_hwirq = *intspec as irq_hw_number_t; if intsize > 1 && *intspec.add(1) < 4 { *out_flags = map[*intspec.add(1) as usize]; } else { *out_flags = IRQ_TYPE_NONE as c_uint; } 0
}
static i8259_host_ops: irq_domain_ops = irq_domain_ops { match_: Some(i8259_host_match), map: Some(i8259_host_map), xlate: Some(i8259_host_xlate) };

pub unsafe extern "C" fn i8259_get_host() -> *mut irq_domain { i8259_host }

pub unsafe extern "C" fn i8259_init(node: *mut device_node, intack_addr: c_ulong) {
    let mut flags = 0; raw_spin_lock_irqsave(&mut i8259_lock, &mut flags);
    outb(0xff, 0xa1); outb(0xff, 0x21); outb(0x11, 0x20); outb(0, 0x21); outb(4, 0x21); outb(1, 0x21); outb(0x11, 0xa0); outb(8, 0xa1); outb(2, 0xa1); outb(1, 0xa1); udelay(100); outb(0x0b, 0x20); outb(0x0b, 0xa0); cached_8259[1] &= !(1 << 2); outb(cached_a1(), 0xa1); outb(cached_21(), 0x21); raw_spin_unlock_irqrestore(&mut i8259_lock, flags);
    i8259_host = irq_domain_create_legacy(of_fwnode_handle(node), NR_IRQS_LEGACY, 0, 0, &i8259_host_ops, core::ptr::null_mut()); if i8259_host.is_null() { printk(b"i8259: failed to allocate irq host !\n\0".as_ptr() as _); return; }
    request_resource(&mut ioport_resource, &mut pic1_iores); request_resource(&mut ioport_resource, &mut pic2_iores); request_resource(&mut ioport_resource, &mut pic_edgectrl_iores); if intack_addr != 0 { pci_intack = ioremap(intack_addr, 1); } printk(b"i8259 legacy interrupt controller initialized\n\0".as_ptr() as _);
}

extern "C" { static mut ioport_resource: resource; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
