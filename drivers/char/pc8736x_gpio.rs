// SPDX-License-Identifier: GPL-2.0-only
/* linux/drivers/char/pc8736x_gpio.c

   National Semiconductor PC8736x GPIO driver.  Allows a user space
   process to play with the GPIO pins.

   Copyright (c) 2005,2006 Jim Cromie <jim.cromie@gmail.com>

   adapted from linux/drivers/char/scx200_gpio.c
   Copyright (c) 2001,2002 Christer Weinigel <wingel@nano-system.com>,
*/

// Kernel dependencies supplied by the surrounding translation unit/build.

const DEVNAME: *const u8 = b"pc8736x_gpio\0".as_ptr();

// MODULE_AUTHOR("Jim Cromie <jim.cromie@gmail.com>");
// MODULE_DESCRIPTION("NatSemi/Winbond PC-8736x GPIO Pin Driver");
// MODULE_LICENSE("GPL");

static mut major: i32 = 0; // default to dynamic major
// module_param(major, int, 0);
// MODULE_PARM_DESC(major, "Major device number");

static mut pc8736x_gpio_config_lock: Mutex = DEFINE_MUTEX!();
static mut pc8736x_gpio_base: u32 = 0;
static mut pc8736x_gpio_shadow: [u8; 4] = [0; 4];

const SIO_BASE1: i32 = 0x2E; // 1st command-reg to check
const SIO_BASE2: i32 = 0x4E; // alt command-reg to check

const SIO_SID: i32 = 0x20; // SuperI/O ID Register
const SIO_SID_PC87365: i32 = 0xe5; // Expected value in ID Register for PC87365
const SIO_SID_PC87366: i32 = 0xe9; // Expected value in ID Register for PC87366

const SIO_CF1: i32 = 0x21; // chip config, bit0 is chip enable

const PC8736X_GPIO_RANGE: u32 = 16; // ioaddr range
const PC8736X_GPIO_CT: u32 = 32; // minors matching 4 8 bit ports

const SIO_UNIT_SEL: i32 = 0x7; // unit select reg
const SIO_UNIT_ACT: i32 = 0x30; // unit enable
const SIO_GPIO_UNIT: u32 = 0x7; // unit number of GPIO
const SIO_VLM_UNIT: u32 = 0x0D;
const SIO_TMS_UNIT: u32 = 0x0E;

// config-space addrs to read/write each unit's runtime addr
const SIO_BASE_HADDR: i32 = 0x60;
const SIO_BASE_LADDR: i32 = 0x61;

// GPIO config-space pin-control addresses
const SIO_GPIO_PIN_SELECT: i32 = 0xF0;
const SIO_GPIO_PIN_CONFIG: i32 = 0xF1;
const SIO_GPIO_PIN_EVENT: i32 = 0xF2;

static mut superio_cmd: u8 = 0;
static mut selected_device: u8 = 0xFF; // bogus start val

// GPIO port runtime access, functionality
static mut port_offset: [i32; 4] = [0, 4, 8, 10]; // non-uniform offsets !
// static int event_capable[] = { 1, 1, 0, 0 };   ports 2,3 are hobbled

const PORT_OUT: i32 = 0;
const PORT_IN: i32 = 1;
const PORT_EVT_EN: i32 = 2;
const PORT_EVT_STST: i32 = 3;

static mut pdev: *mut platform_device = core::ptr::null_mut(); // use in dev_*()

unsafe fn superio_outb(addr: i32, val: i32) {
    outb_p(addr as u8, superio_cmd as u16);
    outb_p(val as u8, (superio_cmd as i32 + 1) as u16);
}

unsafe fn superio_inb(addr: i32) -> i32 {
    outb_p(addr as u8, superio_cmd as u16);
    inb_p((superio_cmd as i32 + 1) as u16) as i32
}

unsafe fn pc8736x_superio_present() -> i32 {
    let mut id: i32;
    superio_cmd = SIO_BASE1 as u8;
    id = superio_inb(SIO_SID);
    if id == SIO_SID_PC87365 || id == SIO_SID_PC87366 { return superio_cmd as i32; }
    superio_cmd = SIO_BASE2 as u8;
    id = superio_inb(SIO_SID);
    if id == SIO_SID_PC87365 || id == SIO_SID_PC87366 { return superio_cmd as i32; }
    0
}

unsafe fn device_select(devldn: u32) {
    superio_outb(SIO_UNIT_SEL, devldn as i32);
    selected_device = devldn as u8;
}

unsafe fn select_pin(iminor: u32) {
    // select GPIO port/pin from device minor number
    device_select(SIO_GPIO_UNIT);
    superio_outb(SIO_GPIO_PIN_SELECT, (((iminor << 1) & 0xF0) | (iminor & 0x7)) as i32);
}

unsafe fn pc8736x_gpio_configure_fn(index: u32, mask: u32, bits: u32, func_slct: u32) -> u32 {
    let config: u32;
    let new_config: u32;
    mutex_lock(&raw mut pc8736x_gpio_config_lock);
    device_select(SIO_GPIO_UNIT);
    select_pin(index);
    config = superio_inb(func_slct as i32) as u32;
    new_config = (config & mask) | bits;
    superio_outb(func_slct as i32, new_config as i32);
    mutex_unlock(&raw mut pc8736x_gpio_config_lock);
    config
}

unsafe fn pc8736x_gpio_configure(index: u32, mask: u32, bits: u32) -> u32 {
    pc8736x_gpio_configure_fn(index, mask, bits, SIO_GPIO_PIN_CONFIG as u32)
}

unsafe fn pc8736x_gpio_get(minor: u32) -> i32 {
    let port = (minor >> 3) as usize;
    let bit = minor & 7;
    let mut val = inb_p((pc8736x_gpio_base as i32 + port_offset[port] + PORT_IN) as u16) as i32;
    val >>= bit;
    val &= 1;
    dev_dbg!(&(*pdev).dev, "_gpio_get(%d from %x bit %d) == val %d\n", minor, pc8736x_gpio_base + port_offset[port] as u32 + PORT_IN as u32, bit, val);
    val
}

unsafe fn pc8736x_gpio_set(mut minor: u32, mut val: i32) {
    minor &= 0x1f;
    let port = (minor >> 3) as usize;
    let bit = minor & 7;
    let mut curval = inb_p((pc8736x_gpio_base as i32 + port_offset[port] + PORT_OUT) as u16) as i32;
    dev_dbg!(&(*pdev).dev, "addr:%x cur:%x bit-pos:%d cur-bit:%x + new:%d -> bit-new:%d\n", pc8736x_gpio_base + port_offset[port] as u32 + PORT_OUT as u32, curval, bit, curval & !(1 << bit), val, val << bit);
    val = (curval & !(1 << bit)) | (val << bit);
    dev_dbg!(&(*pdev).dev, "gpio_set(minor:%d port:%d bit:%d) %2x -> %2x\n", minor, port, bit, curval, val);
    outb_p(val as u8, (pc8736x_gpio_base as i32 + port_offset[port] + PORT_OUT) as u16);
    curval = inb_p((pc8736x_gpio_base as i32 + port_offset[port] + PORT_OUT) as u16) as i32;
    val = inb_p((pc8736x_gpio_base as i32 + port_offset[port] + PORT_IN) as u16) as i32;
    dev_dbg!(&(*pdev).dev, "wrote %x, read: %x\n", curval, val);
    pc8736x_gpio_shadow[port] = val as u8;
}

unsafe fn pc8736x_gpio_current(mut minor: u32) -> i32 {
    minor &= 0x1f;
    let port = (minor >> 3) as usize;
    let bit = minor & 7;
    ((pc8736x_gpio_shadow[port] >> bit) & 0x01) as i32
}

unsafe fn pc8736x_gpio_change(index: u32) {
    pc8736x_gpio_set(index, !pc8736x_gpio_current(index));
}

static mut pc8736x_gpio_ops: nsc_gpio_ops = nsc_gpio_ops {
    owner: THIS_MODULE,
    gpio_config: Some(pc8736x_gpio_configure),
    gpio_dump: Some(nsc_gpio_dump),
    gpio_get: Some(pc8736x_gpio_get),
    gpio_set: Some(pc8736x_gpio_set),
    gpio_change: Some(pc8736x_gpio_change),
    gpio_current: Some(pc8736x_gpio_current),
    dev: core::ptr::null_mut(),
};

unsafe fn pc8736x_gpio_open(inode: *mut inode, file: *mut file) -> i32 {
    let m = iminor(inode);
    (*file).private_data = &raw mut pc8736x_gpio_ops as *mut _ as *mut core::ffi::c_void;
    dev_dbg!(&(*pdev).dev, "open %d\n", m);
    if m >= PC8736X_GPIO_CT { return -EINVAL; }
    nonseekable_open(inode, file)
}

static pc8736x_gpio_fileops: file_operations = file_operations {
    owner: THIS_MODULE,
    open: Some(pc8736x_gpio_open),
    write: Some(nsc_gpio_write),
    read: Some(nsc_gpio_read),
};

unsafe fn pc8736x_init_shadow() {
    for port in 0..4 {
        pc8736x_gpio_shadow[port] = inb_p((pc8736x_gpio_base as i32 + port_offset[port] + PORT_OUT) as u16);
    }
}

static mut pc8736x_gpio_cdev: cdev = cdev::default();

unsafe fn pc8736x_gpio_init() -> i32 {
    let mut rc: i32;
    let mut devid: dev_t = 0;
    pdev = platform_device_alloc(DEVNAME, 0);
    if pdev.is_null() { return -ENOMEM; }
    rc = platform_device_add(pdev);
    if rc != 0 { rc = -ENODEV; goto!(undo_platform_dev_alloc); }
    dev_info!(&(*pdev).dev, "NatSemi pc8736x GPIO Driver Initializing\n");
    if pc8736x_superio_present() == 0 { rc = -ENODEV; dev_err!(&(*pdev).dev, "no device found\n"); goto!(undo_platform_dev_add); }
    pc8736x_gpio_ops.dev = &mut (*pdev).dev;
    rc = superio_inb(SIO_CF1);
    if (rc & 0x01) == 0 { rc = -ENODEV; dev_err!(&(*pdev).dev, "device not enabled\n"); goto!(undo_platform_dev_add); }
    device_select(SIO_GPIO_UNIT);
    if superio_inb(SIO_UNIT_ACT) == 0 { rc = -ENODEV; dev_err!(&(*pdev).dev, "GPIO unit not enabled\n"); goto!(undo_platform_dev_add); }
    pc8736x_gpio_base = ((superio_inb(SIO_BASE_HADDR) << 8) | superio_inb(SIO_BASE_LADDR)) as u32;
    if request_region(pc8736x_gpio_base, PC8736X_GPIO_RANGE, DEVNAME).is_null() { rc = -ENODEV; dev_err!(&(*pdev).dev, "GPIO ioport %x busy\n", pc8736x_gpio_base); goto!(undo_platform_dev_add); }
    dev_info!(&(*pdev).dev, "GPIO ioport %x reserved\n", pc8736x_gpio_base);
    if major != 0 { devid = MKDEV(major as u32, 0); rc = register_chrdev_region(devid, PC8736X_GPIO_CT, DEVNAME); }
    else { rc = alloc_chrdev_region(&mut devid, 0, PC8736X_GPIO_CT, DEVNAME); major = MAJOR(devid) as i32; }
    if rc < 0 { dev_err!(&(*pdev).dev, "register-chrdev failed: %d\n", rc); goto!(undo_request_region); }
    if major == 0 { major = rc; dev_dbg!(&(*pdev).dev, "got dynamic major %d\n", major); }
    pc8736x_init_shadow();
    cdev_init(&raw mut pc8736x_gpio_cdev, &pc8736x_gpio_fileops);
    cdev_add(&raw mut pc8736x_gpio_cdev, devid, PC8736X_GPIO_CT);
    return 0;
    label!(undo_request_region); release_region(pc8736x_gpio_base, PC8736X_GPIO_RANGE);
    label!(undo_platform_dev_add); platform_device_del(pdev);
    label!(undo_platform_dev_alloc); platform_device_put(pdev);
    rc
}

unsafe fn pc8736x_gpio_cleanup() {
    dev_dbg!(&(*pdev).dev, "cleanup\n");
    cdev_del(&raw mut pc8736x_gpio_cdev);
    unregister_chrdev_region(MKDEV(major as u32, 0), PC8736X_GPIO_CT);
    release_region(pc8736x_gpio_base, PC8736X_GPIO_RANGE);
    platform_device_unregister(pdev);
}

// module_init(pc8736x_gpio_init);
// module_exit(pc8736x_gpio_cleanup);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
