/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2023 Inochi Amaoto <inochiama@outlook.com> */

/* Translated from clk-cv18xx-ip.h; dependencies are supplied by the common header. */

#[repr(C)]
pub struct cv1800_clk_gate {
    pub common: cv1800_clk_common,
    pub gate: cv1800_clk_regbit,
}

#[repr(C)]
pub struct cv1800_clk_div_data {
    pub reg: u32,
    pub mask: u32,
    pub width: u32,
    pub init: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct cv1800_clk_div {
    pub common: cv1800_clk_common,
    pub gate: cv1800_clk_regbit,
    pub div: cv1800_clk_regfield,
}

#[repr(C)]
pub struct cv1800_clk_bypass_div {
    pub div: cv1800_clk_div,
    pub bypass: cv1800_clk_regbit,
}

#[repr(C)]
pub struct cv1800_clk_mux {
    pub common: cv1800_clk_common,
    pub gate: cv1800_clk_regbit,
    pub div: cv1800_clk_regfield,
    pub mux: cv1800_clk_regfield,
}

#[repr(C)]
pub struct cv1800_clk_bypass_mux {
    pub mux: cv1800_clk_mux,
    pub bypass: cv1800_clk_regbit,
}

#[repr(C)]
pub struct cv1800_clk_mmux {
    pub common: cv1800_clk_common,
    pub gate: cv1800_clk_regbit,
    pub div: [cv1800_clk_regfield; 2],
    pub mux: [cv1800_clk_regfield; 2],
    pub bypass: cv1800_clk_regbit,
    pub clk_sel: cv1800_clk_regbit,
    pub parent2sel: *const i8,
    pub sel2parent: [*const u8; 2],
}

#[repr(C)]
pub struct cv1800_clk_audio {
    pub common: cv1800_clk_common,
    pub src_en: cv1800_clk_regbit,
    pub output_en: cv1800_clk_regbit,
    pub div_en: cv1800_clk_regbit,
    pub div_up: cv1800_clk_regbit,
    pub m: cv1800_clk_regfield,
    pub n: cv1800_clk_regfield,
    pub target_rate: u32,
}

#[macro_export]
macro_rules! CV1800_GATE {
    ($name:ident, $parent:expr, $gate_reg:expr, $gate_shift:expr, $flags:expr) => {
        let mut $name = cv1800_clk_gate {
            common: CV1800_CLK_COMMON!(stringify!($name), $parent, &cv1800_clk_gate_ops, $flags),
            gate: CV1800_CLK_BIT!($gate_reg, $gate_shift),
        };
    };
}

#[macro_export]
macro_rules! _CV1800_DIV {
    ($name:expr, $parent:expr, $gate_reg:expr, $gate_shift:expr, $div_reg:expr, $div_shift:expr, $div_width:expr, $div_init:expr, $div_flag:expr, $ops:expr, $flags:expr) => {
        cv1800_clk_div { common: CV1800_CLK_COMMON!(stringify!($name), $parent, $ops, $flags), gate: CV1800_CLK_BIT!($gate_reg, $gate_shift), div: CV1800_CLK_REG!($div_reg, $div_shift, $div_width, $div_init, $div_flag) }
    };
}

pub const _CV1800_FIXED_DIV_FLAG: u32 = CLK_DIVIDER_ONE_BASED | CLK_DIVIDER_ROUND_CLOSEST;

#[macro_export]
macro_rules! _CV1800_FIXED_DIV {
    ($name:expr, $parent:expr, $gate_reg:expr, $gate_shift:expr, $fix_div:expr, $ops:expr, $flags:expr) => {
        cv1800_clk_div { common: CV1800_CLK_COMMON!(stringify!($name), $parent, $ops, $flags), gate: CV1800_CLK_BIT!($gate_reg, $gate_shift), div: CV1800_CLK_REG!(0, 0, 0, $fix_div, _CV1800_FIXED_DIV_FLAG) }
    };
}

#[macro_export]
macro_rules! CV1800_DIV {
    ($name:ident, $parent:expr, $gate_reg:expr, $gate_shift:expr, $div_reg:expr, $div_shift:expr, $div_width:expr, $div_init:expr, $div_flag:expr, $flags:expr) => {
        let mut $name = _CV1800_DIV!(stringify!($name), $parent, $gate_reg, $gate_shift, $div_reg, $div_shift, $div_width, $div_init, $div_flag, &cv1800_clk_div_ops, $flags);
    };
}

#[macro_export]
macro_rules! CV1800_BYPASS_DIV {
    ($name:ident, $parent:expr, $gate_reg:expr, $gate_shift:expr, $div_reg:expr, $div_shift:expr, $div_width:expr, $div_init:expr, $div_flag:expr, $bypass_reg:expr, $bypass_shift:expr, $flags:expr) => {
        let mut $name = cv1800_clk_bypass_div { div: _CV1800_DIV!(stringify!($name), $parent, $gate_reg, $gate_shift, $div_reg, $div_shift, $div_width, $div_init, $div_flag, &cv1800_clk_bypass_div_ops, $flags), bypass: CV1800_CLK_BIT!($bypass_reg, $bypass_shift) };
    };
}

#[macro_export]
macro_rules! CV1800_FIXED_DIV {
    ($name:ident, $parent:expr, $gate_reg:expr, $gate_shift:expr, $fix_div:expr, $flags:expr) => {
        let mut $name = _CV1800_FIXED_DIV!(stringify!($name), $parent, $gate_reg, $gate_shift, $fix_div, &cv1800_clk_div_ops, $flags);
    };
}

#[macro_export]
macro_rules! CV1800_BYPASS_FIXED_DIV {
    ($name:ident, $parent:expr, $gate_reg:expr, $gate_shift:expr, $fix_div:expr, $bypass_reg:expr, $bypass_shift:expr, $flags:expr) => {
        let mut $name = cv1800_clk_bypass_div { div: _CV1800_FIXED_DIV!(stringify!($name), $parent, $gate_reg, $gate_shift, $fix_div, &cv1800_clk_bypass_div_ops, $flags), bypass: CV1800_CLK_BIT!($bypass_reg, $bypass_shift) };
    };
}

#[macro_export]
macro_rules! _CV1800_MUX {
    ($name:expr, $parent:expr, $gate_reg:expr, $gate_shift:expr, $div_reg:expr, $div_shift:expr, $div_width:expr, $div_init:expr, $div_flag:expr, $mux_reg:expr, $mux_shift:expr, $mux_width:expr, $ops:expr, $flags:expr) => {
        cv1800_clk_mux { common: CV1800_CLK_COMMON!(stringify!($name), $parent, $ops, $flags), gate: CV1800_CLK_BIT!($gate_reg, $gate_shift), div: CV1800_CLK_REG!($div_reg, $div_shift, $div_width, $div_init, $div_flag), mux: CV1800_CLK_REG!($mux_reg, $mux_shift, $mux_width, 0, 0) }
    };
}

#[macro_export]
macro_rules! CV1800_MUX {
    ($name:ident, $parent:expr, $gate_reg:expr, $gate_shift:expr, $div_reg:expr, $div_shift:expr, $div_width:expr, $div_init:expr, $div_flag:expr, $mux_reg:expr, $mux_shift:expr, $mux_width:expr, $flags:expr) => {
        let mut $name = _CV1800_MUX!(stringify!($name), $parent, $gate_reg, $gate_shift, $div_reg, $div_shift, $div_width, $div_init, $div_flag, $mux_reg, $mux_shift, $mux_width, &cv1800_clk_mux_ops, $flags);
    };
}

#[macro_export]
macro_rules! CV1800_BYPASS_MUX {
    ($name:ident, $parent:expr, $gate_reg:expr, $gate_shift:expr, $div_reg:expr, $div_shift:expr, $div_width:expr, $div_init:expr, $div_flag:expr, $mux_reg:expr, $mux_shift:expr, $mux_width:expr, $bypass_reg:expr, $bypass_shift:expr, $flags:expr) => {
        let mut $name = cv1800_clk_bypass_mux { mux: _CV1800_MUX!(stringify!($name), $parent, $gate_reg, $gate_shift, $div_reg, $div_shift, $div_width, $div_init, $div_flag, $mux_reg, $mux_shift, $mux_width, &cv1800_clk_bypass_mux_ops, $flags), bypass: CV1800_CLK_BIT!($bypass_reg, $bypass_shift) };
    };
}

/* CV1800_MMUX and CV1800_ACLK retain the source macro argument ordering. */
#[macro_export]
macro_rules! CV1800_MMUX {
    ($name:ident, $parent:expr, $gate_reg:expr, $gate_shift:expr, $dr0:expr,$ds0:expr,$dw0:expr,$di0:expr,$df0:expr, $dr1:expr,$ds1:expr,$dw1:expr,$di1:expr,$df1:expr, $mr0:expr,$ms0:expr,$mw0:expr, $mr1:expr,$ms1:expr,$mw1:expr, $br:expr,$bs:expr, $cr:expr,$cs:expr, $p2s:expr,$s2p0:expr,$s2p1:expr,$flags:expr) => {
        let mut $name = cv1800_clk_mmux { common: CV1800_CLK_COMMON!(stringify!($name), $parent, &cv1800_clk_mmux_ops, $flags), gate: CV1800_CLK_BIT!($gate_reg,$gate_shift), div: [CV1800_CLK_REG!($dr0,$ds0,$dw0,$di0,$df0), CV1800_CLK_REG!($dr1,$ds1,$dw1,$di1,$df1)], mux: [CV1800_CLK_REG!($mr0,$ms0,$mw0,0,0), CV1800_CLK_REG!($mr1,$ms1,$mw1,0,0)], bypass: CV1800_CLK_BIT!($br,$bs), clk_sel: CV1800_CLK_BIT!($cr,$cs), parent2sel: $p2s, sel2parent: [$s2p0,$s2p1] };
    };
}

#[macro_export]
macro_rules! CV1800_ACLK {
    ($name:ident,$parent:expr,$sr:expr,$ss:expr,$or:expr,$os:expr,$der:expr,$des:expr,$dur:expr,$dus:expr,$mr:expr,$ms:expr,$mw:expr,$mf:expr,$nr:expr,$ns:expr,$nw:expr,$nf:expr,$target:expr,$flags:expr) => {
        let mut $name = cv1800_clk_audio { common: CV1800_CLK_COMMON!(stringify!($name),$parent,&cv1800_clk_audio_ops,$flags), src_en: CV1800_CLK_BIT!($sr,$ss), output_en: CV1800_CLK_BIT!($or,$os), div_en: CV1800_CLK_BIT!($der,$des), div_up: CV1800_CLK_BIT!($dur,$dus), m: CV1800_CLK_REG!($mr,$ms,$mw,0,$mf), n: CV1800_CLK_REG!($nr,$ns,$nw,0,$nf), target_rate: $target };
    };
}

extern "C" {
    pub static cv1800_clk_gate_ops: clk_ops;
    pub static cv1800_clk_div_ops: clk_ops;
    pub static cv1800_clk_bypass_div_ops: clk_ops;
    pub static cv1800_clk_mux_ops: clk_ops;
    pub static cv1800_clk_bypass_mux_ops: clk_ops;
    pub static cv1800_clk_mmux_ops: clk_ops;
    pub static cv1800_clk_audio_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
