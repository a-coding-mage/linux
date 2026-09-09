// SPDX-License-Identifier: GPL-2.0
/* Driver for IDT/Renesas 79RC3243x Interrupt Controller */

const IDT_PIC_IRQ_PEND: usize = 0x00;
const IDT_PIC_IRQ_MASK: usize = 0x08;

const IDT_GPIO_DIR: usize = 0x00;
const IDT_GPIO_DATA: usize = 0x04;
const IDT_GPIO_ILEVEL: usize = 0x08;
const IDT_GPIO_ISTAT: usize = 0x0c;

#[repr(C)]
struct IdtGpioCtrl {
    chip: GpioGenericChip,
    pic: *mut core::ffi::c_void,
    gpio: *mut core::ffi::c_void,
    mask_cache: u32,
}

extern "C" {
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn chained_irq_enter(chip: *mut IrqChip, desc: *mut IrqDesc);
    fn chained_irq_exit(chip: *mut IrqChip, desc: *mut IrqDesc);
    fn irq_desc_get_handler_data(desc: *mut IrqDesc) -> *mut GpioChip;
    fn gpiochip_get_data(gc: *mut GpioChip) -> *mut IdtGpioCtrl;
    fn irq_desc_get_chip(desc: *mut IrqDesc) -> *mut IrqChip;
    fn irq_find_mapping(domain: *mut IrqDomain, bit: u32) -> u32;
    fn generic_handle_irq(irq: u32);
    fn irq_data_get_irq_chip_data(data: *mut IrqData) -> *mut GpioChip;
    fn irq_set_handler_locked(data: *mut IrqData, handler: unsafe extern "C" fn(*mut IrqDesc));
    fn gpiochip_disable_irq(gc: *mut GpioChip, irq: u32);
    fn gpiochip_enable_irq(gc: *mut GpioChip, irq: u32);
    fn irqd_to_hwirq(data: *mut IrqData) -> u32;
    fn gpio_generic_chip_init(chip: *mut GpioGenericChip, config: *mut GpioGenericChipConfig) -> i32;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource_byname(dev: *mut PlatformDevice, name: *const u8) -> *mut core::ffi::c_void;
    fn ptr_is_err(ptr: *mut core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn device_property_read_u32(dev: *mut Device, name: *const u8, value: *mut i32) -> i32;
    fn device_property_read_bool(dev: *mut Device, name: *const u8) -> bool;
    fn platform_get_irq(dev: *mut PlatformDevice, index: u32) -> i32;
    fn devm_kcalloc(dev: *mut Device, n: usize, size: usize, flags: u32) -> *mut i32;
    fn devm_gpiochip_add_data(dev: *mut Device, gc: *mut GpioChip, data: *mut IdtGpioCtrl) -> i32;
}

#[repr(C)] struct GpioGenericChip { gc: GpioChip }
#[repr(C)] struct GpioChip { parent: *mut Device, ngpio: u32, irq: GpioIrqChip }
#[repr(C)] struct GpioIrqChip { domain: *mut IrqDomain, parents: *mut i32 }
#[repr(C)] struct GpioGenericChipConfig { dev: *mut Device, sz: u32, dat: *mut core::ffi::c_void, dirout: *mut core::ffi::c_void }
#[repr(C)] struct Device;
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct IrqDesc;
#[repr(C)] struct IrqChip;
#[repr(C)] struct IrqDomain;
#[repr(C)] struct IrqData { hwirq: u32 }

const IRQ_TYPE_SENSE_MASK: u32 = 0x0f;
const IRQ_TYPE_NONE: u32 = 0;
const IRQ_TYPE_EDGE_BOTH: u32 = 0x03;
const IRQ_TYPE_LEVEL_HIGH: u32 = 0x04;
const IRQ_TYPE_LEVEL_LOW: u32 = 0x08;

unsafe extern "C" fn idt_gpio_dispatch(desc: *mut IrqDesc) {
    let gc = irq_desc_get_handler_data(desc);
    let ctrl = gpiochip_get_data(gc);
    let host_chip = irq_desc_get_chip(desc);
    chained_irq_enter(host_chip, desc);
    let mut pending = readl((*ctrl).pic.byte_add(IDT_PIC_IRQ_PEND));
    pending &= !(*ctrl).mask_cache;
    let mut bit = 0u32;
    while bit < (*gc).ngpio {
        if pending & (1u32 << bit) != 0 {
            let virq = irq_find_mapping((*gc).irq.domain, bit);
            if virq != 0 { generic_handle_irq(virq); }
        }
        bit += 1;
    }
    chained_irq_exit(host_chip, desc);
}

unsafe extern "C" fn idt_gpio_irq_set_type(d: *mut IrqData, flow_type: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(d);
    let ctrl = gpiochip_get_data(gc);
    let sense = flow_type & IRQ_TYPE_SENSE_MASK;
    if sense == IRQ_TYPE_NONE || sense & IRQ_TYPE_EDGE_BOTH != 0 { return -22; }
    let mut ilevel = readl((*ctrl).gpio.byte_add(IDT_GPIO_ILEVEL));
    if sense & IRQ_TYPE_LEVEL_HIGH != 0 { ilevel |= 1u32 << (*d).hwirq; }
    else if sense & IRQ_TYPE_LEVEL_LOW != 0 { ilevel &= !(1u32 << (*d).hwirq); }
    writel(ilevel, (*ctrl).gpio.byte_add(IDT_GPIO_ILEVEL));
    0
}

unsafe extern "C" fn idt_gpio_ack(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d); let ctrl = gpiochip_get_data(gc);
    writel(!(1u32 << (*d).hwirq), (*ctrl).gpio.byte_add(IDT_GPIO_ISTAT));
}

unsafe extern "C" fn idt_gpio_mask(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d); let ctrl = gpiochip_get_data(gc);
    (*ctrl).mask_cache |= 1u32 << (*d).hwirq;
    writel((*ctrl).mask_cache, (*ctrl).pic.byte_add(IDT_PIC_IRQ_MASK));
    gpiochip_disable_irq(gc, irqd_to_hwirq(d));
}

unsafe extern "C" fn idt_gpio_unmask(d: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(d); let ctrl = gpiochip_get_data(gc);
    gpiochip_enable_irq(gc, irqd_to_hwirq(d));
    (*ctrl).mask_cache &= !(1u32 << (*d).hwirq);
    writel((*ctrl).mask_cache, (*ctrl).pic.byte_add(IDT_PIC_IRQ_MASK));
}

unsafe extern "C" fn idt_gpio_irq_init_hw(gc: *mut GpioChip) -> i32 {
    let ctrl = gpiochip_get_data(gc); (*ctrl).mask_cache = 0xffff_ffff;
    writel((*ctrl).mask_cache, (*ctrl).pic.byte_add(IDT_PIC_IRQ_MASK)); 0
}

unsafe extern "C" fn idt_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev as *mut Device;
    let ctrl = devm_kzalloc(dev, core::mem::size_of::<IdtGpioCtrl>(), 0) as *mut IdtGpioCtrl;
    if ctrl.is_null() { return -12; }
    (*ctrl).gpio = devm_platform_ioremap_resource_byname(pdev, b"gpio\0".as_ptr());
    if ptr_is_err((*ctrl).gpio) { return ptr_err((*ctrl).gpio); }
    (*ctrl).chip.gc.parent = dev;
    let mut config = GpioGenericChipConfig { dev, sz: 4, dat: (*ctrl).gpio.byte_add(IDT_GPIO_DATA), dirout: (*ctrl).gpio.byte_add(IDT_GPIO_DIR) };
    let ret = gpio_generic_chip_init(&mut (*ctrl).chip, &mut config); if ret != 0 { return ret; }
    let mut ngpios = 0; if device_property_read_u32(dev, b"ngpios\0".as_ptr(), &mut ngpios) == 0 { (*ctrl).chip.gc.ngpio = ngpios as u32; }
    if device_property_read_bool(dev, b"interrupt-controller\0".as_ptr()) {
        (*ctrl).pic = devm_platform_ioremap_resource_byname(pdev, b"pic\0".as_ptr());
        if ptr_is_err((*ctrl).pic) { return ptr_err((*ctrl).pic); }
        let parent_irq = platform_get_irq(pdev, 0); if parent_irq < 0 { return parent_irq; }
        (*ctrl).chip.gc.irq.parents = devm_kcalloc(dev, 1, core::mem::size_of::<i32>(), 0);
        if (*ctrl).chip.gc.irq.parents.is_null() { return -12; }
        *(*ctrl).chip.gc.irq.parents = parent_irq;
    }
    devm_gpiochip_add_data(dev, &mut (*ctrl).chip.gc, ctrl)
}

#[repr(C)] struct OfDeviceId { compatible: *const u8 }
static IDT_GPIO_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"idt,32434-gpio\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

#[repr(C)] struct PlatformDriver { probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>, name: *const u8, of_match_table: *const OfDeviceId }
static IDT_GPIO_DRIVER: PlatformDriver = PlatformDriver {
    probe: Some(idt_gpio_probe), name: b"idt3243x-gpio\0".as_ptr(),
    of_match_table: IDT_GPIO_OF_MATCH.as_ptr(),
};

// Corresponds to module_platform_driver(idt_gpio_driver).
// MODULE_DEVICE_TABLE(of, idt_gpio_of_match);
// MODULE_DESCRIPTION("IDT 79RC3243x GPIO/PIC Driver");
// MODULE_AUTHOR("Thomas Bogendoerfer <tsbogend@alpha.franken.de>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
