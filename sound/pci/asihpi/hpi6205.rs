// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2014  AudioScience Inc. <support@audioscience.com>


 Hardware Programming Interface (HPI) for AudioScience
 ASI50xx, AS51xx, ASI6xxx, ASI87xx ASI89xx series adapters.
 These PCI and PCIe bus adapters are based on a
 TMS320C6205 PCI bus mastering DSP,
 and (except ASI50xx) TI TMS320C6xxx floating point DSP

 Exported function:
 void HPI_6205(struct hpi_message *phm, struct hpi_response *phr)

(C) Copyright AudioScience Inc. 1998-2010
*******************************************************************************/

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_assignments, improper_ctypes)]

use core::ffi::{c_int, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{copy_nonoverlapping, null_mut, write_bytes};

type u8 = u8;
type u16 = u16;
type u32 = u32;

/* Includes in the C source:
 * hpi_internal.h, hpimsginit.h, hpidebug.h, hpi6205.h, hpidspcd.h, hpicmn.h.
 * Their constants, structs, unions, and helpers are external dependencies here.
 */

extern "C" {
    static HPI_MAX_STREAMS: usize;
    static HPI_ERROR_INVALID_FUNC: u16;
    static HPI_ERROR_INVALID_OBJ_INDEX: u16;
    static HPI_ERROR_INVALID_TYPE: u16;
    static HPI_ERROR_PROCESSING_MESSAGE: u16;
    static HPI_ERROR_DSP_HARDWARE: u16;
    static HPI_ERROR_BAD_ADAPTER_NUMBER: u16;
    static HPI_ERROR_MEMORY_ALLOC: u16;
    static HPI_ERROR_BACKEND_BASE: u16;
    static HPI_ERROR_DSP_BOOTLOAD: u16;
    static HPI_ERROR_CONTROL_CACHING: u16;
    static HPI_ERROR_INVALID_DATASIZE: u16;
    static HPI_ERROR_INVALID_OPERATION: u16;
    static HPI_ERROR_INVALID_DATA_POINTER: u16;
    static HPI_ERROR_MESSAGE_BUFFER_TOO_SMALL: u16;
    static HPI_ERROR_RESPONSE_BUFFER_TOO_SMALL: u16;
    static HPI_ERROR_DSP_COMMUNICATION: u16;

    static HPI_SUBSYS_CREATE_ADAPTER: u16;
    static HPI_CONTROL_GET_STATE: u16;
    static HPI_CONTROL_GET_INFO: u16;
    static HPI_CONTROL_SET_STATE: u16;
    static HPI_ADAPTER_DELETE: u16;
    static HPI_ADAPTER_GET_INFO: u16;
    static HPI_ADAPTER_CLOSE: u16;
    static HPI_ADAPTER_DEBUG_READ: u16;
    static HPI_OSTREAM_WRITE: u16;
    static HPI_OSTREAM_GET_INFO: u16;
    static HPI_OSTREAM_HOSTBUFFER_ALLOC: u16;
    static HPI_OSTREAM_HOSTBUFFER_GET_INFO: u16;
    static HPI_OSTREAM_HOSTBUFFER_FREE: u16;
    static HPI_OSTREAM_START: u16;
    static HPI_OSTREAM_OPEN: u16;
    static HPI_OSTREAM_RESET: u16;
    static HPI_OSTREAM_SET_FORMAT: u16;
    static HPI_OSTREAM_ANC_READ: u16;
    static HPI_ISTREAM_READ: u16;
    static HPI_ISTREAM_GET_INFO: u16;
    static HPI_ISTREAM_HOSTBUFFER_ALLOC: u16;
    static HPI_ISTREAM_HOSTBUFFER_GET_INFO: u16;
    static HPI_ISTREAM_HOSTBUFFER_FREE: u16;
    static HPI_ISTREAM_START: u16;
    static HPI_ISTREAM_ANC_WRITE: u16;

    static HPI_TYPE_REQUEST: u16;
    static HPI_OBJ_SUBSYSTEM: u16;
    static HPI_OBJ_ADAPTER: u16;
    static HPI_OBJ_CONTROL: u16;
    static HPI_OBJ_OSTREAM: u16;
    static HPI_OBJ_ISTREAM: u16;
    static HPI_METER_PEAK: u16;
    static HPI_BUFFER_CMD_EXTERNAL: u32;
    static HPI_BUFFER_CMD_INTERNAL_ALLOC: u32;
    static HPI_BUFFER_CMD_INTERNAL_GRANTADAPTER: u32;
    static HPI_BUFFER_CMD_INTERNAL_REVOKEADAPTER: u32;
    static HPI_BUFFER_CMD_INTERNAL_FREE: u32;
    static HPI_STATE_STOPPED: u32;
    static HPI_IRQ_MIXER: c_int;
    static HPI_IRQ_NONE: c_int;

    static H620_HIF_UNKNOWN: c_int;
    static H620_HIF_RESET: c_int;
    static H620_HIF_IDLE: c_int;
    static H620_HIF_SEND_DATA: c_int;
    static H620_HIF_GET_DATA: c_int;
    static H620_HIF_GET_RESP: c_int;
    static HPI6205_SIZEOF_DATA: u32;

    fn HPI_ADAPTER_ASI(x: u16) -> u16;
    fn HPI_ADAPTER_FAMILY_ASI(x: u16) -> u16;
}

#[repr(C)]
pub struct consistent_dma_area {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct hpi_control_cache {
    pub adap_idx: u16,
}
#[repr(C)]
pub struct hpi_response_header {
    pub size: u16,
}
#[repr(C)]
pub struct pci_dev {
    pub subsystem_device: u16,
}
#[repr(C)]
pub struct hpi_pci {
    pub ap_mem_base: [*mut u32; 2],
    pub pci_dev: *mut pci_dev,
}
#[repr(C)]
pub struct control_cache_desc {
    pub number_of_controls: u32,
    pub size_in_bytes: u32,
    pub physical_address32: u32,
}
#[repr(C)]
pub struct hpi_hostbuffer_status {
    pub samples_processed: u32,
    pub stream_state: u32,
    pub dsp_index: u32,
    pub host_index: u32,
    pub size_in_bytes: u32,
    pub auxiliary_data_available: u32,
}
#[repr(C)]
pub struct bus_master_interface_u {
    pub b_data: [u8; 0],
    pub message_buffer: [u8; 0],
    pub response_buffer: hpi_response,
}
#[repr(C)]
pub struct bus_master_interface {
    pub dsp_ack: c_int,
    pub host_cmd: c_int,
    pub transfer_size_in_bytes: u32,
    pub control_cache: control_cache_desc,
    pub instream_host_buffer_status: *mut hpi_hostbuffer_status,
    pub outstream_host_buffer_status: *mut hpi_hostbuffer_status,
    pub u: bus_master_interface_u,
}
#[repr(C)]
pub struct hpi_adapter_obj {
    pub priv_: *mut hpi_hw_obj,
    pub pci: hpi_pci,
    pub dsp_crashed: u32,
    pub has_control_cache: u16,
    pub type_: u16,
    pub index: u16,
    pub irq_query_and_clear: Option<unsafe extern "C" fn(*mut hpi_adapter_obj, u32) -> c_int>,
    pub instream_host_buffer_status: *mut hpi_hostbuffer_status,
    pub outstream_host_buffer_status: *mut hpi_hostbuffer_status,
}
#[repr(C)]
pub struct hpi_msg_buffer {
    pub command: u32,
    pub buffer_size: u32,
    pub pci_address: u32,
}
#[repr(C)]
pub struct hpi_msg_data {
    pub pb_data: *mut u8,
    pub data_size: u32,
}
#[repr(C)]
pub struct hpi_msg_stream_info {
    pub state: u16,
    pub samples_transferred: u32,
    pub buffer_size: u32,
    pub data_available: u32,
    pub auxiliary_data_available: u32,
}
#[repr(C)]
pub struct hpi_msg_hostbuffer_info {
    pub p_buffer: *mut u8,
    pub p_status: *mut hpi_hostbuffer_status,
}
#[repr(C)]
pub union hpi_msg_du {
    pub buffer: hpi_msg_buffer,
    pub data: hpi_msg_data,
    pub stream_info: hpi_msg_stream_info,
    pub hostbuffer_info: hpi_msg_hostbuffer_info,
}
#[repr(C)]
pub struct hpi_msg_d {
    pub u: hpi_msg_du,
}
#[repr(C)]
pub struct hpi_msg_c {
    pub attribute: u16,
}
#[repr(C)]
pub struct hpi_resource_r {
    pub pci: *mut hpi_pci,
}
#[repr(C)]
pub struct hpi_resource {
    pub r: hpi_resource_r,
}
#[repr(C)]
pub struct hpi_msg_s {
    pub resource: hpi_resource,
    pub adapter_type: u16,
    pub adapter_index: u16,
    pub data: u32,
}
#[repr(C)]
pub union hpi_message_u {
    pub d: hpi_msg_d,
    pub c: hpi_msg_c,
    pub s: hpi_msg_s,
}
#[repr(C)]
pub struct hpi_message {
    pub type_: u16,
    pub size: u16,
    pub object: u16,
    pub function: u16,
    pub adapter_index: u16,
    pub obj_index: u16,
    pub u: hpi_message_u,
}
#[repr(C)]
pub struct hpi_adapter_info {
    pub adapter_type: u16,
    pub adapter_index: u16,
    pub serial_number: u32,
}
#[repr(C)]
pub struct hpi_response_ax {
    pub info: hpi_adapter_info,
}
#[repr(C)]
pub struct hpi_response_s {
    pub data: u32,
    pub adapter_type: u16,
    pub adapter_index: u16,
}
#[repr(C)]
pub struct hpi_response_d {
    pub u: hpi_msg_du,
}
#[repr(C)]
pub union hpi_response_u {
    pub ax: hpi_response_ax,
    pub s: hpi_response_s,
    pub d: hpi_response_d,
}
#[repr(C)]
pub struct hpi_response {
    pub size: u16,
    pub error: u16,
    pub specific_error: u16,
    pub u: hpi_response_u,
}
#[repr(C)]
pub struct dsp_code {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct hpi_hw_obj {
    /* PCI registers */
    pub prHSR: *mut u32,
    pub prHDCR: *mut u32,
    pub prDSPP: *mut u32,
    pub dsp_page: u32,
    pub h_locked_mem: consistent_dma_area,
    pub p_interface_buffer: *mut bus_master_interface,
    pub flag_outstream_just_reset: [u16; 16],
    /* a non-NULL handle means there is an HPI allocated buffer */
    pub instream_host_buffers: [consistent_dma_area; 16],
    pub outstream_host_buffers: [consistent_dma_area; 16],
    /* non-zero size means a buffer exists, may be external */
    pub instream_host_buffer_size: [u32; 16],
    pub outstream_host_buffer_size: [u32; 16],
    pub h_control_cache: consistent_dma_area,
    pub p_cache: *mut hpi_control_cache,
}

extern "C" {
    fn hpi_init_response(phr: *mut hpi_response, object: u16, function: u16, error: u16);
    fn hpi_find_adapter(adapter_index: u16) -> *mut hpi_adapter_obj;
    fn hpi_delete_adapter(pao: *mut hpi_adapter_obj);
    fn hpi_add_adapter(pao: *mut hpi_adapter_obj) -> u16;
    fn hpi_validate_response(phm: *mut hpi_message, phr: *mut hpi_response) -> u16;
    fn hpi_check_control_cache(
        cache: *mut hpi_control_cache,
        phm: *mut hpi_message,
        phr: *mut hpi_response,
    ) -> c_int;
    fn hpi_cmn_control_cache_sync_to_msg(
        cache: *mut hpi_control_cache,
        phm: *mut hpi_message,
        phr: *mut hpi_response,
    );
    fn hpi_alloc_control_cache(n: u32, size: u32, virt: *mut u8) -> *mut hpi_control_cache;
    fn hpi_free_control_cache(cache: *mut hpi_control_cache);
    fn hpios_locked_mem_alloc(area: *mut consistent_dma_area, size: usize, dev: *mut pci_dev) -> u16;
    fn hpios_locked_mem_get_virt_addr(area: *mut consistent_dma_area, ppv: *mut *mut c_void) -> u16;
    fn hpios_locked_mem_get_phys_addr(area: *mut consistent_dma_area, phys: *mut u32) -> u16;
    fn hpios_locked_mem_valid(area: *mut consistent_dma_area) -> c_int;
    fn hpios_locked_mem_free(area: *mut consistent_dma_area);
    fn hpios_delay_micro_seconds(us: u32);
    fn hpios_dsplock_lock(pao: *mut hpi_adapter_obj);
    fn hpios_dsplock_unlock(pao: *mut hpi_adapter_obj);
    fn hpi_dsp_code_open(id: u16, dev: *mut pci_dev, code: *mut dsp_code, os_error: *mut u32) -> u16;
    fn hpi_dsp_code_read_word(code: *mut dsp_code, word: *mut u32) -> u16;
    fn hpi_dsp_code_read_block(length: u32, code: *mut dsp_code, pcode: *mut *mut u32) -> u16;
    fn hpi_dsp_code_rewind(code: *mut dsp_code);
    fn hpi_dsp_code_close(code: *mut dsp_code);
    fn kzalloc(size: usize) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

#[inline]
unsafe fn ioread32(p: *mut u32) -> u32 {
    core::ptr::read_volatile(p)
}
#[inline]
unsafe fn iowrite32(v: u32, p: *mut u32) {
    core::ptr::write_volatile(p, v);
}
#[inline]
fn rmb() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::Acquire);
}
#[inline]
fn wmb() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::Release);
}
#[inline]
fn roundup_pow_of_two(mut v: u32) -> u32 {
    if v <= 1 {
        return 1;
    }
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v + 1
}
#[inline]
fn min_u32(a: u32, b: u32) -> u32 {
    if a < b { a } else { b }
}

macro_rules! HPI_DEBUG_LOG {
    ($($t:tt)*) => {};
}

const HPI6205_ERROR_BASE: u16 = 1000; /* not actually used anywhere */
const HPI6205_ERROR_MSG_RESP_IDLE_TIMEOUT: u16 = 1015;
const HPI6205_ERROR_MSG_RESP_TIMEOUT: u16 = 1016;
const HPI6205_ERROR_6205_NO_IRQ: u16 = 1002;
const HPI6205_ERROR_6205_INIT_FAILED: u16 = 1003;
const HPI6205_ERROR_6205_REG: u16 = 1006;
const HPI6205_ERROR_6205_DSPPAGE: u16 = 1007;
const HPI6205_ERROR_C6713_HPIC: u16 = 1009;
const HPI6205_ERROR_C6713_HPIA: u16 = 1010;
const HPI6205_ERROR_C6713_PLL: u16 = 1011;
const HPI6205_ERROR_DSP_INTMEM: u16 = 1012;
const HPI6205_ERROR_DSP_EXTMEM: u16 = 1013;
const HPI6205_ERROR_DSP_PLD: u16 = 1014;
const HPI6205_ERROR_6205_EEPROM: u16 = 1017;
const HPI6205_ERROR_DSP_EMIF1: u16 = 1018;
const HPI6205_ERROR_DSP_EMIF2: u16 = 1019;
const HPI6205_ERROR_DSP_EMIF3: u16 = 1020;
const HPI6205_ERROR_DSP_EMIF4: u16 = 1021;

/* for C6205 PCI i/f */
/* Host Status Register (HSR) bitfields */
const C6205_HSR_INTSRC: u32 = 0x01;
const C6205_HSR_INTAVAL: u32 = 0x02;
const C6205_HSR_INTAM: u32 = 0x04;
const C6205_HSR_CFGERR: u32 = 0x08;
const C6205_HSR_EEREAD: u32 = 0x10;
/* Host-to-DSP Control Register (HDCR) bitfields */
const C6205_HDCR_WARMRESET: u32 = 0x01;
const C6205_HDCR_DSPINT: u32 = 0x02;
const C6205_HDCR_PCIBOOT: u32 = 0x04;
/* DSP Page Register (DSPP) bitfields, */
/* defines 4 Mbyte page that BAR0 points to */
const C6205_DSPP_MAP1: u32 = 0x400;

const C6205_BAR1_PCI_IO_OFFSET: u32 = 0x027FFF0;
const C6205_BAR1_HSR: u32 = C6205_BAR1_PCI_IO_OFFSET;
const C6205_BAR1_HDCR: u32 = C6205_BAR1_PCI_IO_OFFSET + 4;
const C6205_BAR1_DSPP: u32 = C6205_BAR1_PCI_IO_OFFSET + 8;

/* used to control LED (revA) and reset C6713 (revB) */
const C6205_BAR0_TIMER1_CTL: u32 = 0x01980000;

/* For first 6713 in CE1 space, using DA17,16,2 */
const HPICL_ADDR: u32 = 0x01400000;
const HPICH_ADDR: u32 = 0x01400004;
const HPIAL_ADDR: u32 = 0x01410000;
const HPIAH_ADDR: u32 = 0x01410004;
const HPIDIL_ADDR: u32 = 0x01420000;
const HPIDIH_ADDR: u32 = 0x01420004;
const HPIDL_ADDR: u32 = 0x01430000;
const HPIDH_ADDR: u32 = 0x01430004;

const C6713_EMIF_GCTL: u32 = 0x01800000;
const C6713_EMIF_CE1: u32 = 0x01800004;
const C6713_EMIF_CE0: u32 = 0x01800008;
const C6713_EMIF_CE2: u32 = 0x01800010;
const C6713_EMIF_CE3: u32 = 0x01800014;
const C6713_EMIF_SDRAMCTL: u32 = 0x01800018;
const C6713_EMIF_SDRAMTIMING: u32 = 0x0180001C;
const C6713_EMIF_SDRAMEXT: u32 = 0x01800020;
const HPI6205_TIMEOUT: c_int = 1000000;
const HPI6205_MAX_FILES_TO_LOAD: usize = 2;
const WS_OFS: u32 = 28;
const WST_OFS: u32 = 22;
const WH_OFS: u32 = 20;
const RS_OFS: u32 = 16;
const RST_OFS: u32 = 8;
const MTYPE_OFS: u32 = 4;
const RH_OFS: u32 = 0;

unsafe fn subsys_message(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    match (*phm).function {
        f if f == HPI_SUBSYS_CREATE_ADAPTER => subsys_create_adapter(phm, phr),
        _ => (*phr).error = HPI_ERROR_INVALID_FUNC,
    }
}

unsafe fn control_message(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let phw = (*pao).priv_;
    let mut pending_cache_error: u16 = 0;
    match (*phm).function {
        f if f == HPI_CONTROL_GET_STATE => {
            if (*pao).has_control_cache != 0 {
                rmb(); /* make sure we see updates DMAed from DSP */
                if hpi_check_control_cache((*phw).p_cache, phm, phr) != 0 {
                    return;
                } else if (*phm).u.c.attribute == HPI_METER_PEAK {
                    pending_cache_error = HPI_ERROR_CONTROL_CACHING;
                }
            }
            hw_message(pao, phm, phr);
            if pending_cache_error != 0 && (*phr).error == 0 {
                (*phr).error = pending_cache_error;
            }
        }
        f if f == HPI_CONTROL_GET_INFO => hw_message(pao, phm, phr),
        f if f == HPI_CONTROL_SET_STATE => {
            hw_message(pao, phm, phr);
            if (*pao).has_control_cache != 0 {
                hpi_cmn_control_cache_sync_to_msg((*phw).p_cache, phm, phr);
            }
        }
        _ => (*phr).error = HPI_ERROR_INVALID_FUNC,
    }
}

unsafe fn adapter_message(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    if (*phm).function == HPI_ADAPTER_DELETE {
        adapter_delete(pao, phm, phr);
    } else {
        hw_message(pao, phm, phr);
    }
}

unsafe fn outstream_message(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    if (*phm).obj_index as usize >= HPI_MAX_STREAMS {
        (*phr).error = HPI_ERROR_INVALID_OBJ_INDEX;
        HPI_DEBUG_LOG!(WARNING, "Message referencing invalid stream\n");
        return;
    }
    match (*phm).function {
        f if f == HPI_OSTREAM_WRITE => outstream_write(pao, phm, phr),
        f if f == HPI_OSTREAM_GET_INFO => outstream_get_info(pao, phm, phr),
        f if f == HPI_OSTREAM_HOSTBUFFER_ALLOC => outstream_host_buffer_allocate(pao, phm, phr),
        f if f == HPI_OSTREAM_HOSTBUFFER_GET_INFO => outstream_host_buffer_get_info(pao, phm, phr),
        f if f == HPI_OSTREAM_HOSTBUFFER_FREE => outstream_host_buffer_free(pao, phm, phr),
        f if f == HPI_OSTREAM_START => outstream_start(pao, phm, phr),
        f if f == HPI_OSTREAM_OPEN => outstream_open(pao, phm, phr),
        f if f == HPI_OSTREAM_RESET => outstream_reset(pao, phm, phr),
        _ => hw_message(pao, phm, phr),
    }
}

unsafe fn instream_message(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    if (*phm).obj_index as usize >= HPI_MAX_STREAMS {
        (*phr).error = HPI_ERROR_INVALID_OBJ_INDEX;
        HPI_DEBUG_LOG!(WARNING, "Message referencing invalid stream\n");
        return;
    }
    match (*phm).function {
        f if f == HPI_ISTREAM_READ => instream_read(pao, phm, phr),
        f if f == HPI_ISTREAM_GET_INFO => instream_get_info(pao, phm, phr),
        f if f == HPI_ISTREAM_HOSTBUFFER_ALLOC => instream_host_buffer_allocate(pao, phm, phr),
        f if f == HPI_ISTREAM_HOSTBUFFER_GET_INFO => instream_host_buffer_get_info(pao, phm, phr),
        f if f == HPI_ISTREAM_HOSTBUFFER_FREE => instream_host_buffer_free(pao, phm, phr),
        f if f == HPI_ISTREAM_START => instream_start(pao, phm, phr),
        _ => hw_message(pao, phm, phr),
    }
}

/** Entry point to this HPI backend
 * All calls to the HPI start here
 */
unsafe fn _HPI_6205(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    if !pao.is_null() && (*pao).dsp_crashed >= 10 && (*phm).function != HPI_ADAPTER_DEBUG_READ {
        /* allow last resort debug read even after crash */
        hpi_init_response(phr, (*phm).object, (*phm).function, HPI_ERROR_DSP_HARDWARE);
        HPI_DEBUG_LOG!(WARNING, "dsp crashed\n");
        return;
    }
    /* Init default response */
    if (*phm).function != HPI_SUBSYS_CREATE_ADAPTER {
        (*phr).error = HPI_ERROR_PROCESSING_MESSAGE;
    }
    HPI_DEBUG_LOG!(VERBOSE, "start of switch\n");
    if (*phm).type_ == HPI_TYPE_REQUEST {
        match (*phm).object {
            o if o == HPI_OBJ_SUBSYSTEM => subsys_message(pao, phm, phr),
            o if o == HPI_OBJ_ADAPTER => adapter_message(pao, phm, phr),
            o if o == HPI_OBJ_CONTROL => control_message(pao, phm, phr),
            o if o == HPI_OBJ_OSTREAM => outstream_message(pao, phm, phr),
            o if o == HPI_OBJ_ISTREAM => instream_message(pao, phm, phr),
            _ => hw_message(pao, phm, phr),
        }
    } else {
        (*phr).error = HPI_ERROR_INVALID_TYPE;
    }
}

#[no_mangle]
pub unsafe extern "C" fn HPI_6205(phm: *mut hpi_message, phr: *mut hpi_response) {
    let mut pao: *mut hpi_adapter_obj = null_mut();
    if (*phm).object != HPI_OBJ_SUBSYSTEM {
        /* normal messages must have valid adapter index */
        pao = hpi_find_adapter((*phm).adapter_index);
    } else {
        /* subsys messages don't address an adapter */
        (*phr).error = HPI_ERROR_INVALID_OBJ_INDEX;
        return;
    }
    if !pao.is_null() {
        _HPI_6205(pao, phm, phr);
    } else {
        hpi_init_response(phr, (*phm).object, (*phm).function, HPI_ERROR_BAD_ADAPTER_NUMBER);
    }
}

unsafe fn subsys_create_adapter(phm: *mut hpi_message, phr: *mut hpi_response) {
    /* create temp adapter obj, because we don't know what index yet */
    let mut ao: hpi_adapter_obj = zeroed();
    let mut os_error_code: u32 = 0;
    HPI_DEBUG_LOG!(DEBUG, " subsys_create_adapter\n");
    ao.priv_ = kzalloc(size_of::<hpi_hw_obj>()) as *mut hpi_hw_obj;
    if ao.priv_.is_null() {
        HPI_DEBUG_LOG!(ERROR, "can't get mem for adapter object\n");
        (*phr).error = HPI_ERROR_MEMORY_ALLOC;
        return;
    }
    ao.pci = *(*phm).u.s.resource.r.pci;
    let err = create_adapter_obj(&mut ao, &mut os_error_code);
    if err != 0 {
        delete_adapter_obj(&mut ao);
        if err >= HPI_ERROR_BACKEND_BASE {
            (*phr).error = HPI_ERROR_DSP_BOOTLOAD;
            (*phr).specific_error = err;
        } else {
            (*phr).error = err;
        }
        (*phr).u.s.data = os_error_code;
        return;
    }
    (*phr).u.s.adapter_type = ao.type_;
    (*phr).u.s.adapter_index = ao.index;
    (*phr).error = 0;
}

/** delete an adapter - required by WDM driver */
unsafe fn adapter_delete(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    if pao.is_null() {
        (*phr).error = HPI_ERROR_INVALID_OBJ_INDEX;
        return;
    }
    let phw = (*pao).priv_;
    /* reset adapter h/w */
    /* Reset C6713 #1 */
    boot_loader_write_mem32(pao, 0, C6205_BAR0_TIMER1_CTL, 0);
    /* reset C6205 */
    iowrite32(C6205_HDCR_WARMRESET, (*phw).prHDCR);
    delete_adapter_obj(pao);
    hpi_delete_adapter(pao);
    (*phr).error = 0;
}

unsafe fn create_adapter_obj(pao: *mut hpi_adapter_obj, pos_error_code: *mut u32) -> u16 {
    let phw = (*pao).priv_;
    let mut phys_addr: u32 = 0;
    (*pao).dsp_crashed = 0;
    for i in 0..HPI_MAX_STREAMS {
        (*phw).flag_outstream_just_reset[i] = 1;
    }
    (*phw).prHSR = (*pao).pci.ap_mem_base[1].add((C6205_BAR1_HSR as usize) / size_of::<u32>());
    (*phw).prHDCR = (*pao).pci.ap_mem_base[1].add((C6205_BAR1_HDCR as usize) / size_of::<u32>());
    (*phw).prDSPP = (*pao).pci.ap_mem_base[1].add((C6205_BAR1_DSPP as usize) / size_of::<u32>());
    (*pao).has_control_cache = 0;

    if hpios_locked_mem_alloc(&mut (*phw).h_locked_mem, size_of::<bus_master_interface>(), (*pao).pci.pci_dev) != 0 {
        (*phw).p_interface_buffer = null_mut();
    } else if hpios_locked_mem_get_virt_addr(
        &mut (*phw).h_locked_mem,
        &mut (*phw).p_interface_buffer as *mut _ as *mut *mut c_void,
    ) != 0 {
        (*phw).p_interface_buffer = null_mut();
    }
    HPI_DEBUG_LOG!(DEBUG, "interface buffer address\n");
    if !(*phw).p_interface_buffer.is_null() {
        write_bytes((*phw).p_interface_buffer as *mut u8, 0, size_of::<bus_master_interface>());
        (*(*phw).p_interface_buffer).dsp_ack = H620_HIF_UNKNOWN;
    }
    let mut err = adapter_boot_load_dsp(pao, pos_error_code);
    if err != 0 {
        HPI_DEBUG_LOG!(ERROR, "DSP code load failed\n");
        return err;
    }
    HPI_DEBUG_LOG!(INFO, "load DSP code OK\n");
    if (*phw).p_interface_buffer.is_null() {
        return HPI_ERROR_MEMORY_ALLOC;
    }
    let interface = (*phw).p_interface_buffer;
    if wait_dsp_ack(phw, H620_HIF_RESET, HPI6205_TIMEOUT * 10) == 0 {
        HPI_DEBUG_LOG!(ERROR, "timed out waiting reset state\n");
        return HPI6205_ERROR_6205_INIT_FAILED;
    }
    if (*interface).control_cache.number_of_controls != 0 {
        let mut p_control_cache_virtual: *mut u8 = null_mut();
        err = hpios_locked_mem_alloc(
            &mut (*phw).h_control_cache,
            (*interface).control_cache.size_in_bytes as usize,
            (*pao).pci.pci_dev,
        );
        if err == 0 {
            err = hpios_locked_mem_get_virt_addr(
                &mut (*phw).h_control_cache,
                &mut p_control_cache_virtual as *mut _ as *mut *mut c_void,
            );
        }
        if err == 0 {
            write_bytes(p_control_cache_virtual, 0, (*interface).control_cache.size_in_bytes as usize);
            (*phw).p_cache = hpi_alloc_control_cache(
                (*interface).control_cache.number_of_controls,
                (*interface).control_cache.size_in_bytes,
                p_control_cache_virtual,
            );
            if (*phw).p_cache.is_null() {
                err = HPI_ERROR_MEMORY_ALLOC;
            }
        }
        if err == 0 {
            err = hpios_locked_mem_get_phys_addr(&mut (*phw).h_control_cache, &mut phys_addr);
            (*interface).control_cache.physical_address32 = phys_addr;
        }
        if err == 0 {
            (*pao).has_control_cache = 1;
        } else {
            if hpios_locked_mem_valid(&mut (*phw).h_control_cache) != 0 {
                hpios_locked_mem_free(&mut (*phw).h_control_cache);
            }
            (*pao).has_control_cache = 0;
        }
    }
    send_dsp_command(phw, H620_HIF_IDLE);
    {
        let mut hm: hpi_message = zeroed();
        let mut hr: hpi_response = zeroed();
        HPI_DEBUG_LOG!(VERBOSE, "init ADAPTER_GET_INFO\n");
        hm.type_ = HPI_TYPE_REQUEST;
        hm.size = size_of::<hpi_message>() as u16;
        hm.object = HPI_OBJ_ADAPTER;
        hm.function = HPI_ADAPTER_GET_INFO;
        hr.size = size_of::<hpi_response>() as u16;
        err = message_response_sequence(pao, &mut hm, &mut hr);
        if err != 0 {
            HPI_DEBUG_LOG!(ERROR, "message transport error\n");
            return err;
        }
        if hr.error != 0 {
            return hr.error;
        }
        (*pao).type_ = hr.u.ax.info.adapter_type;
        (*pao).index = hr.u.ax.info.adapter_index;
        HPI_DEBUG_LOG!(VERBOSE, "got adapter info\n");
    }
    if !(*phw).p_cache.is_null() {
        (*(*phw).p_cache).adap_idx = (*pao).index;
    }
    HPI_DEBUG_LOG!(INFO, "bootload DSP OK\n");
    (*pao).irq_query_and_clear = Some(adapter_irq_query_and_clear);
    (*pao).instream_host_buffer_status = (*(*phw).p_interface_buffer).instream_host_buffer_status;
    (*pao).outstream_host_buffer_status = (*(*phw).p_interface_buffer).outstream_host_buffer_status;
    hpi_add_adapter(pao)
}

unsafe fn delete_adapter_obj(pao: *mut hpi_adapter_obj) {
    let phw = (*pao).priv_;
    if hpios_locked_mem_valid(&mut (*phw).h_control_cache) != 0 {
        hpios_locked_mem_free(&mut (*phw).h_control_cache);
        hpi_free_control_cache((*phw).p_cache);
    }
    if hpios_locked_mem_valid(&mut (*phw).h_locked_mem) != 0 {
        hpios_locked_mem_free(&mut (*phw).h_locked_mem);
        (*phw).p_interface_buffer = null_mut();
    }
    for i in 0..HPI_MAX_STREAMS {
        if hpios_locked_mem_valid(&mut (*phw).instream_host_buffers[i]) != 0 {
            hpios_locked_mem_free(&mut (*phw).instream_host_buffers[i]);
            (*phw).instream_host_buffer_size[i] = 0;
        }
    }
    for i in 0..HPI_MAX_STREAMS {
        if hpios_locked_mem_valid(&mut (*phw).outstream_host_buffers[i]) != 0 {
            hpios_locked_mem_free(&mut (*phw).outstream_host_buffers[i]);
            (*phw).outstream_host_buffer_size[i] = 0;
        }
    }
    kfree(phw as *mut c_void);
}

unsafe extern "C" fn adapter_irq_query_and_clear(pao: *mut hpi_adapter_obj, message: u32) -> c_int {
    let phw = (*pao).priv_;
    let hsr = ioread32((*phw).prHSR);
    if (hsr & C6205_HSR_INTSRC) != 0 {
        /* reset the interrupt from the DSP */
        iowrite32(C6205_HSR_INTSRC, (*phw).prHSR);
        return HPI_IRQ_MIXER;
    }
    HPI_IRQ_NONE
}

/** Allocate or attach buffer for busmastering */
unsafe fn outstream_host_buffer_allocate(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let mut err: u16 = 0;
    let command = (*phm).u.d.u.buffer.command;
    let phw = (*pao).priv_;
    let interface = (*phw).p_interface_buffer;
    let idx = (*phm).obj_index as usize;
    hpi_init_response(phr, (*phm).object, (*phm).function, 0);
    if command == HPI_BUFFER_CMD_EXTERNAL || command == HPI_BUFFER_CMD_INTERNAL_ALLOC {
        (*phm).u.d.u.buffer.buffer_size = roundup_pow_of_two((*phm).u.d.u.buffer.buffer_size);
        (*phr).u.d.u.stream_info.data_available = (*phw).outstream_host_buffer_size[idx];
        (*phr).u.d.u.stream_info.buffer_size = (*phm).u.d.u.buffer.buffer_size;
        if (*phw).outstream_host_buffer_size[idx] == (*phm).u.d.u.buffer.buffer_size {
            return;
        }
        if hpios_locked_mem_valid(&mut (*phw).outstream_host_buffers[idx]) != 0 {
            hpios_locked_mem_free(&mut (*phw).outstream_host_buffers[idx]);
        }
        err = hpios_locked_mem_alloc(&mut (*phw).outstream_host_buffers[idx], (*phm).u.d.u.buffer.buffer_size as usize, (*pao).pci.pci_dev);
        if err != 0 {
            (*phr).error = HPI_ERROR_INVALID_DATASIZE;
            (*phw).outstream_host_buffer_size[idx] = 0;
            return;
        }
        err = hpios_locked_mem_get_phys_addr(&mut (*phw).outstream_host_buffers[idx], &mut (*phm).u.d.u.buffer.pci_address);
        (*phr).u.d.u.stream_info.auxiliary_data_available = (*phm).u.d.u.buffer.pci_address;
        if err != 0 {
            hpios_locked_mem_free(&mut (*phw).outstream_host_buffers[idx]);
            (*phw).outstream_host_buffer_size[idx] = 0;
            (*phr).error = HPI_ERROR_MEMORY_ALLOC;
            return;
        }
    }
    if command == HPI_BUFFER_CMD_EXTERNAL || command == HPI_BUFFER_CMD_INTERNAL_GRANTADAPTER {
        if ((*phm).u.d.u.buffer.buffer_size & ((*phm).u.d.u.buffer.buffer_size - 1)) != 0 {
            HPI_DEBUG_LOG!(ERROR, "Buffer size must be 2^N\n");
            (*phr).error = HPI_ERROR_INVALID_DATASIZE;
            return;
        }
        (*phw).outstream_host_buffer_size[idx] = (*phm).u.d.u.buffer.buffer_size;
        let status = (*interface).outstream_host_buffer_status.add(idx);
        (*status).samples_processed = 0;
        (*status).stream_state = HPI_STATE_STOPPED;
        (*status).dsp_index = 0;
        (*status).host_index = (*status).dsp_index;
        (*status).size_in_bytes = (*phm).u.d.u.buffer.buffer_size;
        (*status).auxiliary_data_available = 0;
        hw_message(pao, phm, phr);
        if (*phr).error != 0 && hpios_locked_mem_valid(&mut (*phw).outstream_host_buffers[idx]) != 0 {
            hpios_locked_mem_free(&mut (*phw).outstream_host_buffers[idx]);
            (*phw).outstream_host_buffer_size[idx] = 0;
        }
    }
}

unsafe fn outstream_host_buffer_get_info(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let phw = (*pao).priv_;
    let interface = (*phw).p_interface_buffer;
    let idx = (*phm).obj_index as usize;
    let mut p_bbm_data: *mut u8 = null_mut();
    if hpios_locked_mem_valid(&mut (*phw).outstream_host_buffers[idx]) != 0 {
        if hpios_locked_mem_get_virt_addr(&mut (*phw).outstream_host_buffers[idx], &mut p_bbm_data as *mut _ as *mut *mut c_void) != 0 {
            (*phr).error = HPI_ERROR_INVALID_OPERATION;
            return;
        }
        let status = (*interface).outstream_host_buffer_status.add(idx);
        hpi_init_response(phr, HPI_OBJ_OSTREAM, HPI_OSTREAM_HOSTBUFFER_GET_INFO, 0);
        (*phr).u.d.u.hostbuffer_info.p_buffer = p_bbm_data;
        (*phr).u.d.u.hostbuffer_info.p_status = status;
    } else {
        hpi_init_response(phr, HPI_OBJ_OSTREAM, HPI_OSTREAM_HOSTBUFFER_GET_INFO, HPI_ERROR_INVALID_OPERATION);
    }
}

unsafe fn outstream_host_buffer_free(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let phw = (*pao).priv_;
    let command = (*phm).u.d.u.buffer.command;
    let idx = (*phm).obj_index as usize;
    if (*phw).outstream_host_buffer_size[idx] != 0 {
        if command == HPI_BUFFER_CMD_EXTERNAL || command == HPI_BUFFER_CMD_INTERNAL_REVOKEADAPTER {
            (*phw).outstream_host_buffer_size[idx] = 0;
            hw_message(pao, phm, phr);
        }
        if command == HPI_BUFFER_CMD_EXTERNAL || command == HPI_BUFFER_CMD_INTERNAL_FREE {
            hpios_locked_mem_free(&mut (*phw).outstream_host_buffers[idx]);
        }
    } else {
        hpi_init_response(phr, HPI_OBJ_OSTREAM, HPI_OSTREAM_HOSTBUFFER_FREE, 0);
    }
}

unsafe fn outstream_get_space_available(status: *mut hpi_hostbuffer_status) -> u32 {
    (*status).size_in_bytes.wrapping_sub((*status).host_index.wrapping_sub((*status).dsp_index))
}

unsafe fn outstream_write(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let phw = (*pao).priv_;
    let interface = (*phw).p_interface_buffer;
    let idx = (*phm).obj_index as usize;
    if (*phw).outstream_host_buffer_size[idx] == 0 {
        hw_message(pao, phm, phr);
        return;
    }
    hpi_init_response(phr, (*phm).object, (*phm).function, 0);
    let status = (*interface).outstream_host_buffer_status.add(idx);
    let space_available = outstream_get_space_available(status);
    if space_available < (*phm).u.d.u.data.data_size {
        (*phr).error = HPI_ERROR_INVALID_DATASIZE;
        return;
    }
    if !(*phm).u.d.u.data.pb_data.is_null() && hpios_locked_mem_valid(&mut (*phw).outstream_host_buffers[idx]) != 0 {
        let mut p_bbm_data: *mut u8 = null_mut();
        let p_app_data = (*phm).u.d.u.data.pb_data;
        if hpios_locked_mem_get_virt_addr(&mut (*phw).outstream_host_buffers[idx], &mut p_bbm_data as *mut _ as *mut *mut c_void) != 0 {
            (*phr).error = HPI_ERROR_INVALID_OPERATION;
            return;
        }
        let l_first_write = min_u32((*phm).u.d.u.data.data_size, (*status).size_in_bytes - ((*status).host_index & ((*status).size_in_bytes - 1)));
        copy_nonoverlapping(p_app_data, p_bbm_data.add(((*status).host_index & ((*status).size_in_bytes - 1)) as usize), l_first_write as usize);
        copy_nonoverlapping(p_app_data.add(l_first_write as usize), p_bbm_data, ((*phm).u.d.u.data.data_size - l_first_write) as usize);
    }
    if (*phw).flag_outstream_just_reset[idx] != 0 {
        let function = (*phm).function;
        (*phw).flag_outstream_just_reset[idx] = 0;
        (*phm).function = HPI_OSTREAM_SET_FORMAT;
        hw_message(pao, phm, phr);
        (*phm).function = function;
        if (*phr).error != 0 {
            return;
        }
    }
    (*status).host_index = (*status).host_index.wrapping_add((*phm).u.d.u.data.data_size);
}

unsafe fn outstream_get_info(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let phw = (*pao).priv_;
    let interface = (*phw).p_interface_buffer;
    let idx = (*phm).obj_index as usize;
    if (*phw).outstream_host_buffer_size[idx] == 0 {
        hw_message(pao, phm, phr);
        return;
    }
    hpi_init_response(phr, (*phm).object, (*phm).function, 0);
    let status = (*interface).outstream_host_buffer_status.add(idx);
    (*phr).u.d.u.stream_info.state = (*status).stream_state as u16;
    (*phr).u.d.u.stream_info.samples_transferred = (*status).samples_processed;
    (*phr).u.d.u.stream_info.buffer_size = (*status).size_in_bytes;
    (*phr).u.d.u.stream_info.data_available = (*status).size_in_bytes - outstream_get_space_available(status);
    (*phr).u.d.u.stream_info.auxiliary_data_available = (*status).auxiliary_data_available;
}

unsafe fn outstream_start(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    hw_message(pao, phm, phr);
}
unsafe fn outstream_reset(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    (*(*pao).priv_).flag_outstream_just_reset[(*phm).obj_index as usize] = 1;
    hw_message(pao, phm, phr);
}
unsafe fn outstream_open(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    outstream_reset(pao, phm, phr);
}

unsafe fn instream_host_buffer_allocate(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let mut err: u16 = 0;
    let command = (*phm).u.d.u.buffer.command;
    let phw = (*pao).priv_;
    let interface = (*phw).p_interface_buffer;
    let idx = (*phm).obj_index as usize;
    hpi_init_response(phr, (*phm).object, (*phm).function, 0);
    if command == HPI_BUFFER_CMD_EXTERNAL || command == HPI_BUFFER_CMD_INTERNAL_ALLOC {
        (*phm).u.d.u.buffer.buffer_size = roundup_pow_of_two((*phm).u.d.u.buffer.buffer_size);
        (*phr).u.d.u.stream_info.data_available = (*phw).instream_host_buffer_size[idx];
        (*phr).u.d.u.stream_info.buffer_size = (*phm).u.d.u.buffer.buffer_size;
        if (*phw).instream_host_buffer_size[idx] == (*phm).u.d.u.buffer.buffer_size {
            return;
        }
        if hpios_locked_mem_valid(&mut (*phw).instream_host_buffers[idx]) != 0 {
            hpios_locked_mem_free(&mut (*phw).instream_host_buffers[idx]);
        }
        err = hpios_locked_mem_alloc(&mut (*phw).instream_host_buffers[idx], (*phm).u.d.u.buffer.buffer_size as usize, (*pao).pci.pci_dev);
        if err != 0 {
            (*phr).error = HPI_ERROR_INVALID_DATASIZE;
            (*phw).instream_host_buffer_size[idx] = 0;
            return;
        }
        err = hpios_locked_mem_get_phys_addr(&mut (*phw).instream_host_buffers[idx], &mut (*phm).u.d.u.buffer.pci_address);
        (*phr).u.d.u.stream_info.auxiliary_data_available = (*phm).u.d.u.buffer.pci_address;
        if err != 0 {
            hpios_locked_mem_free(&mut (*phw).instream_host_buffers[idx]);
            (*phw).instream_host_buffer_size[idx] = 0;
            (*phr).error = HPI_ERROR_MEMORY_ALLOC;
            return;
        }
    }
    if command == HPI_BUFFER_CMD_EXTERNAL || command == HPI_BUFFER_CMD_INTERNAL_GRANTADAPTER {
        if ((*phm).u.d.u.buffer.buffer_size & ((*phm).u.d.u.buffer.buffer_size - 1)) != 0 {
            HPI_DEBUG_LOG!(ERROR, "Buffer size must be 2^N\n");
            (*phr).error = HPI_ERROR_INVALID_DATASIZE;
            return;
        }
        (*phw).instream_host_buffer_size[idx] = (*phm).u.d.u.buffer.buffer_size;
        let status = (*interface).instream_host_buffer_status.add(idx);
        (*status).samples_processed = 0;
        (*status).stream_state = HPI_STATE_STOPPED;
        (*status).dsp_index = 0;
        (*status).host_index = (*status).dsp_index;
        (*status).size_in_bytes = (*phm).u.d.u.buffer.buffer_size;
        (*status).auxiliary_data_available = 0;
        hw_message(pao, phm, phr);
        if (*phr).error != 0 && hpios_locked_mem_valid(&mut (*phw).instream_host_buffers[idx]) != 0 {
            hpios_locked_mem_free(&mut (*phw).instream_host_buffers[idx]);
            (*phw).instream_host_buffer_size[idx] = 0;
        }
    }
}

unsafe fn instream_host_buffer_get_info(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let phw = (*pao).priv_;
    let interface = (*phw).p_interface_buffer;
    let idx = (*phm).obj_index as usize;
    let mut p_bbm_data: *mut u8 = null_mut();
    if hpios_locked_mem_valid(&mut (*phw).instream_host_buffers[idx]) != 0 {
        if hpios_locked_mem_get_virt_addr(&mut (*phw).instream_host_buffers[idx], &mut p_bbm_data as *mut _ as *mut *mut c_void) != 0 {
            (*phr).error = HPI_ERROR_INVALID_OPERATION;
            return;
        }
        let status = (*interface).instream_host_buffer_status.add(idx);
        hpi_init_response(phr, HPI_OBJ_ISTREAM, HPI_ISTREAM_HOSTBUFFER_GET_INFO, 0);
        (*phr).u.d.u.hostbuffer_info.p_buffer = p_bbm_data;
        (*phr).u.d.u.hostbuffer_info.p_status = status;
    } else {
        hpi_init_response(phr, HPI_OBJ_ISTREAM, HPI_ISTREAM_HOSTBUFFER_GET_INFO, HPI_ERROR_INVALID_OPERATION);
    }
}

unsafe fn instream_host_buffer_free(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let phw = (*pao).priv_;
    let command = (*phm).u.d.u.buffer.command;
    let idx = (*phm).obj_index as usize;
    if (*phw).instream_host_buffer_size[idx] != 0 {
        if command == HPI_BUFFER_CMD_EXTERNAL || command == HPI_BUFFER_CMD_INTERNAL_REVOKEADAPTER {
            (*phw).instream_host_buffer_size[idx] = 0;
            hw_message(pao, phm, phr);
        }
        if command == HPI_BUFFER_CMD_EXTERNAL || command == HPI_BUFFER_CMD_INTERNAL_FREE {
            hpios_locked_mem_free(&mut (*phw).instream_host_buffers[idx]);
        }
    } else {
        hpi_init_response(phr, HPI_OBJ_ISTREAM, HPI_ISTREAM_HOSTBUFFER_FREE, 0);
    }
}

unsafe fn instream_start(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    hw_message(pao, phm, phr);
}
unsafe fn instream_get_bytes_available(status: *mut hpi_hostbuffer_status) -> u32 {
    (*status).dsp_index.wrapping_sub((*status).host_index)
}
unsafe fn instream_read(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let phw = (*pao).priv_;
    let interface = (*phw).p_interface_buffer;
    let idx = (*phm).obj_index as usize;
    let p_app_data = (*phm).u.d.u.data.pb_data;
    if (*phw).instream_host_buffer_size[idx] == 0 {
        hw_message(pao, phm, phr);
        return;
    }
    hpi_init_response(phr, (*phm).object, (*phm).function, 0);
    let status = (*interface).instream_host_buffer_status.add(idx);
    let data_available = instream_get_bytes_available(status);
    if data_available < (*phm).u.d.u.data.data_size {
        (*phr).error = HPI_ERROR_INVALID_DATASIZE;
        return;
    }
    if hpios_locked_mem_valid(&mut (*phw).instream_host_buffers[idx]) != 0 {
        let mut p_bbm_data: *mut u8 = null_mut();
        if hpios_locked_mem_get_virt_addr(&mut (*phw).instream_host_buffers[idx], &mut p_bbm_data as *mut _ as *mut *mut c_void) != 0 {
            (*phr).error = HPI_ERROR_INVALID_OPERATION;
            return;
        }
        let l_first_read = min_u32((*phm).u.d.u.data.data_size, (*status).size_in_bytes - ((*status).host_index & ((*status).size_in_bytes - 1)));
        copy_nonoverlapping(p_bbm_data.add(((*status).host_index & ((*status).size_in_bytes - 1)) as usize), p_app_data, l_first_read as usize);
        copy_nonoverlapping(p_bbm_data, p_app_data.add(l_first_read as usize), ((*phm).u.d.u.data.data_size - l_first_read) as usize);
    }
    (*status).host_index = (*status).host_index.wrapping_add((*phm).u.d.u.data.data_size);
}
unsafe fn instream_get_info(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let phw = (*pao).priv_;
    let interface = (*phw).p_interface_buffer;
    let idx = (*phm).obj_index as usize;
    if (*phw).instream_host_buffer_size[idx] == 0 {
        hw_message(pao, phm, phr);
        return;
    }
    let status = (*interface).instream_host_buffer_status.add(idx);
    hpi_init_response(phr, (*phm).object, (*phm).function, 0);
    (*phr).u.d.u.stream_info.state = (*status).stream_state as u16;
    (*phr).u.d.u.stream_info.samples_transferred = (*status).samples_processed;
    (*phr).u.d.u.stream_info.buffer_size = (*status).size_in_bytes;
    (*phr).u.d.u.stream_info.data_available = instream_get_bytes_available(status);
    (*phr).u.d.u.stream_info.auxiliary_data_available = (*status).auxiliary_data_available;
}

unsafe fn adapter_boot_load_dsp(pao: *mut hpi_adapter_obj, pos_error_code: *mut u32) -> u16 {
    let phw = (*pao).priv_;
    let mut dsp_code: dsp_code = zeroed();
    let mut boot_code_id = [0u16; HPI6205_MAX_FILES_TO_LOAD];
    let mut temp: u32;
    let mut err: u16 = 0;

    boot_code_id[0] = HPI_ADAPTER_ASI(0x6205);
    boot_code_id[1] = (*(*pao).pci.pci_dev).subsystem_device;
    boot_code_id[1] = HPI_ADAPTER_FAMILY_ASI(boot_code_id[1]);
    match boot_code_id[1] {
        x if x == HPI_ADAPTER_FAMILY_ASI(0x5000) => {
            boot_code_id[0] = boot_code_id[1];
            boot_code_id[1] = 0;
        }
        x if x == HPI_ADAPTER_FAMILY_ASI(0x5300)
            || x == HPI_ADAPTER_FAMILY_ASI(0x5400)
            || x == HPI_ADAPTER_FAMILY_ASI(0x6300) => boot_code_id[1] = HPI_ADAPTER_FAMILY_ASI(0x6400),
        x if x == HPI_ADAPTER_FAMILY_ASI(0x5500)
            || x == HPI_ADAPTER_FAMILY_ASI(0x5600)
            || x == HPI_ADAPTER_FAMILY_ASI(0x6500) => boot_code_id[1] = HPI_ADAPTER_FAMILY_ASI(0x6600),
        x if x == HPI_ADAPTER_FAMILY_ASI(0x8800) => boot_code_id[1] = HPI_ADAPTER_FAMILY_ASI(0x8900),
        _ => {}
    }

    temp = C6205_HDCR_WARMRESET;
    iowrite32(temp, (*phw).prHDCR);
    hpios_delay_micro_seconds(1000);
    temp = ioread32((*phw).prHSR);
    if (temp & (C6205_HSR_CFGERR | C6205_HSR_EEREAD)) != C6205_HSR_EEREAD {
        return HPI6205_ERROR_6205_EEPROM;
    }
    temp |= 0x04;
    iowrite32(temp, (*phw).prHSR);
    temp = ioread32((*phw).prHDCR);
    if (temp & C6205_HDCR_PCIBOOT) == 0 {
        return HPI6205_ERROR_6205_REG;
    }
    for page in [3u32, 2, 1, 0] {
        iowrite32(page, (*phw).prDSPP);
        if (page | C6205_DSPP_MAP1) != ioread32((*phw).prDSPP) {
            return HPI6205_ERROR_6205_DSPPAGE;
        }
    }
    (*phw).dsp_page = 0;

    if boot_code_id[1] != 0 {
        boot_loader_write_mem32(pao, 0, 0x018C0024, 0x00002202);
        hpios_delay_micro_seconds(100);
        boot_loader_write_mem32(pao, 0, C6205_BAR0_TIMER1_CTL, 0);
        if (boot_loader_read_mem32(pao, 0, C6205_BAR0_TIMER1_CTL) & !8) != 0 {
            return HPI6205_ERROR_6205_REG;
        }
        hpios_delay_micro_seconds(100);
        boot_loader_write_mem32(pao, 0, C6205_BAR0_TIMER1_CTL, 4);
        if (boot_loader_read_mem32(pao, 0, C6205_BAR0_TIMER1_CTL) & !8) != 4 {
            return HPI6205_ERROR_6205_REG;
        }
        hpios_delay_micro_seconds(100);
    }

    for dsp in 0..HPI6205_MAX_FILES_TO_LOAD {
        if boot_code_id[dsp] == 0 {
            continue;
        }
        err = boot_loader_config_emif(pao, dsp as c_int);
        if err != 0 { return err; }
        err = boot_loader_test_internal_memory(pao, dsp as c_int);
        if err != 0 { return err; }
        err = boot_loader_test_external_memory(pao, dsp as c_int);
        if err != 0 { return err; }
        err = boot_loader_test_pld(pao, dsp as c_int);
        if err != 0 { return err; }

        err = hpi_dsp_code_open(boot_code_id[dsp], (*pao).pci.pci_dev, &mut dsp_code, pos_error_code);
        if err != 0 { return err; }
        loop {
            let mut length: u32 = 0;
            let mut address: u32 = 0;
            let mut type_: u32 = 0;
            let mut pcode: *mut u32 = null_mut();
            err = hpi_dsp_code_read_word(&mut dsp_code, &mut length);
            if err != 0 || length == 0xFFFFFFFF { break; }
            err = hpi_dsp_code_read_word(&mut dsp_code, &mut address);
            if err != 0 { break; }
            err = hpi_dsp_code_read_word(&mut dsp_code, &mut type_);
            if err != 0 { break; }
            err = hpi_dsp_code_read_block(length, &mut dsp_code, &mut pcode);
            if err != 0 { break; }
            for i in 0..length {
                boot_loader_write_mem32(pao, dsp as c_int, address, *pcode);
                if i % 4 == 0 {
                    boot_loader_read_mem32(pao, dsp as c_int, address);
                }
                pcode = pcode.add(1);
                address = address.wrapping_add(4);
            }
        }
        if err != 0 {
            hpi_dsp_code_close(&mut dsp_code);
            return err;
        }
        hpi_dsp_code_rewind(&mut dsp_code);
        loop {
            let mut length: u32 = 0;
            let mut address: u32 = 0;
            let mut type_: u32 = 0;
            let mut pcode: *mut u32 = null_mut();
            hpi_dsp_code_read_word(&mut dsp_code, &mut length);
            if length == 0xFFFFFFFF { break; }
            hpi_dsp_code_read_word(&mut dsp_code, &mut address);
            hpi_dsp_code_read_word(&mut dsp_code, &mut type_);
            hpi_dsp_code_read_block(length, &mut dsp_code, &mut pcode);
            for _ in 0..length {
                let data = boot_loader_read_mem32(pao, dsp as c_int, address);
                if data != *pcode {
                    err = 0;
                    break;
                }
                pcode = pcode.add(1);
                address = address.wrapping_add(4);
            }
            if err != 0 { break; }
        }
        hpi_dsp_code_close(&mut dsp_code);
        if err != 0 { return err; }
    }

    if !(*phw).p_interface_buffer.is_null() {
        let mut physicalPC_iaddress: u32 = 0;
        let interface = (*phw).p_interface_buffer;
        let host_mailbox_address_on_dsp: u32 = 0x80000000;
        let mut physicalPC_iaddress_verify: u32 = 0;
        let mut time_out: c_int = 10;
        (*interface).dsp_ack = H620_HIF_UNKNOWN;
        wmb();
        err = hpios_locked_mem_get_phys_addr(&mut (*phw).h_locked_mem, &mut physicalPC_iaddress);
        while physicalPC_iaddress != physicalPC_iaddress_verify && time_out != 0 {
            boot_loader_write_mem32(pao, 0, host_mailbox_address_on_dsp, physicalPC_iaddress);
            physicalPC_iaddress_verify = boot_loader_read_mem32(pao, 0, host_mailbox_address_on_dsp);
            time_out -= 1;
        }
    }
    HPI_DEBUG_LOG!(DEBUG, "starting DS_ps running\n");
    temp = ioread32((*phw).prHSR);
    temp &= !C6205_HSR_INTAM;
    iowrite32(temp, (*phw).prHSR);
    temp = ioread32((*phw).prHDCR);
    temp |= C6205_HDCR_DSPINT;
    iowrite32(temp, (*phw).prHDCR);
    hpios_delay_micro_seconds(10000);
    err
}

unsafe fn boot_loader_read_mem32(pao: *mut hpi_adapter_obj, dsp_index: c_int, mut address: u32) -> u32 {
    let phw = (*pao).priv_;
    let mut data: u32 = 0;
    if dsp_index == 0 {
        let p_data: *mut u32;
        if (address >= 0x01800000) & (address < 0x02000000) {
            p_data = (*pao).pci.ap_mem_base[1].add(((address & 0x007fffff) as usize) / size_of::<u32>());
        } else {
            let dw4M_page = address >> 22;
            if dw4M_page != (*phw).dsp_page {
                (*phw).dsp_page = dw4M_page;
                iowrite32((*phw).dsp_page, (*phw).prDSPP);
            }
            address &= 0x3fffff;
            p_data = (*pao).pci.ap_mem_base[0].add((address as usize) / size_of::<u32>());
        }
        data = ioread32(p_data);
    } else if dsp_index == 1 {
        boot_loader_write_mem32(pao, 0, HPIAL_ADDR, address);
        boot_loader_write_mem32(pao, 0, HPIAH_ADDR, address >> 16);
        let lsb = boot_loader_read_mem32(pao, 0, HPIDL_ADDR);
        data = boot_loader_read_mem32(pao, 0, HPIDH_ADDR);
        data = (data << 16) | (lsb & 0xFFFF);
    }
    data
}

unsafe fn boot_loader_write_mem32(pao: *mut hpi_adapter_obj, dsp_index: c_int, mut address: u32, data: u32) {
    let phw = (*pao).priv_;
    if dsp_index == 0 {
        let p_data: *mut u32;
        if (address >= 0x01800000) & (address < 0x02000000) {
            p_data = (*pao).pci.ap_mem_base[1].add(((address & 0x007fffff) as usize) / size_of::<u32>());
        } else {
            let dw4M_page = address >> 22;
            if dw4M_page != (*phw).dsp_page {
                (*phw).dsp_page = dw4M_page;
                iowrite32((*phw).dsp_page, (*phw).prDSPP);
            }
            address &= 0x3fffff;
            p_data = (*pao).pci.ap_mem_base[0].add((address as usize) / size_of::<u32>());
        }
        iowrite32(data, p_data);
    } else if dsp_index == 1 {
        boot_loader_write_mem32(pao, 0, HPIAL_ADDR, address);
        boot_loader_write_mem32(pao, 0, HPIAH_ADDR, address >> 16);
        boot_loader_read_mem32(pao, 0, 0);
        boot_loader_write_mem32(pao, 0, HPIDL_ADDR, data);
        boot_loader_write_mem32(pao, 0, HPIDH_ADDR, data >> 16);
        boot_loader_read_mem32(pao, 0, 0);
    }
}

unsafe fn boot_loader_config_emif(pao: *mut hpi_adapter_obj, dsp_index: c_int) -> u16 {
    if dsp_index == 0 {
        boot_loader_write_mem32(pao, dsp_index, 0x01800000, 0x3779);
        let mut setting: u32 = 0x00000030;
        boot_loader_write_mem32(pao, dsp_index, 0x01800008, setting);
        if setting != boot_loader_read_mem32(pao, dsp_index, 0x01800008) { return HPI6205_ERROR_DSP_EMIF1; }
        setting = (1 << WS_OFS) | (63 << WST_OFS) | (1 << WH_OFS) | (1 << RS_OFS) | (63 << RST_OFS) | (1 << RH_OFS) | (2 << MTYPE_OFS);
        boot_loader_write_mem32(pao, dsp_index, 0x01800004, setting);
        if setting != boot_loader_read_mem32(pao, dsp_index, 0x01800004) { return HPI6205_ERROR_DSP_EMIF2; }
        setting = (1 << WS_OFS) | (28 << WST_OFS) | (1 << WH_OFS) | (1 << RS_OFS) | (63 << RST_OFS) | (1 << RH_OFS) | (2 << MTYPE_OFS);
        boot_loader_write_mem32(pao, dsp_index, 0x01800010, setting);
        if setting != boot_loader_read_mem32(pao, dsp_index, 0x01800010) { return HPI6205_ERROR_DSP_EMIF3; }
        setting = (1 << WS_OFS) | (10 << WST_OFS) | (1 << WH_OFS) | (1 << RS_OFS) | (10 << RST_OFS) | (1 << RH_OFS) | (2 << MTYPE_OFS);
        boot_loader_write_mem32(pao, dsp_index, 0x01800014, setting);
        if setting != boot_loader_read_mem32(pao, dsp_index, 0x01800014) { return HPI6205_ERROR_DSP_EMIF4; }
        boot_loader_write_mem32(pao, dsp_index, 0x01800018, 0x07117000);
        boot_loader_write_mem32(pao, dsp_index, 0x0180001C, 0x00000410);
    } else if dsp_index == 1 {
        let mut write_data: u32 = 1;
        boot_loader_write_mem32(pao, 0, HPICL_ADDR, write_data);
        boot_loader_write_mem32(pao, 0, HPICH_ADDR, write_data);
        let mut read_data = 0xFFF7 & boot_loader_read_mem32(pao, 0, HPICL_ADDR);
        if write_data != read_data {
            HPI_DEBUG_LOG!(ERROR, "HPICL\n");
            return HPI6205_ERROR_C6713_HPIC;
        }
        write_data = 1;
        for _ in 0..32 {
            boot_loader_write_mem32(pao, 0, HPIAL_ADDR, write_data);
            boot_loader_write_mem32(pao, 0, HPIAH_ADDR, write_data >> 16);
            read_data = 0xFFFF & boot_loader_read_mem32(pao, 0, HPIAL_ADDR);
            read_data |= (0xFFFF & boot_loader_read_mem32(pao, 0, HPIAH_ADDR)) << 16;
            if read_data != write_data {
                HPI_DEBUG_LOG!(ERROR, "HPIA\n");
                return HPI6205_ERROR_C6713_HPIA;
            }
            write_data <<= 1;
        }
        boot_loader_write_mem32(pao, dsp_index, 0x01B7C100, 0x0000);
        hpios_delay_micro_seconds(1000);
        boot_loader_write_mem32(pao, dsp_index, 0x01B7C120, 0x8002);
        boot_loader_write_mem32(pao, dsp_index, 0x01B7C11C, 0x8001);
        boot_loader_write_mem32(pao, dsp_index, 0x01B7C118, 0x8000);
        hpios_delay_micro_seconds(1000);
        boot_loader_write_mem32(pao, 0, 0x018C0024, 0x00002A0A);
        boot_loader_write_mem32(pao, dsp_index, 0x01B7C100, 0x0001);
        hpios_delay_micro_seconds(1000);
        boot_loader_write_mem32(pao, 0, 0x018C0024, 0x00002A02);
        boot_loader_write_mem32(pao, 0, 0x01800004,
            (1 << WS_OFS) | (8 << WST_OFS) | (1 << WH_OFS) |
            (1 << RS_OFS) | (12 << RST_OFS) | (1 << RH_OFS) |
            (2 << MTYPE_OFS));
        hpios_delay_micro_seconds(1000);
        if (boot_loader_read_mem32(pao, dsp_index, 0x01B7C100) & 0xF) != 0x0001 {
            return HPI6205_ERROR_C6713_PLL;
        }
        boot_loader_write_mem32(pao, dsp_index, C6713_EMIF_GCTL, 0x000034A8);
        boot_loader_write_mem32(pao, dsp_index, C6713_EMIF_CE0, 0x00000030);
        boot_loader_write_mem32(pao, dsp_index, C6713_EMIF_SDRAMEXT, 0x001BDF29);
        boot_loader_write_mem32(pao, dsp_index, C6713_EMIF_SDRAMCTL, 0x47116000);
        boot_loader_write_mem32(pao, dsp_index, C6713_EMIF_SDRAMTIMING, 0x00000410);
        hpios_delay_micro_seconds(1000);
    } else if dsp_index == 2 {
        /* DSP 2 is a C6713 */
    }
    0
}

unsafe fn boot_loader_test_memory(pao: *mut hpi_adapter_obj, dsp_index: c_int, start_address: u32, mut length: u32) -> u16 {
    let mut test_addr: u32;
    let mut test_data: u32;
    let mut data: u32;
    length = 1000;
    test_addr = start_address;
    test_data = 0x00000001;
    for _ in 0..32 {
        boot_loader_write_mem32(pao, dsp_index, test_addr, test_data);
        data = boot_loader_read_mem32(pao, dsp_index, test_addr);
        if data != test_data {
            HPI_DEBUG_LOG!(VERBOSE, "Memtest error details\n");
            return 1;
        }
        test_data <<= 1;
    }
    for i in 0..100 {
        test_addr = start_address + i * 4;
        test_data = 0xA5A55A5A;
        boot_loader_write_mem32(pao, dsp_index, test_addr, test_data);
        boot_loader_write_mem32(pao, dsp_index, test_addr + 4, 0);
        data = boot_loader_read_mem32(pao, dsp_index, test_addr);
        if data != test_data {
            HPI_DEBUG_LOG!(VERBOSE, "Memtest error details\n");
            return 1;
        }
        boot_loader_write_mem32(pao, dsp_index, test_addr, 0);
    }
    for i in 0..length {
        test_addr = start_address + i * 4;
        boot_loader_write_mem32(pao, dsp_index, test_addr, 0);
    }
    0
}

unsafe fn boot_loader_test_internal_memory(pao: *mut hpi_adapter_obj, dsp_index: c_int) -> u16 {
    let mut err: u16 = 0;
    if dsp_index == 0 {
        err = boot_loader_test_memory(pao, dsp_index, 0x00000000, 0x10000);
        if err == 0 {
            err = boot_loader_test_memory(pao, dsp_index, 0x80000000, 0x10000);
        }
    } else if dsp_index == 1 {
        err = boot_loader_test_memory(pao, dsp_index, 0x00000000, 0x30000);
        if err == 0 {
            err = boot_loader_test_memory(pao, dsp_index, 0x00030000, 0x10000);
        }
    }
    if err != 0 { HPI6205_ERROR_DSP_INTMEM } else { 0 }
}

unsafe fn boot_loader_test_external_memory(pao: *mut hpi_adapter_obj, dsp_index: c_int) -> u16 {
    let mut dRAM_start_address: u32 = 0;
    let mut dRAM_size: u32 = 0;
    if dsp_index == 0 {
        if (*(*pao).pci.pci_dev).subsystem_device == 0x5000 {
            dRAM_start_address = 0x00400000;
            dRAM_size = 0x200000;
        } else {
            return 0;
        }
    } else if dsp_index == 1 {
        dRAM_start_address = 0x80000000;
        dRAM_size = 0x200000;
    }
    if boot_loader_test_memory(pao, dsp_index, dRAM_start_address, dRAM_size) != 0 {
        return HPI6205_ERROR_DSP_EXTMEM;
    }
    0
}

unsafe fn boot_loader_test_pld(pao: *mut hpi_adapter_obj, dsp_index: c_int) -> u16 {
    let mut data: u32;
    if dsp_index == 0 {
        if (*(*pao).pci.pci_dev).subsystem_device == 0x5000 {
            data = boot_loader_read_mem32(pao, dsp_index, 0x03000008);
            if (data & 0xF) != 0x5 { return HPI6205_ERROR_DSP_PLD; }
            data = boot_loader_read_mem32(pao, dsp_index, 0x0300000C);
            if (data & 0xF) != 0xA { return HPI6205_ERROR_DSP_PLD; }
        }
    } else if dsp_index == 1 {
        if (*(*pao).pci.pci_dev).subsystem_device == 0x8700 {
            data = boot_loader_read_mem32(pao, dsp_index, 0x90000010);
            if (data & 0xFF) != 0xAA { return HPI6205_ERROR_DSP_PLD; }
            boot_loader_write_mem32(pao, dsp_index, 0x90000000, 0x02);
        }
    }
    0
}

/** Transfer data to or from DSP
 nOperation = H620_H620_HIF_SEND_DATA or H620_HIF_GET_DATA
*/
unsafe fn hpi6205_transfer_data(pao: *mut hpi_adapter_obj, p_data: *mut u8, mut data_size: u32, operation: c_int) -> i16 {
    let phw = (*pao).priv_;
    let mut data_transferred: u32 = 0;
    let err: u16 = 0;
    let interface = (*phw).p_interface_buffer;
    if p_data.is_null() {
        return HPI_ERROR_INVALID_DATA_POINTER as i16;
    }
    data_size &= !3;
    if wait_dsp_ack(phw, H620_HIF_IDLE, HPI6205_TIMEOUT) == 0 {
        return HPI_ERROR_DSP_HARDWARE as i16;
    }
    while data_transferred < data_size {
        let mut this_copy = data_size - data_transferred;
        if this_copy > HPI6205_SIZEOF_DATA {
            this_copy = HPI6205_SIZEOF_DATA;
        }
        if operation == H620_HIF_SEND_DATA {
            copy_nonoverlapping(p_data.add(data_transferred as usize), (*interface).u.b_data.as_ptr() as *mut u8, this_copy as usize);
        }
        (*interface).transfer_size_in_bytes = this_copy;
        (*interface).dsp_ack = H620_HIF_IDLE;
        send_dsp_command(phw, operation);
        let temp2 = wait_dsp_ack(phw, operation, HPI6205_TIMEOUT);
        HPI_DEBUG_LOG!(DEBUG, "spun for data xfer\n");
        if temp2 == 0 {
            HPI_DEBUG_LOG!(ERROR, "Timed out waiting for state\n");
            break;
        }
        if operation == H620_HIF_GET_DATA {
            copy_nonoverlapping((*interface).u.b_data.as_ptr(), p_data.add(data_transferred as usize), this_copy as usize);
        }
        data_transferred += this_copy;
    }
    if (*interface).dsp_ack != operation {
        HPI_DEBUG_LOG!(DEBUG, "interface->dsp_ack unexpected\n");
    }
    send_dsp_command(phw, H620_HIF_IDLE);
    err as i16
}

/* wait for up to timeout_us microseconds for the DSP to signal state by DMA into dwDspAck */
unsafe fn wait_dsp_ack(phw: *mut hpi_hw_obj, state: c_int, timeout_us: c_int) -> c_int {
    let interface = (*phw).p_interface_buffer;
    let mut t = timeout_us / 4;
    rmb();
    while (*interface).dsp_ack != state && { t -= 1; t != 0 } {
        hpios_delay_micro_seconds(4);
        rmb();
    }
    t * 4
}

/* set the busmaster interface to cmd, then interrupt the DSP */
unsafe fn send_dsp_command(phw: *mut hpi_hw_obj, cmd: c_int) {
    let interface = (*phw).p_interface_buffer;
    (*interface).host_cmd = cmd;
    wmb();
    let mut r = ioread32((*phw).prHDCR);
    r |= C6205_HDCR_DSPINT;
    iowrite32(r, (*phw).prHDCR);
    r &= !C6205_HDCR_DSPINT;
    iowrite32(r, (*phw).prHDCR);
}

static mut message_count: c_uint = 0;

unsafe fn message_response_sequence(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) -> u16 {
    let mut time_out: u32;
    let time_out2: u32;
    let phw = (*pao).priv_;
    let interface = (*phw).p_interface_buffer;
    let mut err: u16;
    message_count = message_count.wrapping_add(1);
    if (*phm).size as usize > size_of_val_message_buffer(interface) {
        (*phr).error = HPI_ERROR_MESSAGE_BUFFER_TOO_SMALL;
        (*phr).specific_error = size_of_val_message_buffer(interface) as u16;
        (*phr).size = size_of::<hpi_response_header>() as u16;
        HPI_DEBUG_LOG!(ERROR, "message len too big\n");
        return 0;
    }
    if wait_dsp_ack(phw, H620_HIF_IDLE, HPI6205_TIMEOUT) == 0 {
        HPI_DEBUG_LOG!(DEBUG, "timeout waiting for idle\n");
        return HPI6205_ERROR_MSG_RESP_IDLE_TIMEOUT;
    }
    copy_nonoverlapping(phm as *const u8, &mut (*interface).u.message_buffer as *mut _ as *mut u8, (*phm).size as usize);
    send_dsp_command(phw, H620_HIF_GET_RESP);
    time_out2 = wait_dsp_ack(phw, H620_HIF_GET_RESP, HPI6205_TIMEOUT) as u32;
    if time_out2 == 0 {
        HPI_DEBUG_LOG!(ERROR, "Timed out waiting for GET_RESP\n");
    } else {
        HPI_DEBUG_LOG!(VERBOSE, "transition to GET_RESP\n");
    }
    time_out = HPI6205_TIMEOUT as u32;
    if time_out != 0 {
        if (*interface).u.response_buffer.size <= (*phr).size {
            copy_nonoverlapping(&(*interface).u.response_buffer as *const _ as *const u8, phr as *mut u8, (*interface).u.response_buffer.size as usize);
        } else {
            HPI_DEBUG_LOG!(ERROR, "response len too big\n");
            copy_nonoverlapping(&(*interface).u.response_buffer as *const _ as *const u8, phr as *mut u8, size_of::<hpi_response_header>());
            (*phr).error = HPI_ERROR_RESPONSE_BUFFER_TOO_SMALL;
            (*phr).specific_error = (*interface).u.response_buffer.size;
            (*phr).size = size_of::<hpi_response_header>() as u16;
        }
    }
    send_dsp_command(phw, H620_HIF_IDLE);
    if time_out == 0 || time_out2 == 0 {
        HPI_DEBUG_LOG!(DEBUG, "something timed out!\n");
        return HPI6205_ERROR_MSG_RESP_TIMEOUT;
    }
    if (*phm).function == HPI_ADAPTER_CLOSE {
        if wait_dsp_ack(phw, H620_HIF_IDLE, HPI6205_TIMEOUT) == 0 {
            HPI_DEBUG_LOG!(DEBUG, "Timeout waiting for idle (on adapter_close)\n");
            return HPI6205_ERROR_MSG_RESP_IDLE_TIMEOUT;
        }
    }
    err = hpi_validate_response(phm, phr);
    err
}

#[inline]
unsafe fn size_of_val_message_buffer(_interface: *mut bus_master_interface) -> usize {
    size_of::<hpi_message>()
}

unsafe fn hw_message(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let mut err: u16;
    hpios_dsplock_lock(pao);
    err = message_response_sequence(pao, phm, phr);
    if err != 0 {
        if err >= HPI_ERROR_BACKEND_BASE {
            (*phr).error = HPI_ERROR_DSP_COMMUNICATION;
            (*phr).specific_error = err;
        } else {
            (*phr).error = err;
        }
        (*pao).dsp_crashed = (*pao).dsp_crashed.wrapping_add(1);
        (*phr).size = size_of::<hpi_response_header>() as u16;
        hpios_dsplock_unlock(pao);
        return;
    } else {
        (*pao).dsp_crashed = 0;
    }
    if (*phr).error == 0 {
        match (*phm).function {
            f if f == HPI_OSTREAM_WRITE || f == HPI_ISTREAM_ANC_WRITE => {
                err = hpi6205_transfer_data(pao, (*phm).u.d.u.data.pb_data, (*phm).u.d.u.data.data_size, H620_HIF_SEND_DATA) as u16;
            }
            f if f == HPI_ISTREAM_READ || f == HPI_OSTREAM_ANC_READ => {
                err = hpi6205_transfer_data(pao, (*phm).u.d.u.data.pb_data, (*phm).u.d.u.data.data_size, H620_HIF_GET_DATA) as u16;
            }
            _ => {
                err = 0;
            }
        }
        (*phr).error = err;
    }
    hpios_dsplock_unlock(pao);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
