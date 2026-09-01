// SPDX-License-Identifier: GPL-2.0
/*
 * mt8195-audsys-clk.h  --  Mediatek 8195 audsys clock control
 *
 * Copyright (c) 2021 MediaTek Inc.
 * Author: Trevor Wu <trevor.wu@mediatek.com>
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut device,
    pub base_addr: *mut c_void,
    pub platform_priv: *mut mt8195_afe_private,
}

#[repr(C)]
pub struct mt8195_afe_private {
    pub lookup: *mut *mut clk_lookup,
}

#[repr(C)]
pub struct clk_lookup {
    pub clk: *mut clk,
    pub con_id: *const c_char,
    pub dev_id: *const c_char,
}

#[repr(C)]
struct afe_gate {
    id: c_int,
    name: *const c_char,
    parent_name: *const c_char,
    reg: c_int,
    bit: u8,
    ops: *const clk_ops,
    flags: c_ulong,
    cg_flags: u8,
}

unsafe impl Sync for afe_gate {}

extern "C" {
    static CLK_SET_RATE_PARENT: c_ulong;
    static CLK_GATE_SET_TO_DISABLE: u8;
    static GFP_KERNEL: c_ulong;
    static ENOMEM: c_int;

    static CLK_AUD_AFE: c_int;
    static CLK_AUD_LRCK_CNT: c_int;
    static CLK_AUD_SPDIFIN_TUNER_APLL: c_int;
    static CLK_AUD_SPDIFIN_TUNER_DBG: c_int;
    static CLK_AUD_UL_TML: c_int;
    static CLK_AUD_APLL1_TUNER: c_int;
    static CLK_AUD_APLL2_TUNER: c_int;
    static CLK_AUD_TOP0_SPDF: c_int;
    static CLK_AUD_APLL: c_int;
    static CLK_AUD_APLL2: c_int;
    static CLK_AUD_DAC: c_int;
    static CLK_AUD_DAC_PREDIS: c_int;
    static CLK_AUD_TML: c_int;
    static CLK_AUD_ADC: c_int;
    static CLK_AUD_DAC_HIRES: c_int;
    static CLK_AUD_A1SYS_HP: c_int;
    static CLK_AUD_AFE_DMIC1: c_int;
    static CLK_AUD_AFE_DMIC2: c_int;
    static CLK_AUD_AFE_DMIC3: c_int;
    static CLK_AUD_AFE_DMIC4: c_int;
    static CLK_AUD_AFE_26M_DMIC_TM: c_int;
    static CLK_AUD_UL_TML_HIRES: c_int;
    static CLK_AUD_ADC_HIRES: c_int;
    static CLK_AUD_ADDA6_ADC: c_int;
    static CLK_AUD_ADDA6_ADC_HIRES: c_int;
    static CLK_AUD_LINEIN_TUNER: c_int;
    static CLK_AUD_EARC_TUNER: c_int;
    static CLK_AUD_I2SIN: c_int;
    static CLK_AUD_TDM_IN: c_int;
    static CLK_AUD_I2S_OUT: c_int;
    static CLK_AUD_TDM_OUT: c_int;
    static CLK_AUD_HDMI_OUT: c_int;
    static CLK_AUD_ASRC11: c_int;
    static CLK_AUD_ASRC12: c_int;
    static CLK_AUD_MULTI_IN: c_int;
    static CLK_AUD_INTDIR: c_int;
    static CLK_AUD_A1SYS: c_int;
    static CLK_AUD_A2SYS: c_int;
    static CLK_AUD_PCMIF: c_int;
    static CLK_AUD_A3SYS: c_int;
    static CLK_AUD_A4SYS: c_int;
    static CLK_AUD_MEMIF_UL1: c_int;
    static CLK_AUD_MEMIF_UL2: c_int;
    static CLK_AUD_MEMIF_UL3: c_int;
    static CLK_AUD_MEMIF_UL4: c_int;
    static CLK_AUD_MEMIF_UL5: c_int;
    static CLK_AUD_MEMIF_UL6: c_int;
    static CLK_AUD_MEMIF_UL8: c_int;
    static CLK_AUD_MEMIF_UL9: c_int;
    static CLK_AUD_MEMIF_UL10: c_int;
    static CLK_AUD_MEMIF_DL2: c_int;
    static CLK_AUD_MEMIF_DL3: c_int;
    static CLK_AUD_MEMIF_DL6: c_int;
    static CLK_AUD_MEMIF_DL7: c_int;
    static CLK_AUD_MEMIF_DL8: c_int;
    static CLK_AUD_MEMIF_DL10: c_int;
    static CLK_AUD_MEMIF_DL11: c_int;
    static CLK_AUD_GASRC0: c_int;
    static CLK_AUD_GASRC1: c_int;
    static CLK_AUD_GASRC2: c_int;
    static CLK_AUD_GASRC3: c_int;
    static CLK_AUD_GASRC4: c_int;
    static CLK_AUD_GASRC5: c_int;
    static CLK_AUD_GASRC6: c_int;
    static CLK_AUD_GASRC7: c_int;
    static CLK_AUD_GASRC8: c_int;
    static CLK_AUD_GASRC9: c_int;
    static CLK_AUD_GASRC10: c_int;
    static CLK_AUD_GASRC11: c_int;
    static CLK_AUD_GASRC12: c_int;
    static CLK_AUD_GASRC13: c_int;
    static CLK_AUD_GASRC14: c_int;
    static CLK_AUD_GASRC15: c_int;
    static CLK_AUD_GASRC16: c_int;
    static CLK_AUD_GASRC17: c_int;
    static CLK_AUD_GASRC18: c_int;
    static CLK_AUD_GASRC19: c_int;
    static CLK_AUD_NR_CLK: c_int;

    static AUDIO_TOP_CON0: c_int;
    static AUDIO_TOP_CON1: c_int;
    static AUDIO_TOP_CON3: c_int;
    static AUDIO_TOP_CON4: c_int;
    static AUDIO_TOP_CON5: c_int;
    static AUDIO_TOP_CON6: c_int;

    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_ulong) -> *mut c_void;
    fn clk_register_gate(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        reg: *mut c_void,
        bit: u8,
        cg_flags: u8,
        lock: *mut c_void,
    ) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn kzalloc(size: usize, flags: c_ulong) -> *mut c_void;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn clkdev_add(cl: *mut clk_lookup);
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn clk_unregister_gate(clk: *mut clk);
    fn clkdev_drop(cl: *mut clk_lookup);
}

macro_rules! GATE_AFE_FLAGS {
    ($id:expr, $name:expr, $parent:expr, $reg:expr, $bit:expr, $flags:expr, $cgflags:expr) => {
        afe_gate {
            id: $id,
            name: $name.as_ptr() as *const c_char,
            parent_name: $parent.as_ptr() as *const c_char,
            reg: $reg,
            bit: $bit,
            ops: ptr::null(),
            flags: $flags,
            cg_flags: $cgflags,
        }
    };
}

macro_rules! GATE_AFE {
    ($id:expr, $name:expr, $parent:expr, $reg:expr, $bit:expr) => {
        GATE_AFE_FLAGS!(
            $id,
            $name,
            $parent,
            $reg,
            $bit,
            CLK_SET_RATE_PARENT,
            CLK_GATE_SET_TO_DISABLE
        )
    };
}

macro_rules! GATE_AUD0 {
    ($id:expr, $name:expr, $parent:expr, $bit:expr) => {
        GATE_AFE!($id, $name, $parent, AUDIO_TOP_CON0, $bit)
    };
}

macro_rules! GATE_AUD1 {
    ($id:expr, $name:expr, $parent:expr, $bit:expr) => {
        GATE_AFE!($id, $name, $parent, AUDIO_TOP_CON1, $bit)
    };
}

macro_rules! GATE_AUD3 {
    ($id:expr, $name:expr, $parent:expr, $bit:expr) => {
        GATE_AFE!($id, $name, $parent, AUDIO_TOP_CON3, $bit)
    };
}

macro_rules! GATE_AUD4 {
    ($id:expr, $name:expr, $parent:expr, $bit:expr) => {
        GATE_AFE!($id, $name, $parent, AUDIO_TOP_CON4, $bit)
    };
}

macro_rules! GATE_AUD5 {
    ($id:expr, $name:expr, $parent:expr, $bit:expr) => {
        GATE_AFE!($id, $name, $parent, AUDIO_TOP_CON5, $bit)
    };
}

macro_rules! GATE_AUD6 {
    ($id:expr, $name:expr, $parent:expr, $bit:expr) => {
        GATE_AFE!($id, $name, $parent, AUDIO_TOP_CON6, $bit)
    };
}

static aud_clks: [afe_gate; 83] = unsafe {
    [
        /* AUD0 */
        GATE_AUD0!(CLK_AUD_AFE, b"aud_afe\0", b"top_a1sys_hp\0", 2),
        GATE_AUD0!(CLK_AUD_LRCK_CNT, b"aud_lrck_cnt\0", b"top_a1sys_hp\0", 4),
        GATE_AUD0!(CLK_AUD_SPDIFIN_TUNER_APLL, b"aud_spdifin_tuner_apll\0", b"top_apll4\0", 10),
        GATE_AUD0!(CLK_AUD_SPDIFIN_TUNER_DBG, b"aud_spdifin_tuner_dbg\0", b"top_apll4\0", 11),
        GATE_AUD0!(CLK_AUD_UL_TML, b"aud_ul_tml\0", b"top_a1sys_hp\0", 18),
        GATE_AUD0!(CLK_AUD_APLL1_TUNER, b"aud_apll1_tuner\0", b"top_apll1\0", 19),
        GATE_AUD0!(CLK_AUD_APLL2_TUNER, b"aud_apll2_tuner\0", b"top_apll2\0", 20),
        GATE_AUD0!(CLK_AUD_TOP0_SPDF, b"aud_top0_spdf\0", b"top_aud_iec_clk\0", 21),
        GATE_AUD0!(CLK_AUD_APLL, b"aud_apll\0", b"top_apll1\0", 23),
        GATE_AUD0!(CLK_AUD_APLL2, b"aud_apll2\0", b"top_apll2\0", 24),
        GATE_AUD0!(CLK_AUD_DAC, b"aud_dac\0", b"top_a1sys_hp\0", 25),
        GATE_AUD0!(CLK_AUD_DAC_PREDIS, b"aud_dac_predis\0", b"top_a1sys_hp\0", 26),
        GATE_AUD0!(CLK_AUD_TML, b"aud_tml\0", b"top_a1sys_hp\0", 27),
        GATE_AUD0!(CLK_AUD_ADC, b"aud_adc\0", b"top_a1sys_hp\0", 28),
        GATE_AUD0!(CLK_AUD_DAC_HIRES, b"aud_dac_hires\0", b"top_audio_h\0", 31),

        /* AUD1 */
        GATE_AUD1!(CLK_AUD_A1SYS_HP, b"aud_a1sys_hp\0", b"top_a1sys_hp\0", 2),
        GATE_AUD1!(CLK_AUD_AFE_DMIC1, b"aud_afe_dmic1\0", b"top_a1sys_hp\0", 10),
        GATE_AUD1!(CLK_AUD_AFE_DMIC2, b"aud_afe_dmic2\0", b"top_a1sys_hp\0", 11),
        GATE_AUD1!(CLK_AUD_AFE_DMIC3, b"aud_afe_dmic3\0", b"top_a1sys_hp\0", 12),
        GATE_AUD1!(CLK_AUD_AFE_DMIC4, b"aud_afe_dmic4\0", b"top_a1sys_hp\0", 13),
        GATE_AUD1!(CLK_AUD_AFE_26M_DMIC_TM, b"aud_afe_26m_dmic_tm\0", b"top_a1sys_hp\0", 14),
        GATE_AUD1!(CLK_AUD_UL_TML_HIRES, b"aud_ul_tml_hires\0", b"top_audio_h\0", 16),
        GATE_AUD1!(CLK_AUD_ADC_HIRES, b"aud_adc_hires\0", b"top_audio_h\0", 17),
        GATE_AUD1!(CLK_AUD_ADDA6_ADC, b"aud_adda6_adc\0", b"top_a1sys_hp\0", 18),
        GATE_AUD1!(CLK_AUD_ADDA6_ADC_HIRES, b"aud_adda6_adc_hires\0", b"top_audio_h\0", 19),

        /* AUD3 */
        GATE_AUD3!(CLK_AUD_LINEIN_TUNER, b"aud_linein_tuner\0", b"top_apll5\0", 5),
        GATE_AUD3!(CLK_AUD_EARC_TUNER, b"aud_earc_tuner\0", b"top_apll3\0", 7),

        /* AUD4 */
        GATE_AUD4!(CLK_AUD_I2SIN, b"aud_i2sin\0", b"top_a1sys_hp\0", 0),
        GATE_AUD4!(CLK_AUD_TDM_IN, b"aud_tdm_in\0", b"top_a1sys_hp\0", 1),
        GATE_AUD4!(CLK_AUD_I2S_OUT, b"aud_i2s_out\0", b"top_a1sys_hp\0", 6),
        GATE_AUD4!(CLK_AUD_TDM_OUT, b"aud_tdm_out\0", b"top_a1sys_hp\0", 7),
        GATE_AUD4!(CLK_AUD_HDMI_OUT, b"aud_hdmi_out\0", b"top_a1sys_hp\0", 8),
        GATE_AUD4!(CLK_AUD_ASRC11, b"aud_asrc11\0", b"top_a1sys_hp\0", 16),
        GATE_AUD4!(CLK_AUD_ASRC12, b"aud_asrc12\0", b"top_a1sys_hp\0", 17),
        GATE_AUD4!(CLK_AUD_MULTI_IN, b"aud_multi_in\0", b"mphone_slave_b\0", 19),
        GATE_AUD4!(CLK_AUD_INTDIR, b"aud_intdir\0", b"top_intdir\0", 20),
        GATE_AUD4!(CLK_AUD_A1SYS, b"aud_a1sys\0", b"top_a1sys_hp\0", 21),
        GATE_AUD4!(CLK_AUD_A2SYS, b"aud_a2sys\0", b"top_a2sys_hf\0", 22),
        GATE_AUD4!(CLK_AUD_PCMIF, b"aud_pcmif\0", b"top_a1sys_hp\0", 24),
        GATE_AUD4!(CLK_AUD_A3SYS, b"aud_a3sys\0", b"top_a3sys_hf\0", 30),
        GATE_AUD4!(CLK_AUD_A4SYS, b"aud_a4sys\0", b"top_a4sys_hf\0", 31),

        /* AUD5 */
        GATE_AUD5!(CLK_AUD_MEMIF_UL1, b"aud_memif_ul1\0", b"top_a1sys_hp\0", 0),
        GATE_AUD5!(CLK_AUD_MEMIF_UL2, b"aud_memif_ul2\0", b"top_a1sys_hp\0", 1),
        GATE_AUD5!(CLK_AUD_MEMIF_UL3, b"aud_memif_ul3\0", b"top_a1sys_hp\0", 2),
        GATE_AUD5!(CLK_AUD_MEMIF_UL4, b"aud_memif_ul4\0", b"top_a1sys_hp\0", 3),
        GATE_AUD5!(CLK_AUD_MEMIF_UL5, b"aud_memif_ul5\0", b"top_a1sys_hp\0", 4),
        GATE_AUD5!(CLK_AUD_MEMIF_UL6, b"aud_memif_ul6\0", b"top_a1sys_hp\0", 5),
        GATE_AUD5!(CLK_AUD_MEMIF_UL8, b"aud_memif_ul8\0", b"top_a1sys_hp\0", 7),
        GATE_AUD5!(CLK_AUD_MEMIF_UL9, b"aud_memif_ul9\0", b"top_a1sys_hp\0", 8),
        GATE_AUD5!(CLK_AUD_MEMIF_UL10, b"aud_memif_ul10\0", b"top_a1sys_hp\0", 9),
        GATE_AUD5!(CLK_AUD_MEMIF_DL2, b"aud_memif_dl2\0", b"top_a1sys_hp\0", 18),
        GATE_AUD5!(CLK_AUD_MEMIF_DL3, b"aud_memif_dl3\0", b"top_a1sys_hp\0", 19),
        GATE_AUD5!(CLK_AUD_MEMIF_DL6, b"aud_memif_dl6\0", b"top_a1sys_hp\0", 22),
        GATE_AUD5!(CLK_AUD_MEMIF_DL7, b"aud_memif_dl7\0", b"top_a1sys_hp\0", 23),
        GATE_AUD5!(CLK_AUD_MEMIF_DL8, b"aud_memif_dl8\0", b"top_a1sys_hp\0", 24),
        GATE_AUD5!(CLK_AUD_MEMIF_DL10, b"aud_memif_dl10\0", b"top_a1sys_hp\0", 26),
        GATE_AUD5!(CLK_AUD_MEMIF_DL11, b"aud_memif_dl11\0", b"top_a1sys_hp\0", 27),

        /* AUD6 */
        GATE_AUD6!(CLK_AUD_GASRC0, b"aud_gasrc0\0", b"top_asm_h\0", 0),
        GATE_AUD6!(CLK_AUD_GASRC1, b"aud_gasrc1\0", b"top_asm_h\0", 1),
        GATE_AUD6!(CLK_AUD_GASRC2, b"aud_gasrc2\0", b"top_asm_h\0", 2),
        GATE_AUD6!(CLK_AUD_GASRC3, b"aud_gasrc3\0", b"top_asm_h\0", 3),
        GATE_AUD6!(CLK_AUD_GASRC4, b"aud_gasrc4\0", b"top_asm_h\0", 4),
        GATE_AUD6!(CLK_AUD_GASRC5, b"aud_gasrc5\0", b"top_asm_h\0", 5),
        GATE_AUD6!(CLK_AUD_GASRC6, b"aud_gasrc6\0", b"top_asm_h\0", 6),
        GATE_AUD6!(CLK_AUD_GASRC7, b"aud_gasrc7\0", b"top_asm_h\0", 7),
        GATE_AUD6!(CLK_AUD_GASRC8, b"aud_gasrc8\0", b"top_asm_h\0", 8),
        GATE_AUD6!(CLK_AUD_GASRC9, b"aud_gasrc9\0", b"top_asm_h\0", 9),
        GATE_AUD6!(CLK_AUD_GASRC10, b"aud_gasrc10\0", b"top_asm_h\0", 10),
        GATE_AUD6!(CLK_AUD_GASRC11, b"aud_gasrc11\0", b"top_asm_h\0", 11),
        GATE_AUD6!(CLK_AUD_GASRC12, b"aud_gasrc12\0", b"top_asm_h\0", 12),
        GATE_AUD6!(CLK_AUD_GASRC13, b"aud_gasrc13\0", b"top_asm_h\0", 13),
        GATE_AUD6!(CLK_AUD_GASRC14, b"aud_gasrc14\0", b"top_asm_h\0", 14),
        GATE_AUD6!(CLK_AUD_GASRC15, b"aud_gasrc15\0", b"top_asm_h\0", 15),
        GATE_AUD6!(CLK_AUD_GASRC16, b"aud_gasrc16\0", b"top_asm_h\0", 16),
        GATE_AUD6!(CLK_AUD_GASRC17, b"aud_gasrc17\0", b"top_asm_h\0", 17),
        GATE_AUD6!(CLK_AUD_GASRC18, b"aud_gasrc18\0", b"top_asm_h\0", 18),
        GATE_AUD6!(CLK_AUD_GASRC19, b"aud_gasrc19\0", b"top_asm_h\0", 19),
    ]
};

unsafe extern "C" fn mt8195_audsys_clk_unregister(data: *mut c_void) {
    let afe = data as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv;
    let mut clk: *mut clk;
    let mut cl: *mut clk_lookup;
    let mut i: c_int;

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
pub unsafe extern "C" fn mt8195_audsys_clk_register(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let mut clk: *mut clk;
    let mut cl: *mut clk_lookup;
    let mut i: c_int;

    (*afe_priv).lookup = devm_kcalloc(
        (*afe).dev,
        CLK_AUD_NR_CLK as usize,
        size_of::<*mut clk_lookup>(),
        GFP_KERNEL,
    ) as *mut *mut clk_lookup;

    if (*afe_priv).lookup.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while (i as usize) < aud_clks.len() {
        let gate = &aud_clks[i as usize] as *const afe_gate;

        clk = clk_register_gate(
            (*afe).dev,
            (*gate).name,
            (*gate).parent_name,
            (*gate).flags,
            ((*afe).base_addr as *mut u8).offset((*gate).reg as isize) as *mut c_void,
            (*gate).bit,
            (*gate).cg_flags,
            ptr::null_mut(),
        );

        if IS_ERR(clk as *const c_void) {
            dev_err(
                (*afe).dev,
                b"Failed to register clk %s: %ld\n\0".as_ptr() as *const c_char,
                (*gate).name,
                PTR_ERR(clk as *const c_void),
            );
            i += 1;
            continue;
        }

        /* add clk_lookup for devm_clk_get(SND_SOC_DAPM_CLOCK_SUPPLY) */
        cl = kzalloc(size_of::<clk_lookup>(), GFP_KERNEL) as *mut clk_lookup;
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
        mt8195_audsys_clk_unregister,
        afe as *mut c_void,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
