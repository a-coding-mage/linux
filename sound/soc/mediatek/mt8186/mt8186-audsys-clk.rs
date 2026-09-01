// SPDX-License-Identifier: GPL-2.0
//
// mt8186-audsys-clk.h  --  Mediatek 8186 audsys clock control
//
// Copyright (c) 2022 MediaTek Inc.
// Author: Jiaxin Yu <jiaxin.yu@mediatek.com>

// C dependencies translated as external symbols/types expected from other files:
// linux/clk.h, linux/clk-provider.h, linux/clkdev.h
// mt8186-afe-common.h, mt8186-audsys-clk.h, mt8186-audsys-clkid.h, mt8186-reg.h

#[repr(C)]
struct afe_gate {
    id: ::core::ffi::c_int,
    name: *const ::core::ffi::c_char,
    parent_name: *const ::core::ffi::c_char,
    reg: ::core::ffi::c_int,
    bit: u8,
    ops: *const clk_ops,
    flags: ::core::ffi::c_ulong,
    cg_flags: u8,
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const ::core::ffi::c_char
    };
}

macro_rules! GATE_AFE_FLAGS {
    ($_id:expr, $_name:expr, $_parent:expr, $_reg:expr, $_bit:expr, $_flags:expr, $_cgflags:expr) => {
        afe_gate {
            id: $_id,
            name: c_str!($_name),
            parent_name: c_str!($_parent),
            reg: $_reg,
            bit: $_bit,
            ops: ::core::ptr::null(),
            flags: $_flags,
            cg_flags: $_cgflags,
        }
    };
}

macro_rules! GATE_AFE {
    ($_id:expr, $_name:expr, $_parent:expr, $_reg:expr, $_bit:expr) => {
        GATE_AFE_FLAGS!(
            $_id,
            $_name,
            $_parent,
            $_reg,
            $_bit,
            CLK_SET_RATE_PARENT,
            CLK_GATE_SET_TO_DISABLE
        )
    };
}

macro_rules! GATE_AUD0 {
    ($_id:expr, $_name:expr, $_parent:expr, $_bit:expr) => {
        GATE_AFE!($_id, $_name, $_parent, AUDIO_TOP_CON0, $_bit)
    };
}

macro_rules! GATE_AUD1 {
    ($_id:expr, $_name:expr, $_parent:expr, $_bit:expr) => {
        GATE_AFE!($_id, $_name, $_parent, AUDIO_TOP_CON1, $_bit)
    };
}

macro_rules! GATE_AUD2 {
    ($_id:expr, $_name:expr, $_parent:expr, $_bit:expr) => {
        GATE_AFE!($_id, $_name, $_parent, AUDIO_TOP_CON2, $_bit)
    };
}

static aud_clks: [afe_gate; CLK_AUD_NR_CLK as usize] = [
    /* AUD0 */
    GATE_AUD0!(CLK_AUD_AFE, "aud_afe_clk", "top_audio", 2),
    GATE_AUD0!(CLK_AUD_22M, "aud_apll22m_clk", "top_aud_engen1", 8),
    GATE_AUD0!(CLK_AUD_24M, "aud_apll24m_clk", "top_aud_engen2", 9),
    GATE_AUD0!(CLK_AUD_APLL2_TUNER, "aud_apll2_tuner_clk", "top_aud_engen2", 18),
    GATE_AUD0!(CLK_AUD_APLL_TUNER, "aud_apll_tuner_clk", "top_aud_engen1", 19),
    GATE_AUD0!(CLK_AUD_TDM, "aud_tdm_clk", "top_aud_1", 20),
    GATE_AUD0!(CLK_AUD_ADC, "aud_adc_clk", "top_audio", 24),
    GATE_AUD0!(CLK_AUD_DAC, "aud_dac_clk", "top_audio", 25),
    GATE_AUD0!(CLK_AUD_DAC_PREDIS, "aud_dac_predis_clk", "top_audio", 26),
    GATE_AUD0!(CLK_AUD_TML, "aud_tml_clk", "top_audio", 27),
    GATE_AUD0!(CLK_AUD_NLE, "aud_nle_clk", "top_audio", 28),

    /* AUD1 */
    GATE_AUD1!(CLK_AUD_I2S1_BCLK, "aud_i2s1_bclk", "top_audio", 4),
    GATE_AUD1!(CLK_AUD_I2S2_BCLK, "aud_i2s2_bclk", "top_audio", 5),
    GATE_AUD1!(CLK_AUD_I2S3_BCLK, "aud_i2s3_bclk", "top_audio", 6),
    GATE_AUD1!(CLK_AUD_I2S4_BCLK, "aud_i2s4_bclk", "top_audio", 7),
    GATE_AUD1!(CLK_AUD_CONNSYS_I2S_ASRC, "aud_connsys_i2s_asrc", "top_audio", 12),
    GATE_AUD1!(CLK_AUD_GENERAL1_ASRC, "aud_general1_asrc", "top_audio", 13),
    GATE_AUD1!(CLK_AUD_GENERAL2_ASRC, "aud_general2_asrc", "top_audio", 14),
    GATE_AUD1!(CLK_AUD_DAC_HIRES, "aud_dac_hires_clk", "top_audio_h", 15),
    GATE_AUD1!(CLK_AUD_ADC_HIRES, "aud_adc_hires_clk", "top_audio_h", 16),
    GATE_AUD1!(CLK_AUD_ADC_HIRES_TML, "aud_adc_hires_tml", "top_audio_h", 17),
    GATE_AUD1!(CLK_AUD_ADDA6_ADC, "aud_adda6_adc", "top_audio", 20),
    GATE_AUD1!(CLK_AUD_ADDA6_ADC_HIRES, "aud_adda6_adc_hires", "top_audio_h", 21),
    GATE_AUD1!(CLK_AUD_3RD_DAC, "aud_3rd_dac", "top_audio", 28),
    GATE_AUD1!(CLK_AUD_3RD_DAC_PREDIS, "aud_3rd_dac_predis", "top_audio", 29),
    GATE_AUD1!(CLK_AUD_3RD_DAC_TML, "aud_3rd_dac_tml", "top_audio", 30),
    GATE_AUD1!(CLK_AUD_3RD_DAC_HIRES, "aud_3rd_dac_hires", "top_audio_h", 31),

    /* AUD2 */
    GATE_AUD2!(CLK_AUD_ETDM_IN1_BCLK, "aud_etdm_in1_bclk", "top_audio", 23),
    GATE_AUD2!(CLK_AUD_ETDM_OUT1_BCLK, "aud_etdm_out1_bclk", "top_audio", 24),
];

unsafe extern "C" fn mt8186_audsys_clk_unregister(data: *mut ::core::ffi::c_void) {
    let afe: *mut mtk_base_afe = data as *mut mtk_base_afe;
    let afe_priv: *mut mt8186_afe_private = (*afe).platform_priv as *mut mt8186_afe_private;
    let mut clk: *mut clk;
    let mut cl: *mut clk_lookup;
    let mut i: ::core::ffi::c_int;

    if afe_priv.is_null() {
        return;
    }

    i = 0;
    while i < CLK_AUD_NR_CLK {
        cl = *(*afe_priv).lookup.offset(i as isize);
        if cl.is_null() {
            i += 1;
            continue;
        }

        clk = (*cl).clk;
        clk_unregister_gate(clk);

        clkdev_drop(cl);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt8186_audsys_clk_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int {
    let afe_priv: *mut mt8186_afe_private = (*afe).platform_priv as *mut mt8186_afe_private;
    let mut clk: *mut clk;
    let mut cl: *mut clk_lookup;
    let mut i: ::core::ffi::c_int;

    (*afe_priv).lookup = devm_kcalloc(
        (*afe).dev,
        CLK_AUD_NR_CLK as usize,
        ::core::mem::size_of_val(&*(*afe_priv).lookup) as usize,
        GFP_KERNEL,
    ) as *mut *mut clk_lookup;

    if (*afe_priv).lookup.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while (i as usize) < aud_clks.len() {
        let gate: *const afe_gate = &aud_clks[i as usize];

        clk = clk_register_gate(
            (*afe).dev,
            (*gate).name,
            (*gate).parent_name,
            (*gate).flags,
            (*afe).base_addr.offset((*gate).reg as isize),
            (*gate).bit,
            (*gate).cg_flags,
            ::core::ptr::null_mut(),
        );

        if IS_ERR(clk as *const ::core::ffi::c_void) {
            dev_err(
                (*afe).dev,
                c_str!("Failed to register clk %s: %ld\n"),
                (*gate).name,
                PTR_ERR(clk as *const ::core::ffi::c_void),
            );
            i += 1;
            continue;
        }

        /* add clk_lookup for devm_clk_get(SND_SOC_DAPM_CLOCK_SUPPLY) */
        cl = kzalloc_obj::<clk_lookup>();
        if cl.is_null() {
            return -ENOMEM;
        }

        (*cl).clk = clk;
        (*cl).con_id = (*gate).name;
        (*cl).dev_id = dev_name((*afe).dev);
        clkdev_add(cl);

        *(*afe_priv).lookup.offset(i as isize) = cl;
        i += 1;
    }

    devm_add_action_or_reset(
        (*afe).dev,
        Some(mt8186_audsys_clk_unregister),
        afe as *mut ::core::ffi::c_void,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
