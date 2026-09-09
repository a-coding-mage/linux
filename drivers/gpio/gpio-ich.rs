// SPDX-License-Identifier: GPL-2.0+
/*
 * Intel ICH6-10, Series 5 and 6, Atom C2000 (Avoton/Rangeley) GPIO driver
 *
 * Copyright (C) 2010 Extreme Engineering Solutions.
 */

// Linux kernel dependencies are supplied by the surrounding repository.

const DRV_NAME: &str = "gpio_ich";

/*
 * GPIO register offsets in GPIO I/O space.
 * Each chunk of 32 GPIOs is manipulated via its own USE_SELx, IO_SELx, and
 * LVLx registers.  Logic in the read/write functions takes a register and
 * an absolute bit number and determines the proper register offset and bit
 * number in that register.  For example, to read the value of GPIO bit 50
 * the code would access offset ichx_regs[2(=GPIO_LVL)][1(=50/32)],
 * bit 18 (50%32).
 */
#[repr(usize)]
enum GPIO_REG {
    GPIO_USE_SEL = 0,
    GPIO_IO_SEL,
    GPIO_LVL,
    GPO_BLINK,
}

static ICHX_REGS: [[u8; 3]; 4] = [
    [0x00, 0x30, 0x40], // USE_SEL[1-3] offsets
    [0x04, 0x34, 0x44], // IO_SEL[1-3] offsets
    [0x0c, 0x38, 0x48], // LVL[1-3] offsets
    [0x18, 0x18, 0x18], // BLINK offset
];

static ICHX_REGLEN: [u8; 3] = [0x30, 0x10, 0x10];

static AVOTON_REGS: [[u8; 3]; 4] = [
    [0x00, 0x80, 0x00],
    [0x04, 0x84, 0x00],
    [0x08, 0x88, 0x00],
];

static AVOTON_REGLEN: [u8; 3] = [0x10, 0x10, 0x00];

#[inline]
unsafe fn ichx_write(val: u32, reg: u8, base_res: *mut resource) {
    outl(val, reg as u64 + (*base_res).start);
}

#[inline]
unsafe fn ichx_read(reg: u8, base_res: *mut resource) -> u32 {
    inl(reg as u64 + (*base_res).start)
}

#[repr(C)]
struct ichx_desc {
    ngpio: u32,
    regs: *const [u8; 3],
    reglen: *const u8,
    have_blink: bool,
    uses_gpe0: bool,
    use_sel_ignore: [u32; 3],
    request: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    use_outlvl_cache: bool,
}

#[repr(C)]
struct ichx_priv_t {
    lock: spinlock_t,
    dev: *mut device,
    chip: gpio_chip,
    gpio_base: *mut resource,
    pm_base: *mut resource,
    desc: *mut ichx_desc,
    orig_gpio_ctrl: u32,
    use_gpio: u8,
    outlvl_cache: [i32; 3],
}

static mut ICHX_PRIV: ichx_priv_t = unsafe { core::mem::zeroed() };
static mut MODPARAM_GPIOBASE: i32 = -1; // dynamic

unsafe fn ichx_write_bit(reg: usize, nr: u32, val: i32, verify: i32) -> i32 {
    let mut flags: ulong = 0;
    let reg_nr = (nr / 32) as usize;
    let bit = nr & 0x1f;
    spin_lock_irqsave(&mut ICHX_PRIV.lock, &mut flags);
    let mut data = if reg == GPIO_REG::GPIO_LVL as usize && (*ICHX_PRIV.desc).use_outlvl_cache {
        ICHX_PRIV.outlvl_cache[reg_nr] as u32
    } else {
        ichx_read((*(*ICHX_PRIV.desc).regs.add(reg))[reg_nr], ICHX_PRIV.gpio_base)
    };
    if val != 0 { data |= 1u32.wrapping_shl(bit); } else { data &= !(1u32.wrapping_shl(bit)); }
    ichx_write(data, (*(*ICHX_PRIV.desc).regs.add(reg))[reg_nr], ICHX_PRIV.gpio_base);
    if reg == GPIO_REG::GPIO_LVL as usize && (*ICHX_PRIV.desc).use_outlvl_cache { ICHX_PRIV.outlvl_cache[reg_nr] = data as i32; }
    let tmp = ichx_read((*(*ICHX_PRIV.desc).regs.add(reg))[reg_nr], ICHX_PRIV.gpio_base);
    spin_unlock_irqrestore(&mut ICHX_PRIV.lock, flags);
    if verify != 0 && data != tmp { -EPERM } else { 0 }
}

unsafe fn ichx_read_bit(reg: usize, nr: u32) -> i32 {
    let mut flags: ulong = 0;
    let reg_nr = (nr / 32) as usize;
    let bit = nr & 0x1f;
    spin_lock_irqsave(&mut ICHX_PRIV.lock, &mut flags);
    let mut data = ichx_read((*(*ICHX_PRIV.desc).regs.add(reg))[reg_nr], ICHX_PRIV.gpio_base);
    if reg == GPIO_REG::GPIO_LVL as usize && (*ICHX_PRIV.desc).use_outlvl_cache { data = ICHX_PRIV.outlvl_cache[reg_nr] as u32 | data; }
    spin_unlock_irqrestore(&mut ICHX_PRIV.lock, flags);
    if data & 1u32.wrapping_shl(bit) != 0 { 1 } else { 0 }
}

unsafe fn ichx_gpio_check_available(_gpio: *mut gpio_chip, nr: u32) -> bool { ICHX_PRIV.use_gpio & (1u8.wrapping_shl(nr / 32)) != 0 }
unsafe fn ichx_gpio_get_direction(_gpio: *mut gpio_chip, nr: u32) -> i32 { if ichx_read_bit(GPIO_REG::GPIO_IO_SEL as usize, nr) != 0 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT } }
unsafe fn ichx_gpio_direction_input(_gpio: *mut gpio_chip, nr: u32) -> i32 { ichx_write_bit(GPIO_REG::GPIO_IO_SEL as usize, nr, 1, 1) }
unsafe fn ichx_gpio_direction_output(_gpio: *mut gpio_chip, nr: u32, val: i32) -> i32 {
    if nr < 32 && (*ICHX_PRIV.desc).have_blink { ichx_write_bit(GPIO_REG::GPO_BLINK as usize, nr, 0, 0); }
    let ret = ichx_write_bit(GPIO_REG::GPIO_LVL as usize, nr, val, 0);
    if ret != 0 { return ret; }
    ichx_write_bit(GPIO_REG::GPIO_IO_SEL as usize, nr, 0, 1)
}
unsafe fn ichx_gpio_get(_chip: *mut gpio_chip, nr: u32) -> i32 { ichx_read_bit(GPIO_REG::GPIO_LVL as usize, nr) }

unsafe fn ich6_gpio_get(chip: *mut gpio_chip, nr: u32) -> i32 {
    if nr < 16 {
        if ICHX_PRIV.pm_base.is_null() { return -ENXIO; }
        let mut flags: ulong = 0;
        spin_lock_irqsave(&mut ICHX_PRIV.lock, &mut flags);
        ichx_write(1u32.wrapping_shl(16 + nr), 0, ICHX_PRIV.pm_base);
        let data = ichx_read(0, ICHX_PRIV.pm_base);
        spin_unlock_irqrestore(&mut ICHX_PRIV.lock, flags);
        if (data >> 16) & 1u32.wrapping_shl(nr) != 0 { 1 } else { 0 }
    } else { ichx_gpio_get(chip, nr) }
}

unsafe fn ichx_gpio_request(chip: *mut gpio_chip, mut nr: u32) -> i32 {
    if !ichx_gpio_check_available(chip, nr) { return -ENXIO; }
    let reg_nr = (nr / 32) as usize;
    if (*ICHX_PRIV.desc).use_sel_ignore[reg_nr] & 1u32.wrapping_shl(nr & 0x1f) != 0 { return 0; }
    if ichx_read_bit(GPIO_REG::GPIO_USE_SEL as usize, nr) != 0 { 0 } else { -ENODEV }
}

unsafe fn ich6_gpio_request(chip: *mut gpio_chip, mut nr: u32) -> i32 {
    if nr == 16 || nr == 17 { nr -= 16; }
    ichx_gpio_request(chip, nr)
}

unsafe fn ichx_gpio_set(_chip: *mut gpio_chip, nr: u32, val: i32) { ichx_write_bit(GPIO_REG::GPIO_LVL as usize, nr, val, 0); }

unsafe fn ichx_gpiolib_setup(chip: *mut gpio_chip) {
    (*chip).owner = THIS_MODULE;
    (*chip).label = DRV_NAME.as_ptr() as *const i8;
    (*chip).parent = ICHX_PRIV.dev;
    (*chip).request = (*ICHX_PRIV.desc).request.unwrap_or(ichx_gpio_request);
    (*chip).get = (*ICHX_PRIV.desc).get.unwrap_or(ichx_gpio_get);
    (*chip).set = Some(ichx_gpio_set);
    (*chip).get_direction = Some(ichx_gpio_get_direction);
    (*chip).direction_input = Some(ichx_gpio_direction_input);
    (*chip).direction_output = Some(ichx_gpio_direction_output);
    (*chip).base = MODPARAM_GPIOBASE;
    (*chip).ngpio = (*ICHX_PRIV.desc).ngpio;
    (*chip).can_sleep = false;
    (*chip).dbg_show = None;
}

static mut ICH6_DESC: ichx_desc = ichx_desc { ngpio: 50, regs: ICHX_REGS.as_ptr(), reglen: ICHX_REGLEN.as_ptr(), have_blink: true, uses_gpe0: true, use_sel_ignore: [0; 3], request: Some(ich6_gpio_request), get: Some(ich6_gpio_get), use_outlvl_cache: false };
static mut I3100_DESC: ichx_desc = ichx_desc { ngpio: 50, regs: ICHX_REGS.as_ptr(), reglen: ICHX_REGLEN.as_ptr(), have_blink: false, uses_gpe0: true, use_sel_ignore: [0x00130000, 0x00010000, 0], request: Some(ich6_gpio_request), get: Some(ich6_gpio_get), use_outlvl_cache: false };
static mut ICH7_DESC: ichx_desc = ichx_desc { ngpio: 50, regs: ICHX_REGS.as_ptr(), reglen: ICHX_REGLEN.as_ptr(), have_blink: true, uses_gpe0: false, use_sel_ignore: [0; 3], request: None, get: None, use_outlvl_cache: false };
static mut ICH9_DESC: ichx_desc = ichx_desc { ngpio: 61, regs: ICHX_REGS.as_ptr(), reglen: ICHX_REGLEN.as_ptr(), have_blink: true, uses_gpe0: false, use_sel_ignore: [0; 3], request: None, get: None, use_outlvl_cache: false };
static mut ICH10_CONS_DESC: ichx_desc = ichx_desc { ngpio: 61, regs: ICHX_REGS.as_ptr(), reglen: ICHX_REGLEN.as_ptr(), have_blink: true, uses_gpe0: false, use_sel_ignore: [0; 3], request: None, get: None, use_outlvl_cache: false };
static mut ICH10_CORP_DESC: ichx_desc = ichx_desc { ngpio: 72, regs: ICHX_REGS.as_ptr(), reglen: ICHX_REGLEN.as_ptr(), have_blink: true, uses_gpe0: false, use_sel_ignore: [0; 3], request: None, get: None, use_outlvl_cache: false };
static mut INTEL5_DESC: ichx_desc = ichx_desc { ngpio: 76, regs: ICHX_REGS.as_ptr(), reglen: ICHX_REGLEN.as_ptr(), have_blink: false, uses_gpe0: false, use_sel_ignore: [0; 3], request: None, get: None, use_outlvl_cache: false };
static mut AVOTON_DESC: ichx_desc = ichx_desc { ngpio: 60, regs: AVOTON_REGS.as_ptr(), reglen: AVOTON_REGLEN.as_ptr(), have_blink: false, uses_gpe0: false, use_sel_ignore: [0; 3], request: None, get: None, use_outlvl_cache: true };

unsafe fn ichx_gpio_request_regions(dev: *mut device, res_base: *mut resource, name: *const i8, use_gpio: u8) -> i32 {
    if res_base.is_null() || (*res_base).start == 0 || (*res_base).end == 0 { return -ENODEV; }
    for i in 0..3usize {
        if use_gpio & (1u8 << i) == 0 { continue; }
        if devm_request_region(dev, (*res_base).start + (*(*ICHX_PRIV.desc).regs)[i] as u64, *(*ICHX_PRIV.desc).reglen.add(i) as u64, name).is_null() { return -EBUSY; }
    }
    0
}

unsafe fn ichx_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let ich_info = dev_get_platdata(dev);
    if ich_info.is_null() { return -ENODEV; }
    (*ICHX_PRIV.desc) = match (*ich_info).gpio_version {
        ICH_I3100_GPIO => I3100_DESC,
        ICH_V5_GPIO => INTEL5_DESC,
        ICH_V6_GPIO => ICH6_DESC,
        ICH_V7_GPIO => ICH7_DESC,
        ICH_V9_GPIO => ICH9_DESC,
        ICH_V10CORP_GPIO => ICH10_CORP_DESC,
        ICH_V10CONS_GPIO => ICH10_CONS_DESC,
        AVOTON_GPIO => AVOTON_DESC,
        _ => return -ENODEV,
    };
    ICHX_PRIV.dev = dev;
    spin_lock_init(&mut ICHX_PRIV.lock);
    let res_base = platform_get_resource(pdev, IORESOURCE_IO, ICH_RES_GPIO);
    let err = ichx_gpio_request_regions(dev, res_base, (*pdev).name, (*ich_info).use_gpio);
    if err != 0 { return err; }
    ICHX_PRIV.gpio_base = res_base;
    ICHX_PRIV.use_gpio = (*ich_info).use_gpio;
    if (*ICHX_PRIV.desc).uses_gpe0 {
        let res_pm = platform_get_resource(pdev, IORESOURCE_IO, ICH_RES_GPE0);
        if !res_pm.is_null() && !devm_request_region(dev, (*res_pm).start, resource_size(res_pm), (*pdev).name).is_null() { ICHX_PRIV.pm_base = res_pm; }
    }
    ichx_gpiolib_setup(&mut ICHX_PRIV.chip);
    let err = devm_gpiochip_add_data(dev, &mut ICHX_PRIV.chip, core::ptr::null_mut());
    if err != 0 { return err; }
    0
}

#[repr(C)] struct platform_device { dev: device, name: *const i8 }
#[repr(C)] struct lpc_ich_info { gpio_version: i32, use_gpio: u8 }
extern "C" {
    fn dev_get_platdata(dev: *mut device) -> *mut lpc_ich_info;
    fn platform_get_resource(pdev: *mut platform_device, typ: u32, num: u32) -> *mut resource;
    fn devm_request_region(dev: *mut device, start: u64, len: u64, name: *const i8) -> *mut resource;
    fn resource_size(res: *mut resource) -> u64;
    fn spin_lock_init(lock: *mut spinlock_t);
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip, data: *mut core::ffi::c_void) -> i32;
}
const ENODEV: i32 = 19; const EBUSY: i32 = 16; const IORESOURCE_IO: u32 = 0x100;
const ICH_RES_GPIO: u32 = 0; const ICH_RES_GPE0: u32 = 1;
const ICH_I3100_GPIO: i32 = 1; const ICH_V5_GPIO: i32 = 2; const ICH_V6_GPIO: i32 = 3; const ICH_V7_GPIO: i32 = 4;
const ICH_V9_GPIO: i32 = 5; const ICH_V10CORP_GPIO: i32 = 6; const ICH_V10CONS_GPIO: i32 = 7; const AVOTON_GPIO: i32 = 8;

extern "C" {
    fn outl(value: u32, port: u64);
    fn inl(port: u64) -> u32;
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: ulong);
}

type ulong = u64;
type resource = crate::resource;
type device = crate::device;
type gpio_chip = crate::gpio_chip;
type spinlock_t = crate::spinlock_t;
extern "C" { static EPERM: i32; static ENXIO: i32; }
const GPIO_LINE_DIRECTION_IN: i32 = 1;
const GPIO_LINE_DIRECTION_OUT: i32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
