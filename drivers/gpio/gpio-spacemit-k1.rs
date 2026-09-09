// SPDX-License-Identifier: GPL-2.0 OR MIT
/*
 * Copyright (C) 2023-2025 SpacemiT (Hangzhou) Technology Co. Ltd
 * Copyright (C) 2025 Yixun Lan <dlan@gentoo.org>
 */

// Linux kernel dependencies supplied by the surrounding Rust environment.

const SPACEMIT_NR_BANKS: usize = 4;
const SPACEMIT_NR_GPIOS_PER_BANK: u32 = 32;

#[repr(usize)]
enum SpacemitGpioRegisters {
    SPACEMIT_GPLR,
    SPACEMIT_GPDR,
    SPACEMIT_GPSR,
    SPACEMIT_GPCR,
    SPACEMIT_GRER,
    SPACEMIT_GFER,
    SPACEMIT_GEDR,
    SPACEMIT_GSDR,
    SPACEMIT_GCDR,
    SPACEMIT_GSRER,
    SPACEMIT_GCRER,
    SPACEMIT_GSFER,
    SPACEMIT_GCFER,
    SPACEMIT_GAPMASK,
    SPACEMIT_GCPMASK,
}

#[repr(C)]
struct SpacemitGpioData {
    offsets: *const u32,
    bank_offsets: [u32; SPACEMIT_NR_BANKS],
}

#[repr(C)]
struct SpacemitGpioBank {
    chip: gpio_generic_chip,
    sg: *mut SpacemitGpio,
    base: *mut core::ffi::c_void,
    irq_mask: u32,
    irq_rising_edge: u32,
    irq_falling_edge: u32,
}

#[repr(C)]
struct SpacemitGpio {
    dev: *mut device,
    data: *const SpacemitGpioData,
    sgb: [SpacemitGpioBank; SPACEMIT_NR_BANKS],
}

unsafe fn spacemit_gpio_read(gb: *mut SpacemitGpioBank, reg: SpacemitGpioRegisters) -> u32 {
    let offset = *(*(*gb).sg).data.offsets.add(reg as usize);
    readl((*gb).base.add(offset as usize))
}

unsafe fn spacemit_gpio_write(gb: *mut SpacemitGpioBank, reg: SpacemitGpioRegisters, val: u32) {
    let offset = *(*(*gb).sg).data.offsets.add(reg as usize);
    writel(val, (*gb).base.add(offset as usize));
}

unsafe fn spacemit_gpio_bank_index(gb: *mut SpacemitGpioBank) -> u32 {
    gb.offset_from((*gb).sg.as_ref().unwrap().sgb.as_ptr() as *mut SpacemitGpioBank) as u32
}

unsafe extern "C" fn spacemit_gpio_irq_handler(irq: i32, dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let gb = dev_id as *mut SpacemitGpioBank;
    let gedr = spacemit_gpio_read(gb, SpacemitGpioRegisters::SPACEMIT_GEDR);
    if gedr == 0 { return IRQ_NONE; }
    spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GEDR, gedr);
    let pending = gedr & (*gb).irq_mask;
    if pending == 0 { return IRQ_NONE; }
    let mut n = 0u32;
    while n < core::mem::size_of::<usize>() as u32 * 8 {
        if pending & (1u32 << n) != 0 {
            handle_nested_irq(irq_find_mapping((*gb).chip.gc.irq.domain, n));
        }
        n += 1;
    }
    IRQ_HANDLED
}

unsafe extern "C" fn spacemit_gpio_irq_ack(d: *mut irq_data) {
    let gb = irq_data_get_irq_chip_data(d);
    spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GEDR, 1u32 << irqd_to_hwirq(d));
}

unsafe extern "C" fn spacemit_gpio_irq_mask(d: *mut irq_data) {
    let gb = irq_data_get_irq_chip_data(d);
    let bit = 1u32 << irqd_to_hwirq(d);
    (*gb).irq_mask &= !bit;
    spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GAPMASK, (*gb).irq_mask);
    if bit & (*gb).irq_rising_edge != 0 { spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GCRER, bit); }
    if bit & (*gb).irq_falling_edge != 0 { spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GCFER, bit); }
}

unsafe extern "C" fn spacemit_gpio_irq_unmask(d: *mut irq_data) {
    let gb = irq_data_get_irq_chip_data(d);
    let bit = 1u32 << irqd_to_hwirq(d);
    (*gb).irq_mask |= bit;
    if bit & (*gb).irq_rising_edge != 0 { spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GSRER, bit); }
    if bit & (*gb).irq_falling_edge != 0 { spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GSFER, bit); }
    spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GAPMASK, (*gb).irq_mask);
}

unsafe extern "C" fn spacemit_gpio_irq_set_type(d: *mut irq_data, kind: u32) -> i32 {
    let gb = irq_data_get_irq_chip_data(d);
    let bit = 1u32 << irqd_to_hwirq(d);
    if kind & IRQ_TYPE_EDGE_RISING != 0 { (*gb).irq_rising_edge |= bit; spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GSRER, bit); }
    else { (*gb).irq_rising_edge &= !bit; spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GCRER, bit); }
    if kind & IRQ_TYPE_EDGE_FALLING != 0 { (*gb).irq_falling_edge |= bit; spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GSFER, bit); }
    else { (*gb).irq_falling_edge &= !bit; spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GCFER, bit); }
    0
}

unsafe extern "C" fn spacemit_gpio_irq_print_chip(data: *mut irq_data, p: *mut seq_file) {
    let gb = irq_data_get_irq_chip_data(data);
    seq_printf(p, "%s-%d", dev_name((*gb).chip.gc.parent), spacemit_gpio_bank_index(gb));
}

static mut SPACEMIT_GPIO_CHIP: irq_chip = irq_chip {
    name: b"k1-gpio-irqchip\0".as_ptr() as *const _,
    irq_ack: Some(spacemit_gpio_irq_ack), irq_mask: Some(spacemit_gpio_irq_mask),
    irq_unmask: Some(spacemit_gpio_irq_unmask), irq_set_type: Some(spacemit_gpio_irq_set_type),
    irq_print_chip: Some(spacemit_gpio_irq_print_chip), flags: IRQCHIP_IMMUTABLE | IRQCHIP_SKIP_SET_WAKE,
};

unsafe extern "C" fn spacemit_of_node_instance_match(gc: *mut gpio_chip, i: u32) -> bool {
    if i as usize >= SPACEMIT_NR_BANKS { return false; }
    let gb = gpiochip_get_data(gc);
    gc == &mut (*gb).chip.gc as *mut _
}

unsafe fn spacemit_gpio_add_bank(sg: *mut SpacemitGpio, regs: *mut core::ffi::c_void, index: i32, irq: i32) -> i32 {
    let gb = &mut (*sg).sgb[index as usize];
    gb.base = regs.add((*(*sg).data).bank_offsets[index as usize] as usize);
    gb.sg = sg;
    let dat = gb.base.add(*(*sg).data.offsets.add(SpacemitGpioRegisters::SPACEMIT_GPLR as usize) as usize);
    let set = gb.base.add(*(*sg).data.offsets.add(SpacemitGpioRegisters::SPACEMIT_GPSR as usize) as usize);
    let clr = gb.base.add(*(*sg).data.offsets.add(SpacemitGpioRegisters::SPACEMIT_GPCR as usize) as usize);
    let dirout = gb.base.add(*(*sg).data.offsets.add(SpacemitGpioRegisters::SPACEMIT_GPDR as usize) as usize);
    let config = gpio_generic_chip_config { dev: (*sg).dev, sz: 4, dat, set, clr, dirout, flags: GPIO_GENERIC_UNREADABLE_REG_SET };
    let ret = gpio_generic_chip_init(&mut gb.chip, &config);
    if ret != 0 { return dev_err_probe((*sg).dev, ret, b"failed to init gpio chip\0".as_ptr() as _); }
    let gc = &mut gb.chip.gc;
    gc.label = dev_name((*sg).dev); gc.ngpio = SPACEMIT_NR_GPIOS_PER_BANK; gc.base = -1;
    gc.of_gpio_n_cells = 3; gc.of_node_instance_match = Some(spacemit_of_node_instance_match);
    gc.request = Some(gpiochip_generic_request); gc.free = Some(gpiochip_generic_free); gc.set_config = Some(gpiochip_generic_config);
    gc.irq.threaded = true; gc.irq.handler = Some(handle_simple_irq); gpio_irq_chip_set_chip(&mut gc.irq, &raw mut SPACEMIT_GPIO_CHIP);
    spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GAPMASK, 0);
    spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GRER, 0); spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GFER, 0);
    spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GCRER, 0xffff_ffff); spacemit_gpio_write(gb, SpacemitGpioRegisters::SPACEMIT_GCFER, 0xffff_ffff);
    let ret = devm_request_threaded_irq((*sg).dev, irq, None, Some(spacemit_gpio_irq_handler), IRQF_ONESHOT | IRQF_SHARED, gc.label, gb as *mut _ as _);
    if ret < 0 { return dev_err_probe((*sg).dev, ret, b"failed to register IRQ\0".as_ptr() as _); }
    let ret = devm_gpiochip_add_data((*sg).dev, gc, gb as *mut _ as _); if ret != 0 { return ret; }
    irq_domain_update_bus_token(gc.irq.domain, DOMAIN_BUS_WIRED); 0
}

unsafe fn spacemit_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev; let sg = devm_kzalloc(dev, core::mem::size_of::<SpacemitGpio>(), GFP_KERNEL) as *mut SpacemitGpio;
    if sg.is_null() { return -ENOMEM; }
    (*sg).data = of_device_get_match_data(dev); if (*sg).data.is_null() { return dev_err_probe(dev, -EINVAL, b"No available compatible data.\0".as_ptr() as _); }
    let regs = devm_platform_ioremap_resource(pdev, 0); if is_err(regs) { return ptr_err(regs); }
    let irq = platform_get_irq(pdev, 0); if irq < 0 { return irq; } (*sg).dev = dev;
    if is_err(devm_clk_get_enabled(dev, b"core\0".as_ptr() as _)) { return -EINVAL; }
    if is_err(devm_clk_get_enabled(dev, b"bus\0".as_ptr() as _)) { return -EINVAL; }
    for i in 0..SPACEMIT_NR_BANKS { let ret = spacemit_gpio_add_bank(sg, regs, i as i32, irq); if ret != 0 { return ret; } } 0
}

static SPACEMIT_GPIO_K1_OFFSETS: [u32; 15] = [0x00,0x0c,0x18,0x24,0x30,0x3c,0x48,0x54,0x60,0x6c,0x78,0x84,0x90,0x9c,0xA8];
static SPACEMIT_GPIO_K3_OFFSETS: [u32; 15] = [0x0,0x4,0x8,0xc,0x10,0x14,0x18,0x1c,0x20,0x24,0x28,0x2c,0x30,0x34,0x38];

static K1_GPIO_DATA: SpacemitGpioData = SpacemitGpioData { offsets: SPACEMIT_GPIO_K1_OFFSETS.as_ptr(), bank_offsets: [0x0,0x4,0x8,0x100] };
static K3_GPIO_DATA: SpacemitGpioData = SpacemitGpioData { offsets: SPACEMIT_GPIO_K3_OFFSETS.as_ptr(), bank_offsets: [0x0,0x40,0x80,0x100] };

// Device-table and module registration are provided by the kernel-facing Rust bindings.
// MODULE_DEVICE_TABLE(of, spacemit_gpio_dt_ids);
// module_platform_driver(spacemit_gpio_driver);
// MODULE_AUTHOR("Yixun Lan <dlan@gentoo.org>");
// MODULE_DESCRIPTION("GPIO driver for SpacemiT K1/K3 SoC");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
