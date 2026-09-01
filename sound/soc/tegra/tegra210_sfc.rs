// SPDX-License-Identifier: GPL-2.0-only
// SPDX-FileCopyrightText: Copyright (c) 2021-2024 NVIDIA CORPORATION & AFFILIATES.
// All rights reserved.
//
// tegra210_sfc.c - Tegra210 SFC driver

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::zeroed;

// Dependencies supplied by the surrounding kernel/ASoC/tegra driver code.
type bool_ = bool;



const ((-EOPNOTSUPP) as isize as *mut i32): *mut i32 = (-EOPNOTSUPP) as isize as *mut i32;
const core::ptr::null_mut(): *mut i32 = core::ptr::null_mut();

static tegra210_sfc_reg_defaults: [reg_default; 6] = [
	[ TEGRA210_SFC_RX_INT_MASK, 0x00000001],
	[ TEGRA210_SFC_RX_CIF_CTRL, 0x00007700],
	[ TEGRA210_SFC_TX_INT_MASK, 0x00000001],
	[ TEGRA210_SFC_TX_CIF_CTRL, 0x00007700],
	[ TEGRA210_SFC_CG, 0x1],
	[ TEGRA210_SFC_CFG_RAM_CTRL, 0x00004000],
];

static tegra210_sfc_rates: [i32; TEGRA210_SFC_NUM_RATES as usize] = [
	8000,
	11025,
	16000,
	22050,
	24000,
	32000,
	44100,
	48000,
	64000,
	88200,
	96000,
	176400,
	192000,
];

/* coeff RAM tables required for SFC */
static mut coef_8to11: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x0018a102,//header
	0x000005d6,//input gain
	0x00c6543e, 0xff342935, 0x0052f116,
	0x000a1d78, 0xff3330c0, 0x005f88a3,
	0xffbee7c0, 0xff2b5ba5, 0x0073eb26,
	0x00000003,//output gain
	0x00235204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0000015f,//input gain
	0x00a7909c, 0xff241c71, 0x005f5e00,
	0xffca77f4, 0xff20dd50, 0x006855eb,
	0xff86c552, 0xff18137a, 0x00773648,
	0x00000001//output gain
];

static mut coef_8to16: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00006102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002//output gain
];

static mut coef_8to22: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x0018a102,//header
	0x000005d6,//input gain
	0x00c6543e, 0xff342935, 0x0052f116,
	0x000a1d78, 0xff3330c0, 0x005f88a3,
	0xffbee7c0, 0xff2b5ba5, 0x0073eb26,
	0x00000003,//output gain
	0x00230204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x000005f3,//input gain
	0x00d816d6, 0xff385383, 0x004fe566,
	0x003c548d, 0xff38c23d, 0x005d0b1c,
	0xfff02f7d, 0xff31e983, 0x0072d65d,
	0x00000001//output gain
];

static mut coef_8to24: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x0000a105,//header
	0x000005e1,//input gain
	0x00dca92f, 0xff45647a, 0x0046b59c,
	0x00429d1e, 0xff4fec62, 0x00516d30,
	0xffdea779, 0xff5e08ba, 0x0060185e,
	0xffafbab2, 0xff698d5a, 0x006ce3ae,
	0xff9a82d2, 0xff704674, 0x007633c5,
	0xff923433, 0xff721128, 0x007cff42,
	0x00000003//output gain
];

static mut coef_8to32: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00006102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002//output gain
];

static mut coef_8to44: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x0156105,//interpolation + IIR filter
	0x0000d649,//input gain
	0x00e87afb, 0xff5f69d0, 0x003df3cf,
	0x007ce488, 0xff99a5c8, 0x0056a6a0,
	0x00344928, 0xffcba3e5, 0x006be470,
	0x00137aa7, 0xffe60276, 0x00773410,
	0x0005fa2a, 0xfff1ac11, 0x007c795b,
	0x00012d36, 0xfff5eca2, 0x007f10ef,
	0x00000002,//ouptut gain
	0x0021a102,//interpolation + IIR filter
	0x00000e00,//input gain
	0x00e2e000, 0xff6e1a00, 0x002aaa00,
	0x00610a00, 0xff5dda00, 0x003ccc00,
	0x00163a00, 0xff3c0400, 0x00633200,
	0x00000003,//Output gain
	0x00000204,//Farrow filter
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_8to48: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00156105,//interpolation + IIR Filter
	0x0000d649,//input gain
	0x00e87afb, 0xff5f69d0, 0x003df3cf,
	0x007ce488, 0xff99a5c8, 0x0056a6a0,
	0x00344928, 0xffcba3e5, 0x006be470,
	0x00137aa7, 0xffe60276, 0x00773410,
	0x0005fa2a, 0xfff1ac11, 0x007c795b,
	0x00012d36, 0xfff5eca2, 0x007f10ef,
	0x00000002,//ouptut gain
	0x0000a102,//interpolation + IIR filter
	0x00000e00,//input gain
	0x00e2e000, 0xff6e1a00, 0x002aaa00,
	0x00610a00, 0xff5dda00, 0x003ccc00,
	0x00163a00, 0xff3c0400, 0x00633200,
	0x00000003//output gain
];

static mut coef_8to88: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x0024a102,//header
	0x0000007d,//input gain
	0x007d1f20, 0xff1a540e, 0x00678bf9,
	0xff916625, 0xff16b0ff, 0x006e433a,
	0xff5af660, 0xff0eb91f, 0x00797356,
	0x00000003,//output gain
	0x00000204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_8to96: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x0000a102,//header
	0x0000007d,//input gain
	0x007d1f20, 0xff1a540e, 0x00678bf9,
	0xff916625, 0xff16b0ff, 0x006e433a,
	0xff5af660, 0xff0eb91f, 0x00797356,
	0x00000003//output gain
];

static mut coef_11to8: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0000015f,//input gain
	0x00a7909c, 0xff241c71, 0x005f5e00,
	0xffca77f4, 0xff20dd50, 0x006855eb,
	0xff86c552, 0xff18137a, 0x00773648,
	0x00000002,//output gain
	0x00186102,//header
	0x000005f3,//input gain
	0x00d816d6, 0xff385383, 0x004fe566,
	0x003c548d, 0xff38c23d, 0x005d0b1c,
	0xfff02f7d, 0xff31e983, 0x0072d65d,
	0x00000002,//output gain
	0x00239204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_11to16: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00009204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_11to22: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00006102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002//output gain
];

static mut coef_11to24: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00005204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_11to32: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00246102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002,//output gain
	0x00009204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_11to44: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00006102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002//output gain
];

static mut coef_11to48: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00246102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002,//output gain
	0x00005204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_11to88: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00006102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002//output gain
];

static mut coef_11to96: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00246102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002,//output gain
	0x00000204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_16to8: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_16to11: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000fa103,//header
	0x000001e0,//input gain
	0x00de44c0, 0xff380b7f, 0x004ffc73,
	0x00494b44, 0xff3d493a, 0x005908bf,
	0xffe9a3c8, 0xff425647, 0x006745f7,
	0xffc42d61, 0xff40a6c7, 0x00776709,
	0x00000003,//output gain
	0x001a5204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_16to22: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x0018a102,//header
	0x000005d6,//input gain
	0x00c6543e, 0xff342935, 0x0052f116,
	0x000a1d78, 0xff3330c0, 0x005f88a3,
	0xffbee7c0, 0xff2b5ba5, 0x0073eb26,
	0x00000003,//output gain
	0x00235204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0000015f,//input gain
	0x00a7909c, 0xff241c71, 0x005f5e00,
	0xffca77f4, 0xff20dd50, 0x006855eb,
	0xff86c552, 0xff18137a, 0x00773648,
	0x00000001//output gain
];

static mut coef_16to24: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x0015a105,//header
	0x00000292,//input gain
	0x00e4320a, 0xff41d2d9, 0x004911ac,
	0x005dd9e3, 0xff4c7d80, 0x0052103e,
	0xfff8ebef, 0xff5b6fab, 0x005f0a0d,
	0xffc4b414, 0xff68582c, 0x006b38e5,
	0xffabb861, 0xff704bec, 0x0074de52,
	0xffa19f4c, 0xff729059, 0x007c7e90,
	0x00000003,//output gain
	0x00005105,//header
	0x00000292,//input gain
	0x00e4320a, 0xff41d2d9, 0x004911ac,
	0x005dd9e3, 0xff4c7d80, 0x0052103e,
	0xfff8ebef, 0xff5b6fab, 0x005f0a0d,
	0xffc4b414, 0xff68582c, 0x006b38e5,
	0xffabb861, 0xff704bec, 0x0074de52,
	0xffa19f4c, 0xff729059, 0x007c7e90,
	0x00000001//output gain
];

static mut coef_16to32: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00006102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002//output gain
];

static mut coef_16to44: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00156105,//interpolation + IIR filter
	0x0000d649,//input gain
	0x00e87afb, 0xff5f69d0, 0x003df3cf,
	0x007ce488, 0xff99a5c8, 0x0056a6a0,
	0x00344928, 0xffcba3e5, 0x006be470,
	0x00137aa7, 0xffe60276, 0x00773410,
	0x0005fa2a, 0xfff1ac11, 0x007c795b,
	0x00012d36, 0xfff5eca2, 0x007f10ef,
	0x00000002,//output gain
	0x0021a102,//interpolation + IIR filter
	0x00000e00,//input gain
	0x00e2e000, 0xff6e1a00, 0x002aaa00,
	0x00610a00, 0xff5dda00, 0x003ccc00,
	0x00163a00, 0xff3c0400, 0x00633200,
	0x00000003,//output gain
	0x002c0204,//Farrow Filter
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005101,//IIR Filter + Decimator
	0x0000203c,//input gain
	0x00f52d35, 0xff2e2162, 0x005a21e0,
	0x00c6f0f0, 0xff2ecd69, 0x006fa78d,
	0x00000001//output gain
];

static mut coef_16to48: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x0000a105,//interpolation + IIR Filter
	0x00000784,//input gain
	0x00cc516e, 0xff2c9639, 0x005ad5b3,
	0x0013ad0d, 0xff3d4799, 0x0063ce75,
	0xffb6f398, 0xff5138d1, 0x006e9e1f,
	0xff9186e5, 0xff5f96a4, 0x0076a86e,
	0xff82089c, 0xff676b81, 0x007b9f8a,
	0xff7c48a5, 0xff6a31e7, 0x007ebb7b,
	0x00000003//output gain
];

static mut coef_16to88: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x0018a102,//header
	0x000005d6,//input gain
	0x00c6543e, 0xff342935, 0x0052f116,
	0x000a1d78, 0xff3330c0, 0x005f88a3,
	0xffbee7c0, 0xff2b5ba5, 0x0073eb26,
	0x00000003,//output gain
	0x00000204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_16to96: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x0000a102,//header
	0x000005d6,//input gain
	0x00c6543e, 0xff342935, 0x0052f116,
	0x000a1d78, 0xff3330c0, 0x005f88a3,
	0xffbee7c0, 0xff2b5ba5, 0x0073eb26,
	0x00000003//output gain
];

static mut coef_16to176: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x0024a102,//header
	0x0000007d,//input gain
	0x007d1f20, 0xff1a540e, 0x00678bf9,
	0xff916625, 0xff16b0ff, 0x006e433a,
	0xff5af660, 0xff0eb91f, 0x00797356,
	0x00000003,//output gain
	0x00000204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_16to192: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x0000a102,//header
	0x0000007d,//input gain
	0x007d1f20, 0xff1a540e, 0x00678bf9,
	0xff916625, 0xff16b0ff, 0x006e433a,
	0xff5af660, 0xff0eb91f, 0x00797356,
	0x00000003//output gain
];

static mut coef_22to8: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x000005f3,//input gain
	0x00d816d6, 0xff385383, 0x004fe566,
	0x003c548d, 0xff38c23d, 0x005d0b1c,
	0xfff02f7d, 0xff31e983, 0x0072d65d,
	0x00000002,//output gain
	0x00179204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_22to11: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_22to16: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0000015f,//input gain
	0x00a7909c, 0xff241c71, 0x005f5e00,
	0xffca77f4, 0xff20dd50, 0x006855eb,
	0xff86c552, 0xff18137a, 0x00773648,
	0x00000002,//output gain
	0x00186102,//header
	0x000005f3,//input gain
	0x00d816d6, 0xff385383, 0x004fe566,
	0x003c548d, 0xff38c23d, 0x005d0b1c,
	0xfff02f7d, 0xff31e983, 0x0072d65d,
	0x00000002,//output gain
	0x00239204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_22to24: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00235204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d029,//input gain
	0x00f2a98b, 0xff92aa71, 0x001fcd16,
	0x00ae9004, 0xffb85140, 0x0041813a,
	0x007f8ed1, 0xffd585fc, 0x006a69e6,
	0x00000001//output gain
];

static mut coef_22to32: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00009204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_22to44: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00006102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002//output gain
];

static mut coef_22to48: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00005204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_22to88: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00006102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002//output gain
];

static mut coef_22to96: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00246102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002,//output gain
	0x00005204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_22to176: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00006102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002//output gain
];

static mut coef_22to192: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00246102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002,//output gain
	0x00000204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_24to8: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00009105,//header
	0x000005e1,//input gain
	0x00dca92f, 0xff45647a, 0x0046b59c,
	0x00429d1e, 0xff4fec62, 0x00516d30,
	0xffdea779, 0xff5e08ba, 0x0060185e,
	0xffafbab2, 0xff698d5a, 0x006ce3ae,
	0xff9a82d2, 0xff704674, 0x007633c5,
	0xff923433, 0xff721128, 0x007cff42,
	0x00000001//output gain
];

static mut coef_24to11: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000f6103,//header
	0x000001e0,//input gain
	0x00de44c0, 0xff380b7f, 0x004ffc73,
	0x00494b44, 0xff3d493a, 0x005908bf,
	0xffe9a3c8, 0xff425647, 0x006745f7,
	0xffc42d61, 0xff40a6c7, 0x00776709,
	0x00000002,//output gain
	0x001a5204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_24to16: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00156105,//header
	0x00000292,//input gain
	0x00e4320a, 0xff41d2d9, 0x004911ac,
	0x005dd9e3, 0xff4c7d80, 0x0052103e,
	0xfff8ebef, 0xff5b6fab, 0x005f0a0d,
	0xffc4b414, 0xff68582c, 0x006b38e5,
	0xffabb861, 0xff704bec, 0x0074de52,
	0xffa19f4c, 0xff729059, 0x007c7e90,
	0x00000002,//output gain
	0x00009105,//header
	0x00000292,//input gain
	0x00e4320a, 0xff41d2d9, 0x004911ac,
	0x005dd9e3, 0xff4c7d80, 0x0052103e,
	0xfff8ebef, 0xff5b6fab, 0x005f0a0d,
	0xffc4b414, 0xff68582c, 0x006b38e5,
	0xffabb861, 0xff704bec, 0x0074de52,
	0xffa19f4c, 0xff729059, 0x007c7e90,
	0x00000001//output gain
];

static mut coef_24to22: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d029,//input gain
	0x00f2a98b, 0xff92aa71, 0x001fcd16,
	0x00ae9004, 0xffb85140, 0x0041813a,
	0x007f8ed1, 0xffd585fc, 0x006a69e6,
	0x00000002,//output gain
	0x001b6103,//header
	0x000001e0,//input gain
	0x00de44c0, 0xff380b7f, 0x004ffc73,
	0x00494b44, 0xff3d493a, 0x005908bf,
	0xffe9a3c8, 0xff425647, 0x006745f7,
	0xffc42d61, 0xff40a6c7, 0x00776709,
	0x00000002,//output gain
	0x00265204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_24to32: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00009102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001//output gain
];

static mut coef_24to44: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00230204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x00001685,//input gain
	0x00f53ae9, 0xff52f196, 0x003e3e08,
	0x00b9f857, 0xff5d8985, 0x0050070a,
	0x008c3e86, 0xff6053f0, 0x006d98ef,
	0x00000001//output gain
];

static mut coef_24to48: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00006102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002//output gain
];

static mut coef_24to88: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00246102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002,//output gain
	0x002f0204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x00000138,//input gain
	0x00d5d232, 0xff2a3bf8, 0x005a785c,
	0x0034001b, 0xff283109, 0x006462a6,
	0xffe6746a, 0xff1fb09c, 0x00758a91,
	0x00000001//output gain
];

static mut coef_24to96: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00006102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002//output gain
];

static mut coef_24to176: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00246102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002,//output gain
	0x00000204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_24to192: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00006102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002//output gain
];

static mut coef_32to8: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c5102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_32to11: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000ca102,//header
	0x000000af,//input gain
	0x00c65663, 0xff23d2ce, 0x005f97d6,
	0x00086ad6, 0xff20ec4f, 0x00683201,
	0xffbbbef6, 0xff184447, 0x00770963,
	0x00000003,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x0000d102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001//output gain
];

static mut coef_32to16: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_32to22: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000fa103,//header
	0x000001e0,//input gain
	0x00de44c0, 0xff380b7f, 0x004ffc73,
	0x00494b44, 0xff3d493a, 0x005908bf,
	0xffe9a3c8, 0xff425647, 0x006745f7,
	0xffc42d61, 0xff40a6c7, 0x00776709,
	0x00000003,//output gain
	0x001a5204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_32to24: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000ca102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000003,//output gain
	0x0000d102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001//output gain
];

static mut coef_32to44: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x0018a102,//header
	0x000005d6,//input gain
	0x00c6543e, 0xff342935, 0x0052f116,
	0x000a1d78, 0xff3330c0, 0x005f88a3,
	0xffbee7c0, 0xff2b5ba5, 0x0073eb26,
	0x00000003,//output gain
	0x00235204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0000015f,//input gain
	0x00a7909c, 0xff241c71, 0x005f5e00,
	0xffca77f4, 0xff20dd50, 0x006855eb,
	0xff86c552, 0xff18137a, 0x00773648,
	0x00000001//output gain
];

static mut coef_32to48: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x0015a105,//header
	0x00000292,//input gain
	0x00e4320a, 0xff41d2d9, 0x004911ac,
	0x005dd9e3, 0xff4c7d80, 0x0052103e,
	0xfff8ebef, 0xff5b6fab, 0x005f0a0d,
	0xffc4b414, 0xff68582c, 0x006b38e5,
	0xffabb861, 0xff704bec, 0x0074de52,
	0xffa19f4c, 0xff729059, 0x007c7e90,
	0x00000003,//output gain
	0x00005105,//header
	0x00000292,//input gain
	0x00e4320a, 0xff41d2d9, 0x004911ac,
	0x005dd9e3, 0xff4c7d80, 0x0052103e,
	0xfff8ebef, 0xff5b6fab, 0x005f0a0d,
	0xffc4b414, 0xff68582c, 0x006b38e5,
	0xffabb861, 0xff704bec, 0x0074de52,
	0xffa19f4c, 0xff729059, 0x007c7e90,
	0x00000001//output gain
];

static mut coef_32to88: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x0018a102,//header
	0x000005d6,//input gain
	0x00c6543e, 0xff342935, 0x0052f116,
	0x000a1d78, 0xff3330c0, 0x005f88a3,
	0xffbee7c0, 0xff2b5ba5, 0x0073eb26,
	0x00000003,//output gain
	0x00230204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x000005f3,//input gain
	0x00d816d6, 0xff385383, 0x004fe566,
	0x003c548d, 0xff38c23d, 0x005d0b1c,
	0xfff02f7d, 0xff31e983, 0x0072d65d,
	0x00000001//output gain
];

static mut coef_32to96: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x0000a105,//header
	0x00000292,//input gain
	0x00e4320a, 0xff41d2d9, 0x004911ac,
	0x005dd9e3, 0xff4c7d80, 0x0052103e,
	0xfff8ebef, 0xff5b6fab, 0x005f0a0d,
	0xffc4b414, 0xff68582c, 0x006b38e5,
	0xffabb861, 0xff704bec, 0x0074de52,
	0xffa19f4c, 0xff729059, 0x007c7e90,
	0x00000003//output gain
];

static mut coef_32to176: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x0018a102,//header
	0x000005d6,//input gain
	0x00c6543e, 0xff342935, 0x0052f116,
	0x000a1d78, 0xff3330c0, 0x005f88a3,
	0xffbee7c0, 0xff2b5ba5, 0x0073eb26,
	0x00000003,//output gain
	0x00000204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_32to192: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x0000a102,//header
	0x000005d6,//input gain
	0x00c6543e, 0xff342935, 0x0052f116,
	0x000a1d78, 0xff3330c0, 0x005f88a3,
	0xffbee7c0, 0xff2b5ba5, 0x0073eb26,
	0x00000003//output gain
];

static mut coef_44to8: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00120104,//IIR Filter
	0x00000af2,//input gain
	0x0057eebe, 0xff1e9863, 0x00652604,
	0xff7206ea, 0xff22ad7e, 0x006d47e1,
	0xff42a4d7, 0xff26e722, 0x0075fd83,
	0xff352f66, 0xff29312b, 0x007b986b,
	0xff310a07, 0xff296f51, 0x007eca7c,
	0x00000001,//output gain
	0x001d9204,//Farrow Filter + decimation
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005105,//IIR Filter + Decimator
	0x0000d649,//input gain
	0x00e87afb, 0xff5f69d0, 0x003df3cf,
	0x007ce488, 0xff99a5c8, 0x0056a6a0,
	0x00344928, 0xffcba3e5, 0x006be470,
	0x00137aa7, 0xffe60276, 0x00773410,
	0x0005fa2a, 0xfff1ac11, 0x007c795b,
	0x00012d36, 0xfff5eca2, 0x007f10ef,
	0x00000001//output gain
];

static mut coef_44to11: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c5102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_44to16: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00126104,//IIR Filter + interpolation
	0x00000af2,//input gain
	0x0057eebe, 0xff1e9863, 0x00652604,
	0xff7206ea, 0xff22ad7e, 0x006d47e1,
	0xff42a4d7, 0xff26e722, 0x0075fd83,
	0xff352f66, 0xff29312b, 0x007b986b,
	0xff310a07, 0xff296f51, 0x007eca7c,
	0x00000002,//output gain
	0x001d9204,//Farrow Filter + decimation
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005105,//IIR Filter + Decimator
	0x0000d649,//input gain
	0x00e87afb, 0xff5f69d0, 0x003df3cf,
	0x007ce488, 0xff99a5c8, 0x0056a6a0,
	0x00344928, 0xffcba3e5, 0x006be470,
	0x00137aa7, 0xffe60276, 0x00773410,
	0x0005fa2a, 0xfff1ac11, 0x007c795b,
	0x00012d36, 0xfff5eca2, 0x007f10ef,
	0x00000001//output gain
];

static mut coef_44to22: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_44to24: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x00001685,//input gain
	0x00f53ae9, 0xff52f196, 0x003e3e08,
	0x00b9f857, 0xff5d8985, 0x0050070a,
	0x008c3e86, 0xff6053f0, 0x006d98ef,
	0x00000002,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_44to32: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0000015f,//input gain
	0x00a7909c, 0xff241c71, 0x005f5e00,
	0xffca77f4, 0xff20dd50, 0x006855eb,
	0xff86c552, 0xff18137a, 0x00773648,
	0x00000002,//output gain
	0x00186102,//header
	0x000005f3,//input gain
	0x00d816d6, 0xff385383, 0x004fe566,
	0x003c548d, 0xff38c23d, 0x005d0b1c,
	0xfff02f7d, 0xff31e983, 0x0072d65d,
	0x00000002,//output gain
	0x00239204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_44to48: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00235204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d029,//input gain
	0x00f2a98b, 0xff92aa71, 0x001fcd16,
	0x00ae9004, 0xffb85140, 0x0041813a,
	0x007f8ed1, 0xffd585fc, 0x006a69e6,
	0x00000001//output gain
];

static mut coef_44to88: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00006102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002//output gain
];

static mut coef_44to96: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00005204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_44to176: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00006102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002//output gain
];

static mut coef_44to192: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00246102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002,//output gain
	0x00005204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_48to8: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c9102,//IIR Filter + Decimator
	0x00000e00,//input gain
	0x00e2e000, 0xff6e1a00, 0x002aaa00,
	0x00610a00, 0xff5dda00, 0x003ccc00,
	0x00163a00, 0xff3c0400, 0x00633200,
	0x00000001,//output gain
	0x00005105,//IIR Filter + Decimator
	0x0000d649,//input gain
	0x00e87afb, 0xff5f69d0, 0x003df3cf,
	0x007ce488, 0xff99a5c8, 0x0056a6a0,
	0x00344928, 0xffcba3e5, 0x006be470,
	0x00137aa7, 0xffe60276, 0x00773410,
	0x0005fa2a, 0xfff1ac11, 0x007c795b,
	0x00012d36, 0xfff5eca2, 0x007f10ef,
	0x00000001//output gain
];

static mut coef_48to11: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x000000af,//input gain
	0x00c65663, 0xff23d2ce, 0x005f97d6,
	0x00086ad6, 0xff20ec4f, 0x00683201,
	0xffbbbef6, 0xff184447, 0x00770963,
	0x00000002,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00235102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_48to16: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00009105,//IIR Filter + Decimator
	0x00000784,//input gain
	0x00cc516e, 0xff2c9639, 0x005ad5b3,
	0x0013ad0d, 0xff3d4799, 0x0063ce75,
	0xffb6f398, 0xff5138d1, 0x006e9e1f,
	0xff9186e5, 0xff5f96a4, 0x0076a86e,
	0xff82089c, 0xff676b81, 0x007b9f8a,
	0xff7c48a5, 0xff6a31e7, 0x007ebb7b,
	0x00000001//output gain
];

static mut coef_48to22: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000f6103,//header
	0x000001e0,//input gain
	0x00de44c0, 0xff380b7f, 0x004ffc73,
	0x00494b44, 0xff3d493a, 0x005908bf,
	0xffe9a3c8, 0xff425647, 0x006745f7,
	0xffc42d61, 0xff40a6c7, 0x00776709,
	0x00000002,//output gain
	0x001a5204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_48to24: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_48to32: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00156105,//header
	0x00000292,//input gain
	0x00e4320a, 0xff41d2d9, 0x004911ac,
	0x005dd9e3, 0xff4c7d80, 0x0052103e,
	0xfff8ebef, 0xff5b6fab, 0x005f0a0d,
	0xffc4b414, 0xff68582c, 0x006b38e5,
	0xffabb861, 0xff704bec, 0x0074de52,
	0xffa19f4c, 0xff729059, 0x007c7e90,
	0x00000002,//output gain
	0x00009105,//header
	0x00000292,//input gain
	0x00e4320a, 0xff41d2d9, 0x004911ac,
	0x005dd9e3, 0xff4c7d80, 0x0052103e,
	0xfff8ebef, 0xff5b6fab, 0x005f0a0d,
	0xffc4b414, 0xff68582c, 0x006b38e5,
	0xffabb861, 0xff704bec, 0x0074de52,
	0xffa19f4c, 0xff729059, 0x007c7e90,
	0x00000001//output gain
];

static mut coef_48to44: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d029,//input gain
	0x00f2a98b, 0xff92aa71, 0x001fcd16,
	0x00ae9004, 0xffb85140, 0x0041813a,
	0x007f8ed1, 0xffd585fc, 0x006a69e6,
	0x00000002,//output gain
	0x001b6103,//header
	0x000001e0,//input gain
	0x00de44c0, 0xff380b7f, 0x004ffc73,
	0x00494b44, 0xff3d493a, 0x005908bf,
	0xffe9a3c8, 0xff425647, 0x006745f7,
	0xffc42d61, 0xff40a6c7, 0x00776709,
	0x00000002,//output gain
	0x00265204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_48to88: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00230204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x00001685,//input gain
	0x00f53ae9, 0xff52f196, 0x003e3e08,
	0x00b9f857, 0xff5d8985, 0x0050070a,
	0x008c3e86, 0xff6053f0, 0x006d98ef,
	0x00000001//output gain
];

static mut coef_48to96: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00006102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002//output gain
];

static mut coef_48to176: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00186102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00246102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002,//output gain
	0x002f0204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x00000138,//input gain
	0x00d5d232, 0xff2a3bf8, 0x005a785c,
	0x0034001b, 0xff283109, 0x006462a6,
	0xffe6746a, 0xff1fb09c, 0x00758a91,
	0x00000001//output gain
];

static mut coef_48to192: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000002,//output gain
	0x00006102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002//output gain
];

static mut coef_88to8: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c0102,//header
	0x00000057,//input gain
	0x00a8e717, 0xff1c748d, 0x0065b976,
	0xffcbccab, 0xff190aff, 0x006cc1cf,
	0xff871ce1, 0xff10d878, 0x0078cfc5,
	0x00000001,//output gain
	0x00179204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00235102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_88to11: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c5102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000001,//output gain
	0x00185102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_88to16: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c0102,//header
	0x000005f3,//input gain
	0x00d816d6, 0xff385383, 0x004fe566,
	0x003c548d, 0xff38c23d, 0x005d0b1c,
	0xfff02f7d, 0xff31e983, 0x0072d65d,
	0x00000001,//output gain
	0x00179204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_88to22: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c5102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_88to24: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c0102,//header
	0x00001685,//input gain
	0x00f53ae9, 0xff52f196, 0x003e3e08,
	0x00b9f857, 0xff5d8985, 0x0050070a,
	0x008c3e86, 0xff6053f0, 0x006d98ef,
	0x00000001,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_88to32: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x000005f3,//input gain
	0x00d816d6, 0xff385383, 0x004fe566,
	0x003c548d, 0xff38c23d, 0x005d0b1c,
	0xfff02f7d, 0xff31e983, 0x0072d65d,
	0x00000002,//output gain
	0x00179204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_88to44: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_88to48: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x00001685,//input gain
	0x00f53ae9, 0xff52f196, 0x003e3e08,
	0x00b9f857, 0xff5d8985, 0x0050070a,
	0x008c3e86, 0xff6053f0, 0x006d98ef,
	0x00000002,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_88to96: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00005204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_88to176: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00006102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002//output gain
];

static mut coef_88to192: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000002,//output gain
	0x00186102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002,//output gain
	0x00005204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_96to8: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c9102,//header
	0x0000007d,//input gain
	0x007d1f20, 0xff1a540e, 0x00678bf9,
	0xff916625, 0xff16b0ff, 0x006e433a,
	0xff5af660, 0xff0eb91f, 0x00797356,
	0x00000001,//output gain
	0x00185102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_96to11: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c0102,//header
	0x000000af,//input gain
	0x00c65663, 0xff23d2ce, 0x005f97d6,
	0x00086ad6, 0xff20ec4f, 0x00683201,
	0xffbbbef6, 0xff184447, 0x00770963,
	0x00000001,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00235102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_96to16: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c9102,//header
	0x000005d6,//input gain
	0x00c6543e, 0xff342935, 0x0052f116,
	0x000a1d78, 0xff3330c0, 0x005f88a3,
	0xffbee7c0, 0xff2b5ba5, 0x0073eb26,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_96to22: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x000000af,//input gain
	0x00c65663, 0xff23d2ce, 0x005f97d6,
	0x00086ad6, 0xff20ec4f, 0x00683201,
	0xffbbbef6, 0xff184447, 0x00770963,
	0x00000002,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00235102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_96to24: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c5102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_96to32: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00009105,//header
	0x00000292,//input gain
	0x00e4320a, 0xff41d2d9, 0x004911ac,
	0x005dd9e3, 0xff4c7d80, 0x0052103e,
	0xfff8ebef, 0xff5b6fab, 0x005f0a0d,
	0xffc4b414, 0xff68582c, 0x006b38e5,
	0xffabb861, 0xff704bec, 0x0074de52,
	0xffa19f4c, 0xff729059, 0x007c7e90,
	0x00000001//output gain
];

static mut coef_96to44: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000f6103,//header
	0x000001e0,//input gain
	0x00de44c0, 0xff380b7f, 0x004ffc73,
	0x00494b44, 0xff3d493a, 0x005908bf,
	0xffe9a3c8, 0xff425647, 0x006745f7,
	0xffc42d61, 0xff40a6c7, 0x00776709,
	0x00000002,//output gain
	0x001a5204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_96to48: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_96to88: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000f6103,//header
	0x000001e0,//input gain
	0x00de44c0, 0xff380b7f, 0x004ffc73,
	0x00494b44, 0xff3d493a, 0x005908bf,
	0xffe9a3c8, 0xff425647, 0x006745f7,
	0xffc42d61, 0xff40a6c7, 0x00776709,
	0x00000002,//output gain
	0x001a0204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001//output gain
];

static mut coef_96to176: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000f6103,//header
	0x000001e0,//input gain
	0x00de44c0, 0xff380b7f, 0x004ffc73,
	0x00494b44, 0xff3d493a, 0x005908bf,
	0xffe9a3c8, 0xff425647, 0x006745f7,
	0xffc42d61, 0xff40a6c7, 0x00776709,
	0x00000002,//output gain
	0x001b6102,//header
	0x000000af,//input gain
	0x00c65663, 0xff23d2ce, 0x005f97d6,
	0x00086ad6, 0xff20ec4f, 0x00683201,
	0xffbbbef6, 0xff184447, 0x00770963,
	0x00000002,//output gain
	0x00260204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000001//output gain
];

static mut coef_96to192: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00006103,//header
	0x000001e0,//input gain
	0x00de44c0, 0xff380b7f, 0x004ffc73,
	0x00494b44, 0xff3d493a, 0x005908bf,
	0xffe9a3c8, 0xff425647, 0x006745f7,
	0xffc42d61, 0xff40a6c7, 0x00776709,
	0x00000002//output gain
];

static mut coef_176to16: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c0102,//header
	0x00000057,//input gain
	0x00a8e717, 0xff1c748d, 0x0065b976,
	0xffcbccab, 0xff190aff, 0x006cc1cf,
	0xff871ce1, 0xff10d878, 0x0078cfc5,
	0x00000001,//output gain
	0x00179204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00235102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_176to22: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c5102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000001,//output gain
	0x00185102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_176to24: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c0102,//header
	0x00000138,//input gain
	0x00d5d232, 0xff2a3bf8, 0x005a785c,
	0x0034001b, 0xff283109, 0x006462a6,
	0xffe6746a, 0xff1fb09c, 0x00758a91,
	0x00000001,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00235102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_176to32: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c0102,//header
	0x000005f3,//input gain
	0x00d816d6, 0xff385383, 0x004fe566,
	0x003c548d, 0xff38c23d, 0x005d0b1c,
	0xfff02f7d, 0xff31e983, 0x0072d65d,
	0x00000001,//output gain
	0x00179204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_176to44: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c5102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_176to48: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c0102,//header
	0x00001685,//input gain
	0x00f53ae9, 0xff52f196, 0x003e3e08,
	0x00b9f857, 0xff5d8985, 0x0050070a,
	0x008c3e86, 0xff6053f0, 0x006d98ef,
	0x00000001,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_176to88: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00005102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001//output gain
];

static mut coef_176to96: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005103,//header
	0x000001e0,//input gain
	0x00de44c0, 0xff380b7f, 0x004ffc73,
	0x00494b44, 0xff3d493a, 0x005908bf,
	0xffe9a3c8, 0xff425647, 0x006745f7,
	0xffc42d61, 0xff40a6c7, 0x00776709,
	0x00000001//output gain
];

static mut coef_176to192: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000002,//output gain
	0x00005204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000
];

static mut coef_192to16: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c9102,//header
	0x0000007d,//input gain
	0x007d1f20, 0xff1a540e, 0x00678bf9,
	0xff916625, 0xff16b0ff, 0x006e433a,
	0xff5af660, 0xff0eb91f, 0x00797356,
	0x00000001,//output gain
	0x00185102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_192to22: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c0102,//header
	0x000000af,//input gain
	0x00c65663, 0xff23d2ce, 0x005f97d6,
	0x00086ad6, 0xff20ec4f, 0x00683201,
	0xffbbbef6, 0xff184447, 0x00770963,
	0x00000001,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00235102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_192to24: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c5102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000001,//output gain
	0x00185102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_192to32: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c9102,//header
	0x000005d6,//input gain
	0x00c6543e, 0xff342935, 0x0052f116,
	0x000a1d78, 0xff3330c0, 0x005f88a3,
	0xffbee7c0, 0xff2b5ba5, 0x0073eb26,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_192to44: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x000000af,//input gain
	0x00c65663, 0xff23d2ce, 0x005f97d6,
	0x00086ad6, 0xff20ec4f, 0x00683201,
	0xffbbbef6, 0xff184447, 0x00770963,
	0x00000002,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00235102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_192to48: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c5102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001,//output gain
	0x00005102,//header
	0x0001d727,//input gain
	0x00fc2fc7, 0xff9bb27b, 0x001c564c,
	0x00e55557, 0xffcadd5b, 0x003d80ba,
	0x00d13397, 0xfff232f8, 0x00683337,
	0x00000001//output gain
];

static mut coef_192to88: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x000000af,//input gain
	0x00c65663, 0xff23d2ce, 0x005f97d6,
	0x00086ad6, 0xff20ec4f, 0x00683201,
	0xffbbbef6, 0xff184447, 0x00770963,
	0x00000002,//output gain
	0x00175204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x000013d9,//input gain
	0x00ebd477, 0xff4ce383, 0x0042049d,
	0x0089c278, 0xff54414d, 0x00531ded,
	0x004a5e07, 0xff53cf41, 0x006efbdc,
	0x00000001//output gain
];

static mut coef_192to96: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x00005103,//header
	0x000001e0,//input gain
	0x00de44c0, 0xff380b7f, 0x004ffc73,
	0x00494b44, 0xff3d493a, 0x005908bf,
	0xffe9a3c8, 0xff425647, 0x006745f7,
	0xffc42d61, 0xff40a6c7, 0x00776709,
	0x00000001//output gain
];

static mut coef_192to176: [u32; TEGRA210_SFC_COEF_RAM_DEPTH as usize] = [
	0x000c6102,//header
	0x000000af,//input gain
	0x00c65663, 0xff23d2ce, 0x005f97d6,
	0x00086ad6, 0xff20ec4f, 0x00683201,
	0xffbbbef6, 0xff184447, 0x00770963,
	0x00000002,//output gain
	0x00170204,//farrow
	0x000aaaab,
	0xffaaaaab,
	0xfffaaaab,
	0x00555555,
	0xff600000,
	0xfff55555,
	0x00155555,
	0x00055555,
	0xffeaaaab,
	0x00200000,
	0x00005102,//header
	0x0000010a,//input gain
	0x00c93dc4, 0xff26f5f6, 0x005d1041,
	0x001002c4, 0xff245b76, 0x00666002,
	0xffc30a45, 0xff1baecd, 0x00765921,
	0x00000001//output gain
];

/*
 * Coefficient table for various sample rate conversions. The sample
 * rates available are as per tegra210_sfc_rates[].
 */
static mut coef_addr_table: [[*mut i32; TEGRA210_SFC_NUM_RATES as usize]; TEGRA210_SFC_NUM_RATES as usize] = [
	/* Convertions from 8 kHz */
	[
		core::ptr::null_mut(),
		coef_8to11.as_mut_ptr(),
		coef_8to16.as_mut_ptr(),
		coef_8to22.as_mut_ptr(),
		coef_8to24.as_mut_ptr(),
		coef_8to32.as_mut_ptr(),
		coef_8to44.as_mut_ptr(),
		coef_8to48.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_8to88.as_mut_ptr(),
		coef_8to96.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
	],
	/* Convertions from 11.025 kHz */
	[
		coef_11to8.as_mut_ptr(),
		core::ptr::null_mut(),
		coef_11to16.as_mut_ptr(),
		coef_11to22.as_mut_ptr(),
		coef_11to24.as_mut_ptr(),
		coef_11to32.as_mut_ptr(),
		coef_11to44.as_mut_ptr(),
		coef_11to48.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_11to88.as_mut_ptr(),
		coef_11to96.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
	],
	/* Convertions from 16 kHz */
	[
		coef_16to8.as_mut_ptr(),
		coef_16to11.as_mut_ptr(),
		core::ptr::null_mut(),
		coef_16to22.as_mut_ptr(),
		coef_16to24.as_mut_ptr(),
		coef_16to32.as_mut_ptr(),
		coef_16to44.as_mut_ptr(),
		coef_16to48.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_16to88.as_mut_ptr(),
		coef_16to96.as_mut_ptr(),
		coef_16to176.as_mut_ptr(),
		coef_16to192.as_mut_ptr(),
	],
	/* Convertions from 22.05 kHz */
	[
		coef_22to8.as_mut_ptr(),
		coef_22to11.as_mut_ptr(),
		coef_22to16.as_mut_ptr(),
		core::ptr::null_mut(),
		coef_22to24.as_mut_ptr(),
		coef_22to32.as_mut_ptr(),
		coef_22to44.as_mut_ptr(),
		coef_22to48.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_22to88.as_mut_ptr(),
		coef_22to96.as_mut_ptr(),
		coef_22to176.as_mut_ptr(),
		coef_22to192.as_mut_ptr(),
	],
	/* Convertions from 24 kHz */
	[
		coef_24to8.as_mut_ptr(),
		coef_24to11.as_mut_ptr(),
		coef_24to16.as_mut_ptr(),
		coef_24to22.as_mut_ptr(),
		core::ptr::null_mut(),
		coef_24to32.as_mut_ptr(),
		coef_24to44.as_mut_ptr(),
		coef_24to48.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_24to88.as_mut_ptr(),
		coef_24to96.as_mut_ptr(),
		coef_24to176.as_mut_ptr(),
		coef_24to192.as_mut_ptr(),
	],
	/* Convertions from 32 kHz */
	[
		coef_32to8.as_mut_ptr(),
		coef_32to11.as_mut_ptr(),
		coef_32to16.as_mut_ptr(),
		coef_32to22.as_mut_ptr(),
		coef_32to24.as_mut_ptr(),
		core::ptr::null_mut(),
		coef_32to44.as_mut_ptr(),
		coef_32to48.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_32to88.as_mut_ptr(),
		coef_32to96.as_mut_ptr(),
		coef_32to176.as_mut_ptr(),
		coef_32to192.as_mut_ptr(),
	],
	/* Convertions from 44.1 kHz */
	[
		coef_44to8.as_mut_ptr(),
		coef_44to11.as_mut_ptr(),
		coef_44to16.as_mut_ptr(),
		coef_44to22.as_mut_ptr(),
		coef_44to24.as_mut_ptr(),
		coef_44to32.as_mut_ptr(),
		core::ptr::null_mut(),
		coef_44to48.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_44to88.as_mut_ptr(),
		coef_44to96.as_mut_ptr(),
		coef_44to176.as_mut_ptr(),
		coef_44to192.as_mut_ptr(),
	],
	/* Convertions from 48 kHz */
	[
		coef_48to8.as_mut_ptr(),
		coef_48to11.as_mut_ptr(),
		coef_48to16.as_mut_ptr(),
		coef_48to22.as_mut_ptr(),
		coef_48to24.as_mut_ptr(),
		coef_48to32.as_mut_ptr(),
		coef_48to44.as_mut_ptr(),
		core::ptr::null_mut(),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_48to88.as_mut_ptr(),
		coef_48to96.as_mut_ptr(),
		coef_48to176.as_mut_ptr(),
		coef_48to192.as_mut_ptr(),
	],
	/* Convertions from 64 kHz */
	[
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
	],
	/* Convertions from 88.2 kHz */
	[
		coef_88to8.as_mut_ptr(),
		coef_88to11.as_mut_ptr(),
		coef_88to16.as_mut_ptr(),
		coef_88to22.as_mut_ptr(),
		coef_88to24.as_mut_ptr(),
		coef_88to32.as_mut_ptr(),
		coef_88to44.as_mut_ptr(),
		coef_88to48.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		core::ptr::null_mut(),
		coef_88to96.as_mut_ptr(),
		coef_88to176.as_mut_ptr(),
		coef_88to192.as_mut_ptr(),
	],
	/* Convertions from 96 kHz */
	[	coef_96to8.as_mut_ptr(),
		coef_96to11.as_mut_ptr(),
		coef_96to16.as_mut_ptr(),
		coef_96to22.as_mut_ptr(),
		coef_96to24.as_mut_ptr(),
		coef_96to32.as_mut_ptr(),
		coef_96to44.as_mut_ptr(),
		coef_96to48.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_96to88.as_mut_ptr(),
		core::ptr::null_mut(),
		coef_96to176.as_mut_ptr(),
		coef_96to192.as_mut_ptr(),
	],
	/* Convertions from 176.4 kHz */
	[
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_176to16.as_mut_ptr(),
		coef_176to22.as_mut_ptr(),
		coef_176to24.as_mut_ptr(),
		coef_176to32.as_mut_ptr(),
		coef_176to44.as_mut_ptr(),
		coef_176to48.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_176to88.as_mut_ptr(),
		coef_176to96.as_mut_ptr(),
		core::ptr::null_mut(),
		coef_176to192.as_mut_ptr(),
	],
	/* Convertions from 192 kHz */
	[
		((-EOPNOTSUPP) as isize as *mut i32),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_192to16.as_mut_ptr(),
		coef_192to22.as_mut_ptr(),
		coef_192to24.as_mut_ptr(),
		coef_192to32.as_mut_ptr(),
		coef_192to44.as_mut_ptr(),
		coef_192to48.as_mut_ptr(),
		((-EOPNOTSUPP) as isize as *mut i32),
		coef_192to88.as_mut_ptr(),
		coef_192to96.as_mut_ptr(),
		coef_192to176.as_mut_ptr(),
		core::ptr::null_mut(),
	],
];

unsafe fn tegra210_sfc_runtime_suspend(dev: *mut device) -> c_int {
    let sfc = dev_get_drvdata(dev) as *mut tegra210_sfc;

    regcache_cache_only((*sfc).regmap, true);
    regcache_mark_dirty((*sfc).regmap);

    0
}

unsafe fn tegra210_sfc_runtime_resume(dev: *mut device) -> c_int {
    let sfc = dev_get_drvdata(dev) as *mut tegra210_sfc;

    regcache_cache_only((*sfc).regmap, false);
    regcache_sync((*sfc).regmap);

    0
}

unsafe fn tegra210_sfc_write_ram(regmap: *mut regmap, data: *mut i32) {
    regmap_write(
        regmap,
        TEGRA210_SFC_CFG_RAM_CTRL,
        TEGRA210_SFC_RAM_CTRL_SEQ_ACCESS_EN
            | TEGRA210_SFC_RAM_CTRL_ADDR_INIT_EN
            | TEGRA210_SFC_RAM_CTRL_RW_WRITE,
    );

    let mut i = 0;
    while i < TEGRA210_SFC_COEF_RAM_DEPTH {
        regmap_write(regmap, TEGRA210_SFC_CFG_RAM_DATA, *data.add(i as usize) as c_uint);
        i += 1;
    }
}

unsafe fn tegra210_sfc_write_coeff_ram(cmpnt: *mut snd_soc_component) -> c_int {
    let sfc = dev_get_drvdata((*cmpnt).dev) as *mut tegra210_sfc;
    let coeff_ram: *mut i32;

    /* Bypass */
    if (*sfc).srate_in == (*sfc).srate_out {
        return 0;
    }

    coeff_ram = coef_addr_table[(*sfc).srate_in as usize][(*sfc).srate_out as usize];
    if IS_ERR_OR_NULL(coeff_ram as *const c_void) {
        dev_err(
            (*cmpnt).dev,
            b"Conversion from %d to %d Hz is not supported\n\0".as_ptr() as *const c_char,
            (*sfc).srate_in,
            (*sfc).srate_out,
        );

        return PTR_ERR_OR_ZERO(coeff_ram as *const c_void);
    }

    tegra210_sfc_write_ram((*sfc).regmap, coeff_ram);

    regmap_update_bits(
        (*sfc).regmap,
        TEGRA210_SFC_COEF_RAM,
        TEGRA210_SFC_COEF_RAM_EN,
        TEGRA210_SFC_COEF_RAM_EN,
    );

    0
}

unsafe fn tegra210_sfc_set_audio_cif(
    sfc: *mut tegra210_sfc,
    params: *mut snd_pcm_hw_params,
    reg: c_uint,
) -> c_int {
    let channels: c_uint;
    let audio_bits: c_uint;
    let path: c_uint;
    let mut cif_conf: tegra_cif_conf = zeroed();

    channels = params_channels(params);

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            audio_bits = TEGRA_ACIF_BITS_16;
        }
        SNDRV_PCM_FORMAT_S24_LE | SNDRV_PCM_FORMAT_S32_LE => {
            audio_bits = TEGRA_ACIF_BITS_32;
        }
        _ => {
            return -EOPNOTSUPP;
        }
    }

    cif_conf.audio_ch = channels;
    cif_conf.client_ch = channels;
    cif_conf.audio_bits = audio_bits;
    cif_conf.client_bits = TEGRA_ACIF_BITS_32;

    if reg == TEGRA210_SFC_RX_CIF_CTRL {
        path = SFC_RX_PATH;
    } else {
        path = SFC_TX_PATH;
    }

    cif_conf.stereo_conv = (*sfc).stereo_to_mono[path as usize];
    cif_conf.mono_conv = (*sfc).mono_to_stereo[path as usize];

    tegra_set_cif((*sfc).regmap, reg, &mut cif_conf);

    0
}

unsafe fn tegra210_sfc_soft_reset(sfc: *mut tegra210_sfc) -> c_int {
    let mut val: u32 = 0;

    /*
     * Soft Reset: Below performs module soft reset which clears
     * all FSM logic, flushes flow control of FIFO and resets the
     * state register. It also brings module back to disabled
     * state (without flushing the data in the pipe).
     */
    regmap_update_bits(
        (*sfc).regmap,
        TEGRA210_SFC_SOFT_RESET,
        TEGRA210_SFC_SOFT_RESET_EN,
        1,
    );

    regmap_read_poll_timeout(
        (*sfc).regmap,
        TEGRA210_SFC_SOFT_RESET,
        &mut val,
        TEGRA210_SFC_SOFT_RESET_EN,
        10,
        10000,
    )
}

unsafe fn tegra210_sfc_rate_to_idx(dev: *mut device, rate: c_int, rate_idx: *mut c_int) -> c_int {
    let mut i = 0usize;

    while i < tegra210_sfc_rates.len() {
        if rate == tegra210_sfc_rates[i] {
            *rate_idx = i as c_int;
            return 0;
        }
        i += 1;
    }

    dev_err(
        dev,
        b"Sample rate %d Hz is not supported\n\0".as_ptr() as *const c_char,
        rate,
    );

    -EOPNOTSUPP
}

unsafe fn tegra210_sfc_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let sfc = snd_soc_dai_get_drvdata(dai) as *mut tegra210_sfc;
    let err: c_int;

    regmap_update_bits(
        (*sfc).regmap,
        TEGRA210_SFC_COEF_RAM,
        TEGRA210_SFC_COEF_RAM_EN,
        0,
    );

    err = tegra210_sfc_soft_reset(sfc);
    if err < 0 {
        dev_err(
            (*dai).dev,
            b"Failed to reset SFC in %s, err = %d\n\0".as_ptr() as *const c_char,
            b"tegra210_sfc_startup\0".as_ptr() as *const c_char,
            err,
        );
        return err;
    }

    0
}

unsafe fn tegra210_sfc_in_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let sfc = snd_soc_dai_get_drvdata(dai) as *mut tegra210_sfc;
    let dev = (*dai).dev;
    let mut err: c_int;

    err = tegra210_sfc_rate_to_idx(dev, params_rate(params), &mut (*sfc).srate_in);
    if err < 0 {
        return err;
    }

    err = tegra210_sfc_set_audio_cif(sfc, params, TEGRA210_SFC_RX_CIF_CTRL);
    if err < 0 {
        dev_err(dev, b"Can't set SFC RX CIF: %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    regmap_write((*sfc).regmap, TEGRA210_SFC_RX_FREQ, (*sfc).srate_in as c_uint);

    err
}

unsafe fn tegra210_sfc_out_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let sfc = snd_soc_dai_get_drvdata(dai) as *mut tegra210_sfc;
    let dev = (*dai).dev;
    let mut err: c_int;

    err = tegra210_sfc_rate_to_idx(dev, params_rate(params), &mut (*sfc).srate_out);
    if err < 0 {
        return err;
    }

    err = tegra210_sfc_set_audio_cif(sfc, params, TEGRA210_SFC_TX_CIF_CTRL);
    if err < 0 {
        dev_err(dev, b"Can't set SFC TX CIF: %d\n\0".as_ptr() as *const c_char, err);
        return err;
    }

    regmap_write((*sfc).regmap, TEGRA210_SFC_TX_FREQ, (*sfc).srate_out as c_uint);

    0
}

unsafe fn tegra210_sfc_init(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    _event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);

    tegra210_sfc_write_coeff_ram(cmpnt)
}

unsafe fn tegra210_sfc_get_conv(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
    tx: bool,
    mono_to_stereo: bool,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let sfc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_sfc;
    let path = if tx { SFC_TX_PATH } else { SFC_RX_PATH } as usize;

    (*ucontrol).value.enumerated.item[0] = if mono_to_stereo {
        (*sfc).mono_to_stereo[path]
    } else {
        (*sfc).stereo_to_mono[path]
    };

    0
}

unsafe fn tegra210_sfc_put_conv(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
    tx: bool,
    mono_to_stereo: bool,
) -> c_int {
    let cmpnt = snd_kcontrol_chip(kcontrol) as *mut snd_soc_component;
    let sfc = snd_soc_component_get_drvdata(cmpnt) as *mut tegra210_sfc;
    let value = (*ucontrol).value.enumerated.item[0];
    let path = if tx { SFC_TX_PATH } else { SFC_RX_PATH } as usize;
    let slot = if mono_to_stereo {
        &mut (*sfc).mono_to_stereo[path]
    } else {
        &mut (*sfc).stereo_to_mono[path]
    };

    if value == *slot {
        return 0;
    }

    *slot = value;
    1
}

unsafe fn tegra210_sfc_iget_stereo_to_mono(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    tegra210_sfc_get_conv(k, u, false, false)
}
unsafe fn tegra210_sfc_iput_stereo_to_mono(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    tegra210_sfc_put_conv(k, u, false, false)
}
unsafe fn tegra210_sfc_iget_mono_to_stereo(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    tegra210_sfc_get_conv(k, u, false, true)
}
unsafe fn tegra210_sfc_iput_mono_to_stereo(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    tegra210_sfc_put_conv(k, u, false, true)
}
unsafe fn tegra210_sfc_oget_stereo_to_mono(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    tegra210_sfc_get_conv(k, u, true, false)
}
unsafe fn tegra210_sfc_oput_stereo_to_mono(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    tegra210_sfc_put_conv(k, u, true, false)
}
unsafe fn tegra210_sfc_oget_mono_to_stereo(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    tegra210_sfc_get_conv(k, u, true, true)
}
unsafe fn tegra210_sfc_oput_mono_to_stereo(k: *mut snd_kcontrol, u: *mut snd_ctl_elem_value) -> c_int {
    tegra210_sfc_put_conv(k, u, true, true)
}

static tegra210_sfc_in_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra210_sfc_in_hw_params),
    startup: Some(tegra210_sfc_startup),
};

static tegra210_sfc_out_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tegra210_sfc_out_hw_params),
    startup: None,
};

/* The following compound initializers translate the ASoC registration tables
 * from C. Their concrete field layouts and helper macros are supplied by the
 * surrounding driver framework.
 */
static mut tegra210_sfc_dais: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: b"SFC-RX-CIF\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"RX-CIF-Playback\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"RX-CIF-Capture\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
        },
        ops: &tegra210_sfc_in_dai_ops,
    },
    snd_soc_dai_driver {
        name: b"SFC-TX-CIF\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"TX-CIF-Playback\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"TX-CIF-Capture\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: SNDRV_PCM_FMTBIT_S8 | SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
        },
        ops: &tegra210_sfc_out_dai_ops,
    },
];

static tegra210_sfc_widgets: [snd_soc_dapm_widget; 2] = [
    SND_SOC_DAPM_AIF_IN(b"RX\0".as_ptr() as *const c_char, core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT_E(
        b"TX\0".as_ptr() as *const c_char,
        core::ptr::null(),
        0,
        TEGRA210_SFC_ENABLE,
        TEGRA210_SFC_EN_SHIFT,
        0,
        Some(tegra210_sfc_init),
        SND_SOC_DAPM_PRE_PMU,
    ),
];

macro_rules! RESAMPLE_ROUTE {
    ($sname:expr) => {
        [
            snd_soc_dapm_route { sink: concat!("RX XBAR-", $sname, "\0").as_ptr() as *const c_char, control: core::ptr::null(), source: b"XBAR-TX\0".as_ptr() as *const c_char },
            snd_soc_dapm_route { sink: concat!("RX-CIF-", $sname, "\0").as_ptr() as *const c_char, control: core::ptr::null(), source: concat!("RX XBAR-", $sname, "\0").as_ptr() as *const c_char },
            snd_soc_dapm_route { sink: b"RX\0".as_ptr() as *const c_char, control: core::ptr::null(), source: concat!("RX-CIF-", $sname, "\0").as_ptr() as *const c_char },
            snd_soc_dapm_route { sink: concat!("TX-CIF-", $sname, "\0").as_ptr() as *const c_char, control: core::ptr::null(), source: b"TX\0".as_ptr() as *const c_char },
            snd_soc_dapm_route { sink: concat!("TX XBAR-", $sname, "\0").as_ptr() as *const c_char, control: core::ptr::null(), source: concat!("TX-CIF-", $sname, "\0").as_ptr() as *const c_char },
            snd_soc_dapm_route { sink: b"XBAR-RX\0".as_ptr() as *const c_char, control: core::ptr::null(), source: concat!("TX XBAR-", $sname, "\0").as_ptr() as *const c_char },
        ]
    };
}

static tegra210_sfc_routes: [snd_soc_dapm_route; 13] = [
    snd_soc_dapm_route { sink: b"TX\0".as_ptr() as *const c_char, control: core::ptr::null(), source: b"RX\0".as_ptr() as *const c_char },
    RESAMPLE_ROUTE!("Playback")[0], RESAMPLE_ROUTE!("Playback")[1], RESAMPLE_ROUTE!("Playback")[2],
    RESAMPLE_ROUTE!("Playback")[3], RESAMPLE_ROUTE!("Playback")[4], RESAMPLE_ROUTE!("Playback")[5],
    RESAMPLE_ROUTE!("Capture")[0], RESAMPLE_ROUTE!("Capture")[1], RESAMPLE_ROUTE!("Capture")[2],
    RESAMPLE_ROUTE!("Capture")[3], RESAMPLE_ROUTE!("Capture")[4], RESAMPLE_ROUTE!("Capture")[5],
];

static tegra210_sfc_stereo_conv_text: [*const c_char; 3] = [
    b"CH0\0".as_ptr() as *const c_char,
    b"CH1\0".as_ptr() as *const c_char,
    b"AVG\0".as_ptr() as *const c_char,
];

static tegra210_sfc_mono_conv_text: [*const c_char; 2] = [
    b"Zero\0".as_ptr() as *const c_char,
    b"Copy\0".as_ptr() as *const c_char,
];

static tegra210_sfc_stereo_conv_enum: soc_enum =
    SOC_ENUM_SINGLE(SND_SOC_NOPM, 0, tegra210_sfc_stereo_conv_text.len() as c_uint, tegra210_sfc_stereo_conv_text.as_ptr());

static tegra210_sfc_mono_conv_enum: soc_enum =
    SOC_ENUM_SINGLE(SND_SOC_NOPM, 0, tegra210_sfc_mono_conv_text.len() as c_uint, tegra210_sfc_mono_conv_text.as_ptr());

static tegra210_sfc_controls: [snd_kcontrol_new; 4] = [
    SOC_ENUM_EXT(b"Input Stereo To Mono\0".as_ptr() as *const c_char, tegra210_sfc_stereo_conv_enum, Some(tegra210_sfc_iget_stereo_to_mono), Some(tegra210_sfc_iput_stereo_to_mono)),
    SOC_ENUM_EXT(b"Input Mono To Stereo\0".as_ptr() as *const c_char, tegra210_sfc_mono_conv_enum, Some(tegra210_sfc_iget_mono_to_stereo), Some(tegra210_sfc_iput_mono_to_stereo)),
    SOC_ENUM_EXT(b"Output Stereo To Mono\0".as_ptr() as *const c_char, tegra210_sfc_stereo_conv_enum, Some(tegra210_sfc_oget_stereo_to_mono), Some(tegra210_sfc_oput_stereo_to_mono)),
    SOC_ENUM_EXT(b"Output Mono To Stereo\0".as_ptr() as *const c_char, tegra210_sfc_mono_conv_enum, Some(tegra210_sfc_oget_mono_to_stereo), Some(tegra210_sfc_oput_mono_to_stereo)),
];

static tegra210_sfc_cmpnt: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: tegra210_sfc_widgets.as_ptr(),
    num_dapm_widgets: tegra210_sfc_widgets.len() as c_uint,
    dapm_routes: tegra210_sfc_routes.as_ptr(),
    num_dapm_routes: tegra210_sfc_routes.len() as c_uint,
    controls: tegra210_sfc_controls.as_ptr(),
    num_controls: tegra210_sfc_controls.len() as c_uint,
};

unsafe fn tegra210_sfc_wr_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        r if r >= TEGRA210_SFC_RX_INT_MASK && r <= TEGRA210_SFC_RX_FREQ => true,
        r if r >= TEGRA210_SFC_TX_INT_MASK && r <= TEGRA210_SFC_TX_FREQ => true,
        r if r >= TEGRA210_SFC_ENABLE && r <= TEGRA210_SFC_CG => true,
        r if r >= TEGRA210_SFC_COEF_RAM && r <= TEGRA210_SFC_CFG_RAM_DATA => true,
        _ => false,
    }
}

unsafe fn tegra210_sfc_rd_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        r if r >= TEGRA210_SFC_RX_STATUS && r <= TEGRA210_SFC_RX_FREQ => true,
        r if r >= TEGRA210_SFC_TX_STATUS && r <= TEGRA210_SFC_TX_FREQ => true,
        r if r >= TEGRA210_SFC_ENABLE && r <= TEGRA210_SFC_INT_STATUS => true,
        r if r >= TEGRA210_SFC_COEF_RAM && r <= TEGRA210_SFC_CFG_RAM_DATA => true,
        _ => false,
    }
}

unsafe fn tegra210_sfc_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TEGRA210_SFC_RX_STATUS | TEGRA210_SFC_RX_INT_STATUS | TEGRA210_SFC_RX_INT_SET |
        TEGRA210_SFC_TX_STATUS | TEGRA210_SFC_TX_INT_STATUS | TEGRA210_SFC_TX_INT_SET |
        TEGRA210_SFC_SOFT_RESET | TEGRA210_SFC_STATUS | TEGRA210_SFC_INT_STATUS |
        TEGRA210_SFC_CFG_RAM_CTRL | TEGRA210_SFC_CFG_RAM_DATA => true,
        _ => false,
    }
}

unsafe fn tegra210_sfc_precious_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        TEGRA210_SFC_CFG_RAM_DATA => true,
        _ => false,
    }
}

static tegra210_sfc_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: TEGRA210_SFC_CFG_RAM_DATA,
    writeable_reg: Some(tegra210_sfc_wr_reg),
    readable_reg: Some(tegra210_sfc_rd_reg),
    volatile_reg: Some(tegra210_sfc_volatile_reg),
    precious_reg: Some(tegra210_sfc_precious_reg),
    reg_defaults: tegra210_sfc_reg_defaults.as_ptr(),
    num_reg_defaults: tegra210_sfc_reg_defaults.len() as c_uint,
    reg_default_cb: Some(regmap_default_zero_cb),
    cache_type: REGCACHE_FLAT,
};

static tegra210_sfc_of_match: [of_device_id; 2] = [
    of_device_id { compatible: b"nvidia,tegra210-sfc\0".as_ptr() as *const c_char },
    of_device_id { compatible: core::ptr::null() },
];

unsafe fn tegra210_sfc_platform_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let sfc: *mut tegra210_sfc;
    let regs: *mut c_void;
    let err: c_int;

    sfc = devm_kzalloc(dev, core::mem::size_of::<tegra210_sfc>(), GFP_KERNEL) as *mut tegra210_sfc;
    if sfc.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, sfc as *mut c_void);

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    (*sfc).regmap = devm_regmap_init_mmio(dev, regs, &tegra210_sfc_regmap_config);
    if IS_ERR((*sfc).regmap as *const c_void) {
        return dev_err_probe(dev, PTR_ERR((*sfc).regmap as *const c_void), b"regmap init failed\n\0".as_ptr() as *const c_char);
    }

    regcache_cache_only((*sfc).regmap, true);

    err = devm_snd_soc_register_component(
        dev,
        &tegra210_sfc_cmpnt,
        tegra210_sfc_dais.as_mut_ptr(),
        tegra210_sfc_dais.len() as c_uint,
    );
    if err != 0 {
        return dev_err_probe(dev, err, b"can't register SFC component\n\0".as_ptr() as *const c_char);
    }

    pm_runtime_enable(&mut (*pdev).dev);

    0
}

unsafe fn tegra210_sfc_platform_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

static tegra210_sfc_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(tegra210_sfc_runtime_suspend),
    runtime_resume: Some(tegra210_sfc_runtime_resume),
    suspend: Some(pm_runtime_force_suspend),
    resume: Some(pm_runtime_force_resume),
};

static mut tegra210_sfc_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"tegra210-sfc\0".as_ptr() as *const c_char,
        of_match_table: tegra210_sfc_of_match.as_ptr(),
        pm: &tegra210_sfc_pm_ops,
    },
    probe: Some(tegra210_sfc_platform_probe),
    remove: Some(tegra210_sfc_platform_remove),
};

module_platform_driver!(tegra210_sfc_driver);

MODULE_AUTHOR!("Arun Shamanna Lakshmi <aruns@nvidia.com>");
MODULE_DESCRIPTION!("Tegra210 SFC ASoC driver");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
