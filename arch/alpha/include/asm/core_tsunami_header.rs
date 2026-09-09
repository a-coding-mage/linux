/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from the C header; build-time include/guard conditions are preserved here as comments. */

#[cfg(not(feature = "use_48_bit_kseg"))]
pub const TS_BIAS: usize = 0x10000000000;
#[cfg(feature = "use_48_bit_kseg")]
pub const TS_BIAS: usize = 0x80000000000;

#[repr(C, align(64))]
pub struct tsunami_64 {
    pub csr: core::cell::UnsafeCell<usize>,
}

#[repr(C)]
pub struct tsunami_cchip {
    pub csc: tsunami_64, pub mtr: tsunami_64, pub misc: tsunami_64, pub mpd: tsunami_64,
    pub aar0: tsunami_64, pub aar1: tsunami_64, pub aar2: tsunami_64, pub aar3: tsunami_64,
    pub dim0: tsunami_64, pub dim1: tsunami_64, pub dir0: tsunami_64, pub dir1: tsunami_64,
    pub drir: tsunami_64, pub prben: tsunami_64, pub iic: tsunami_64, pub wdr: tsunami_64,
    pub mpr0: tsunami_64, pub mpr1: tsunami_64, pub mpr2: tsunami_64, pub mpr3: tsunami_64,
    pub mctl: tsunami_64, pub __pad1: tsunami_64, pub ttr: tsunami_64, pub tdr: tsunami_64,
    pub dim2: tsunami_64, pub dim3: tsunami_64, pub dir2: tsunami_64, pub dir3: tsunami_64,
    pub iic2: tsunami_64, pub iic3: tsunami_64,
}

#[repr(C)]
pub struct tsunami_dchip { pub dsc: tsunami_64, pub str_: tsunami_64, pub drev: tsunami_64 }

#[repr(C)]
pub struct tsunami_pchip {
    pub wsba: [tsunami_64; 4], pub wsm: [tsunami_64; 4], pub tba: [tsunami_64; 4],
    pub pctl: tsunami_64, pub plat: tsunami_64, pub reserved: tsunami_64,
    pub perror: tsunami_64, pub perrmask: tsunami_64, pub perrset: tsunami_64,
    pub tlbiv: tsunami_64, pub tlbia: tsunami_64, pub pmonctl: tsunami_64, pub pmoncnt: tsunami_64,
}

extern "C" {
    pub static mut TSUNAMI_bootcpu: core::ffi::c_int;
    pub fn tsunami_ioportmap(addr: usize) -> *mut core::ffi::c_void;
    pub fn tsunami_ioremap(addr: usize, size: usize) -> *mut core::ffi::c_void;
}

/* IDENT_ADDR is supplied by the surrounding architecture headers. */
pub const fn TSUNAMI_cchip() -> *mut tsunami_cchip { (IDENT_ADDR + TS_BIAS + 0x1A0000000) as *mut tsunami_cchip }
pub const fn TSUNAMI_dchip() -> *mut tsunami_dchip { (IDENT_ADDR + TS_BIAS + 0x1B0000800) as *mut tsunami_dchip }
pub const fn TSUNAMI_pchip0() -> *mut tsunami_pchip { (IDENT_ADDR + TS_BIAS + 0x180000000) as *mut tsunami_pchip }
pub const fn TSUNAMI_pchip1() -> *mut tsunami_pchip { (IDENT_ADDR + TS_BIAS + 0x380000000) as *mut tsunami_pchip }

pub const perror_m_lost: usize = 0x1; pub const perror_m_serr: usize = 0x2; pub const perror_m_perr: usize = 0x4;
pub const perror_m_dcrto: usize = 0x8; pub const perror_m_sge: usize = 0x10; pub const perror_m_ape: usize = 0x20;
pub const perror_m_ta: usize = 0x40; pub const perror_m_rdpe: usize = 0x80; pub const perror_m_nds: usize = 0x100;
pub const perror_m_rto: usize = 0x200; pub const perror_m_uecc: usize = 0x400; pub const perror_m_cre: usize = 0x800;
pub const perror_m_addrl: usize = 0xFFFFFFFF0000; pub const perror_m_addrh: usize = 0x7000000000000;
pub const perror_m_cmd: usize = 0xF0000000000000; pub const perror_m_syn: usize = 0xFF00000000000000;

#[repr(C)] pub union TPchipPERROR { pub perror_r_bits: u64, pub perror_q_whole: [i32; 2] }

pub const wsba_m_ena: usize = 0x1; pub const wsba_m_sg: usize = 0x2; pub const wsba_m_ptp: usize = 0x4;
pub const wsba_m_addr: usize = 0xFFF00000; pub const wmask_k_sz1gb: usize = 0x3FF00000;
#[repr(C)] pub union TPchipWSBA { pub wsba_r_bits: u64, pub wsba_q_whole: [i32; 2] }

pub const pctl_m_fdsc: usize = 0x1; pub const pctl_m_fbtb: usize = 0x2; pub const pctl_m_thdis: usize = 0x4;
pub const pctl_m_chaindis: usize = 0x8; pub const pctl_m_tgtlat: usize = 0x10; pub const pctl_m_hole: usize = 0x20;
pub const pctl_m_mwin: usize = 0x40; pub const pctl_m_arbena: usize = 0x80; pub const pctl_m_prigrp: usize = 0x7F00;
pub const pctl_m_ppri: usize = 0x8000; pub const pctl_m_rsvd1: usize = 0x30000; pub const pctl_m_eccen: usize = 0x40000;
pub const pctl_m_padm: usize = 0x80000; pub const pctl_m_cdqmax: usize = 0xF00000; pub const pctl_m_rev: usize = 0xFF000000;
pub const pctl_m_crqmax: usize = 0xF00000000; pub const pctl_m_ptpmax: usize = 0xF000000000;
pub const pctl_m_pclkx: usize = 0x30000000000; pub const pctl_m_fdsdis: usize = 0x40000000000;
pub const pctl_m_fdwdis: usize = 0x80000000000; pub const pctl_m_ptevrfy: usize = 0x100000000000;
pub const pctl_m_rpp: usize = 0x200000000000; pub const pctl_m_pid: usize = 0xC00000000000;
pub const pctl_m_rsvd2: usize = 0xFFFF000000000000;
#[repr(C)] pub union TPchipPCTL { pub pctl_r_bits: u64, pub pctl_q_whole: [i32; 2] }

pub const perrmask_m_lost: usize = 0x1; pub const perrmask_m_serr: usize = 0x2; pub const perrmask_m_perr: usize = 0x4;
pub const perrmask_m_dcrto: usize = 0x8; pub const perrmask_m_sge: usize = 0x10; pub const perrmask_m_ape: usize = 0x20;
pub const perrmask_m_ta: usize = 0x40; pub const perrmask_m_rdpe: usize = 0x80; pub const perrmask_m_nds: usize = 0x100;
pub const perrmask_m_rto: usize = 0x200; pub const perrmask_m_uecc: usize = 0x400; pub const perrmask_m_cre: usize = 0x800;
pub const perrmask_m_rsvd: usize = 0xFFFFFFFFFFFFF000;
#[repr(C)] pub union TPchipPERRMASK { pub perrmask_r_bits: u64, pub perrmask_q_whole: [i32; 2] }

pub const fn TSUNAMI_HOSE(h: usize) -> usize { h << 33 }
pub const fn TSUNAMI_BASE() -> usize { IDENT_ADDR + TS_BIAS }
pub const fn TSUNAMI_MEM(h: usize) -> usize { TSUNAMI_BASE() + TSUNAMI_HOSE(h) }
pub const fn _TSUNAMI_IACK_SC(h: usize) -> usize { TSUNAMI_BASE() + TSUNAMI_HOSE(h) + 0x1F8000000 }
pub const fn TSUNAMI_IO(h: usize) -> usize { TSUNAMI_BASE() + TSUNAMI_HOSE(h) + 0x1FC000000 }
pub const fn TSUNAMI_CONF(h: usize) -> usize { TSUNAMI_BASE() + TSUNAMI_HOSE(h) + 0x1FE000000 }
pub const fn TSUNAMI_IACK_SC() -> usize { _TSUNAMI_IACK_SC(0) }
pub const fn TSUNAMI_IO_BIAS() -> usize { TSUNAMI_IO(0) }
pub const fn TSUNAMI_MEM_BIAS() -> usize { TSUNAMI_MEM(0) }
pub const fn TSUNAMI_IO_SPACE() -> usize { TSUNAMI_CONF(0) - TSUNAMI_IO(0) }
pub const TSUNAMI_DAC_OFFSET: usize = 1usize << 40;

#[repr(C)] pub struct el_TSUNAMI_sysdata_mcheck {}

#[inline]
pub unsafe fn tsunami_is_ioaddr(addr: usize) -> core::ffi::c_int { (addr >= TSUNAMI_BASE()) as core::ffi::c_int }
#[inline]
pub unsafe fn tsunami_is_mmio(xaddr: *const core::ffi::c_void) -> core::ffi::c_int {
    (((xaddr as usize) & 0x100000000) == 0) as core::ffi::c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
