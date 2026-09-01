/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * Isolate byte #n and put it into byte #m, for __u##b type.
 * E.g., moving byte #6 (nnnnnnnn) into byte #1 (mmmmmmmm) for __u64:
 * 1) xxxxxxxx nnnnnnnn xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx mmmmmmmm xxxxxxxx
 * 2) nnnnnnnn xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx mmmmmmmm xxxxxxxx 00000000
 * 3) 00000000 00000000 00000000 00000000 00000000 00000000 00000000 nnnnnnnn
 * 4) 00000000 00000000 00000000 00000000 00000000 00000000 nnnnnnnn 00000000
 */
#[macro_export]
macro_rules! ___bpf_mvb {
    ($x:expr, 16, $n:expr, $m:expr) => {
        (($x as __u16) << (16 - (($n + 1) * 8)) >> (16 - 8) << ($m * 8))
    };
    ($x:expr, 32, $n:expr, $m:expr) => {
        (($x as __u32) << (32 - (($n + 1) * 8)) >> (32 - 8) << ($m * 8))
    };
    ($x:expr, 64, $n:expr, $m:expr) => {
        (($x as __u64) << (64 - (($n + 1) * 8)) >> (64 - 8) << ($m * 8))
    };
}

#[macro_export]
macro_rules! ___bpf_swab16 {
    ($x:expr) => {
        ((___bpf_mvb!($x, 16, 0, 1) | ___bpf_mvb!($x, 16, 1, 0)) as __u16)
    };
}

#[macro_export]
macro_rules! ___bpf_swab32 {
    ($x:expr) => {
        ((
            ___bpf_mvb!($x, 32, 0, 3)
                | ___bpf_mvb!($x, 32, 1, 2)
                | ___bpf_mvb!($x, 32, 2, 1)
                | ___bpf_mvb!($x, 32, 3, 0)
        ) as __u32)
    };
}

#[macro_export]
macro_rules! ___bpf_swab64 {
    ($x:expr) => {
        ((
            ___bpf_mvb!($x, 64, 0, 7)
                | ___bpf_mvb!($x, 64, 1, 6)
                | ___bpf_mvb!($x, 64, 2, 5)
                | ___bpf_mvb!($x, 64, 3, 4)
                | ___bpf_mvb!($x, 64, 4, 3)
                | ___bpf_mvb!($x, 64, 5, 2)
                | ___bpf_mvb!($x, 64, 6, 1)
                | ___bpf_mvb!($x, 64, 7, 0)
        ) as __u64)
    };
}

/* LLVM's BPF target selects the endianness of the CPU
 * it compiles on, or the user specifies (bpfel/bpfeb),
 * respectively. The used __BYTE_ORDER__ is defined by
 * the compiler, we cannot rely on __BYTE_ORDER from
 * libc headers, since it doesn't reflect the actual
 * requested byte order.
 *
 * Note, LLVM's BPF target has different __builtin_bswapX()
 * semantics. It does map to BPF_ALU | BPF_END | BPF_TO_BE
 * in bpfel and bpfeb case, which means below, that we map
 * to cpu_to_be16(). We could use it unconditionally in BPF
 * case, but better not rely on it, so that this header here
 * can be used from application and BPF program side, which
 * use different targets.
 */
#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! __bpf_ntohs {
    ($x:expr) => {
        ($x).swap_bytes()
    };
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! __bpf_htons {
    ($x:expr) => {
        ($x).swap_bytes()
    };
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! __bpf_constant_ntohs {
    ($x:expr) => {
        ___bpf_swab16!($x)
    };
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! __bpf_constant_htons {
    ($x:expr) => {
        ___bpf_swab16!($x)
    };
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! __bpf_ntohl {
    ($x:expr) => {
        ($x).swap_bytes()
    };
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! __bpf_htonl {
    ($x:expr) => {
        ($x).swap_bytes()
    };
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! __bpf_constant_ntohl {
    ($x:expr) => {
        ___bpf_swab32!($x)
    };
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! __bpf_constant_htonl {
    ($x:expr) => {
        ___bpf_swab32!($x)
    };
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! __bpf_be64_to_cpu {
    ($x:expr) => {
        ($x).swap_bytes()
    };
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! __bpf_cpu_to_be64 {
    ($x:expr) => {
        ($x).swap_bytes()
    };
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! __bpf_constant_be64_to_cpu {
    ($x:expr) => {
        ___bpf_swab64!($x)
    };
}

#[cfg(target_endian = "little")]
#[macro_export]
macro_rules! __bpf_constant_cpu_to_be64 {
    ($x:expr) => {
        ___bpf_swab64!($x)
    };
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! __bpf_ntohs {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! __bpf_htons {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! __bpf_constant_ntohs {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! __bpf_constant_htons {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! __bpf_ntohl {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! __bpf_htonl {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! __bpf_constant_ntohl {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! __bpf_constant_htonl {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! __bpf_be64_to_cpu {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! __bpf_cpu_to_be64 {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! __bpf_constant_be64_to_cpu {
    ($x:expr) => {
        ($x)
    };
}

#[cfg(target_endian = "big")]
#[macro_export]
macro_rules! __bpf_constant_cpu_to_be64 {
    ($x:expr) => {
        ($x)
    };
}

#[macro_export]
macro_rules! bpf_htons {
    ($x:expr) => {
        __bpf_htons!($x)
    };
}

#[macro_export]
macro_rules! bpf_ntohs {
    ($x:expr) => {
        __bpf_ntohs!($x)
    };
}

#[macro_export]
macro_rules! bpf_htonl {
    ($x:expr) => {
        __bpf_htonl!($x)
    };
}

#[macro_export]
macro_rules! bpf_ntohl {
    ($x:expr) => {
        __bpf_ntohl!($x)
    };
}

#[macro_export]
macro_rules! bpf_cpu_to_be64 {
    ($x:expr) => {
        __bpf_cpu_to_be64!($x)
    };
}

#[macro_export]
macro_rules! bpf_be64_to_cpu {
    ($x:expr) => {
        __bpf_be64_to_cpu!($x)
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
