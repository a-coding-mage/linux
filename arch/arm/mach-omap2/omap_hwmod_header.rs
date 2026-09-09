/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of omap_hwmod.h. C includes and build-system symbols are external dependencies. */

extern "C" {
    pub static mut omap_hwmod_sysc_type1: sysc_regbits;
    pub static mut omap_hwmod_sysc_type2: sysc_regbits;
    pub static mut omap_hwmod_sysc_type3: sysc_regbits;
    pub static mut omap34xx_sr_sysc_fields: sysc_regbits;
    pub static mut omap36xx_sr_sysc_fields: sysc_regbits;
    pub static mut omap3_sham_sysc_fields: sysc_regbits;
    pub static mut omap3xxx_aes_sysc_fields: sysc_regbits;
    pub static mut omap_hwmod_sysc_type_mcasp: sysc_regbits;
    pub static mut omap_hwmod_sysc_type_usb_host_fs: sysc_regbits;
}

pub const SYSC_TYPE1_MIDLEMODE_SHIFT: u32 = 12;
pub const SYSC_TYPE1_MIDLEMODE_MASK: u32 = 0x3 << SYSC_TYPE1_MIDLEMODE_SHIFT;
pub const SYSC_TYPE1_CLOCKACTIVITY_SHIFT: u32 = 8;
pub const SYSC_TYPE1_CLOCKACTIVITY_MASK: u32 = 0x3 << SYSC_TYPE1_CLOCKACTIVITY_SHIFT;
pub const SYSC_TYPE1_SIDLEMODE_SHIFT: u32 = 3;
pub const SYSC_TYPE1_SIDLEMODE_MASK: u32 = 0x3 << SYSC_TYPE1_SIDLEMODE_SHIFT;
pub const SYSC_TYPE1_ENAWAKEUP_SHIFT: u32 = 2;
pub const SYSC_TYPE1_ENAWAKEUP_MASK: u32 = 1 << SYSC_TYPE1_ENAWAKEUP_SHIFT;
pub const SYSC_TYPE1_SOFTRESET_SHIFT: u32 = 1;
pub const SYSC_TYPE1_SOFTRESET_MASK: u32 = 1 << SYSC_TYPE1_SOFTRESET_SHIFT;
pub const SYSC_TYPE1_AUTOIDLE_SHIFT: u32 = 0;
pub const SYSC_TYPE1_AUTOIDLE_MASK: u32 = 1 << SYSC_TYPE1_AUTOIDLE_SHIFT;
pub const SYSC_TYPE2_SOFTRESET_SHIFT: u32 = 0;
pub const SYSC_TYPE2_SOFTRESET_MASK: u32 = 1 << SYSC_TYPE2_SOFTRESET_SHIFT;
pub const SYSC_TYPE2_SIDLEMODE_SHIFT: u32 = 2;
pub const SYSC_TYPE2_SIDLEMODE_MASK: u32 = 0x3 << SYSC_TYPE2_SIDLEMODE_SHIFT;
pub const SYSC_TYPE2_MIDLEMODE_SHIFT: u32 = 4;
pub const SYSC_TYPE2_MIDLEMODE_MASK: u32 = 0x3 << SYSC_TYPE2_MIDLEMODE_SHIFT;
pub const SYSC_TYPE2_DMADISABLE_SHIFT: u32 = 16;
pub const SYSC_TYPE2_DMADISABLE_MASK: u32 = 1 << SYSC_TYPE2_DMADISABLE_SHIFT;
pub const SYSC_TYPE3_SIDLEMODE_SHIFT: u32 = 0;
pub const SYSC_TYPE3_SIDLEMODE_MASK: u32 = 0x3 << SYSC_TYPE3_SIDLEMODE_SHIFT;
pub const SYSC_TYPE3_MIDLEMODE_SHIFT: u32 = 2;
pub const SYSC_TYPE3_MIDLEMODE_MASK: u32 = 0x3 << SYSC_TYPE3_MIDLEMODE_SHIFT;
pub const SYSS_RESETDONE_SHIFT: u32 = 0;
pub const SYSS_RESETDONE_MASK: u32 = 1 << SYSS_RESETDONE_SHIFT;
pub const HWMOD_IDLEMODE_FORCE: u32 = 1 << 0;
pub const HWMOD_IDLEMODE_NO: u32 = 1 << 1;
pub const HWMOD_IDLEMODE_SMART: u32 = 1 << 2;
pub const HWMOD_IDLEMODE_SMART_WKUP: u32 = 1 << 3;
pub const MODULEMODE_HWCTRL: u32 = 1;
pub const MODULEMODE_SWCTRL: u32 = 2;

pub const DEBUG_OMAP2UART1_FLAGS: u32 = 0;
pub const DEBUG_OMAP2UART2_FLAGS: u32 = 0;
pub const DEBUG_OMAP2UART3_FLAGS: u32 = 0;
pub const DEBUG_OMAP3UART3_FLAGS: u32 = 0;
pub const DEBUG_OMAP3UART4_FLAGS: u32 = 0;
pub const DEBUG_OMAP4UART3_FLAGS: u32 = 0;
pub const DEBUG_OMAP4UART4_FLAGS: u32 = 0;
pub const DEBUG_TI81XXUART1_FLAGS: u32 = 0;
pub const DEBUG_TI81XXUART2_FLAGS: u32 = 0;
pub const DEBUG_TI81XXUART3_FLAGS: u32 = 0;
pub const DEBUG_AM33XXUART1_FLAGS: u32 = 0;
pub const DEBUG_OMAPUART_FLAGS: u32 = HWMOD_INIT_NO_IDLE | HWMOD_INIT_NO_RESET;
// CONFIG_OMAP_GPMC_DEBUG selects HWMOD_INIT_NO_RESET; otherwise this is zero.
#[cfg(CONFIG_OMAP_GPMC_DEBUG)]
pub const DEBUG_OMAP_GPMC_HWMOD_FLAGS: u32 = HWMOD_INIT_NO_RESET;
#[cfg(not(CONFIG_OMAP_GPMC_DEBUG))]
pub const DEBUG_OMAP_GPMC_HWMOD_FLAGS: u32 = 0;

#[repr(C)]
pub struct omap_hwmod_rst_info { pub name: *const core::ffi::c_char, pub rst_shift: u8, pub st_shift: u8 }
#[repr(C)]
pub struct omap_hwmod_opt_clk { pub role: *const core::ffi::c_char, pub clk: *const core::ffi::c_char, pub _clk: *mut clk }
pub const OMAP_FIREWALL_L3: u8 = 1 << 0;
pub const OMAP_FIREWALL_L4: u8 = 1 << 1;
#[repr(C)]
pub struct omap_hwmod_omap2_firewall { pub l3_perm_bit: u8, pub l4_fw_region: u8, pub l4_prot_group: u8, pub flags: u8 }
pub const OCP_USER_MPU: u8 = 1 << 0;
pub const OCP_USER_SDMA: u8 = 1 << 1;
pub const OCP_USER_DSP: u8 = 1 << 2;
pub const OCP_USER_IVA: u8 = 1 << 3;
pub const OCPIF_SWSUP_IDLE: u8 = 1 << 0;
pub const OCPIF_CAN_BURST: u8 = 1 << 1;
pub const _OCPIF_INT_FLAGS_REGISTERED: u8 = 1 << 0;
#[repr(C)]
pub union omap_hwmod_ocp_if_fw { pub omap2: omap_hwmod_omap2_firewall }
#[repr(C)]
pub struct omap_hwmod_ocp_if { pub master: *mut omap_hwmod, pub slave: *mut omap_hwmod, pub addr: *mut omap_hwmod_addr_space, pub clk: *const core::ffi::c_char, pub _clk: *mut clk, pub node: list_head, pub fw: omap_hwmod_ocp_if_fw, pub width: u8, pub user: u8, pub flags: u8, pub _int_flags: u8 }

pub const MASTER_STANDBY_SHIFT: u32 = 4;
pub const SLAVE_IDLE_SHIFT: u32 = 0;
pub const SIDLE_FORCE: u32 = HWMOD_IDLEMODE_FORCE << SLAVE_IDLE_SHIFT;
pub const SIDLE_NO: u32 = HWMOD_IDLEMODE_NO << SLAVE_IDLE_SHIFT;
pub const SIDLE_SMART: u32 = HWMOD_IDLEMODE_SMART << SLAVE_IDLE_SHIFT;
pub const SIDLE_SMART_WKUP: u32 = HWMOD_IDLEMODE_SMART_WKUP << SLAVE_IDLE_SHIFT;
pub const MSTANDBY_FORCE: u32 = HWMOD_IDLEMODE_FORCE << MASTER_STANDBY_SHIFT;
pub const MSTANDBY_NO: u32 = HWMOD_IDLEMODE_NO << MASTER_STANDBY_SHIFT;
pub const MSTANDBY_SMART: u32 = HWMOD_IDLEMODE_SMART << MASTER_STANDBY_SHIFT;
pub const MSTANDBY_SMART_WKUP: u32 = HWMOD_IDLEMODE_SMART_WKUP << MASTER_STANDBY_SHIFT;
pub const SYSC_HAS_AUTOIDLE: u16 = 1 << 0; pub const SYSC_HAS_SOFTRESET: u16 = 1 << 1; pub const SYSC_HAS_ENAWAKEUP: u16 = 1 << 2; pub const SYSC_HAS_EMUFREE: u16 = 1 << 3; pub const SYSC_HAS_CLOCKACTIVITY: u16 = 1 << 4; pub const SYSC_HAS_SIDLEMODE: u16 = 1 << 5; pub const SYSC_HAS_MIDLEMODE: u16 = 1 << 6; pub const SYSS_HAS_RESET_STATUS: u16 = 1 << 7; pub const SYSC_NO_CACHE: u16 = 1 << 8; pub const SYSC_HAS_RESET_STATUS: u16 = 1 << 9; pub const SYSC_HAS_DMADISABLE: u16 = 1 << 10;
pub const CLOCKACT_TEST_BOTH: u8 = 0; pub const CLOCKACT_TEST_MAIN: u8 = 1; pub const CLOCKACT_TEST_ICLK: u8 = 2; pub const CLOCKACT_TEST_NONE: u8 = 3;
#[repr(C)] pub struct omap_hwmod_class_sysconfig { pub rev_offs: i32, pub sysc_offs: i32, pub syss_offs: i32, pub sysc_flags: u16, pub sysc_fields: *mut sysc_regbits, pub srst_udelay: u8, pub idlemodes: u8 }
#[repr(C)] pub struct omap_hwmod_omap2_prcm { pub module_offs: i16, pub idlest_reg_id: u8, pub idlest_idle_bit: u8 }
pub const HWMOD_OMAP4_NO_CONTEXT_LOSS_BIT: u8 = 1 << 0; pub const HWMOD_OMAP4_ZERO_CLKCTRL_OFFSET: u8 = 1 << 1; pub const HWMOD_OMAP4_CLKFWK_CLKCTR_CLOCK: u8 = 1 << 2;
#[repr(C)] pub struct omap_hwmod_omap4_prcm { pub clkctrl_offs: u16, pub rstctrl_offs: u16, pub rstst_offs: u16, pub context_offs: u16, pub lostcontext_mask: u32, pub submodule_wkdep_bit: u8, pub modulemode: u8, pub flags: u8, pub context_lost_counter: i32 }

pub const HWMOD_SWSUP_SIDLE: u32=1<<0; pub const HWMOD_SWSUP_MSTANDBY:u32=1<<1; pub const HWMOD_INIT_NO_RESET:u32=1<<2; pub const HWMOD_INIT_NO_IDLE:u32=1<<3; pub const HWMOD_NO_OCP_AUTOIDLE:u32=1<<4; pub const HWMOD_SET_DEFAULT_CLOCKACT:u32=1<<5; pub const HWMOD_NO_IDLEST:u32=1<<6; pub const HWMOD_CONTROL_OPT_CLKS_IN_RESET:u32=1<<7; pub const HWMOD_16BIT_REG:u32=1<<8; pub const HWMOD_EXT_OPT_MAIN_CLK:u32=1<<9; pub const HWMOD_BLOCK_WFI:u32=1<<10; pub const HWMOD_FORCE_MSTANDBY:u32=1<<11; pub const HWMOD_SWSUP_SIDLE_ACT:u32=1<<12; pub const HWMOD_RECONFIG_IO_CHAIN:u32=1<<13; pub const HWMOD_OPT_CLKS_NEEDED:u32=1<<14; pub const HWMOD_NO_IDLE:u32=1<<15; pub const HWMOD_CLKDM_NOAUTO:u32=1<<16;
pub const _HWMOD_NO_MPU_PORT:u32=1<<0; pub const _HWMOD_SYSCONFIG_LOADED:u32=1<<1; pub const _HWMOD_SKIP_ENABLE:u32=1<<2;
pub const _HWMOD_STATE_UNKNOWN:u8=0; pub const _HWMOD_STATE_REGISTERED:u8=1; pub const _HWMOD_STATE_CLKS_INITED:u8=2; pub const _HWMOD_STATE_INITIALIZED:u8=3; pub const _HWMOD_STATE_ENABLED:u8=4; pub const _HWMOD_STATE_IDLE:u8=5; pub const _HWMOD_STATE_DISABLED:u8=6;
#[cfg(CONFIG_PM)] pub const _HWMOD_STATE_DEFAULT:u8=_HWMOD_STATE_IDLE; #[cfg(not(CONFIG_PM))] pub const _HWMOD_STATE_DEFAULT:u8=_HWMOD_STATE_ENABLED;

#[repr(C)] pub struct omap_hwmod_class { pub name:*const core::ffi::c_char, pub sysc:*mut omap_hwmod_class_sysconfig, pub pre_shutdown:Option<unsafe extern "C" fn(*mut omap_hwmod)->i32>, pub reset:Option<unsafe extern "C" fn(*mut omap_hwmod)->i32>, pub lock:Option<unsafe extern "C" fn(*mut omap_hwmod)>, pub unlock:Option<unsafe extern "C" fn(*mut omap_hwmod)> }
#[repr(C)] pub union omap_hwmod_prcm { pub omap2:omap_hwmod_omap2_prcm, pub omap4:omap_hwmod_omap4_prcm }
#[repr(C)] pub struct omap_hwmod { pub name:*const core::ffi::c_char, pub class:*mut omap_hwmod_class, pub od:*mut omap_device, pub rst_lines:*mut omap_hwmod_rst_info, pub prcm:omap_hwmod_prcm, pub main_clk:*const core::ffi::c_char, pub _clk:*mut clk, pub opt_clks:*mut omap_hwmod_opt_clk, pub clkdm_name:*const core::ffi::c_char, pub clkdm:*mut clockdomain, pub slave_ports:list_head, pub dev_attr:*mut core::ffi::c_void, pub _sysc_cache:u32, pub _mpu_rt_va:*mut core::ffi::c_void, pub _lock:spinlock_t, pub hwmod_key:lock_class_key, pub node:list_head, pub _mpu_port:*mut omap_hwmod_ocp_if, pub flags:u32, pub mpu_rt_idx:u8, pub response_lat:u8, pub rst_lines_cnt:u8, pub opt_clks_cnt:u8, pub slaves_cnt:u8, pub hwmods_cnt:u8, pub _int_flags:u8, pub _state:u8, pub _postsetup_state:u8, pub parent_hwmod:*mut omap_hwmod }

// External types supplied by Linux headers.
extern "C" { pub fn omap_hwmod_lookup(name:*const core::ffi::c_char)->*mut omap_hwmod; pub fn omap_hwmod_for_each(fn_:Option<unsafe extern "C" fn(*mut omap_hwmod,*mut core::ffi::c_void)->i32>,data:*mut core::ffi::c_void)->i32; pub fn omap_hwmod_parse_module_range(oh:*mut omap_hwmod,np:*mut device_node,res:*mut resource)->i32; pub fn omap_hwmod_init_module(dev:*mut device,data:*const ti_sysc_module_data,cookie:*mut ti_sysc_cookie)->i32; pub fn omap_hwmod_enable(oh:*mut omap_hwmod)->i32; pub fn omap_hwmod_idle(oh:*mut omap_hwmod)->i32; pub fn omap_hwmod_shutdown(oh:*mut omap_hwmod)->i32; pub fn omap_hwmod_assert_hardreset(oh:*mut omap_hwmod,name:*const core::ffi::c_char)->i32; pub fn omap_hwmod_deassert_hardreset(oh:*mut omap_hwmod,name:*const core::ffi::c_char)->i32; pub fn omap_hwmod_write(v:u32,oh:*mut omap_hwmod,reg_offs:u16); pub fn omap_hwmod_read(oh:*mut omap_hwmod,reg_offs:u16)->u32; pub fn omap_hwmod_softreset(oh:*mut omap_hwmod)->i32; pub fn omap_hwmod_get_mpu_rt_va(oh:*mut omap_hwmod)->*mut core::ffi::c_void; pub fn omap_hwmod_for_each_by_class(classname:*const core::ffi::c_char,fn_:Option<unsafe extern "C" fn(*mut omap_hwmod,*mut core::ffi::c_void)->i32>,user:*mut core::ffi::c_void)->i32; pub fn omap_hwmod_set_postsetup_state(oh:*mut omap_hwmod,state:u8)->i32; pub fn omap_hwmod_init(); pub fn omap2420_hwmod_init()->i32; pub fn omap2430_hwmod_init()->i32; pub fn omap3xxx_hwmod_init()->i32; pub fn dm814x_hwmod_init()->i32; pub fn dm816x_hwmod_init()->i32; pub fn omap_hwmod_register_links(ois:*mut *mut omap_hwmod_ocp_if)->i32; }

#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct resource { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct ti_sysc_module_data { _private: [u8; 0] }
#[repr(C)] pub struct ti_sysc_cookie { _private: [u8; 0] }

#[cfg(not(CONFIG_OMAP_HWMOD))]
pub unsafe fn omap_hwmod_for_each_by_class(_classname:*const core::ffi::c_char,_fn:Option<unsafe extern "C" fn(*mut omap_hwmod,*mut core::ffi::c_void)->i32>,_user:*mut core::ffi::c_void)->i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
