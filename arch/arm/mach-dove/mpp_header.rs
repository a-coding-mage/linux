/* SPDX-License-Identifier: GPL-2.0 */

pub const fn MPP(num: u32, sel: u32, input: u32, output: u32) -> u32 {
    (num & 0xff) | ((sel & 0xf) << 8) | (((input != 0) as u32) << 12) | (((output != 0) as u32) << 13)
}

pub const fn MPP_GRP(grp: u32, mode: u32) -> u32 { MPP(grp, mode, 0, 0) }
pub const MPP0_GPIO0:u32=MPP(0,0,1,1); pub const MPP0_UA2_RTSn:u32=MPP(0,2,0,0); pub const MPP0_SDIO0_CD:u32=MPP(0,3,0,0); pub const MPP0_LCD0_PWM:u32=MPP(0,15,0,0);
pub const MPP1_GPIO1:u32=MPP(1,0,1,1); pub const MPP1_UA2_CTSn:u32=MPP(1,2,0,0); pub const MPP1_SDIO0_WP:u32=MPP(1,3,0,0); pub const MPP1_LCD1_PWM:u32=MPP(1,15,0,0);
pub const MPP2_GPIO2:u32=MPP(2,0,1,1); pub const MPP2_SATA_PRESENT:u32=MPP(2,1,0,0); pub const MPP2_UA2_TXD:u32=MPP(2,2,0,0); pub const MPP2_SDIO0_BUS_POWER:u32=MPP(2,3,0,0); pub const MPP2_UA_RTSn1:u32=MPP(2,4,0,0);
pub const MPP3_GPIO3:u32=MPP(3,0,1,1); pub const MPP3_SATA_ACT:u32=MPP(3,1,0,0); pub const MPP3_UA2_RXD:u32=MPP(3,2,0,0); pub const MPP3_SDIO0_LED_CTRL:u32=MPP(3,3,0,0); pub const MPP3_UA_CTSn1:u32=MPP(3,4,0,0); pub const MPP3_SPI_LCD_CS1:u32=MPP(3,15,0,0);
pub const MPP4_GPIO4:u32=MPP(4,0,1,1); pub const MPP4_UA3_RTSn:u32=MPP(4,2,0,0); pub const MPP4_SDIO1_CD:u32=MPP(4,3,0,0); pub const MPP4_SPI_1_MISO:u32=MPP(4,4,0,0);
pub const MPP5_GPIO5:u32=MPP(5,0,1,1); pub const MPP5_UA3_CTSn:u32=MPP(5,2,0,0); pub const MPP5_SDIO1_WP:u32=MPP(5,3,0,0); pub const MPP5_SPI_1_CS:u32=MPP(5,4,0,0);
pub const MPP6_GPIO6:u32=MPP(6,0,1,1); pub const MPP6_UA3_TXD:u32=MPP(6,2,0,0); pub const MPP6_SDIO1_BUS_POWER:u32=MPP(6,3,0,0); pub const MPP6_SPI_1_MOSI:u32=MPP(6,4,0,0);
pub const MPP7_GPIO7:u32=MPP(7,0,1,1); pub const MPP7_UA3_RXD:u32=MPP(7,2,0,0); pub const MPP7_SDIO1_LED_CTRL:u32=MPP(7,3,0,0); pub const MPP7_SPI_1_SCK:u32=MPP(7,4,0,0);
pub const MPP8_GPIO8:u32=MPP(8,0,1,1); pub const MPP8_WD_RST_OUT:u32=MPP(8,1,0,0); pub const MPP9_GPIO9:u32=MPP(9,0,1,1); pub const MPP9_PEX1_CLKREQn:u32=MPP(9,5,0,0); pub const MPP10_GPIO10:u32=MPP(10,0,1,1); pub const MPP10_SSP_SCLK:u32=MPP(10,5,0,0);
pub const MPP_MAX:u32=23;
pub const MPP11_GPIO11:u32=MPP(11,0,1,1); pub const MPP11_SATA_PRESENT:u32=MPP(11,1,0,0); pub const MPP11_SATA_ACT:u32=MPP(11,2,0,0); pub const MPP11_SDIO0_LED_CTRL:u32=MPP(11,3,0,0); pub const MPP11_SDIO1_LED_CTRL:u32=MPP(11,4,0,0); pub const MPP11_PEX0_CLKREQn:u32=MPP(11,5,0,0);
pub const MPP12_GPIO12:u32=MPP(12,0,1,1); pub const MPP12_SATA_ACT:u32=MPP(12,1,0,0); pub const MPP12_UA2_RTSn:u32=MPP(12,2,0,0); pub const MPP12_AD0_I2S_EXT_MCLK:u32=MPP(12,3,0,0); pub const MPP12_SDIO1_CD:u32=MPP(12,4,0,0);
pub const MPP13_GPIO13:u32=MPP(13,0,1,1); pub const MPP13_UA2_CTSn:u32=MPP(13,2,0,0); pub const MPP13_AD1_I2S_EXT_MCLK:u32=MPP(13,3,0,0); pub const MPP13_SDIO1WP:u32=MPP(13,4,0,0); pub const MPP13_SSP_EXTCLK:u32=MPP(13,5,0,0);
pub const MPP14_GPIO14:u32=MPP(14,0,1,1); pub const MPP14_UA2_TXD:u32=MPP(14,2,0,0); pub const MPP14_SDIO1_BUS_POWER:u32=MPP(14,4,0,0); pub const MPP14_SSP_RXD:u32=MPP(14,5,0,0);
pub const MPP15_GPIO15:u32=MPP(15,0,1,1); pub const MPP15_UA2_RXD:u32=MPP(15,2,0,0); pub const MPP15_SDIO1_LED_CTRL:u32=MPP(15,4,0,0); pub const MPP15_SSP_SFRM:u32=MPP(15,5,0,0);
pub const MPP16_GPIO16:u32=MPP(16,0,1,1); pub const MPP16_UA3_RTSn:u32=MPP(16,2,0,0); pub const MPP16_SDIO0_CD:u32=MPP(16,3,0,0); pub const MPP16_SPI_LCD_CS1:u32=MPP(16,4,0,0); pub const MPP16_AC97_SDATA_IN1:u32=MPP(16,5,0,0);
pub const MPP17_GPIO17:u32=MPP(17,0,1,1); pub const MPP17_AC97_SYSCLK_OUT:u32=MPP(17,1,0,0); pub const MPP17_UA3_CTSn:u32=MPP(17,2,0,0); pub const MPP17_SDIO0_WP:u32=MPP(17,3,0,0); pub const MPP17_TW_SDA2:u32=MPP(17,4,0,0); pub const MPP17_AC97_SDATA_IN2:u32=MPP(17,5,0,0);
pub const MPP18_GPIO18:u32=MPP(18,0,1,1); pub const MPP18_UA3_TXD:u32=MPP(18,2,0,0); pub const MPP18_SDIO0_BUS_POWER:u32=MPP(18,3,0,0); pub const MPP18_LCD0_PWM:u32=MPP(18,4,0,0); pub const MPP18_AC_SDATA_IN3:u32=MPP(18,5,0,0);
pub const MPP19_GPIO19:u32=MPP(19,0,1,1); pub const MPP19_UA3_RXD:u32=MPP(19,2,0,0); pub const MPP19_SDIO0_LED_CTRL:u32=MPP(19,3,0,0); pub const MPP19_TW_SCK2:u32=MPP(19,4,0,0);
pub const MPP20_GPIO20:u32=MPP(20,0,1,1); pub const MPP20_AC97_SYSCLK_OUT:u32=MPP(20,1,0,0); pub const MPP20_SPI_LCD_MISO:u32=MPP(20,2,0,0); pub const MPP20_SDIO1_CD:u32=MPP(20,3,0,0); pub const MPP20_SDIO0_CD:u32=MPP(20,5,0,0); pub const MPP20_SPI_1_MISO:u32=MPP(20,6,0,0);
pub const MPP21_GPIO21:u32=MPP(21,0,1,1); pub const MPP21_UA1_RTSn:u32=MPP(21,1,0,0); pub const MPP21_SPI_LCD_CS0:u32=MPP(21,2,0,0); pub const MPP21_SDIO1_WP:u32=MPP(21,3,0,0); pub const MPP21_SSP_SFRM:u32=MPP(21,4,0,0); pub const MPP21_SDIO0_WP:u32=MPP(21,5,0,0); pub const MPP21_SPI_1_CS:u32=MPP(21,6,0,0);
pub const MPP22_GPIO22:u32=MPP(22,0,1,1); pub const MPP22_UA1_CTSn:u32=MPP(22,1,0,0); pub const MPP22_SPI_LCD_MOSI:u32=MPP(22,2,0,0); pub const MPP22_SDIO1_BUS_POWER:u32=MPP(22,3,0,0); pub const MPP22_SSP_TXD:u32=MPP(22,4,0,0); pub const MPP22_SDIO0_BUS_POWER:u32=MPP(22,5,0,0); pub const MPP22_SPI_1_MOSI:u32=MPP(22,6,0,0);
pub const MPP23_GPIO23:u32=MPP(23,0,1,1); pub const MPP23_SPI_LCD_SCK:u32=MPP(23,2,0,0); pub const MPP23_SDIO1_LED_CTRL:u32=MPP(23,3,0,0); pub const MPP23_SSP_SCLK:u32=MPP(23,4,0,0); pub const MPP23_SDIO0_LED_CTRL:u32=MPP(23,5,0,0); pub const MPP23_SPI_1_SCK:u32=MPP(23,6,0,0);
#[repr(C)] pub enum dove_mpp_grp_idx { MPP_24_39=2, MPP_40_45=0, MPP_46_51=1, MPP_58_61=5, MPP_62_63=4, MPP_GRP_MAX=5 }
pub const MPP_GRP_24_39_GPIO:u32=MPP_GRP(2,1); pub const MPP_GRP_24_39_CAM:u32=MPP_GRP(2,0);
pub const MPP_GRP_40_45_GPIO:u32=MPP_GRP(0,1); pub const MPP_GRP_40_45_SD0:u32=MPP_GRP(0,0);
pub const MPP_GRP_46_51_GPIO:u32=MPP_GRP(1,1); pub const MPP_GRP_46_51_SD1:u32=MPP_GRP(1,0);
pub const MPP_GRP_58_61_GPIO:u32=MPP_GRP(5,1); pub const MPP_GRP_58_61_SPI:u32=MPP_GRP(5,0);
pub const MPP_GRP_62_63_GPIO:u32=MPP_GRP(4,1); pub const MPP_GRP_62_63_UA1:u32=MPP_GRP(4,0);
/* The MPP[64:71] control differs from other groups */
pub const MPP_GRP_NFC_64_71_GPO:u32=0x1; pub const MPP_GRP_NFC_64_71_NFC:u32=0x0;
/* The MPP[52:57] functionality is encoded by 4 bits in different registers. */
pub const MPP_GRP_AU1_52_57_AU1:u32=0x0; pub const MPP_GRP_AU1_52_57_AU1_GPIO57:u32=0x2; pub const MPP_GRP_AU1_52_57_GPIO:u32=0xa; pub const MPP_GRP_AU1_52_57_TW_GPIO:u32=0xb; pub const MPP_GRP_AU1_52_57_AU1_SSP:u32=0xc; pub const MPP_GRP_AU1_52_57_SSP_GPIO:u32=0xe; pub const MPP_GRP_AU1_52_57_SSP_TW:u32=0xf;
extern "C" { pub fn dove_mpp_conf(mpp_list:*mut u32, mpp_grp_list:*mut u32, grp_au1_52_57:u32, grp_nfc_64_71:u32); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
