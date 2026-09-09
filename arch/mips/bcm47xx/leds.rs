// SPDX-License-Identifier: GPL-2.0
// Dependencies are supplied by bcm47xx_private.h, linux/leds.h, and bcm47xx_board.h.

macro_rules! led {
    ($gpio:expr, $color:expr, $function:expr, $active_low:expr, $default:expr) => {
        gpio_led { name: concat!("bcm47xx:", $color, ":", $function), gpio: $gpio, active_low: $active_low, default_state: $default, default_trigger: None }
    };
}
macro_rules! trigger_led {
    ($gpio:expr, $color:expr, $function:expr, $active_low:expr, $trigger:expr) => {
        gpio_led { name: concat!("bcm47xx:", $color, ":", $function), gpio: $gpio, active_low: $active_low, default_state: LEDS_GPIO_DEFSTATE_OFF, default_trigger: Some($trigger) }
    };
}
macro_rules! leds { ($($x:expr),* $(,)?) => { &[$($x),*] }; }

static mut bcm47xx_leds_pdata: gpio_led_platform_data = gpio_led_platform_data { leds: &[], num_leds: 0 };
static mut bcm47xx_leds_pdata_extra: gpio_led_platform_data = gpio_led_platform_data { leds: &[], num_leds: 0 };

static bcm47xx_leds_asus_rtn10u: &[gpio_led] = leds![led!(5,"green","wlan",0,LEDS_GPIO_DEFSTATE_OFF),led!(6,"green","power",1,LEDS_GPIO_DEFSTATE_ON),led!(7,"green","wps",0,LEDS_GPIO_DEFSTATE_OFF),led!(8,"green","usb",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_asus_rtn12: &[gpio_led] = leds![led!(2,"unk","power",1,LEDS_GPIO_DEFSTATE_ON),led!(7,"unk","wlan",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_asus_rtn15u: &[gpio_led] = leds![led!(3,"blue","wan",1,LEDS_GPIO_DEFSTATE_OFF),led!(4,"blue","lan",1,LEDS_GPIO_DEFSTATE_OFF),led!(6,"blue","power",1,LEDS_GPIO_DEFSTATE_ON),led!(9,"blue","usb",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_asus_rtn16: &[gpio_led] = leds![led!(1,"blue","power",1,LEDS_GPIO_DEFSTATE_ON),led!(7,"blue","wlan",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_asus_rtn66u: &[gpio_led] = leds![led!(12,"blue","power",1,LEDS_GPIO_DEFSTATE_ON),led!(15,"blue","usb",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_asus_wl300g: &[gpio_led] = leds![led!(0,"unk","power",1,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_asus_wl320ge: &[gpio_led] = leds![led!(0,"unk","wlan",1,LEDS_GPIO_DEFSTATE_OFF),led!(2,"unk","power",1,LEDS_GPIO_DEFSTATE_ON),led!(11,"unk","link",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_asus_wl330ge: &[gpio_led] = leds![led!(0,"unk","power",1,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_asus_wl500g: &[gpio_led] = leds![led!(0,"unk","power",1,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_asus_wl500gd: &[gpio_led] = leds![led!(0,"unk","power",1,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_asus_wl500gpv1: &[gpio_led] = leds![led!(1,"unk","power",1,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_asus_wl500gpv2: &[gpio_led] = leds![led!(0,"unk","power",1,LEDS_GPIO_DEFSTATE_ON),led!(1,"unk","wlan",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_asus_wl500w: &[gpio_led] = leds![led!(5,"unk","power",1,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_asus_wl520gc: &[gpio_led] = leds![led!(0,"unk","power",1,LEDS_GPIO_DEFSTATE_ON),led!(1,"unk","wlan",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_asus_wl520gu: &[gpio_led] = bcm47xx_leds_asus_wl520gc;
static bcm47xx_leds_asus_wl700ge: &[gpio_led] = leds![led!(1,"unk","power",1,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_asus_wlhdd: &[gpio_led] = leds![led!(0,"unk","power",1,LEDS_GPIO_DEFSTATE_ON),led!(2,"unk","usb",1,LEDS_GPIO_DEFSTATE_OFF)];

static bcm47xx_leds_belkin_f7d4301: &[gpio_led] = leds![led!(10,"green","power",1,LEDS_GPIO_DEFSTATE_ON),led!(11,"amber","power",1,LEDS_GPIO_DEFSTATE_OFF),led!(12,"unk","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(13,"unk","wlan",1,LEDS_GPIO_DEFSTATE_OFF),led!(14,"unk","usb0",1,LEDS_GPIO_DEFSTATE_OFF),led!(15,"unk","usb1",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_buffalo_whr2_a54g54: &[gpio_led] = leds![led!(7,"unk","diag",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_buffalo_whr_g125: &[gpio_led] = leds![led!(1,"unk","bridge",1,LEDS_GPIO_DEFSTATE_OFF),led!(2,"unk","wlan",1,LEDS_GPIO_DEFSTATE_OFF),led!(3,"unk","internal",1,LEDS_GPIO_DEFSTATE_OFF),led!(6,"unk","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(7,"unk","diag",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_buffalo_whr_g54s: &[gpio_led] = leds![led!(1,"green","bridge",1,LEDS_GPIO_DEFSTATE_OFF),led!(2,"green","wlan",1,LEDS_GPIO_DEFSTATE_OFF),led!(3,"green","internal",1,LEDS_GPIO_DEFSTATE_OFF),led!(6,"amber","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(7,"red","diag",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_buffalo_whr_hp_g54: &[gpio_led] = bcm47xx_leds_buffalo_whr_g125;
static bcm47xx_leds_buffalo_wzr_g300n: &[gpio_led] = leds![led!(1,"unk","bridge",1,LEDS_GPIO_DEFSTATE_OFF),led!(6,"unk","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(7,"unk","diag",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_buffalo_wzr_rs_g54: &[gpio_led] = leds![led!(6,"unk","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(1,"unk","vpn",1,LEDS_GPIO_DEFSTATE_OFF),led!(7,"unk","diag",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_buffalo_wzr_rs_g54hp: &[gpio_led] = bcm47xx_leds_buffalo_wzr_rs_g54;

static bcm47xx_leds_dell_tm2300: &[gpio_led] = leds![led!(6,"unk","wlan",1,LEDS_GPIO_DEFSTATE_OFF),led!(7,"unk","power",1,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_dlink_dir130: &[gpio_led] = leds![trigger_led!(0,"green","status",1,"timer"),led!(6,"blue","unk",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_dlink_dir330: &[gpio_led] = leds![trigger_led!(0,"green","status",1,"timer"),led!(4,"unk","usb",1,LEDS_GPIO_DEFSTATE_OFF),led!(6,"blue","unk",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_huawei_b593u_12: &[gpio_led] = leds![led!(5,"blue","wlan",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_huawei_e970: &[gpio_led] = leds![led!(0,"unk","wlan",0,LEDS_GPIO_DEFSTATE_OFF)];

static bcm47xx_leds_linksys_e1000v1: &[gpio_led] = leds![led!(0,"blue","wlan",0,LEDS_GPIO_DEFSTATE_OFF),led!(1,"blue","power",0,LEDS_GPIO_DEFSTATE_ON),led!(2,"amber","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(4,"blue","wps",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_e1000v21: &[gpio_led] = leds![led!(5,"blue","wlan",0,LEDS_GPIO_DEFSTATE_OFF),led!(6,"blue","power",1,LEDS_GPIO_DEFSTATE_ON),led!(7,"amber","wps",0,LEDS_GPIO_DEFSTATE_OFF),led!(8,"blue","wps",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_e2000v1: &[gpio_led] = leds![led!(1,"blue","wlan",0,LEDS_GPIO_DEFSTATE_OFF),led!(2,"blue","power",0,LEDS_GPIO_DEFSTATE_ON),led!(3,"blue","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(4,"amber","wps",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_e3000v1: &[gpio_led] = leds![led!(0,"amber","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(1,"unk","wlan",0,LEDS_GPIO_DEFSTATE_OFF),led!(3,"blue","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(5,"unk","power",0,LEDS_GPIO_DEFSTATE_ON),led!(7,"unk","usb",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_e3200v1: &[gpio_led] = leds![led!(3,"green","power",1,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_linksys_e4200v1: &[gpio_led] = leds![led!(5,"white","power",1,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_linksys_wrt150nv1: &[gpio_led] = leds![led!(1,"unk","power",0,LEDS_GPIO_DEFSTATE_ON),led!(3,"amber","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(5,"green","wps",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_wrt150nv11: &[gpio_led] = bcm47xx_leds_linksys_wrt150nv1;
static bcm47xx_leds_linksys_wrt160nv1: &[gpio_led] = leds![led!(1,"unk","power",0,LEDS_GPIO_DEFSTATE_ON),led!(3,"amber","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(5,"blue","wps",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_wrt160nv3: &[gpio_led] = leds![led!(1,"unk","power",0,LEDS_GPIO_DEFSTATE_ON),led!(2,"amber","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(4,"blue","wps",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_wrt300n_v1: &[gpio_led] = leds![led!(1,"green","power",0,LEDS_GPIO_DEFSTATE_ON),led!(3,"amber","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(5,"green","wps",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_wrt300nv11: &[gpio_led] = leds![led!(1,"unk","power",0,LEDS_GPIO_DEFSTATE_ON),led!(3,"amber","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(5,"green","wps",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_wrt310nv1: &[gpio_led] = leds![led!(1,"blue","power",0,LEDS_GPIO_DEFSTATE_ON),led!(3,"amber","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(9,"blue","wps",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_wrt320n_v1: &[gpio_led] = leds![led!(1,"blue","wlan",1,LEDS_GPIO_DEFSTATE_OFF),led!(2,"blue","power",0,LEDS_GPIO_DEFSTATE_ON),led!(4,"amber","wps",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_wrt54g_generic: &[gpio_led] = leds![led!(0,"unk","dmz",1,LEDS_GPIO_DEFSTATE_OFF),led!(1,"unk","power",0,LEDS_GPIO_DEFSTATE_ON),led!(5,"white","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(7,"orange","wps",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_wrt54g3gv2: &[gpio_led] = leds![led!(1,"unk","power",0,LEDS_GPIO_DEFSTATE_ON),led!(2,"green","3g",0,LEDS_GPIO_DEFSTATE_OFF),led!(3,"blue","3g",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_wrt54g_type_0101: &[gpio_led] = leds![led!(0,"green","wlan",1,LEDS_GPIO_DEFSTATE_OFF),led!(1,"green","power",0,LEDS_GPIO_DEFSTATE_ON),led!(7,"green","dmz",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_wrt54g_type_0467: &[gpio_led] = leds![led!(0,"green","wlan",1,LEDS_GPIO_DEFSTATE_OFF),led!(1,"green","power",0,LEDS_GPIO_DEFSTATE_ON),led!(2,"white","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(3,"orange","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(7,"green","dmz",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_wrt610nv1: &[gpio_led] = leds![led!(0,"unk","usb",1,LEDS_GPIO_DEFSTATE_OFF),led!(1,"unk","power",0,LEDS_GPIO_DEFSTATE_OFF),led!(3,"amber","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(9,"blue","wps",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_linksys_wrt610nv2: &[gpio_led] = bcm47xx_leds_linksys_e3000v1;
static bcm47xx_leds_linksys_wrtsl54gs: &[gpio_led] = leds![led!(0,"green","dmz",1,LEDS_GPIO_DEFSTATE_OFF),led!(1,"green","power",0,LEDS_GPIO_DEFSTATE_ON),led!(5,"white","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(7,"orange","wps",1,LEDS_GPIO_DEFSTATE_OFF)];

static bcm47xx_leds_luxul_abr_4400_v1: &[gpio_led] = leds![led!(12,"green","usb",0,LEDS_GPIO_DEFSTATE_OFF),trigger_led!(15,"green","status",0,"timer")];
static bcm47xx_leds_luxul_xap_310_v1: &[gpio_led] = leds![trigger_led!(6,"green","status",1,"timer")];
static bcm47xx_leds_luxul_xap_1210_v1: &[gpio_led] = bcm47xx_leds_luxul_xap_310_v1;
static bcm47xx_leds_luxul_xap_1230_v1: &[gpio_led] = leds![led!(3,"blue","2ghz",0,LEDS_GPIO_DEFSTATE_OFF),led!(4,"green","bridge",0,LEDS_GPIO_DEFSTATE_OFF),trigger_led!(6,"green","status",1,"timer")];
static bcm47xx_leds_luxul_xap_1240_v1: &[gpio_led] = bcm47xx_leds_luxul_xap_1230_v1;
static bcm47xx_leds_luxul_xap_1500_v1: &[gpio_led] = leds![trigger_led!(13,"green","status",1,"timer")];
static bcm47xx_leds_luxul_xap1500_v1_extra: &[gpio_led] = leds![led!(44,"green","5ghz",0,LEDS_GPIO_DEFSTATE_OFF),led!(76,"green","2ghz",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_luxul_xbr_4400_v1: &[gpio_led] = bcm47xx_leds_luxul_abr_4400_v1;
static bcm47xx_leds_luxul_xvw_p30_v1: &[gpio_led] = leds![trigger_led!(0,"blue","status",1,"timer"),led!(1,"green","link",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_luxul_xwr_600_v1: &[gpio_led] = leds![led!(3,"green","wps",0,LEDS_GPIO_DEFSTATE_OFF),trigger_led!(6,"green","status",1,"timer"),led!(9,"green","usb",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_luxul_xwr_1750_v1: &[gpio_led] = leds![led!(5,"green","5ghz",0,LEDS_GPIO_DEFSTATE_OFF),led!(12,"green","usb",0,LEDS_GPIO_DEFSTATE_OFF),trigger_led!(13,"green","status",0,"timer"),led!(15,"green","wps",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_luxul_xwr1750_v1_extra: &[gpio_led] = leds![led!(76,"green","2ghz",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_microsoft_nm700: &[gpio_led] = leds![led!(6,"unk","power",0,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_motorola_we800g: &[gpio_led] = leds![led!(1,"amber","wlan",0,LEDS_GPIO_DEFSTATE_OFF),led!(2,"unk","unk",1,LEDS_GPIO_DEFSTATE_OFF),led!(4,"green","power",0,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_motorola_wr850gp: &[gpio_led] = leds![led!(0,"unk","wlan",1,LEDS_GPIO_DEFSTATE_OFF),led!(1,"unk","power",0,LEDS_GPIO_DEFSTATE_ON),led!(6,"unk","dmz",1,LEDS_GPIO_DEFSTATE_OFF),led!(7,"unk","diag",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_motorola_wr850gv2v3: &[gpio_led] = leds![led!(0,"unk","wlan",1,LEDS_GPIO_DEFSTATE_OFF),led!(1,"unk","power",0,LEDS_GPIO_DEFSTATE_ON),led!(7,"unk","diag",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_netgear_wndr3400v1: &[gpio_led] = leds![led!(2,"green","usb",1,LEDS_GPIO_DEFSTATE_OFF),led!(3,"green","power",0,LEDS_GPIO_DEFSTATE_ON),led!(7,"amber","power",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_netgear_wndr4500v1: &[gpio_led] = leds![led!(1,"green","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(2,"green","power",1,LEDS_GPIO_DEFSTATE_ON),led!(3,"amber","power",1,LEDS_GPIO_DEFSTATE_OFF),led!(8,"green","usb1",1,LEDS_GPIO_DEFSTATE_OFF),led!(9,"green","2ghz",1,LEDS_GPIO_DEFSTATE_OFF),led!(11,"blue","5ghz",1,LEDS_GPIO_DEFSTATE_OFF),led!(14,"green","usb2",1,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_netgear_wnr1000_v3: &[gpio_led] = leds![led!(0,"blue","wlan",0,LEDS_GPIO_DEFSTATE_OFF),led!(1,"green","wps",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_netgear_wnr3500lv1: &[gpio_led] = leds![led!(0,"blue","wlan",1,LEDS_GPIO_DEFSTATE_OFF),led!(1,"green","wps",1,LEDS_GPIO_DEFSTATE_OFF),led!(2,"green","wan",1,LEDS_GPIO_DEFSTATE_OFF),led!(3,"green","power",0,LEDS_GPIO_DEFSTATE_ON),led!(7,"amber","power",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_netgear_wnr3500lv2: &[gpio_led] = leds![led!(0,"blue","wlan",0,LEDS_GPIO_DEFSTATE_OFF),led!(1,"green","wps",0,LEDS_GPIO_DEFSTATE_OFF),led!(3,"green","power",0,LEDS_GPIO_DEFSTATE_ON),led!(7,"amber","power",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_netgear_wnr834bv2: &[gpio_led] = leds![led!(2,"green","power",0,LEDS_GPIO_DEFSTATE_ON),led!(3,"amber","power",0,LEDS_GPIO_DEFSTATE_OFF),led!(7,"unk","connected",0,LEDS_GPIO_DEFSTATE_OFF)];
static bcm47xx_leds_siemens_se505v2: &[gpio_led] = leds![led!(0,"unk","dmz",1,LEDS_GPIO_DEFSTATE_OFF),led!(3,"unk","wlan",1,LEDS_GPIO_DEFSTATE_OFF),led!(5,"unk","power",1,LEDS_GPIO_DEFSTATE_ON)];
static bcm47xx_leds_simpletech_simpleshare: &[gpio_led] = leds![led!(1,"unk","status",1,LEDS_GPIO_DEFSTATE_OFF)];

// Board selection and registration retain the original switch and external calls.
pub unsafe fn bcm47xx_leds_register() {
    let board = bcm47xx_board_get();
    macro_rules! set { ($x:ident) => {{ bcm47xx_leds_pdata.leds = $x; bcm47xx_leds_pdata.num_leds = $x.len(); }} }
    match board {
        BCM47XX_BOARD_ASUS_RTN10U => set!(bcm47xx_leds_asus_rtn10u), BCM47XX_BOARD_ASUS_RTN12 => set!(bcm47xx_leds_asus_rtn12), BCM47XX_BOARD_ASUS_RTN15U => set!(bcm47xx_leds_asus_rtn15u), BCM47XX_BOARD_ASUS_RTN16 => set!(bcm47xx_leds_asus_rtn16), BCM47XX_BOARD_ASUS_RTN66U => set!(bcm47xx_leds_asus_rtn66u), BCM47XX_BOARD_ASUS_WL300G => set!(bcm47xx_leds_asus_wl300g), BCM47XX_BOARD_ASUS_WL320GE => set!(bcm47xx_leds_asus_wl320ge), BCM47XX_BOARD_ASUS_WL330GE => set!(bcm47xx_leds_asus_wl330ge), BCM47XX_BOARD_ASUS_WL500G => set!(bcm47xx_leds_asus_wl500g), BCM47XX_BOARD_ASUS_WL500GD => set!(bcm47xx_leds_asus_wl500gd), BCM47XX_BOARD_ASUS_WL500GPV1 => set!(bcm47xx_leds_asus_wl500gpv1), BCM47XX_BOARD_ASUS_WL500GPV2 => set!(bcm47xx_leds_asus_wl500gpv2), BCM47XX_BOARD_ASUS_WL500W => set!(bcm47xx_leds_asus_wl500w), BCM47XX_BOARD_ASUS_WL520GC => set!(bcm47xx_leds_asus_wl520gc), BCM47XX_BOARD_ASUS_WL520GU => set!(bcm47xx_leds_asus_wl520gu), BCM47XX_BOARD_ASUS_WL700GE => set!(bcm47xx_leds_asus_wl700ge), BCM47XX_BOARD_ASUS_WLHDD => set!(bcm47xx_leds_asus_wlhdd),
        BCM47XX_BOARD_BELKIN_F7D3301 | BCM47XX_BOARD_BELKIN_F7D3302 | BCM47XX_BOARD_BELKIN_F7D4301 | BCM47XX_BOARD_BELKIN_F7D4302 | BCM47XX_BOARD_BELKIN_F7D4401 => set!(bcm47xx_leds_belkin_f7d4301),
        BCM47XX_BOARD_BUFFALO_WHR2_A54G54 => set!(bcm47xx_leds_buffalo_whr2_a54g54), BCM47XX_BOARD_BUFFALO_WHR_G125 => set!(bcm47xx_leds_buffalo_whr_g125), BCM47XX_BOARD_BUFFALO_WHR_G54S => set!(bcm47xx_leds_buffalo_whr_g54s), BCM47XX_BOARD_BUFFALO_WHR_HP_G54 => set!(bcm47xx_leds_buffalo_whr_hp_g54), BCM47XX_BOARD_BUFFALO_WZR_G300N => set!(bcm47xx_leds_buffalo_wzr_g300n), BCM47XX_BOARD_BUFFALO_WZR_RS_G54 => set!(bcm47xx_leds_buffalo_wzr_rs_g54), BCM47XX_BOARD_BUFFALO_WZR_RS_G54HP => set!(bcm47xx_leds_buffalo_wzr_rs_g54hp), BCM47XX_BOARD_DELL_TM2300 => set!(bcm47xx_leds_dell_tm2300), BCM47XX_BOARD_DLINK_DIR130 => set!(bcm47xx_leds_dlink_dir130), BCM47XX_BOARD_DLINK_DIR330 => set!(bcm47xx_leds_dlink_dir330), BCM47XX_BOARD_HUAWEI_B593U_12 => set!(bcm47xx_leds_huawei_b593u_12), BCM47XX_BOARD_HUAWEI_E970 => set!(bcm47xx_leds_huawei_e970),
        BCM47XX_BOARD_LINKSYS_E1000V1 => set!(bcm47xx_leds_linksys_e1000v1), BCM47XX_BOARD_LINKSYS_E1000V21 => set!(bcm47xx_leds_linksys_e1000v21), BCM47XX_BOARD_LINKSYS_E2000V1 => set!(bcm47xx_leds_linksys_e2000v1), BCM47XX_BOARD_LINKSYS_E3000V1 => set!(bcm47xx_leds_linksys_e3000v1), BCM47XX_BOARD_LINKSYS_E3200V1 => set!(bcm47xx_leds_linksys_e3200v1), BCM47XX_BOARD_LINKSYS_E4200V1 => set!(bcm47xx_leds_linksys_e4200v1), BCM47XX_BOARD_LINKSYS_WRT150NV1 => set!(bcm47xx_leds_linksys_wrt150nv1), BCM47XX_BOARD_LINKSYS_WRT150NV11 => set!(bcm47xx_leds_linksys_wrt150nv11), BCM47XX_BOARD_LINKSYS_WRT160NV1 => set!(bcm47xx_leds_linksys_wrt160nv1), BCM47XX_BOARD_LINKSYS_WRT160NV3 => set!(bcm47xx_leds_linksys_wrt160nv3), BCM47XX_BOARD_LINKSYS_WRT300N_V1 => set!(bcm47xx_leds_linksys_wrt300n_v1), BCM47XX_BOARD_LINKSYS_WRT300NV11 => set!(bcm47xx_leds_linksys_wrt300nv11), BCM47XX_BOARD_LINKSYS_WRT310NV1 => set!(bcm47xx_leds_linksys_wrt310nv1), BCM47XX_BOARD_LINKSYS_WRT320N_V1 => set!(bcm47xx_leds_linksys_wrt320n_v1), BCM47XX_BOARD_LINKSYS_WRT54G3GV2 => set!(bcm47xx_leds_linksys_wrt54g3gv2), BCM47XX_BOARD_LINKSYS_WRT54G_TYPE_0101 => set!(bcm47xx_leds_linksys_wrt54g_type_0101), BCM47XX_BOARD_LINKSYS_WRT54G_TYPE_0467 => set!(bcm47xx_leds_linksys_wrt54g_type_0467), BCM47XX_BOARD_LINKSYS_WRT54G_TYPE_0708 => set!(bcm47xx_leds_linksys_wrt54g_generic), BCM47XX_BOARD_LINKSYS_WRT610NV1 => set!(bcm47xx_leds_linksys_wrt610nv1), BCM47XX_BOARD_LINKSYS_WRT610NV2 => set!(bcm47xx_leds_linksys_wrt610nv2), BCM47XX_BOARD_LINKSYS_WRTSL54GS => set!(bcm47xx_leds_linksys_wrtsl54gs),
        BCM47XX_BOARD_LUXUL_ABR_4400_V1 => set!(bcm47xx_leds_luxul_abr_4400_v1), BCM47XX_BOARD_LUXUL_XAP_310_V1 => set!(bcm47xx_leds_luxul_xap_310_v1), BCM47XX_BOARD_LUXUL_XAP_1210_V1 => set!(bcm47xx_leds_luxul_xap_1210_v1), BCM47XX_BOARD_LUXUL_XAP_1230_V1 => set!(bcm47xx_leds_luxul_xap_1230_v1), BCM47XX_BOARD_LUXUL_XAP_1240_V1 => set!(bcm47xx_leds_luxul_xap_1240_v1), BCM47XX_BOARD_LUXUL_XAP_1500_V1 => { set!(bcm47xx_leds_luxul_xap_1500_v1); bcm47xx_leds_pdata_extra.leds = bcm47xx_leds_luxul_xap1500_v1_extra; bcm47xx_leds_pdata_extra.num_leds = bcm47xx_leds_luxul_xap1500_v1_extra.len(); }, BCM47XX_BOARD_LUXUL_XBR_4400_V1 => set!(bcm47xx_leds_luxul_xbr_4400_v1), BCM47XX_BOARD_LUXUL_XVW_P30_V1 => set!(bcm47xx_leds_luxul_xvw_p30_v1), BCM47XX_BOARD_LUXUL_XWR_600_V1 => set!(bcm47xx_leds_luxul_xwr_600_v1), BCM47XX_BOARD_LUXUL_XWR_1750_V1 => { set!(bcm47xx_leds_luxul_xwr_1750_v1); bcm47xx_leds_pdata_extra.leds = bcm47xx_leds_luxul_xwr1750_v1_extra; bcm47xx_leds_pdata_extra.num_leds = bcm47xx_leds_luxul_xwr1750_v1_extra.len(); },
        BCM47XX_BOARD_MICROSOFT_MN700 => set!(bcm47xx_leds_microsoft_nm700), BCM47XX_BOARD_MOTOROLA_WE800G => set!(bcm47xx_leds_motorola_we800g), BCM47XX_BOARD_MOTOROLA_WR850GP => set!(bcm47xx_leds_motorola_wr850gp), BCM47XX_BOARD_MOTOROLA_WR850GV2V3 => set!(bcm47xx_leds_motorola_wr850gv2v3), BCM47XX_BOARD_NETGEAR_WNDR3400V1 => set!(bcm47xx_leds_netgear_wndr3400v1), BCM47XX_BOARD_NETGEAR_WNDR4500V1 => set!(bcm47xx_leds_netgear_wndr4500v1), BCM47XX_BOARD_NETGEAR_WNR1000_V3 => set!(bcm47xx_leds_netgear_wnr1000_v3), BCM47XX_BOARD_NETGEAR_WNR3500L => set!(bcm47xx_leds_netgear_wnr3500lv1), BCM47XX_BOARD_NETGEAR_WNR3500L_V2 => set!(bcm47xx_leds_netgear_wnr3500lv2), BCM47XX_BOARD_NETGEAR_WNR834BV2 => set!(bcm47xx_leds_netgear_wnr834bv2), BCM47XX_BOARD_SIEMENS_SE505V2 => set!(bcm47xx_leds_siemens_se505v2), BCM47XX_BOARD_SIMPLETECH_SIMPLESHARE => set!(bcm47xx_leds_simpletech_simpleshare),
        _ => { pr_debug!("No LEDs configuration found for this device\n"); return; }
    }
    gpio_led_register_device(-1, &bcm47xx_leds_pdata);
    if bcm47xx_leds_pdata_extra.num_leds != 0 { gpio_led_register_device(0, &bcm47xx_leds_pdata_extra); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
