/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* atm.h - general ATM declarations */

/*
 * WARNING: User-space programs should not include <linux/atm.h> directly.
 *          Instead, include <atm.h>
 */

// C header dependencies: linux/compiler.h, linux/atmapi.h, linux/atmsap.h,
// linux/atmioc.h, and linux/types.h.

use core::ffi::c_void;

pub const ATM_CELL_SIZE: i32 = 53;
pub const ATM_CELL_PAYLOAD: i32 = 48;
pub const ATM_AAL0_SDU: i32 = 52;
pub const ATM_MAX_AAL34_PDU: i32 = 65535;
pub const ATM_AAL5_TRAILER: i32 = 8;
pub const ATM_MAX_AAL5_PDU: i32 = 65535;
pub const ATM_MAX_CDV: i32 = 9999;
pub const ATM_NOT_RSV_VCI: i32 = 32;
pub const ATM_MAX_VPI: i32 = 255;
pub const ATM_MAX_VPI_NNI: i32 = 4096;
pub const ATM_MAX_VCI: i32 = 65535;

pub const ATM_NO_AAL: i32 = 0;
pub const ATM_AAL0: i32 = 13;
pub const ATM_AAL1: i32 = 1;
pub const ATM_AAL2: i32 = 2;
pub const ATM_AAL34: i32 = 3;
pub const ATM_AAL5: i32 = 5;

pub const fn __so_encode(l: i32, n: i32, t_size: usize) -> i32 {
    (((l & 0x1ff) << 22) | (n << 16) | t_size as i32)
}
pub const fn __so_level_match(c: i32, m: i32) -> bool {
    (c >> 22) == (m & 0x1ff)
}
pub const fn __so_number(c: i32) -> i32 { (c >> 16) & 0x3f }
pub const fn __so_size(c: i32) -> i32 { c & 0x3fff }

// SOL_ATM is supplied by the corresponding ATM API dependency.
pub const SO_SETCLP: i32 = __so_encode(SOL_ATM, 0, core::mem::size_of::<i32>());
pub const SO_CIRANGE: i32 = __so_encode(SOL_ATM, 1, core::mem::size_of::<AtmCirange>());
pub const SO_ATMQOS: i32 = __so_encode(SOL_ATM, 2, core::mem::size_of::<AtmQos>());
pub const SO_ATMSAP: i32 = __so_encode(SOL_ATM, 3, core::mem::size_of::<AtmSap>());
pub const SO_ATMPVC: i32 = __so_encode(SOL_ATM, 4, core::mem::size_of::<SockaddrAtmpvc>());
pub const SO_MULTIPOINT: i32 = __so_encode(SOL_ATM, 5, core::mem::size_of::<i32>());

pub const ATM_HDR_GFC_MASK: u32 = 0xf0000000;
pub const ATM_HDR_GFC_SHIFT: u32 = 28;
pub const ATM_HDR_VPI_MASK: u32 = 0x0ff00000;
pub const ATM_HDR_VPI_SHIFT: u32 = 20;
pub const ATM_HDR_VCI_MASK: u32 = 0x000ffff0;
pub const ATM_HDR_VCI_SHIFT: u32 = 4;
pub const ATM_HDR_PTI_MASK: u32 = 0x0000000e;
pub const ATM_HDR_PTI_SHIFT: u32 = 1;
pub const ATM_HDR_CLP: u32 = 0x00000001;

pub const ATM_PTI_US0: i32 = 0;
pub const ATM_PTI_US1: i32 = 1;
pub const ATM_PTI_UCES0: i32 = 2;
pub const ATM_PTI_UCES1: i32 = 3;
pub const ATM_PTI_SEGF5: i32 = 4;
pub const ATM_PTI_E2EF5: i32 = 5;
pub const ATM_PTI_RSV_RM: i32 = 6;
pub const ATM_PTI_RSV: i32 = 7;

pub const ATM_NONE: i32 = 0;
pub const ATM_UBR: i32 = 1;
pub const ATM_CBR: i32 = 2;
pub const ATM_VBR: i32 = 3;
pub const ATM_ABR: i32 = 4;
pub const ATM_ANYCLASS: i32 = 5;
pub const ATM_MAX_PCR: i32 = -1;

#[repr(C)]
pub struct AtmTrafprm {
    pub traffic_class: u8,
    pub max_pcr: i32,
    pub pcr: i32,
    pub min_pcr: i32,
    pub max_cdv: i32,
    pub max_sdu: i32,
    pub icr: u32,
    pub tbe: u32,
    pub frtt: u32,
    pub rif: u32,
    pub rdf: u32,
    pub nrm_pres: u32,
    pub trm_pres: u32,
    pub adtf_pres: u32,
    pub cdf_pres: u32,
    pub nrm: u32,
    pub trm: u32,
    pub adtf: u32,
    pub cdf: u32,
    pub spare: u32,
}

#[repr(C)]
pub struct AtmQos {
    pub txtp: AtmTrafprm,
    pub rxtp: AtmTrafprm, // __ATM_API_ALIGN
    pub aal: u8, // __ATM_API_ALIGN
}

pub const ATM_ITF_ANY: i32 = -1;
pub const ATM_VPI_ANY: i32 = -1;
pub const ATM_VCI_ANY: i32 = -1;
pub const ATM_VPI_UNSPEC: i32 = -2;
pub const ATM_VCI_UNSPEC: i32 = -2;

#[repr(C)]
pub struct AtmPvcAddr {
    pub itf: i16,
    pub vpi: i16,
    pub vci: i32,
}

#[repr(C)]
pub struct SockaddrAtmpvc {
    pub sap_family: u16,
    pub sap_addr: AtmPvcAddr, // __ATM_API_ALIGN
}

pub const ATM_ESA_LEN: usize = 20;
pub const ATM_E164_LEN: usize = 12;
pub const ATM_AFI_DCC: u8 = 0x39;
pub const ATM_AFI_ICD: u8 = 0x47;
pub const ATM_AFI_E164: u8 = 0x45;
pub const ATM_AFI_LOCAL: u8 = 0x49;
pub const ATM_AFI_DCC_GROUP: u8 = 0xbd;
pub const ATM_AFI_ICD_GROUP: u8 = 0xc5;
pub const ATM_AFI_E164_GROUP: u8 = 0xc3;
pub const ATM_AFI_LOCAL_GROUP: u8 = 0xc7;
pub const ATM_LIJ_NONE: i8 = 0;
pub const ATM_LIJ: i8 = 1;
pub const ATM_LIJ_RPJ: i8 = 2;
pub const ATM_LIJ_NJ: i8 = 3;

#[repr(C)]
pub struct AtmSvcAddr {
    pub prv: [u8; ATM_ESA_LEN],
    pub pub_: [i8; ATM_E164_LEN + 1],
    pub lij_type: i8,
    pub lij_id: u32,
}

#[repr(C)]
pub struct SockaddrAtmsvc {
    pub sas_family: u16,
    pub sas_addr: AtmSvcAddr, // __ATM_API_ALIGN
}

#[inline]
pub unsafe fn atmsvc_addr_in_use(addr: SockaddrAtmsvc) -> i32 {
    if addr.sas_addr.prv[0] != 0 || addr.sas_addr.pub_[0] != 0 { 1 } else { 0 }
}

#[inline]
pub unsafe fn atmpvc_addr_in_use(addr: SockaddrAtmpvc) -> i32 {
    if addr.sap_addr.itf != 0 || addr.sap_addr.vpi != 0 || addr.sap_addr.vci != 0 { 1 } else { 0 }
}

#[repr(C)]
pub struct AtmifSioc {
    pub number: i32,
    pub length: i32,
    pub arg: *mut c_void,
}

pub type AtmBackendT = u16;

// External types and constants supplied by included ATM headers: AtmCirange,
// AtmSap, and SOL_ATM.
extern "C" {
    pub static SOL_ATM: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
