// SPDX-License-Identifier: GPL-2.0-only

// Linux kernel dependencies supplied by the surrounding build.

const AIROHA_GPIO_MAX: u32 = 32;

#[repr(C)]
pub struct gpio_generic_chip {
    pub gc: gpio_chip,
}

#[repr(C)]
pub struct gpio_chip {
    pub ngpio: u32,
    pub owner: *mut core::ffi::c_void,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
}

#[repr(C)]
pub struct gpio_generic_chip_config {
    pub dev: *mut device,
    pub sz: u32,
    pub dat: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device { pub dev: device }

#[repr(C)]
pub struct airoha_gpio_ctrl {
    pub gen_gc: gpio_generic_chip,
    pub data: *mut core::ffi::c_void,
    pub dir: [*mut core::ffi::c_void; 2],
    pub output: *mut core::ffi::c_void,
}

extern "C" {
    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut airoha_gpio_ctrl;
    fn ioread32(addr: *mut core::ffi::c_void) -> u32;
    fn iowrite32(value: u32, addr: *mut core::ffi::c_void);
    fn gpio_generic_chip_set(gc: *mut gpio_generic_chip, gpio: u32, val: i32);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn gpio_generic_chip_init(gc: *mut gpio_generic_chip, config: *mut gpio_generic_chip_config) -> i32;
    fn dev_err_probe(dev: *mut device, err: i32, msg: *const u8) -> i32;
    fn devm_gpiochip_add_data(dev: *mut device, gc: *mut gpio_chip, data: *mut airoha_gpio_ctrl) -> i32;
}

const GFP_KERNEL: u32 = 0;
const GPIO_LINE_DIRECTION_IN: i32 = 1;
const GPIO_LINE_DIRECTION_OUT: i32 = 0;

unsafe fn airoha_dir_set(gc: *mut gpio_chip, gpio: u32, val: i32, out: i32) -> i32 {
    let ctrl = gpiochip_get_data(gc);
    let mut dir = ioread32((*ctrl).dir[(gpio / 16) as usize]);
    let mut output = ioread32((*ctrl).output);
    let mask = 1u32.wrapping_shl((gpio % 16) * 2);

    if out != 0 {
        dir |= mask;
        output |= 1u32.wrapping_shl(gpio);
    } else {
        dir &= !mask;
        output &= !1u32.wrapping_shl(gpio);
    }

    iowrite32(dir, (*ctrl).dir[(gpio / 16) as usize]);

    if out != 0 {
        gpio_generic_chip_set(&mut (*ctrl).gen_gc, gpio, val);
    }

    iowrite32(output, (*ctrl).output);
    0
}

unsafe extern "C" fn airoha_dir_out(gc: *mut gpio_chip, gpio: u32, val: i32) -> i32 {
    airoha_dir_set(gc, gpio, val, 1)
}

unsafe extern "C" fn airoha_dir_in(gc: *mut gpio_chip, gpio: u32) -> i32 {
    airoha_dir_set(gc, gpio, 0, 0)
}

unsafe extern "C" fn airoha_get_dir(gc: *mut gpio_chip, gpio: u32) -> i32 {
    let ctrl = gpiochip_get_data(gc);
    let dir = ioread32((*ctrl).dir[(gpio / 16) as usize]);
    let mask = 1u32.wrapping_shl((gpio % 16) * 2);
    if dir & mask != 0 { GPIO_LINE_DIRECTION_OUT } else { GPIO_LINE_DIRECTION_IN }
}

unsafe extern "C" fn airoha_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut config = core::mem::zeroed::<gpio_generic_chip_config>();
    let dev = &mut (*pdev).dev as *mut device;
    let ctrl = devm_kzalloc(dev, core::mem::size_of::<airoha_gpio_ctrl>(), GFP_KERNEL)
        as *mut airoha_gpio_ctrl;
    if ctrl.is_null() { return -12; }

    (*ctrl).data = devm_platform_ioremap_resource(pdev, 0);
    (*ctrl).dir[0] = devm_platform_ioremap_resource(pdev, 1);
    (*ctrl).dir[1] = devm_platform_ioremap_resource(pdev, 2);
    (*ctrl).output = devm_platform_ioremap_resource(pdev, 3);

    config.dev = dev;
    config.sz = 4;
    config.dat = (*ctrl).data;
    let err = gpio_generic_chip_init(&mut (*ctrl).gen_gc, &mut config);
    if err != 0 { return dev_err_probe(dev, err, b"unable to init generic GPIO\0".as_ptr()); }

    (*ctrl).gen_gc.gc.ngpio = AIROHA_GPIO_MAX;
    (*ctrl).gen_gc.gc.direction_output = Some(airoha_dir_out);
    (*ctrl).gen_gc.gc.direction_input = Some(airoha_dir_in);
    (*ctrl).gen_gc.gc.get_direction = Some(airoha_get_dir);
    devm_gpiochip_add_data(dev, &mut (*ctrl).gen_gc.gc, ctrl)
}

// Device matching, platform-driver registration, and module metadata are provided by the kernel build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
