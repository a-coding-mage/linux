// SPDX-License-Identifier: GPL-2.0
// Translated from buttons.c. External kernel symbols are supplied by other files.

#[repr(C)]
pub struct bcm47xx_gpio_key { pub code: u16, pub pin: u8, pub flags: u8 }

const GPIO_ACTIVE_LOW: u8 = 1;
const GPIO_ACTIVE_HIGH: u8 = 0;

macro_rules! BCM47XX_GPIO_KEY { ($gpio:expr, $code:expr) => { bcm47xx_gpio_key { code: $code, pin: $gpio, flags: GPIO_ACTIVE_LOW } }; }
macro_rules! BCM47XX_GPIO_KEY_H { ($gpio:expr, $code:expr) => { bcm47xx_gpio_key { code: $code, pin: $gpio, flags: GPIO_ACTIVE_HIGH } }; }

// Key-code constants are provided by the kernel input-event definitions.
extern "Rust" {
    static bcm47xx_bus_type: i32;
    fn bcm47xx_board_get() -> i32;
    fn bcm47xx_buttons_add(buttons: *const bcm47xx_gpio_key, nbuttons: i32) -> i32;
}

macro_rules! keys { ($($e:expr),* $(,)?) => { &[$($e),*] }; }

pub static bcm47xx_buttons_asus_rtn10u: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(20, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(21, KEY_RESTART)];
pub static bcm47xx_buttons_asus_rtn12: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(0, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(1, KEY_RESTART), BCM47XX_GPIO_KEY!(4, BTN_0), BCM47XX_GPIO_KEY!(5, BTN_1), BCM47XX_GPIO_KEY!(6, BTN_2)];
pub static bcm47xx_buttons_asus_rtn16: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(6, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(8, KEY_RESTART)];
pub static bcm47xx_buttons_asus_rtn66u: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(9, KEY_RESTART)];
pub static bcm47xx_buttons_asus_wl300g: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_asus_wl320ge: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_asus_wl330ge: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(2, KEY_RESTART)];
pub static bcm47xx_buttons_asus_wl500g: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_asus_wl500gd: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_asus_wl500gpv1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(0, KEY_RESTART), BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_asus_wl500gpv2: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(2, KEY_RESTART), BCM47XX_GPIO_KEY!(3, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_asus_wl500w: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY_H!(6, KEY_RESTART), BCM47XX_GPIO_KEY_H!(7, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_asus_wl520gc: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(2, KEY_RESTART), BCM47XX_GPIO_KEY!(3, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_asus_wl520gu: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(2, KEY_RESTART), BCM47XX_GPIO_KEY!(3, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_asus_wl700ge: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(0, KEY_POWER), BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_COPY), BCM47XX_GPIO_KEY!(7, KEY_RESTART)];
pub static bcm47xx_buttons_asus_wlhdd: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_huawei_e970: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_belkin_f7d4301: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(6, KEY_RESTART), BCM47XX_GPIO_KEY!(8, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_buffalo_whr2_a54g54: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_RESTART)];
pub static bcm47xx_buttons_buffalo_whr_g125: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(0, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(4, KEY_RESTART), BCM47XX_GPIO_KEY!(5, BTN_0)];
pub static bcm47xx_buttons_buffalo_whr_g54s: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(0, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY_H!(4, KEY_RESTART), BCM47XX_GPIO_KEY!(5, BTN_0)];
pub static bcm47xx_buttons_buffalo_whr_hp_g54: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(0, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(4, KEY_RESTART), BCM47XX_GPIO_KEY!(5, BTN_0)];
pub static bcm47xx_buttons_buffalo_wzr_g300n: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_RESTART)];
pub static bcm47xx_buttons_buffalo_wzr_rs_g54: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(0, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(4, KEY_RESTART)];
pub static bcm47xx_buttons_buffalo_wzr_rs_g54hp: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(0, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(4, KEY_RESTART)];
pub static bcm47xx_buttons_dell_tm2300: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(0, KEY_RESTART)];
pub static bcm47xx_buttons_dlink_dir130: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(3, KEY_RESTART), BCM47XX_GPIO_KEY!(7, KEY_UNKNOWN)];
pub static bcm47xx_buttons_dlink_dir330: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(3, KEY_RESTART), BCM47XX_GPIO_KEY!(7, KEY_UNKNOWN)];
pub static bcm47xx_buttons_linksys_e1000v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(5, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_e1000v21: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(9, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(10, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_e2000v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(5, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(8, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_e2500v3: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(9, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(10, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_e3000v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_e3200v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(5, KEY_RESTART), BCM47XX_GPIO_KEY!(8, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_linksys_e4200v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_wrt150nv1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_wrt150nv11: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_wrt160nv1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_wrt160nv3: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(5, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_wrt300n_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_wrt300nv11: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_UNKNOWN), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_wrt310nv1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(6, KEY_RESTART), BCM47XX_GPIO_KEY!(8, KEY_UNKNOWN)];
pub static bcm47xx_buttons_linksys_wrt310n_v2: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(5, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_wrt320n_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(5, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(8, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_wrt54g3gv2: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(5, KEY_WIMAX), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_wrt54g_generic: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_wrt610nv1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(6, KEY_RESTART), BCM47XX_GPIO_KEY!(8, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_linksys_wrt610nv2: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_linksys_wrtsl54gs: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_luxul_abr_4400_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(14, KEY_RESTART)];
pub static bcm47xx_buttons_luxul_xap_310_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(20, KEY_RESTART)];
pub static bcm47xx_buttons_luxul_xap_1210_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(8, KEY_RESTART)];
pub static bcm47xx_buttons_luxul_xap_1230_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(8, KEY_RESTART)];
pub static bcm47xx_buttons_luxul_xap_1240_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(8, KEY_RESTART)];
pub static bcm47xx_buttons_luxul_xap_1500_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(14, KEY_RESTART)];
pub static bcm47xx_buttons_luxul_xbr_4400_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(14, KEY_RESTART)];
pub static bcm47xx_buttons_luxul_xvw_p30_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(20, KEY_RESTART)];
pub static bcm47xx_buttons_luxul_xwr_600_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(8, KEY_RESTART)];
pub static bcm47xx_buttons_luxul_xwr_1750_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(14, KEY_RESTART)];
pub static bcm47xx_buttons_microsoft_nm700: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(7, KEY_RESTART)];
pub static bcm47xx_buttons_motorola_we800g: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(0, KEY_RESTART)];
pub static bcm47xx_buttons_motorola_wr850gp: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(5, KEY_RESTART)];
pub static bcm47xx_buttons_motorola_wr850gv2v3: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(5, KEY_RESTART)];
pub static bcm47xx_buttons_netgear_r6200_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(2, KEY_RFKILL), BCM47XX_GPIO_KEY!(3, KEY_RESTART), BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_netgear_r6300_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_netgear_wn2500rp_v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(12, KEY_RESTART), BCM47XX_GPIO_KEY!(31, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_netgear_wndr3400v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_RESTART), BCM47XX_GPIO_KEY!(6, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(8, KEY_RFKILL)];
pub static bcm47xx_buttons_netgear_wndr3400_v3: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(12, KEY_RESTART), BCM47XX_GPIO_KEY!(23, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_netgear_wndr3700v3: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(2, KEY_RFKILL), BCM47XX_GPIO_KEY!(3, KEY_RESTART), BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_netgear_wndr4500v1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(5, KEY_RFKILL), BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_netgear_wnr1000_v3: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(2, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(3, KEY_RESTART)];
pub static bcm47xx_buttons_netgear_wnr3500lv1: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_RESTART), BCM47XX_GPIO_KEY!(6, KEY_WPS_BUTTON)];
pub static bcm47xx_buttons_netgear_wnr3500lv2: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(4, KEY_RESTART), BCM47XX_GPIO_KEY!(6, KEY_WPS_BUTTON), BCM47XX_GPIO_KEY!(8, KEY_RFKILL)];
pub static bcm47xx_buttons_netgear_wnr834bv2: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(6, KEY_RESTART)];
pub static bcm47xx_buttons_simpletech_simpleshare: &[bcm47xx_gpio_key] = keys![BCM47XX_GPIO_KEY!(0, KEY_RESTART)];

pub fn bcm47xx_buttons_register() -> i32 {
    let board = unsafe { bcm47xx_board_get() };
    let buttons: Option<&'static [bcm47xx_gpio_key]> = match board {
        BCM47XX_BOARD_ASUS_RTN10U => Some(bcm47xx_buttons_asus_rtn10u), BCM47XX_BOARD_ASUS_RTN12 => Some(bcm47xx_buttons_asus_rtn12), BCM47XX_BOARD_ASUS_RTN16 => Some(bcm47xx_buttons_asus_rtn16), BCM47XX_BOARD_ASUS_RTN66U => Some(bcm47xx_buttons_asus_rtn66u), BCM47XX_BOARD_ASUS_WL300G => Some(bcm47xx_buttons_asus_wl300g), BCM47XX_BOARD_ASUS_WL320GE => Some(bcm47xx_buttons_asus_wl320ge), BCM47XX_BOARD_ASUS_WL330GE => Some(bcm47xx_buttons_asus_wl330ge), BCM47XX_BOARD_ASUS_WL500G => Some(bcm47xx_buttons_asus_wl500g), BCM47XX_BOARD_ASUS_WL500GD => Some(bcm47xx_buttons_asus_wl500gd), BCM47XX_BOARD_ASUS_WL500GPV1 => Some(bcm47xx_buttons_asus_wl500gpv1), BCM47XX_BOARD_ASUS_WL500GPV2 => Some(bcm47xx_buttons_asus_wl500gpv2), BCM47XX_BOARD_ASUS_WL500W => Some(bcm47xx_buttons_asus_wl500w), BCM47XX_BOARD_ASUS_WL520GC => Some(bcm47xx_buttons_asus_wl520gc), BCM47XX_BOARD_ASUS_WL520GU => Some(bcm47xx_buttons_asus_wl520gu), BCM47XX_BOARD_ASUS_WL700GE => Some(bcm47xx_buttons_asus_wl700ge), BCM47XX_BOARD_ASUS_WLHDD => Some(bcm47xx_buttons_asus_wlhdd),
        BCM47XX_BOARD_BELKIN_F7D3301 | BCM47XX_BOARD_BELKIN_F7D3302 | BCM47XX_BOARD_BELKIN_F7D4301 | BCM47XX_BOARD_BELKIN_F7D4302 | BCM47XX_BOARD_BELKIN_F7D4401 => Some(bcm47xx_buttons_belkin_f7d4301),
        BCM47XX_BOARD_BUFFALO_WHR2_A54G54 => Some(bcm47xx_buttons_buffalo_whr2_a54g54), BCM47XX_BOARD_BUFFALO_WHR_G125 => Some(bcm47xx_buttons_buffalo_whr_g125), BCM47XX_BOARD_BUFFALO_WHR_G54S => Some(bcm47xx_buttons_buffalo_whr_g54s), BCM47XX_BOARD_BUFFALO_WHR_HP_G54 => Some(bcm47xx_buttons_buffalo_whr_hp_g54), BCM47XX_BOARD_BUFFALO_WZR_G300N => Some(bcm47xx_buttons_buffalo_wzr_g300n), BCM47XX_BOARD_BUFFALO_WZR_RS_G54 => Some(bcm47xx_buttons_buffalo_wzr_rs_g54), BCM47XX_BOARD_BUFFALO_WZR_RS_G54HP => Some(bcm47xx_buttons_buffalo_wzr_rs_g54hp),
        BCM47XX_BOARD_DELL_TM2300 => Some(bcm47xx_buttons_dell_tm2300), BCM47XX_BOARD_DLINK_DIR130 => Some(bcm47xx_buttons_dlink_dir130), BCM47XX_BOARD_DLINK_DIR330 => Some(bcm47xx_buttons_dlink_dir330), BCM47XX_BOARD_HUAWEI_E970 => Some(bcm47xx_buttons_huawei_e970),
        BCM47XX_BOARD_LINKSYS_E1000V1 => Some(bcm47xx_buttons_linksys_e1000v1), BCM47XX_BOARD_LINKSYS_E1000V21 => Some(bcm47xx_buttons_linksys_e1000v21), BCM47XX_BOARD_LINKSYS_E2000V1 => Some(bcm47xx_buttons_linksys_e2000v1), BCM47XX_BOARD_LINKSYS_E2500V3 => Some(bcm47xx_buttons_linksys_e2500v3), BCM47XX_BOARD_LINKSYS_E3000V1 => Some(bcm47xx_buttons_linksys_e3000v1), BCM47XX_BOARD_LINKSYS_E3200V1 => Some(bcm47xx_buttons_linksys_e3200v1), BCM47XX_BOARD_LINKSYS_E4200V1 => Some(bcm47xx_buttons_linksys_e4200v1), BCM47XX_BOARD_LINKSYS_WRT150NV1 => Some(bcm47xx_buttons_linksys_wrt150nv1), BCM47XX_BOARD_LINKSYS_WRT150NV11 => Some(bcm47xx_buttons_linksys_wrt150nv11), BCM47XX_BOARD_LINKSYS_WRT160NV1 => Some(bcm47xx_buttons_linksys_wrt160nv1), BCM47XX_BOARD_LINKSYS_WRT160NV3 => Some(bcm47xx_buttons_linksys_wrt160nv3), BCM47XX_BOARD_LINKSYS_WRT300N_V1 => Some(bcm47xx_buttons_linksys_wrt300n_v1), BCM47XX_BOARD_LINKSYS_WRT300NV11 => Some(bcm47xx_buttons_linksys_wrt300nv11), BCM47XX_BOARD_LINKSYS_WRT310NV1 => Some(bcm47xx_buttons_linksys_wrt310nv1), BCM47XX_BOARD_LINKSYS_WRT310NV2 => Some(bcm47xx_buttons_linksys_wrt310n_v2), BCM47XX_BOARD_LINKSYS_WRT320N_V1 => Some(bcm47xx_buttons_linksys_wrt320n_v1), BCM47XX_BOARD_LINKSYS_WRT54G3GV2 => Some(bcm47xx_buttons_linksys_wrt54g3gv2), BCM47XX_BOARD_LINKSYS_WRT54G_TYPE_0101 | BCM47XX_BOARD_LINKSYS_WRT54G_TYPE_0467 | BCM47XX_BOARD_LINKSYS_WRT54G_TYPE_0708 => Some(bcm47xx_buttons_linksys_wrt54g_generic), BCM47XX_BOARD_LINKSYS_WRT610NV1 => Some(bcm47xx_buttons_linksys_wrt610nv1), BCM47XX_BOARD_LINKSYS_WRT610NV2 => Some(bcm47xx_buttons_linksys_wrt610nv2), BCM47XX_BOARD_LINKSYS_WRTSL54GS => Some(bcm47xx_buttons_linksys_wrtsl54gs),
        BCM47XX_BOARD_LUXUL_ABR_4400_V1 => Some(bcm47xx_buttons_luxul_abr_4400_v1), BCM47XX_BOARD_LUXUL_XAP_310_V1 => Some(bcm47xx_buttons_luxul_xap_310_v1), BCM47XX_BOARD_LUXUL_XAP_1210_V1 => Some(bcm47xx_buttons_luxul_xap_1210_v1), BCM47XX_BOARD_LUXUL_XAP_1230_V1 => Some(bcm47xx_buttons_luxul_xap_1230_v1), BCM47XX_BOARD_LUXUL_XAP_1240_V1 => Some(bcm47xx_buttons_luxul_xap_1240_v1), BCM47XX_BOARD_LUXUL_XAP_1500_V1 => Some(bcm47xx_buttons_luxul_xap_1500_v1), BCM47XX_BOARD_LUXUL_XBR_4400_V1 => Some(bcm47xx_buttons_luxul_xbr_4400_v1), BCM47XX_BOARD_LUXUL_XVW_P30_V1 => Some(bcm47xx_buttons_luxul_xvw_p30_v1), BCM47XX_BOARD_LUXUL_XWR_600_V1 => Some(bcm47xx_buttons_luxul_xwr_600_v1), BCM47XX_BOARD_LUXUL_XWR_1750_V1 => Some(bcm47xx_buttons_luxul_xwr_1750_v1), BCM47XX_BOARD_MICROSOFT_MN700 => Some(bcm47xx_buttons_microsoft_nm700), BCM47XX_BOARD_MOTOROLA_WE800G => Some(bcm47xx_buttons_motorola_we800g), BCM47XX_BOARD_MOTOROLA_WR850GP => Some(bcm47xx_buttons_motorola_wr850gp), BCM47XX_BOARD_MOTOROLA_WR850GV2V3 => Some(bcm47xx_buttons_motorola_wr850gv2v3), BCM47XX_BOARD_NETGEAR_R6200_V1 => Some(bcm47xx_buttons_netgear_r6200_v1), BCM47XX_BOARD_NETGEAR_R6300_V1 => Some(bcm47xx_buttons_netgear_r6300_v1), BCM47XX_BOARD_NETGEAR_WN2500RP_V1 => Some(bcm47xx_buttons_netgear_wn2500rp_v1), BCM47XX_BOARD_NETGEAR_WNDR3400V1 => Some(bcm47xx_buttons_netgear_wndr3400v1), BCM47XX_BOARD_NETGEAR_WNDR3400_V3 => Some(bcm47xx_buttons_netgear_wndr3400_v3), BCM47XX_BOARD_NETGEAR_WNDR3700V3 => Some(bcm47xx_buttons_netgear_wndr3700v3), BCM47XX_BOARD_NETGEAR_WNDR4500V1 => Some(bcm47xx_buttons_netgear_wndr4500v1), BCM47XX_BOARD_NETGEAR_WNR1000_V3 => Some(bcm47xx_buttons_netgear_wnr1000_v3), BCM47XX_BOARD_NETGEAR_WNR3500L => Some(bcm47xx_buttons_netgear_wnr3500lv1), BCM47XX_BOARD_NETGEAR_WNR3500L_V2 => Some(bcm47xx_buttons_netgear_wnr3500lv2), BCM47XX_BOARD_NETGEAR_WNR834BV2 => Some(bcm47xx_buttons_netgear_wnr834bv2), BCM47XX_BOARD_SIMPLETECH_SIMPLESHARE => Some(bcm47xx_buttons_simpletech_simpleshare), _ => return -ENOTSUPP,
    };
    let b = buttons.unwrap();
    unsafe { bcm47xx_buttons_add(b.as_ptr(), b.len() as i32) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
