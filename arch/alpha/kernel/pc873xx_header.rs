/* SPDX-License-Identifier: GPL-2.0 */

// Control Register Values
pub const REG_FER: u32 = 0x00;
pub const REG_FAR: u32 = 0x01;
pub const REG_PTR: u32 = 0x02;
pub const REG_FCR: u32 = 0x03;
pub const REG_PCR: u32 = 0x04;
pub const REG_KRR: u32 = 0x05;
pub const REG_PMC: u32 = 0x06;
pub const REG_TUP: u32 = 0x07;
pub const REG_SID: u32 = 0x08;
pub const REG_ASC: u32 = 0x09;
pub const REG_IRC: u32 = 0x0e;

// Model numbers
pub const PC87303: u32 = 0;
pub const PC87306: u32 = 1;
pub const PC87312: u32 = 2;
pub const PC87332: u32 = 3;
pub const PC87334: u32 = 4;

extern "C" {
    pub fn pc873xx_probe() -> ::core::ffi::c_int;
    pub fn pc873xx_get_base() -> ::core::ffi::c_uint;
    pub fn pc873xx_get_model() -> *mut ::core::ffi::c_char;
    pub fn pc873xx_enable_epp19();
    pub fn pc873xx_enable_ide();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
