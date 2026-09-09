/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  Author: Nicolas Pitre
 *  Created: Jun 15, 2001
 *  Copyright: MontaVista Software Inc.
 */

/* The C header conditionally includes <asm/cputype.h> when CONFIG_ARM is set. */

/*
 *   CPU     Stepping     CPU_ID         JTAG_ID
 *
 *  PXA210 B0 0x69052922 0x2926C013; PXA210 B1 0x69052923 0x3926C013
 *  PXA210 B2 0x69052924 0x4926C013; PXA210 C0 0x69052D25 0x5926C013
 *  PXA250 A0 0x69052100 0x09264013; PXA250 A1 0x69052101 0x19264013
 *  PXA250 B0 0x69052902 0x29264013; PXA250 B1 0x69052903 0x39264013
 *  PXA250 B2 0x69052904 0x49264013; PXA250 C0 0x69052D05 0x59264013
 *  PXA255 A0 0x69052D06 0x69264013
 *  PXA26x A0 0x69052903 0x39264013; PXA26x B0 0x69052D05 0x59264013
 *  PXA27x A0 0x69054110 0x09265013; PXA27x A1 0x69054111 0x19265013
 *  PXA27x B0 0x69054112 0x29265013; PXA27x B1 0x69054113 0x39265013
 *  PXA27x C0 0x69054114 0x49265013; PXA27x C5 0x69054117 0x79265013
 *  PXA30x A0 0x69056880 0x0E648013; PXA30x A1 0x69056881 0x1E648013
 *  PXA31x A0 0x69056890 0x0E649013; PXA31x A1 0x69056891 0x1E649013
 *  PXA31x A2 0x69056892 0x2E649013
 *  PXA32x B1 0x69056825 0x5E642013; PXA32x B2 0x69056826 0x6E642013
 */

extern "C" {
    pub fn read_cpuid_id() -> u32;
}

#[cfg(feature = "CONFIG_PXA25x")]
pub fn __cpu_is_pxa210(id: u32) -> bool { (id & 0xf3f0) == 0x2120 }
#[cfg(not(feature = "CONFIG_PXA25x"))]
pub fn __cpu_is_pxa210(_id: u32) -> bool { false }

#[cfg(feature = "CONFIG_PXA25x")]
pub fn __cpu_is_pxa250(id: u32) -> bool { (id & 0xf3ff) <= 0x2105 }
#[cfg(not(feature = "CONFIG_PXA25x"))]
pub fn __cpu_is_pxa250(_id: u32) -> bool { false }

#[cfg(feature = "CONFIG_PXA25x")]
pub fn __cpu_is_pxa255(id: u32) -> bool { (id & 0xffff) == 0x2d06 }
#[cfg(not(feature = "CONFIG_PXA25x"))]
pub fn __cpu_is_pxa255(_id: u32) -> bool { false }

#[cfg(feature = "CONFIG_PXA25x")]
pub fn __cpu_is_pxa25x(id: u32) -> bool { (id & 0xf300) == 0x2100 }
#[cfg(not(feature = "CONFIG_PXA25x"))]
pub fn __cpu_is_pxa25x(_id: u32) -> bool { false }

#[cfg(feature = "CONFIG_PXA27x")]
pub fn __cpu_is_pxa27x(id: u32) -> bool { ((id >> 4) & 0xfff) == 0x411 }
#[cfg(not(feature = "CONFIG_PXA27x"))]
pub fn __cpu_is_pxa27x(_id: u32) -> bool { false }

#[cfg(feature = "CONFIG_CPU_PXA300")]
pub fn __cpu_is_pxa300(id: u32) -> bool { ((id >> 4) & 0xfff) == 0x688 }
#[cfg(not(feature = "CONFIG_CPU_PXA300"))]
pub fn __cpu_is_pxa300(_id: u32) -> bool { false }

#[cfg(feature = "CONFIG_CPU_PXA310")]
pub fn __cpu_is_pxa310(id: u32) -> bool { ((id >> 4) & 0xfff) == 0x689 }
#[cfg(not(feature = "CONFIG_CPU_PXA310"))]
pub fn __cpu_is_pxa310(_id: u32) -> bool { false }

#[cfg(feature = "CONFIG_CPU_PXA320")]
pub fn __cpu_is_pxa320(id: u32) -> bool { let id = (id >> 4) & 0xfff; id == 0x603 || id == 0x682 }
#[cfg(not(feature = "CONFIG_CPU_PXA320"))]
pub fn __cpu_is_pxa320(_id: u32) -> bool { false }

pub unsafe fn cpu_is_pxa210() -> bool { __cpu_is_pxa210(read_cpuid_id()) }
pub unsafe fn cpu_is_pxa250() -> bool { __cpu_is_pxa250(read_cpuid_id()) }
pub unsafe fn cpu_is_pxa255() -> bool { __cpu_is_pxa255(read_cpuid_id()) }
pub unsafe fn cpu_is_pxa25x() -> bool { __cpu_is_pxa25x(read_cpuid_id()) }
pub unsafe fn cpu_is_pxa27x() -> bool { __cpu_is_pxa27x(read_cpuid_id()) }
pub unsafe fn cpu_is_pxa300() -> bool { __cpu_is_pxa300(read_cpuid_id()) }
pub unsafe fn cpu_is_pxa310() -> bool { __cpu_is_pxa310(read_cpuid_id()) }
pub unsafe fn cpu_is_pxa320() -> bool { __cpu_is_pxa320(read_cpuid_id()) }

/* CPUID Core Generation Bit; <= 0x2 for pxa21x/pxa25x/pxa26x/pxa27x. */
#[cfg(any(feature = "CONFIG_PXA25x", feature = "CONFIG_PXA27x"))]
pub fn __cpu_is_pxa2xx(id: u32) -> bool { ((id >> 13) & 0x7) <= 0x2 }
#[cfg(not(any(feature = "CONFIG_PXA25x", feature = "CONFIG_PXA27x")))]
pub fn __cpu_is_pxa2xx(_id: u32) -> bool { false }

#[cfg(feature = "CONFIG_PXA3xx")]
pub fn __cpu_is_pxa3xx(id: u32) -> bool { __cpu_is_pxa300(id) || __cpu_is_pxa310(id) || __cpu_is_pxa320(id) }
#[cfg(not(feature = "CONFIG_PXA3xx"))]
pub fn __cpu_is_pxa3xx(_id: u32) -> bool { false }

pub unsafe fn cpu_is_pxa2xx() -> bool { __cpu_is_pxa2xx(read_cpuid_id()) }
pub unsafe fn cpu_is_pxa3xx() -> bool { __cpu_is_pxa3xx(read_cpuid_id()) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
