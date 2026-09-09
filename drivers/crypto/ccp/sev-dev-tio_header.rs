/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding kernel translation unit:
// linux/pci-tsm.h, linux/pci-ide.h, linux/tsm.h, and uapi/linux/psp-sev.h.

#[repr(C)]
pub union sla_addr_t__bindgen_ty_1 {
    pub sla: u64,
    // C bitfields: page_type:1, page_size:1, reserved1:10, pfn:40,
    // reserved2:12. The raw representation preserves the packed layout.
    pub fields: u64,
}

#[repr(C, packed)]
pub struct sla_addr_t {
    pub __bindgen_anon_1: sla_addr_t__bindgen_ty_1,
}

pub const SEV_TIO_MAX_COMMAND_LENGTH: usize = 128;

#[repr(C)]
pub struct tsm_spdm {
    pub req_len: ::core::ffi::c_ulong,
    pub req: *mut ::core::ffi::c_void,
    pub rsp_len: ::core::ffi::c_ulong,
    pub rsp: *mut ::core::ffi::c_void,
}

#[repr(C)]
pub struct tsm_dsm_tio {
    pub cert_slot: u8,
    pub dev_ctx: sla_addr_t,
    pub req: sla_addr_t,
    pub resp: sla_addr_t,
    pub scratch: sla_addr_t,
    pub output: sla_addr_t,
    pub output_len: usize,
    pub scratch_len: usize,
    pub spdm: tsm_spdm,
    /// vmap'ed @req for DOE
    pub reqbuf: *mut sla_buffer_hdr,
    /// vmap'ed @resp for DOE
    pub respbuf: *mut sla_buffer_hdr,
    pub cmd: i32,
    pub psp_ret: i32,
    pub cmd_data: [u8; SEV_TIO_MAX_COMMAND_LENGTH],
    /// Data page for DEV_STATUS/TDI_STATUS/TDI_INFO/ASID_FENCE
    pub data_pg: *mut ::core::ffi::c_void,
    pub ide: [*mut pci_ide; TIO_IDE_MAX_TC],
}

pub const TIO_IDE_MAX_TC: usize = 8;

#[repr(C)]
pub struct tio_dsm {
    pub tsm: pci_tsm_pf0,
    pub data: tsm_dsm_tio,
    pub sev: *mut sev_device,
}

pub const SPDM_DOBJ_ID_NONE: u32 = 0;
pub const SPDM_DOBJ_ID_REQ: u32 = 1;
pub const SPDM_DOBJ_ID_RESP: u32 = 2;

#[repr(C, packed)]
pub struct spdm_dobj_hdr_version {
    pub minor: u8,
    pub major: u8,
}

#[repr(C, packed)]
pub struct spdm_dobj_hdr {
    /// Data object type identifier
    pub id: u32,
    /// Length of the data object, INCLUDING THIS HEADER
    pub length: u32,
    /// Version of the data object structure
    pub version: spdm_dobj_hdr_version,
}

/**
 * struct sev_tio_status - TIO_STATUS command's info_paddr buffer
 *
 * @length: Length of this structure in bytes
 * @tio_en: Indicates that SNP_INIT_EX initialized the RMP for SEV-TIO
 * @tio_init_done: Indicates TIO_INIT has been invoked
 * @spdm_req_size_min: Minimum SPDM request buffer size in bytes
 * @spdm_req_size_max: Maximum SPDM request buffer size in bytes
 * @spdm_scratch_size_min: Minimum SPDM scratch buffer size in bytes
 * @spdm_scratch_size_max: Maximum SPDM scratch buffer size in bytes
 * @spdm_out_size_min: Minimum SPDM output buffer size in bytes
 * @spdm_out_size_max: Maximum for the SPDM output buffer size in bytes
 * @spdm_rsp_size_min: Minimum SPDM response buffer size in bytes
 * @spdm_rsp_size_max: Maximum SPDM response buffer size in bytes
 * @devctx_size: Size of a device context buffer in bytes
 * @tdictx_size: Size of a TDI context buffer in bytes
 * @tio_crypto_alg: TIO crypto algorithms supported
 */
#[repr(C, packed)]
pub struct sev_tio_status {
    pub length: u32,
    // C bitfields: tio_en:1, tio_init_done:1, reserved:30.
    pub flags: u32,
    pub spdm_req_size_min: u32,
    pub spdm_req_size_max: u32,
    pub spdm_scratch_size_min: u32,
    pub spdm_scratch_size_max: u32,
    pub spdm_out_size_min: u32,
    pub spdm_out_size_max: u32,
    pub spdm_rsp_size_min: u32,
    pub spdm_rsp_size_max: u32,
    pub devctx_size: u32,
    pub tdictx_size: u32,
    pub tio_crypto_alg: u32,
    pub reserved2: [u8; 12],
}

unsafe extern "C" {
    pub fn sev_tio_init_locked(tio_status_page: *mut ::core::ffi::c_void) -> i32;
    pub fn sev_tio_continue(dev_data: *mut tsm_dsm_tio) -> i32;
    pub fn sev_tio_dev_create(
        dev_data: *mut tsm_dsm_tio,
        device_id: u16,
        root_port_id: u16,
        segment_id: u8,
    ) -> i32;
    pub fn sev_tio_dev_connect(
        dev_data: *mut tsm_dsm_tio,
        tc_mask: u8,
        ids: *mut u8,
        cert_slot: u8,
    ) -> i32;
    pub fn sev_tio_dev_disconnect(dev_data: *mut tsm_dsm_tio, force: bool) -> i32;
    pub fn sev_tio_dev_reclaim(dev_data: *mut tsm_dsm_tio) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
