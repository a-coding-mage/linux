/*
 * Prototypes, etc. for the Freescale MPC52xx embedded cpu chips
 * May need to be cleaned as the port goes on ...
 *
 * Copyright (C) 2004-2005 Sylvain Munaut <tnt@246tNt.com>
 * Copyright (C) 2003 MontaVista, Software, Inc.
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2. This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 */

// C dependencies: asm/types.h, asm/mpc5xxx.h, and linux/suspend.h.

pub const MPC5200_SVR: u32 = 0x80110010;
pub const MPC5200_SVR_MASK: u32 = 0xfffffff0;
pub const MPC5200B_SVR: u32 = 0x80110020;
pub const MPC5200B_SVR_MASK: u32 = 0xfffffff0;

#[repr(C)]
pub struct mpc52xx_mmap_ctl {
    pub mbar: u32, pub cs0_start: u32, pub cs0_stop: u32, pub cs1_start: u32, pub cs1_stop: u32,
    pub cs2_start: u32, pub cs2_stop: u32, pub cs3_start: u32, pub cs3_stop: u32,
    pub cs4_start: u32, pub cs4_stop: u32, pub cs5_start: u32, pub cs5_stop: u32,
    pub sdram0: u32, pub sdram1: u32, pub reserved: [u32; 4], pub boot_start: u32, pub boot_stop: u32,
    pub ipbi_ws_ctrl: u32, pub cs6_start: u32, pub cs6_stop: u32, pub cs7_start: u32, pub cs7_stop: u32,
}

#[repr(C)] pub struct mpc52xx_sdram { pub mode: u32, pub ctrl: u32, pub config1: u32, pub config2: u32 }

#[repr(C)]
pub struct mpc52xx_sdma {
    pub taskBar: u32, pub currentPointer: u32, pub endPointer: u32, pub variablePointer: u32,
    pub IntVect1: u8, pub IntVect2: u8, pub PtdCntrl: u16, pub IntPend: u32, pub IntMask: u32,
    pub tcr: [u16; 16], pub ipr: [u8; 32], pub cReqSelect: u32, pub task_size0: u32, pub task_size1: u32,
    pub MDEDebug: u32, pub ADSDebug: u32, pub Value1: u32, pub Value2: u32, pub Control: u32,
    pub Status: u32, pub PTDDebug: u32,
}

#[repr(C)] pub struct mpc52xx_gpt { pub mode: u32, pub count: u32, pub pwm: u32, pub status: u32 }

#[repr(C)]
pub struct mpc52xx_gpio {
    pub port_config: u32, pub simple_gpioe: u32, pub simple_ode: u32, pub simple_ddr: u32,
    pub simple_dvo: u32, pub simple_ival: u32, pub outo_gpioe: u8, pub reserved1: [u8; 3],
    pub outo_dvo: u8, pub reserved2: [u8; 3], pub sint_gpioe: u8, pub reserved3: [u8; 3],
    pub sint_ode: u8, pub reserved4: [u8; 3], pub sint_ddr: u8, pub reserved5: [u8; 3],
    pub sint_dvo: u8, pub reserved6: [u8; 3], pub sint_inten: u8, pub reserved7: [u8; 3],
    pub sint_itype: u16, pub reserved8: u16, pub gpio_control: u8, pub reserved9: [u8; 3],
    pub sint_istat: u8, pub sint_ival: u8, pub bus_errs: u8, pub reserved10: u8,
}
pub const MPC52xx_GPIO_PSC_CONFIG_UART_WITHOUT_CD: u32 = 4;
pub const MPC52xx_GPIO_PSC_CONFIG_UART_WITH_CD: u32 = 5;
pub const MPC52xx_GPIO_PCI_DIS: u32 = 1 << 15;

#[repr(C)]
pub struct mpc52xx_gpio_wkup {
    pub wkup_gpioe: u8, pub reserved1: [u8;3], pub wkup_ode: u8, pub reserved2: [u8;3],
    pub wkup_ddr: u8, pub reserved3: [u8;3], pub wkup_dvo: u8, pub reserved4: [u8;3],
    pub wkup_inten: u8, pub reserved5: [u8;3], pub wkup_iinten: u8, pub reserved6: [u8;3],
    pub wkup_itype: u16, pub reserved7: [u8;2], pub wkup_maste: u8, pub reserved8: [u8;3],
    pub wkup_ival: u8, pub reserved9: [u8;3], pub wkup_istat: u8, pub reserved10: [u8;3],
}

#[repr(C)] pub struct mpc52xx_xlb {
    pub reserved: [u8; 0x40], pub config: u32, pub version: u32, pub status: u32, pub int_enable: u32,
    pub addr_capture: u32, pub bus_sig_capture: u32, pub addr_timeout: u32, pub data_timeout: u32,
    pub bus_act_timeout: u32, pub master_pri_enable: u32, pub master_priority: u32,
    pub base_address: u32, pub snoop_window: u32,
}
pub const MPC52xx_XLB_CFG_PLDIS: u32 = 1 << 31;
pub const MPC52xx_XLB_CFG_SNOOP: u32 = 1 << 15;

#[repr(C)]
pub struct mpc52xx_cdm {
    pub jtag_id:u32, pub rstcfg:u32, pub breadcrumb:u32, pub mem_clk_sel:u8, pub xlb_clk_sel:u8,
    pub ipb_clk_sel:u8, pub pci_clk_sel:u8, pub ext_48mhz_en:u8, pub fd_enable:u8, pub fd_counters:u16,
    pub clk_enables:u32, pub osc_disable:u8, pub reserved0:[u8;3], pub ccs_sleep_enable:u8,
    pub osc_sleep_enable:u8, pub reserved1:u8, pub ccs_qreq_test:u8, pub soft_reset:u8, pub no_ckstp:u8,
    pub reserved2:[u8;2], pub pll_lock:u8, pub pll_looselock:u8, pub pll_sm_lockwin:u8, pub reserved3:u8,
    pub reserved4:u16, pub mclken_div_psc1:u16, pub reserved5:u16, pub mclken_div_psc2:u16,
    pub reserved6:u16, pub mclken_div_psc3:u16, pub reserved7:u16, pub mclken_div_psc6:u16,
}

#[repr(C)] pub struct mpc52xx_intr {
    pub per_mask:u32, pub per_pri1:u32, pub per_pri2:u32, pub per_pri3:u32, pub ctrl:u32,
    pub main_mask:u32, pub main_pri1:u32, pub main_pri2:u32, pub reserved1:u32, pub enc_status:u32,
    pub crit_status:u32, pub main_status:u32, pub per_status:u32, pub reserved2:u32, pub per_error:u32,
}

pub enum device_node {}
pub enum mpc52xx_gpt_priv {}

extern "C" {
    pub fn mpc5200_setup_xlb_arbiter();
    pub fn mpc52xx_declare_of_platform_devices();
    pub fn mpc5200_psc_ac97_gpio_reset(psc_number: i32) -> i32;
    pub fn mpc52xx_map_common_devices();
    pub fn mpc52xx_set_psc_clkdiv(psc_id: i32, clkdiv: i32) -> i32;
    pub fn mpc52xx_restart(cmd: *mut i8) -> !;
    pub fn mpc52xx_gpt_from_irq(irq: i32) -> *mut mpc52xx_gpt_priv;
    pub fn mpc52xx_gpt_start_timer(gpt: *mut mpc52xx_gpt_priv, period: u64, continuous: i32) -> i32;
    pub fn mpc52xx_gpt_timer_period(gpt: *mut mpc52xx_gpt_priv) -> u64;
    pub fn mpc52xx_gpt_stop_timer(gpt: *mut mpc52xx_gpt_priv) -> i32;
    pub fn mpc52xx_init_irq();
    pub fn mpc52xx_get_irq() -> u32;
    pub fn mpc52xx_setup_pci();
    pub static mut mpc52xx_suspend: mpc52xx_suspend;
    pub fn mpc52xx_pm_init() -> i32;
    pub fn mpc52xx_set_wakeup_gpio(pin: u8, level: u8) -> i32;
    pub fn mpc52xx_pm_prepare() -> i32;
    pub fn mpc52xx_pm_enter(state: suspend_state_t) -> i32;
    pub fn mpc52xx_pm_finish();
    pub static mut saved_sram: [i8; 0x4000];
    pub fn lite5200_pm_init() -> i32;
}

pub type suspend_state_t = i32;
#[repr(C)] pub struct mpc52xx_suspend {
    pub board_suspend_prepare: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
    pub board_resume_finish: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
