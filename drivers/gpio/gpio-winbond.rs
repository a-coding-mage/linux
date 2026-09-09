// SPDX-License-Identifier: GPL-2.0+
/* GPIO interface for Winbond Super I/O chips. */

// Kernel dependencies supplied by the surrounding translation unit.
use core::ffi::c_void;

const WB_SIO_BASE: usize = 0x2e;
const WB_SIO_BASE_HIGH: usize = 0x4e;
const WB_SIO_EXT_ENTER_KEY: u8 = 0x87;
const WB_SIO_EXT_EXIT_KEY: u8 = 0xaa;
const WB_SIO_REG_LOGICAL: u8 = 0x07;
const WB_SIO_REG_CHIP_MSB: u8 = 0x20;
const WB_SIO_REG_CHIP_LSB: u8 = 0x21;
const WB_SIO_CHIP_ID_W83627UHG: u32 = 0xa230;
const WB_SIO_CHIP_ID_W83627UHG_MASK: u32 = 0xfff0;
const WB_SIO_REG_DPD: u8 = 0x22;
const WB_SIO_REG_DPD_UARTA: u8 = 4;
const WB_SIO_REG_DPD_UARTB: u8 = 5;
const WB_SIO_REG_IDPD: u8 = 0x23;
const WB_SIO_REG_IDPD_UARTC: u8 = 4;
const WB_SIO_REG_IDPD_UARTD: u8 = 5;
const WB_SIO_REG_IDPD_UARTE: u8 = 6;
const WB_SIO_REG_IDPD_UARTF: u8 = 7;
const WB_SIO_REG_GLOBAL_OPT: u8 = 0x24;
const WB_SIO_REG_GO_ENFDC: u8 = 1;
const WB_SIO_REG_OVTGPIO3456: u8 = 0x29;
const WB_SIO_REG_OG3456_G3PP: u8 = 3;
const WB_SIO_REG_OG3456_G4PP: u8 = 4;
const WB_SIO_REG_OG3456_G5PP: u8 = 5;
const WB_SIO_REG_OG3456_G6PP: u8 = 7;
const WB_SIO_REG_I2C_PS: u8 = 0x2a;
const WB_SIO_REG_I2CPS_I2CFS: u8 = 1;
const WB_SIO_REG_GPIO1_MF: u8 = 0x2c;
const WB_SIO_REG_G1MF_G1PP: u8 = 6;
const WB_SIO_REG_G1MF_G2PP: u8 = 7;
const WB_SIO_REG_G1MF_FS_MASK: u8 = 3;
const WB_SIO_REG_G1MF_FS_GPIO1: u8 = 2;
const WB_SIO_DEV_NONE: u8 = 0xff;

const WB_SIO_DEV_UARTB: u8 = 3; const WB_SIO_UARTB_REG_ENABLE: u8 = 0x30; const WB_SIO_UARTB_ENABLE_ON: u8 = 0;
const WB_SIO_DEV_UARTC: u8 = 6; const WB_SIO_UARTC_REG_ENABLE: u8 = 0x30; const WB_SIO_UARTC_ENABLE_ON: u8 = 0;
const WB_SIO_DEV_GPIO34: u8 = 7; const WB_SIO_GPIO34_REG_ENABLE: u8 = 0x30; const WB_SIO_GPIO34_ENABLE_3: u8 = 0; const WB_SIO_GPIO34_ENABLE_4: u8 = 1;
const WB_SIO_GPIO34_REG_IO3: u8 = 0xe0; const WB_SIO_GPIO34_REG_DATA3: u8 = 0xe1; const WB_SIO_GPIO34_REG_INV3: u8 = 0xe2; const WB_SIO_GPIO34_REG_IO4: u8 = 0xe4; const WB_SIO_GPIO34_REG_DATA4: u8 = 0xe5; const WB_SIO_GPIO34_REG_INV4: u8 = 0xe6;
const WB_SIO_DEV_WDGPIO56: u8 = 8; const WB_SIO_WDGPIO56_REG_ENABLE: u8 = 0x30; const WB_SIO_WDGPIO56_ENABLE_5: u8 = 1; const WB_SIO_WDGPIO56_ENABLE_6: u8 = 2;
const WB_SIO_WDGPIO56_REG_IO5: u8 = 0xe0; const WB_SIO_WDGPIO56_REG_DATA5: u8 = 0xe1; const WB_SIO_WDGPIO56_REG_INV5: u8 = 0xe2; const WB_SIO_WDGPIO56_REG_IO6: u8 = 0xe4; const WB_SIO_WDGPIO56_REG_DATA6: u8 = 0xe5; const WB_SIO_WDGPIO56_REG_INV6: u8 = 0xe6;
const WB_SIO_DEV_GPIO12: u8 = 9; const WB_SIO_GPIO12_REG_ENABLE: u8 = 0x30; const WB_SIO_GPIO12_ENABLE_1: u8 = 0; const WB_SIO_GPIO12_ENABLE_2: u8 = 1;
const WB_SIO_GPIO12_REG_IO1: u8 = 0xe0; const WB_SIO_GPIO12_REG_DATA1: u8 = 0xe1; const WB_SIO_GPIO12_REG_INV1: u8 = 0xe2; const WB_SIO_GPIO12_REG_IO2: u8 = 0xe4; const WB_SIO_GPIO12_REG_DATA2: u8 = 0xe5; const WB_SIO_GPIO12_REG_INV2: u8 = 0xe6;
const WB_SIO_DEV_UARTD: u8 = 0x0d; const WB_SIO_UARTD_REG_ENABLE: u8 = 0x30; const WB_SIO_UARTD_ENABLE_ON: u8 = 0;
const WB_SIO_DEV_UARTE: u8 = 0x0e; const WB_SIO_UARTE_REG_ENABLE: u8 = 0x30; const WB_SIO_UARTE_ENABLE_ON: u8 = 0;

#[repr(C)] struct WinbondGpioParams { base: usize, gpios: usize, ppgpios: usize, odgpios: usize, pledgpio: bool, beepgpio: bool, i2cgpio: bool }
static mut PARAMS: WinbondGpioParams = WinbondGpioParams { base: 0, gpios: 0, ppgpios: 0, odgpios: 0, pledgpio: false, beepgpio: false, i2cgpio: false };

extern "C" {
    fn request_muxed_region(base: usize, len: usize, name: *const i8) -> *mut c_void;
    fn release_region(base: usize, len: usize);
    fn inb(port: usize) -> u8;
    fn outb(value: u8, port: usize);
}

unsafe fn winbond_sio_enter(base: usize) -> i32 { if request_muxed_region(base, 2, core::ptr::null()) .is_null() { return -16; } outb(WB_SIO_EXT_ENTER_KEY, base); outb(WB_SIO_EXT_ENTER_KEY, base); 0 }
unsafe fn winbond_sio_select_logical(base: usize, dev: u8) { outb(WB_SIO_REG_LOGICAL, base); outb(dev, base + 1); }
unsafe fn winbond_sio_leave(base: usize) { outb(WB_SIO_EXT_EXIT_KEY, base); release_region(base, 2); }
unsafe fn winbond_sio_reg_write(base: usize, reg: u8, data: u8) { outb(reg, base); outb(data, base + 1); }
unsafe fn winbond_sio_reg_read(base: usize, reg: u8) -> u8 { outb(reg, base); inb(base + 1) }
unsafe fn winbond_sio_reg_bset(base: usize, reg: u8, bit: u8) { let v = winbond_sio_reg_read(base, reg) | (1u8 << bit); winbond_sio_reg_write(base, reg, v); }
unsafe fn winbond_sio_reg_bclear(base: usize, reg: u8, bit: u8) { let v = winbond_sio_reg_read(base, reg) & !(1u8 << bit); winbond_sio_reg_write(base, reg, v); }
unsafe fn winbond_sio_reg_btest(base: usize, reg: u8, bit: u8) -> bool { winbond_sio_reg_read(base, reg) & (1u8 << bit) != 0 }

#[repr(C)] struct WinbondGpioPortConflict { name: *const i8, dev: u8, testreg: u8, testbit: u8, warnonly: bool }
#[repr(C)] struct WinbondGpioInfo { dev: u8, enablereg: u8, enablebit: u8, outputreg: u8, outputppbit: u8, ioreg: u8, invreg: u8, datareg: u8, conflict: WinbondGpioPortConflict }
const NONE: WinbondGpioPortConflict = WinbondGpioPortConflict { name: core::ptr::null(), dev: 0, testreg: 0, testbit: 0, warnonly: false };
static WINBOND_GPIO_INFOS: [WinbondGpioInfo; 6] = [
    WinbondGpioInfo { dev:9,enablereg:0x30,enablebit:0,outputreg:0x2c,outputppbit:7,ioreg:0xe0,invreg:0xe2,datareg:0xe1,conflict:NONE },
    WinbondGpioInfo { dev:9,enablereg:0x30,enablebit:1,outputreg:0x2c,outputppbit:7,ioreg:0xe4,invreg:0xe6,datareg:0xe5,conflict:NONE },
    WinbondGpioInfo { dev:7,enablereg:0x30,enablebit:0,outputreg:0x29,outputppbit:3,ioreg:0xe0,invreg:0xe2,datareg:0xe1,conflict:NONE },
    WinbondGpioInfo { dev:7,enablereg:0x30,enablebit:1,outputreg:0x29,outputppbit:4,ioreg:0xe4,invreg:0xe6,datareg:0xe5,conflict:NONE },
    WinbondGpioInfo { dev:8,enablereg:0x30,enablebit:1,outputreg:0x29,outputppbit:5,ioreg:0xe0,invreg:0xe2,datareg:0xe1,conflict:NONE },
    WinbondGpioInfo { dev:8,enablereg:0x30,enablebit:2,outputreg:0x29,outputppbit:7,ioreg:0xe4,invreg:0xe6,datareg:0xe5,conflict:NONE },
];

// The remaining GPIO-chip and ISA-driver glue is represented with the same C ABI;
// kernel-provided types and logging/module facilities are dependencies of this file.
extern "C" { fn devm_gpiochip_add_data(dev: *mut c_void, chip: *mut c_void, data: *mut c_void) -> i32; }

#[repr(C)] struct GpioChip { base: i32, label: *const i8, owner: *mut c_void, can_sleep: bool, get: Option<unsafe extern "C" fn(*mut GpioChip,u32)->i32>, direction_input: Option<unsafe extern "C" fn(*mut GpioChip,u32)->i32>, set: Option<unsafe extern "C" fn(*mut GpioChip,u32,i32)>, direction_output: Option<unsafe extern "C" fn(*mut GpioChip,u32,i32)->i32>, ngpio: u32, parent: *mut c_void }
unsafe fn gpiochip_get_data(_gc: *mut GpioChip) -> *mut usize { core::ptr::addr_of_mut!(PARAMS.base) }

unsafe fn winbond_gpio_get_info(gpio_num: &mut u32) -> (&'static WinbondGpioInfo, bool) {
    let mut i = 0usize; while i < 6 { if (*gpio_num) < 8 { break; } *gpio_num -= 8; i += 1; }
    let mut allow = true; if i == 1 { if (*gpio_num == 0 && !PARAMS.pledgpio) || (*gpio_num == 1 && !PARAMS.beepgpio) || ((*gpio_num == 5 || *gpio_num == 6) && !PARAMS.i2cgpio) { allow = false; } }
    (&WINBOND_GPIO_INFOS[i], allow)
}
unsafe extern "C" fn winbond_gpio_get(gc: *mut GpioChip, mut offset: u32) -> i32 { let base=**gpiochip_get_data(gc); let (info,_)=winbond_gpio_get_info(&mut offset); if winbond_sio_enter(base)!=0{return -16}; winbond_sio_select_logical(base,info.dev); let mut v=winbond_sio_reg_btest(base,info.datareg,offset as u8); if winbond_sio_reg_btest(base,info.invreg,offset as u8){v=!v}; winbond_sio_leave(base); v as i32 }
unsafe extern "C" fn winbond_gpio_direction_in(gc:*mut GpioChip,mut offset:u32)->i32 {let base=**gpiochip_get_data(gc);let(info,ok)=winbond_gpio_get_info(&mut offset);if !ok{return -13}let r=winbond_sio_enter(base);if r!=0{return r}winbond_sio_select_logical(base,info.dev);winbond_sio_reg_bset(base,info.ioreg,offset as u8);winbond_sio_leave(base);0}
unsafe extern "C" fn winbond_gpio_direction_out(gc:*mut GpioChip,mut offset:u32,mut val:i32)->i32 {let base=**gpiochip_get_data(gc);let(info,ok)=winbond_gpio_get_info(&mut offset);if !ok{return -13}let r=winbond_sio_enter(base);if r!=0{return r}winbond_sio_select_logical(base,info.dev);winbond_sio_reg_bclear(base,info.ioreg,offset as u8);if winbond_sio_reg_btest(base,info.invreg,offset as u8){val=!val}if val!=0{winbond_sio_reg_bset(base,info.datareg,offset as u8)}else{winbond_sio_reg_bclear(base,info.datareg,offset as u8)}winbond_sio_leave(base);0}
unsafe extern "C" fn winbond_gpio_set(gc:*mut GpioChip,mut offset:u32,mut val:i32){let base=**gpiochip_get_data(gc);let(info,ok)=winbond_gpio_get_info(&mut offset);if !ok{return}if winbond_sio_enter(base)!=0{return}winbond_sio_select_logical(base,info.dev);if winbond_sio_reg_btest(base,info.invreg,offset as u8){val=!val}if val!=0{winbond_sio_reg_bset(base,info.datareg,offset as u8)}else{winbond_sio_reg_bclear(base,info.datareg,offset as u8)}winbond_sio_leave(base)}

static mut WINBOND_GPIO_CHIP: GpioChip = GpioChip { base:-1,label:core::ptr::null(),owner:core::ptr::null_mut(),can_sleep:true,get:Some(winbond_gpio_get),direction_input:Some(winbond_gpio_direction_in),set:Some(winbond_gpio_set),direction_output:Some(winbond_gpio_direction_out),ngpio:0,parent:core::ptr::null_mut() };

unsafe fn winbond_gpio_configure_port0_pins(base:usize){let mut v=winbond_sio_reg_read(base,WB_SIO_REG_GPIO1_MF);if(v&3)==WB_SIO_REG_G1MF_FS_GPIO1{return}v&=!3;v|=WB_SIO_REG_G1MF_FS_GPIO1;winbond_sio_reg_write(base,WB_SIO_REG_GPIO1_MF,v)}
unsafe fn winbond_gpio_configure_port1_check_i2c(base:usize){PARAMS.i2cgpio=!winbond_sio_reg_btest(base,WB_SIO_REG_I2C_PS,WB_SIO_REG_I2CPS_I2CFS)}
unsafe fn winbond_gpio_configure_port(base:usize,idx:usize)->bool{let i=&WINBOND_GPIO_INFOS[idx];if idx==0{winbond_gpio_configure_port0_pins(base)}else if idx==1{winbond_gpio_configure_port1_check_i2c(base)};winbond_sio_select_logical(base,i.dev);winbond_sio_reg_bset(base,i.enablereg,i.enablebit);if PARAMS.ppgpios&(1<<idx)!=0{winbond_sio_reg_bset(base,i.outputreg,i.outputppbit)}else if PARAMS.odgpios&(1<<idx)!=0{winbond_sio_reg_bclear(base,i.outputreg,i.outputppbit)};true}
unsafe fn winbond_gpio_configure(base:usize)->i32{for i in 0..6{if PARAMS.gpios&(1<<i)!=0&&!winbond_gpio_configure_port(base,i){PARAMS.gpios&=!(1<<i)}}if PARAMS.gpios==0{-22}else{0}}
unsafe fn winbond_gpio_check_chip(base:usize)->i32{let r=winbond_sio_enter(base);if r!=0{return r}let chip=((winbond_sio_reg_read(base,WB_SIO_REG_CHIP_MSB)as u32)<<8)|winbond_sio_reg_read(base,WB_SIO_REG_CHIP_LSB)as u32;winbond_sio_leave(base);if chip&WB_SIO_CHIP_ID_W83627UHG_MASK==WB_SIO_CHIP_ID_W83627UHG{0}else{-19}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
