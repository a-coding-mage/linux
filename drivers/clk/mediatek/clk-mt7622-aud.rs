// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2017 MediaTek Inc.
 * Author: Chen Zhong <chen.zhong@mediatek.com>
 *	   Sean Wang <sean.wang@mediatek.com>
 */

// External kernel and MediaTek clock definitions are supplied by other files.

macro_rules! gate_audio0 {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        mtk_gate!($id, $name, $parent, &audio0_cg_regs, $shift, &mtk_clk_gate_ops_no_setclr)
    };
}

macro_rules! gate_audio1 {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        mtk_gate!($id, $name, $parent, &audio1_cg_regs, $shift, &mtk_clk_gate_ops_no_setclr)
    };
}

macro_rules! gate_audio2 {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        mtk_gate!($id, $name, $parent, &audio2_cg_regs, $shift, &mtk_clk_gate_ops_no_setclr)
    };
}

macro_rules! gate_audio3 {
    ($id:ident, $name:literal, $parent:literal, $shift:expr) => {
        mtk_gate!($id, $name, $parent, &audio3_cg_regs, $shift, &mtk_clk_gate_ops_no_setclr)
    };
}

static audio0_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x0,
    clr_ofs: 0x0,
    sta_ofs: 0x0,
};

static audio1_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x10,
    clr_ofs: 0x10,
    sta_ofs: 0x10,
};

static audio2_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x14,
    clr_ofs: 0x14,
    sta_ofs: 0x14,
};

static audio3_cg_regs: mtk_gate_regs = mtk_gate_regs {
    set_ofs: 0x634,
    clr_ofs: 0x634,
    sta_ofs: 0x634,
};

static audio_clks: [mtk_gate; 48] = [
    // AUDIO0
    gate_audio0!(CLK_AUDIO_AFE, "audio_afe", "rtc", 2),
    gate_audio0!(CLK_AUDIO_HDMI, "audio_hdmi", "apll1_ck_sel", 20),
    gate_audio0!(CLK_AUDIO_SPDF, "audio_spdf", "apll1_ck_sel", 21),
    gate_audio0!(CLK_AUDIO_APLL, "audio_apll", "apll1_ck_sel", 23),
    // AUDIO1
    gate_audio1!(CLK_AUDIO_I2SIN1, "audio_i2sin1", "a1sys_hp_sel", 0),
    gate_audio1!(CLK_AUDIO_I2SIN2, "audio_i2sin2", "a1sys_hp_sel", 1),
    gate_audio1!(CLK_AUDIO_I2SIN3, "audio_i2sin3", "a1sys_hp_sel", 2),
    gate_audio1!(CLK_AUDIO_I2SIN4, "audio_i2sin4", "a1sys_hp_sel", 3),
    gate_audio1!(CLK_AUDIO_I2SO1, "audio_i2so1", "a1sys_hp_sel", 6),
    gate_audio1!(CLK_AUDIO_I2SO2, "audio_i2so2", "a1sys_hp_sel", 7),
    gate_audio1!(CLK_AUDIO_I2SO3, "audio_i2so3", "a1sys_hp_sel", 8),
    gate_audio1!(CLK_AUDIO_I2SO4, "audio_i2so4", "a1sys_hp_sel", 9),
    gate_audio1!(CLK_AUDIO_ASRCI1, "audio_asrci1", "asm_h_sel", 12),
    gate_audio1!(CLK_AUDIO_ASRCI2, "audio_asrci2", "asm_h_sel", 13),
    gate_audio1!(CLK_AUDIO_ASRCO1, "audio_asrco1", "asm_h_sel", 14),
    gate_audio1!(CLK_AUDIO_ASRCO2, "audio_asrco2", "asm_h_sel", 15),
    gate_audio1!(CLK_AUDIO_INTDIR, "audio_intdir", "intdir_sel", 20),
    gate_audio1!(CLK_AUDIO_A1SYS, "audio_a1sys", "a1sys_hp_sel", 21),
    gate_audio1!(CLK_AUDIO_A2SYS, "audio_a2sys", "a2sys_hp_sel", 22),
    gate_audio1!(CLK_AUDIO_AFE_CONN, "audio_afe_conn", "a1sys_hp_sel", 23),
    gate_audio1!(CLK_AUDIO_AFE_MRGIF, "audio_afe_mrgif", "aud_mux1_sel", 25),
    // AUDIO2
    gate_audio2!(CLK_AUDIO_UL1, "audio_ul1", "a1sys_hp_sel", 0),
    gate_audio2!(CLK_AUDIO_UL2, "audio_ul2", "a1sys_hp_sel", 1),
    gate_audio2!(CLK_AUDIO_UL3, "audio_ul3", "a1sys_hp_sel", 2),
    gate_audio2!(CLK_AUDIO_UL4, "audio_ul4", "a1sys_hp_sel", 3),
    gate_audio2!(CLK_AUDIO_UL5, "audio_ul5", "a1sys_hp_sel", 4),
    gate_audio2!(CLK_AUDIO_UL6, "audio_ul6", "a1sys_hp_sel", 5),
    gate_audio2!(CLK_AUDIO_DL1, "audio_dl1", "a1sys_hp_sel", 6),
    gate_audio2!(CLK_AUDIO_DL2, "audio_dl2", "a1sys_hp_sel", 7),
    gate_audio2!(CLK_AUDIO_DL3, "audio_dl3", "a1sys_hp_sel", 8),
    gate_audio2!(CLK_AUDIO_DL4, "audio_dl4", "a1sys_hp_sel", 9),
    gate_audio2!(CLK_AUDIO_DL5, "audio_dl5", "a1sys_hp_sel", 10),
    gate_audio2!(CLK_AUDIO_DL6, "audio_dl6", "a1sys_hp_sel", 11),
    gate_audio2!(CLK_AUDIO_DLMCH, "audio_dlmch", "a1sys_hp_sel", 12),
    gate_audio2!(CLK_AUDIO_ARB1, "audio_arb1", "a1sys_hp_sel", 13),
    gate_audio2!(CLK_AUDIO_AWB, "audio_awb", "a1sys_hp_sel", 14),
    gate_audio2!(CLK_AUDIO_AWB2, "audio_awb2", "a1sys_hp_sel", 15),
    gate_audio2!(CLK_AUDIO_DAI, "audio_dai", "a1sys_hp_sel", 16),
    gate_audio2!(CLK_AUDIO_MOD, "audio_mod", "a1sys_hp_sel", 17),
    // AUDIO3
    gate_audio3!(CLK_AUDIO_ASRCI3, "audio_asrci3", "asm_h_sel", 2),
    gate_audio3!(CLK_AUDIO_ASRCI4, "audio_asrci4", "asm_h_sel", 3),
    gate_audio3!(CLK_AUDIO_ASRCO3, "audio_asrco3", "asm_h_sel", 6),
    gate_audio3!(CLK_AUDIO_ASRCO4, "audio_asrco4", "asm_h_sel", 7),
    gate_audio3!(CLK_AUDIO_MEM_ASRC1, "audio_mem_asrc1", "asm_h_sel", 10),
    gate_audio3!(CLK_AUDIO_MEM_ASRC2, "audio_mem_asrc2", "asm_h_sel", 11),
    gate_audio3!(CLK_AUDIO_MEM_ASRC3, "audio_mem_asrc3", "asm_h_sel", 12),
    gate_audio3!(CLK_AUDIO_MEM_ASRC4, "audio_mem_asrc4", "asm_h_sel", 13),
    gate_audio3!(CLK_AUDIO_MEM_ASRC5, "audio_mem_asrc5", "asm_h_sel", 14),
];

static audio_desc: mtk_clk_desc = mtk_clk_desc {
    clks: audio_clks.as_ptr(),
    num_clks: audio_clks.len(),
};

unsafe extern "C" fn clk_mt7622_aud_probe(pdev: *mut platform_device) -> c_int {
    let mut r: c_int;

    r = mtk_clk_simple_probe(pdev);
    if r != 0 {
        dev_err(
            &(*pdev).dev,
            "could not register clock provider: %s: %d\n",
            (*pdev).name,
            r,
        );
        return r;
    }

    r = devm_of_platform_populate(&(*pdev).dev);
    if r != 0 {
        mtk_clk_simple_remove(pdev);
        return r;
    }

    0
}

unsafe extern "C" fn clk_mt7622_aud_remove(pdev: *mut platform_device) {
    of_platform_depopulate(&(*pdev).dev);
    mtk_clk_simple_remove(pdev);
}

static of_match_clk_mt7622_aud: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt7622-audsys",
        data: &audio_desc,
    },
    of_device_id { sentinel: true },
];

static mut clk_mt7622_aud_drv: platform_driver = platform_driver {
    probe: Some(clk_mt7622_aud_probe),
    remove: Some(clk_mt7622_aud_remove),
    driver: device_driver {
        name: "clk-mt7622-aud",
        of_match_table: audio_desc_of_match(&of_match_clk_mt7622_aud),
    },
};

module_platform_driver!(clk_mt7622_aud_drv);

module_description!("MediaTek MT7622 audio clocks driver");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
