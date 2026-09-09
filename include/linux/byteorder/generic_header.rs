/* SPDX-License-Identifier: GPL-2.0 */

/*
 * linux/byteorder/generic.h
 * Generic Byte-reordering support
 *
 * The conversion macros and inline helpers below preserve the generic
 * byte-order interface. Architecture-specific implementations are supplied
 * by the corresponding byte-order dependency.
 *
 * TODO:
 *   = Regular kernel maintainers could also replace all these manual
 *    byteswap macros that remain, disseminated among drivers,
 *    after some grep or the sources...
 *   = Linus might want to rename all these macros and files to fit his taste,
 *    to fit his personal naming scheme.
 *   = it seems that a few drivers would also appreciate
 *    nybble swapping support...
 *   = every architecture could add their byteswap macro in asm/byteorder.h
 *    see how some architectures already do (i386, alpha, ppc, etc)
 *   = cpu_to_beXX and beXX_to_cpu might some day need to be well
 *    distinguished throughout the kernel. This is not the case currently,
 *    since little endian, big endian, and pdp endian machines needn't it.
 *    But this might be the case for, say, a port of Linux to 20/21 bit
 *    architectures (and F21 Linux addict around?).
 */

macro_rules! cpu_to_le64 { ($x:expr) => { __cpu_to_le64($x) }; }
macro_rules! le64_to_cpu { ($x:expr) => { __le64_to_cpu($x) }; }
macro_rules! cpu_to_le32 { ($x:expr) => { __cpu_to_le32($x) }; }
macro_rules! le32_to_cpu { ($x:expr) => { __le32_to_cpu($x) }; }
macro_rules! cpu_to_le16 { ($x:expr) => { __cpu_to_le16($x) }; }
macro_rules! le16_to_cpu { ($x:expr) => { __le16_to_cpu($x) }; }
macro_rules! cpu_to_be64 { ($x:expr) => { __cpu_to_be64($x) }; }
macro_rules! be64_to_cpu { ($x:expr) => { __be64_to_cpu($x) }; }
macro_rules! cpu_to_be32 { ($x:expr) => { __cpu_to_be32($x) }; }
macro_rules! be32_to_cpu { ($x:expr) => { __be32_to_cpu($x) }; }
macro_rules! cpu_to_be16 { ($x:expr) => { __cpu_to_be16($x) }; }
macro_rules! be16_to_cpu { ($x:expr) => { __be16_to_cpu($x) }; }
macro_rules! cpu_to_le64p { ($x:expr) => { __cpu_to_le64p($x) }; }
macro_rules! le64_to_cpup { ($x:expr) => { __le64_to_cpup($x) }; }
macro_rules! cpu_to_le32p { ($x:expr) => { __cpu_to_le32p($x) }; }
macro_rules! le32_to_cpup { ($x:expr) => { __le32_to_cpup($x) }; }
macro_rules! cpu_to_le16p { ($x:expr) => { __cpu_to_le16p($x) }; }
macro_rules! le16_to_cpup { ($x:expr) => { __le16_to_cpup($x) }; }
macro_rules! cpu_to_be64p { ($x:expr) => { __cpu_to_be64p($x) }; }
macro_rules! be64_to_cpup { ($x:expr) => { __be64_to_cpup($x) }; }
macro_rules! cpu_to_be32p { ($x:expr) => { __cpu_to_be32p($x) }; }
macro_rules! be32_to_cpup { ($x:expr) => { __be32_to_cpup($x) }; }
macro_rules! cpu_to_be16p { ($x:expr) => { __cpu_to_be16p($x) }; }
macro_rules! be16_to_cpup { ($x:expr) => { __be16_to_cpup($x) }; }
macro_rules! cpu_to_le64s { ($x:expr) => { __cpu_to_le64s($x) }; }
macro_rules! le64_to_cpus { ($x:expr) => { __le64_to_cpus($x) }; }
macro_rules! cpu_to_le32s { ($x:expr) => { __cpu_to_le32s($x) }; }
macro_rules! le32_to_cpus { ($x:expr) => { __le32_to_cpus($x) }; }
macro_rules! cpu_to_le16s { ($x:expr) => { __cpu_to_le16s($x) }; }
macro_rules! le16_to_cpus { ($x:expr) => { __le16_to_cpus($x) }; }
macro_rules! cpu_to_be64s { ($x:expr) => { __cpu_to_be64s($x) }; }
macro_rules! be64_to_cpus { ($x:expr) => { __be64_to_cpus($x) }; }
macro_rules! cpu_to_be32s { ($x:expr) => { __cpu_to_be32s($x) }; }
macro_rules! be32_to_cpus { ($x:expr) => { __be32_to_cpus($x) }; }
macro_rules! cpu_to_be16s { ($x:expr) => { __cpu_to_be16s($x) }; }
macro_rules! be16_to_cpus { ($x:expr) => { __be16_to_cpus($x) }; }

macro_rules! ___htonl { ($x:expr) => { __cpu_to_be32($x) }; }
macro_rules! ___htons { ($x:expr) => { __cpu_to_be16($x) }; }
macro_rules! ___ntohl { ($x:expr) => { __be32_to_cpu($x) }; }
macro_rules! ___ntohs { ($x:expr) => { __be16_to_cpu($x) }; }
macro_rules! htonl { ($x:expr) => { ___htonl!($x) }; }
macro_rules! ntohl { ($x:expr) => { ___ntohl!($x) }; }
macro_rules! htons { ($x:expr) => { ___htons!($x) }; }
macro_rules! ntohs { ($x:expr) => { ___ntohs!($x) }; }

pub unsafe fn le16_add_cpu(var: *mut __le16, val: u16) {
    *var = cpu_to_le16!(__le16_to_cpu(*var).wrapping_add(val));
}

pub unsafe fn le32_add_cpu(var: *mut __le32, val: u32) {
    *var = cpu_to_le32!(__le32_to_cpu(*var).wrapping_add(val));
}

pub unsafe fn le64_add_cpu(var: *mut __le64, val: u64) {
    *var = cpu_to_le64!(__le64_to_cpu(*var).wrapping_add(val));
}

/* XXX: this stuff can be optimized */
pub unsafe fn le32_to_cpu_array(mut buf: *mut u32, mut words: u32) {
    while words != 0 {
        __le32_to_cpus(buf);
        words -= 1;
        buf = buf.add(1);
    }
}

pub unsafe fn cpu_to_le32_array(mut buf: *mut u32, mut words: u32) {
    while words != 0 {
        __cpu_to_le32s(buf);
        words -= 1;
        buf = buf.add(1);
    }
}

pub unsafe fn le64_to_cpu_array(mut buf: *mut u64, mut words: u32) {
    while words != 0 {
        __le64_to_cpus(buf);
        words -= 1;
        buf = buf.add(1);
    }
}

pub unsafe fn cpu_to_le64_array(mut buf: *mut u64, mut words: u32) {
    while words != 0 {
        __cpu_to_le64s(buf);
        words -= 1;
        buf = buf.add(1);
    }
}

pub unsafe fn memcpy_from_le32(dst: *mut u32, src: *const __le32, words: usize) {
    for i in 0..words { *dst.add(i) = le32_to_cpu!(*src.add(i)); }
}

pub unsafe fn memcpy_to_le32(dst: *mut __le32, src: *const u32, words: usize) {
    for i in 0..words { *dst.add(i) = cpu_to_le32!(*src.add(i)); }
}

pub unsafe fn be16_add_cpu(var: *mut __be16, val: u16) {
    *var = cpu_to_be16!(__be16_to_cpu(*var).wrapping_add(val));
}

pub unsafe fn be32_add_cpu(var: *mut __be32, val: u32) {
    *var = cpu_to_be32!(__be32_to_cpu(*var).wrapping_add(val));
}

pub unsafe fn be64_add_cpu(var: *mut __be64, val: u64) {
    *var = cpu_to_be64!(__be64_to_cpu(*var).wrapping_add(val));
}

pub unsafe fn cpu_to_be32_array(dst: *mut __be32, src: *const u32, len: usize) {
    for i in 0..len { *dst.add(i) = cpu_to_be32!(*src.add(i)); }
}

pub unsafe fn be32_to_cpu_array(dst: *mut u32, src: *const __be32, len: usize) {
    for i in 0..len { *dst.add(i) = be32_to_cpu!(*src.add(i)); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
