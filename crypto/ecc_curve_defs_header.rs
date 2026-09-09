/* SPDX-License-Identifier: GPL-2.0 */

/* NIST P-192: a = p - 3 */
static mut nist_p192_g_x: [u64; 3] = [0xF4FF0AFD82FF1012, 0x7CBF20EB43A18800,
	0x188DA80EB03090F6];
static mut nist_p192_g_y: [u64; 3] = [0x73F977A11E794811, 0x631011ED6B24CDD5,
	0x07192B95FFC8DA78];
static mut nist_p192_p: [u64; 3] = [0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFE,
	0xFFFFFFFFFFFFFFFF];
static mut nist_p192_n: [u64; 3] = [0x146BC9B1B4D22831, 0xFFFFFFFF99DEF836,
	0xFFFFFFFFFFFFFFFF];
static mut nist_p192_a: [u64; 3] = [0xFFFFFFFFFFFFFFFC, 0xFFFFFFFFFFFFFFFE,
	0xFFFFFFFFFFFFFFFF];
static mut nist_p192_b: [u64; 3] = [0xFEB8DEECC146B9B1, 0x0FA7E9AB72243049,
	0x64210519E59C80E7];
static mut nist_p192: ecc_curve = ecc_curve {
	name: b"nist_192\0".as_ptr() as *const i8,
	nbits: 192,
	g: ecc_point { x: unsafe { nist_p192_g_x.as_mut_ptr() }, y: unsafe { nist_p192_g_y.as_mut_ptr() }, ndigits: 3 },
	p: unsafe { nist_p192_p.as_mut_ptr() }, n: unsafe { nist_p192_n.as_mut_ptr() },
	a: unsafe { nist_p192_a.as_mut_ptr() }, b: unsafe { nist_p192_b.as_mut_ptr() },
};

/* NIST P-256: a = p - 3 */
static mut nist_p256_g_x: [u64; 4] = [0xF4A13945D898C296, 0x77037D812DEB33A0, 0xF8BCE6E563A440F2, 0x6B17D1F2E12C4247];
static mut nist_p256_g_y: [u64; 4] = [0xCBB6406837BF51F5, 0x2BCE33576B315ECE, 0x8EE7EB4A7C0F9E16, 0x4FE342E2FE1A7F9B];
static mut nist_p256_p: [u64; 4] = [0xFFFFFFFFFFFFFFFF, 0x00000000FFFFFFFF, 0x0000000000000000, 0xFFFFFFFF00000001];
static mut nist_p256_n: [u64; 4] = [0xF3B9CAC2FC632551, 0xBCE6FAADA7179E84, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFF00000000];
static mut nist_p256_a: [u64; 4] = [0xFFFFFFFFFFFFFFFC, 0x00000000FFFFFFFF, 0x0000000000000000, 0xFFFFFFFF00000001];
static mut nist_p256_b: [u64; 4] = [0x3BCE3C3E27D2604B, 0x651D06B0CC53B0F6, 0xB3EBBD55769886BC, 0x5AC635D8AA3A93E7];
static mut nist_p256: ecc_curve = ecc_curve {
	name: b"nist_256\0".as_ptr() as *const i8, nbits: 256,
	g: ecc_point { x: unsafe { nist_p256_g_x.as_mut_ptr() }, y: unsafe { nist_p256_g_y.as_mut_ptr() }, ndigits: 4 },
	p: unsafe { nist_p256_p.as_mut_ptr() }, n: unsafe { nist_p256_n.as_mut_ptr() }, a: unsafe { nist_p256_a.as_mut_ptr() }, b: unsafe { nist_p256_b.as_mut_ptr() },
};

/* NIST P-384 */
static mut nist_p384_g_x: [u64; 6] = [0x3A545E3872760AB7, 0x5502F25DBF55296C, 0x59F741E082542A38, 0x6E1D3B628BA79B98, 0x8Eb1C71EF320AD74, 0xAA87CA22BE8B0537];
static mut nist_p384_g_y: [u64; 6] = [0x7A431D7C90EA0E5F, 0x0A60B1CE1D7E819D, 0xE9DA3113B5F0B8C0, 0xF8F41DBD289A147C, 0x5D9E98BF9292DC29, 0x3617DE4A96262C6F];
static mut nist_p384_p: [u64; 6] = [0x00000000FFFFFFFF, 0xFFFFFFFF00000000, 0xFFFFFFFFFFFFFFFE, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
static mut nist_p384_n: [u64; 6] = [0xECEC196ACCC52973, 0x581A0DB248B0A77A, 0xC7634D81F4372DDF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
static mut nist_p384_a: [u64; 6] = [0x00000000FFFFFFFC, 0xFFFFFFFF00000000, 0xFFFFFFFFFFFFFFFE, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF];
static mut nist_p384_b: [u64; 6] = [0x2a85c8edd3ec2aef, 0xc656398d8a2ed19d, 0x0314088f5013875a, 0x181d9c6efe814112, 0x988e056be3f82d19, 0xb3312fa7e23ee7e4];
static mut nist_p384: ecc_curve = ecc_curve {
	name: b"nist_384\0".as_ptr() as *const i8, nbits: 384,
	g: ecc_point { x: unsafe { nist_p384_g_x.as_mut_ptr() }, y: unsafe { nist_p384_g_y.as_mut_ptr() }, ndigits: 6 },
	p: unsafe { nist_p384_p.as_mut_ptr() }, n: unsafe { nist_p384_n.as_mut_ptr() }, a: unsafe { nist_p384_a.as_mut_ptr() }, b: unsafe { nist_p384_b.as_mut_ptr() },
};

/* NIST P-521 */
static mut nist_p521_g_x: [u64; 9] = [0xf97e7e31c2e5bd66, 0x3348b3c1856a429b, 0xfe1dc127a2ffa8de, 0xa14b5e77efe75928, 0xf828af606b4d3dba, 0x9c648139053fb521, 0x9e3ecb662395b442, 0x858e06b70404e9cd, 0xc6];
static mut nist_p521_g_y: [u64; 9] = [0x88be94769fd16650, 0x353c7086a272c240, 0xc550b9013fad0761, 0x97ee72995ef42640, 0x17afbd17273e662c, 0x98f54449579b4468, 0x5c8a5fb42c7d1bd9, 0x39296a789a3bc004, 0x118];
static mut nist_p521_p: [u64; 9] = [0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x1ff];
static mut nist_p521_n: [u64; 9] = [0xbb6fb71e91386409, 0x3bb5c9b8899c47ae, 0x7fcc0148f709a5d0, 0x51868783bf2f966b, 0xfffffffffffffffa, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x1ff];
static mut nist_p521_a: [u64; 9] = [0xfffffffffffffffc, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0xffffffffffffffff, 0x1ff];
static mut nist_p521_b: [u64; 9] = [0xef451fd46b503f00, 0x3573df883d2c34f1, 0x1652c0bd3bb1bf07, 0x56193951ec7e937b, 0xb8b489918ef109e1, 0xa2da725b99b315f3, 0x929a21a0b68540ee, 0x953eb9618e1c9a1f, 0x051];
static mut nist_p521: ecc_curve = ecc_curve {
	name: b"nist_521\0".as_ptr() as *const i8, nbits: 521,
	g: ecc_point { x: unsafe { nist_p521_g_x.as_mut_ptr() }, y: unsafe { nist_p521_g_y.as_mut_ptr() }, ndigits: 9 },
	p: unsafe { nist_p521_p.as_mut_ptr() }, n: unsafe { nist_p521_n.as_mut_ptr() }, a: unsafe { nist_p521_a.as_mut_ptr() }, b: unsafe { nist_p521_b.as_mut_ptr() },
};

/* curve25519 */
static mut curve25519_g_x: [u64; 4] = [0x0000000000000009, 0, 0, 0];
static mut curve25519_p: [u64; 4] = [0xffffffffffffffed, 0xffffffffffffffff, 0xffffffffffffffff, 0x7fffffffffffffff];
static mut curve25519_a: [u64; 4] = [0x000000000001DB41, 0, 0, 0];
static ecc_25519: ecc_curve = ecc_curve {
	name: b"curve25519\0".as_ptr() as *const i8, nbits: 255,
	g: ecc_point { x: unsafe { curve25519_g_x.as_mut_ptr() }, y: core::ptr::null_mut(), ndigits: 4 },
	p: unsafe { curve25519_p.as_mut_ptr() }, n: core::ptr::null_mut(), a: unsafe { curve25519_a.as_mut_ptr() }, b: core::ptr::null_mut(),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
