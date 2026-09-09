// Translation of cvmx-gpio-defs.h.

macro_rules! CVMX_GPIO_BIT_CFGX { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001070000000800u64) + (($offset) & 15) * 8 }; }
macro_rules! CVMX_GPIO_BOOT_ENA { () => { CVMX_ADD_IO_SEG(0x00010700000008A8u64) }; }
macro_rules! CVMX_GPIO_CLK_GENX { ($offset:expr) => { CVMX_ADD_IO_SEG(0x00010700000008C0u64) + (($offset) & 3) * 8 }; }
macro_rules! CVMX_GPIO_CLK_QLMX { ($offset:expr) => { CVMX_ADD_IO_SEG(0x00010700000008E0u64) + (($offset) & 1) * 8 }; }
macro_rules! CVMX_GPIO_DBG_ENA { () => { CVMX_ADD_IO_SEG(0x00010700000008A0u64) }; }
macro_rules! CVMX_GPIO_INT_CLR { () => { CVMX_ADD_IO_SEG(0x0001070000000898u64) }; }
macro_rules! CVMX_GPIO_MULTI_CAST { () => { CVMX_ADD_IO_SEG(0x00010700000008B0u64) }; }
macro_rules! CVMX_GPIO_PIN_ENA { () => { CVMX_ADD_IO_SEG(0x00010700000008B8u64) }; }
macro_rules! CVMX_GPIO_RX_DAT { () => { CVMX_ADD_IO_SEG(0x0001070000000880u64) }; }
macro_rules! CVMX_GPIO_TIM_CTL { () => { CVMX_ADD_IO_SEG(0x00010700000008A0u64) }; }
macro_rules! CVMX_GPIO_TX_CLR { () => { CVMX_ADD_IO_SEG(0x0001070000000890u64) }; }
macro_rules! CVMX_GPIO_TX_SET { () => { CVMX_ADD_IO_SEG(0x0001070000000888u64) }; }
macro_rules! CVMX_GPIO_XBIT_CFGX { ($offset:expr) => { CVMX_ADD_IO_SEG(0x0001070000000900u64) + (($offset) & 31) * 8 - 8 * 16 }; }

#[repr(C)]
pub union cvmx_gpio_bit_cfgx {
    pub u64: u64,
    pub s: cvmx_gpio_bit_cfgx_s,
    pub cn30xx: cvmx_gpio_bit_cfgx_cn30xx,
    pub cn52xx: cvmx_gpio_bit_cfgx_cn52xx,
}
#[repr(C)] pub struct cvmx_gpio_bit_cfgx_s { pub bits: u64 }
// tx_oe:1, rx_xor:1, int_en:1, int_type:1, fil_cnt:4, fil_sel:4, clk_sel:2, clk_gen:1, synce_sel:2, output_sel:5, reserved_21_63:42
#[repr(C)] pub struct cvmx_gpio_bit_cfgx_cn30xx { pub bits: u64 }
// tx_oe:1, rx_xor:1, int_en:1, int_type:1, fil_cnt:4, fil_sel:4, reserved_12_63:52
#[repr(C)] pub struct cvmx_gpio_bit_cfgx_cn52xx { pub bits: u64 }
// tx_oe:1, rx_xor:1, int_en:1, int_type:1, fil_cnt:4, fil_sel:4, clk_sel:2, clk_gen:1, reserved_15_63:49

macro_rules! cvmx_gpio_union { ($name:ident, $($variant:ident),+ $(,)?) => {
    #[repr(C)] pub union $name { pub u64: u64, $(pub $variant: $variant,)+ }
}; }

#[repr(C)] pub struct cvmx_gpio_boot_ena_s { pub bits: u64 }
// reserved_0_7:8, boot_ena:4, reserved_12_63:52
cvmx_gpio_union!(cvmx_gpio_boot_ena, cvmx_gpio_boot_ena_s);
#[repr(C)] pub struct cvmx_gpio_clk_genx_s { pub bits: u64 }
// n:32, reserved_32_63:32
cvmx_gpio_union!(cvmx_gpio_clk_genx, cvmx_gpio_clk_genx_s);
#[repr(C)] pub struct cvmx_gpio_clk_qlmx_s { pub bits: u64 }
#[repr(C)] pub struct cvmx_gpio_clk_qlmx_cn61xx { pub bits: u64 }
#[repr(C)] pub struct cvmx_gpio_clk_qlmx_cn63xx { pub bits: u64 }
// s: lane_sel:2, div:1, reserved_3_7:5, qlm_sel:3, reserved_11_63:53
// cn61xx: lane_sel:2, div:1, reserved_3_7:5, qlm_sel:2, reserved_10_63:54
// cn63xx: lane_sel:2, div:1, reserved_3_63:61
cvmx_gpio_union!(cvmx_gpio_clk_qlmx, cvmx_gpio_clk_qlmx_s, cvmx_gpio_clk_qlmx_cn61xx, cvmx_gpio_clk_qlmx_cn63xx);

macro_rules! cvmx_gpio_simple { ($union:ident, $struct:ident, $comment:literal) => {
    #[repr(C)] pub struct $struct { pub bits: u64 }
    // $comment
    cvmx_gpio_union!($union, $struct);
}; }
cvmx_gpio_simple!(cvmx_gpio_dbg_ena, cvmx_gpio_dbg_ena_s, "dbg_ena:21, reserved_21_63:43");
cvmx_gpio_simple!(cvmx_gpio_int_clr, cvmx_gpio_int_clr_s, "type:16, reserved_16_63:48");
cvmx_gpio_simple!(cvmx_gpio_multi_cast, cvmx_gpio_multi_cast_s, "en:1, reserved_1_63:63");
cvmx_gpio_simple!(cvmx_gpio_pin_ena, cvmx_gpio_pin_ena_s, "reserved_0_17:18, ena18:1, ena19:1, reserved_20_63:44");

#[repr(C)] pub struct cvmx_gpio_rx_dat_s { pub bits: u64 }
#[repr(C)] pub struct cvmx_gpio_rx_dat_cn38xx { pub bits: u64 }
#[repr(C)] pub struct cvmx_gpio_rx_dat_cn61xx { pub bits: u64 }
// dat:24/16/20 with corresponding reserved high bits.
cvmx_gpio_union!(cvmx_gpio_rx_dat, cvmx_gpio_rx_dat_s, cvmx_gpio_rx_dat_cn38xx, cvmx_gpio_rx_dat_cn61xx);
#[repr(C)] pub struct cvmx_gpio_tim_ctl_s { pub bits: u64 }
// sel:4, reserved_4_63:60
cvmx_gpio_union!(cvmx_gpio_tim_ctl, cvmx_gpio_tim_ctl_s);
#[repr(C)] pub struct cvmx_gpio_tx_clr_s { pub bits: u64 }
#[repr(C)] pub struct cvmx_gpio_tx_clr_cn38xx { pub bits: u64 }
#[repr(C)] pub struct cvmx_gpio_tx_clr_cn61xx { pub bits: u64 }
// clr:24/16/20 with corresponding reserved high bits.
cvmx_gpio_union!(cvmx_gpio_tx_clr, cvmx_gpio_tx_clr_s, cvmx_gpio_tx_clr_cn38xx, cvmx_gpio_tx_clr_cn61xx);
#[repr(C)] pub struct cvmx_gpio_tx_set_s { pub bits: u64 }
#[repr(C)] pub struct cvmx_gpio_tx_set_cn38xx { pub bits: u64 }
#[repr(C)] pub struct cvmx_gpio_tx_set_cn61xx { pub bits: u64 }
// set:24/16/20 with corresponding reserved high bits.
cvmx_gpio_union!(cvmx_gpio_tx_set, cvmx_gpio_tx_set_s, cvmx_gpio_tx_set_cn38xx, cvmx_gpio_tx_set_cn61xx);
#[repr(C)] pub struct cvmx_gpio_xbit_cfgx_s { pub bits: u64 }
#[repr(C)] pub struct cvmx_gpio_xbit_cfgx_cn30xx { pub bits: u64 }
// tx_oe:1, rx_xor:1, int_en:1, int_type:1, fil_cnt:4, fil_sel:4, clk_sel:2, clk_gen:1, synce_sel:2, reserved_17_63:47.
// cn30xx: tx_oe:1, rx_xor:1, reserved_2_3:2, fil_cnt:4, fil_sel:4, reserved_12_63:52.
cvmx_gpio_union!(cvmx_gpio_xbit_cfgx, cvmx_gpio_xbit_cfgx_s, cvmx_gpio_xbit_cfgx_cn30xx);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
