/*
 * Utility functions for the Freescale MPC52xx.
 *
 * Copyright (C) 2006 Sylvain Munaut <tnt@246tNt.com>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

use core::ptr;

// Kernel and architecture declarations supplied by the surrounding tree.
extern "C" {
    fn of_find_matching_node(from: *mut device_node, ids: *const of_device_id) -> *mut device_node;
    fn of_iomap(np: *mut device_node, index: i32) -> *mut core::ffi::c_void;
    fn of_node_put(np: *mut device_node);
    fn of_property_read_bool(np: *mut device_node, name: *const i8) -> bool;
    fn of_platform_populate(a: *mut device_node, ids: *const of_device_id, b: *mut core::ffi::c_void, c: *mut core::ffi::c_void) -> i32;
    fn iounmap(addr: *mut core::ffi::c_void);
    fn mfspr(reg: u32) -> u32;
    fn local_irq_disable();
    fn udelay(usecs: u32);
    fn __delay(value: u32);
    fn printk(fmt: *const i8, ...);
    fn pr_err(fmt: *const i8, ...);
    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: usize);
    fn in_be16(addr: *const u16) -> u16;
    fn in_be32(addr: *const u32) -> u32;
    fn out_be16(addr: *mut u16, value: u16);
    fn out_be32(addr: *mut u32, value: u32);
    fn setbits8(addr: *mut u8, value: u8);
    fn setbits32(addr: *mut u32, value: u32);
    fn clrbits8(addr: *mut u8, value: u8);
    fn clrbits32(addr: *mut u32, value: u32);
}

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct of_device_id { pub name: *const i8, pub type_: *const i8, pub compatible: *const i8, pub data: *const core::ffi::c_void }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mpc52xx_xlb { pub master_pri_enable: u32, pub master_priority: u32, pub config: u32 }
#[repr(C)] pub struct mpc52xx_gpt { pub mode: u32, pub count: u32 }
#[repr(C)] pub struct mpc52xx_cdm { pub mclken_div_psc1: u16, pub mclken_div_psc2: u16, pub mclken_div_psc3: u16, pub mclken_div_psc6: u16, pub clk_enables: u32 }
#[repr(C)] pub struct mpc52xx_gpio { pub port_config: u32, pub simple_gpioe: u32, pub simple_ddr: u32, pub simple_dvo: u32 }
#[repr(C)] pub struct mpc52xx_gpio_wkup { pub wkup_gpioe: u8, pub wkup_ddr: u8, pub wkup_dvo: u8 }

static mut MPC52XX_LOCK: spinlock_t = spinlock_t { _private: [] };
static mut GPIO_LOCK: spinlock_t = spinlock_t { _private: [] };
static mut MPC52XX_WDT: *mut mpc52xx_gpt = ptr::null_mut();
static mut MPC52XX_CDM: *mut mpc52xx_cdm = ptr::null_mut();
#[no_mangle] pub static mut simple_gpio: *mut mpc52xx_gpio = ptr::null_mut();
#[no_mangle] pub static mut wkup_gpio: *mut mpc52xx_gpio_wkup = ptr::null_mut();

const SPRN_SVR: u32 = 287;
const MPC5200_SVR_MASK: u32 = 0xffff_ff00;
const MPC5200_SVR: u32 = 0x8012_0000;
const MPC52XX_XLB_CFG_PLDIS: u32 = 0x0000_0001;

static MPC52XX_XLB_IDS: [of_device_id; 3] = [of_device_id { name: ptr::null(), type_: ptr::null(), compatible: b"fsl,mpc5200-xlb\0".as_ptr() as _, data: ptr::null() }, of_device_id { name: ptr::null(), type_: ptr::null(), compatible: b"mpc5200-xlb\0".as_ptr() as _, data: ptr::null() }, of_device_id { name: ptr::null(), type_: ptr::null(), compatible: ptr::null(), data: ptr::null() }];

#[no_mangle]
pub unsafe extern "C" fn mpc5200_setup_xlb_arbiter() {
    let np = of_find_matching_node(ptr::null_mut(), MPC52XX_XLB_IDS.as_ptr());
    let xlb = of_iomap(np, 0) as *mut mpc52xx_xlb;
    of_node_put(np);
    if xlb.is_null() { printk(b"Error mapping XLB in mpc52xx_setup_cpu(). Expect some abnormal behavior\n\0".as_ptr() as _); return; }
    out_be32(&mut (*xlb).master_pri_enable, 0xff);
    out_be32(&mut (*xlb).master_priority, 0x11111111);
    if (mfspr(SPRN_SVR) & MPC5200_SVR_MASK) == MPC5200_SVR { out_be32(&mut (*xlb).config, in_be32(&(*xlb).config) | MPC52XX_XLB_CFG_PLDIS); }
    iounmap(xlb as _);
}

#[no_mangle] pub unsafe extern "C" fn mpc52xx_declare_of_platform_devices() { if of_platform_populate(ptr::null_mut(), ptr::null(), ptr::null_mut(), ptr::null_mut()) != 0 { pr_err(b"Error while populating devices from DT\n\0".as_ptr() as _); } }

#[no_mangle] pub unsafe extern "C" fn mpc52xx_map_common_devices() {
    // The C match tables are retained here as local compatible-only tables.
    static GPT: [of_device_id; 3] = [of_device_id { name: ptr::null(), type_: ptr::null(), compatible: b"fsl,mpc5200-gpt\0".as_ptr() as _, data: ptr::null() }, of_device_id { name: ptr::null(), type_: ptr::null(), compatible: b"mpc5200-gpt\0".as_ptr() as _, data: ptr::null() }, of_device_id { name: ptr::null(), type_: ptr::null(), compatible: ptr::null(), data: ptr::null() }];
    static CDM: [of_device_id; 3] = [of_device_id { name: ptr::null(), type_: ptr::null(), compatible: b"fsl,mpc5200-cdm\0".as_ptr() as _, data: ptr::null() }, of_device_id { name: ptr::null(), type_: ptr::null(), compatible: b"mpc5200-cdm\0".as_ptr() as _, data: ptr::null() }, of_device_id { name: ptr::null(), type_: ptr::null(), compatible: ptr::null(), data: ptr::null() }];
    static GPIO: [of_device_id; 2] = [of_device_id { name: ptr::null(), type_: ptr::null(), compatible: b"fsl,mpc5200-gpio\0".as_ptr() as _, data: ptr::null() }, of_device_id { name: ptr::null(), type_: ptr::null(), compatible: ptr::null(), data: ptr::null() }];
    static WKUP: [of_device_id; 2] = [of_device_id { name: ptr::null(), type_: ptr::null(), compatible: b"fsl,mpc5200-gpio-wkup\0".as_ptr() as _, data: ptr::null() }, of_device_id { name: ptr::null(), type_: ptr::null(), compatible: ptr::null(), data: ptr::null() }];
    let mut np = of_find_matching_node(ptr::null_mut(), GPT.as_ptr());
    while !np.is_null() { if of_property_read_bool(np, b"fsl,has-wdt\0".as_ptr() as _) || of_property_read_bool(np, b"has-wdt\0".as_ptr() as _) { MPC52XX_WDT = of_iomap(np, 0) as _; of_node_put(np); break; } of_node_put(np); np = ptr::null_mut(); }
    np = of_find_matching_node(ptr::null_mut(), CDM.as_ptr()); MPC52XX_CDM = of_iomap(np, 0) as _; of_node_put(np);
    np = of_find_matching_node(ptr::null_mut(), GPIO.as_ptr()); simple_gpio = of_iomap(np, 0) as _; of_node_put(np);
    np = of_find_matching_node(ptr::null_mut(), WKUP.as_ptr()); wkup_gpio = of_iomap(np, 0) as _; of_node_put(np);
}

#[no_mangle]
pub unsafe extern "C" fn mpc52xx_set_psc_clkdiv(psc_id: i32, clkdiv: i32) -> i32 {
    if MPC52XX_CDM.is_null() { return -19; }
    let (reg, mask) = match psc_id { 1 => (&mut (*MPC52XX_CDM).mclken_div_psc1, 0x20), 2 => (&mut (*MPC52XX_CDM).mclken_div_psc2, 0x40), 3 => (&mut (*MPC52XX_CDM).mclken_div_psc3, 0x80), 6 => (&mut (*MPC52XX_CDM).mclken_div_psc6, 0x10), _ => return -19 };
    let mut flags = 0; spin_lock_irqsave(&mut MPC52XX_LOCK, &mut flags); out_be16(reg, (0x8000 | (clkdiv & 0x1ff)) as u16); let val = in_be32(&(*MPC52XX_CDM).clk_enables); out_be32(&mut (*MPC52XX_CDM).clk_enables, val | mask); spin_unlock_irqrestore(&mut MPC52XX_LOCK, flags); 0
}

#[no_mangle] pub unsafe extern "C" fn mpc52xx_restart(_cmd: *mut i8) -> ! { local_irq_disable(); if !MPC52XX_WDT.is_null() { out_be32(&mut (*MPC52XX_WDT).mode, 0); out_be32(&mut (*MPC52XX_WDT).count, 0xff); out_be32(&mut (*MPC52XX_WDT).mode, 0x9004); } else { printk(b"mpc52xx_restart: Can't access wdt. Restart impossible, system halted.\n\0".as_ptr() as _); } loop {} }

const PSC1_RESET: i32 = 1; const PSC1_SYNC: u32 = 4; const PSC1_SDATA_OUT: u32 = 1; const PSC2_RESET: i32 = 2; const PSC2_SYNC: u32 = 0x40; const PSC2_SDATA_OUT: u32 = 0x10;

#[no_mangle] pub unsafe extern "C" fn mpc5200_psc_ac97_gpio_reset(psc_number: i32) -> i32 {
    if simple_gpio.is_null() || wkup_gpio.is_null() { return -19; }
    let (reset, sync, out, gpio) = match psc_number { 0 => (PSC1_RESET, PSC1_SYNC, PSC1_SDATA_OUT, 7), 1 => (PSC2_RESET, PSC2_SYNC, PSC2_SDATA_OUT, 0x70), _ => return -19 };
    let mut flags = 0; spin_lock_irqsave(&mut GPIO_LOCK, &mut flags); let mux = in_be32(&(*simple_gpio).port_config); out_be32(&mut (*simple_gpio).port_config, mux & !gpio); setbits8(&mut (*wkup_gpio).wkup_gpioe, reset as u8); setbits32(&mut (*simple_gpio).simple_gpioe, sync | out); setbits8(&mut (*wkup_gpio).wkup_ddr, reset as u8); setbits32(&mut (*simple_gpio).simple_ddr, sync | out); clrbits32(&mut (*simple_gpio).simple_dvo, sync | out); clrbits8(&mut (*wkup_gpio).wkup_dvo, reset as u8); udelay(1); setbits8(&mut (*wkup_gpio).wkup_dvo, reset as u8); __delay(7); out_be32(&mut (*simple_gpio).port_config, mux); spin_unlock_irqrestore(&mut GPIO_LOCK, flags); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
