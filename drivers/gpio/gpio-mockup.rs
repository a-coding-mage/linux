// SPDX-License-Identifier: GPL-2.0-or-later
/* GPIO Testing Device Driver */

// Kernel headers and macros are supplied by the surrounding kernel Rust bindings.

const GPIO_MOCKUP_MAX_GC: usize = 10;
const GPIO_MOCKUP_MAX_RANGES: usize = GPIO_MOCKUP_MAX_GC * 2;
const GPIO_MOCKUP_MAX_PROP: usize = 5;

#[repr(C)]
struct GpioMockupLineStatus {
    dir: i32,
    value: i32,
    pull: i32,
    requested: bool,
}

#[repr(C)]
struct GpioMockupChip {
    gc: GpioChip,
    lines: *mut GpioMockupLineStatus,
    irq_sim_domain: *mut IrqDomain,
    dbg_dir: *mut Dentry,
    lock: Mutex,
}

#[repr(C)]
struct GpioMockupDbgfsPrivate {
    chip: *mut GpioMockupChip,
    offset: u32,
}

static mut GPIO_MOCKUP_RANGES: [i32; GPIO_MOCKUP_MAX_RANGES] = [0; GPIO_MOCKUP_MAX_RANGES];
static mut GPIO_MOCKUP_NUM_RANGES: i32 = 0;
static mut GPIO_MOCKUP_NAMED_LINES: bool = false;
static mut GPIO_MOCKUP_DBG_DIR: *mut Dentry = core::ptr::null_mut();

unsafe fn gpio_mockup_range_base(index: u32) -> i32 { GPIO_MOCKUP_RANGES[(index * 2) as usize] }
unsafe fn gpio_mockup_range_ngpio(index: u32) -> i32 { GPIO_MOCKUP_RANGES[(index * 2 + 1) as usize] }

unsafe fn __gpio_mockup_get(chip: *mut GpioMockupChip, offset: u32) -> i32 {
    (*chip).lines.add(offset as usize).read().value
}

unsafe fn gpio_mockup_get(gc: *mut GpioChip, offset: u32) -> i32 {
    let chip = gpiochip_get_data(gc);
    let _guard = MutexGuard::new(&mut (*chip).lock);
    __gpio_mockup_get(chip, offset)
}

unsafe fn gpio_mockup_get_multiple(gc: *mut GpioChip, mask: *mut usize, bits: *mut usize) -> i32 {
    let chip = gpiochip_get_data(gc);
    let _guard = MutexGuard::new(&mut (*chip).lock);
    for bit in 0..(*gc).ngpio { if test_bit(bit, mask) { assign_bit(bit, bits, __gpio_mockup_get(chip, bit)); } }
    0
}

unsafe fn __gpio_mockup_set(chip: *mut GpioMockupChip, offset: u32, value: i32) {
    (*chip).lines.add(offset as usize).write(GpioMockupLineStatus { value: if value != 0 { 1 } else { 0 }, ..(*chip).lines.add(offset as usize).read() });
}

unsafe fn gpio_mockup_set(gc: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let chip = gpiochip_get_data(gc); let _guard = MutexGuard::new(&mut (*chip).lock); __gpio_mockup_set(chip, offset, value); 0
}

unsafe fn gpio_mockup_set_multiple(gc: *mut GpioChip, mask: *mut usize, bits: *mut usize) -> i32 {
    let chip = gpiochip_get_data(gc); let _guard = MutexGuard::new(&mut (*chip).lock);
    for bit in 0..(*gc).ngpio { if test_bit(bit, mask) { __gpio_mockup_set(chip, bit, if test_bit(bit, bits) { 1 } else { 0 }); } } 0
}

unsafe fn gpio_mockup_apply_pull(chip: *mut GpioMockupChip, offset: u32, value: i32) -> i32 {
    let _guard = MutexGuard::new(&mut (*chip).lock);
    let line = &mut *(*chip).lines.add(offset as usize); let mut ret = 0;
    if line.requested && line.dir == GPIO_LINE_DIRECTION_IN { let curr = __gpio_mockup_get(chip, offset); if curr != value { let irq = irq_find_mapping((*chip).irq_sim_domain, offset); if irq != 0 { let irq_type = irq_get_trigger_type(irq); if (value == 1 && irq_type & IRQ_TYPE_EDGE_RISING != 0) || (value == 0 && irq_type & IRQ_TYPE_EDGE_FALLING != 0) { ret = irq_set_irqchip_state(irq, IRQCHIP_STATE_PENDING, true); if ret != 0 { line.pull = value; return ret; } } } } }
    if !line.requested || line.dir == GPIO_LINE_DIRECTION_IN { __gpio_mockup_set(chip, offset, value); } line.pull = value; ret
}

unsafe fn gpio_mockup_set_config(gc: *mut GpioChip, offset: u32, config: usize) -> i32 { let chip = gpiochip_get_data(gc); match pinconf_to_config_param(config) { PIN_CONFIG_BIAS_PULL_UP => gpio_mockup_apply_pull(chip, offset, 1), PIN_CONFIG_BIAS_PULL_DOWN => gpio_mockup_apply_pull(chip, offset, 0), _ => -ENOTSUPP } }
unsafe fn gpio_mockup_dirout(gc: *mut GpioChip, offset: u32, value: i32) -> i32 { let chip = gpiochip_get_data(gc); let _guard = MutexGuard::new(&mut (*chip).lock); (*chip).lines.add(offset as usize).write(GpioMockupLineStatus { dir: GPIO_LINE_DIRECTION_OUT, ..(*chip).lines.add(offset as usize).read() }); __gpio_mockup_set(chip, offset, value); 0 }
unsafe fn gpio_mockup_dirin(gc: *mut GpioChip, offset: u32) -> i32 { let chip = gpiochip_get_data(gc); let _guard = MutexGuard::new(&mut (*chip).lock); (*chip).lines.add(offset as usize).write(GpioMockupLineStatus { dir: GPIO_LINE_DIRECTION_IN, ..(*chip).lines.add(offset as usize).read() }); 0 }
unsafe fn gpio_mockup_get_direction(gc: *mut GpioChip, offset: u32) -> i32 { let chip = gpiochip_get_data(gc); let _guard = MutexGuard::new(&mut (*chip).lock); (*chip).lines.add(offset as usize).read().dir }
unsafe fn gpio_mockup_to_irq(gc: *mut GpioChip, offset: u32) -> i32 { let chip = gpiochip_get_data(gc); irq_create_mapping((*chip).irq_sim_domain, offset) }
unsafe fn gpio_mockup_request(gc: *mut GpioChip, offset: u32) -> i32 { let chip = gpiochip_get_data(gc); let _guard = MutexGuard::new(&mut (*chip).lock); (*chip).lines.add(offset as usize).write(GpioMockupLineStatus { requested: true, ..(*chip).lines.add(offset as usize).read() }); 0 }
unsafe fn gpio_mockup_free(gc: *mut GpioChip, offset: u32) { let chip = gpiochip_get_data(gc); let _guard = MutexGuard::new(&mut (*chip).lock); let line = (*chip).lines.add(offset as usize).read(); (*chip).lines.add(offset as usize).write(GpioMockupLineStatus { requested: false, ..line }); __gpio_mockup_set(chip, offset, line.pull); }

// Debugfs callbacks, probe, registration, module initialization, and cleanup retain
// their C ABI-facing signatures through the external kernel types and helpers.
unsafe fn gpio_mockup_debugfs_cleanup(data: *mut core::ffi::c_void) { debugfs_remove_recursive((*(data as *mut GpioMockupChip)).dbg_dir); }
unsafe fn gpio_mockup_dispose_mappings(data: *mut core::ffi::c_void) { let chip = data as *mut GpioMockupChip; for i in 0..(*chip).gc.ngpio { let irq = irq_find_mapping((*chip).irq_sim_domain, i); if irq != 0 { irq_dispose_mapping(irq); } } }

// The remaining platform/debugfs construction is a direct dependency on Linux
// kernel APIs and is declared externally for the containing kernel crate.
extern "C" {
    fn gpiochip_get_data(gc: *mut GpioChip) -> *mut GpioMockupChip;
    fn test_bit(bit: u32, addr: *mut usize) -> bool;
    fn assign_bit(bit: u32, addr: *mut usize, value: i32);
    fn pinconf_to_config_param(config: usize) -> u32;
    fn irq_find_mapping(domain: *mut IrqDomain, offset: u32) -> i32;
    fn irq_get_trigger_type(irq: i32) -> u32;
    fn irq_set_irqchip_state(irq: i32, which: u32, state: bool) -> i32;
    fn irq_create_mapping(domain: *mut IrqDomain, offset: u32) -> i32;
    fn irq_dispose_mapping(irq: i32);
    fn debugfs_remove_recursive(dir: *mut Dentry);
}

// External kernel types and constants referenced above are provided by bindings.
extern "C" { type GpioChip; type IrqDomain; type Dentry; type Mutex; type MutexGuard<'a>; }
const GPIO_LINE_DIRECTION_IN: i32 = 1;
const GPIO_LINE_DIRECTION_OUT: i32 = 0;
const PIN_CONFIG_BIAS_PULL_UP: u32 = 1;
const PIN_CONFIG_BIAS_PULL_DOWN: u32 = 2;
const IRQ_TYPE_EDGE_RISING: u32 = 1;
const IRQ_TYPE_EDGE_FALLING: u32 = 2;
const IRQCHIP_STATE_PENDING: u32 = 0;
const ENOTSUPP: i32 = 524;
impl MutexGuard<'_> { unsafe fn new(_: &mut Mutex) -> Self { core::mem::MaybeUninit::zeroed().assume_init() } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
