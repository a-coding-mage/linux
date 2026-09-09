/* SPDX-License-Identifier: GPL-2.0 */
/* CPU-measurement facilities */

// C dependencies: linux/errno.h, linux/kmsan-checks.h, asm/asm-extable.h,
// asm/facility.h, asm/asm.h, and asm/cpu_mf-insn.h.

pub const CPU_MF_INT_SF_IAE: i32 = 1 << 31; // invalid entry address
pub const CPU_MF_INT_SF_ISE: i32 = 1 << 30; // incorrect SDBT entry
pub const CPU_MF_INT_SF_PRA: i32 = 1 << 29; // program request alert
pub const CPU_MF_INT_SF_SACA: i32 = 1 << 23; // sampler auth. change alert
pub const CPU_MF_INT_SF_LSDA: i32 = 1 << 22; // loss of sample data alert
pub const CPU_MF_INT_CF_MTDA: i32 = 1 << 15; // loss of MT ctr. data alert
pub const CPU_MF_INT_CF_CACA: i32 = 1 << 7; // counter auth. change alert
pub const CPU_MF_INT_CF_LCDA: i32 = 1 << 6; // loss of counter data alert
pub const CPU_MF_INT_CF_MASK: i32 = CPU_MF_INT_CF_MTDA | CPU_MF_INT_CF_CACA | CPU_MF_INT_CF_LCDA;
pub const CPU_MF_INT_SF_MASK: i32 = CPU_MF_INT_SF_IAE | CPU_MF_INT_SF_ISE | CPU_MF_INT_SF_PRA | CPU_MF_INT_SF_SACA | CPU_MF_INT_SF_LSDA;
pub const CPU_MF_SF_RIBM_NOTAV: u32 = 0x1; // Sampling unavailable

extern "C" {
    fn test_facility(facility: i32) -> i32;
    fn kmsan_unpoison_memory(addr: *mut core::ffi::c_void, size: usize);
}

#[inline]
pub unsafe fn cpum_cf_avail() -> i32 { test_facility(40) & test_facility(67) }
#[inline]
pub unsafe fn cpum_sf_avail() -> i32 { test_facility(40) & test_facility(68) }

#[repr(C, packed)]
pub struct cpumf_ctr_info { pub cfvn: u16, pub auth_ctl: u16, pub enable_ctl: u16, pub act_ctl: u16, pub max_cpu: u16, pub csvn: u16, pub max_cg: u16, pub reserved1: u16, pub reserved2: [u32; 12] }

// C bit-fields are represented by their containing words; bit positions are preserved in comments.
#[repr(C, packed)]
pub struct hws_qsi_info_block {
    pub b0_13: u32, pub as_: u32, pub ad: u32, pub b16_21: u32, pub es: u32, pub ed: u32,
    pub b24_29: u32, pub cs: u32, pub cd: u32, pub bsdes: u32, pub dsdes: u32,
    pub min_sampl_rate: usize, pub max_sampl_rate: usize, pub tear: usize, pub dear: usize,
    pub rsvrd0: u32, pub ribm: u32, pub cpu_speed: u32, pub rsvrd1: u64, pub rsvrd2: u64,
}
#[repr(C, packed)]
pub struct hws_lsctl_request_block {
    pub s: u32, pub h: u32, pub b2_53: u64, pub es: u32, pub ed: u32, pub b56_61: u32,
    pub cs: u32, pub cd: u32, pub interval: usize, pub tear: usize, pub dear: usize,
    pub rsvrd1: usize, pub rsvrd2: usize, pub rsvrd3: usize, pub rsvrd4: usize,
}
#[repr(C, packed)]
pub struct hws_basic_entry {
    pub def: u32, pub r_: u32, pub u_: u32, pub z: u32, pub t: u32, pub w: u32, pub p: u32,
    pub as_: u32, pub i: u32, pub cl: u32, pub h: u32, pub ls: u32, pub reserved: u32,
    pub prim_asn: u32, pub ia: u64, pub gpp: u64, pub hpp: u64,
}
#[repr(C, packed)]
pub struct hws_diag_entry { pub def: u32, pub r_: u32, pub i: u32, pub data: [u8; 0] }
#[repr(C, packed)]
pub struct hws_combined_entry { pub basic: hws_basic_entry, pub diag: hws_diag_entry }

#[repr(C)]
pub union hws_trailer_header { pub fields: hws_trailer_header_fields, pub val: u128 }
#[repr(C, packed)]
pub struct hws_trailer_header_fields { pub f: u32, pub a: u32, pub t: u32, pub reserved: u32, pub bsdes: u32, pub dsdes: u32, pub overflow: u64 }
#[repr(C, packed)]
pub struct hws_trailer_entry { pub header: hws_trailer_header, pub timestamp: [u8; 16], pub reserved1: u64, pub reserved2: u64, pub progusage: [u64; 2] }

#[inline] pub unsafe fn lpp(pp: *mut core::ffi::c_void) { core::arch::asm!("lpp 0({0})", in(reg) pp, options(nostack, preserves_flags)); }

#[inline] pub unsafe fn qctri(info: *mut cpumf_ctr_info) -> i32 { let mut rc: i32 = -22; core::arch::asm!("qctri {info}", "lhi {rc},0", info = in(reg) info, rc = inout(reg) rc, options(nostack)); rc }
#[inline] pub unsafe fn lcctl(ctl: u64) -> i32 { let mut cc: i32; core::arch::asm!("lcctl {ctl}", "ipm {cc}", ctl = in(reg) ctl, cc = lateout(reg) cc, options(nostack)); cc }
#[inline] pub unsafe fn __ecctr(ctr: u64, content: *mut u64) -> i32 { let mut cc: i32; core::arch::asm!("ecctr {content},{ctr}", "ipm {cc}", content = lateout(reg) *content, ctr = in(reg) ctr, cc = lateout(reg) cc, options(nostack)); cc }
#[inline] pub unsafe fn ecctr(ctr: u64, val: *mut u64) -> i32 { let mut content = 0; let cc = __ecctr(ctr, &mut content); if cc == 0 { *val = content; } cc }

#[repr(C)]
pub enum stcctm_ctr_set { EXTENDED = 0, BASIC = 1, PROBLEM_STATE = 2, CRYPTO_ACTIVITY = 3, MT_DIAG = 5, MT_DIAG_CLEARING = 9 }
#[inline] pub unsafe fn stcctm(set: stcctm_ctr_set, range: u64, dest: *mut u64) -> i32 { let mut cc: i32; core::arch::asm!("stcctm {range},{set},{dest}", "ipm {cc}", range = in(reg) range, set = const set as i32, dest = in(reg) dest, cc = lateout(reg) cc, options(nostack)); kmsan_unpoison_memory(dest.cast(), range.wrapping_mul(core::mem::size_of::<u64>() as u64) as usize); cc }
#[inline] pub unsafe fn qsi(info: *mut hws_qsi_info_block) -> i32 { let mut cc: i32 = 1; core::arch::asm!("qsi {info}", "lhi {cc},0", info = in(reg) info, cc = inout(reg) cc, options(nostack)); if cc != 0 { -22 } else { 0 } }
#[inline] pub unsafe fn lsctl(req: *mut hws_lsctl_request_block) -> i32 { let mut cc: i32; let mut exception: i32 = 1; core::arch::asm!("lsctl {req}", "lhi {exception},0", "ipm {cc}", req = in(reg) req, exception = inout(reg) exception, cc = lateout(reg) cc, options(nostack)); if exception != 0 || cc != 0 { -22 } else { 0 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
