// SPDX-License-Identifier: GPL-2.0-only
//
// Regmap tables and other data for Cirrus Logic CS48L32 audio DSP.
//
// Copyright (C) 2018, 2020, 2022, 2025 Cirrus Logic, Inc. and
//               Cirrus Logic International Semiconductor Ltd.
//
// Translated from C. Original include dependencies are expected to provide the
// Linux regmap, device, SPI, regulator, and CS48L32 register symbols.

use core::ffi::{c_int, c_uint};

static CS48L32_REVA_PATCH: &[reg_sequence] = &[
	reg_sequence { reg: 0x00001044, def: 0x0005000f },
	reg_sequence { reg: 0x00001c34, def: 0x000037e8 },
	reg_sequence { reg: 0x000046d8, def: 0x00000fe0 },
];

pub unsafe extern "C" fn cs48l32_apply_patch(cs48l32: *mut cs48l32) -> c_int
{
	let ret = regmap_register_patch((*cs48l32).regmap, CS48L32_REVA_PATCH.as_ptr(),
				    CS48L32_REVA_PATCH.len());
	if ret < 0 {
		return dev_err_probe((*cs48l32).dev, ret, c"Failed to apply patch\n".as_ptr());
	}

	0
}

static CS48L32_REG_DEFAULT: &[reg_default] = &[
	reg_default { reg: 0x00000c08, def: 0xe1000001 }, /* GPIO1_CTRL1 */
	reg_default { reg: 0x00000c0c, def: 0xe1000001 }, /* GPIO2_CTRL1 */
	reg_default { reg: 0x00000c10, def: 0xe1000001 }, /* GPIO3_CTRL1 */
	reg_default { reg: 0x00000c14, def: 0xe1000001 }, /* GPIO4_CTRL1 */
	reg_default { reg: 0x00000c18, def: 0xe1000001 }, /* GPIO5_CTRL1 */
	reg_default { reg: 0x00000c1c, def: 0xe1000001 }, /* GPIO6_CTRL1 */
	reg_default { reg: 0x00000c20, def: 0xe1000001 }, /* GPIO7_CTRL1 */
	reg_default { reg: 0x00000c24, def: 0xe1000001 }, /* GPIO8_CTRL1 */
	reg_default { reg: 0x00000c28, def: 0xe1000001 }, /* GPIO9_CTRL1 */
	reg_default { reg: 0x00000c2c, def: 0xe1000001 }, /* GPIO10_CTRL1 */
	reg_default { reg: 0x00000c30, def: 0xe1000001 }, /* GPIO11_CTRL1 */
	reg_default { reg: 0x00000c34, def: 0xe1000001 }, /* GPIO12_CTRL1 */
	reg_default { reg: 0x00000c38, def: 0xe1000001 }, /* GPIO13_CTRL1 */
	reg_default { reg: 0x00000c3c, def: 0xe1000001 }, /* GPIO14_CTRL1 */
	reg_default { reg: 0x00000c40, def: 0xe1000001 }, /* GPIO15_CTRL1 */
	reg_default { reg: 0x00000c44, def: 0xe1000001 }, /* GPIO16_CTRL1 */
	reg_default { reg: 0x00001020, def: 0x00000000 }, /* OUTPUT_SYS_CLK */
	reg_default { reg: 0x00001044, def: 0x0005000f }, /* AUXPDM_CTRL */
	reg_default { reg: 0x0000105c, def: 0x00000000 }, /* AUXPDM_CTRL2 */
	reg_default { reg: 0x00001400, def: 0x00000002 }, /* CLOCK32K */
	reg_default { reg: 0x00001404, def: 0x00000404 }, /* SYSTEM_CLOCK1 */
	reg_default { reg: 0x00001420, def: 0x00000003 }, /* SAMPLE_RATE1 */
	reg_default { reg: 0x00001424, def: 0x00000003 }, /* SAMPLE_RATE2 */
	reg_default { reg: 0x00001428, def: 0x00000003 }, /* SAMPLE_RATE3 */
	reg_default { reg: 0x0000142c, def: 0x00000003 }, /* SAMPLE_RATE4 */
	reg_default { reg: 0x00001c00, def: 0x00000002 }, /* FLL1_CONTROL1 */
	reg_default { reg: 0x00001c04, def: 0x88203004 }, /* FLL1_CONTROL2 */
	reg_default { reg: 0x00001c08, def: 0x00000000 }, /* FLL1_CONTROL3 */
	reg_default { reg: 0x00001c0c, def: 0x21f05001 }, /* FLL1_CONTROL4 */
	reg_default { reg: 0x00001ca0, def: 0x00000c04 }, /* FLL1_GPIO_CLOCK */
	reg_default { reg: 0x00002000, def: 0x00000006 }, /* CHARGE_PUMP1 */
	reg_default { reg: 0x00002408, def: 0x000003e4 }, /* LDO2_CTRL1 */
	reg_default { reg: 0x00002410, def: 0x000000e6 }, /* MICBIAS_CTRL1 */
	reg_default { reg: 0x00002418, def: 0x00000222 }, /* MICBIAS_CTRL5 */
	reg_default { reg: 0x00002710, def: 0x00004600 }, /* IRQ1_CTRL_AOD */
	reg_default { reg: 0x00004000, def: 0x00000000 }, /* INPUT_CONTROL */
	reg_default { reg: 0x00004008, def: 0x00000400 }, /* INPUT_RATE_CONTROL */
	reg_default { reg: 0x0000400c, def: 0x00000000 }, /* INPUT_CONTROL2 */
	reg_default { reg: 0x00004020, def: 0x00050020 }, /* INPUT1_CONTROL1 */
	reg_default { reg: 0x00004024, def: 0x00000000 }, /* IN1L_CONTROL1 */
	reg_default { reg: 0x00004028, def: 0x10800080 }, /* IN1L_CONTROL2 */
	reg_default { reg: 0x00004044, def: 0x00000000 }, /* IN1R_CONTROL1 */
	reg_default { reg: 0x00004048, def: 0x10800080 }, /* IN1R_CONTROL2 */
	reg_default { reg: 0x00004060, def: 0x00050020 }, /* INPUT2_CONTROL1 */
	reg_default { reg: 0x00004064, def: 0x00000000 }, /* IN2L_CONTROL1 */
	reg_default { reg: 0x00004068, def: 0x10800000 }, /* IN2L_CONTROL2 */
	reg_default { reg: 0x00004084, def: 0x00000000 }, /* IN2R_CONTROL1 */
	reg_default { reg: 0x00004088, def: 0x10800000 }, /* IN2R_CONTROL2 */
	reg_default { reg: 0x00004244, def: 0x00000002 }, /* INPUT_HPF_CONTROL */
	reg_default { reg: 0x00004248, def: 0x00000022 }, /* INPUT_VOL_CONTROL */
	reg_default { reg: 0x00004300, def: 0x00000000 }, /* AUXPDM_CONTROL1 */
	reg_default { reg: 0x00004304, def: 0x00000000 }, /* AUXPDM_CONTROL2 */
	reg_default { reg: 0x00004308, def: 0x00010008 }, /* AUXPDM1_CONTROL1 */
	reg_default { reg: 0x00004310, def: 0x00010008 }, /* AUXPDM2_CONTROL1 */
	reg_default { reg: 0x00004688, def: 0x00000000 }, /* ADC1L_ANA_CONTROL1 */
	reg_default { reg: 0x0000468c, def: 0x00000000 }, /* ADC1R_ANA_CONTROL1 */
	reg_default { reg: 0x00006000, def: 0x00000000 }, /* ASP1_ENABLES1 */
	reg_default { reg: 0x00006004, def: 0x00000028 }, /* ASP1_CONTROL1 */
	reg_default { reg: 0x00006008, def: 0x18180200 }, /* ASP1_CONTROL2 */
	reg_default { reg: 0x0000600c, def: 0x00000002 }, /* ASP1_CONTROL3 */
	reg_default { reg: 0x00006010, def: 0x03020100 }, /* ASP1_FRAME_CONTROL1 */
	reg_default { reg: 0x00006014, def: 0x07060504 }, /* ASP1_FRAME_CONTROL2 */
	reg_default { reg: 0x00006020, def: 0x03020100 }, /* ASP1_FRAME_CONTROL5 */
	reg_default { reg: 0x00006024, def: 0x07060504 }, /* ASP1_FRAME_CONTROL6 */
	reg_default { reg: 0x00006030, def: 0x00000020 }, /* ASP1_DATA_CONTROL1 */
	reg_default { reg: 0x00006040, def: 0x00000020 }, /* ASP1_DATA_CONTROL5 */
	reg_default { reg: 0x00006080, def: 0x00000000 }, /* ASP2_ENABLES1 */
	reg_default { reg: 0x00006084, def: 0x00000028 }, /* ASP2_CONTROL1 */
	reg_default { reg: 0x00006088, def: 0x18180200 }, /* ASP2_CONTROL2 */
	reg_default { reg: 0x0000608c, def: 0x00000002 }, /* ASP2_CONTROL3 */
	reg_default { reg: 0x00006090, def: 0x03020100 }, /* ASP2_FRAME_CONTROL1 */
	reg_default { reg: 0x000060a0, def: 0x03020100 }, /* ASP2_FRAME_CONTROL5 */
	reg_default { reg: 0x000060b0, def: 0x00000020 }, /* ASP2_DATA_CONTROL1 */
	reg_default { reg: 0x000060c0, def: 0x00000020 }, /* ASP2_DATA_CONTROL5 */
	reg_default { reg: 0x00008200, def: 0x00800000 }, /* ASP1TX1_INPUT1 */
	reg_default { reg: 0x00008204, def: 0x00800000 }, /* ASP1TX1_INPUT2 */
	reg_default { reg: 0x00008208, def: 0x00800000 }, /* ASP1TX1_INPUT3 */
	reg_default { reg: 0x0000820c, def: 0x00800000 }, /* ASP1TX1_INPUT4 */
	reg_default { reg: 0x00008210, def: 0x00800000 }, /* ASP1TX2_INPUT1 */
	reg_default { reg: 0x00008214, def: 0x00800000 }, /* ASP1TX2_INPUT2 */
	reg_default { reg: 0x00008218, def: 0x00800000 }, /* ASP1TX2_INPUT3 */
	reg_default { reg: 0x0000821c, def: 0x00800000 }, /* ASP1TX2_INPUT4 */
	reg_default { reg: 0x00008220, def: 0x00800000 }, /* ASP1TX3_INPUT1 */
	reg_default { reg: 0x00008224, def: 0x00800000 }, /* ASP1TX3_INPUT2 */
	reg_default { reg: 0x00008228, def: 0x00800000 }, /* ASP1TX3_INPUT3 */
	reg_default { reg: 0x0000822c, def: 0x00800000 }, /* ASP1TX3_INPUT4 */
	reg_default { reg: 0x00008230, def: 0x00800000 }, /* ASP1TX4_INPUT1 */
	reg_default { reg: 0x00008234, def: 0x00800000 }, /* ASP1TX4_INPUT2 */
	reg_default { reg: 0x00008238, def: 0x00800000 }, /* ASP1TX4_INPUT3 */
	reg_default { reg: 0x0000823c, def: 0x00800000 }, /* ASP1TX4_INPUT4 */
	reg_default { reg: 0x00008240, def: 0x00800000 }, /* ASP1TX5_INPUT1 */
	reg_default { reg: 0x00008244, def: 0x00800000 }, /* ASP1TX5_INPUT2 */
	reg_default { reg: 0x00008248, def: 0x00800000 }, /* ASP1TX5_INPUT3 */
	reg_default { reg: 0x0000824c, def: 0x00800000 }, /* ASP1TX5_INPUT4 */
	reg_default { reg: 0x00008250, def: 0x00800000 }, /* ASP1TX6_INPUT1 */
	reg_default { reg: 0x00008254, def: 0x00800000 }, /* ASP1TX6_INPUT2 */
	reg_default { reg: 0x00008258, def: 0x00800000 }, /* ASP1TX6_INPUT3 */
	reg_default { reg: 0x0000825c, def: 0x00800000 }, /* ASP1TX6_INPUT4 */
	reg_default { reg: 0x00008260, def: 0x00800000 }, /* ASP1TX7_INPUT1 */
	reg_default { reg: 0x00008264, def: 0x00800000 }, /* ASP1TX7_INPUT2 */
	reg_default { reg: 0x00008268, def: 0x00800000 }, /* ASP1TX7_INPUT3 */
	reg_default { reg: 0x0000826c, def: 0x00800000 }, /* ASP1TX7_INPUT4 */
	reg_default { reg: 0x00008270, def: 0x00800000 }, /* ASP1TX8_INPUT1 */
	reg_default { reg: 0x00008274, def: 0x00800000 }, /* ASP1TX8_INPUT2 */
	reg_default { reg: 0x00008278, def: 0x00800000 }, /* ASP1TX8_INPUT3 */
	reg_default { reg: 0x0000827c, def: 0x00800000 }, /* ASP1TX8_INPUT4 */
	reg_default { reg: 0x00008300, def: 0x00800000 }, /* ASP2TX1_INPUT1 */
	reg_default { reg: 0x00008304, def: 0x00800000 }, /* ASP2TX1_INPUT2 */
	reg_default { reg: 0x00008308, def: 0x00800000 }, /* ASP2TX1_INPUT3 */
	reg_default { reg: 0x0000830c, def: 0x00800000 }, /* ASP2TX1_INPUT4 */
	reg_default { reg: 0x00008310, def: 0x00800000 }, /* ASP2TX2_INPUT1 */
	reg_default { reg: 0x00008314, def: 0x00800000 }, /* ASP2TX2_INPUT2 */
	reg_default { reg: 0x00008318, def: 0x00800000 }, /* ASP2TX2_INPUT3 */
	reg_default { reg: 0x0000831c, def: 0x00800000 }, /* ASP2TX2_INPUT4 */
	reg_default { reg: 0x00008320, def: 0x00800000 }, /* ASP2TX3_INPUT1 */
	reg_default { reg: 0x00008324, def: 0x00800000 }, /* ASP2TX3_INPUT2 */
	reg_default { reg: 0x00008328, def: 0x00800000 }, /* ASP2TX3_INPUT3 */
	reg_default { reg: 0x0000832c, def: 0x00800000 }, /* ASP2TX3_INPUT4 */
	reg_default { reg: 0x00008330, def: 0x00800000 }, /* ASP2TX4_INPUT1 */
	reg_default { reg: 0x00008334, def: 0x00800000 }, /* ASP2TX4_INPUT2 */
	reg_default { reg: 0x00008338, def: 0x00800000 }, /* ASP2TX4_INPUT3 */
	reg_default { reg: 0x0000833c, def: 0x00800000 }, /* ASP2TX4_INPUT4 */
	reg_default { reg: 0x00008980, def: 0x00000000 }, /* ISRC1INT1_INPUT1 */
	reg_default { reg: 0x00008990, def: 0x00000000 }, /* ISRC1INT2_INPUT1 */
	reg_default { reg: 0x000089a0, def: 0x00000000 }, /* ISRC1INT3_INPUT1 */
	reg_default { reg: 0x000089b0, def: 0x00000000 }, /* ISRC1INT4_INPUT1 */
	reg_default { reg: 0x000089c0, def: 0x00000000 }, /* ISRC1DEC1_INPUT1 */
	reg_default { reg: 0x000089d0, def: 0x00000000 }, /* ISRC1DEC2_INPUT1 */
	reg_default { reg: 0x000089e0, def: 0x00000000 }, /* ISRC1DEC3_INPUT1 */
	reg_default { reg: 0x000089f0, def: 0x00000000 }, /* ISRC1DEC4_INPUT1 */
	reg_default { reg: 0x00008a00, def: 0x00000000 }, /* ISRC2INT1_INPUT1 */
	reg_default { reg: 0x00008a10, def: 0x00000000 }, /* ISRC2INT2_INPUT1 */
	reg_default { reg: 0x00008a40, def: 0x00000000 }, /* ISRC2DEC1_INPUT1 */
	reg_default { reg: 0x00008a50, def: 0x00000000 }, /* ISRC2DEC2_INPUT1 */
	reg_default { reg: 0x00008a80, def: 0x00000000 }, /* ISRC3INT1_INPUT1 */
	reg_default { reg: 0x00008a90, def: 0x00000000 }, /* ISRC3INT2_INPUT1 */
	reg_default { reg: 0x00008ac0, def: 0x00000000 }, /* ISRC3DEC1_INPUT1 */
	reg_default { reg: 0x00008ad0, def: 0x00000000 }, /* ISRC3DEC2_INPUT1 */
	reg_default { reg: 0x00008b80, def: 0x00800000 }, /* EQ1_INPUT1 */
	reg_default { reg: 0x00008b84, def: 0x00800000 }, /* EQ1_INPUT2 */
	reg_default { reg: 0x00008b88, def: 0x00800000 }, /* EQ1_INPUT3 */
	reg_default { reg: 0x00008b8c, def: 0x00800000 }, /* EQ1_INPUT4 */
	reg_default { reg: 0x00008b90, def: 0x00800000 }, /* EQ2_INPUT1 */
	reg_default { reg: 0x00008b94, def: 0x00800000 }, /* EQ2_INPUT2 */
	reg_default { reg: 0x00008b98, def: 0x00800000 }, /* EQ2_INPUT3 */
	reg_default { reg: 0x00008b9c, def: 0x00800000 }, /* EQ2_INPUT4 */
	reg_default { reg: 0x00008ba0, def: 0x00800000 }, /* EQ3_INPUT1 */
	reg_default { reg: 0x00008ba4, def: 0x00800000 }, /* EQ3_INPUT2 */
	reg_default { reg: 0x00008ba8, def: 0x00800000 }, /* EQ3_INPUT3 */
	reg_default { reg: 0x00008bac, def: 0x00800000 }, /* EQ3_INPUT4 */
	reg_default { reg: 0x00008bb0, def: 0x00800000 }, /* EQ4_INPUT1 */
	reg_default { reg: 0x00008bb4, def: 0x00800000 }, /* EQ4_INPUT2 */
	reg_default { reg: 0x00008bb8, def: 0x00800000 }, /* EQ4_INPUT3 */
	reg_default { reg: 0x00008bbc, def: 0x00800000 }, /* EQ4_INPUT4 */
	reg_default { reg: 0x00008c00, def: 0x00800000 }, /* DRC1L_INPUT1 */
	reg_default { reg: 0x00008c04, def: 0x00800000 }, /* DRC1L_INPUT2 */
	reg_default { reg: 0x00008c08, def: 0x00800000 }, /* DRC1L_INPUT3 */
	reg_default { reg: 0x00008c0c, def: 0x00800000 }, /* DRC1L_INPUT4 */
	reg_default { reg: 0x00008c10, def: 0x00800000 }, /* DRC1R_INPUT1 */
	reg_default { reg: 0x00008c14, def: 0x00800000 }, /* DRC1R_INPUT2 */
	reg_default { reg: 0x00008c18, def: 0x00800000 }, /* DRC1R_INPUT3 */
	reg_default { reg: 0x00008c1c, def: 0x00800000 }, /* DRC1R_INPUT4 */
	reg_default { reg: 0x00008c20, def: 0x00800000 }, /* DRC2L_INPUT1 */
	reg_default { reg: 0x00008c24, def: 0x00800000 }, /* DRC2L_INPUT2 */
	reg_default { reg: 0x00008c28, def: 0x00800000 }, /* DRC2L_INPUT3 */
	reg_default { reg: 0x00008c2c, def: 0x00800000 }, /* DRC2L_INPUT4 */
	reg_default { reg: 0x00008c30, def: 0x00800000 }, /* DRC2R_INPUT1 */
	reg_default { reg: 0x00008c34, def: 0x00800000 }, /* DRC2R_INPUT2 */
	reg_default { reg: 0x00008c38, def: 0x00800000 }, /* DRC2R_INPUT3 */
	reg_default { reg: 0x00008c3c, def: 0x00800000 }, /* DRC2R_INPUT4 */
	reg_default { reg: 0x00008c80, def: 0x00800000 }, /* LHPF1_INPUT1 */
	reg_default { reg: 0x00008c84, def: 0x00800000 }, /* LHPF1_INPUT2 */
	reg_default { reg: 0x00008c88, def: 0x00800000 }, /* LHPF1_INPUT3 */
	reg_default { reg: 0x00008c8c, def: 0x00800000 }, /* LHPF1_INPUT4 */
	reg_default { reg: 0x00008c90, def: 0x00800000 }, /* LHPF2_INPUT1 */
	reg_default { reg: 0x00008c94, def: 0x00800000 }, /* LHPF2_INPUT2 */
	reg_default { reg: 0x00008c98, def: 0x00800000 }, /* LHPF2_INPUT3 */
	reg_default { reg: 0x00008c9c, def: 0x00800000 }, /* LHPF2_INPUT4 */
	reg_default { reg: 0x00008ca0, def: 0x00800000 }, /* LHPF3_INPUT1 */
	reg_default { reg: 0x00008ca4, def: 0x00800000 }, /* LHPF3_INPUT2 */
	reg_default { reg: 0x00008ca8, def: 0x00800000 }, /* LHPF3_INPUT3 */
	reg_default { reg: 0x00008cac, def: 0x00800000 }, /* LHPF3_INPUT4 */
	reg_default { reg: 0x00008cb0, def: 0x00800000 }, /* LHPF4_INPUT1 */
	reg_default { reg: 0x00008cb4, def: 0x00800000 }, /* LHPF4_INPUT2 */
	reg_default { reg: 0x00008cb8, def: 0x00800000 }, /* LHPF4_INPUT3 */
	reg_default { reg: 0x00008cbc, def: 0x00800000 }, /* LHPF4_INPUT4 */
	reg_default { reg: 0x00009000, def: 0x00800000 }, /* DSP1RX1_INPUT1 */
	reg_default { reg: 0x00009004, def: 0x00800000 }, /* DSP1RX1_INPUT2 */
	reg_default { reg: 0x00009008, def: 0x00800000 }, /* DSP1RX1_INPUT3 */
	reg_default { reg: 0x0000900c, def: 0x00800000 }, /* DSP1RX1_INPUT4 */
	reg_default { reg: 0x00009010, def: 0x00800000 }, /* DSP1RX2_INPUT1 */
	reg_default { reg: 0x00009014, def: 0x00800000 }, /* DSP1RX2_INPUT2 */
	reg_default { reg: 0x00009018, def: 0x00800000 }, /* DSP1RX2_INPUT3 */
	reg_default { reg: 0x0000901c, def: 0x00800000 }, /* DSP1RX2_INPUT4 */
	reg_default { reg: 0x00009020, def: 0x00800000 }, /* DSP1RX3_INPUT1 */
	reg_default { reg: 0x00009024, def: 0x00800000 }, /* DSP1RX3_INPUT2 */
	reg_default { reg: 0x00009028, def: 0x00800000 }, /* DSP1RX3_INPUT3 */
	reg_default { reg: 0x0000902c, def: 0x00800000 }, /* DSP1RX3_INPUT4 */
	reg_default { reg: 0x00009030, def: 0x00800000 }, /* DSP1RX4_INPUT1 */
	reg_default { reg: 0x00009034, def: 0x00800000 }, /* DSP1RX4_INPUT2 */
	reg_default { reg: 0x00009038, def: 0x00800000 }, /* DSP1RX4_INPUT3 */
	reg_default { reg: 0x0000903c, def: 0x00800000 }, /* DSP1RX4_INPUT4 */
	reg_default { reg: 0x00009040, def: 0x00800000 }, /* DSP1RX5_INPUT1 */
	reg_default { reg: 0x00009044, def: 0x00800000 }, /* DSP1RX5_INPUT2 */
	reg_default { reg: 0x00009048, def: 0x00800000 }, /* DSP1RX5_INPUT3 */
	reg_default { reg: 0x0000904c, def: 0x00800000 }, /* DSP1RX5_INPUT4 */
	reg_default { reg: 0x00009050, def: 0x00800000 }, /* DSP1RX6_INPUT1 */
	reg_default { reg: 0x00009054, def: 0x00800000 }, /* DSP1RX6_INPUT2 */
	reg_default { reg: 0x00009058, def: 0x00800000 }, /* DSP1RX6_INPUT3 */
	reg_default { reg: 0x0000905c, def: 0x00800000 }, /* DSP1RX6_INPUT4 */
	reg_default { reg: 0x00009060, def: 0x00800000 }, /* DSP1RX7_INPUT1 */
	reg_default { reg: 0x00009064, def: 0x00800000 }, /* DSP1RX7_INPUT2 */
	reg_default { reg: 0x00009068, def: 0x00800000 }, /* DSP1RX7_INPUT3 */
	reg_default { reg: 0x0000906c, def: 0x00800000 }, /* DSP1RX7_INPUT4 */
	reg_default { reg: 0x00009070, def: 0x00800000 }, /* DSP1RX8_INPUT1 */
	reg_default { reg: 0x00009074, def: 0x00800000 }, /* DSP1RX8_INPUT2 */
	reg_default { reg: 0x00009078, def: 0x00800000 }, /* DSP1RX8_INPUT3 */
	reg_default { reg: 0x0000907c, def: 0x00800000 }, /* DSP1RX8_INPUT4 */
	reg_default { reg: 0x0000a400, def: 0x00000000 }, /* ISRC1_CONTROL1 */
	reg_default { reg: 0x0000a404, def: 0x00000000 }, /* ISRC1_CONTROL2 */
	reg_default { reg: 0x0000a510, def: 0x00000000 }, /* ISRC2_CONTROL1 */
	reg_default { reg: 0x0000a514, def: 0x00000000 }, /* ISRC2_CONTROL2 */
	reg_default { reg: 0x0000a620, def: 0x00000000 }, /* ISRC3_CONTROL1 */
	reg_default { reg: 0x0000a624, def: 0x00000000 }, /* ISRC3_CONTROL2 */
	reg_default { reg: 0x0000a800, def: 0x00000000 }, /* FX_SAMPLE_RATE */
	reg_default { reg: 0x0000a808, def: 0x00000000 }, /* EQ_CONTROL1 */
	reg_default { reg: 0x0000a80c, def: 0x00000000 }, /* EQ_CONTROL2 */
	reg_default { reg: 0x0000a810, def: 0x0c0c0c0c }, /* EQ1_GAIN1 */
	reg_default { reg: 0x0000a814, def: 0x0000000c }, /* EQ1_GAIN2 */
	reg_default { reg: 0x0000a818, def: 0x03fe0fc8 }, /* EQ1_BAND1_COEFF1 */
	reg_default { reg: 0x0000a81c, def: 0x00000b75 }, /* EQ1_BAND1_COEFF2 */
	reg_default { reg: 0x0000a820, def: 0x000000e0 }, /* EQ1_BAND1_PG */
	reg_default { reg: 0x0000a824, def: 0xf1361ec4 }, /* EQ1_BAND2_COEFF1 */
	reg_default { reg: 0x0000a828, def: 0x00000409 }, /* EQ1_BAND2_COEFF2 */
	reg_default { reg: 0x0000a82c, def: 0x000004cc }, /* EQ1_BAND2_PG */
	reg_default { reg: 0x0000a830, def: 0xf3371c9b }, /* EQ1_BAND3_COEFF1 */
	reg_default { reg: 0x0000a834, def: 0x0000040b }, /* EQ1_BAND3_COEFF2 */
	reg_default { reg: 0x0000a838, def: 0x00000cbb }, /* EQ1_BAND3_PG */
	reg_default { reg: 0x0000a83c, def: 0xf7d916f8 }, /* EQ1_BAND4_COEFF1 */
	reg_default { reg: 0x0000a840, def: 0x0000040a }, /* EQ1_BAND4_COEFF2 */
	reg_default { reg: 0x0000a844, def: 0x00001f14 }, /* EQ1_BAND4_PG */
	reg_default { reg: 0x0000a848, def: 0x0563058c }, /* EQ1_BAND5_COEFF1 */
	reg_default { reg: 0x0000a84c, def: 0x00000000 }, /* EQ1_BAND5_COEFF1 + 4 */
	reg_default { reg: 0x0000a850, def: 0x00004000 }, /* EQ1_BAND5_PG */
	reg_default { reg: 0x0000a854, def: 0x0c0c0c0c }, /* EQ2_GAIN1 */
	reg_default { reg: 0x0000a858, def: 0x0000000c }, /* EQ2_GAIN2 */
	reg_default { reg: 0x0000a85c, def: 0x03fe0fc8 }, /* EQ2_BAND1_COEFF1 */
	reg_default { reg: 0x0000a860, def: 0x00000b75 }, /* EQ2_BAND1_COEFF2 */
	reg_default { reg: 0x0000a864, def: 0x000000e0 }, /* EQ2_BAND1_PG */
	reg_default { reg: 0x0000a868, def: 0xf1361ec4 }, /* EQ2_BAND2_COEFF1 */
	reg_default { reg: 0x0000a86c, def: 0x00000409 }, /* EQ2_BAND2_COEFF2 */
	reg_default { reg: 0x0000a870, def: 0x000004cc }, /* EQ2_BAND2_PG */
	reg_default { reg: 0x0000a874, def: 0xf3371c9b }, /* EQ2_BAND3_COEFF1 */
	reg_default { reg: 0x0000a878, def: 0x0000040b }, /* EQ2_BAND3_COEFF2 */
	reg_default { reg: 0x0000a87c, def: 0x00000cbb }, /* EQ2_BAND3_PG */
	reg_default { reg: 0x0000a880, def: 0xf7d916f8 }, /* EQ2_BAND4_COEFF1 */
	reg_default { reg: 0x0000a884, def: 0x0000040a }, /* EQ2_BAND4_COEFF2 */
	reg_default { reg: 0x0000a888, def: 0x00001f14 }, /* EQ2_BAND4_PG */
	reg_default { reg: 0x0000a88c, def: 0x0563058c }, /* EQ2_BAND5_COEFF1 */
	reg_default { reg: 0x0000a890, def: 0x00000000 }, /* EQ2_BAND5_COEFF1 + 4 */
	reg_default { reg: 0x0000a894, def: 0x00004000 }, /* EQ2_BAND5_PG */
	reg_default { reg: 0x0000a898, def: 0x0c0c0c0c }, /* EQ3_GAIN1 */
	reg_default { reg: 0x0000a89c, def: 0x0000000c }, /* EQ3_GAIN2 */
	reg_default { reg: 0x0000a8a0, def: 0x03fe0fc8 }, /* EQ3_BAND1_COEFF1 */
	reg_default { reg: 0x0000a8a4, def: 0x00000b75 }, /* EQ3_BAND1_COEFF2 */
	reg_default { reg: 0x0000a8a8, def: 0x000000e0 }, /* EQ3_BAND1_PG */
	reg_default { reg: 0x0000a8ac, def: 0xf1361ec4 }, /* EQ3_BAND2_COEFF1 */
	reg_default { reg: 0x0000a8b0, def: 0x00000409 }, /* EQ3_BAND2_COEFF2 */
	reg_default { reg: 0x0000a8b4, def: 0x000004cc }, /* EQ3_BAND2_PG */
	reg_default { reg: 0x0000a8b8, def: 0xf3371c9b }, /* EQ3_BAND3_COEFF1 */
	reg_default { reg: 0x0000a8bc, def: 0x0000040b }, /* EQ3_BAND3_COEFF2 */
	reg_default { reg: 0x0000a8c0, def: 0x00000cbb }, /* EQ3_BAND3_PG */
	reg_default { reg: 0x0000a8c4, def: 0xf7d916f8 }, /* EQ3_BAND4_COEFF1 */
	reg_default { reg: 0x0000a8c8, def: 0x0000040a }, /* EQ3_BAND4_COEFF2 */
	reg_default { reg: 0x0000a8cc, def: 0x00001f14 }, /* EQ3_BAND4_PG */
	reg_default { reg: 0x0000a8d0, def: 0x0563058c }, /* EQ3_BAND5_COEFF1 */
	reg_default { reg: 0x0000a8d4, def: 0x00000000 }, /* EQ3_BAND5_COEFF1 + 4 */
	reg_default { reg: 0x0000a8d8, def: 0x00004000 }, /* EQ3_BAND5_PG */
	reg_default { reg: 0x0000a8dc, def: 0x0c0c0c0c }, /* EQ4_GAIN1 */
	reg_default { reg: 0x0000a8e0, def: 0x0000000c }, /* EQ4_GAIN2 */
	reg_default { reg: 0x0000a8e4, def: 0x03fe0fc8 }, /* EQ4_BAND1_COEFF1 */
	reg_default { reg: 0x0000a8e8, def: 0x00000b75 }, /* EQ4_BAND1_COEFF2 */
	reg_default { reg: 0x0000a8ec, def: 0x000000e0 }, /* EQ4_BAND1_PG */
	reg_default { reg: 0x0000a8f0, def: 0xf1361ec4 }, /* EQ4_BAND2_COEFF1 */
	reg_default { reg: 0x0000a8f4, def: 0x00000409 }, /* EQ4_BAND2_COEFF2 */
	reg_default { reg: 0x0000a8f8, def: 0x000004cc }, /* EQ4_BAND2_PG */
	reg_default { reg: 0x0000a8fc, def: 0xf3371c9b }, /* EQ4_BAND3_COEFF1 */
	reg_default { reg: 0x0000a900, def: 0x0000040b }, /* EQ4_BAND3_COEFF2 */
	reg_default { reg: 0x0000a904, def: 0x00000cbb }, /* EQ4_BAND3_PG */
	reg_default { reg: 0x0000a908, def: 0xf7d916f8 }, /* EQ4_BAND4_COEFF1 */
	reg_default { reg: 0x0000a90c, def: 0x0000040a }, /* EQ4_BAND4_COEFF2 */
	reg_default { reg: 0x0000a910, def: 0x00001f14 }, /* EQ4_BAND4_PG */
	reg_default { reg: 0x0000a914, def: 0x0563058c }, /* EQ4_BAND5_COEFF1 */
	reg_default { reg: 0x0000a918, def: 0x00000000 }, /* EQ4_BAND5_COEFF1 + 4 */
	reg_default { reg: 0x0000a91c, def: 0x00004000 }, /* EQ4_BAND5_PG */
	reg_default { reg: 0x0000aa30, def: 0x00000000 }, /* LHPF_CONTROL1 */
	reg_default { reg: 0x0000aa34, def: 0x00000000 }, /* LHPF_CONTROL2 */
	reg_default { reg: 0x0000aa38, def: 0x00000000 }, /* LHPF1_COEFF */
	reg_default { reg: 0x0000aa3c, def: 0x00000000 }, /* LHPF2_COEFF */
	reg_default { reg: 0x0000aa40, def: 0x00000000 }, /* LHPF3_COEFF */
	reg_default { reg: 0x0000aa44, def: 0x00000000 }, /* LHPF4_COEFF */
	reg_default { reg: 0x0000ab00, def: 0x00000000 }, /* DRC1_CONTROL1 */
	reg_default { reg: 0x0000ab04, def: 0x49130018 }, /* DRC1_CONTROL2 */
	reg_default { reg: 0x0000ab08, def: 0x00000018 }, /* DRC1_CONTROL3 */
	reg_default { reg: 0x0000ab0c, def: 0x00000000 }, /* DRC1_CONTROL4 */
	reg_default { reg: 0x0000ab14, def: 0x00000000 }, /* DRC2_CONTROL1 */
	reg_default { reg: 0x0000ab18, def: 0x49130018 }, /* DRC2_CONTROL2 */
	reg_default { reg: 0x0000ab1c, def: 0x00000018 }, /* DRC2_CONTROL3 */
	reg_default { reg: 0x0000ab20, def: 0x00000000 }, /* DRC2_CONTROL4 */
	reg_default { reg: 0x0000b000, def: 0x00000000 }, /* TONE_GENERATOR1 */
	reg_default { reg: 0x0000b004, def: 0x00100000 }, /* TONE_GENERATOR2 */
	reg_default { reg: 0x0000b400, def: 0x00000000 }, /* COMFORT_NOISE_GENERATOR */
	reg_default { reg: 0x0000b800, def: 0x00000000 }, /* US_CONTROL */
	reg_default { reg: 0x0000b804, def: 0x00002020 }, /* US1_CONTROL */
	reg_default { reg: 0x0000b808, def: 0x00000000 }, /* US1_DET_CONTROL */
	reg_default { reg: 0x0000b814, def: 0x00002020 }, /* US2_CONTROL */
	reg_default { reg: 0x0000b818, def: 0x00000000 }, /* US2_DET_CONTROL */
	reg_default { reg: 0x00018110, def: 0x00000700 }, /* IRQ1_MASK_1 */
	reg_default { reg: 0x00018114, def: 0x00000004 }, /* IRQ1_MASK_2 */
	reg_default { reg: 0x00018120, def: 0x03ff0000 }, /* IRQ1_MASK_5 */
	reg_default { reg: 0x00018124, def: 0x00000103 }, /* IRQ1_MASK_6 */
	reg_default { reg: 0x00018128, def: 0x003f0000 }, /* IRQ1_MASK_7 */
	reg_default { reg: 0x00018130, def: 0xff00000f }, /* IRQ1_MASK_9 */
	reg_default { reg: 0x00018138, def: 0xffff0000 }, /* IRQ1_MASK_11 */
];

unsafe extern "C" fn cs48l32_readable_register(_dev: *mut device, reg: c_uint) -> bool
{
	match reg {
	CS48L32_DEVID | CS48L32_REVID | CS48L32_OTPID | CS48L32_SFT_RESET | CS48L32_CTRL_IF_DEBUG3 | CS48L32_MCU_CTRL1 | CS48L32_GPIO1_CTRL1 ..= CS48L32_GPIO16_CTRL1 | CS48L32_OUTPUT_SYS_CLK | CS48L32_AUXPDM_CTRL | CS48L32_AUXPDM_CTRL2 | CS48L32_CLOCK32K | CS48L32_SYSTEM_CLOCK1 ..= CS48L32_SYSTEM_CLOCK2 | CS48L32_SAMPLE_RATE1 ..= CS48L32_SAMPLE_RATE4 | CS48L32_FLL1_CONTROL1 ..= CS48L32_FLL1_GPIO_CLOCK | CS48L32_CHARGE_PUMP1 | CS48L32_LDO2_CTRL1 | CS48L32_MICBIAS_CTRL1 | CS48L32_MICBIAS_CTRL5 | CS48L32_IRQ1_CTRL_AOD | CS48L32_INPUT_CONTROL | CS48L32_INPUT_STATUS | CS48L32_INPUT_RATE_CONTROL | CS48L32_INPUT_CONTROL2 | CS48L32_INPUT_CONTROL3 | CS48L32_INPUT1_CONTROL1 | CS48L32_IN1L_CONTROL1 ..= CS48L32_IN1L_CONTROL2 | CS48L32_IN1R_CONTROL1 ..= CS48L32_IN1R_CONTROL2 | CS48L32_INPUT2_CONTROL1 | CS48L32_IN2L_CONTROL1 ..= CS48L32_IN2L_CONTROL2 | CS48L32_IN2R_CONTROL1 ..= CS48L32_IN2R_CONTROL2 | CS48L32_INPUT_HPF_CONTROL | CS48L32_INPUT_VOL_CONTROL | CS48L32_AUXPDM_CONTROL1 | CS48L32_AUXPDM_CONTROL2 | CS48L32_AUXPDM1_CONTROL1 | CS48L32_AUXPDM2_CONTROL1 | CS48L32_ADC1L_ANA_CONTROL1 | CS48L32_ADC1R_ANA_CONTROL1 | CS48L32_ASP1_ENABLES1 ..= CS48L32_ASP1_DATA_CONTROL5 | CS48L32_ASP2_ENABLES1 ..= CS48L32_ASP2_DATA_CONTROL5 | CS48L32_ASP1TX1_INPUT1 ..= CS48L32_ASP1TX8_INPUT4 | CS48L32_ASP2TX1_INPUT1 ..= CS48L32_ASP2TX4_INPUT4 | CS48L32_ISRC1INT1_INPUT1 ..= CS48L32_ISRC1DEC4_INPUT1 | CS48L32_ISRC2INT1_INPUT1 ..= CS48L32_ISRC2DEC2_INPUT1 | CS48L32_ISRC3INT1_INPUT1 ..= CS48L32_ISRC3DEC2_INPUT1 | CS48L32_EQ1_INPUT1 ..= CS48L32_EQ4_INPUT4 | CS48L32_DRC1L_INPUT1 ..= CS48L32_DRC1R_INPUT4 | CS48L32_DRC2L_INPUT1 ..= CS48L32_DRC2R_INPUT4 | CS48L32_LHPF1_INPUT1 ..= CS48L32_LHPF1_INPUT4 | CS48L32_LHPF2_INPUT1 ..= CS48L32_LHPF2_INPUT4 | CS48L32_LHPF3_INPUT1 ..= CS48L32_LHPF3_INPUT4 | CS48L32_LHPF4_INPUT1 ..= CS48L32_LHPF4_INPUT4 | CS48L32_DSP1RX1_INPUT1 ..= CS48L32_DSP1RX8_INPUT4 | CS48L32_ISRC1_CONTROL1 ..= CS48L32_ISRC1_CONTROL2 | CS48L32_ISRC2_CONTROL1 ..= CS48L32_ISRC2_CONTROL2 | CS48L32_ISRC3_CONTROL1 ..= CS48L32_ISRC3_CONTROL2 | CS48L32_FX_SAMPLE_RATE | CS48L32_EQ_CONTROL1 ..= CS48L32_EQ_CONTROL2 | CS48L32_EQ1_GAIN1 ..= CS48L32_EQ1_BAND5_PG | CS48L32_EQ2_GAIN1 ..= CS48L32_EQ2_BAND5_PG | CS48L32_EQ3_GAIN1 ..= CS48L32_EQ3_BAND5_PG | CS48L32_EQ4_GAIN1 ..= CS48L32_EQ4_BAND5_PG | CS48L32_LHPF_CONTROL1 ..= CS48L32_LHPF_CONTROL2 | CS48L32_LHPF1_COEFF ..= CS48L32_LHPF4_COEFF | CS48L32_DRC1_CONTROL1 ..= CS48L32_DRC1_CONTROL4 | CS48L32_DRC2_CONTROL1 ..= CS48L32_DRC2_CONTROL4 | CS48L32_TONE_GENERATOR1 ..= CS48L32_TONE_GENERATOR2 | CS48L32_COMFORT_NOISE_GENERATOR | CS48L32_US_CONTROL | CS48L32_US1_CONTROL | CS48L32_US1_DET_CONTROL | CS48L32_US2_CONTROL | CS48L32_US2_DET_CONTROL | CS48L32_DSP1_XM_SRAM_IBUS_SETUP_0 ..= CS48L32_DSP1_XM_SRAM_IBUS_SETUP_24 | CS48L32_DSP1_YM_SRAM_IBUS_SETUP_0 ..= CS48L32_DSP1_YM_SRAM_IBUS_SETUP_8 | CS48L32_DSP1_PM_SRAM_IBUS_SETUP_0 ..= CS48L32_DSP1_PM_SRAM_IBUS_SETUP_7 | CS48L32_IRQ1_STATUS | CS48L32_IRQ1_EINT_1 ..= CS48L32_IRQ1_EINT_11 | CS48L32_IRQ1_STS_1 ..= CS48L32_IRQ1_STS_11 | CS48L32_IRQ1_MASK_1 ..= CS48L32_IRQ1_MASK_11 | CS48L32_DSP1_XMEM_PACKED_0 ..= CS48L32_DSP1_XMEM_PACKED_LAST | CS48L32_DSP1_SYS_INFO_ID ..= CS48L32_DSP1_AHBM_WINDOW_DEBUG_1 | CS48L32_DSP1_XMEM_UNPACKED24_0 ..= CS48L32_DSP1_XMEM_UNPACKED24_LAST | CS48L32_DSP1_CLOCK_FREQ ..= CS48L32_DSP1_SAMPLE_RATE_TX8 | CS48L32_DSP1_SCRATCH1 ..= CS48L32_DSP1_SCRATCH4 | CS48L32_DSP1_CCM_CORE_CONTROL ..= CS48L32_DSP1_STREAM_ARB_RESYNC_MSK1 | CS48L32_DSP1_YMEM_PACKED_0 ..= CS48L32_DSP1_YMEM_PACKED_LAST | CS48L32_DSP1_YMEM_UNPACKED24_0 ..= CS48L32_DSP1_YMEM_UNPACKED24_LAST | CS48L32_DSP1_PMEM_0 ..= CS48L32_DSP1_PMEM_LAST =>
		true,
	_ => false,
	}
}

unsafe extern "C" fn cs48l32_volatile_register(_dev: *mut device, reg: c_uint) -> bool
{
	match reg {
	CS48L32_DEVID | CS48L32_REVID | CS48L32_OTPID | CS48L32_SFT_RESET | CS48L32_CTRL_IF_DEBUG3 | CS48L32_MCU_CTRL1 | CS48L32_SYSTEM_CLOCK2 | CS48L32_FLL1_CONTROL5 | CS48L32_FLL1_CONTROL6 | CS48L32_INPUT_STATUS | CS48L32_INPUT_CONTROL3 | CS48L32_DSP1_XM_SRAM_IBUS_SETUP_0 ..= CS48L32_DSP1_XM_SRAM_IBUS_SETUP_24 | CS48L32_DSP1_YM_SRAM_IBUS_SETUP_0 ..= CS48L32_DSP1_YM_SRAM_IBUS_SETUP_8 | CS48L32_DSP1_PM_SRAM_IBUS_SETUP_0 ..= CS48L32_DSP1_PM_SRAM_IBUS_SETUP_7 | CS48L32_IRQ1_STATUS | CS48L32_IRQ1_EINT_1 ..= CS48L32_IRQ1_EINT_11 | CS48L32_IRQ1_STS_1 ..= CS48L32_IRQ1_STS_11 | CS48L32_DSP1_XMEM_PACKED_0 ..= CS48L32_DSP1_XMEM_PACKED_LAST | CS48L32_DSP1_SYS_INFO_ID ..= CS48L32_DSP1_AHBM_WINDOW_DEBUG_1 | CS48L32_DSP1_XMEM_UNPACKED24_0 ..= CS48L32_DSP1_XMEM_UNPACKED24_LAST | CS48L32_DSP1_CLOCK_FREQ ..= CS48L32_DSP1_SAMPLE_RATE_TX8 | CS48L32_DSP1_SCRATCH1 ..= CS48L32_DSP1_SCRATCH4 | CS48L32_DSP1_CCM_CORE_CONTROL ..= CS48L32_DSP1_STREAM_ARB_RESYNC_MSK1 | CS48L32_DSP1_YMEM_PACKED_0 ..= CS48L32_DSP1_YMEM_PACKED_LAST | CS48L32_DSP1_YMEM_UNPACKED24_0 ..= CS48L32_DSP1_YMEM_UNPACKED24_LAST | CS48L32_DSP1_PMEM_0 ..= CS48L32_DSP1_PMEM_LAST =>
		true,
	_ => false,
	}
}

/*
 * The bus bridge requires DSP packed memory registers to be accessed in
 * aligned block multiples.
 * Mark precious to prevent regmap debugfs causing an illegal bus transaction.
 */
unsafe extern "C" fn cs48l32_precious_register(_dev: *mut device, reg: c_uint) -> bool
{
	match reg {
	CS48L32_DSP1_XMEM_PACKED_0 ..= CS48L32_DSP1_XMEM_PACKED_LAST | CS48L32_DSP1_YMEM_PACKED_0 ..= CS48L32_DSP1_YMEM_PACKED_LAST | CS48L32_DSP1_PMEM_0 ..= CS48L32_DSP1_PMEM_LAST =>
		true,
	_ => false,
	}
}

static CS48L32_REGMAP: regmap_config = regmap_config {
	name: c"cs48l32".as_ptr(),
	reg_bits: 32,
	reg_stride: 4,
	pad_bits: 32,
	val_bits: 32,
	reg_format_endian: REGMAP_ENDIAN_BIG,
	val_format_endian: REGMAP_ENDIAN_BIG,

	max_register: CS48L32_DSP1_PMEM_LAST,
	readable_reg: Some(cs48l32_readable_register),
	volatile_reg: Some(cs48l32_volatile_register),
	precious_reg: Some(cs48l32_precious_register),

	cache_type: REGCACHE_MAPLE,
	reg_defaults: CS48L32_REG_DEFAULT.as_ptr(),
	num_reg_defaults: CS48L32_REG_DEFAULT.len(),
];

pub unsafe extern "C" fn cs48l32_create_regmap(spi: *mut spi_device, cs48l32: *mut cs48l32) -> c_int
{
	(*cs48l32).regmap = devm_regmap_init_spi(spi, &CS48L32_REGMAP);

	PTR_ERR_OR_ZERO((*cs48l32).regmap)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
