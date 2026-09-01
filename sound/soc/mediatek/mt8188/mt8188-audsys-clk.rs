// SPDX-License-Identifier: GPL-2.0
/*
 * mt8188-audsys-clk.c  --  MediaTek 8188 audsys clock control
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Chun-Chia Chiu <chun-chia.chiu@mediatek.com>
 */

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::null_mut;

// Dependencies from:
// <linux/clk.h>, <linux/clk-provider.h>, <linux/clkdev.h>,
// "mt8188-afe-common.h", "mt8188-audsys-clk.h",
// "mt8188-audsys-clkid.h", and "mt8188-reg.h".

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_lookup {
    pub list: *mut c_void,
    pub dev_id: *const c_char,
    pub con_id: *const c_char,
    pub clk: *mut clk,
    pub clk_hw: *mut clk_hw,
}

#[repr(C)]
pub struct mtk_base_afe {
    pub dev: *mut device,
    pub base_addr: *mut c_void,
    pub platform_priv: *mut mt8188_afe_private,
}

#[repr(C)]
pub struct mt8188_afe_private {
    pub lookup: *mut *mut clk_lookup,
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
    static GFP_KERNEL: c_int;
    static ENOMEM: c_int;

    static CLK_AUD_NR_CLK: c_int;
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
    static CLK_AUD_DMIC_HIRES1: c_int;
    static CLK_AUD_DMIC_HIRES2: c_int;
    static CLK_AUD_DMIC_HIRES3: c_int;
    static CLK_AUD_DMIC_HIRES4: c_int;
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

    static AUDIO_TOP_CON0: c_int;
    static AUDIO_TOP_CON1: c_int;
    static AUDIO_TOP_CON3: c_int;
    static AUDIO_TOP_CON4: c_int;
    static AUDIO_TOP_CON5: c_int;
    static AUDIO_TOP_CON6: c_int;

    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_int) -> *mut c_void;
    fn clk_unregister_gate(clk: *mut clk);
    fn clkdev_drop(cl: *mut clk_lookup);
    fn clk_register_gate(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        reg: *mut c_void,
        bit: u8,
        clk_gate_flags: u8,
        lock: *mut c_void,
    ) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn kzalloc(size: usize, flags: c_int) -> *mut c_void;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn clkdev_add(cl: *mut clk_lookup);
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
}

macro_rules! GATE_AFE_FLAGS {
    ($_id:expr, $_name:expr, $_parent:expr, $_reg:expr, $_bit:expr, $_flags:expr, $_cgflags:expr) => {
        afe_gate {
            id: $_id,
            name: $_name.as_ptr(),
            parent_name: $_parent.as_ptr(),
            reg: $_reg,
            bit: $_bit,
            ops: core::ptr::null(),
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

macro_rules! GATE_AUD3 {
    ($_id:expr, $_name:expr, $_parent:expr, $_bit:expr) => {
        GATE_AFE!($_id, $_name, $_parent, AUDIO_TOP_CON3, $_bit)
    };
}

macro_rules! GATE_AUD4 {
    ($_id:expr, $_name:expr, $_parent:expr, $_bit:expr) => {
        GATE_AFE!($_id, $_name, $_parent, AUDIO_TOP_CON4, $_bit)
    };
}

macro_rules! GATE_AUD5 {
    ($_id:expr, $_name:expr, $_parent:expr, $_bit:expr) => {
        GATE_AFE!($_id, $_name, $_parent, AUDIO_TOP_CON5, $_bit)
    };
}

macro_rules! GATE_AUD6 {
    ($_id:expr, $_name:expr, $_parent:expr, $_bit:expr) => {
        GATE_AFE!($_id, $_name, $_parent, AUDIO_TOP_CON6, $_bit)
    };
}

static aud_clks: [afe_gate; 73] = unsafe {
    [
        /* AUD0 */
        GATE_AUD0!(CLK_AUD_AFE, c"aud_afe", c"top_a1sys_hp", 2),
        GATE_AUD0!(CLK_AUD_LRCK_CNT, c"aud_lrck_cnt", c"top_a1sys_hp", 4),
        GATE_AUD0!(CLK_AUD_SPDIFIN_TUNER_APLL, c"aud_spdifin_tuner_apll", c"top_apll4", 10),
        GATE_AUD0!(CLK_AUD_SPDIFIN_TUNER_DBG, c"aud_spdifin_tuner_dbg", c"top_apll4", 11),
        GATE_AUD0!(CLK_AUD_UL_TML, c"aud_ul_tml", c"top_a1sys_hp", 18),
        GATE_AUD0!(CLK_AUD_APLL1_TUNER, c"aud_apll1_tuner", c"top_apll1", 19),
        GATE_AUD0!(CLK_AUD_APLL2_TUNER, c"aud_apll2_tuner", c"top_apll2", 20),
        GATE_AUD0!(CLK_AUD_TOP0_SPDF, c"aud_top0_spdf", c"top_aud_iec_clk", 21),
        GATE_AUD0!(CLK_AUD_APLL, c"aud_apll", c"top_apll1", 23),
        GATE_AUD0!(CLK_AUD_APLL2, c"aud_apll2", c"top_apll2", 24),
        GATE_AUD0!(CLK_AUD_DAC, c"aud_dac", c"top_a1sys_hp", 25),
        GATE_AUD0!(CLK_AUD_DAC_PREDIS, c"aud_dac_predis", c"top_a1sys_hp", 26),
        GATE_AUD0!(CLK_AUD_TML, c"aud_tml", c"top_a1sys_hp", 27),
        GATE_AUD0!(CLK_AUD_ADC, c"aud_adc", c"top_a1sys_hp", 28),
        GATE_AUD0!(CLK_AUD_DAC_HIRES, c"aud_dac_hires", c"top_audio_h", 31),

        /* AUD1 */
        GATE_AUD1!(CLK_AUD_A1SYS_HP, c"aud_a1sys_hp", c"top_a1sys_hp", 2),
        GATE_AUD1!(CLK_AUD_AFE_DMIC1, c"aud_afe_dmic1", c"top_a1sys_hp", 10),
        GATE_AUD1!(CLK_AUD_AFE_DMIC2, c"aud_afe_dmic2", c"top_a1sys_hp", 11),
        GATE_AUD1!(CLK_AUD_AFE_DMIC3, c"aud_afe_dmic3", c"top_a1sys_hp", 12),
        GATE_AUD1!(CLK_AUD_AFE_DMIC4, c"aud_afe_dmic4", c"top_a1sys_hp", 13),
        GATE_AUD1!(CLK_AUD_AFE_26M_DMIC_TM, c"aud_afe_26m_dmic_tm", c"top_a1sys_hp", 14),
        GATE_AUD1!(CLK_AUD_UL_TML_HIRES, c"aud_ul_tml_hires", c"top_audio_h", 16),
        GATE_AUD1!(CLK_AUD_ADC_HIRES, c"aud_adc_hires", c"top_audio_h", 17),
        GATE_AUD1!(CLK_AUD_DMIC_HIRES1, c"aud_dmic_hires1", c"top_audio_h", 20),
        GATE_AUD1!(CLK_AUD_DMIC_HIRES2, c"aud_dmic_hires2", c"top_audio_h", 21),
        GATE_AUD1!(CLK_AUD_DMIC_HIRES3, c"aud_dmic_hires3", c"top_audio_h", 22),
        GATE_AUD1!(CLK_AUD_DMIC_HIRES4, c"aud_dmic_hires4", c"top_audio_h", 23),

        /* AUD3 */
        GATE_AUD3!(CLK_AUD_LINEIN_TUNER, c"aud_linein_tuner", c"top_apll5", 5),
        GATE_AUD3!(CLK_AUD_EARC_TUNER, c"aud_earc_tuner", c"top_apll3", 7),

        /* AUD4 */
        GATE_AUD4!(CLK_AUD_I2SIN, c"aud_i2sin", c"top_a1sys_hp", 0),
        GATE_AUD4!(CLK_AUD_TDM_IN, c"aud_tdm_in", c"top_a1sys_hp", 1),
        GATE_AUD4!(CLK_AUD_I2S_OUT, c"aud_i2s_out", c"top_a1sys_hp", 6),
        GATE_AUD4!(CLK_AUD_TDM_OUT, c"aud_tdm_out", c"top_a1sys_hp", 7),
        GATE_AUD4!(CLK_AUD_HDMI_OUT, c"aud_hdmi_out", c"top_a1sys_hp", 8),
        GATE_AUD4!(CLK_AUD_ASRC11, c"aud_asrc11", c"top_a1sys_hp", 16),
        GATE_AUD4!(CLK_AUD_ASRC12, c"aud_asrc12", c"top_a1sys_hp", 17),
        GATE_AUD4!(CLK_AUD_MULTI_IN, c"aud_multi_in", c"mphone_slave_b", 19),
        GATE_AUD4!(CLK_AUD_INTDIR, c"aud_intdir", c"top_intdir", 20),
        GATE_AUD4!(CLK_AUD_A1SYS, c"aud_a1sys", c"top_a1sys_hp", 21),
        GATE_AUD4!(CLK_AUD_A2SYS, c"aud_a2sys", c"top_a2sys", 22),
        GATE_AUD4!(CLK_AUD_PCMIF, c"aud_pcmif", c"top_a1sys_hp", 24),
        GATE_AUD4!(CLK_AUD_A3SYS, c"aud_a3sys", c"top_a3sys", 30),
        GATE_AUD4!(CLK_AUD_A4SYS, c"aud_a4sys", c"top_a4sys", 31),

        /* AUD5 */
        GATE_AUD5!(CLK_AUD_MEMIF_UL1, c"aud_memif_ul1", c"top_a1sys_hp", 0),
        GATE_AUD5!(CLK_AUD_MEMIF_UL2, c"aud_memif_ul2", c"top_a1sys_hp", 1),
        GATE_AUD5!(CLK_AUD_MEMIF_UL3, c"aud_memif_ul3", c"top_a1sys_hp", 2),
        GATE_AUD5!(CLK_AUD_MEMIF_UL4, c"aud_memif_ul4", c"top_a1sys_hp", 3),
        GATE_AUD5!(CLK_AUD_MEMIF_UL5, c"aud_memif_ul5", c"top_a1sys_hp", 4),
        GATE_AUD5!(CLK_AUD_MEMIF_UL6, c"aud_memif_ul6", c"top_a1sys_hp", 5),
        GATE_AUD5!(CLK_AUD_MEMIF_UL8, c"aud_memif_ul8", c"top_a1sys_hp", 7),
        GATE_AUD5!(CLK_AUD_MEMIF_UL9, c"aud_memif_ul9", c"top_a1sys_hp", 8),
        GATE_AUD5!(CLK_AUD_MEMIF_UL10, c"aud_memif_ul10", c"top_a1sys_hp", 9),
        GATE_AUD5!(CLK_AUD_MEMIF_DL2, c"aud_memif_dl2", c"top_a1sys_hp", 18),
        GATE_AUD5!(CLK_AUD_MEMIF_DL3, c"aud_memif_dl3", c"top_a1sys_hp", 19),
        GATE_AUD5!(CLK_AUD_MEMIF_DL6, c"aud_memif_dl6", c"top_a1sys_hp", 22),
        GATE_AUD5!(CLK_AUD_MEMIF_DL7, c"aud_memif_dl7", c"top_a1sys_hp", 23),
        GATE_AUD5!(CLK_AUD_MEMIF_DL8, c"aud_memif_dl8", c"top_a1sys_hp", 24),
        GATE_AUD5!(CLK_AUD_MEMIF_DL10, c"aud_memif_dl10", c"top_a1sys_hp", 26),
        GATE_AUD5!(CLK_AUD_MEMIF_DL11, c"aud_memif_dl11", c"top_a1sys_hp", 27),

        /* AUD6 */
        GATE_AUD6!(CLK_AUD_GASRC0, c"aud_gasrc0", c"top_asm_h", 0),
        GATE_AUD6!(CLK_AUD_GASRC1, c"aud_gasrc1", c"top_asm_h", 1),
        GATE_AUD6!(CLK_AUD_GASRC2, c"aud_gasrc2", c"top_asm_h", 2),
        GATE_AUD6!(CLK_AUD_GASRC3, c"aud_gasrc3", c"top_asm_h", 3),
        GATE_AUD6!(CLK_AUD_GASRC4, c"aud_gasrc4", c"top_asm_h", 4),
        GATE_AUD6!(CLK_AUD_GASRC5, c"aud_gasrc5", c"top_asm_h", 5),
        GATE_AUD6!(CLK_AUD_GASRC6, c"aud_gasrc6", c"top_asm_h", 6),
        GATE_AUD6!(CLK_AUD_GASRC7, c"aud_gasrc7", c"top_asm_h", 7),
        GATE_AUD6!(CLK_AUD_GASRC8, c"aud_gasrc8", c"top_asm_h", 8),
        GATE_AUD6!(CLK_AUD_GASRC9, c"aud_gasrc9", c"top_asm_h", 9),
        GATE_AUD6!(CLK_AUD_GASRC10, c"aud_gasrc10", c"top_asm_h", 10),
        GATE_AUD6!(CLK_AUD_GASRC11, c"aud_gasrc11", c"top_asm_h", 11),
    ]
};

unsafe extern "C" fn mt8188_audsys_clk_unregister(data: *mut c_void) {
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
pub unsafe extern "C" fn mt8188_audsys_clk_register(afe: *mut mtk_base_afe) -> c_int {
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
            null_mut(),
        );

        if IS_ERR(clk as *const c_void) {
            dev_err(
                (*afe).dev,
                c"Failed to register clk %s: %ld\n".as_ptr(),
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
        (*cl).clk_hw = null_mut();
        clkdev_add(cl);

        *(*afe_priv).lookup.offset(i as isize) = cl;
        i += 1;
    }

    devm_add_action_or_reset(
        (*afe).dev,
        mt8188_audsys_clk_unregister,
        afe as *mut c_void,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
