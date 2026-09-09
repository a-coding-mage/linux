/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependencies supplied by the corresponding Linux headers:
// linux/types.h, linux/swab.h, and the __BIG_ENDIAN configuration.

pub const __BIG_ENDIAN: u32 = 4321;

macro_rules! __constant_htonl { ($x:expr) => { $x as __be32 }; }
macro_rules! __constant_ntohl { ($x:expr) => { $x as __u32 }; }
macro_rules! __constant_htons { ($x:expr) => { $x as __be16 }; }
macro_rules! __constant_ntohs { ($x:expr) => { $x as __u16 }; }
macro_rules! __constant_cpu_to_le64 { ($x:expr) => { ___constant_swab64($x) as __le64 }; }
macro_rules! __constant_le64_to_cpu { ($x:expr) => { ___constant_swab64($x as __u64) }; }
macro_rules! __constant_cpu_to_le32 { ($x:expr) => { ___constant_swab32($x) as __le32 }; }
macro_rules! __constant_le32_to_cpu { ($x:expr) => { ___constant_swab32($x as __u32) }; }
macro_rules! __constant_cpu_to_le16 { ($x:expr) => { ___constant_swab16($x) as __le16 }; }
macro_rules! __constant_le16_to_cpu { ($x:expr) => { ___constant_swab16($x as __u16) }; }
macro_rules! __constant_cpu_to_be64 { ($x:expr) => { $x as __be64 }; }
macro_rules! __constant_be64_to_cpu { ($x:expr) => { $x as __u64 }; }
macro_rules! __constant_cpu_to_be32 { ($x:expr) => { $x as __be32 }; }
macro_rules! __constant_be32_to_cpu { ($x:expr) => { $x as __u32 }; }
macro_rules! __constant_cpu_to_be16 { ($x:expr) => { $x as __be16 }; }
macro_rules! __constant_be16_to_cpu { ($x:expr) => { $x as __u16 }; }

macro_rules! __cpu_to_le64 { ($x:expr) => { __swab64($x) as __le64 }; }
macro_rules! __le64_to_cpu { ($x:expr) => { __swab64($x as __u64) }; }
macro_rules! __cpu_to_le32 { ($x:expr) => { __swab32($x) as __le32 }; }
macro_rules! __le32_to_cpu { ($x:expr) => { __swab32($x as __u32) }; }
macro_rules! __cpu_to_le16 { ($x:expr) => { __swab16($x) as __le16 }; }
macro_rules! __le16_to_cpu { ($x:expr) => { __swab16($x as __u16) }; }
macro_rules! __cpu_to_be64 { ($x:expr) => { $x as __be64 }; }
macro_rules! __be64_to_cpu { ($x:expr) => { $x as __u64 }; }
macro_rules! __cpu_to_be32 { ($x:expr) => { $x as __be32 }; }
macro_rules! __be32_to_cpu { ($x:expr) => { $x as __u32 }; }
macro_rules! __cpu_to_be16 { ($x:expr) => { $x as __be16 }; }
macro_rules! __be16_to_cpu { ($x:expr) => { $x as __u16 }; }

#[inline(always)]
pub unsafe fn __cpu_to_le64p(p: *const __u64) -> __le64 { __swab64p(p) as __le64 }
#[inline(always)]
pub unsafe fn __le64_to_cpup(p: *const __le64) -> __u64 { __swab64p(p as *const __u64) }
#[inline(always)]
pub unsafe fn __cpu_to_le32p(p: *const __u32) -> __le32 { __swab32p(p) as __le32 }
#[inline(always)]
pub unsafe fn __le32_to_cpup(p: *const __le32) -> __u32 { __swab32p(p as *const __u32) }
#[inline(always)]
pub unsafe fn __cpu_to_le16p(p: *const __u16) -> __le16 { __swab16p(p) as __le16 }
#[inline(always)]
pub unsafe fn __le16_to_cpup(p: *const __le16) -> __u16 { __swab16p(p as *const __u16) }
#[inline(always)]
pub unsafe fn __cpu_to_be64p(p: *const __u64) -> __be64 { *p as __be64 }
#[inline(always)]
pub unsafe fn __be64_to_cpup(p: *const __be64) -> __u64 { *p as __u64 }
#[inline(always)]
pub unsafe fn __cpu_to_be32p(p: *const __u32) -> __be32 { *p as __be32 }
#[inline(always)]
pub unsafe fn __be32_to_cpup(p: *const __be32) -> __u32 { *p as __u32 }
#[inline(always)]
pub unsafe fn __cpu_to_be16p(p: *const __u16) -> __be16 { *p as __be16 }
#[inline(always)]
pub unsafe fn __be16_to_cpup(p: *const __be16) -> __u16 { *p as __u16 }

macro_rules! __cpu_to_le64s { ($x:expr) => { __swab64s($x) }; }
macro_rules! __le64_to_cpus { ($x:expr) => { __swab64s($x) }; }
macro_rules! __cpu_to_le32s { ($x:expr) => { __swab32s($x) }; }
macro_rules! __le32_to_cpus { ($x:expr) => { __swab32s($x) }; }
macro_rules! __cpu_to_le16s { ($x:expr) => { __swab16s($x) }; }
macro_rules! __le16_to_cpus { ($x:expr) => { __swab16s($x) }; }
macro_rules! __cpu_to_be64s { ($x:expr) => {{ let _ = &$x; }}; }
macro_rules! __be64_to_cpus { ($x:expr) => {{ let _ = &$x; }}; }
macro_rules! __cpu_to_be32s { ($x:expr) => {{ let _ = &$x; }}; }
macro_rules! __be32_to_cpus { ($x:expr) => {{ let _ = &$x; }}; }
macro_rules! __cpu_to_be16s { ($x:expr) => {{ let _ = &$x; }}; }
macro_rules! __be16_to_cpus { ($x:expr) => {{ let _ = &$x; }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
