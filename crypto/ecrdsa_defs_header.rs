/* SPDX-License-Identifier: GPL-2.0+ */
/* Definitions of EC-RDSA Curve Parameters */

// Dependency supplied by the surrounding ECC implementation.
// The C header included <crypto/internal/ecc.h>.

pub const ECRDSA_MAX_SIG_SIZE: usize = 2 * 512 / 8;
pub const ECRDSA_MAX_DIGITS: usize = 512 / 64;

/* EC-RDSA uses its own set of curves. */
/* OID_gostCPSignA 1.2.643.2.2.35.1 */
static mut cp256a_g_x: [u64; 4] = [0x0000000000000001, 0, 0, 0];
static mut cp256a_g_y: [u64; 4] = [0x22ACC99C9E9F1E14, 0x35294F2DDF23E3B1, 0x27DF505A453F2B76, 0x8D91E471E0989CDA];
static mut cp256a_p: [u64; 4] = [0xFFFFFFFFFFFFFD97, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
static mut cp256a_n: [u64; 4] = [0x45841B09B761B893, 0x6C611070995AD100, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
static mut cp256a_a: [u64; 4] = [0xFFFFFFFFFFFFFD94, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
static mut cp256a_b: [u64; 4] = [0xA6, 0, 0, 0];

static mut gost_cp256a: ecc_curve = ecc_curve { name: b"cp256a\0".as_ptr(), nbits: 256, g: ecc_point { x: cp256a_g_x.as_ptr(), y: cp256a_g_y.as_ptr(), ndigits: 256 / 64 }, p: cp256a_p.as_ptr(), n: cp256a_n.as_ptr(), a: cp256a_a.as_ptr(), b: cp256a_b.as_ptr() };

/* OID_gostCPSignB 1.2.643.2.2.35.2 */
static mut cp256b_g_x: [u64; 4] = [1, 0, 0, 0];
static mut cp256b_g_y: [u64; 4] = [0x744BF8D717717EFC, 0xC545C9858D03ECFB, 0xB83D1C3EB2C070E5, 0x3FA8124359F96680];
static mut cp256b_p: [u64; 4] = [0xC99, 0, 0, 0x8000000000000000];
static mut cp256b_n: [u64; 4] = [0xE497161BCC8A198F, 0x5F700CFFF1A624E5, 1, 0x8000000000000000];
static mut cp256b_a: [u64; 4] = [0xC96, 0, 0, 0x8000000000000000];
static mut cp256b_b: [u64; 4] = [0x2F49D4CE7E1BBC8B, 0xE979259373FF2B18, 0x66A7D3C25C3DF80A, 0x3E1AF419A269A5F8];
static mut gost_cp256b: ecc_curve = ecc_curve { name: b"cp256b\0".as_ptr(), nbits: 256, g: ecc_point { x: cp256b_g_x.as_ptr(), y: cp256b_g_y.as_ptr(), ndigits: 256 / 64 }, p: cp256b_p.as_ptr(), n: cp256b_n.as_ptr(), a: cp256b_a.as_ptr(), b: cp256b_b.as_ptr() };

/* OID_gostCPSignC 1.2.643.2.2.35.3 */
static mut cp256c_g_x: [u64; 4] = [0, 0, 0, 0];
static mut cp256c_g_y: [u64; 4] = [0x366E550DFDB3BB67, 0x4D4DC440D4641A8F, 0x3CBF3783CD08C0EE, 0x41ECE55743711A8C];
static mut cp256c_p: [u64; 9] = [0x7998F7B9022D759B, 0xCF846E86789051D3, 0xAB1EC85E6B41C8AA, 0x9B9F605F5A858107, 0xedc283cdd217b5a2, 0xbac48fc06398ae59, 0x405384d55f9f3b73, 0xa51f176161f1d734, 1];
static mut cp256c_n: [u64; 4] = [0xF02F3A6598980BB9, 0x582CA3511EDDFB74, 0xAB1EC85E6B41C8AA, 0x9B9F605F5A858107];
static mut cp256c_a: [u64; 4] = [0x7998F7B9022D7598, 0xCF846E86789051D3, 0xAB1EC85E6B41C8AA, 0x9B9F605F5A858107];
static mut cp256c_b: [u64; 4] = [0x805A, 0, 0, 0];
static mut gost_cp256c: ecc_curve = ecc_curve { name: b"cp256c\0".as_ptr(), nbits: 256, g: ecc_point { x: cp256c_g_x.as_ptr(), y: cp256c_g_y.as_ptr(), ndigits: 256 / 64 }, p: cp256c_p.as_ptr(), n: cp256c_n.as_ptr(), a: cp256c_a.as_ptr(), b: cp256c_b.as_ptr() };

/* OID_gostTC26Sign512A 1.2.643.7.1.2.1.2.1 */
static mut tc512a_g_x: [u64; 8] = [3, 0, 0, 0, 0, 0, 0, 0];
static mut tc512a_g_y: [u64; 8] = [0x89A589CB5215F2A4, 0x8028FE5FC235F5B8, 0x3D75E6A50E3A41E9, 0xDF1626BE4FD036E9, 0x778064FDCBEFA921, 0xCE5E1C93ACF1ABC1, 0xA61B8816E25450E6, 0x7503CFE87A836AE3];
static mut tc512a_p: [u64; 8] = [0xFFFFFFFFFFFFFDC7, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
static mut tc512a_n: [u64; 8] = [0xCACDB1411F10B275, 0x9B4B38ABFAD2B85D, 0x6FF22B8D4E056060, 0x27E69532F48D8911, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
static mut tc512a_a: [u64; 8] = [0xFFFFFFFFFFFFFDC4, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
static mut tc512a_b: [u64; 8] = [0x503190785A71C760, 0x862EF9D4EBEE4761, 0x4CB4574010DA90DD, 0xEE3CB090F30D2761, 0x79BD081CFD0B6265, 0x34B82574761CB0E8, 0xC1BD0B2B6667F1DA, 0xE8C2505DEDFC86DD];
static mut gost_tc512a: ecc_curve = ecc_curve { name: b"tc512a\0".as_ptr(), nbits: 512, g: ecc_point { x: tc512a_g_x.as_ptr(), y: tc512a_g_y.as_ptr(), ndigits: 512 / 64 }, p: tc512a_p.as_ptr(), n: tc512a_n.as_ptr(), a: tc512a_a.as_ptr(), b: tc512a_b.as_ptr() };

/* OID_gostTC26Sign512B 1.2.643.7.1.2.1.2.2 */
static mut tc512b_g_x: [u64; 8] = [2, 0, 0, 0, 0, 0, 0, 0];
static mut tc512b_g_y: [u64; 8] = [0x7E21340780FE41BD, 0x28041055F94CEEEC, 0x152CBCAAF8C03988, 0xDCB228FD1EDF4A39, 0xBE6DD9E6C8EC7335, 0x3C123B697578C213, 0x2C071E3647A8940F, 0x1A8F7EDA389B094C];
static mut tc512b_p: [u64; 8] = [0x6F, 0, 0, 0, 0, 0, 0, 0x8000000000000000];
static mut tc512b_n: [u64; 8] = [0xC6346C54374F25BD, 0x8B996712101BEA0E, 0xACFDB77BD9D40CFA, 0x49A1EC142565A545, 1, 0, 0, 0x8000000000000000];
static mut tc512b_a: [u64; 8] = [0x6C, 0, 0, 0, 0, 0, 0, 0x8000000000000000];
static mut tc512b_b: [u64; 8] = [0xFB8CCBC7C5140116, 0x50F78BEE1FA3106E, 0x7F8B276FAD1AB69C, 0x3E965D2DB1416D21, 0xBF85DC806C4B289F, 0xB97C7D614AF138BC, 0x7E3E06CF6F5E2517, 0x687D1B459DC84145];
static mut gost_tc512b: ecc_curve = ecc_curve { name: b"tc512b\0".as_ptr(), nbits: 512, g: ecc_point { x: tc512b_g_x.as_ptr(), y: tc512b_g_y.as_ptr(), ndigits: 512 / 64 }, p: tc512b_p.as_ptr(), n: tc512b_n.as_ptr(), a: tc512b_a.as_ptr(), b: tc512b_b.as_ptr() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
