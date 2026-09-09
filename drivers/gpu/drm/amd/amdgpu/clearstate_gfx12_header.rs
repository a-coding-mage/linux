/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

#[repr(C)]
pub struct cs_extent_def {
    pub data: *const u32,
    pub reg: u32,
    pub size: u32,
}

#[repr(C)]
pub struct cs_section_def {
    pub extents: *const cs_extent_def,
    pub section: u32,
}

extern "C" {
    pub static SECT_CONTEXT: u32;
    pub static SECT_NONE: u32;
}

static gfx12_SECT_CONTEXT_def_1: [u32; 34] = [
    0x00000000, // mmSC_MEM_TEMPORAL
    0x00000000, // mmSC_MEM_SPEC_READ
    0x00000000, // mmPA_SC_VPORT_0_TL
    0x00000000, // mmPA_SC_VPORT_0_BR
    0x00000000, // mmPA_SC_VPORT_1_TL
    0x00000000, // mmPA_SC_VPORT_1_BR
    0x00000000, // mmPA_SC_VPORT_2_TL
    0x00000000, // mmPA_SC_VPORT_2_BR
    0x00000000, // mmPA_SC_VPORT_3_TL
    0x00000000, // mmPA_SC_VPORT_3_BR
    0x00000000, // mmPA_SC_VPORT_4_TL
    0x00000000, // mmPA_SC_VPORT_4_BR
    0x00000000, // mmPA_SC_VPORT_5_TL
    0x00000000, // mmPA_SC_VPORT_5_BR
    0x00000000, // mmPA_SC_VPORT_6_TL
    0x00000000, // mmPA_SC_VPORT_6_BR
    0x00000000, // mmPA_SC_VPORT_7_TL
    0x00000000, // mmPA_SC_VPORT_7_BR
    0x00000000, // mmPA_SC_VPORT_8_TL
    0x00000000, // mmPA_SC_VPORT_8_BR
    0x00000000, // mmPA_SC_VPORT_9_TL
    0x00000000, // mmPA_SC_VPORT_9_BR
    0x00000000, // mmPA_SC_VPORT_10_TL
    0x00000000, // mmPA_SC_VPORT_10_BR
    0x00000000, // mmPA_SC_VPORT_11_TL
    0x00000000, // mmPA_SC_VPORT_11_BR
    0x00000000, // mmPA_SC_VPORT_12_TL
    0x00000000, // mmPA_SC_VPORT_12_BR
    0x00000000, // mmPA_SC_VPORT_13_TL
    0x00000000, // mmPA_SC_VPORT_13_BR
    0x00000000, // mmPA_SC_VPORT_14_TL
    0x00000000, // mmPA_SC_VPORT_14_BR
    0x00000000, // mmPA_SC_VPORT_15_TL
    0x00000000, // mmPA_SC_VPORT_15_BR
];

static gfx12_SECT_CONTEXT_def_2: [u32; 2] = [0x00000000, 0x00000000];
static gfx12_SECT_CONTEXT_def_3: [u32; 1] = [0x00000000];
static gfx12_SECT_CONTEXT_def_4: [u32; 6] = [0x00000000; 6];
static gfx12_SECT_CONTEXT_def_5: [u32; 11] = [0x00000000; 11];
static gfx12_SECT_CONTEXT_def_6: [u32; 8] = [0x00000000; 8];

static gfx12_SECT_CONTEXT_defs: [cs_extent_def; 7] = [
    cs_extent_def { data: gfx12_SECT_CONTEXT_def_1.as_ptr(), reg: 0x0000a03e, size: 34 },
    cs_extent_def { data: gfx12_SECT_CONTEXT_def_2.as_ptr(), reg: 0x0000a0cc, size: 2 },
    cs_extent_def { data: gfx12_SECT_CONTEXT_def_3.as_ptr(), reg: 0x0000a0d8, size: 1 },
    cs_extent_def { data: gfx12_SECT_CONTEXT_def_4.as_ptr(), reg: 0x0000a0db, size: 6 },
    cs_extent_def { data: gfx12_SECT_CONTEXT_def_5.as_ptr(), reg: 0x0000a2e5, size: 11 },
    cs_extent_def { data: gfx12_SECT_CONTEXT_def_6.as_ptr(), reg: 0x0000a3c0, size: 8 },
    cs_extent_def { data: core::ptr::null(), reg: 0, size: 0 },
];

static gfx12_cs_data: [cs_section_def; 2] = [
    cs_section_def { extents: gfx12_SECT_CONTEXT_defs.as_ptr(), section: unsafe { SECT_CONTEXT } },
    cs_section_def { extents: core::ptr::null(), section: unsafe { SECT_NONE } },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
