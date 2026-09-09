// SPDX-License-Identifier: GPL-2.0-or-later
/*

    bt8xx GPIO abuser

    Copyright (C) 2008 Michael Buesch <m@bues.ch>

    Please do _only_ contact the people listed _above_ with issues related to this driver.
    All the other people listed below are not related to this driver. Their names
    are only here, because this driver is derived from the bt848 driver.

    Derived from the bt848 driver:

    Copyright (C) 1996,97,98 Ralph  Metzler
                           & Marcus Metzler
    (c) 1999-2002 Gerd Knorr

    some v4l2 code lines are taken from Justin's bttv2 driver which is
    (c) 2000 Justin Schoeman

    V4L1 removal from:
    (c) 2005-2006 Nickolay V. Shmyrev

    Fixes to be fully V4L2 compliant by
    (c) 2006 Mauro Carvalho Chehab

    Cropping and overscan support
    Copyright (C) 2005, 2006 Michael H. Schimek
    Sponsored by OPQ Systems AB
*/

// Kernel dependencies supplied externally: cleanup, module, PCI, spinlock,
// GPIO driver, slab allocation, and the bt848 hardware definitions.

const BT8XXGPIO_NR_GPIOS: u32 = 24; /* We have 24 GPIO pins */

#[repr(C)]
struct Bt8xxgpio {
    lock: SpinlockT,
    mmio: *mut core::ffi::c_void,
    pdev: *mut PciDev,
    gpio: GpioChip,
    saved_outen: u32,
    saved_data: u32,
}

type SpinlockT = core::ffi::c_void;
#[repr(C)] struct PciDev { _private: [u8; 0] }
#[repr(C)] struct Device { _private: [u8; 0] }
#[repr(C)] struct GpioChip {
    label: *const core::ffi::c_char,
    owner: *mut core::ffi::c_void,
    direction_input: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    get: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    direction_output: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32)>,
    dbg_show: *mut core::ffi::c_void,
    base: i32,
    ngpio: u32,
    can_sleep: bool,
}

#[repr(C)] struct PciDeviceId { vendor: u32, device: u32 }
#[repr(C)] struct PciDriver {
    name: *const core::ffi::c_char,
    id_table: *const PciDeviceId,
    probe: Option<unsafe extern "C" fn(*mut PciDev, *const PciDeviceId) -> i32>,
    remove: Option<unsafe extern "C" fn(*mut PciDev)>,
    pm: *const core::ffi::c_void,
}

extern "C" {
    static mut modparam_gpiobase: i32;
    static THIS_MODULE: core::ffi::c_void;
    fn gpiochip_get_data(gpio: *mut GpioChip) -> *mut Bt8xxgpio;
    fn readl(addr: *mut core::ffi::c_void) -> u32;
    fn writel(value: u32, addr: *mut core::ffi::c_void);
    fn dev_name(dev: *mut Device) -> *const core::ffi::c_char;
    fn pci_resource_n(dev: *mut PciDev, bar: u32) -> u64;
    fn devm_ioremap_resource(dev: *mut Device, resource: u64) -> *mut core::ffi::c_void;
    fn is_err(ptr: *mut core::ffi::c_void) -> bool;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut Bt8xxgpio;
    fn spin_lock_init(lock: *mut SpinlockT);
    fn pci_enable_device(dev: *mut PciDev) -> i32;
    fn pci_set_master(dev: *mut PciDev);
    fn pci_set_drvdata(dev: *mut PciDev, data: *mut Bt8xxgpio);
    fn pci_get_drvdata(dev: *mut PciDev) -> *mut Bt8xxgpio;
    fn pci_disable_device(dev: *mut PciDev);
    fn gpiochip_add_data(chip: *mut GpioChip, data: *mut Bt8xxgpio) -> i32;
    fn gpiochip_remove(chip: *mut GpioChip);
    fn to_pci_dev(dev: *mut Device) -> *mut PciDev;
}

const BT848_GPIO_DATA: usize = 0;
const BT848_GPIO_OUT_EN: usize = 0;
const BT848_INT_MASK: usize = 0;
const BT848_GPIO_DMA_CTL: usize = 0;
const BT848_GPIO_REG_INP: usize = 0;
const BT848_INT_STAT: usize = 0;
const GFP_KERNEL: u32 = 0;
const PCI_VENDOR_ID_BROOKTREE: u32 = 0;
const PCI_DEVICE_ID_BT848: u32 = 0;
const PCI_DEVICE_ID_BT849: u32 = 0;
const PCI_DEVICE_ID_BT878: u32 = 0;
const PCI_DEVICE_ID_BT879: u32 = 0;

unsafe fn bgwrite(bg: *mut Bt8xxgpio, dat: u32, adr: usize) { writel(dat, (*bg).mmio.add(adr)); }
unsafe fn bgread(bg: *mut Bt8xxgpio, adr: usize) -> u32 { readl((*bg).mmio.add(adr)) }

unsafe extern "C" fn bt8xxgpio_gpio_direction_input(gpio: *mut GpioChip, nr: u32) -> i32 {
    let bg = gpiochip_get_data(gpio); let mut data; let mut outen;
    data = bgread(bg, BT848_GPIO_DATA); data &= !(1u32 << nr); bgwrite(bg, data, BT848_GPIO_DATA);
    outen = bgread(bg, BT848_GPIO_OUT_EN); outen &= !(1u32 << nr); bgwrite(bg, outen, BT848_GPIO_OUT_EN); 0
}
unsafe extern "C" fn bt8xxgpio_gpio_get(gpio: *mut GpioChip, nr: u32) -> i32 {
    let bg = gpiochip_get_data(gpio); ((bgread(bg, BT848_GPIO_DATA) & (1u32 << nr)) != 0) as i32
}
unsafe extern "C" fn bt8xxgpio_gpio_direction_output(gpio: *mut GpioChip, nr: u32, val: i32) -> i32 {
    let bg = gpiochip_get_data(gpio); let mut outen = bgread(bg, BT848_GPIO_OUT_EN); outen |= 1u32 << nr; bgwrite(bg, outen, BT848_GPIO_OUT_EN);
    let mut data = bgread(bg, BT848_GPIO_DATA); if val != 0 { data |= 1u32 << nr; } else { data &= !(1u32 << nr); } bgwrite(bg, data, BT848_GPIO_DATA); 0
}
unsafe extern "C" fn bt8xxgpio_gpio_set(gpio: *mut GpioChip, nr: u32, val: i32) { let bg = gpiochip_get_data(gpio); let mut data = bgread(bg, BT848_GPIO_DATA); if val != 0 { data |= 1u32 << nr; } else { data &= !(1u32 << nr); } bgwrite(bg, data, BT848_GPIO_DATA); }

unsafe fn bt8xxgpio_gpio_setup(bg: *mut Bt8xxgpio) {
    let c = &mut (*bg).gpio; c.label = dev_name(core::ptr::null_mut()); c.owner = &THIS_MODULE as *const _ as *mut _;
    c.direction_input = Some(bt8xxgpio_gpio_direction_input); c.get = Some(bt8xxgpio_gpio_get); c.direction_output = Some(bt8xxgpio_gpio_direction_output); c.set = Some(bt8xxgpio_gpio_set); c.dbg_show = core::ptr::null_mut(); c.base = modparam_gpiobase; c.ngpio = BT8XXGPIO_NR_GPIOS; c.can_sleep = false;
}

unsafe extern "C" fn bt8xxgpio_probe(dev: *mut PciDev, _pci_id: *const PciDeviceId) -> i32 {
    let mmio = devm_ioremap_resource(core::ptr::null_mut(), pci_resource_n(dev, 0)); if is_err(mmio) { return ptr_err(mmio); }
    let bg = devm_kzalloc(core::ptr::null_mut(), core::mem::size_of::<Bt8xxgpio>(), GFP_KERNEL); if bg.is_null() { return -12; }
    (*bg).mmio = mmio; (*bg).pdev = dev; spin_lock_init(&mut (*bg).lock); let err = pci_enable_device(dev); if err != 0 { return err; } pci_set_master(dev); pci_set_drvdata(dev, bg);
    bgwrite(bg, 0, BT848_INT_MASK); bgwrite(bg, 0, BT848_GPIO_DMA_CTL); bgwrite(bg, 0, BT848_GPIO_REG_INP); bgwrite(bg, 0, BT848_GPIO_OUT_EN); bt8xxgpio_gpio_setup(bg);
    let err = gpiochip_add_data(&mut (*bg).gpio, bg); if err != 0 { pci_disable_device(dev); return err; } 0
}
unsafe extern "C" fn bt8xxgpio_remove(pdev: *mut PciDev) { let bg = pci_get_drvdata(pdev); gpiochip_remove(&mut (*bg).gpio); bgwrite(bg, 0, BT848_INT_MASK); bgwrite(bg, !0, BT848_INT_STAT); bgwrite(bg, 0, BT848_GPIO_OUT_EN); pci_disable_device(pdev); }
unsafe extern "C" fn bt8xxgpio_suspend(dev: *mut Device) -> i32 { let pdev = to_pci_dev(dev); let bg = pci_get_drvdata(pdev); (*bg).saved_outen = bgread(bg, BT848_GPIO_OUT_EN); (*bg).saved_data = bgread(bg, BT848_GPIO_DATA); bgwrite(bg, 0, BT848_INT_MASK); bgwrite(bg, !0, BT848_INT_STAT); bgwrite(bg, 0, BT848_GPIO_OUT_EN); 0 }
unsafe extern "C" fn bt8xxgpio_resume(dev: *mut Device) -> i32 { let pdev = to_pci_dev(dev); let bg = pci_get_drvdata(pdev); bgwrite(bg, 0, BT848_INT_MASK); bgwrite(bg, 0, BT848_GPIO_DMA_CTL); bgwrite(bg, 0, BT848_GPIO_REG_INP); bgwrite(bg, (*bg).saved_outen, BT848_GPIO_OUT_EN); bgwrite(bg, (*bg).saved_data & (*bg).saved_outen, BT848_GPIO_DATA); 0 }

static BT8XXGPIO_PCI_TBL: [PciDeviceId; 5] = [
    PciDeviceId { vendor: PCI_VENDOR_ID_BROOKTREE, device: PCI_DEVICE_ID_BT848 }, PciDeviceId { vendor: PCI_VENDOR_ID_BROOKTREE, device: PCI_DEVICE_ID_BT849 }, PciDeviceId { vendor: PCI_VENDOR_ID_BROOKTREE, device: PCI_DEVICE_ID_BT878 }, PciDeviceId { vendor: PCI_VENDOR_ID_BROOKTREE, device: PCI_DEVICE_ID_BT879 }, PciDeviceId { vendor: 0, device: 0 },
];
static mut BT8XXGPIO_PCI_DRIVER: PciDriver = PciDriver { name: b"bt8xxgpio\0".as_ptr() as *const _, id_table: BT8XXGPIO_PCI_TBL.as_ptr(), probe: Some(bt8xxgpio_probe), remove: Some(bt8xxgpio_remove), pm: core::ptr::null() };
// Equivalent to module_pci_driver(bt8xxgpio_pci_driver), MODULE_DEVICE_TABLE,
// module_param_named, MODULE_PARM_DESC, MODULE_LICENSE, MODULE_AUTHOR, and
// MODULE_DESCRIPTION; registration and metadata are supplied by the kernel.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
