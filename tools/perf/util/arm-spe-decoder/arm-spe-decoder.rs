// SPDX-License-Identifier: GPL-2.0
/*
 * arm_spe_decoder.c: ARM SPE support
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_int, c_void};
use core::mem;
use core::ptr;

type u64 = u64;
type u32 = u32;

const EBADMSG: c_int = 74;

/*
 * Constants and packet helper macros are supplied by the translated
 * arm-spe-decoder dependencies.
 */
extern "C" {
    static SPE_ADDR_PKT_HDR_INDEX_INS: c_int;
    static SPE_ADDR_PKT_HDR_INDEX_BRANCH: c_int;
    static SPE_ADDR_PKT_HDR_INDEX_PREV_BRANCH: c_int;
    static SPE_ADDR_PKT_HDR_INDEX_DATA_VIRT: c_int;
    static SPE_ADDR_PKT_HDR_INDEX_DATA_PHYS: c_int;
    static SPE_ADDR_PKT_EL1: u64;
    static SPE_ADDR_PKT_EL2: u64;
    static SPE_ADDR_PKT_ADDR_BYTE7_SHIFT: c_int;

    static ARM_SPE_PAD: c_int;
    static ARM_SPE_TIMESTAMP: c_int;
    static ARM_SPE_END: c_int;
    static ARM_SPE_ADDRESS: c_int;
    static ARM_SPE_COUNTER: c_int;
    static ARM_SPE_CONTEXT: c_int;
    static ARM_SPE_OP_TYPE: c_int;
    static ARM_SPE_EVENTS: c_int;
    static ARM_SPE_DATA_SOURCE: c_int;
    static ARM_SPE_BAD: c_int;

    static SPE_CNT_PKT_HDR_INDEX_TOTAL_LAT: c_int;
    static SPE_OP_PKT_HDR_CLASS_LD_ST_ATOMIC: c_int;
    static SPE_OP_PKT_HDR_CLASS_OTHER: c_int;
    static SPE_OP_PKT_HDR_CLASS_BR_ERET: c_int;

    static ARM_SPE_OP_LDST: u64;
    static ARM_SPE_OP_ST: u64;
    static ARM_SPE_OP_LD: u64;
    static ARM_SPE_OP_GP_REG: u64;
    static ARM_SPE_OP_SIMD_FP: u64;
    static ARM_SPE_OP_UNSPEC_REG: u64;
    static ARM_SPE_OP_NV_SYSREG: u64;
    static ARM_SPE_OP_MTE_TAG: u64;
    static ARM_SPE_OP_AR: u64;
    static ARM_SPE_OP_EXCL: u64;
    static ARM_SPE_OP_ATOMIC: u64;
    static ARM_SPE_OP_SVE: u64;
    static ARM_SPE_OP_PRED: u64;
    static ARM_SPE_OP_SG: u64;
    static ARM_SPE_OP_MEMCPY: u64;
    static ARM_SPE_OP_MEMSET: u64;
    static ARM_SPE_OP_GCS: u64;
    static ARM_SPE_OP_COMM: u64;
    static ARM_SPE_OP_OTHER: u64;
    static ARM_SPE_OP_DP: u64;
    static ARM_SPE_OP_FP: u64;
    static ARM_SPE_OP_SME: u64;
    static ARM_SPE_OP_ASE: u64;
    static ARM_SPE_OP_COND: u64;
    static ARM_SPE_OP_BRANCH_ERET: u64;
    static ARM_SPE_OP_BR_COND: u64;
    static ARM_SPE_OP_BR_INDIRECT: u64;
    static ARM_SPE_OP_BR_GCS: u64;
    static ARM_SPE_OP_BR_CR_BL: u64;
    static ARM_SPE_OP_BR_CR_RET: u64;
    static ARM_SPE_OP_BR_CR_NON_BL_RET: u64;

    static SPE_OP_PKT_ST: u64;
    static SPE_OP_PKT_AR: u64;
    static SPE_OP_PKT_EXCL: u64;
    static SPE_OP_PKT_AT: u64;
    static SPE_OP_PKT_SVE_PRED: u64;
    static SPE_OP_PKT_SVE_SG: u64;
    static SPE_OP_PKT_GCS_COMM: u64;
    static SPE_OP_PKT_OTHER_FP: u64;
    static SPE_OP_PKT_OTHER_ASE: u64;
    static SPE_OP_PKT_COND: u64;
    static SPE_OP_PKT_INDIRECT_BRANCH: u64;
    static SPE_OP_PKT_GCS: u64;

    fn SPE_ADDR_PKT_GET_NS(payload: u64) -> u64;
    fn SPE_ADDR_PKT_GET_EL(payload: u64) -> u64;
    fn SPE_ADDR_PKT_ADDR_GET_BYTES_0_6(payload: u64) -> u64;
    fn SPE_ADDR_PKT_ADDR_GET_BYTE_6(payload: u64) -> u64;

    fn SPE_OP_PKT_LDST_SUBCLASS_GP_REG(payload: u64) -> bool;
    fn SPE_OP_PKT_LDST_SUBCLASS_SIMD_FP(payload: u64) -> bool;
    fn SPE_OP_PKT_LDST_SUBCLASS_UNSPEC_REG(payload: u64) -> bool;
    fn SPE_OP_PKT_LDST_SUBCLASS_NV_SYSREG(payload: u64) -> bool;
    fn SPE_OP_PKT_LDST_SUBCLASS_MTE_TAG(payload: u64) -> bool;
    fn SPE_OP_PKT_LDST_SUBCLASS_EXTENDED(payload: u64) -> bool;
    fn SPE_OP_PKT_LDST_SUBCLASS_SVE_SME_REG(payload: u64) -> bool;
    fn SPE_OP_PKT_LDST_SUBCLASS_MEMCPY(payload: u64) -> bool;
    fn SPE_OP_PKT_LDST_SUBCLASS_MEMSET(payload: u64) -> bool;
    fn SPE_OP_PKT_LDST_SUBCLASS_GCS(payload: u64) -> bool;
    fn SPE_OP_PKT_OTHER_SUBCLASS_SVE(payload: u64) -> bool;
    fn SPE_OP_PKT_OTHER_SUBCLASS_SME(payload: u64) -> bool;
    fn SPE_OP_PKT_OTHER_SUBCLASS_OTHER(payload: u64) -> bool;
    fn SPE_OP_PKT_CR_BL(payload: u64) -> bool;
    fn SPE_OP_PKT_CR_RET(payload: u64) -> bool;
    fn SPE_OP_PKT_CR_NON_BL_RET(payload: u64) -> bool;

    fn arm_spe_get_packet(
        buf: *const u8,
        len: usize,
        packet: *mut arm_spe_pkt,
        midr: u64,
    ) -> c_int;
    fn pr_warning(fmt: *const u8, ...) -> c_int;
    fn pr_debug(fmt: *const u8, ...) -> c_int;
    fn pr_err(fmt: *const u8, ...) -> c_int;
    fn zalloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

#[repr(C)]
pub struct arm_spe_buffer {
    pub buf: *const u8,
    pub len: usize,
}

#[repr(C)]
pub struct arm_spe_pkt {
    pub type_: c_int,
    pub index: c_int,
    pub payload: u64,
}

#[repr(C)]
pub struct arm_spe_record {
    pub timestamp: u64,
    pub context_id: u64,
    pub from_ip: u64,
    pub to_ip: u64,
    pub virt_addr: u64,
    pub phys_addr: u64,
    pub prev_br_tgt: u64,
    pub latency: u64,
    pub op: u64,
    pub type_: u64,
    pub source: u64,
}

pub type arm_spe_get_trace_fn =
    Option<unsafe extern "C" fn(buffer: *mut arm_spe_buffer, data: *mut c_void) -> c_int>;

#[repr(C)]
pub struct arm_spe_params {
    pub get_trace: arm_spe_get_trace_fn,
    pub data: *mut c_void,
}

#[repr(C)]
pub struct arm_spe_decoder {
    pub get_trace: arm_spe_get_trace_fn,
    pub data: *mut c_void,
    pub buf: *const u8,
    pub len: usize,
    pub midr: u64,
    pub record: arm_spe_record,
}

fn BIT(index: c_int) -> u32 {
    1u32 << index
}

unsafe fn arm_spe_calc_ip(index: c_int, mut payload: u64) -> u64 {
    let ns: u64;
    let el: u64;
    let val: u64;

    /* Instruction virtual address or Branch target address */
    if index == SPE_ADDR_PKT_HDR_INDEX_INS
        || index == SPE_ADDR_PKT_HDR_INDEX_BRANCH
        || index == SPE_ADDR_PKT_HDR_INDEX_PREV_BRANCH
    {
        ns = SPE_ADDR_PKT_GET_NS(payload);
        el = SPE_ADDR_PKT_GET_EL(payload);

        /* Clean highest byte */
        payload = SPE_ADDR_PKT_ADDR_GET_BYTES_0_6(payload);

        /* Fill highest byte for EL1 or EL2 (VHE) mode */
        if ns != 0 && (el == SPE_ADDR_PKT_EL1 || el == SPE_ADDR_PKT_EL2) {
            payload |= 0xffu64 << SPE_ADDR_PKT_ADDR_BYTE7_SHIFT;
        }

    /* Data access virtual address */
    } else if index == SPE_ADDR_PKT_HDR_INDEX_DATA_VIRT {
        /* Clean tags */
        payload = SPE_ADDR_PKT_ADDR_GET_BYTES_0_6(payload);

        /*
         * Armv8 ARM (ARM DDI 0487F.c), chapter "D10.2.1 Address packet"
         * defines the data virtual address payload format, the top byte
         * (bits [63:56]) is assigned as top-byte tag; so we only can
         * retrieve address value from bits [55:0].
         *
         * According to Documentation/arch/arm64/memory.rst, if detects the
         * specific pattern in bits [55:52] of payload which falls in
         * the kernel space, should fixup the top byte and this allows
         * perf tool to parse DSO symbol for data address correctly.
         *
         * For this reason, if detects the bits [55:52] is 0xf, will
         * fill 0xff into the top byte.
         */
        val = SPE_ADDR_PKT_ADDR_GET_BYTE_6(payload);
        if (val & 0xf0u64) == 0xf0u64 {
            payload |= 0xffu64 << SPE_ADDR_PKT_ADDR_BYTE7_SHIFT;
        }

    /* Data access physical address */
    } else if index == SPE_ADDR_PKT_HDR_INDEX_DATA_PHYS {
        /* Clean highest byte */
        payload = SPE_ADDR_PKT_ADDR_GET_BYTES_0_6(payload);
    } else {
        static mut seen_idx: u32 = 0;
        if (seen_idx & BIT(index)) == 0 {
            seen_idx |= BIT(index);
            pr_warning(
                b"ignoring unsupported address packet index: 0x%x\n\0".as_ptr(),
                index,
            );
        }
    }

    payload
}

#[no_mangle]
pub unsafe extern "C" fn arm_spe_decoder_new(
    params: *mut arm_spe_params,
) -> *mut arm_spe_decoder {
    let decoder: *mut arm_spe_decoder;

    if (*params).get_trace.is_none() {
        return ptr::null_mut();
    }

    decoder = zalloc(mem::size_of::<arm_spe_decoder>()) as *mut arm_spe_decoder;
    if decoder.is_null() {
        return ptr::null_mut();
    }

    (*decoder).get_trace = (*params).get_trace;
    (*decoder).data = (*params).data;

    decoder
}

#[no_mangle]
pub unsafe extern "C" fn arm_spe_decoder_free(decoder: *mut arm_spe_decoder) {
    free(decoder as *mut c_void);
}

unsafe fn arm_spe_get_data(decoder: *mut arm_spe_decoder) -> c_int {
    let mut buffer = arm_spe_buffer {
        buf: ptr::null(),
        len: 0,
    };
    let ret: c_int;

    pr_debug(b"Getting more data\n\0".as_ptr());
    ret = ((*decoder).get_trace.unwrap())(&mut buffer, (*decoder).data);
    if ret < 0 {
        return ret;
    }

    (*decoder).buf = buffer.buf;
    (*decoder).len = buffer.len;

    if (*decoder).len == 0 {
        pr_debug(b"No more data\n\0".as_ptr());
    }

    (*decoder).len as c_int
}

unsafe fn arm_spe_get_next_packet(
    decoder: *mut arm_spe_decoder,
    packet: *mut arm_spe_pkt,
) -> c_int {
    let mut ret: c_int;

    loop {
        if (*decoder).len == 0 {
            ret = arm_spe_get_data(decoder);

            /* Failed to read out trace data */
            if ret <= 0 {
                return ret;
            }
        }

        ret = arm_spe_get_packet((*decoder).buf, (*decoder).len, packet, (*decoder).midr);

        if ret <= 0 {
            /* Move forward for 1 byte */
            (*decoder).buf = (*decoder).buf.add(1);
            (*decoder).len -= 1;
            return -EBADMSG;
        }

        (*decoder).buf = (*decoder).buf.add(ret as usize);
        (*decoder).len -= ret as usize;

        if (*packet).type_ != ARM_SPE_PAD {
            break;
        }
    }

    1
}

unsafe fn arm_spe_read_record(decoder: *mut arm_spe_decoder) -> c_int {
    let mut err: c_int;
    let mut idx: c_int;
    let mut payload: u64;
    let ip: u64;
    let mut packet: arm_spe_pkt = mem::zeroed();

    ptr::write_bytes(
        &mut (*decoder).record as *mut arm_spe_record as *mut u8,
        0x0,
        mem::size_of::<arm_spe_record>(),
    );
    (*decoder).record.context_id = -1i64 as u64;

    loop {
        err = arm_spe_get_next_packet(decoder, &mut packet);
        if err <= 0 {
            return err;
        }

        idx = packet.index;
        payload = packet.payload;

        if packet.type_ == ARM_SPE_TIMESTAMP {
            (*decoder).record.timestamp = payload;
            return 1;
        } else if packet.type_ == ARM_SPE_END {
            return 1;
        } else if packet.type_ == ARM_SPE_ADDRESS {
            ip = arm_spe_calc_ip(idx, payload);
            if idx == SPE_ADDR_PKT_HDR_INDEX_INS {
                (*decoder).record.from_ip = ip;
            } else if idx == SPE_ADDR_PKT_HDR_INDEX_BRANCH {
                (*decoder).record.to_ip = ip;
            } else if idx == SPE_ADDR_PKT_HDR_INDEX_DATA_VIRT {
                (*decoder).record.virt_addr = ip;
            } else if idx == SPE_ADDR_PKT_HDR_INDEX_DATA_PHYS {
                (*decoder).record.phys_addr = ip;
            } else if idx == SPE_ADDR_PKT_HDR_INDEX_PREV_BRANCH {
                (*decoder).record.prev_br_tgt = ip;
            }
        } else if packet.type_ == ARM_SPE_COUNTER {
            if idx == SPE_CNT_PKT_HDR_INDEX_TOTAL_LAT {
                (*decoder).record.latency = payload;
            }
        } else if packet.type_ == ARM_SPE_CONTEXT {
            (*decoder).record.context_id = payload;
        } else if packet.type_ == ARM_SPE_OP_TYPE {
            if idx == SPE_OP_PKT_HDR_CLASS_LD_ST_ATOMIC {
                (*decoder).record.op |= ARM_SPE_OP_LDST;
                if (payload & SPE_OP_PKT_ST) != 0 {
                    (*decoder).record.op |= ARM_SPE_OP_ST;
                } else {
                    (*decoder).record.op |= ARM_SPE_OP_LD;
                }

                if SPE_OP_PKT_LDST_SUBCLASS_GP_REG(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_GP_REG;
                } else if SPE_OP_PKT_LDST_SUBCLASS_SIMD_FP(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_SIMD_FP;
                } else if SPE_OP_PKT_LDST_SUBCLASS_UNSPEC_REG(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_UNSPEC_REG;
                } else if SPE_OP_PKT_LDST_SUBCLASS_NV_SYSREG(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_NV_SYSREG;
                } else if SPE_OP_PKT_LDST_SUBCLASS_MTE_TAG(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_MTE_TAG;
                } else if SPE_OP_PKT_LDST_SUBCLASS_EXTENDED(payload) {
                    if (payload & SPE_OP_PKT_AR) != 0 {
                        (*decoder).record.op |= ARM_SPE_OP_AR;
                    }
                    if (payload & SPE_OP_PKT_EXCL) != 0 {
                        (*decoder).record.op |= ARM_SPE_OP_EXCL;
                    }
                    if (payload & SPE_OP_PKT_AT) != 0 {
                        (*decoder).record.op |= ARM_SPE_OP_ATOMIC;
                    }
                } else if SPE_OP_PKT_LDST_SUBCLASS_SVE_SME_REG(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_SVE;
                    if (payload & SPE_OP_PKT_SVE_PRED) != 0 {
                        (*decoder).record.op |= ARM_SPE_OP_PRED;
                    }
                    if (payload & SPE_OP_PKT_SVE_SG) != 0 {
                        (*decoder).record.op |= ARM_SPE_OP_SG;
                    }
                } else if SPE_OP_PKT_LDST_SUBCLASS_MEMCPY(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_MEMCPY;
                } else if SPE_OP_PKT_LDST_SUBCLASS_MEMSET(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_MEMSET;
                } else if SPE_OP_PKT_LDST_SUBCLASS_GCS(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_GCS;
                    if (payload & SPE_OP_PKT_GCS_COMM) != 0 {
                        (*decoder).record.op |= ARM_SPE_OP_COMM;
                    }
                }
            } else if idx == SPE_OP_PKT_HDR_CLASS_OTHER {
                (*decoder).record.op |= ARM_SPE_OP_OTHER;
                if SPE_OP_PKT_OTHER_SUBCLASS_SVE(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_SVE | ARM_SPE_OP_DP;
                    if (payload & SPE_OP_PKT_OTHER_FP) != 0 {
                        (*decoder).record.op |= ARM_SPE_OP_FP;
                    }
                    if (payload & SPE_OP_PKT_SVE_PRED) != 0 {
                        (*decoder).record.op |= ARM_SPE_OP_PRED;
                    }
                } else if SPE_OP_PKT_OTHER_SUBCLASS_SME(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_SME;
                    if (payload & SPE_OP_PKT_OTHER_FP) != 0 {
                        (*decoder).record.op |= ARM_SPE_OP_FP;
                    }
                } else if SPE_OP_PKT_OTHER_SUBCLASS_OTHER(payload) {
                    if (payload & SPE_OP_PKT_OTHER_ASE) != 0 {
                        (*decoder).record.op |= ARM_SPE_OP_ASE;
                    }
                    if (payload & SPE_OP_PKT_OTHER_FP) != 0 {
                        (*decoder).record.op |= ARM_SPE_OP_FP;
                    }
                    if (payload & SPE_OP_PKT_COND) != 0 {
                        (*decoder).record.op |= ARM_SPE_OP_COND;
                    }
                }
            } else if idx == SPE_OP_PKT_HDR_CLASS_BR_ERET {
                (*decoder).record.op |= ARM_SPE_OP_BRANCH_ERET;
                if (payload & SPE_OP_PKT_COND) != 0 {
                    (*decoder).record.op |= ARM_SPE_OP_BR_COND;
                }
                if (payload & SPE_OP_PKT_INDIRECT_BRANCH) != 0 {
                    (*decoder).record.op |= ARM_SPE_OP_BR_INDIRECT;
                }
                if (payload & SPE_OP_PKT_GCS) != 0 {
                    (*decoder).record.op |= ARM_SPE_OP_BR_GCS;
                }
                if SPE_OP_PKT_CR_BL(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_BR_CR_BL;
                }
                if SPE_OP_PKT_CR_RET(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_BR_CR_RET;
                }
                if SPE_OP_PKT_CR_NON_BL_RET(payload) {
                    (*decoder).record.op |= ARM_SPE_OP_BR_CR_NON_BL_RET;
                }
            } else {
                pr_err(b"Get packet error!\n\0".as_ptr());
                return -1;
            }
        } else if packet.type_ == ARM_SPE_EVENTS {
            (*decoder).record.type_ = payload;
        } else if packet.type_ == ARM_SPE_DATA_SOURCE {
            (*decoder).record.source = payload;
        } else if packet.type_ == ARM_SPE_BAD {
        } else if packet.type_ == ARM_SPE_PAD {
        } else {
            pr_err(b"Get packet error!\n\0".as_ptr());
            return -1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn arm_spe_decode(decoder: *mut arm_spe_decoder) -> c_int {
    arm_spe_read_record(decoder)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
