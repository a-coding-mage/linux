// SPDX-License-Identifier: GPL-2.0-only
/*
 * linux/drivers/char/ds1620.c: Dallas Semiconductors DS1620
 *   thermometer driver (as used in the Rebel.com NetWinder)
 */
// Linux and machine-specific includes are supplied by the surrounding kernel bindings.

const THERM_START_CONVERT: i32 = 0xee;
const THERM_RESET: i32 = 0xaf;
const THERM_READ_CONFIG: i32 = 0xac;
const THERM_READ_TEMP: i32 = 0xaa;
const THERM_READ_TL: i32 = 0xa2;
const THERM_READ_TH: i32 = 0xa1;
const THERM_WRITE_CONFIG: i32 = 0x0c;
const THERM_WRITE_TL: i32 = 0x02;
const THERM_WRITE_TH: i32 = 0x01;

const CFG_CPU: i32 = 2;
const CFG_1SHOT: i32 = 1;

static mut ds1620_mutex: Mutex = Mutex::new();
static fan_state: [&str; 3] = ["off", "on", "on (hardwired)"];

/* Start of NetWinder specifics. */
extern "C" {
    static mut system_rev: u32;
    static mut nw_gpio_lock: raw_spinlock_t;
    fn nw_gpio_modify_op(mask: u32, value: u32);
    fn nw_gpio_read() -> u32;
    fn nw_gpio_modify_io(mask: u32, value: u32);
    fn nw_cpld_modify(mask: u32, value: u32);
    fn udelay(usecs: u32);
    fn msleep(msecs: u32);
}

#[inline]
unsafe fn netwinder_ds1620_set_clk(clk: i32) {
    nw_gpio_modify_op(GPIO_DSCLK, if clk != 0 { GPIO_DSCLK } else { 0 });
}

#[inline]
unsafe fn netwinder_ds1620_set_data(dat: i32) {
    nw_gpio_modify_op(GPIO_DATA, if dat != 0 { GPIO_DATA } else { 0 });
}

#[inline]
unsafe fn netwinder_ds1620_get_data() -> i32 {
    (nw_gpio_read() & GPIO_DATA) as i32
}

#[inline]
unsafe fn netwinder_ds1620_set_data_dir(dir: i32) {
    nw_gpio_modify_io(GPIO_DATA, if dir != 0 { GPIO_DATA } else { 0 });
}

#[inline]
unsafe fn netwinder_ds1620_reset() {
    nw_cpld_modify(CPLD_DS_ENABLE, 0);
    nw_cpld_modify(CPLD_DS_ENABLE, CPLD_DS_ENABLE);
}

#[inline]
unsafe fn netwinder_lock(flags: *mut usize) {
    raw_spin_lock_irqsave(&mut nw_gpio_lock, flags);
}

#[inline]
unsafe fn netwinder_unlock(flags: *mut usize) {
    raw_spin_unlock_irqrestore(&mut nw_gpio_lock, *flags);
}

#[inline]
unsafe fn netwinder_set_fan(i: i32) {
    let mut flags: usize = 0;
    raw_spin_lock_irqsave(&mut nw_gpio_lock, &mut flags);
    nw_gpio_modify_op(GPIO_FAN, if i != 0 { GPIO_FAN } else { 0 });
    raw_spin_unlock_irqrestore(&mut nw_gpio_lock, flags);
}

#[inline]
unsafe fn netwinder_get_fan() -> i32 {
    if (system_rev & 0xf000) == 0x4000 { return FAN_ALWAYS_ON; }
    if (nw_gpio_read() & GPIO_FAN) != 0 { FAN_ON } else { FAN_OFF }
}

unsafe fn ds1620_send_bits(nr: i32, mut value: i32) {
    for _ in 0..nr {
        netwinder_ds1620_set_data(value & 1);
        netwinder_ds1620_set_clk(0); udelay(1);
        netwinder_ds1620_set_clk(1); udelay(1);
        value >>= 1;
    }
}

unsafe fn ds1620_recv_bits(nr: i32) -> u32 {
    let mut value: u32 = 0;
    let mut mask: u32 = 1;
    netwinder_ds1620_set_data(0);
    for _ in 0..nr {
        netwinder_ds1620_set_clk(0); udelay(1);
        if netwinder_ds1620_get_data() != 0 { value |= mask; }
        mask <<= 1;
        netwinder_ds1620_set_clk(1); udelay(1);
    }
    value
}

unsafe fn ds1620_out(cmd: i32, bits: i32, value: i32) {
    let mut flags: usize = 0;
    netwinder_lock(&mut flags);
    netwinder_ds1620_set_clk(1); netwinder_ds1620_set_data_dir(0); netwinder_ds1620_reset();
    udelay(1); ds1620_send_bits(8, cmd);
    if bits != 0 { ds1620_send_bits(bits, value); }
    udelay(1); netwinder_ds1620_reset(); netwinder_unlock(&mut flags); msleep(20);
}

unsafe fn ds1620_in(cmd: i32, bits: i32) -> u32 {
    let mut flags: usize = 0;
    netwinder_lock(&mut flags);
    netwinder_ds1620_set_clk(1); netwinder_ds1620_set_data_dir(0); netwinder_ds1620_reset();
    udelay(1); ds1620_send_bits(8, cmd); netwinder_ds1620_set_data_dir(1);
    let value = ds1620_recv_bits(bits);
    netwinder_ds1620_reset(); netwinder_unlock(&mut flags); value
}

fn cvt_9_to_int(mut val: u32) -> i32 {
    if val & 0x100 != 0 { val |= 0xfffffe00; }
    val as i32
}

unsafe fn ds1620_write_state(therm: *mut therm) {
    ds1620_out(THERM_WRITE_CONFIG, 8, CFG_CPU);
    ds1620_out(THERM_WRITE_TL, 9, (*therm).lo);
    ds1620_out(THERM_WRITE_TH, 9, (*therm).hi);
    ds1620_out(THERM_START_CONVERT, 0, 0);
}

unsafe fn ds1620_read_state(therm: *mut therm) {
    (*therm).lo = cvt_9_to_int(ds1620_in(THERM_READ_TL, 9));
    (*therm).hi = cvt_9_to_int(ds1620_in(THERM_READ_TH, 9));
}

unsafe fn ds1620_open(inode: *mut inode, file: *mut file) -> i32 { stream_open(inode, file) }

unsafe fn ds1620_read(_file: *mut file, buf: *mut u8, _count: usize, _ptr: *mut loff_t) -> isize {
    let cur_temp = cvt_9_to_int(ds1620_in(THERM_READ_TEMP, 9)) >> 1;
    let cur_temp_deg_f = ((cur_temp * 9) / 5 + 32) as i8;
    if copy_to_user(buf, &cur_temp_deg_f as *const i8 as *const u8, 1) != 0 { return -EFAULT as isize; }
    1
}

unsafe fn ds1620_ioctl(_file: *mut file, cmd: u32, arg: usize) -> i32 {
    let mut therm = therm { lo: 0, hi: 0 };
    let p = arg as *mut i32;
    match cmd {
        CMD_SET_THERMOSTATE | CMD_SET_THERMOSTATE2 => {
            if !capable(CAP_SYS_ADMIN) { return -EPERM; }
            if cmd == CMD_SET_THERMOSTATE {
                if get_user(&mut therm.hi, p) != 0 { return -EFAULT; }
                therm.lo = therm.hi - 3;
            } else if copy_from_user(&mut therm as *mut therm as *mut u8, arg as *const u8, core::mem::size_of::<therm>()) != 0 { return -EFAULT; }
            therm.lo <<= 1; therm.hi <<= 1; ds1620_write_state(&mut therm);
        }
        CMD_GET_THERMOSTATE | CMD_GET_THERMOSTATE2 => {
            ds1620_read_state(&mut therm); therm.lo >>= 1; therm.hi >>= 1;
            if cmd == CMD_GET_THERMOSTATE { if put_user(therm.hi, p) != 0 { return -EFAULT; } }
            else if copy_to_user(arg as *mut u8, &therm as *const therm as *const u8, core::mem::size_of::<therm>()) != 0 { return -EFAULT; }
        }
        CMD_GET_TEMPERATURE | CMD_GET_TEMPERATURE2 => {
            let mut i = cvt_9_to_int(ds1620_in(THERM_READ_TEMP, 9));
            if cmd == CMD_GET_TEMPERATURE { i >>= 1; }
            return if put_user(i, p) != 0 { -EFAULT } else { 0 };
        }
        CMD_GET_STATUS => { let i = (ds1620_in(THERM_READ_CONFIG, 8) & 0xe3) as i32; return if put_user(i, p) != 0 { -EFAULT } else { 0 }; }
        CMD_GET_FAN => { return if put_user(netwinder_get_fan(), p) != 0 { -EFAULT } else { 0 }; }
        CMD_SET_FAN => {
            if !capable(CAP_SYS_ADMIN) { return -EPERM; }
            let mut i = 0; if get_user(&mut i, p) != 0 { return -EFAULT; } netwinder_set_fan(i);
        }
        _ => return -ENOIOCTLCMD,
    }
    0
}

extern "C" { fn mutex_lock(m: *mut Mutex); fn mutex_unlock(m: *mut Mutex); }

unsafe fn ds1620_unlocked_ioctl(file: *mut file, cmd: u32, arg: usize) -> isize {
    mutex_lock(&mut ds1620_mutex); let ret = ds1620_ioctl(file, cmd, arg); mutex_unlock(&mut ds1620_mutex); ret as isize
}

unsafe fn ds1620_init() -> i32 {
    if !machine_is_netwinder() { return -ENODEV; }
    ds1620_out(THERM_RESET, 0, 0); ds1620_out(THERM_WRITE_CONFIG, 8, CFG_CPU); ds1620_out(THERM_START_CONVERT, 0, 0);
    let mut th = therm { lo: 0, hi: 0 }; let mut th_start = therm { lo: 0, hi: 1 };
    ds1620_read_state(&mut th); ds1620_write_state(&mut th_start); msleep(2000); ds1620_write_state(&mut th);
    let ret = misc_register(&mut ds1620_miscdev); if ret < 0 { return ret; }
    ds1620_read_state(&mut th); let ret = cvt_9_to_int(ds1620_in(THERM_READ_TEMP, 9));
    printk(ret); 0
}

unsafe fn ds1620_exit() { misc_deregister(&mut ds1620_miscdev); }

// C module_init/module_exit and MODULE_* metadata are represented by the surrounding kernel bindings.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
