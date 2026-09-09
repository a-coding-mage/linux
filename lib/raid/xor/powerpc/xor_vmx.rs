// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 * Copyright (C) IBM Corporation, 2012
 *
 * Author: Anton Blanchard <anton@au.ibm.com>
 */

/* Translation of the Altivec/VMX XOR implementation. */

type Unative = [i8; 16];

#[inline]
unsafe fn load(v: *const Unative) -> [Unative; 4] {
    [*v, *v.add(1), *v.add(2), *v.add(3)]
}

#[inline]
unsafe fn store(v: *mut Unative, value: [Unative; 4]) {
    *v = value[0];
    *v.add(1) = value[1];
    *v.add(2) = value[2];
    *v.add(3) = value[3];
}

#[inline]
fn vec_xor(a: Unative, b: Unative) -> Unative {
    let mut result = [0i8; 16];
    let mut i = 0;
    while i < 16 {
        result[i] = a[i] ^ b[i];
        i += 1;
    }
    result
}

#[inline]
fn xor4(a: &mut [Unative; 4], b: &[Unative; 4]) {
    a[0] = vec_xor(a[0], b[0]);
    a[1] = vec_xor(a[1], b[1]);
    a[2] = vec_xor(a[2], b[2]);
    a[3] = vec_xor(a[3], b[3]);
}

unsafe fn __xor_altivec_2(
    bytes: usize,
    v1_in: *mut core::ffi::c_ulong,
    v2_in: *const core::ffi::c_ulong,
) {
    let mut v1 = v1_in as *mut Unative;
    let mut v2 = v2_in as *const Unative;
    let mut lines = bytes / core::mem::size_of::<Unative>() / 4;

    loop {
        let mut v1_ = load(v1);
        let v2_ = load(v2);
        xor4(&mut v1_, &v2_);
        store(v1, v1_);

        v1 = v1.add(4);
        v2 = v2.add(4);
        lines = lines.wrapping_sub(1);
        if lines == 0 { break; }
    }
}

unsafe fn __xor_altivec_3(
    bytes: usize,
    v1_in: *mut core::ffi::c_ulong,
    v2_in: *const core::ffi::c_ulong,
    v3_in: *const core::ffi::c_ulong,
) {
    let mut v1 = v1_in as *mut Unative;
    let mut v2 = v2_in as *const Unative;
    let mut v3 = v3_in as *const Unative;
    let mut lines = bytes / core::mem::size_of::<Unative>() / 4;

    loop {
        let mut v1_ = load(v1);
        let v2_ = load(v2);
        let v3_ = load(v3);
        xor4(&mut v1_, &v2_);
        xor4(&mut v1_, &v3_);
        store(v1, v1_);
        v1 = v1.add(4); v2 = v2.add(4); v3 = v3.add(4);
        lines = lines.wrapping_sub(1);
        if lines == 0 { break; }
    }
}

unsafe fn __xor_altivec_4(
    bytes: usize, v1_in: *mut core::ffi::c_ulong,
    v2_in: *const core::ffi::c_ulong, v3_in: *const core::ffi::c_ulong,
    v4_in: *const core::ffi::c_ulong,
) {
    let (mut v1, mut v2, mut v3, mut v4) = (v1_in as *mut Unative, v2_in as *const Unative, v3_in as *const Unative, v4_in as *const Unative);
    let mut lines = bytes / core::mem::size_of::<Unative>() / 4;
    loop {
        let mut a = load(v1); let b = load(v2); let c = load(v3); let d = load(v4);
        xor4(&mut a, &b); xor4(&mut c, &d); xor4(&mut a, &c); store(v1, a);
        v1=v1.add(4); v2=v2.add(4); v3=v3.add(4); v4=v4.add(4);
        lines=lines.wrapping_sub(1); if lines==0 { break; }
    }
}

unsafe fn __xor_altivec_5(
    bytes: usize, v1_in: *mut core::ffi::c_ulong,
    v2_in: *const core::ffi::c_ulong, v3_in: *const core::ffi::c_ulong,
    v4_in: *const core::ffi::c_ulong, v5_in: *const core::ffi::c_ulong,
) {
    let (mut v1, mut v2, mut v3, mut v4, mut v5) = (v1_in as *mut Unative, v2_in as *const Unative, v3_in as *const Unative, v4_in as *const Unative, v5_in as *const Unative);
    let mut lines = bytes / core::mem::size_of::<Unative>() / 4;
    loop {
        let mut a=load(v1); let b=load(v2); let mut c=load(v3); let d=load(v4); let e=load(v5);
        xor4(&mut a,&b); xor4(&mut c,&d); xor4(&mut a,&e); xor4(&mut a,&c); store(v1,a);
        v1=v1.add(4); v2=v2.add(4); v3=v3.add(4); v4=v4.add(4); v5=v5.add(4);
        lines=lines.wrapping_sub(1); if lines==0 { break; }
    }
}

// __DO_XOR_BLOCKS(altivec_inner, __xor_altivec_2, __xor_altivec_3,
//                 __xor_altivec_4, __xor_altivec_5) is supplied by xor_impl.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
