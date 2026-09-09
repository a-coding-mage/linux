/* SPDX-License-Identifier: GPL-2.0 */

pub const SPARC_CR_OPCODE_PRIORITY: u32 = 300;

#[inline]
pub const fn f3f(x: u32, y: u32, z: u32) -> u32 {
    (x << 30) | (y << 19) | (z << 5)
}

#[inline]
pub const fn fpd_encode(x: u32) -> u32 {
    (x >> 5) | (x & !0x20)
}

#[inline]
pub const fn rs1(x: u32) -> u32 { fpd_encode(x) << 14 }
#[inline]
pub const fn rs2(x: u32) -> u32 { fpd_encode(x) }
#[inline]
pub const fn rs3(x: u32) -> u32 { fpd_encode(x) << 9 }
#[inline]
pub const fn rd(x: u32) -> u32 { fpd_encode(x) << 25 }
#[inline]
pub const fn imm5_0(x: u32) -> u32 { x }
#[inline]
pub const fn imm5_9(x: u32) -> u32 { x << 9 }

#[inline]
pub const fn crc32c(a: u32, b: u32, c: u32) -> u32 {
    f3f(2, 0x36, 0x147) | rs1(a) | rs2(b) | rd(c)
}

pub const MD5: u32 = 0x81b02800;
pub const SHA1: u32 = 0x81b02820;
pub const SHA256: u32 = 0x81b02840;
pub const SHA512: u32 = 0x81b02860;

#[inline]
pub const fn aes_eround01(a: u32, b: u32, c: u32, d: u32) -> u32 {
    f3f(2, 0x19, 0) | rs1(a) | rs2(b) | rs3(c) | rd(d)
}
#[inline]
pub const fn aes_eround23(a: u32, b: u32, c: u32, d: u32) -> u32 {
    f3f(2, 0x19, 1) | rs1(a) | rs2(b) | rs3(c) | rd(d)
}
#[inline]
pub const fn aes_dround01(a: u32, b: u32, c: u32, d: u32) -> u32 {
    f3f(2, 0x19, 2) | rs1(a) | rs2(b) | rs3(c) | rd(d)
}
#[inline]
pub const fn aes_dround23(a: u32, b: u32, c: u32, d: u32) -> u32 {
    f3f(2, 0x19, 3) | rs1(a) | rs2(b) | rs3(c) | rd(d)
}
#[inline]
pub const fn aes_eround01_l(a: u32, b: u32, c: u32, d: u32) -> u32 {
    f3f(2, 0x19, 4) | rs1(a) | rs2(b) | rs3(c) | rd(d)
}
#[inline]
pub const fn aes_eround23_l(a: u32, b: u32, c: u32, d: u32) -> u32 {
    f3f(2, 0x19, 5) | rs1(a) | rs2(b) | rs3(c) | rd(d)
}
#[inline]
pub const fn aes_dround01_l(a: u32, b: u32, c: u32, d: u32) -> u32 {
    f3f(2, 0x19, 6) | rs1(a) | rs2(b) | rs3(c) | rd(d)
}
#[inline]
pub const fn aes_dround23_l(a: u32, b: u32, c: u32, d: u32) -> u32 {
    f3f(2, 0x19, 7) | rs1(a) | rs2(b) | rs3(c) | rd(d)
}
#[inline]
pub const fn aes_kexpand1(a: u32, b: u32, c: u32, d: u32) -> u32 {
    f3f(2, 0x19, 8) | rs1(a) | rs2(b) | imm5_9(c) | rd(d)
}
#[inline]
pub const fn aes_kexpand0(a: u32, b: u32, c: u32) -> u32 {
    f3f(2, 0x36, 0x130) | rs1(a) | rs2(b) | rd(c)
}
#[inline]
pub const fn aes_kexpand2(a: u32, b: u32, c: u32) -> u32 {
    f3f(2, 0x36, 0x131) | rs1(a) | rs2(b) | rd(c)
}

#[inline]
pub const fn des_ip(a: u32, b: u32) -> u32 {
    f3f(2, 0x36, 0x134) | rs1(a) | rd(b)
}
#[inline]
pub const fn des_iip(a: u32, b: u32) -> u32 {
    f3f(2, 0x36, 0x135) | rs1(a) | rd(b)
}
#[inline]
pub const fn des_kexpand(a: u32, b: u32, c: u32) -> u32 {
    f3f(2, 0x36, 0x136) | rs1(a) | imm5_0(b) | rd(c)
}
#[inline]
pub const fn des_round(a: u32, b: u32, c: u32, d: u32) -> u32 {
    f3f(2, 0x19, 0x009) | rs1(a) | rs2(b) | rs3(c) | rd(d)
}

#[inline]
pub const fn camellia_f(a: u32, b: u32, c: u32, d: u32) -> u32 {
    f3f(2, 0x19, 0x00c) | rs1(a) | rs2(b) | rs3(c) | rd(d)
}
#[inline]
pub const fn camellia_fl(a: u32, b: u32, c: u32) -> u32 {
    f3f(2, 0x36, 0x13c) | rs1(a) | rs2(b) | rd(c)
}
#[inline]
pub const fn camellia_fli(a: u32, b: u32, c: u32) -> u32 {
    f3f(2, 0x36, 0x13d) | rs1(a) | rs2(b) | rd(c)
}

pub const MOVDTOX_F0_O4: u32 = 0x99b02200;
pub const MOVDTOX_F2_O5: u32 = 0x9bb02202;
pub const MOVXTOD_G1_F60: u32 = 0xbbb02301;
pub const MOVXTOD_G1_F62: u32 = 0xbfb02301;
pub const MOVXTOD_G3_F4: u32 = 0x89b02303;
pub const MOVXTOD_G7_F6: u32 = 0x8db02307;
pub const MOVXTOD_G3_F0: u32 = 0x81b02303;
pub const MOVXTOD_G7_F2: u32 = 0x85b02307;
pub const MOVXTOD_O0_F0: u32 = 0x81b02308;
pub const MOVXTOD_O5_F0: u32 = 0x81b0230d;
pub const MOVXTOD_O5_F2: u32 = 0x85b0230d;
pub const MOVXTOD_O5_F4: u32 = 0x89b0230d;
pub const MOVXTOD_O5_F6: u32 = 0x8db0230d;
pub const MOVXTOD_G3_F60: u32 = 0xbbb02303;
pub const MOVXTOD_G7_F62: u32 = 0xbfb02307;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
