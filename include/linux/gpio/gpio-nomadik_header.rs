/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies and include guards are intentionally omitted.

pub struct fwnode_handle;

/* Package definitions */
pub const PINCTRL_NMK_STN8815: u32 = 0;
pub const PINCTRL_NMK_DB8500: u32 = 1;

pub const GPIO_BLOCK_SHIFT: u32 = 5;
pub const NMK_GPIO_PER_CHIP: u32 = 1u32 << GPIO_BLOCK_SHIFT;
pub const NMK_MAX_BANKS: u32 = (512 + NMK_GPIO_PER_CHIP - 1) / NMK_GPIO_PER_CHIP;

/* Register in the logic block */
pub const NMK_GPIO_DAT: u32 = 0x00;
pub const NMK_GPIO_DATS: u32 = 0x04;
pub const NMK_GPIO_DATC: u32 = 0x08;
pub const NMK_GPIO_PDIS: u32 = 0x0c;
pub const NMK_GPIO_DIR: u32 = 0x10;
pub const NMK_GPIO_DIRS: u32 = 0x14;
pub const NMK_GPIO_DIRC: u32 = 0x18;
pub const NMK_GPIO_SLPC: u32 = 0x1c;
pub const NMK_GPIO_AFSLA: u32 = 0x20;
pub const NMK_GPIO_AFSLB: u32 = 0x24;
pub const NMK_GPIO_LOWEMI: u32 = 0x28;
pub const NMK_GPIO_RIMSC: u32 = 0x40;
pub const NMK_GPIO_FIMSC: u32 = 0x44;
pub const NMK_GPIO_IS: u32 = 0x48;
pub const NMK_GPIO_IC: u32 = 0x4c;
pub const NMK_GPIO_RWIMSC: u32 = 0x50;
pub const NMK_GPIO_FWIMSC: u32 = 0x54;
pub const NMK_GPIO_WKS: u32 = 0x58;

/* Pull up/down values */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum nmk_gpio_pull { NMK_GPIO_PULL_NONE, NMK_GPIO_PULL_UP, NMK_GPIO_PULL_DOWN }

/* Sleep mode */
#[repr(C)]
#[derive(Copy, Clone)]
pub enum nmk_gpio_slpm {
    NMK_GPIO_SLPM_INPUT,
    NMK_GPIO_SLPM_WAKEUP_ENABLE = 0,
    NMK_GPIO_SLPM_NOCHANGE,
    NMK_GPIO_SLPM_WAKEUP_DISABLE = 1,
}

#[repr(C)]
pub struct nmk_gpio_chip {
    pub chip: gpio_chip,
    pub addr: *mut core::ffi::c_void,
    pub clk: *mut clk,
    pub bank: core::ffi::c_uint,
    pub set_ioforce: Option<unsafe extern "C" fn(enable: bool)>,
    pub lock: spinlock_t,
    pub sleepmode: bool,
    pub is_mobileye_soc: bool,
    /* Keep track of configured edges */
    pub edge_rising: u32,
    pub edge_falling: u32,
    pub real_wake: u32,
    pub rwimsc: u32,
    pub fwimsc: u32,
    pub rimsc: u32,
    pub fimsc: u32,
    pub pull_up: u32,
    pub lowemi: u32,
}

/* Alternate functions: function C is set in hw by setting both A and B */
pub const NMK_GPIO_ALT_GPIO: u32 = 0;
pub const NMK_GPIO_ALT_A: u32 = 1;
pub const NMK_GPIO_ALT_B: u32 = 2;
pub const NMK_GPIO_ALT_C: u32 = NMK_GPIO_ALT_A | NMK_GPIO_ALT_B;
pub const NMK_GPIO_ALT_CX_SHIFT: u32 = 2;
pub const NMK_GPIO_ALT_C1: u32 = (1 << NMK_GPIO_ALT_CX_SHIFT) | NMK_GPIO_ALT_C;
pub const NMK_GPIO_ALT_C2: u32 = (2 << NMK_GPIO_ALT_CX_SHIFT) | NMK_GPIO_ALT_C;
pub const NMK_GPIO_ALT_C3: u32 = (3 << NMK_GPIO_ALT_CX_SHIFT) | NMK_GPIO_ALT_C;
pub const NMK_GPIO_ALT_C4: u32 = (4 << NMK_GPIO_ALT_CX_SHIFT) | NMK_GPIO_ALT_C;

#[macro_export]
macro_rules! PRCM_GPIOCR_ALTCX {
    ($pin_num:expr, $altc1_used:expr, $altc1_ri:expr, $altc1_cb:expr,
     $altc2_used:expr, $altc2_ri:expr, $altc2_cb:expr,
     $altc3_used:expr, $altc3_ri:expr, $altc3_cb:expr,
     $altc4_used:expr, $altc4_ri:expr, $altc4_cb:expr) => {
        nmk_prcm_gpiocr_pin_desc { pin: $pin_num, altcx: [
            nmk_prcm_gpiocr_altcx { used: $altc1_used, reg_index: $altc1_ri, control_bit: $altc1_cb },
            nmk_prcm_gpiocr_altcx { used: $altc2_used, reg_index: $altc2_ri, control_bit: $altc2_cb },
            nmk_prcm_gpiocr_altcx { used: $altc3_used, reg_index: $altc3_ri, control_bit: $altc3_cb },
            nmk_prcm_gpiocr_altcx { used: $altc4_used, reg_index: $altc4_ri, control_bit: $altc4_cb },
        ] }
    };
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum prcm_gpiocr_reg_index { PRCM_IDX_GPIOCR1, PRCM_IDX_GPIOCR2, PRCM_IDX_GPIOCR3 }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum prcm_gpiocr_altcx_index {
    PRCM_IDX_GPIOCR_ALTC1, PRCM_IDX_GPIOCR_ALTC2, PRCM_IDX_GPIOCR_ALTC3,
    PRCM_IDX_GPIOCR_ALTC4, PRCM_IDX_GPIOCR_ALTC_MAX,
}

#[repr(C, packed)]
pub struct prcm_gpiocr_altcx { pub used: bool, pub reg_index: u8, pub control_bit: u8 }
#[repr(C)]
pub struct nmk_prcm_gpiocr_pin_desc { pub pin: u16, pub altcx: [prcm_gpiocr_altcx; 4] }
pub type prcm_gpiocr_altcx_pin_desc = nmk_prcm_gpiocr_pin_desc;

#[repr(C)]
pub struct nmk_function { pub name: *const core::ffi::c_char, pub groups: *const *const core::ffi::c_char, pub ngroups: core::ffi::c_uint }
#[repr(C)]
pub struct nmk_pingroup { pub grp: pingroup, pub altsetting: core::ffi::c_int }

#[macro_export]
macro_rules! NMK_PIN_GROUP { ($a:ident, $b:expr) => { nmk_pingroup { grp: PINCTRL_PINGROUP!(stringify!($a), $a##_pins, $a##_pins.len()), altsetting: $b } }; }

#[repr(C)]
pub struct nmk_pinctrl_soc_data {
    pub pins: *const pinctrl_pin_desc, pub npins: core::ffi::c_uint,
    pub functions: *const nmk_function, pub nfunctions: core::ffi::c_uint,
    pub groups: *const nmk_pingroup, pub ngroups: core::ffi::c_uint,
    pub altcx_pins: *const prcm_gpiocr_altcx_pin_desc, pub npins_altcx: core::ffi::c_uint,
    pub prcm_gpiocr_registers: *const u16,
}

// CONFIG_PINCTRL_STN8815 selects the external implementation; otherwise this is a no-op.
#[cfg(feature = "CONFIG_PINCTRL_STN8815")]
unsafe extern "C" { pub fn nmk_pinctrl_stn8815_init(soc: *const *const nmk_pinctrl_soc_data); }
#[cfg(not(feature = "CONFIG_PINCTRL_STN8815"))]
pub unsafe fn nmk_pinctrl_stn8815_init(_soc: *const *const nmk_pinctrl_soc_data) {}

// CONFIG_PINCTRL_DB8500 selects the external implementation; otherwise this is a no-op.
#[cfg(feature = "CONFIG_PINCTRL_DB8500")]
unsafe extern "C" { pub fn nmk_pinctrl_db8500_init(soc: *const *const nmk_pinctrl_soc_data); }
#[cfg(not(feature = "CONFIG_PINCTRL_DB8500"))]
pub unsafe fn nmk_pinctrl_db8500_init(_soc: *const *const nmk_pinctrl_soc_data) {}

pub struct platform_device;

// CONFIG_DEBUG_FS selects the external debug implementation; otherwise this is a no-op.
#[cfg(feature = "CONFIG_DEBUG_FS")]
unsafe extern "C" { pub fn nmk_gpio_dbg_show_one(s: *mut seq_file, pctldev: *mut pinctrl_dev, chip: *mut gpio_chip, offset: core::ffi::c_uint); }
#[cfg(not(feature = "CONFIG_DEBUG_FS"))]
pub unsafe fn nmk_gpio_dbg_show_one(_s: *mut seq_file, _pctldev: *mut pinctrl_dev, _chip: *mut gpio_chip, _offset: core::ffi::c_uint) {}

unsafe extern "C" {
    pub fn __nmk_gpio_make_output(nmk_chip: *mut nmk_gpio_chip, offset: core::ffi::c_uint, val: core::ffi::c_int);
    pub fn __nmk_gpio_set_slpm(nmk_chip: *mut nmk_gpio_chip, offset: core::ffi::c_uint, mode: nmk_gpio_slpm);
    pub fn nmk_gpio_populate_chip(fwnode: *mut fwnode_handle, pdev: *mut platform_device) -> *mut nmk_gpio_chip;
}

// CONFIG_PINCTRL_NOMADIK declares symbols supplied by pinctrl-nomadik.
#[cfg(feature = "CONFIG_PINCTRL_NOMADIK")]
unsafe extern "C" {
    pub static mut nmk_gpio_chips: [*mut nmk_gpio_chip; NMK_MAX_BANKS as usize];
    pub static mut nmk_gpio_slpm_lock: spinlock_t;
    pub fn nmk_prcm_gpiocr_get_mode(pctldev: *mut pinctrl_dev, gpio: core::ffi::c_int) -> core::ffi::c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
