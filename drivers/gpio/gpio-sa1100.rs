// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/arch/arm/mach-sa1100/gpio.c
 *
 * Generic SA-1100 GPIO handling
 */

// External kernel/platform dependencies supplied by other translation units.
use core::ffi::c_void;

#[repr(C)]
pub struct software_node { pub name: *const u8 }
#[repr(C)] pub struct gpio_chip { pub label: *const u8, pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>, pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>, pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>, pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>, pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>, pub to_irq: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>, pub base: i32, pub ngpio: u32, pub fwnode: *mut c_void }
#[repr(C)] pub struct irq_data { pub hwirq: u32 }
#[repr(C)] pub struct irq_desc;
#[repr(C)] pub struct irq_domain { pub host_data: *mut c_void }
#[repr(C)] pub struct irq_chip { pub name: *const u8 }
#[repr(C)] pub struct irq_domain_ops;
#[repr(C)] pub struct syscore_ops;
#[repr(C)] pub struct syscore { pub ops: *const syscore_ops }

extern "C" {
    static GPLR: u32;
    fn readl_relaxed(addr: *const c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut c_void);
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn irq_data_get_irq_chip_data(d: *mut irq_data) -> *mut c_void;
    fn sa11x0_gpio_set_wake(hwirq: u32, on: u32) -> i32;
    fn irq_set_chip_data(irq: u32, data: *mut c_void);
    fn irq_set_chip_and_handler(irq: u32, chip: *mut irq_chip, handler: unsafe extern "C" fn(*mut irq_desc));
    fn irq_set_probe(irq: u32);
    fn irq_desc_get_handler_data(desc: *mut irq_desc) -> *mut c_void;
    fn generic_handle_irq(irq: u32);
    fn register_syscore(syscore: *mut syscore);
    fn software_node_register(node: *const software_node);
    fn software_node_fwnode(node: *const software_node) -> *mut c_void;
    fn gpiochip_add_data(chip: *mut gpio_chip, data: *mut c_void) -> i32;
    fn irq_domain_create_simple(first: *mut c_void, size: u32, first_irq: u32, ops: *const irq_domain_ops, host_data: *mut c_void) -> *mut irq_domain;
    fn irq_set_chained_handler_and_data(irq: u32, handler: unsafe extern "C" fn(*mut irq_desc), data: *mut c_void);
    fn handle_edge_irq(desc: *mut irq_desc);
}

pub static sa1100_gpiochip_node: software_node = software_node { name: b"sa1100-gpio\0".as_ptr() };

pub const IRQ_GPIO0: i32 = 0;
pub const IRQ_GPIO0_SC: u32 = 0;
pub const IRQ_GPIO1_SC: u32 = 1;
pub const IRQ_GPIO2_SC: u32 = 2;
pub const IRQ_GPIO3_SC: u32 = 3;
pub const IRQ_GPIO4_SC: u32 = 4;
pub const IRQ_GPIO5_SC: u32 = 5;
pub const IRQ_GPIO6_SC: u32 = 6;
pub const IRQ_GPIO7_SC: u32 = 7;
pub const IRQ_GPIO8_SC: u32 = 8;
pub const IRQ_GPIO9_SC: u32 = 9;
pub const IRQ_GPIO10_SC: u32 = 10;
pub const IRQ_GPIO11_27: u32 = 11;

#[allow(dead_code)]
static SA1100_GPIO_IRQS: [u32; 12] = [IRQ_GPIO0_SC, IRQ_GPIO1_SC, IRQ_GPIO2_SC, IRQ_GPIO3_SC, IRQ_GPIO4_SC, IRQ_GPIO5_SC, IRQ_GPIO6_SC, IRQ_GPIO7_SC, IRQ_GPIO8_SC, IRQ_GPIO9_SC, IRQ_GPIO10_SC, IRQ_GPIO11_27];

// GPIO IRQ chip, domain operations, and syscore registration correspond to the
// kernel structures initialized in the C implementation.
static mut SA1100_GPIO_IRQ_CHIP: irq_chip = irq_chip { name: b"GPIO\0".as_ptr() };
static mut SA1100_GPIO_SYSCORE_OPS: *const syscore_ops = core::ptr::null();
static mut SA1100_GPIO_SYSCORE: syscore = syscore { ops: core::ptr::null() };

unsafe extern "C" fn sa1100_gpio_init_devicefs() -> i32 { register_syscore(core::ptr::addr_of_mut!(SA1100_GPIO_SYSCORE)); 0 }
// device_initcall(sa1100_gpio_init_devicefs);

unsafe extern "C" fn sa1100_gpio_irqdomain_map(_d: *mut irq_domain, irq: u32, _hwirq: u32) -> i32 {
    irq_set_chip_data(irq, core::ptr::addr_of_mut!(SA1100_GPIO_CHIP) as *mut c_void);
    irq_set_chip_and_handler(irq, core::ptr::addr_of_mut!(SA1100_GPIO_IRQ_CHIP), handle_edge_irq);
    irq_set_probe(irq);
    0
}

// .map = sa1100_gpio_irqdomain_map, .xlate = irq_domain_xlate_onetwocell
static SA1100_GPIO_IRQDOMAIN_OPS: irq_domain_ops = unsafe { core::mem::zeroed() };

pub const R_GPLR: usize = 0x00;
pub const R_GPDR: usize = 0x04;
pub const R_GPSR: usize = 0x08;
pub const R_GPCR: usize = 0x0c;
pub const R_GRER: usize = 0x10;
pub const R_GFER: usize = 0x14;
pub const R_GEDR: usize = 0x18;
pub const R_GAFR: usize = 0x1c;

#[repr(C)]
pub struct sa1100_gpio_chip {
    pub chip: gpio_chip,
    pub membase: *mut u8,
    pub irqbase: i32,
    pub irqmask: u32,
    pub irqrising: u32,
    pub irqfalling: u32,
    pub irqwake: u32,
}

#[inline]
unsafe fn sa1100_gpio_chip(x: *mut gpio_chip) -> *mut sa1100_gpio_chip {
    (x as *mut u8).sub(core::mem::offset_of!(sa1100_gpio_chip, chip)) as *mut sa1100_gpio_chip
}

unsafe extern "C" fn sa1100_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    (readl_relaxed((*sa1100_gpio_chip(chip)).membase.add(R_GPLR) as *const c_void) & (1u32 << offset)) as i32
}

unsafe extern "C" fn sa1100_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) {
    let reg = if value != 0 { R_GPSR } else { R_GPCR };
    writel_relaxed(1u32 << offset, (*sa1100_gpio_chip(chip)).membase.add(reg) as *mut c_void);
}

unsafe extern "C" fn sa1100_get_direction(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpdr = (*sa1100_gpio_chip(chip)).membase.add(R_GPDR);
    if readl_relaxed(gpdr as *const c_void) & (1u32 << offset) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn sa1100_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpdr = (*sa1100_gpio_chip(chip)).membase.add(R_GPDR); let mut flags = 0usize;
    local_irq_save(&mut flags); writel_relaxed(readl_relaxed(gpdr as *const c_void) & !(1u32 << offset), gpdr as *mut c_void); local_irq_restore(flags); 0
}

unsafe extern "C" fn sa1100_direction_output(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpdr = (*sa1100_gpio_chip(chip)).membase.add(R_GPDR); let mut flags = 0usize;
    local_irq_save(&mut flags); sa1100_gpio_set(chip, offset, value); writel_relaxed(readl_relaxed(gpdr as *const c_void) | (1u32 << offset), gpdr as *mut c_void); local_irq_restore(flags); 0
}

unsafe extern "C" fn sa1100_to_irq(chip: *mut gpio_chip, offset: u32) -> i32 { (*sa1100_gpio_chip(chip)).irqbase + offset as i32 }

static mut SA1100_GPIO_CHIP: sa1100_gpio_chip = sa1100_gpio_chip { chip: gpio_chip { label: b"gpio\0".as_ptr(), get_direction: Some(sa1100_get_direction), direction_input: Some(sa1100_direction_input), direction_output: Some(sa1100_direction_output), set: Some(sa1100_gpio_set), get: Some(sa1100_gpio_get), to_irq: Some(sa1100_to_irq), base: 0, ngpio: 28, fwnode: core::ptr::null_mut() }, membase: core::ptr::addr_of!(GPLR) as *mut u8, irqbase: 0, irqmask: 0, irqrising: 0, irqfalling: 0, irqwake: 0 };

unsafe fn sa1100_update_edge_regs(sgc: *mut sa1100_gpio_chip) { let base = (*sgc).membase; writel_relaxed((*sgc).irqrising & (*sgc).irqmask, base.add(R_GRER) as *mut c_void); writel_relaxed((*sgc).irqfalling & (*sgc).irqmask, base.add(R_GFER) as *mut c_void); }

unsafe extern "C" fn sa1100_gpio_type(d: *mut irq_data, mut typ: u32) -> i32 { let sgc = irq_data_get_irq_chip_data(d) as *mut sa1100_gpio_chip; let mask = 1u32 << (*d).hwirq; if typ == 0 { if ((*sgc).irqrising | (*sgc).irqfalling) & mask != 0 { return 0; } typ = 3; } if typ & 1 != 0 { (*sgc).irqrising |= mask; } else { (*sgc).irqrising &= !mask; } if typ & 2 != 0 { (*sgc).irqfalling |= mask; } else { (*sgc).irqfalling &= !mask; } sa1100_update_edge_regs(sgc); 0 }
unsafe extern "C" fn sa1100_gpio_ack(d: *mut irq_data) { let sgc = irq_data_get_irq_chip_data(d) as *mut sa1100_gpio_chip; writel_relaxed(1u32 << (*d).hwirq, (*sgc).membase.add(R_GEDR) as *mut c_void); }
unsafe extern "C" fn sa1100_gpio_mask(d: *mut irq_data) { let sgc = irq_data_get_irq_chip_data(d) as *mut sa1100_gpio_chip; (*sgc).irqmask &= !(1u32 << (*d).hwirq); sa1100_update_edge_regs(sgc); }
unsafe extern "C" fn sa1100_gpio_unmask(d: *mut irq_data) { let sgc = irq_data_get_irq_chip_data(d) as *mut sa1100_gpio_chip; (*sgc).irqmask |= 1u32 << (*d).hwirq; sa1100_update_edge_regs(sgc); }
unsafe extern "C" fn sa1100_gpio_wake(d: *mut irq_data, on: u32) -> i32 { let sgc = irq_data_get_irq_chip_data(d) as *mut sa1100_gpio_chip; let ret = sa11x0_gpio_set_wake((*d).hwirq, on); if ret == 0 { if on != 0 { (*sgc).irqwake |= 1u32 << (*d).hwirq; } else { (*sgc).irqwake &= !(1u32 << (*d).hwirq); } } ret }

static mut SA1100_GPIO_IRQDOMAIN: *mut irq_domain = core::ptr::null_mut();

unsafe extern "C" fn sa1100_gpio_handler(desc: *mut irq_desc) { let sgc = irq_desc_get_handler_data(desc) as *mut sa1100_gpio_chip; let gedr = (*sgc).membase.add(R_GEDR) as *mut c_void; let mut mask = readl_relaxed(gedr); while mask != 0 { writel_relaxed(mask, gedr as *mut c_void); let mut irq = (*sgc).irqbase as u32; while mask != 0 { if mask & 1 != 0 { generic_handle_irq(irq); } mask >>= 1; irq += 1; } mask = readl_relaxed(gedr); } }

unsafe extern "C" fn sa1100_gpio_suspend(_data: *mut c_void) -> i32 { let sgc = core::ptr::addr_of_mut!(SA1100_GPIO_CHIP); writel_relaxed((*sgc).irqwake & (*sgc).irqrising, (*sgc).membase.add(R_GRER) as *mut c_void); writel_relaxed((*sgc).irqwake & (*sgc).irqfalling, (*sgc).membase.add(R_GFER) as *mut c_void); let p = (*sgc).membase.add(R_GEDR) as *mut c_void; writel_relaxed(readl_relaxed(p), p as *mut c_void); 0 }
unsafe extern "C" fn sa1100_gpio_resume(_data: *mut c_void) { sa1100_update_edge_regs(core::ptr::addr_of_mut!(SA1100_GPIO_CHIP)); }

pub unsafe extern "C" fn sa1100_init_gpio() { let sgc = core::ptr::addr_of_mut!(SA1100_GPIO_CHIP); let gc = &mut (*sgc).chip; writel_relaxed(0, (*sgc).membase.add(R_GFER) as *mut c_void); writel_relaxed(0, (*sgc).membase.add(R_GRER) as *mut c_void); writel_relaxed(u32::MAX, (*sgc).membase.add(R_GEDR) as *mut c_void); software_node_register(&sa1100_gpiochip_node); gc.fwnode = software_node_fwnode(&sa1100_gpiochip_node); gpiochip_add_data(gc, core::ptr::null_mut()); SA1100_GPIO_IRQDOMAIN = irq_domain_create_simple(core::ptr::null_mut(), 28, IRQ_GPIO0, &SA1100_GPIO_IRQDOMAIN_OPS, sgc); for irq in SA1100_GPIO_IRQS { irq_set_chained_handler_and_data(irq, sa1100_gpio_handler, sgc as *mut c_void); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
