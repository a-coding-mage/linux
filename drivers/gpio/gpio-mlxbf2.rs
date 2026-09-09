// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright (C) 2020-2021 NVIDIA CORPORATION & AFFILIATES
 */

// Kernel dependencies supplied by the surrounding Rust kernel environment.

const MLXBF2_GPIO_MAX_PINS_PER_BLOCK: u32 = 32;

const YU_ARM_GPIO_LOCK_ADDR: usize = 0x2801088;
const YU_ARM_GPIO_LOCK_SIZE: usize = 0x8;
const YU_ARM_GPIO_LOCK_ACQUIRE: u32 = 0xd42f;
const YU_ARM_GPIO_LOCK_RELEASE: u32 = 0x0;

const YU_GPIO_DATAIN: usize = 0x04;
const YU_GPIO_MODE1: usize = 0x08;
const YU_GPIO_MODE0: usize = 0x0c;
const YU_GPIO_DATASET: usize = 0x14;
const YU_GPIO_DATACLEAR: usize = 0x18;
const YU_GPIO_CAUSE_RISE_EN: usize = 0x44;
const YU_GPIO_CAUSE_FALL_EN: usize = 0x48;
const YU_GPIO_MODE1_CLEAR: usize = 0x50;
const YU_GPIO_MODE0_SET: usize = 0x54;
const YU_GPIO_MODE0_CLEAR: usize = 0x58;
const YU_GPIO_CAUSE_OR_CAUSE_EVTEN0: usize = 0x80;
const YU_GPIO_CAUSE_OR_EVTEN0: usize = 0x94;
const YU_GPIO_CAUSE_OR_CLRCAUSE: usize = 0x98;

#[repr(C)]
struct Mlxbf2GpioContextSaveRegs {
    gpio_mode0: u32,
    gpio_mode1: u32,
}

#[repr(C)]
struct Mlxbf2GpioContext {
    chip: GpioGenericChip,
    gpio_io: *mut core::ffi::c_void,
    dev: *mut Device,
    csave_regs: *mut Mlxbf2GpioContextSaveRegs,
}

#[repr(C)]
struct Mlxbf2GpioParam {
    io: *mut core::ffi::c_void,
    res: *mut Resource,
    lock: *mut Mutex,
}

extern "C" {
    static mut yu_arm_gpio_lock_param: Mlxbf2GpioParam;
}

unsafe fn mlxbf2_gpio_get_lock_res(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev as *mut Device;
    let mut ret: i32 = 0;

    mutex_lock((*yu_arm_gpio_lock_param).lock);
    if !(*yu_arm_gpio_lock_param).io.is_null() {
        mutex_unlock((*yu_arm_gpio_lock_param).lock);
        return ret;
    }

    let res = (*yu_arm_gpio_lock_param).res;
    let size = resource_size(res);
    if devm_request_mem_region(dev, (*res).start, size, (*res).name).is_null() {
        ret = -EFAULT;
        mutex_unlock((*yu_arm_gpio_lock_param).lock);
        return ret;
    }

    (*yu_arm_gpio_lock_param).io = devm_ioremap(dev, (*res).start, size);
    if (*yu_arm_gpio_lock_param).io.is_null() {
        ret = -ENOMEM;
    }
    mutex_unlock((*yu_arm_gpio_lock_param).lock);
    ret
}

unsafe fn mlxbf2_gpio_lock_acquire(gs: *mut Mlxbf2GpioContext) -> i32 {
    mutex_lock((*yu_arm_gpio_lock_param).lock);
    gpio_generic_chip_lock(&mut (*gs).chip);
    let arm_gpio_lock_val = readl((*yu_arm_gpio_lock_param).io);
    if (arm_gpio_lock_val >> 31) != 0 {
        gpio_generic_chip_unlock(&mut (*gs).chip);
        mutex_unlock((*yu_arm_gpio_lock_param).lock);
        return -EINVAL;
    }
    writel(YU_ARM_GPIO_LOCK_ACQUIRE, (*yu_arm_gpio_lock_param).io);
    0
}

unsafe fn mlxbf2_gpio_lock_release(gs: *mut Mlxbf2GpioContext) {
    writel(YU_ARM_GPIO_LOCK_RELEASE, (*yu_arm_gpio_lock_param).io);
    gpio_generic_chip_unlock(&mut (*gs).chip);
    mutex_unlock((*yu_arm_gpio_lock_param).lock);
}

unsafe extern "C" fn mlxbf2_gpio_direction_input(chip: *mut GpioChip, offset: u32) -> i32 {
    let gs = gpiochip_get_data(chip) as *mut Mlxbf2GpioContext;
    let ret = mlxbf2_gpio_lock_acquire(gs);
    if ret < 0 { return ret; }
    writel(1u32.wrapping_shl(offset), (*gs).gpio_io.add(YU_GPIO_MODE0_CLEAR));
    writel(1u32.wrapping_shl(offset), (*gs).gpio_io.add(YU_GPIO_MODE1_CLEAR));
    mlxbf2_gpio_lock_release(gs);
    ret
}

unsafe extern "C" fn mlxbf2_gpio_direction_output(chip: *mut GpioChip, offset: u32, _value: i32) -> i32 {
    let gs = gpiochip_get_data(chip) as *mut Mlxbf2GpioContext;
    let ret = mlxbf2_gpio_lock_acquire(gs);
    if ret < 0 { return ret; }
    writel(1u32.wrapping_shl(offset), (*gs).gpio_io.add(YU_GPIO_MODE1_CLEAR));
    writel(1u32.wrapping_shl(offset), (*gs).gpio_io.add(YU_GPIO_MODE0_SET));
    mlxbf2_gpio_lock_release(gs);
    ret
}

unsafe extern "C" fn mlxbf2_gpio_irq_enable(irqd: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(irqd);
    let gs = gpiochip_get_data(gc) as *mut Mlxbf2GpioContext;
    let offset = irqd_to_hwirq(irqd);
    gpiochip_enable_irq(gc, offset);
    gpio_generic_lock_irqsave(&mut (*gs).chip);
    let mut val = readl((*gs).gpio_io.add(YU_GPIO_CAUSE_OR_CLRCAUSE));
    val |= 1u32.wrapping_shl(offset);
    writel(val, (*gs).gpio_io.add(YU_GPIO_CAUSE_OR_CLRCAUSE));
    val = readl((*gs).gpio_io.add(YU_GPIO_CAUSE_OR_EVTEN0));
    val |= 1u32.wrapping_shl(offset);
    writel(val, (*gs).gpio_io.add(YU_GPIO_CAUSE_OR_EVTEN0));
    gpio_generic_unlock_irqrestore(&mut (*gs).chip);
}

unsafe extern "C" fn mlxbf2_gpio_irq_disable(irqd: *mut IrqData) {
    let gc = irq_data_get_irq_chip_data(irqd);
    let gs = gpiochip_get_data(gc) as *mut Mlxbf2GpioContext;
    let offset = irqd_to_hwirq(irqd);
    gpio_generic_lock_irqsave(&mut (*gs).chip);
    let mut val = readl((*gs).gpio_io.add(YU_GPIO_CAUSE_OR_EVTEN0));
    val &= !(1u32.wrapping_shl(offset));
    writel(val, (*gs).gpio_io.add(YU_GPIO_CAUSE_OR_EVTEN0));
    gpio_generic_unlock_irqrestore(&mut (*gs).chip);
    gpiochip_disable_irq(gc, offset);
}

unsafe extern "C" fn mlxbf2_gpio_irq_handler(_irq: i32, ptr: *mut core::ffi::c_void) -> IrqReturn {
    let gs = ptr as *mut Mlxbf2GpioContext;
    let gc = &mut (*gs).chip.gc;
    let pending = readl((*gs).gpio_io.add(YU_GPIO_CAUSE_OR_CAUSE_EVTEN0));
    writel(pending, (*gs).gpio_io.add(YU_GPIO_CAUSE_OR_CLRCAUSE));
    for level in 0..gc.ngpio {
        if (pending & (1u32.wrapping_shl(level))) != 0 {
            generic_handle_domain_irq_safe((*gc).irq.domain, level);
        }
    }
    irq_retval(pending)
}

unsafe extern "C" fn mlxbf2_gpio_irq_set_type(irqd: *mut IrqData, irq_type: u32) -> i32 {
    let gc = irq_data_get_irq_chip_data(irqd);
    let gs = gpiochip_get_data(gc) as *mut Mlxbf2GpioContext;
    let offset = irqd_to_hwirq(irqd);
    let (fall, rise) = match irq_type & IRQ_TYPE_SENSE_MASK {
        IRQ_TYPE_EDGE_BOTH => (true, true),
        IRQ_TYPE_EDGE_RISING => (false, true),
        IRQ_TYPE_EDGE_FALLING => (true, false),
        _ => return -EINVAL,
    };
    gpio_generic_lock_irqsave(&mut (*gs).chip);
    if fall {
        let mut val = readl((*gs).gpio_io.add(YU_GPIO_CAUSE_FALL_EN));
        val |= 1u32.wrapping_shl(offset);
        writel(val, (*gs).gpio_io.add(YU_GPIO_CAUSE_FALL_EN));
    }
    if rise {
        let mut val = readl((*gs).gpio_io.add(YU_GPIO_CAUSE_RISE_EN));
        val |= 1u32.wrapping_shl(offset);
        writel(val, (*gs).gpio_io.add(YU_GPIO_CAUSE_RISE_EN));
    }
    gpio_generic_unlock_irqrestore(&mut (*gs).chip);
    0
}

unsafe extern "C" fn mlxbf2_gpio_irq_print_chip(irqd: *mut IrqData, p: *mut SeqFile) {
    let gc = irq_data_get_irq_chip_data(irqd);
    let gs = gpiochip_get_data(gc) as *mut Mlxbf2GpioContext;
    seq_puts(p, dev_name((*gs).dev));
}

unsafe extern "C" fn mlxbf2_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev as *mut Device;
    let gs = devm_kzalloc(dev, core::mem::size_of::<Mlxbf2GpioContext>(), GFP_KERNEL) as *mut Mlxbf2GpioContext;
    if gs.is_null() { return -ENOMEM; }
    (*gs).dev = dev;
    (*gs).gpio_io = devm_platform_ioremap_resource(pdev, 0);
    if is_err((*gs).gpio_io) { return ptr_err((*gs).gpio_io); }
    let ret = mlxbf2_gpio_get_lock_res(pdev);
    if ret != 0 { return dev_err_probe(dev, ret, "Failed to get yu_arm_gpio_lock resource\n"); }
    let mut npins = 0u32;
    if device_property_read_u32(dev, "npins", &mut npins) != 0 { npins = MLXBF2_GPIO_MAX_PINS_PER_BLOCK; }
    let gc = &mut (*gs).chip.gc;
    let config = GpioGenericChipConfig { dev, sz: 4, dat: (*gs).gpio_io.add(YU_GPIO_DATAIN), set: (*gs).gpio_io.add(YU_GPIO_DATASET), clr: (*gs).gpio_io.add(YU_GPIO_DATACLEAR) };
    let ret = gpio_generic_chip_init(&mut (*gs).chip, &config);
    if ret != 0 { return dev_err_probe(dev, ret, "failed to initialize the generic GPIO chip\n"); }
    (*gc).direction_input = Some(mlxbf2_gpio_direction_input);
    (*gc).direction_output = Some(mlxbf2_gpio_direction_output);
    (*gc).ngpio = npins;
    (*gc).owner = THIS_MODULE;
    let irq = platform_get_irq_optional(pdev, 0);
    if irq >= 0 {
        let girq = &mut (*gc).irq;
        gpio_irq_chip_set_chip(girq, &mlxbf2_gpio_irq_chip);
        (*girq).handler = handle_simple_irq;
        (*girq).default_type = IRQ_TYPE_NONE;
        (*girq).num_parents = 0;
        (*girq).parents = core::ptr::null_mut();
        (*girq).parent_handler = None;
        let ret = devm_request_irq(dev, irq, Some(mlxbf2_gpio_irq_handler), IRQF_SHARED, dev_name(dev), gs as *mut _);
        if ret != 0 { return dev_err_probe(dev, ret, "failed to request IRQ"); }
    }
    platform_set_drvdata(pdev, gs as *mut _);
    let ret = devm_gpiochip_add_data(dev, gc, gs as *mut _);
    if ret != 0 { return dev_err_probe(dev, ret, "Failed adding memory mapped gpiochip\n"); }
    0
}

unsafe extern "C" fn mlxbf2_gpio_suspend(dev: *mut Device) -> i32 {
    let gs = dev_get_drvdata(dev) as *mut Mlxbf2GpioContext;
    (*(*gs).csave_regs).gpio_mode0 = readl((*gs).gpio_io.add(YU_GPIO_MODE0));
    (*(*gs).csave_regs).gpio_mode1 = readl((*gs).gpio_io.add(YU_GPIO_MODE1));
    0
}

unsafe extern "C" fn mlxbf2_gpio_resume(dev: *mut Device) -> i32 {
    let gs = dev_get_drvdata(dev) as *mut Mlxbf2GpioContext;
    writel((*(*gs).csave_regs).gpio_mode0, (*gs).gpio_io.add(YU_GPIO_MODE0));
    writel((*(*gs).csave_regs).gpio_mode1, (*gs).gpio_io.add(YU_GPIO_MODE1));
    0
}

static MLXBF2_GPIO_ACPI_MATCH: &[AcpiDeviceId] = &[AcpiDeviceId { id: "MLNXBF22", driver_data: 0 }, AcpiDeviceId::EMPTY];
static MLXBF2_GPIO_DRIVER: PlatformDriver = PlatformDriver::new("mlxbf2_gpio", mlxbf2_gpio_probe, MLXBF2_GPIO_ACPI_MATCH, mlxbf2_gpio_suspend, mlxbf2_gpio_resume);

// module_platform_driver(mlxbf2_gpio_driver)
// MODULE_DEVICE_TABLE(acpi, mlxbf2_gpio_acpi_match)
// MODULE_DESCRIPTION("Mellanox BlueField-2 GPIO Driver")
// MODULE_AUTHOR("Asmaa Mnebhi <asmaa@nvidia.com>")
// MODULE_LICENSE("GPL v2")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
