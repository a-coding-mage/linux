// SPDX-License-Identifier: GPL-2.0-only
/******************************************************************************

    AudioScience HPI driver
    Copyright (C) 1997-2011  AudioScience Inc. <support@audioscience.com>


 Hardware Programming Interface (HPI) for AudioScience ASI6200 series adapters.
 These PCI bus adapters are based on the TI C6711 DSP.

 Exported functions:
 void HPI_6000(struct hpi_message *phm, struct hpi_response *phr)

 #defines
 HIDE_PCI_ASSERTS to show the PCI asserts
 PROFILE_DSP2 get profile data from DSP2 if present (instead of DSP 1)

(C) Copyright AudioScience Inc. 1998-2003
*******************************************************************************/

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_short, c_uchar, c_uint, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null_mut};

const SOURCEFILE_NAME: &str = "hpi6000.c";

/* Includes in the original C source supplied these external definitions:
 * hpi_internal.h, hpimsginit.h, hpidebug.h, hpi6000.h, hpidspcd.h, hpicmn.h.
 */
type u16 = u16;
type u32 = u32;
type short = c_short;

const HPI_HIF_BASE: u32 = 0x0000_0200; /* start of C67xx internal RAM */
const HPI_HIF_ERROR_MASK: u32 = 0x4000;

/* Offsets of struct hpi_hif_6000 members. The C source used offsetof through
 * HPI_HIF_ADDR(member); these symbolic offsets are provided by the translated
 * dependency set.
 */
unsafe extern "C" {
    static HPI_HIF_OFF_control_cache_size_in_bytes: u32;
    static HPI_HIF_OFF_control_cache_count: u32;
    static HPI_HIF_OFF_host_cmd: u32;
    static HPI_HIF_OFF_dsp_number: u32;
    static HPI_HIF_OFF_adapter_info: u32;
    static HPI_HIF_OFF_message_buffer_address: u32;
    static HPI_HIF_OFF_response_buffer_address: u32;
    static HPI_HIF_OFF_length: u32;
    static HPI_HIF_OFF_address: u32;
    static HPI_HIF_OFF_dsp_ack: u32;
    static HPI_HIF_OFF_control_cache_is_dirty: u32;
    static HPI_HIF_OFF_control_cache_address: u32;
}
unsafe fn HPI_HIF_ADDR(offset: u32) -> u32 { HPI_HIF_BASE.wrapping_add(offset) }

/* HPI6000 specific error codes */
const HPI6000_ERROR_BASE: u16 = 900; /* not actually used anywhere */

/* operational/messaging errors */
const HPI6000_ERROR_MSG_RESP_IDLE_TIMEOUT: u16 = 901;
const HPI6000_ERROR_RESP_GET_LEN: u16 = 902;
const HPI6000_ERROR_MSG_RESP_GET_RESP_ACK: u16 = 903;
const HPI6000_ERROR_MSG_GET_ADR: u16 = 904;
const HPI6000_ERROR_RESP_GET_ADR: u16 = 905;
const HPI6000_ERROR_MSG_RESP_BLOCKWRITE32: u16 = 906;
const HPI6000_ERROR_MSG_RESP_BLOCKREAD32: u16 = 907;
const HPI6000_ERROR_CONTROL_CACHE_PARAMS: u16 = 909;
const HPI6000_ERROR_SEND_DATA_IDLE_TIMEOUT: u16 = 911;
const HPI6000_ERROR_SEND_DATA_ACK: u16 = 912;
const HPI6000_ERROR_SEND_DATA_ADR: u16 = 913;
const HPI6000_ERROR_SEND_DATA_TIMEOUT: u16 = 914;
const HPI6000_ERROR_SEND_DATA_CMD: u16 = 915;
const HPI6000_ERROR_SEND_DATA_WRITE: u16 = 916;
const HPI6000_ERROR_SEND_DATA_IDLECMD: u16 = 917;
const HPI6000_ERROR_GET_DATA_IDLE_TIMEOUT: u16 = 921;
const HPI6000_ERROR_GET_DATA_ACK: u16 = 922;
const HPI6000_ERROR_GET_DATA_CMD: u16 = 923;
const HPI6000_ERROR_GET_DATA_READ: u16 = 924;
const HPI6000_ERROR_GET_DATA_IDLECMD: u16 = 925;
const HPI6000_ERROR_CONTROL_CACHE_ADDRLEN: u16 = 951;
const HPI6000_ERROR_CONTROL_CACHE_READ: u16 = 952;
const HPI6000_ERROR_CONTROL_CACHE_FLUSH: u16 = 953;
const HPI6000_ERROR_MSG_RESP_GETRESPCMD: u16 = 961;
const HPI6000_ERROR_MSG_RESP_IDLECMD: u16 = 962;

/* Initialisation/bootload errors */
const HPI6000_ERROR_UNHANDLED_SUBSYS_ID: u16 = 930;
/* can't access PCI2040 */
const HPI6000_ERROR_INIT_PCI2040: u16 = 931;
/* can't access DSP HPI i/f */
const HPI6000_ERROR_INIT_DSPHPI: u16 = 932;
/* can't access internal DSP memory */
const HPI6000_ERROR_INIT_DSPINTMEM: u16 = 933;
/* can't access SDRAM - test#1 */
const HPI6000_ERROR_INIT_SDRAM1: u16 = 934;
/* can't access SDRAM - test#2 */
const HPI6000_ERROR_INIT_SDRAM2: u16 = 935;
const HPI6000_ERROR_INIT_VERIFY: u16 = 938;
const HPI6000_ERROR_INIT_NOACK: u16 = 939;
const HPI6000_ERROR_INIT_PLDTEST1: u16 = 941;
const HPI6000_ERROR_INIT_PLDTEST2: u16 = 942;

/* local defines */
/* HIDE_PCI_ASSERTS */
/* PROFILE_DSP2 */

/* for PCI2040 i/f chip */
/* HPI CSR registers */
/* word offsets from CSR base */
/* use when io addresses defined as u32 * */
const INTERRUPT_EVENT_SET: usize = 0;
const INTERRUPT_EVENT_CLEAR: usize = 1;
const INTERRUPT_MASK_SET: usize = 2;
const INTERRUPT_MASK_CLEAR: usize = 3;
const HPI_ERROR_REPORT: usize = 4;
const HPI_RESET: usize = 5;
const HPI_DATA_WIDTH: usize = 6;

const MAX_DSPS: usize = 2;
/* HPI registers, spaced 8K bytes = 2K words apart */
const DSP_SPACING: usize = 0x800;
const CONTROL: usize = 0x0000;
const ADDRESS: usize = 0x0200;
const DATA_AUTOINC: usize = 0x0400;
const DATA: usize = 0x0600;
const TIMEOUT: u32 = 500000;
const PCI_TIMEOUT: u32 = 100;
const H6READ: u16 = 1;
const H6WRITE: u16 = 0;

#[repr(C)]
pub struct dsp_obj {
    pub prHPI_control: *mut u32,
    pub prHPI_address: *mut u32,
    pub prHPI_data: *mut u32,
    pub prHPI_data_auto_inc: *mut u32,
    pub c_dsp_rev: c_char, /*A, B */
    pub control_cache_address_on_dsp: u32,
    pub control_cache_length_on_dsp: u32,
    pub pa_parent_adapter: *mut hpi_adapter_obj,
}

#[repr(C)]
pub struct hpi_hw_obj {
    pub dw2040_HPICSR: *mut u32,
    pub dw2040_HPIDSP: *mut u32,
    pub num_dsp: u16,
    pub ado: [dsp_obj; MAX_DSPS],
    pub message_buffer_address_on_dsp: u32,
    pub response_buffer_address_on_dsp: u32,
    pub pCI2040HPI_error_count: u32,
    pub control_cache: [hpi_control_cache_single; HPI_NMIXER_CONTROLS as usize],
    pub p_cache: *mut hpi_control_cache,
}

#[repr(C)] pub struct hpi_adapter_obj { pub priv_: *mut hpi_hw_obj, pub pci: hpi_pci, pub has_control_cache: u16, pub dsp_crashed: u16, pub type_: u16, pub index: u16 }
#[repr(C)] pub struct hpi_pci { pub ap_mem_base: [*mut u32; 2], pub pci_dev: *mut pci_dev }
#[repr(C)] pub struct pci_dev { pub subsystem_device: u16 }
#[repr(C)] pub struct hpi_control_cache_single { _private: [u8; 0] }
#[repr(C)] pub struct hpi_control_cache { pub adap_idx: u16 }
#[repr(C)] pub struct dsp_code { _private: [u8; 0] }

#[repr(C)] pub struct hpi_response_header { _private: [u8; 0] }
#[repr(C)] pub struct hpi_adapter_res { _private: [u8; 0] }

#[repr(C)] pub struct hpi_message { pub type_: u16, pub size: u16, pub object: u16, pub function: u16, pub adapter_index: u16, pub obj_index: u16, pub u: hpi_message_u }
#[repr(C)] pub union hpi_message_u { pub s: hpi_msg_subsys, pub d: hpi_msg_data_outer }
#[repr(C)] pub struct hpi_msg_subsys { pub resource: hpi_resource }
#[repr(C)] pub struct hpi_resource { pub r: hpi_resource_r }
#[repr(C)] pub struct hpi_resource_r { pub pci: *mut hpi_pci }
#[repr(C)] pub struct hpi_msg_data_outer { pub u: hpi_msg_data_union }
#[repr(C)] pub union hpi_msg_data_union { pub data: hpi_msg_data, pub stream: hpi_msg_stream }
#[repr(C)] pub struct hpi_msg_data { pub pb_data: *mut c_void, pub data_size: u32 }
#[repr(C)] pub struct hpi_msg_stream { pub stream_index: u16, pub object_type: u16 }

#[repr(C)] pub struct hpi_response { pub size: u16, pub error: u16, pub specific_error: u16, pub u: hpi_response_u }
#[repr(C)] pub union hpi_response_u { pub s: hpi_res_subsys, pub ax: hpi_res_adapter }
#[repr(C)] pub struct hpi_res_subsys { pub data: u32, pub adapter_type: u16, pub adapter_index: u16 }
#[repr(C)] pub struct hpi_res_adapter { pub info: hpi_adapter_info, pub assert: hpi_assert }
#[repr(C)] pub struct hpi_adapter_info { pub adapter_type: u16, pub adapter_index: u16 }
#[repr(C)] pub struct hpi_assert { pub p1: u32, pub p2: u32, pub count: u16, pub dsp_index: i16, pub sz_message: [c_char; 128], pub dsp_msg_addr: u32 }

unsafe extern "C" {
    static HPI_NMIXER_CONTROLS: u32;
    static HPI_SUBSYS_CREATE_ADAPTER: u16; static HPI_CONTROL_GET_STATE: u16; static HPI_CONTROL_SET_STATE: u16; static HPI_CONTROL_GET_INFO: u16;
    static HPI_ADAPTER_GET_ASSERT: u16; static HPI_ADAPTER_DELETE: u16; static HPI_ADAPTER_GET_INFO: u16;
    static HPI_OSTREAM_HOSTBUFFER_ALLOC: u16; static HPI_OSTREAM_HOSTBUFFER_FREE: u16; static HPI_ISTREAM_HOSTBUFFER_ALLOC: u16; static HPI_ISTREAM_HOSTBUFFER_FREE: u16;
    static HPI_TYPE_REQUEST: u16; static HPI_OBJ_SUBSYSTEM: u16; static HPI_OBJ_ADAPTER: u16; static HPI_OBJ_CONTROL: u16; static HPI_OBJ_OSTREAM: u16; static HPI_OBJ_ISTREAM: u16; static HPI_OBJ_PROFILE: u16;
    static HPI_ERROR_INVALID_FUNC: u16; static HPI_ERROR_BAD_ADAPTER_NUMBER: u16; static HPI_ERROR_DSP_HARDWARE: u16; static HPI_ERROR_PROCESSING_MESSAGE: u16; static HPI_ERROR_INVALID_TYPE: u16;
    static HPI_ERROR_MEMORY_ALLOC: u16; static HPI_ERROR_BACKEND_BASE: u16; static HPI_ERROR_DSP_BOOTLOAD: u16; static HPI_ERROR_BAD_ADAPTER: u16; static HPI_ERROR_CONTROL_CACHING: u16;
    static HPI_ERROR_RESPONSE_BUFFER_TOO_SMALL: u16; static HPI_ERROR_DSP_COMMUNICATION: u16; static HPI_ERROR_NO_INTERDSP_GROUPS: u16;
    static HPI_HIF_IDLE: u32; static HPI_HIF_GET_RESP: u32; static HPI_HIF_SEND_DATA: u32; static HPI_HIF_GET_DATA: u32;
    static HPI_ISTREAM_GROUP_ADD: u16; static HPI_OSTREAM_GROUP_ADD: u16; static HPI_OSTREAM_WRITE: u16; static HPI_ISTREAM_ANC_WRITE: u16; static HPI_ISTREAM_READ: u16; static HPI_OSTREAM_ANC_READ: u16;

    fn hpi_find_adapter(adapter_index: u16) -> *mut hpi_adapter_obj;
    fn hpi_init_response(phr: *mut hpi_response, object: u16, function: u16, error: u16);
    fn hpi_check_control_cache(cache: *mut hpi_control_cache, phm: *mut hpi_message, phr: *mut hpi_response) -> c_int;
    fn hpi_cmn_control_cache_sync_to_msg(cache: *mut hpi_control_cache, phm: *mut hpi_message, phr: *mut hpi_response);
    fn hpi_alloc_control_cache(count: u32, size: u32, base: *mut c_uchar) -> *mut hpi_control_cache;
    fn hpi_free_control_cache(cache: *mut hpi_control_cache);
    fn hpi_add_adapter(pao: *mut hpi_adapter_obj) -> short;
    fn hpi_delete_adapter(pao: *mut hpi_adapter_obj);
    fn hpi_validate_response(phm: *mut hpi_message, phr: *mut hpi_response) -> u16;
    fn hpios_delay_micro_seconds(usec: u32);
    fn hpios_dsplock_lock(pao: *mut hpi_adapter_obj);
    fn hpios_dsplock_unlock(pao: *mut hpi_adapter_obj);
    fn hpi_dsp_code_open(family: u16, dev: *mut pci_dev, code: *mut dsp_code, os_error: *mut u32) -> short;
    fn hpi_dsp_code_read_word(code: *mut dsp_code, data: *mut u32) -> short;
    fn hpi_dsp_code_read_block(length: u32, code: *mut dsp_code, pcode: *mut *mut u32) -> short;
    fn hpi_dsp_code_rewind(code: *mut dsp_code);
    fn hpi_dsp_code_close(code: *mut dsp_code);
    fn iowrite32(value: u32, addr: *mut u32);
    fn ioread32(addr: *mut u32) -> u32;
    fn iowrite32_rep(addr: *mut u32, data: *mut u32, count: u16);
    fn ioread32_rep(addr: *mut u32, data: *mut u32, count: u16);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn strscpy(dst: *mut c_char, src: *const c_char) -> isize;
    fn HPI_ADAPTER_FAMILY_ASI(adapter: u16) -> u16;
    fn HPI_HIF_ADAPTER_INFO_EXTRACT_ADAPTER(info: u32) -> u16;
}

macro_rules! HPI_DEBUG_LOG { ($($arg:tt)*) => {{ /* external debug macro */ }}; }
const DEBUG: c_int = 0; const VERBOSE: c_int = 0; const ERROR: c_int = 0; const INFO: c_int = 0;

/* local globals */
static mut gw_pci_read_asserts: u16 = 0; /* used to count PCI2040 errors */
static mut gw_pci_write_asserts: u16 = 0; /* used to count PCI2040 errors */

unsafe fn phw(pao: *mut hpi_adapter_obj) -> *mut hpi_hw_obj { (*pao).priv_ }

unsafe fn subsys_message(phm: *mut hpi_message, phr: *mut hpi_response) {
    match (*phm).function {
        f if f == HPI_SUBSYS_CREATE_ADAPTER => subsys_create_adapter(phm, phr),
        _ => (*phr).error = HPI_ERROR_INVALID_FUNC,
    }
}

unsafe fn control_message(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    let phw = phw(pao);
    match (*phm).function {
        f if f == HPI_CONTROL_GET_STATE => {
            if (*pao).has_control_cache != 0 {
                let err = hpi6000_update_control_cache(pao, phm);
                if err != 0 {
                    if err >= HPI_ERROR_BACKEND_BASE { (*phr).error = HPI_ERROR_CONTROL_CACHING; (*phr).specific_error = err; } else { (*phr).error = err; }
                    return;
                }
                if hpi_check_control_cache((*phw).p_cache, phm, phr) != 0 { return; }
            }
            hw_message(pao, phm, phr);
        }
        f if f == HPI_CONTROL_SET_STATE => { hw_message(pao, phm, phr); hpi_cmn_control_cache_sync_to_msg((*phw).p_cache, phm, phr); }
        _ => hw_message(pao, phm, phr),
    }
}

unsafe fn adapter_message(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    match (*phm).function {
        f if f == HPI_ADAPTER_GET_ASSERT => adapter_get_asserts(pao, phm, phr),
        f if f == HPI_ADAPTER_DELETE => adapter_delete(pao, phm, phr),
        _ => hw_message(pao, phm, phr),
    }
}

unsafe fn outstream_message(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    match (*phm).function {
        f if f == HPI_OSTREAM_HOSTBUFFER_ALLOC || f == HPI_OSTREAM_HOSTBUFFER_FREE => (*phr).error = HPI_ERROR_INVALID_FUNC,
        _ => hw_message(pao, phm, phr),
    }
}

unsafe fn instream_message(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    match (*phm).function {
        f if f == HPI_ISTREAM_HOSTBUFFER_ALLOC || f == HPI_ISTREAM_HOSTBUFFER_FREE => (*phr).error = HPI_ERROR_INVALID_FUNC,
        _ => hw_message(pao, phm, phr),
    }
}

/************************************************************************/
/** HPI_6000()
 * Entry point from HPIMAN
 * All calls to the HPI start here
 */
#[no_mangle]
pub unsafe extern "C" fn HPI_6000(phm: *mut hpi_message, phr: *mut hpi_response) {
    let mut pao: *mut hpi_adapter_obj = null_mut();
    if (*phm).object != HPI_OBJ_SUBSYSTEM {
        pao = hpi_find_adapter((*phm).adapter_index);
        if pao.is_null() {
            hpi_init_response(phr, (*phm).object, (*phm).function, HPI_ERROR_BAD_ADAPTER_NUMBER);
            HPI_DEBUG_LOG!(DEBUG, "invalid adapter index: %d \n", (*phm).adapter_index);
            return;
        }
        /* Don't even try to communicate with crashed DSP */
        if (*pao).dsp_crashed >= 10 {
            hpi_init_response(phr, (*phm).object, (*phm).function, HPI_ERROR_DSP_HARDWARE);
            HPI_DEBUG_LOG!(DEBUG, "adapter %d dsp crashed\n", (*phm).adapter_index);
            return;
        }
    }
    /* Init default response including the size field */
    if (*phm).function != HPI_SUBSYS_CREATE_ADAPTER { hpi_init_response(phr, (*phm).object, (*phm).function, HPI_ERROR_PROCESSING_MESSAGE); }
    match (*phm).type_ {
        t if t == HPI_TYPE_REQUEST => match (*phm).object {
            o if o == HPI_OBJ_SUBSYSTEM => subsys_message(phm, phr),
            o if o == HPI_OBJ_ADAPTER => { (*phr).size = (size_of::<hpi_response_header>() + size_of::<hpi_adapter_res>()) as u16; adapter_message(pao, phm, phr); }
            o if o == HPI_OBJ_CONTROL => control_message(pao, phm, phr),
            o if o == HPI_OBJ_OSTREAM => outstream_message(pao, phm, phr),
            o if o == HPI_OBJ_ISTREAM => instream_message(pao, phm, phr),
            _ => hw_message(pao, phm, phr),
        },
        _ => (*phr).error = HPI_ERROR_INVALID_TYPE,
    }
}

/************************************************************************/
/* SUBSYSTEM */

unsafe fn subsys_create_adapter(phm: *mut hpi_message, phr: *mut hpi_response) {
    /* create temp adapter obj, because we don't know what index yet */
    let mut ao: hpi_adapter_obj = zeroed();
    let mut os_error_code: u32 = 0;
    HPI_DEBUG_LOG!(VERBOSE, "subsys_create_adapter\n");
    ao.priv_ = kzalloc(size_of::<hpi_hw_obj>(), 0) as *mut hpi_hw_obj;
    if ao.priv_.is_null() { HPI_DEBUG_LOG!(ERROR, "can't get mem for adapter object\n"); (*phr).error = HPI_ERROR_MEMORY_ALLOC; return; }
    ao.pci = *(*phm).u.s.resource.r.pci;
    let err = create_adapter_obj(&mut ao, &mut os_error_code);
    if err != 0 {
        delete_adapter_obj(&mut ao);
        if err >= HPI_ERROR_BACKEND_BASE { (*phr).error = HPI_ERROR_DSP_BOOTLOAD; (*phr).specific_error = err; } else { (*phr).error = err; }
        (*phr).u.s.data = os_error_code;
        return;
    }
    /* need to update paParentAdapter */
    let pao = hpi_find_adapter(ao.index);
    if pao.is_null() { HPI_DEBUG_LOG!(ERROR, "lost adapter after boot\n"); (*phr).error = HPI_ERROR_BAD_ADAPTER; return; }
    for dsp_index in 0..MAX_DSPS { (*phw(pao)).ado[dsp_index].pa_parent_adapter = pao; }
    (*phr).u.s.adapter_type = ao.type_;
    (*phr).u.s.adapter_index = ao.index;
    (*phr).error = 0;
}

unsafe fn adapter_delete(pao: *mut hpi_adapter_obj, _phm: *mut hpi_message, phr: *mut hpi_response) { delete_adapter_obj(pao); hpi_delete_adapter(pao); (*phr).error = 0; }

/* this routine is called from SubSysFindAdapter and SubSysCreateAdapter */
unsafe fn create_adapter_obj(pao: *mut hpi_adapter_obj, pos_error_code: *mut u32) -> short {
    let mut boot_error: short;
    let mut control_cache_size: u32;
    let mut control_cache_count: u32;
    let phw = phw(pao);
    /* The PCI2040 has the following address map */
    /* BAR0 - 4K = HPI control and status registers on PCI2040 (HPI CSR) */
    /* BAR1 - 32K = HPI registers on DSP */
    (*phw).dw2040_HPICSR = (*pao).pci.ap_mem_base[0];
    (*phw).dw2040_HPIDSP = (*pao).pci.ap_mem_base[1];
    for dsp_index in 0..MAX_DSPS {
        (*phw).ado[dsp_index].prHPI_control = (*phw).dw2040_HPIDSP.add(CONTROL + DSP_SPACING * dsp_index);
        (*phw).ado[dsp_index].prHPI_address = (*phw).dw2040_HPIDSP.add(ADDRESS + DSP_SPACING * dsp_index);
        (*phw).ado[dsp_index].prHPI_data = (*phw).dw2040_HPIDSP.add(DATA + DSP_SPACING * dsp_index);
        (*phw).ado[dsp_index].prHPI_data_auto_inc = (*phw).dw2040_HPIDSP.add(DATA_AUTOINC + DSP_SPACING * dsp_index);
        (*phw).ado[dsp_index].pa_parent_adapter = pao;
    }
    (*phw).pCI2040HPI_error_count = 0;
    (*pao).has_control_cache = 0;
    (*phw).num_dsp = 1;
    boot_error = hpi6000_adapter_boot_load_dsp(pao, pos_error_code);
    if boot_error != 0 { return boot_error; }
    (*phw).message_buffer_address_on_dsp = 0;
    (*phw).response_buffer_address_on_dsp = 0;
    {
        let mut hm: hpi_message = zeroed();
        let mut hr0: hpi_response = zeroed();
        let mut hr1: hpi_response = zeroed();
        hm.type_ = HPI_TYPE_REQUEST; hm.size = size_of::<hpi_message>() as u16; hm.object = HPI_OBJ_ADAPTER; hm.function = HPI_ADAPTER_GET_INFO; hm.adapter_index = 0;
        hr0.size = size_of::<hpi_response>() as u16; hr1.size = size_of::<hpi_response>() as u16;
        let mut error = hpi6000_message_response_sequence(pao, 0, &mut hm, &mut hr0);
        if error != 0 { return error as short; }
        if hr0.error != 0 { return hr0.error as short; }
        if (*phw).num_dsp == 2 { error = hpi6000_message_response_sequence(pao, 1, &mut hm, &mut hr1); if error != 0 { return error as short; } }
        (*pao).type_ = hr0.u.ax.info.adapter_type; (*pao).index = hr0.u.ax.info.adapter_index;
    }
    core::ptr::write_bytes(addr_of_mut!((*phw).control_cache[0]), 0, HPI_NMIXER_CONTROLS as usize);
    control_cache_size = hpi_read_word(&mut (*phw).ado[0], HPI_HIF_ADDR(HPI_HIF_OFF_control_cache_size_in_bytes));
    if control_cache_size != 0 {
        control_cache_count = hpi_read_word(&mut (*phw).ado[0], HPI_HIF_ADDR(HPI_HIF_OFF_control_cache_count));
        (*phw).p_cache = hpi_alloc_control_cache(control_cache_count, control_cache_size, addr_of_mut!((*phw).control_cache[0]) as *mut c_uchar);
        if !(*phw).p_cache.is_null() { (*pao).has_control_cache = 1; }
    }
    if !(*phw).p_cache.is_null() { (*(*phw).p_cache).adap_idx = (*pao).index; }
    hpi_add_adapter(pao)
}

unsafe fn delete_adapter_obj(pao: *mut hpi_adapter_obj) {
    let phw = phw(pao);
    if (*pao).has_control_cache != 0 { hpi_free_control_cache((*phw).p_cache); }
    /* reset DSPs on adapter */
    iowrite32(0x0003_000F, (*phw).dw2040_HPICSR.add(HPI_RESET));
    kfree(phw as *mut c_void);
}

/************************************************************************/
/* ADAPTER */
unsafe fn adapter_get_asserts(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) {
    /* HIDE_PCI_ASSERTS is defined in this source, so the PCI2040 assert block
     * guarded by #ifndef HIDE_PCI_ASSERTS is intentionally not emitted.
     */
    hw_message(pao, phm, phr); /*get DSP asserts */
}

/************************************************************************/
/* LOW-LEVEL */
unsafe fn hpi6000_adapter_boot_load_dsp(pao: *mut hpi_adapter_obj, pos_error_code: *mut u32) -> short {
    let phw = phw(pao);
    let mut error: short = 0;
    let mut timeout: u32;
    let mut read: u32 = 0;
    let mut i: u32;
    let mut data: u32 = 0;
    let mut j: u32;
    let mut test_addr: u32;
    let mut test_data: u32;
    let mut dw2040_reset: u32;
    let mut adapter_info: u32 = 0;
    let mut delay: u32;
    let mut dsp_code: dsp_code = zeroed();
    let boot_load_family: u16;

    /* NOTE don't use wAdapterType in this routine. It is not setup yet */
    match (*(*pao).pci.pci_dev).subsystem_device {
        0x5100 | 0x5110 | 0x5200 | 0x6100 | 0x6200 => boot_load_family = HPI_ADAPTER_FAMILY_ASI(0x6200),
        _ => return HPI6000_ERROR_UNHANDLED_SUBSYS_ID as short,
    }
    dw2040_reset = 0x0003_000F;
    iowrite32(dw2040_reset, (*phw).dw2040_HPICSR.add(HPI_RESET));
    hpios_delay_micro_seconds(1000);
    delay = ioread32((*phw).dw2040_HPICSR.add(HPI_RESET));
    if delay != dw2040_reset { return HPI6000_ERROR_INIT_PCI2040 as short; }
    iowrite32(0x0000_0003, (*phw).dw2040_HPICSR.add(HPI_DATA_WIDTH));
    iowrite32(0x6000_0000, (*phw).dw2040_HPICSR.add(INTERRUPT_MASK_SET));
    dw2040_reset &= !(0u32 << 3);
    iowrite32(dw2040_reset, (*phw).dw2040_HPICSR.add(HPI_RESET));
    (*phw).ado[0].c_dsp_rev = b'B' as c_char; (*phw).ado[1].c_dsp_rev = b'B' as c_char;
    dw2040_reset &= !0x0000_0001; iowrite32(dw2040_reset, (*phw).dw2040_HPICSR.add(HPI_RESET));
    dw2040_reset &= !0x0000_0002; iowrite32(dw2040_reset, (*phw).dw2040_HPICSR.add(HPI_RESET));
    dw2040_reset &= !0x0000_0008; iowrite32(dw2040_reset, (*phw).dw2040_HPICSR.add(HPI_RESET));
    hpios_delay_micro_seconds(100);

    let mut dsp_index: u32 = 0;
    while dsp_index < (*phw).num_dsp as u32 {
        let pdo = &mut (*phw).ado[dsp_index as usize] as *mut dsp_obj;
        iowrite32(0x0001_0001, (*pdo).prHPI_control);
        test_data = 0x0000_0001;
        j = 0; while j < 32 { iowrite32(test_data, (*pdo).prHPI_address); data = ioread32((*pdo).prHPI_address); if data != test_data { return HPI6000_ERROR_INIT_DSPHPI as short; } test_data <<= 1; j += 1; }
        hpi_write_word(pdo, 0x01B7C100, 0x0000); hpios_delay_micro_seconds(100);
        hpi_write_word(pdo, 0x01B7C120, 0x8002); hpios_delay_micro_seconds(100);
        hpi_write_word(pdo, 0x01B7C11C, 0x8001); hpios_delay_micro_seconds(100);
        hpi_write_word(pdo, 0x01B7C118, 0x8000); hpios_delay_micro_seconds(2000);
        hpi_write_word(pdo, 0x01B7C100, 0x0001); hpios_delay_micro_seconds(2000);
        i = 0; while i < 100 { test_addr = 0; test_data = 1; j = 0; while j < 32 { hpi_write_word(pdo, test_addr.wrapping_add(i), test_data); data = hpi_read_word(pdo, test_addr.wrapping_add(i)); if data != test_data { return HPI6000_ERROR_INIT_DSPINTMEM as short; } test_data <<= 1; j += 1; } i += 1; }
        hpi_write_word(pdo, 0x0180_0000, 0x34A8);
        hpi_write_word(pdo, 0x0180_0008, 0x0000_0030);
        hpi_write_word(pdo, 0x0180_0020, 0x001B_DF29);
        hpi_write_word(pdo, 0x0180_0018, 0x4711_7000);
        hpi_write_word(pdo, 0x0180_001C, 0x0000_0410);
        { let cE1: u32 = (1u32 << 28) | (3u32 << 22) | (1u32 << 20) | (1u32 << 16) | (2u32 << 14) | (3u32 << 8) | (2u32 << 4) | 1u32; hpi_write_word(pdo, 0x0180_0004, cE1); }
        hpios_delay_micro_seconds(1000);
        test_addr = 0x8000_0000; test_data = 1; j = 0; while j < 32 { hpi_write_word(pdo, test_addr, test_data); data = hpi_read_word(pdo, test_addr); if data != test_data { return HPI6000_ERROR_INIT_SDRAM1 as short; } test_data <<= 1; j += 1; }
        const DRAM_SIZE_WORDS: u32 = 0x200000; const DRAM_INC: u32 = 1024;
        test_addr = 0x8000_0000; test_data = 0; i = 0; while i < DRAM_SIZE_WORDS { hpi_write_word(pdo, test_addr.wrapping_add(i), test_data); test_data = test_data.wrapping_add(1); i = i.wrapping_add(DRAM_INC); }
        test_addr = 0x8000_0000; test_data = 0; i = 0; while i < DRAM_SIZE_WORDS { data = hpi_read_word(pdo, test_addr.wrapping_add(i)); if data != test_data { return HPI6000_ERROR_INIT_SDRAM2 as short; } test_data = test_data.wrapping_add(1); i = i.wrapping_add(DRAM_INC); }
        error = hpi_dsp_code_open(boot_load_family, (*pao).pci.pci_dev, &mut dsp_code, pos_error_code); if error != 0 { return error; }
        loop { let mut length=0; let mut address=0; let mut typ=0; let mut pcode: *mut u32 = null_mut(); error = hpi_dsp_code_read_word(&mut dsp_code, &mut length); if error != 0 { break; } if length == 0xFFFF_FFFF { break; } error = hpi_dsp_code_read_word(&mut dsp_code, &mut address); if error != 0 { break; } error = hpi_dsp_code_read_word(&mut dsp_code, &mut typ); if error != 0 { break; } error = hpi_dsp_code_read_block(length, &mut dsp_code, &mut pcode); if error != 0 { break; } error = hpi6000_dsp_block_write32(pao, dsp_index as u16, address, pcode, length) as short; if error != 0 { break; } }
        if error != 0 { hpi_dsp_code_close(&mut dsp_code); return error; }
        hpi_dsp_code_rewind(&mut dsp_code);
        loop { let mut length=0; let mut address=0; let mut typ=0; let mut pcode: *mut u32 = null_mut(); hpi_dsp_code_read_word(&mut dsp_code, &mut length); if length == 0xFFFF_FFFF { break; } hpi_dsp_code_read_word(&mut dsp_code, &mut address); hpi_dsp_code_read_word(&mut dsp_code, &mut typ); hpi_dsp_code_read_block(length, &mut dsp_code, &mut pcode); i = 0; while i < length { data = hpi_read_word(pdo, address); if data != *pcode { error = HPI6000_ERROR_INIT_VERIFY as short; break; } pcode = pcode.add(1); address = address.wrapping_add(4); i += 1; } if error != 0 { break; } }
        hpi_dsp_code_close(&mut dsp_code); if error != 0 { return error; }
        { let mut address = HPI_HIF_ADDR(HPI_HIF_OFF_host_cmd); i = 0; while i < 4 { hpi_write_word(pdo, address, 0); address = address.wrapping_add(4); i += 1; } }
        hpi_write_word(pdo, HPI_HIF_ADDR(HPI_HIF_OFF_dsp_number), dsp_index);
        if dsp_index > 0 { hpi_write_word(pdo, HPI_HIF_ADDR(HPI_HIF_OFF_adapter_info), adapter_info); }
        iowrite32(0x0003_0003, (*pdo).prHPI_control); hpios_delay_micro_seconds(10000);
        timeout = 2000000;
        while timeout != 0 { loop { read = hpi_read_word(pdo, HPI_HIF_ADDR(HPI_HIF_OFF_host_cmd)); timeout = timeout.wrapping_sub(1); if !(timeout != 0 && hpi6000_check_PCI2040_error_flag(pao, H6READ) != 0) { break; } } if read != 0 { break; } else { hpios_delay_micro_seconds(10000); } }
        if timeout == 0 { return HPI6000_ERROR_INIT_NOACK as short; }
        if dsp_index == 0 {
            let mut mask: u32 = 0;
            adapter_info = hpi_read_word(pdo, HPI_HIF_ADDR(HPI_HIF_OFF_adapter_info));
            if HPI_ADAPTER_FAMILY_ASI(HPI_HIF_ADAPTER_INFO_EXTRACT_ADAPTER(adapter_info)) == HPI_ADAPTER_FAMILY_ASI(0x6200) { (*phw).num_dsp = 2; }
            const PLD_BASE_ADDRESS: u32 = 0x9000_0000;
            match boot_load_family { f if f == HPI_ADAPTER_FAMILY_ASI(0x6200) => { mask = 0xFFFF_FF00; if HPI_ADAPTER_FAMILY_ASI((*(*pao).pci.pci_dev).subsystem_device) == HPI_ADAPTER_FAMILY_ASI(0x5100) { mask = 0; } if HPI_ADAPTER_FAMILY_ASI((*(*pao).pci.pci_dev).subsystem_device) == HPI_ADAPTER_FAMILY_ASI(0x5200) { mask = 0; } } f if f == HPI_ADAPTER_FAMILY_ASI(0x8800) => mask = 0xFFFF_0000, _ => {} }
            test_data = 0xAAAA_AA00 & mask; hpi_write_word(pdo, PLD_BASE_ADDRESS + 4, test_data); read = hpi_read_word(pdo, PLD_BASE_ADDRESS + 4) & mask; if read != test_data { return HPI6000_ERROR_INIT_PLDTEST1 as short; }
            test_data = 0x5555_5500 & mask; hpi_write_word(pdo, PLD_BASE_ADDRESS + 4, test_data); read = hpi_read_word(pdo, PLD_BASE_ADDRESS + 4) & mask; if read != test_data { return HPI6000_ERROR_INIT_PLDTEST2 as short; }
        }
        dsp_index += 1;
    }
    0
}

unsafe fn hpi_set_address(pdo: *mut dsp_obj, address: u32) -> c_int { let mut timeout = PCI_TIMEOUT; loop { iowrite32(address, (*pdo).prHPI_address); if !(hpi6000_check_PCI2040_error_flag((*pdo).pa_parent_adapter, H6WRITE) != 0 && { timeout-=1; timeout != 0 }) { break; } } if timeout != 0 { 0 } else { 1 } }
/* write one word to the HPI port */
unsafe fn hpi_write_word(pdo: *mut dsp_obj, address: u32, data: u32) { if hpi_set_address(pdo, address) != 0 { return; } iowrite32(data, (*pdo).prHPI_data); }
/* read one word from the HPI port */
unsafe fn hpi_read_word(pdo: *mut dsp_obj, address: u32) -> u32 { if hpi_set_address(pdo, address) != 0 { return 0; } ioread32((*pdo).prHPI_data) }
/* write a block of 32bit words to the DSP HPI port using auto-inc mode */
unsafe fn hpi_write_block(pdo: *mut dsp_obj, address: u32, pdata: *mut u32, length: u32) { let length16 = length.wrapping_sub(1) as u16; if length == 0 { return; } if hpi_set_address(pdo, address) != 0 { return; } iowrite32_rep((*pdo).prHPI_data_auto_inc, pdata, length16); iowrite32(*pdata.add(length as usize - 1), (*pdo).prHPI_data); }
/** read a block of 32bit words from the DSP HPI port using auto-inc mode */
unsafe fn hpi_read_block(pdo: *mut dsp_obj, address: u32, pdata: *mut u32, length: u32) { let length16 = length.wrapping_sub(1) as u16; if length == 0 { return; } if hpi_set_address(pdo, address) != 0 { return; } ioread32_rep((*pdo).prHPI_data_auto_inc, pdata, length16); *pdata.add(length as usize - 1) = ioread32((*pdo).prHPI_data); }

unsafe fn hpi6000_dsp_block_write32(pao: *mut hpi_adapter_obj, dsp_index: u16, hpi_address: u32, source: *mut u32, count: u32) -> u16 { let pdo = &mut (*phw(pao)).ado[dsp_index as usize] as *mut dsp_obj; let mut time_out=PCI_TIMEOUT; let c6711_burst_size=128i32; let mut local_hpi_address=hpi_address; let mut local_count=count as i32; let mut pdata=source; while local_count != 0 { let xfer_size=if local_count>c6711_burst_size { c6711_burst_size } else { local_count }; time_out=PCI_TIMEOUT; loop { hpi_write_block(pdo, local_hpi_address, pdata, xfer_size as u32); if !(hpi6000_check_PCI2040_error_flag(pao,H6WRITE)!=0 && { time_out-=1; time_out!=0 }) { break; } } if time_out==0 { break; } pdata=pdata.add(xfer_size as usize); local_hpi_address=local_hpi_address.wrapping_add(size_of::<u32>() as u32 * xfer_size as u32); local_count-=xfer_size; } if time_out != 0 {0} else {1} }
unsafe fn hpi6000_dsp_block_read32(pao: *mut hpi_adapter_obj, dsp_index: u16, hpi_address: u32, dest: *mut u32, count: u32) -> u16 { let pdo = &mut (*phw(pao)).ado[dsp_index as usize] as *mut dsp_obj; let mut time_out=PCI_TIMEOUT; let c6711_burst_size=16i32; let mut local_hpi_address=hpi_address; let mut local_count=count as i32; let mut pdata=dest; while local_count != 0 { let xfer_size=if local_count>c6711_burst_size { c6711_burst_size } else { local_count }; time_out=PCI_TIMEOUT; loop { hpi_read_block(pdo, local_hpi_address, pdata, xfer_size as u32); if !(hpi6000_check_PCI2040_error_flag(pao,H6READ)!=0 && { time_out-=1; time_out!=0 }) { break; } } if time_out==0 { break; } pdata=pdata.add(xfer_size as usize); local_hpi_address=local_hpi_address.wrapping_add(size_of::<u32>() as u32 * xfer_size as u32); local_count-=xfer_size; } if time_out != 0 {0} else {1} }

unsafe fn hpi6000_message_response_sequence(pao: *mut hpi_adapter_obj, dsp_index: u16, phm: *mut hpi_message, phr: *mut hpi_response) -> u16 {
    let phw = phw(pao); let pdo = &mut (*phw).ado[dsp_index as usize] as *mut dsp_obj; let mut timeout: u32; let mut address: u32; let mut length: u32;
    let mut ack = hpi6000_wait_dsp_ack(pao, dsp_index, HPI_HIF_IDLE);
    if (ack as u32 & HPI_HIF_ERROR_MASK) != 0 { (*pao).dsp_crashed = (*pao).dsp_crashed.wrapping_add(1); return HPI6000_ERROR_MSG_RESP_IDLE_TIMEOUT; }
    (*pao).dsp_crashed = 0;
    if (*phw).message_buffer_address_on_dsp == 0 { timeout = TIMEOUT; loop { address = hpi_read_word(pdo, HPI_HIF_ADDR(HPI_HIF_OFF_message_buffer_address)); (*phw).message_buffer_address_on_dsp = address; if !(hpi6000_check_PCI2040_error_flag(pao,H6READ)!=0 && {timeout-=1; timeout!=0}) {break;} } if timeout==0 { return HPI6000_ERROR_MSG_GET_ADR; } } else { address = (*phw).message_buffer_address_on_dsp; }
    length = (*phm).size as u32;
    if hpi6000_dsp_block_write32(pao,dsp_index,address,phm as *mut u32,(length as u16/4) as u32) != 0 { return HPI6000_ERROR_MSG_RESP_BLOCKWRITE32; }
    if hpi6000_send_host_command(pao,dsp_index,HPI_HIF_GET_RESP) != 0 { return HPI6000_ERROR_MSG_RESP_GETRESPCMD; }
    hpi6000_send_dsp_interrupt(pdo);
    ack = hpi6000_wait_dsp_ack(pao,dsp_index,HPI_HIF_GET_RESP); if (ack as u32 & HPI_HIF_ERROR_MASK)!=0 { return HPI6000_ERROR_MSG_RESP_GET_RESP_ACK; }
    if (*phw).response_buffer_address_on_dsp == 0 { timeout=TIMEOUT; loop { address=hpi_read_word(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_response_buffer_address)); if !(hpi6000_check_PCI2040_error_flag(pao,H6READ)!=0 && {timeout-=1; timeout!=0}) {break;} } (*phw).response_buffer_address_on_dsp=address; if timeout==0 { return HPI6000_ERROR_RESP_GET_ADR; } } else { address=(*phw).response_buffer_address_on_dsp; }
    timeout=TIMEOUT; loop { length=hpi_read_word(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_length)); if !(hpi6000_check_PCI2040_error_flag(pao,H6READ)!=0 && {timeout-=1; timeout!=0}) {break;} } if timeout==0 { return HPI6000_ERROR_RESP_GET_LEN; }
    if length > (*phr).size as u32 { return HPI_ERROR_RESPONSE_BUFFER_TOO_SMALL; }
    if hpi6000_dsp_block_read32(pao,dsp_index,address,phr as *mut u32,(length as u16/4) as u32) != 0 { return HPI6000_ERROR_MSG_RESP_BLOCKREAD32; }
    if hpi6000_send_host_command(pao,dsp_index,HPI_HIF_IDLE) != 0 { return HPI6000_ERROR_MSG_RESP_IDLECMD; }
    hpi6000_send_dsp_interrupt(pdo);
    hpi_validate_response(phm, phr)
}

/* have to set up the below defines to match stuff in the MAP file */
const MSG_ADDRESS: u32 = HPI_HIF_BASE + 0x18; const MSG_LENGTH: u32 = 11; const RESP_ADDRESS: u32 = HPI_HIF_BASE + 0x44; const RESP_LENGTH: u32 = 16; const QUEUE_START: u32 = HPI_HIF_BASE + 0x88; const QUEUE_SIZE: u32 = 0x8000;
unsafe fn hpi6000_send_data_check_adr(_address: u32, _length_in_dwords: u32) -> short { /* CHECKING is not defined in the C source. */ 1 }

unsafe fn hpi6000_send_data(pao: *mut hpi_adapter_obj, dsp_index: u16, phm: *mut hpi_message, _phr: *mut hpi_response) -> short { let pdo=&mut (*phw(pao)).ado[dsp_index as usize] as *mut dsp_obj; let mut data_sent=0u32; let mut p_data=(*phm).u.d.u.data.pb_data as *mut u32; let mut time_out: u16=8; while data_sent < ((*phm).u.d.u.data.data_size & !3u32) && {time_out=time_out.wrapping_sub(1); time_out!=0} { let mut ack=hpi6000_wait_dsp_ack(pao,dsp_index,HPI_HIF_IDLE); if (ack as u32 & HPI_HIF_ERROR_MASK)!=0 {return HPI6000_ERROR_SEND_DATA_IDLE_TIMEOUT as short;} if hpi6000_send_host_command(pao,dsp_index,HPI_HIF_SEND_DATA)!=0 {return HPI6000_ERROR_SEND_DATA_CMD as short;} hpi6000_send_dsp_interrupt(pdo); ack=hpi6000_wait_dsp_ack(pao,dsp_index,HPI_HIF_SEND_DATA); if (ack as u32 & HPI_HIF_ERROR_MASK)!=0 {return HPI6000_ERROR_SEND_DATA_ACK as short;} let mut address; let length; loop { address=hpi_read_word(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_address)); length=hpi_read_word(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_length)); if hpi6000_check_PCI2040_error_flag(pao,H6READ)==0 {break;} } if hpi6000_send_data_check_adr(address,length)==0 {return HPI6000_ERROR_SEND_DATA_ADR as short;} let mut len=length; let mut blk_len=512u32; while len!=0 { if len<blk_len {blk_len=len;} if hpi6000_dsp_block_write32(pao,dsp_index,address,p_data,blk_len)!=0 {return HPI6000_ERROR_SEND_DATA_WRITE as short;} address=address.wrapping_add(blk_len*4); p_data=p_data.add(blk_len as usize); len-=blk_len; } if hpi6000_send_host_command(pao,dsp_index,HPI_HIF_IDLE)!=0 {return HPI6000_ERROR_SEND_DATA_IDLECMD as short;} hpi6000_send_dsp_interrupt(pdo); data_sent=data_sent.wrapping_add(length*4); } if time_out==0 {HPI6000_ERROR_SEND_DATA_TIMEOUT as short} else {0} }
unsafe fn hpi6000_get_data(pao: *mut hpi_adapter_obj, dsp_index: u16, phm: *mut hpi_message, _phr: *mut hpi_response) -> short { let pdo=&mut (*phw(pao)).ado[dsp_index as usize] as *mut dsp_obj; let mut data_got=0u32; let mut p_data=(*phm).u.d.u.data.pb_data as *mut u32; while data_got < ((*phm).u.d.u.data.data_size & !3u32) { let mut ack=hpi6000_wait_dsp_ack(pao,dsp_index,HPI_HIF_IDLE); if (ack as u32 & HPI_HIF_ERROR_MASK)!=0 {return HPI6000_ERROR_GET_DATA_IDLE_TIMEOUT as short;} if hpi6000_send_host_command(pao,dsp_index,HPI_HIF_GET_DATA)!=0 {return HPI6000_ERROR_GET_DATA_CMD as short;} hpi6000_send_dsp_interrupt(pdo); ack=hpi6000_wait_dsp_ack(pao,dsp_index,HPI_HIF_GET_DATA); if (ack as u32 & HPI_HIF_ERROR_MASK)!=0 {return HPI6000_ERROR_GET_DATA_ACK as short;} let mut address; let length; loop { address=hpi_read_word(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_address)); length=hpi_read_word(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_length)); if hpi6000_check_PCI2040_error_flag(pao,H6READ)==0 {break;} } let mut len=length; let mut blk_len=512u32; while len!=0 { if len<blk_len {blk_len=len;} if hpi6000_dsp_block_read32(pao,dsp_index,address,p_data,blk_len)!=0 {return HPI6000_ERROR_GET_DATA_READ as short;} address=address.wrapping_add(blk_len*4); p_data=p_data.add(blk_len as usize); len-=blk_len; } if hpi6000_send_host_command(pao,dsp_index,HPI_HIF_IDLE)!=0 {return HPI6000_ERROR_GET_DATA_IDLECMD as short;} hpi6000_send_dsp_interrupt(pdo); data_got=data_got.wrapping_add(length*4); } 0 }

unsafe fn hpi6000_send_dsp_interrupt(pdo: *mut dsp_obj) { iowrite32(0x0003_0003, (*pdo).prHPI_control); /* DSPINT */ }
unsafe fn hpi6000_send_host_command(pao: *mut hpi_adapter_obj, dsp_index: u16, host_cmd: u32) -> short { let pdo=&mut (*phw(pao)).ado[dsp_index as usize] as *mut dsp_obj; let mut timeout=TIMEOUT; loop { hpi_write_word(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_host_cmd),host_cmd); hpi_set_address(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_host_cmd)); if !(hpi6000_check_PCI2040_error_flag(pao,H6WRITE)!=0 && {timeout-=1; timeout!=0}) {break;} } iowrite32(0x0004_0004,(*pdo).prHPI_control); if timeout!=0 {0} else {1} }
/* if the PCI2040 has recorded an HPI timeout, reset the error and return 1 */
unsafe fn hpi6000_check_PCI2040_error_flag(pao: *mut hpi_adapter_obj, read_or_write: u16) -> short { let phw=phw(pao); let hPI_error=ioread32((*phw).dw2040_HPICSR.add(HPI_ERROR_REPORT)); if hPI_error != 0 { iowrite32(0,(*phw).dw2040_HPICSR.add(HPI_ERROR_REPORT)); (*phw).pCI2040HPI_error_count=(*phw).pCI2040HPI_error_count.wrapping_add(1); if read_or_write==1 { gw_pci_read_asserts=gw_pci_read_asserts.wrapping_add(1); } else { gw_pci_write_asserts=gw_pci_write_asserts.wrapping_add(1); } 1 } else {0} }
unsafe fn hpi6000_wait_dsp_ack(pao: *mut hpi_adapter_obj, dsp_index: u16, ack_value: u32) -> short { let pdo=&mut (*phw(pao)).ado[dsp_index as usize] as *mut dsp_obj; let mut ack=0u32; let mut timeout=TIMEOUT; while {timeout-=1; timeout!=0} { let hPIC=ioread32((*pdo).prHPI_control); if (hPIC & 0x04)!=0 {break;} } if timeout==0 {return HPI_HIF_ERROR_MASK as short;} timeout=TIMEOUT; while {timeout-=1; timeout!=0} { ack=hpi_read_word(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_dsp_ack)); if ack==ack_value {break;} if (ack & HPI_HIF_ERROR_MASK)!=0 && hpi6000_check_PCI2040_error_flag(pao,H6READ)==0 {break;} } if (ack & HPI_HIF_ERROR_MASK)!=0 {ack=HPI_HIF_ERROR_MASK;} if timeout==0 {ack=HPI_HIF_ERROR_MASK;} ack as short }

unsafe fn hpi6000_update_control_cache(pao: *mut hpi_adapter_obj, _phm: *mut hpi_message) -> short { const dsp_index: u16=0; let phw=phw(pao); let pdo=&mut (*phw).ado[dsp_index as usize] as *mut dsp_obj; let mut timeout=TIMEOUT; hpios_dsplock_lock(pao); let mut cache_dirty_flag; loop { cache_dirty_flag=hpi_read_word(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_control_cache_is_dirty)); if !(hpi6000_check_PCI2040_error_flag(pao,H6READ)!=0 && {timeout-=1; timeout!=0}) {break;} } if timeout==0 { hpios_dsplock_unlock(pao); return HPI6000_ERROR_CONTROL_CACHE_PARAMS as short; } if cache_dirty_flag!=0 { timeout=TIMEOUT; let address; let length; if (*pdo).control_cache_address_on_dsp==0 { loop { address=hpi_read_word(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_control_cache_address)); length=hpi_read_word(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_control_cache_size_in_bytes)); if !(hpi6000_check_PCI2040_error_flag(pao,H6READ)!=0 && {timeout-=1; timeout!=0}) {break;} } if timeout==0 { hpios_dsplock_unlock(pao); return HPI6000_ERROR_CONTROL_CACHE_ADDRLEN as short; } (*pdo).control_cache_address_on_dsp=address; (*pdo).control_cache_length_on_dsp=length; } else { address=(*pdo).control_cache_address_on_dsp; length=(*pdo).control_cache_length_on_dsp; } if hpi6000_dsp_block_read32(pao,dsp_index,address,addr_of_mut!((*phw).control_cache[0]) as *mut u32,length/size_of::<u32>() as u32)!=0 { hpios_dsplock_unlock(pao); return HPI6000_ERROR_CONTROL_CACHE_READ as short; } loop { hpi_write_word(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_control_cache_is_dirty),0); hpi_set_address(pdo,HPI_HIF_ADDR(HPI_HIF_OFF_host_cmd)); if !(hpi6000_check_PCI2040_error_flag(pao,H6WRITE)!=0 && {timeout-=1; timeout!=0}) {break;} } if timeout==0 { hpios_dsplock_unlock(pao); return HPI6000_ERROR_CONTROL_CACHE_FLUSH as short; } } hpios_dsplock_unlock(pao); 0 }

/** Get dsp index for multi DSP adapters only */
unsafe fn get_dsp_index(_pao: *mut hpi_adapter_obj, phm: *mut hpi_message) -> u16 { let mut ret=0; match (*phm).object { o if o==HPI_OBJ_ISTREAM => { if (*phm).obj_index < 2 { ret=1; } } o if o==HPI_OBJ_PROFILE => ret=(*phm).obj_index, _=>{} } ret }
/** Complete transaction with DSP

Send message, get response, send or get stream data if any.
*/
unsafe fn hw_message(pao: *mut hpi_adapter_obj, phm: *mut hpi_message, phr: *mut hpi_response) { let phw=phw(pao); let num_dsp=(*phw).num_dsp; let mut dsp_index=0u16; if num_dsp < 2 { dsp_index=0; } else { dsp_index=get_dsp_index(pao,phm); if (*phm).function==HPI_ISTREAM_GROUP_ADD || (*phm).function==HPI_OSTREAM_GROUP_ADD { let mut hm: hpi_message=zeroed(); hm.obj_index=(*phm).u.d.u.stream.stream_index; hm.object=(*phm).u.d.u.stream.object_type; let add_index=get_dsp_index(pao,&mut hm); if add_index != dsp_index { (*phr).error=HPI_ERROR_NO_INTERDSP_GROUPS; return; } } } hpios_dsplock_lock(pao); let mut error=hpi6000_message_response_sequence(pao,dsp_index,phm,phr); if error==0 && (*phr).error==0 { match (*phm).function { f if f==HPI_OSTREAM_WRITE || f==HPI_ISTREAM_ANC_WRITE => error=hpi6000_send_data(pao,dsp_index,phm,phr) as u16, f if f==HPI_ISTREAM_READ || f==HPI_OSTREAM_ANC_READ => error=hpi6000_get_data(pao,dsp_index,phm,phr) as u16, f if f==HPI_ADAPTER_GET_ASSERT => { (*phr).u.ax.assert.dsp_index=0; if num_dsp==2 && (*phr).u.ax.assert.count==0 { error=hpi6000_message_response_sequence(pao,1,phm,phr); (*phr).u.ax.assert.dsp_index=1; } } _=>{} } } if error!=0 { if error>=HPI_ERROR_BACKEND_BASE { (*phr).error=HPI_ERROR_DSP_COMMUNICATION; (*phr).specific_error=error; } else { (*phr).error=error; } (*phr).size=size_of::<hpi_response_header>() as u16; } hpios_dsplock_unlock(pao); }

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
