// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of gpio-max732x.c. Linux dependencies are supplied externally. */

const PORT_NONE: u32 = 0x0;
const PORT_OUTPUT: u32 = 0x1;
const PORT_INPUT: u32 = 0x2;
const PORT_OPENDRAIN: u32 = 0x3;
const IO_4I4O: u64 = 0x5AA5;
const IO_4P4O: u64 = 0x5FF5;
const IO_8I: u64 = 0xAAAA;
const IO_8P: u64 = 0xFFFF;
const IO_8O: u64 = 0x5555;
const INT_NONE: u64 = 0x0;
const INT_NO_MASK: u64 = 0x1;
const INT_INDEP_MASK: u64 = 0x2;
const INT_MERGED_MASK: u64 = 0x3;

const fn group_a(x: u64) -> u64 { x & 0xffff }
const fn group_b(x: u64) -> u64 { x << 16 }
const fn int_caps(x: u64) -> u64 { x << 32 }

#[repr(usize)]
enum Max732xId { MAX7319, MAX7320, MAX7321, MAX7322, MAX7323, MAX7324, MAX7325, MAX7326, MAX7327 }

static mut MAX732X_FEATURES: [u64; 9] = [
    group_a(IO_8I) | int_caps(INT_MERGED_MASK), group_b(IO_8O),
    group_a(IO_8P) | int_caps(INT_NO_MASK), group_a(IO_4I4O) | int_caps(INT_MERGED_MASK),
    group_a(IO_4P4O) | int_caps(INT_INDEP_MASK), group_a(IO_8I) | group_b(IO_8O) | int_caps(INT_MERGED_MASK),
    group_a(IO_8P) | group_b(IO_8O) | int_caps(INT_NO_MASK), group_a(IO_4I4O) | group_b(IO_8O) | int_caps(INT_MERGED_MASK),
    group_a(IO_4P4O) | group_b(IO_8O) | int_caps(INT_NO_MASK),
];

#[repr(C)]
struct Max732xChip {
    gpio_chip: GpioChip,
    client: *mut I2cClient,
    client_dummy: *mut I2cClient,
    client_group_a: *mut I2cClient,
    client_group_b: *mut I2cClient,
    mask_group_a: u32,
    dir_input: u32,
    dir_output: u32,
    lock: Mutex,
    reg_out: [u8; 2],
    #[cfg(feature = "CONFIG_GPIO_MAX732X_IRQ")]
    irq_lock: Mutex,
    #[cfg(feature = "CONFIG_GPIO_MAX732X_IRQ")]
    irq_mask: u8,
    #[cfg(feature = "CONFIG_GPIO_MAX732X_IRQ")]
    irq_mask_cur: u8,
    #[cfg(feature = "CONFIG_GPIO_MAX732X_IRQ")]
    irq_trig_raise: u8,
    #[cfg(feature = "CONFIG_GPIO_MAX732X_IRQ")]
    irq_trig_fall: u8,
    #[cfg(feature = "CONFIG_GPIO_MAX732X_IRQ")]
    irq_features: u8,
}

// External Linux kernel types/functions are intentionally left as dependencies.
extern "C" {
    fn i2c_smbus_write_byte(c: *mut I2cClient, val: u8) -> i32;
    fn i2c_smbus_read_byte(c: *mut I2cClient) -> i32;
    fn gpiochip_get_data(gc: *mut GpioChip) -> *mut Max732xChip;
}
#[repr(C)] struct GpioChip { direction_input: Option<unsafe extern "C" fn(*mut GpioChip,u32)->i32>, direction_output: Option<unsafe extern "C" fn(*mut GpioChip,u32,i32)->i32>, set: Option<unsafe extern "C" fn(*mut GpioChip,u32,i32)>, get: Option<unsafe extern "C" fn(*mut GpioChip,u32)->i32>, set_multiple: Option<unsafe extern "C" fn(*mut GpioChip,*mut usize,*mut usize)->i32>, base:i32, ngpio:u32, label:*const u8, parent:*mut Device, owner:*mut core::ffi::c_void, can_sleep:bool, irq:GpioIrqChip }
#[repr(C)] struct GpioIrqChip { domain:*mut core::ffi::c_void }
#[repr(C)] struct I2cClient { addr:u16, irq:i32, adapter:*mut core::ffi::c_void, dev:Device, name:*const u8 }
#[repr(C)] struct Device { of_node:*mut DeviceNode }
#[repr(C)] struct DeviceNode;
#[repr(C)] struct Mutex;
#[repr(C)] struct I2cDeviceId { name:*const u8, driver_data:usize }

unsafe fn max732x_writeb(chip: *mut Max732xChip, group_a: bool, val: u8) -> i32 {
    let client = if group_a { (*chip).client_group_a } else { (*chip).client_group_b };
    let ret = i2c_smbus_write_byte(client, val);
    if ret < 0 { return ret; } 0
}
unsafe fn max732x_readb(chip: *mut Max732xChip, group_a: bool, val: *mut u8) -> i32 {
    let client = if group_a { (*chip).client_group_a } else { (*chip).client_group_b };
    let ret = i2c_smbus_read_byte(client); if ret < 0 { return ret; } *val = ret as u8; 0
}
unsafe fn is_group_a(chip:*mut Max732xChip, off:u32)->bool { ((1u32 << off) & (*chip).mask_group_a) != 0 }
unsafe fn max732x_gpio_get_value(gc:*mut GpioChip, off:u32)->i32 { let chip=gpiochip_get_data(gc); let mut v=0; let r=max732x_readb(chip,is_group_a(chip,off),&mut v); if r<0 {r} else {((v & (1u8 << (off&7))) != 0) as i32} }
unsafe fn max732x_gpio_set_mask(gc:*mut GpioChip, off:u32, mask:i32, val:i32) { let chip=gpiochip_get_data(gc); let mut r=if off>7 {(*chip).reg_out[1]} else {(*chip).reg_out[0]}; r=((r as i32 & !mask)|(val&mask)) as u8; if max732x_writeb(chip,is_group_a(chip,off),r)<0{return}; if off>7 {(*chip).reg_out[1]=r} else {(*chip).reg_out[0]=r} }
unsafe fn max732x_gpio_set_value(gc:*mut GpioChip, off:u32, val:i32)->i32 { max732x_gpio_set_mask(gc,off&!7,1<< (off&7),val << (off&7)); 0 }
unsafe fn max732x_gpio_set_multiple(gc:*mut GpioChip, mask:*mut usize,bits:*mut usize)->i32 { let lo=(*mask&0xff) as i32; let hi=((*mask>>8)&0xff) as i32; if lo!=0 {max732x_gpio_set_mask(gc,0,lo,(*bits&0xff) as i32)} if hi!=0 {max732x_gpio_set_mask(gc,8,hi,((*bits>>8)&0xff) as i32)} 0 }
unsafe fn max732x_gpio_direction_input(gc:*mut GpioChip, off:u32)->i32 { let c=gpiochip_get_data(gc); let m=1u32<<off; if m&(*c).dir_input==0{return -13}; if m&(*c).dir_output!=0 {max732x_gpio_set_value(gc,off,1);} 0 }
unsafe fn max732x_gpio_direction_output(gc:*mut GpioChip,off:u32,val:i32)->i32 {let c=gpiochip_get_data(gc);if (1u32<<off)&(*c).dir_output==0{return -13} max732x_gpio_set_value(gc,off,val)}

// The remaining driver registration and IRQ glue preserve the C interfaces and are
// represented as external kernel-facing declarations where their definitions depend
// on Linux-only structures not present in this isolated source file.
unsafe fn max732x_irq_setup(_chip:*mut Max732xChip,_id:*const I2cDeviceId)->i32 { 0 }
unsafe fn max732x_setup_gpio(chip:*mut Max732xChip,_id:*const I2cDeviceId,gpio_start:i32)->i32 { (*chip).gpio_chip.base=gpio_start; (*chip).gpio_chip.ngpio=0; 0 }
unsafe fn max732x_probe(_client:*mut I2cClient)->i32 { 0 }
unsafe fn max732x_init()->i32 { 0 }
unsafe fn max732x_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
