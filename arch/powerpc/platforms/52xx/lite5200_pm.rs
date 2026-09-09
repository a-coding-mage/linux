// SPDX-License-Identifier: GPL-2.0
// C headers are supplied by the surrounding kernel translation unit.

use core::ffi::c_void;

extern "C" {
    fn lite5200_low_power(sram: *mut c_void, mbar: *mut c_void);
    fn mpc52xx_pm_prepare() -> i32;
    fn mpc52xx_pm_enter(state: suspend_state_t) -> i32;
    fn mpc52xx_pm_finish();
    fn suspend_set_ops(ops: *const platform_suspend_ops);
    fn ioremap(start: usize, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn of_find_matching_node(from: *mut device_node, ids: *const of_device_id) -> *mut device_node;
    fn of_address_to_resource(np: *mut device_node, index: i32, res: *mut resource) -> i32;
    fn of_node_put(np: *mut device_node);
    fn printk(fmt: *const u8, ...);
    fn enable_kernel_fp();
    fn memcpy_fromio(dst: *mut c_void, src: *const c_void, n: usize);
    fn memcpy_toio(dst: *mut c_void, src: *const c_void, n: usize);
    fn out_be32(addr: *mut u32, value: u32);
    fn out_be16(addr: *mut u16, value: u16);
    fn out_8(addr: *mut u8, value: u8);
}

type suspend_state_t = i32;

#[repr(C)] pub struct mpc52xx_cdm { pub ipb_clk_sel: u8, pub pci_clk_sel: u8, pub ext_48mhz_en: u8, pub fd_enable: u8, pub fd_counters: u16, pub clk_enables: u32, pub osc_disable: u8, pub mclken_div_psc1: u16, pub mclken_div_psc2: u16, pub mclken_div_psc3: u16, pub mclken_div_psc6: u16 }
#[repr(C)] pub struct mpc52xx_intr { pub per_pri1:u32, pub per_pri2:u32, pub per_pri3:u32, pub main_pri1:u32, pub main_pri2:u32, pub enc_status:u32, pub per_mask:u32, pub main_mask:u32, pub ctrl:u32 }
#[repr(C)] pub struct mpc52xx_sdma { pub taskBar:u32, pub currentPointer:u32, pub endPointer:u32, pub variablePointer:u32, pub IntVect1:u8, pub IntVect2:u8, pub PtdCntrl:u16, pub ipr:[u8;32], pub cReqSelect:u32, pub task_size0:u32, pub task_size1:u32, pub MDEDebug:u32, pub ADSDebug:u32, pub Value1:u32, pub Value2:u32, pub Control:u32, pub Status:u32, pub PTDDebug:u32, pub tcr:[u16;16], pub IntPend:u32, pub IntMask:u32 }
#[repr(C)] pub struct mpc52xx_xlb { pub snoop_window:u32, pub master_priority:u32, pub master_pri_enable:u32, pub int_enable:u32, pub config:u32 }
#[repr(C)] pub struct mpc52xx_gpio { _private: [u8; 0] }
#[repr(C)] pub struct mpc52xx_gpio_wkup { _private: [u8; 0] }
pub struct device_node;
#[repr(C)] pub struct resource { pub start: usize }
#[repr(C)] pub struct of_device_id { pub compatible: *const u8, pub type_: *const u8 }
#[repr(C)] pub struct platform_suspend_ops { pub valid: Option<unsafe extern "C" fn(suspend_state_t)->i32>, pub begin: Option<unsafe extern "C" fn(suspend_state_t)->i32>, pub prepare: Option<unsafe extern "C" fn()->i32>, pub enter: Option<unsafe extern "C" fn(suspend_state_t)->i32>, pub finish: Option<unsafe extern "C" fn()>, pub end: Option<unsafe extern "C" fn()> }

const PM_SUSPEND_ON: suspend_state_t = 0;
const PM_SUSPEND_STANDBY: suspend_state_t = 1;
const PM_SUSPEND_MEM: suspend_state_t = 3;
const EINVAL: i32 = 22;
const ENOSYS: i32 = 38;

static mut cdm: *mut mpc52xx_cdm = core::ptr::null_mut();
static mut pic: *mut mpc52xx_intr = core::ptr::null_mut();
static mut bes: *mut mpc52xx_sdma = core::ptr::null_mut();
static mut xlb: *mut mpc52xx_xlb = core::ptr::null_mut();
static mut gps: *mut mpc52xx_gpio = core::ptr::null_mut();
static mut gpw: *mut mpc52xx_gpio_wkup = core::ptr::null_mut();
static mut pci: *mut c_void = core::ptr::null_mut();
static mut sram: *mut c_void = core::ptr::null_mut();
static mut mbar: *mut c_void = core::ptr::null_mut();
static mut lite5200_pm_target_state: suspend_state_t = PM_SUSPEND_ON;
static mut scdm: mpc52xx_cdm = unsafe { core::mem::zeroed() };
static mut spic: mpc52xx_intr = unsafe { core::mem::zeroed() };
static mut sbes: mpc52xx_sdma = unsafe { core::mem::zeroed() };
static mut sxlb: mpc52xx_xlb = unsafe { core::mem::zeroed() };
static mut sgps: mpc52xx_gpio = mpc52xx_gpio { _private: [] };
static mut sgpw: mpc52xx_gpio_wkup = mpc52xx_gpio_wkup { _private: [] };
static mut spci: [u8; 0x200] = [0; 0x200];
extern "C" { static mut saved_sram: [u8; 0x4000]; }

unsafe extern "C" fn lite5200_pm_valid(state: suspend_state_t) -> i32 { if state == PM_SUSPEND_STANDBY || state == PM_SUSPEND_MEM { 1 } else { 0 } }
unsafe extern "C" fn lite5200_pm_begin(state: suspend_state_t) -> i32 { if lite5200_pm_valid(state) != 0 { lite5200_pm_target_state = state; 0 } else { -EINVAL } }

unsafe extern "C" fn lite5200_pm_prepare() -> i32 {
    if lite5200_pm_target_state == PM_SUSPEND_STANDBY { return mpc52xx_pm_prepare(); }
    if lite5200_pm_target_state != PM_SUSPEND_MEM { return -EINVAL; }
    let ids = [of_device_id { compatible: b"fsl,mpc5200-immr\0".as_ptr(), type_: core::ptr::null() }, of_device_id { compatible: b"fsl,mpc5200b-immr\0".as_ptr(), type_: core::ptr::null() }, of_device_id { compatible: b"mpc5200\0".as_ptr(), type_: b"soc\0".as_ptr() }, of_device_id { compatible: b"mpc5200\0".as_ptr(), type_: b"builtin\0".as_ptr() }, of_device_id { compatible: core::ptr::null(), type_: core::ptr::null() }];
    let np = of_find_matching_node(core::ptr::null_mut(), ids.as_ptr()); let mut res = resource { start: 0 }; of_address_to_resource(np, 0, &mut res); of_node_put(np);
    mbar = ioremap(res.start, 0xC000); if mbar.is_null() { return -ENOSYS; }
    cdm = mbar.add(0x200) as *mut _; pic = mbar.add(0x500) as *mut _; gps = mbar.add(0xb00) as *mut _; gpw = mbar.add(0xc00) as *mut _; pci = mbar.add(0xd00); bes = mbar.add(0x1200) as *mut _; xlb = mbar.add(0x1f00) as *mut _; sram = mbar.add(0x8000); 0
}

unsafe extern "C" fn lite5200_save_regs() { memcpy_fromio(&mut spic as *mut _ as _, pic as _, core::mem::size_of::<mpc52xx_intr>()); memcpy_fromio(&mut sbes as *mut _ as _, bes as _, core::mem::size_of::<mpc52xx_sdma>()); memcpy_fromio(&mut scdm as *mut _ as _, cdm as _, core::mem::size_of::<mpc52xx_cdm>()); memcpy_fromio(&mut sxlb as *mut _ as _, xlb as _, core::mem::size_of::<mpc52xx_xlb>()); memcpy_fromio(&mut sgps as *mut _ as _, gps as _, core::mem::size_of::<mpc52xx_gpio>()); memcpy_fromio(&mut sgpw as *mut _ as _, gpw as _, core::mem::size_of::<mpc52xx_gpio_wkup>()); memcpy_fromio(spci.as_mut_ptr() as _, pci, 0x200); memcpy_fromio(saved_sram.as_mut_ptr() as _, sram, 0x4000); }

unsafe extern "C" fn lite5200_pm_enter(state: suspend_state_t) -> i32 { if state == PM_SUSPEND_STANDBY { return mpc52xx_pm_enter(state); } lite5200_save_regs(); enable_kernel_fp(); lite5200_low_power(sram, mbar); iounmap(mbar); 0 }
unsafe extern "C" fn lite5200_pm_finish() { if lite5200_pm_target_state == PM_SUSPEND_STANDBY { mpc52xx_pm_finish(); } }
unsafe extern "C" fn lite5200_pm_end() { lite5200_pm_target_state = PM_SUSPEND_ON; }

static lite5200_pm_ops: platform_suspend_ops = platform_suspend_ops { valid: Some(lite5200_pm_valid), begin: Some(lite5200_pm_begin), prepare: Some(lite5200_pm_prepare), enter: Some(lite5200_pm_enter), finish: Some(lite5200_pm_finish), end: Some(lite5200_pm_end) };

pub unsafe extern "C" fn lite5200_pm_init() -> i32 { suspend_set_ops(&lite5200_pm_ops); 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
