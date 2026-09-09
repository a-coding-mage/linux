// SPDX-License-Identifier: GPL-2.0
// Samsung GPIOlib support. C headers and externally supplied kernel symbols are
// intentionally represented by declarations below.

use core::ffi::c_void;

extern "C" {
    fn __raw_readl(reg: *mut c_void) -> u32;
    fn __raw_writel(value: u32, reg: *mut c_void);
    fn samsung_gpio_is_cfg_special(cfg: u32) -> bool;
    fn samsung_gpio_lock(chip: *mut samsung_gpio_chip, flags: *mut usize);
    fn samsung_gpio_unlock(chip: *mut samsung_gpio_chip, flags: usize);
    fn samsung_gpio_do_setcfg(chip: *mut samsung_gpio_chip, off: i32, cfg: u32) -> i32;
    fn samsung_gpio_do_setpull(chip: *mut samsung_gpio_chip, off: i32, pull: samsung_gpio_pull_t) -> i32;
    fn samsung_gpiolib_getchip(pin: u32) -> *mut samsung_gpio_chip;
    fn s3c_gpio_setpull(pin: u32, pull: samsung_gpio_pull_t) -> i32;
    fn gpiochip_add_data(chip: *mut gpio_chip, data: *mut samsung_gpio_chip) -> i32;
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut samsung_gpio_chip;
    fn of_have_populated_dt() -> bool;
    fn soc_is_s3c64xx() -> bool;
}

type samsung_gpio_pull_t = u32;
type gpio_fn = unsafe extern "C" fn(*mut gpio_chip, u32) -> i32;
type gpio_out_fn = unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32;
type cfg_fn = unsafe extern "C" fn(*mut samsung_gpio_chip, u32, u32) -> i32;
type getcfg_fn = unsafe extern "C" fn(*mut samsung_gpio_chip, u32) -> u32;
type pull_fn = unsafe extern "C" fn(*mut samsung_gpio_chip, u32, samsung_gpio_pull_t) -> i32;
type getpull_fn = unsafe extern "C" fn(*mut samsung_gpio_chip, u32) -> samsung_gpio_pull_t;

#[repr(C)] pub struct gpio_chip {
    pub base: u32, pub ngpio: u32, pub label: *const u8,
    pub direction_input: Option<gpio_fn>, pub direction_output: Option<gpio_out_fn>,
    pub set: Option<gpio_out_fn>, pub get: Option<gpio_fn>, pub to_irq: Option<unsafe extern "C" fn(*mut gpio_chip,u32)->i32>,
}
#[repr(C)] pub struct samsung_gpio_cfg { pub cfg_eint:u32, pub set_config:Option<cfg_fn>, pub get_config:Option<getcfg_fn>, pub set_pull:Option<pull_fn>, pub get_pull:Option<getpull_fn> }
#[repr(C)] pub struct samsung_gpio_chip { pub chip:gpio_chip, pub base:*mut c_void, pub config:*mut samsung_gpio_cfg, pub pm:*mut c_void, pub lock:u8, pub irq_base:i32, pub bitmap_gpio_int:u32 }

const GPIOCON_OFF: usize = 0; const GPIODAT_OFF: usize = 4;
const EINVAL:i32 = -22; const ENXIO:i32 = -6;
#[inline] unsafe fn special(v:u32)->u32 { (v & 0x8000_0000) | (v & 0xf) }
#[inline] unsafe fn shift4(o:u32)->u32 { (o & 7) * 4 }
unsafe fn setpull(c:*mut samsung_gpio_chip,o:u32,p:samsung_gpio_pull_t)->i32 { let r=(*c).base.add(8); let s=o*2; let mut v=__raw_readl(r); v &= !(3<<s); v |= p<<s; __raw_writel(v,r); 0 }
unsafe fn getpull(c:*mut samsung_gpio_chip,o:u32)->samsung_gpio_pull_t { let mut v=__raw_readl((*c).base.add(8)); v >>= o*2; v&=3 }
unsafe fn setcfg2(c:*mut samsung_gpio_chip,o:u32,mut cfg:u32)->i32 { let s=o*2; if samsung_gpio_is_cfg_special(cfg){cfg&=15;if cfg>3{return EINVAL}cfg<<=s;} let r=(*c).base;let mut v=__raw_readl(r);v&=!(3<<s);v|=cfg;__raw_writel(v,r);0 }
unsafe fn getcfg2(c:*mut samsung_gpio_chip,o:u32)->u32 { special((__raw_readl((*c).base)>>o*2)&3) }
unsafe fn setcfg4(c:*mut samsung_gpio_chip,o:u32,mut cfg:u32)->i32 { let mut r=(*c).base;let s=(o&7)*4;if o<8&&(*c).chip.ngpio>8{r=r.sub(4)} if samsung_gpio_is_cfg_special(cfg){cfg=(cfg&15)<<s}let mut v=__raw_readl(r);v&=!(15<<s);v|=cfg;__raw_writel(v,r);0 }
unsafe fn getcfg4(c:*mut samsung_gpio_chip,o:u32)->u32 { let mut r=(*c).base;let s=(o&7)*4;if o<8&&(*c).chip.ngpio>8{r=r.sub(4)} special((__raw_readl(r)>>s)&15) }

unsafe fn input2(ch:*mut gpio_chip,o:u32)->i32 { let c=ch as *mut samsung_gpio_chip;let mut f=0; samsung_gpio_lock(c,&mut f);let r=(*c).base;let mut v=__raw_readl(r);v&=!(3<<(o*2));__raw_writel(v,r);samsung_gpio_unlock(c,f);0 }
unsafe fn output2(ch:*mut gpio_chip,o:u32,val:i32)->i32 {let c=ch as *mut samsung_gpio_chip;let mut f=0;samsung_gpio_lock(c,&mut f);let r=(*c).base;let mut d=__raw_readl(r.add(4));d&=!(1<<o);if val!=0{d|=1<<o}__raw_writel(d,r.add(4));let mut v=__raw_readl(r);v&=!(3<<(o*2));v|=1<<(o*2);__raw_writel(v,r);__raw_writel(d,r.add(4));samsung_gpio_unlock(c,f);0 }
unsafe fn set_gpio(ch:*mut gpio_chip,o:u32,val:i32)->i32 {let c=ch as *mut samsung_gpio_chip;let mut f=0;samsung_gpio_lock(c,&mut f);let r=(*c).base.add(4);let mut d=__raw_readl(r);d&=!(1<<o);if val!=0{d|=1<<o}__raw_writel(d,r);samsung_gpio_unlock(c,f);0}
unsafe fn get_gpio(ch:*mut gpio_chip,o:u32)->i32 {((__raw_readl((*(ch as *mut samsung_gpio_chip)).base.add(4))>>o)&1) as i32}

// The remaining registration tables and SoC-specific initialization retain the
// original interfaces; their definitions are supplied by the surrounding port.
pub unsafe fn samsung_gpiolib_to_irq(ch:*mut gpio_chip,o:u32)->i32 { (*gpiochip_get_data(ch)).irq_base + o as i32 }
pub unsafe fn s3c64xx_gpiolib_mbank_to_irq(_: *mut gpio_chip,p:u32)->i32 { if p<5 { 23+p as i32 } else { ENXIO } }
pub unsafe fn s3c64xx_gpiolib_lbank_to_irq(_: *mut gpio_chip,p:u32)->i32 { if p>=8 { 16+p as i32-8 } else { ENXIO } }
pub unsafe fn s3c_gpio_cfgpin(pin:u32,cfg:u32)->i32 {let c=samsung_gpiolib_getchip(pin);if c.is_null(){return EINVAL}let o=pin-(*c).chip.base;let mut f=0;samsung_gpio_lock(c,&mut f);let r=samsung_gpio_do_setcfg(c,o as i32,cfg);samsung_gpio_unlock(c,f);r}
pub unsafe fn s3c_gpio_cfgpin_range(mut start:u32,mut nr:u32,cfg:u32)->i32 {while nr>0{let r=s3c_gpio_cfgpin(start,cfg);if r!=0{return r}nr-=1;start+=1}0}
pub unsafe fn s3c_gpio_cfgall_range(mut start:u32,mut nr:u32,cfg:u32,p:samsung_gpio_pull_t)->i32 {while nr>0{ s3c_gpio_setpull(start,p);let r=s3c_gpio_cfgpin(start,cfg);if r!=0{return r}nr-=1;start+=1}0}

// Configuration classes used by the original indexed initializer.
#[no_mangle] pub static mut samsung_gpio_cfgs:[samsung_gpio_cfg;8]=[
 samsung_gpio_cfg{cfg_eint:0,set_config:None,get_config:None,set_pull:None,get_pull:None},
 samsung_gpio_cfg{cfg_eint:3,set_config:None,get_config:None,set_pull:None,get_pull:None},
 samsung_gpio_cfg{cfg_eint:7,set_config:None,get_config:None,set_pull:None,get_pull:None},
 samsung_gpio_cfg{cfg_eint:15,set_config:None,get_config:None,set_pull:None,get_pull:None},
 samsung_gpio_cfg{cfg_eint:0,set_config:Some(setcfg2),get_config:Some(getcfg2),set_pull:None,get_pull:None},
 samsung_gpio_cfg{cfg_eint:2,set_config:Some(setcfg2),get_config:Some(getcfg2),set_pull:None,get_pull:None},
 samsung_gpio_cfg{cfg_eint:3,set_config:Some(setcfg2),get_config:Some(getcfg2),set_pull:None,get_pull:None},
 samsung_gpio_cfg{cfg_eint:0,set_config:Some(setcfg2),get_config:Some(getcfg2),set_pull:None,get_pull:None},
];

// CONFIG_S3C_GPIO_TRACK is a build-time option in the C source.
#[cfg(feature="CONFIG_S3C_GPIO_TRACK")]
pub static mut s3c_gpios:*mut samsung_gpio_chip = core::ptr::null_mut();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
