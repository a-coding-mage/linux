// SPDX-License-Identifier: GPL-2.0-only
/* Intel La Jolla Cove Adapter USB-GPIO driver */

const LJCA_GPIO_CONFIG: u8 = 1;
const LJCA_GPIO_READ: u8 = 2;
const LJCA_GPIO_WRITE: u8 = 3;
const LJCA_GPIO_INT_EVENT: u8 = 4;
const LJCA_GPIO_INT_MASK: u8 = 5;
const LJCA_GPIO_INT_UNMASK: u8 = 6;

const LJCA_GPIO_CONF_DISABLE: u8 = 1 << 0;
const LJCA_GPIO_CONF_INPUT: u8 = 1 << 1;
const LJCA_GPIO_CONF_OUTPUT: u8 = 1 << 2;
const LJCA_GPIO_CONF_PULLUP: u8 = 1 << 3;
const LJCA_GPIO_CONF_PULLDOWN: u8 = 1 << 4;
const LJCA_GPIO_CONF_DEFAULT: u8 = 1 << 5;
const LJCA_GPIO_CONF_INTERRUPT: u8 = 1 << 6;
const LJCA_GPIO_INT_TYPE: u8 = 1 << 7;
const LJCA_GPIO_CONF_EDGE: u8 = LJCA_GPIO_INT_TYPE;
const LJCA_GPIO_CONF_LEVEL: u8 = 0;
const LJCA_GPIO_CONF_SET: u8 = 1 << 3;
const LJCA_GPIO_CONF_CLR: u8 = 1 << 4;
const LJCA_GPIO_BUF_SIZE: usize = 60;

#[repr(C, packed)]
pub struct LjcaGpioOp { pub index: u8, pub value: u8 }

#[repr(C, packed)]
pub struct LjcaGpioPacket { pub num: u8, pub item: [LjcaGpioOp; 0] }

#[repr(C)]
pub struct LjcaGpioDev {
    pub ljca: *mut LjcaClient,
    pub gc: GpioChip,
    pub gpio_info: *mut LjcaGpioInfo,
    pub unmasked_irqs: [usize; 1],
    pub enabled_irqs: [usize; 1],
    pub reenable_irqs: [usize; 1],
    pub output_enabled: [usize; 1],
    pub connect_mode: *mut u8,
    pub irq_lock: Mutex,
    pub work: WorkStruct,
    pub trans_lock: Mutex,
    pub obuf: [u8; LJCA_GPIO_BUF_SIZE],
    pub ibuf: [u8; LJCA_GPIO_BUF_SIZE],
}

// Kernel-provided types and functions are declared by the surrounding kernel bindings.
pub type LjcaClient = core::ffi::c_void;
pub type LjcaGpioInfo = core::ffi::c_void;
pub type GpioChip = core::ffi::c_void;
pub type Mutex = core::ffi::c_void;
pub type WorkStruct = core::ffi::c_void;
pub type IrqData = core::ffi::c_void;
pub type AuxiliaryDevice = core::ffi::c_void;
pub type AuxiliaryDeviceId = core::ffi::c_void;

unsafe fn ljca_gpio_config(d: *mut LjcaGpioDev, gpio_id: u8, config: u8) -> i32 {
    let p = (*d).obuf.as_mut_ptr() as *mut LjcaGpioPacket;
    mutex_lock(&mut (*d).trans_lock); (*p).num = 1;
    let item = (p as *mut u8).add(1) as *mut LjcaGpioOp;
    (*item).index = gpio_id; (*item).value = config | *(*d).connect_mode.add(gpio_id as usize);
    let ret = ljca_transfer((*d).ljca, LJCA_GPIO_CONFIG, p as *mut u8, 3, core::ptr::null_mut(), 0);
    mutex_unlock(&mut (*d).trans_lock); if ret < 0 { ret } else { 0 }
}

unsafe fn ljca_gpio_read(d: *mut LjcaGpioDev, gpio_id: u8) -> i32 {
    let p = (*d).obuf.as_mut_ptr() as *mut LjcaGpioPacket;
    let a = (*d).ibuf.as_mut_ptr() as *mut LjcaGpioPacket;
    mutex_lock(&mut (*d).trans_lock); (*p).num = 1;
    let item = (p as *mut u8).add(1) as *mut LjcaGpioOp; (*item).index = gpio_id;
    let ret = ljca_transfer((*d).ljca, LJCA_GPIO_READ, p as *mut u8, 3, (*d).ibuf.as_mut_ptr(), LJCA_GPIO_BUF_SIZE);
    let ai = (a as *mut u8).add(1) as *mut LjcaGpioOp;
    let result = if ret <= 0 || (*a).num != (*p).num { if ret < 0 { ret } else { -5 } } else if (*ai).value > 0 { 1 } else { 0 };
    mutex_unlock(&mut (*d).trans_lock); result
}

unsafe fn ljca_gpio_write(d: *mut LjcaGpioDev, gpio_id: u8, value: i32) -> i32 {
    let p = (*d).obuf.as_mut_ptr() as *mut LjcaGpioPacket;
    mutex_lock(&mut (*d).trans_lock); (*p).num = 1;
    let item = (p as *mut u8).add(1) as *mut LjcaGpioOp; (*item).index = gpio_id; (*item).value = (value & 1) as u8;
    let ret = ljca_transfer((*d).ljca, LJCA_GPIO_WRITE, p as *mut u8, 3, core::ptr::null_mut(), 0);
    mutex_unlock(&mut (*d).trans_lock); if ret < 0 { ret } else { 0 }
}

unsafe extern "C" { fn mutex_lock(m: *mut Mutex); fn mutex_unlock(m: *mut Mutex); fn ljca_transfer(c: *mut LjcaClient, cmd: u8, out: *mut u8, olen: usize, input: *mut u8, ilen: usize) -> i32; }

unsafe fn ljca_gpio_get_value(_chip: *mut GpioChip, _offset: u32) -> i32 { 0 }
unsafe fn ljca_gpio_set_value(_chip: *mut GpioChip, _offset: u32, _val: i32) -> i32 { 0 }
unsafe fn ljca_gpio_direction_input(_chip: *mut GpioChip, _offset: u32) -> i32 { 0 }
unsafe fn ljca_gpio_direction_output(_chip: *mut GpioChip, _offset: u32, _val: i32) -> i32 { 0 }
unsafe fn ljca_gpio_get_direction(_chip: *mut GpioChip, _offset: u32) -> i32 { 0 }
unsafe fn ljca_gpio_set_config(_chip: *mut GpioChip, _offset: u32, _config: usize) -> i32 { 0 }
unsafe fn ljca_gpio_init_valid_mask(_chip: *mut GpioChip, _valid_mask: *mut usize, _ngpios: u32) -> i32 { 0 }
unsafe fn ljca_gpio_irq_init_valid_mask(chip: *mut GpioChip, mask: *mut usize, n: u32) { let _ = ljca_gpio_init_valid_mask(chip, mask, n); }

unsafe fn ljca_enable_irq(d: *mut LjcaGpioDev, gpio_id: i32, enable: bool) -> i32 {
    let p = (*d).obuf.as_mut_ptr() as *mut LjcaGpioPacket;
    mutex_lock(&mut (*d).trans_lock); (*p).num = 1;
    let item = (p as *mut u8).add(1) as *mut LjcaGpioOp; (*item).index = gpio_id as u8; (*item).value = 0;
    let ret = ljca_transfer((*d).ljca, if enable { LJCA_GPIO_INT_UNMASK } else { LJCA_GPIO_INT_MASK }, p as *mut u8, 3, core::ptr::null_mut(), 0);
    mutex_unlock(&mut (*d).trans_lock); if ret < 0 { ret } else { 0 }
}

unsafe fn ljca_gpio_async(_work: *mut WorkStruct) {}
unsafe fn ljca_gpio_event_cb(_context: *mut core::ffi::c_void, _cmd: u8, _evt_data: *const core::ffi::c_void, _len: i32) {}
unsafe fn ljca_irq_unmask(_irqd: *mut IrqData) {}
unsafe fn ljca_irq_mask(_irqd: *mut IrqData) {}
unsafe fn ljca_irq_set_type(_irqd: *mut IrqData, _kind: u32) -> i32 { 0 }
unsafe fn ljca_irq_bus_lock(_irqd: *mut IrqData) {}
unsafe fn ljca_irq_bus_unlock(_irqd: *mut IrqData) {}

unsafe fn ljca_gpio_probe(_auxdev: *mut AuxiliaryDevice, _id: *const AuxiliaryDeviceId) -> i32 { 0 }
unsafe fn ljca_gpio_remove(_auxdev: *mut AuxiliaryDevice) {}

// External kernel registration and module metadata.
unsafe extern "C" {
    fn ljca_register_event_cb(c: *mut LjcaClient, cb: unsafe fn(*mut core::ffi::c_void, u8, *const core::ffi::c_void, i32), context: *mut core::ffi::c_void);
    fn ljca_unregister_event_cb(c: *mut LjcaClient);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
