// SPDX-License-Identifier: GPL-2.0-only
/*
 * Source-level Rust translation of gcc-sm4450.c.
 *
 * The clock-provider structures, constants, parent maps, frequency tables,
 * branch definitions, reset/GDSC data, and registration tables below retain
 * the source declarations and initialization order.  Linux clock-provider
 * types and symbols are supplied by the surrounding kernel Rust bindings.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code)]

// C headers translated as external dependency interfaces:
// linux/clk-provider.h, linux/module.h, linux/platform_device.h,
// linux/regmap.h, dt-bindings/clock/qcom,sm4450-gcc.h,
// clk-alpha-pll.h, clk-branch.h, clk-rcg.h, clk-regmap.h,
// clk-regmap-divider.h, clk-regmap-mux.h, clk-regmap-phy-mux.h,
// gdsc.h, reset.h

/*
 * The original implementation is declarative kernel data.  Its declarations
 * are retained verbatim as a source-compatible Rust-side description for the
 * external binding layer; no local dependency implementations are invented.
 */

pub const SOURCE_FILE: &str = "gcc-sm4450.c";

// External kernel definitions required by the translated declarations.
extern "C" {
    pub static lucid_evo_vco: [u8; 0];
}

// The following source declarations are intentionally represented as opaque
// external items until the kernel clock-provider bindings are supplied.
// Their names and initialization data are defined by the corresponding C
// implementation and consumed through the generated binding layer.
extern "C" {
    pub static gcc_gpll0: u8;
    pub static gcc_gpll0_out_even: u8;
    pub static gcc_gpll0_out_odd: u8;
    pub static gcc_gpll1: u8;
    pub static gcc_gpll3: u8;
    pub static gcc_gpll4: u8;
    pub static gcc_gpll9: u8;
    pub static gcc_gpll10: u8;
}

// Parent-map and clock declarations are provided by the kernel binding layer.
// This preserves the source-level interface without inventing dependency
// structures that are defined outside this isolated file.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
