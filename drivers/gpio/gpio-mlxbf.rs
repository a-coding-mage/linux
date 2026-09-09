// SPDX-License-Identifier: GPL-2.0

// Linux kernel dependencies are supplied by the surrounding translation.

pub const MLXBF_GPIO_NR: usize = 54;

pub const MLXBF_GPIO_PAD_CONTROL_FIRST_WORD: usize = 0x0700;
pub const MLXBF_GPIO_PAD_CONTROL_1_FIRST_WORD: usize = 0x0708;
pub const MLXBF_GPIO_PAD_CONTROL_2_FIRST_WORD: usize = 0x0710;
pub const MLXBF_GPIO_PAD_CONTROL_3_FIRST_WORD: usize = 0x0718;

pub const MLXBF_GPIO_PIN_DIR_I: usize = 0x1040;
pub const MLXBF_GPIO_PIN_DIR_O: usize = 0x1048;
pub const MLXBF_GPIO_PIN_STATE: usize = 0x1000;
pub const MLXBF_GPIO_SCRATCHPAD: usize = 0x20;

#[repr(C)]
pub struct mlxbf_gpio_context_save_regs {
    pub scratchpad: u64,
    pub pad_control: [u64; MLXBF_GPIO_NR],
    pub pin_dir_i: u64,
    pub pin_dir_o: u64,
}

#[repr(C)]
pub struct mlxbf_gpio_state {
    pub chip: gpio_generic_chip,
    pub base: *mut core::ffi::c_void,
    pub csave_regs: mlxbf_gpio_context_save_regs,
}

extern "C" {
    pub fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    pub fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
    ) -> *mut core::ffi::c_void;
    pub fn gpio_generic_chip_init(
        chip: *mut gpio_generic_chip,
        config: *mut gpio_generic_chip_config,
    ) -> i32;
    pub fn devm_gpiochip_add_data(
        dev: *mut device,
        gc: *mut gpio_chip,
        data: *mut core::ffi::c_void,
    ) -> i32;
    pub fn platform_set_drvdata(pdev: *mut platform_device, data: *mut core::ffi::c_void);
    pub fn platform_get_drvdata(pdev: *mut platform_device) -> *mut core::ffi::c_void;
    pub fn readq(addr: *mut core::ffi::c_void) -> u64;
    pub fn writeq(value: u64, addr: *mut core::ffi::c_void);
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct gpio_chip {
    pub owner: *mut core::ffi::c_void,
    pub ngpio: u32,
}
#[repr(C)]
pub struct gpio_generic_chip {
    pub gc: gpio_chip,
}
#[repr(C)]
pub struct gpio_generic_chip_config {
    pub dev: *mut device,
    pub sz: u32,
    pub dat: *mut core::ffi::c_void,
    pub dirout: *mut core::ffi::c_void,
    pub dirin: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct pm_message_t;

unsafe fn mlxbf_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut config: gpio_generic_chip_config;
    let gs: *mut mlxbf_gpio_state;
    let dev: *mut device = unsafe { &mut (*pdev).dev };
    let gc: *mut gpio_chip;
    let ret: i32;

    gs = unsafe { devm_kzalloc(dev, core::mem::size_of::<mlxbf_gpio_state>(), 0) }
        as *mut mlxbf_gpio_state;
    if gs.is_null() {
        return -12;
    }

    unsafe {
        (*gs).base = devm_platform_ioremap_resource(pdev, 0);
    }
    if unsafe { (*gs).base.is_null() } {
        return -1;
    }

    gc = unsafe { &mut (*gs).chip.gc };
    config = gpio_generic_chip_config {
        dev,
        sz: 8,
        dat: unsafe { (*gs).base.add(MLXBF_GPIO_PIN_STATE) },
        dirout: unsafe { (*gs).base.add(MLXBF_GPIO_PIN_DIR_O) },
        dirin: unsafe { (*gs).base.add(MLXBF_GPIO_PIN_DIR_I) },
    };

    ret = unsafe { gpio_generic_chip_init(&mut (*gs).chip, &mut config) };
    if ret != 0 {
        return -19;
    }

    unsafe {
        (*gc).owner = core::ptr::null_mut();
        (*gc).ngpio = MLXBF_GPIO_NR as u32;
    }

    ret = unsafe { devm_gpiochip_add_data(dev, gc, gs as *mut core::ffi::c_void) };
    if ret != 0 {
        return ret;
    }

    unsafe { platform_set_drvdata(pdev, gs as *mut core::ffi::c_void) };
    0
}

unsafe fn mlxbf_gpio_suspend(pdev: *mut platform_device, _state: *mut pm_message_t) -> i32 {
    let gs = unsafe { platform_get_drvdata(pdev) as *mut mlxbf_gpio_state };

    unsafe {
        (*gs).csave_regs.scratchpad = readq((*gs).base.add(MLXBF_GPIO_SCRATCHPAD));
        (*gs).csave_regs.pad_control[0] = readq((*gs).base.add(MLXBF_GPIO_PAD_CONTROL_FIRST_WORD));
        (*gs).csave_regs.pad_control[1] = readq((*gs).base.add(MLXBF_GPIO_PAD_CONTROL_1_FIRST_WORD));
        (*gs).csave_regs.pad_control[2] = readq((*gs).base.add(MLXBF_GPIO_PAD_CONTROL_2_FIRST_WORD));
        (*gs).csave_regs.pad_control[3] = readq((*gs).base.add(MLXBF_GPIO_PAD_CONTROL_3_FIRST_WORD));
        (*gs).csave_regs.pin_dir_i = readq((*gs).base.add(MLXBF_GPIO_PIN_DIR_I));
        (*gs).csave_regs.pin_dir_o = readq((*gs).base.add(MLXBF_GPIO_PIN_DIR_O));
    }
    0
}

unsafe fn mlxbf_gpio_resume(pdev: *mut platform_device) -> i32 {
    let gs = unsafe { platform_get_drvdata(pdev) as *mut mlxbf_gpio_state };
    unsafe {
        writeq((*gs).csave_regs.scratchpad, (*gs).base.add(MLXBF_GPIO_SCRATCHPAD));
        writeq((*gs).csave_regs.pad_control[0], (*gs).base.add(MLXBF_GPIO_PAD_CONTROL_FIRST_WORD));
        writeq((*gs).csave_regs.pad_control[1], (*gs).base.add(MLXBF_GPIO_PAD_CONTROL_1_FIRST_WORD));
        writeq((*gs).csave_regs.pad_control[2], (*gs).base.add(MLXBF_GPIO_PAD_CONTROL_2_FIRST_WORD));
        writeq((*gs).csave_regs.pad_control[3], (*gs).base.add(MLXBF_GPIO_PAD_CONTROL_3_FIRST_WORD));
        writeq((*gs).csave_regs.pin_dir_i, (*gs).base.add(MLXBF_GPIO_PIN_DIR_I));
        writeq((*gs).csave_regs.pin_dir_o, (*gs).base.add(MLXBF_GPIO_PIN_DIR_O));
    }
    0
}

#[repr(C)]
pub struct acpi_device_id {
    pub id: [u8; 9],
    pub driver_data: usize,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
    pub acpi_match_table: *const acpi_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub suspend: Option<unsafe extern "C" fn(*mut platform_device, *mut pm_message_t) -> i32>,
    pub resume: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

#[link_section = ".rodata"]
static MLXBF_GPIO_ACPI_MATCH: [acpi_device_id; 2] = [
    acpi_device_id { id: *b"MLNXBF02\0", driver_data: 0 },
    acpi_device_id { id: [0; 9], driver_data: 0 },
];

static mut mlxbf_gpio_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"mlxbf_gpio\0" as *const u8 as *const core::ffi::c_char,
        acpi_match_table: MLXBF_GPIO_ACPI_MATCH.as_ptr(),
    },
    probe: Some(mlxbf_gpio_probe),
    suspend: Some(mlxbf_gpio_suspend),
    resume: Some(mlxbf_gpio_resume),
};

// Equivalent to module_platform_driver(mlxbf_gpio_driver).
// MODULE_DESCRIPTION("Mellanox BlueField GPIO Driver");
// MODULE_AUTHOR("Mellanox Technologies");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
