/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from core_titan.h.  USE_48_BIT_KSEG is a build-time condition. */

#[cfg(feature = "use_48_bit_kseg")]
pub const TI_BIAS: u64 = 0x80000000000;
#[cfg(not(feature = "use_48_bit_kseg"))]
pub const TI_BIAS: u64 = 0x10000000000;

#[repr(C, align(64))]
pub struct titan_64 { pub csr: core::cell::UnsafeCell<u64> }

#[repr(C)]
pub struct titan_cchip {
    pub csc:titan_64,pub mtr:titan_64,pub misc:titan_64,pub mpd:titan_64,
    pub aar0:titan_64,pub aar1:titan_64,pub aar2:titan_64,pub aar3:titan_64,
    pub dim0:titan_64,pub dim1:titan_64,pub dir0:titan_64,pub dir1:titan_64,
    pub drir:titan_64,pub prben:titan_64,pub iic0:titan_64,pub iic1:titan_64,
    pub mpr0:titan_64,pub mpr1:titan_64,pub mpr2:titan_64,pub mpr3:titan_64,
    pub rsvd:[titan_64;2],pub ttr:titan_64,pub tdr:titan_64,
    pub dim2:titan_64,pub dim3:titan_64,pub dir2:titan_64,pub dir3:titan_64,
    pub iic2:titan_64,pub iic3:titan_64,pub pwr:titan_64,pub reserved:[titan_64;17],
    pub cmonctla:titan_64,pub cmonctlb:titan_64,pub cmoncnt01:titan_64,
    pub cmoncnt23:titan_64,pub cpen:titan_64,
}
#[repr(C)] pub struct titan_dchip { pub dsc:titan_64,pub str_:titan_64,pub drev:titan_64,pub dsc2:titan_64 }
#[repr(C)] pub struct titan_pachip_port { pub wsba:[titan_64;4],pub wsm:[titan_64;4],pub tba:[titan_64;4],pub pctl:titan_64,pub plat:titan_64,pub reserved0:[titan_64;2],pub port_specific:TPortSpecific,pub sprst:titan_64,pub reserved1:[titan_64;31] }
#[repr(C)] pub union TPortSpecific { pub g:TPortG,pub a:TPortA }
#[repr(C)] pub struct TPortG { pub regs:[titan_64;15] }
#[repr(C)] pub struct TPortA { pub regs:[titan_64;14] }
#[repr(C)] pub struct titan_pachip { pub g_port:titan_pachip_port,pub a_port:titan_pachip_port }

/* IDENT_ADDR is supplied by the platform headers. */
pub const fn titan_cchip_addr(ident_addr:u64)->u64 { ident_addr+TI_BIAS+0x1A0000000 }
pub const fn titan_dchip_addr(ident_addr:u64)->u64 { ident_addr+TI_BIAS+0x1B0000800 }
pub const fn titan_pachip0_addr(ident_addr:u64)->u64 { ident_addr+TI_BIAS+0x180000000 }
pub const fn titan_pachip1_addr(ident_addr:u64)->u64 { ident_addr+TI_BIAS+0x380000000 }
extern "C" { pub static mut TITAN_agp:u32; pub static mut TITAN_bootcpu:i32; }

pub const wsba_m_ena:u64=0x1; pub const wsba_m_sg:u64=0x2; pub const wsba_m_addr:u64=0xFFF00000; pub const wmask_k_sz1gb:u64=0x3FF00000;
#[repr(C)] pub union TPAchipWSBA { pub wsba_r_bits:[u64;1], pub wsba_q_whole:[i32;2] }

pub const pctl_m_fbtb:u64=0x1; pub const pctl_m_thdis:u64=0x2; pub const pctl_m_chaindis:u64=0x4; pub const pctl_m_tgtlat:u64=0x18; pub const pctl_m_hole:u64=0x20; pub const pctl_m_mwin:u64=0x40; pub const pctl_m_arbena:u64=0x80; pub const pctl_m_prigrp:u64=0xFF00; pub const pctl_m_ppri:u64=0x10000; pub const pctl_m_pcispd66:u64=0x20000; pub const pctl_m_cngstlt:u64=0x003C0000; pub const pctl_m_ptpdesten:u64=0x3FC00000; pub const pctl_m_dpcen:u64=0x40000000; pub const pctl_m_apcen:u64=0x0000000080000000; pub const pctl_m_dcrtv:u64=0x0000000300000000; pub const pctl_m_en_stepping:u64=0x0000000400000000; pub const apctl_m_rsvd1:u64=0x000FFFF800000000; pub const apctl_m_agp_rate:u64=0x0030000000000000; pub const apctl_m_agp_sba_en:u64=0x0040000000000000; pub const apctl_m_agp_en:u64=0x0080000000000000; pub const apctl_m_rsvd2:u64=0x0100000000000000; pub const apctl_m_agp_present:u64=0x0200000000000000; pub const apctl_agp_hp_rd:u64=0x1C00000000000000; pub const apctl_agp_lp_rd:u64=0xE000000000000000; pub const gpctl_m_rsvd:u64=0xFFFFFFF800000000;
#[repr(C)] pub union TPAchipPCTL { pub pctl_r_bits:[u64;1], pub pctl_l_whole:[u32;2], pub pctl_q_whole:u64 }
#[repr(C)] pub union TPAchipSERR { pub serr_r_bits:[u64;1], pub serr_l_whole:[u32;2], pub serr_q_whole:u64 }
#[repr(C)] pub union TPAchipPERR { pub perr_r_bits:[u64;1], pub perr_l_whole:[u32;2], pub perr_q_whole:u64 }
#[repr(C)] pub union TPAchipAGPERR { pub agperr_r_bits:[u64;1], pub agperr_l_whole:[u32;2], pub agperr_q_whole:u64 }

pub const TITAN_HOSE_SHIFT:u32=33; pub const fn titan_hose(h:u64)->u64 { h<<TITAN_HOSE_SHIFT }
pub const fn titan_base(ident_addr:u64)->u64 { ident_addr+TI_BIAS }
pub const fn titan_mem(ident_addr:u64,h:u64)->u64 { titan_base(ident_addr)+titan_hose(h) }
pub const fn titan_iack_sc(ident_addr:u64,h:u64)->u64 { titan_base(ident_addr)+titan_hose(h)+0x1F8000000 }
pub const fn titan_io(ident_addr:u64,h:u64)->u64 { titan_base(ident_addr)+titan_hose(h)+0x1FC000000 }
pub const fn titan_conf(ident_addr:u64,h:u64)->u64 { titan_base(ident_addr)+titan_hose(h)+0x1FE000000 }
pub const TITAN_DAC_OFFSET:u64=1u64<<40;

pub const SCB_Q_SYSERR:u64=0x620; pub const SCB_Q_PROCERR:u64=0x630; pub const SCB_Q_SYSMCHK:u64=0x660; pub const SCB_Q_PROCMCHK:u64=0x670; pub const SCB_Q_SYSEVENT:u64=0x680;
#[repr(C)] pub struct el_TITAN_sysdata_mcheck { pub summary:u64,pub c_dirx:u64,pub c_misc:u64,pub p0_serror:u64,pub p0_gperror:u64,pub p0_aperror:u64,pub p0_agperror:u64,pub p1_serror:u64,pub p1_gperror:u64,pub p1_aperror:u64,pub p1_agperror:u64 }
#[repr(C)] pub struct el_PRIVATEER_envdata_mcheck { pub summary:u64,pub c_dirx:u64,pub smir:u64,pub cpuir:u64,pub psir:u64,pub fault:u64,pub sys_doors:u64,pub temp_warn:u64,pub fan_ctrl:u64,pub code:u64,pub reserved:u64 }

extern "C" { pub fn titan_ioportmap(addr:u64)->*mut core::ffi::c_void; pub fn titan_ioremap(addr:u64,size:u64)->*mut core::ffi::c_void; pub fn titan_iounmap(addr:*const core::ffi::c_void); pub fn titan_is_mmio(addr:*const core::ffi::c_void)->i32; }
#[inline] pub fn titan_is_ioaddr(addr:u64, titan_base:u64)->bool { addr>=titan_base }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
