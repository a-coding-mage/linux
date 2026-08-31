// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

/*
 * Copyright (C) 2013-2015 Alexei Starovoitov <ast@kernel.org>
 * Copyright (C) 2015 Wang Nan <wangnan0@huawei.com>
 * Copyright (C) 2015 Huawei Inc.
 * Copyright (C) 2017 Nicira, Inc.
 */

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

// C includes translated as external dependency intent:
// <stdio.h>, <string.h>, <errno.h>, <inttypes.h>, <linux/kernel.h>,
// "libbpf.h", and "libbpf_internal.h".

pub type __u8 = u8;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __be32 = u32;
pub type __be64 = u64;
pub type size_t = usize;

pub const ENOTSUPP: c_int = 524;

unsafe extern "C" {
    fn strerror_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;

    fn libbpf_err(err: c_int) -> c_int;
    fn libbpf_err_errno(err: c_int) -> c_int;
}

const fn errno_offset(e: c_int) -> usize {
    (e - __LIBBPF_ERRNO__START) as usize
}

const fn nr_errno() -> usize {
    (__LIBBPF_ERRNO__END - __LIBBPF_ERRNO__START) as usize
}

const SHA256_BLOCK_LENGTH: usize = 64;

const fn ch(x: __u32, y: __u32, z: __u32) -> __u32 {
    (x & y) ^ (!x & z)
}

const fn maj(x: __u32, y: __u32, z: __u32) -> __u32 {
    (x & y) ^ (x & z) ^ (y & z)
}

const fn sigma_upper_0(x: __u32) -> __u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

const fn sigma_upper_1(x: __u32) -> __u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

const fn sigma_0(x: __u32) -> __u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

const fn sigma_1(x: __u32) -> __u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

unsafe fn cstr_bytes(s: &'static [u8]) -> *const c_char {
    s.as_ptr() as *const c_char
}

unsafe fn libbpf_strerror_msg(err: c_int) -> *const c_char {
    match err {
        LIBBPF_ERRNO__LIBELF => cstr_bytes(b"Something wrong in libelf\0"),
        LIBBPF_ERRNO__FORMAT => cstr_bytes(b"BPF object format invalid\0"),
        LIBBPF_ERRNO__KVERSION => cstr_bytes(b"'version' section incorrect or lost\0"),
        LIBBPF_ERRNO__ENDIAN => cstr_bytes(b"Endian mismatch\0"),
        LIBBPF_ERRNO__INTERNAL => cstr_bytes(b"Internal error in libbpf\0"),
        LIBBPF_ERRNO__RELOC => cstr_bytes(b"Relocation failed\0"),
        LIBBPF_ERRNO__VERIFY => cstr_bytes(b"Kernel verifier blocks program loading\0"),
        LIBBPF_ERRNO__PROG2BIG => cstr_bytes(b"Program too big\0"),
        LIBBPF_ERRNO__KVER => cstr_bytes(b"Incorrect kernel version\0"),
        LIBBPF_ERRNO__PROGTYPE => cstr_bytes(b"Kernel doesn't support this program type\0"),
        LIBBPF_ERRNO__WRNGPID => cstr_bytes(b"Wrong pid in netlink message\0"),
        LIBBPF_ERRNO__INVSEQ => cstr_bytes(b"Invalid netlink sequence\0"),
        LIBBPF_ERRNO__NLPARSE => cstr_bytes(b"Incorrect netlink message parsing\0"),
        _ => ptr::null(),
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbpf_strerror(mut err: c_int, buf: *mut c_char, size: size_t) -> c_int {
    let ret: c_int;

    if buf.is_null() || size == 0 {
        return libbpf_err(-EINVAL);
    }

    err = if err > 0 { err } else { -err };

    if err < __LIBBPF_ERRNO__START {
        ret = strerror_r(err, buf, size);
        *buf.add(size - 1) = 0;
        return libbpf_err_errno(ret);
    }

    if err < __LIBBPF_ERRNO__END {
        let msg = libbpf_strerror_msg(err);

        ret = snprintf(buf, size, cstr_bytes(b"%s\0"), msg);
        *buf.add(size - 1) = 0;
        /*
         * The length of the buf and msg is positive.
         * A negative number may be returned only when the
         * size exceeds INT_MAX. Not likely to appear.
         */
        if ret as size_t >= size {
            return libbpf_err(-ERANGE);
        }
        return 0;
    }

    ret = snprintf(buf, size, cstr_bytes(b"Unknown libbpf error %d\0"), err);
    *buf.add(size - 1) = 0;
    if ret as size_t >= size {
        return libbpf_err(-ERANGE);
    }
    libbpf_err(-ENOENT)
}

#[thread_local]
static mut LIBBPF_ERRSTR_BUF: [c_char; 12] = [0; 12];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbpf_errstr(mut err: c_int) -> *const c_char {
    if err > 0 {
        err = -err;
    }

    match err {
        x if x == -E2BIG => cstr_bytes(b"-E2BIG\0"),
        x if x == -EACCES => cstr_bytes(b"-EACCES\0"),
        x if x == -EADDRINUSE => cstr_bytes(b"-EADDRINUSE\0"),
        x if x == -EADDRNOTAVAIL => cstr_bytes(b"-EADDRNOTAVAIL\0"),
        x if x == -EAGAIN => cstr_bytes(b"-EAGAIN\0"),
        x if x == -EALREADY => cstr_bytes(b"-EALREADY\0"),
        x if x == -EBADF => cstr_bytes(b"-EBADF\0"),
        x if x == -EBADFD => cstr_bytes(b"-EBADFD\0"),
        x if x == -EBUSY => cstr_bytes(b"-EBUSY\0"),
        x if x == -ECANCELED => cstr_bytes(b"-ECANCELED\0"),
        x if x == -ECHILD => cstr_bytes(b"-ECHILD\0"),
        x if x == -EDEADLK => cstr_bytes(b"-EDEADLK\0"),
        x if x == -EDOM => cstr_bytes(b"-EDOM\0"),
        x if x == -EEXIST => cstr_bytes(b"-EEXIST\0"),
        x if x == -EFAULT => cstr_bytes(b"-EFAULT\0"),
        x if x == -EFBIG => cstr_bytes(b"-EFBIG\0"),
        x if x == -EILSEQ => cstr_bytes(b"-EILSEQ\0"),
        x if x == -EINPROGRESS => cstr_bytes(b"-EINPROGRESS\0"),
        x if x == -EINTR => cstr_bytes(b"-EINTR\0"),
        x if x == -EINVAL => cstr_bytes(b"-EINVAL\0"),
        x if x == -EIO => cstr_bytes(b"-EIO\0"),
        x if x == -EISDIR => cstr_bytes(b"-EISDIR\0"),
        x if x == -ELOOP => cstr_bytes(b"-ELOOP\0"),
        x if x == -EMFILE => cstr_bytes(b"-EMFILE\0"),
        x if x == -EMLINK => cstr_bytes(b"-EMLINK\0"),
        x if x == -EMSGSIZE => cstr_bytes(b"-EMSGSIZE\0"),
        x if x == -ENAMETOOLONG => cstr_bytes(b"-ENAMETOOLONG\0"),
        x if x == -ENFILE => cstr_bytes(b"-ENFILE\0"),
        x if x == -ENODATA => cstr_bytes(b"-ENODATA\0"),
        x if x == -ENODEV => cstr_bytes(b"-ENODEV\0"),
        x if x == -ENOENT => cstr_bytes(b"-ENOENT\0"),
        x if x == -ENOEXEC => cstr_bytes(b"-ENOEXEC\0"),
        x if x == -ENOLINK => cstr_bytes(b"-ENOLINK\0"),
        x if x == -ENOMEM => cstr_bytes(b"-ENOMEM\0"),
        x if x == -ENOSPC => cstr_bytes(b"-ENOSPC\0"),
        x if x == -ENOTBLK => cstr_bytes(b"-ENOTBLK\0"),
        x if x == -ENOTDIR => cstr_bytes(b"-ENOTDIR\0"),
        x if x == -ENOTSUPP => cstr_bytes(b"-ENOTSUPP\0"),
        x if x == -ENOTTY => cstr_bytes(b"-ENOTTY\0"),
        x if x == -ENXIO => cstr_bytes(b"-ENXIO\0"),
        x if x == -EOPNOTSUPP => cstr_bytes(b"-EOPNOTSUPP\0"),
        x if x == -EOVERFLOW => cstr_bytes(b"-EOVERFLOW\0"),
        x if x == -EPERM => cstr_bytes(b"-EPERM\0"),
        x if x == -EPIPE => cstr_bytes(b"-EPIPE\0"),
        x if x == -EPROTO => cstr_bytes(b"-EPROTO\0"),
        x if x == -EPROTONOSUPPORT => cstr_bytes(b"-EPROTONOSUPPORT\0"),
        x if x == -ERANGE => cstr_bytes(b"-ERANGE\0"),
        x if x == -EROFS => cstr_bytes(b"-EROFS\0"),
        x if x == -ESPIPE => cstr_bytes(b"-ESPIPE\0"),
        x if x == -ESRCH => cstr_bytes(b"-ESRCH\0"),
        x if x == -ETXTBSY => cstr_bytes(b"-ETXTBSY\0"),
        x if x == -EUCLEAN => cstr_bytes(b"-EUCLEAN\0"),
        x if x == -EXDEV => cstr_bytes(b"-EXDEV\0"),
        _ => {
            snprintf(
                LIBBPF_ERRSTR_BUF.as_mut_ptr(),
                size_of::<[c_char; 12]>(),
                cstr_bytes(b"%d\0"),
                err,
            );
            LIBBPF_ERRSTR_BUF.as_ptr()
        }
    }
}

unsafe fn get_unaligned_be32(p: *const c_void) -> __u32 {
    let mut val: __be32 = 0;

    ptr::copy_nonoverlapping(p as *const u8, &mut val as *mut __be32 as *mut u8, size_of::<__be32>());
    __u32::from_be(val)
}

unsafe fn put_unaligned_be32(val: __u32, p: *mut c_void) {
    let be_val: __be32 = val.to_be();

    ptr::copy_nonoverlapping(&be_val as *const __be32 as *const u8, p as *mut u8, size_of::<__be32>());
}

static SHA256_K: [__u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1,
    0x923f82a4, 0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
    0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786,
    0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147,
    0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
    0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
    0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a,
    0x5b9cca4f, 0x682e6ff3, 0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
    0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

macro_rules! sha256_round {
    ($i:expr, $a:ident, $b:ident, $c:ident, $d:ident, $e:ident, $f:ident, $g:ident, $h:ident, $w:ident) => {{
        let tmp: __u32 = $h
            .wrapping_add(sigma_upper_1($e))
            .wrapping_add(ch($e, $f, $g))
            .wrapping_add(SHA256_K[$i])
            .wrapping_add($w[$i]);
        $d = $d.wrapping_add(tmp);
        $h = tmp
            .wrapping_add(sigma_upper_0($a))
            .wrapping_add(maj($a, $b, $c));
    }};
}

unsafe fn sha256_blocks(state: *mut __u32, mut data: *const __u8, mut nblocks: size_t) {
    while {
        let old = nblocks;
        nblocks = nblocks.wrapping_sub(1);
        old != 0
    } {
        let mut a: __u32 = *state.add(0);
        let mut b: __u32 = *state.add(1);
        let mut c: __u32 = *state.add(2);
        let mut d: __u32 = *state.add(3);
        let mut e: __u32 = *state.add(4);
        let mut f: __u32 = *state.add(5);
        let mut g: __u32 = *state.add(6);
        let mut h: __u32 = *state.add(7);
        let mut w: [__u32; 64] = [0; 64];
        let mut i: c_int;

        i = 0;
        while i < 16 {
            w[i as usize] = get_unaligned_be32(data.add(4 * i as usize) as *const c_void);
            i += 1;
        }
        while (i as usize) < w.len() {
            w[i as usize] = sigma_1(w[i as usize - 2])
                .wrapping_add(w[i as usize - 7])
                .wrapping_add(sigma_0(w[i as usize - 15]))
                .wrapping_add(w[i as usize - 16]);
            i += 1;
        }
        i = 0;
        while (i as usize) < w.len() {
            let base = i as usize;
            sha256_round!(base + 0, a, b, c, d, e, f, g, h, w);
            sha256_round!(base + 1, h, a, b, c, d, e, f, g, w);
            sha256_round!(base + 2, g, h, a, b, c, d, e, f, w);
            sha256_round!(base + 3, f, g, h, a, b, c, d, e, w);
            sha256_round!(base + 4, e, f, g, h, a, b, c, d, w);
            sha256_round!(base + 5, d, e, f, g, h, a, b, c, w);
            sha256_round!(base + 6, c, d, e, f, g, h, a, b, w);
            sha256_round!(base + 7, b, c, d, e, f, g, h, a, w);
            i += 8;
        }
        *state.add(0) = (*state.add(0)).wrapping_add(a);
        *state.add(1) = (*state.add(1)).wrapping_add(b);
        *state.add(2) = (*state.add(2)).wrapping_add(c);
        *state.add(3) = (*state.add(3)).wrapping_add(d);
        *state.add(4) = (*state.add(4)).wrapping_add(e);
        *state.add(5) = (*state.add(5)).wrapping_add(f);
        *state.add(6) = (*state.add(6)).wrapping_add(g);
        *state.add(7) = (*state.add(7)).wrapping_add(h);
        data = data.add(SHA256_BLOCK_LENGTH);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn libbpf_sha256(data: *const c_void, len: size_t, out: *mut __u8) {
    let mut state: [__u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];
    let bitcount: __be64 = ((len as __u64).wrapping_mul(8)).to_be();
    let mut final_data: [__u8; 2 * SHA256_BLOCK_LENGTH] = [0; 2 * SHA256_BLOCK_LENGTH];
    let mut final_len: size_t = len % SHA256_BLOCK_LENGTH;
    let mut i: c_int;

    sha256_blocks(state.as_mut_ptr(), data as *const __u8, len / SHA256_BLOCK_LENGTH);

    ptr::copy_nonoverlapping(
        (data as *const __u8).add(len - final_len),
        final_data.as_mut_ptr(),
        final_len,
    );
    final_data[final_len] = 0x80;
    final_len = (final_len + 9 + SHA256_BLOCK_LENGTH - 1) / SHA256_BLOCK_LENGTH * SHA256_BLOCK_LENGTH;
    ptr::copy_nonoverlapping(
        &bitcount as *const __be64 as *const u8,
        final_data.as_mut_ptr().add(final_len - 8),
        8,
    );

    sha256_blocks(state.as_mut_ptr(), final_data.as_ptr(), final_len / SHA256_BLOCK_LENGTH);

    i = 0;
    while (i as usize) < state.len() {
        put_unaligned_be32(state[i as usize], out.add(4 * i as usize) as *mut c_void);
        i += 1;
    }
}
