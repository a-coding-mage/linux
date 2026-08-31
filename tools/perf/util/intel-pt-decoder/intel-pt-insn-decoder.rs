// SPDX-License-Identifier: GPL-2.0-only
/*
 * intel_pt_insn_decoder.c: Intel Processor Trace support
 * Copyright (c) 2013-2014, Intel Corporation.
 */

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_ulonglong, c_void};

type size_t = usize;
type uint64_t = u64;
type u8 = u8;

/* Dependencies supplied by the translated equivalents of:
 * <linux/kernel.h>, <stdio.h>, <string.h>, <endian.h>, <byteswap.h>,
 * "../../../arch/x86/include/asm/insn.h", "event.h",
 * "intel-pt-insn-decoder.h", "dump-insn.h", and "util/sample.h".
 */

const INSN_MODE_64: c_int = 2;
const INSN_MODE_32: c_int = 1;

const INTEL_PT_INSN_BUF_SZ: usize = 16;
const MAX_INSN_SIZE: usize = 16;
const MAX_INSN: usize = 16;

const PERF_IP_FLAG_BRANCH: c_int = 1 << 0;
const PERF_IP_FLAG_CALL: c_int = 1 << 1;
const PERF_IP_FLAG_RETURN: c_int = 1 << 2;
const PERF_IP_FLAG_CONDITIONAL: c_int = 1 << 3;
const PERF_IP_FLAG_INTERRUPT: c_int = 1 << 4;
const PERF_IP_FLAG_SYSCALLRET: c_int = 1 << 5;
const PERF_IP_FLAG_VMENTRY: c_int = 1 << 6;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_insn_op {
    INTEL_PT_OP_OTHER = 0,
    INTEL_PT_OP_CALL,
    INTEL_PT_OP_RET,
    INTEL_PT_OP_JCC,
    INTEL_PT_OP_JMP,
    INTEL_PT_OP_LOOP,
    INTEL_PT_OP_IRET,
    INTEL_PT_OP_INT,
    INTEL_PT_OP_SYSCALL,
    INTEL_PT_OP_SYSRET,
    INTEL_PT_OP_VMENTRY,
    INTEL_PT_OP_ERETS,
    INTEL_PT_OP_ERETU,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pt_insn_branch {
    INTEL_PT_BR_NO_BRANCH = 0,
    INTEL_PT_BR_CONDITIONAL,
    INTEL_PT_BR_UNCONDITIONAL,
    INTEL_PT_BR_INDIRECT,
}

#[repr(C)]
pub struct insn_bytes {
    pub bytes: [c_uchar; 16],
}

#[repr(C)]
pub struct insn_field {
    pub value: c_int,
    pub nbytes: c_uchar,
}

#[repr(C)]
pub struct insn {
    pub prefixes: insn_bytes,
    pub opcode: insn_bytes,
    pub modrm: insn_bytes,
    pub immediate: insn_field,
    pub length: c_int,
}

#[repr(C)]
pub struct intel_pt_insn {
    pub op: intel_pt_insn_op,
    pub branch: intel_pt_insn_branch,
    pub length: c_int,
    pub rel: c_int,
    pub emulated_ptwrite: bool,
    pub buf: [c_uchar; INTEL_PT_INSN_BUF_SZ],
}

#[repr(C)]
pub struct perf_insn {
    pub is64bit: bool,
    pub out: [c_char; 256],
}

unsafe extern "C" {
    fn insn_decode(insn: *mut insn, buf: *const c_uchar, len: size_t, mode: c_int) -> c_int;
    fn insn_is_avx_or_xop(insn: *mut insn) -> bool;
    fn insn_is_rex2(insn: *mut insn) -> bool;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn snprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn scnprintf(str_: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
}

/* Based on branch_type() from arch/x86/events/intel/lbr.c */
unsafe fn intel_pt_insn_decoder(insn: *mut insn, intel_pt_insn: *mut intel_pt_insn) {
    let mut op: intel_pt_insn_op = intel_pt_insn_op::INTEL_PT_OP_OTHER;
    let mut branch: intel_pt_insn_branch = intel_pt_insn_branch::INTEL_PT_BR_NO_BRANCH;
    let ext: c_int;

    unsafe {
        (*intel_pt_insn).rel = 0;
        (*intel_pt_insn).emulated_ptwrite = false;

        if insn_is_avx_or_xop(insn) {
            (*intel_pt_insn).op = intel_pt_insn_op::INTEL_PT_OP_OTHER;
            (*intel_pt_insn).branch = intel_pt_insn_branch::INTEL_PT_BR_NO_BRANCH;
            (*intel_pt_insn).length = (*insn).length;
            return;
        }

        match (*insn).opcode.bytes[0] as c_int {
            0x0f => {
                match (*insn).opcode.bytes[1] as c_int {
                    0x01 => {
                        match (*insn).modrm.bytes[0] as c_int {
                            0xc2 | /* vmlaunch */ 0xc3 /* vmresume */ => {
                                op = intel_pt_insn_op::INTEL_PT_OP_VMENTRY;
                                branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
                            }
                            0xca => {
                                match (*insn).prefixes.bytes[3] as c_int {
                                    0xf2 => {
                                        /* erets */
                                        op = intel_pt_insn_op::INTEL_PT_OP_ERETS;
                                        branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
                                    }
                                    0xf3 => {
                                        /* eretu */
                                        op = intel_pt_insn_op::INTEL_PT_OP_ERETU;
                                        branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                    }
                    0x05 | /* syscall */ 0x34 /* sysenter */ => {
                        op = intel_pt_insn_op::INTEL_PT_OP_SYSCALL;
                        branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
                    }
                    0x07 | /* sysret */ 0x35 /* sysexit */ => {
                        op = intel_pt_insn_op::INTEL_PT_OP_SYSRET;
                        branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
                    }
                    0x80..=0x8f => {
                        /* jcc */
                        op = intel_pt_insn_op::INTEL_PT_OP_JCC;
                        branch = intel_pt_insn_branch::INTEL_PT_BR_CONDITIONAL;
                    }
                    _ => {}
                }
            }
            0x70..=0x7f => {
                /* jcc */
                op = intel_pt_insn_op::INTEL_PT_OP_JCC;
                branch = intel_pt_insn_branch::INTEL_PT_BR_CONDITIONAL;
            }
            0xa1 => {
                if insn_is_rex2(insn) {
                    /* jmpabs */
                    (*intel_pt_insn).op = intel_pt_insn_op::INTEL_PT_OP_JMP;
                    /* jmpabs causes a TIP packet like an indirect branch */
                    (*intel_pt_insn).branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
                    (*intel_pt_insn).length = (*insn).length;
                    return;
                }
            }
            0xc2 | /* near ret */ 0xc3 | /* near ret */ 0xca | /* far ret */ 0xcb /* far ret */ => {
                op = intel_pt_insn_op::INTEL_PT_OP_RET;
                branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
            }
            0xcf => {
                /* iret */
                op = intel_pt_insn_op::INTEL_PT_OP_IRET;
                branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
            }
            0xcc..=0xce => {
                /* int */
                op = intel_pt_insn_op::INTEL_PT_OP_INT;
                branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
            }
            0xe8 => {
                /* call near rel */
                op = intel_pt_insn_op::INTEL_PT_OP_CALL;
                branch = intel_pt_insn_branch::INTEL_PT_BR_UNCONDITIONAL;
            }
            0x9a => {
                /* call far absolute */
                op = intel_pt_insn_op::INTEL_PT_OP_CALL;
                branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
            }
            0xe0..=0xe2 => {
                /* loop */
                op = intel_pt_insn_op::INTEL_PT_OP_LOOP;
                branch = intel_pt_insn_branch::INTEL_PT_BR_CONDITIONAL;
            }
            0xe3 => {
                /* jcc */
                op = intel_pt_insn_op::INTEL_PT_OP_JCC;
                branch = intel_pt_insn_branch::INTEL_PT_BR_CONDITIONAL;
            }
            0xe9 | /* jmp */ 0xeb /* jmp */ => {
                op = intel_pt_insn_op::INTEL_PT_OP_JMP;
                branch = intel_pt_insn_branch::INTEL_PT_BR_UNCONDITIONAL;
            }
            0xea => {
                /* far jmp */
                op = intel_pt_insn_op::INTEL_PT_OP_JMP;
                branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
            }
            0xff => {
                /* call near absolute, call far absolute ind */
                ext = (((*insn).modrm.bytes[0] >> 3) & 0x7) as c_int;
                match ext {
                    2 | /* near ind call */ 3 /* far ind call */ => {
                        op = intel_pt_insn_op::INTEL_PT_OP_CALL;
                        branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
                    }
                    4 | 5 => {
                        op = intel_pt_insn_op::INTEL_PT_OP_JMP;
                        branch = intel_pt_insn_branch::INTEL_PT_BR_INDIRECT;
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        (*intel_pt_insn).op = op;
        (*intel_pt_insn).branch = branch;
        (*intel_pt_insn).length = (*insn).length;

        if branch == intel_pt_insn_branch::INTEL_PT_BR_CONDITIONAL
            || branch == intel_pt_insn_branch::INTEL_PT_BR_UNCONDITIONAL
        {
            /* C source used:
             * #if __BYTE_ORDER__ == __ORDER_BIG_ENDIAN__
             *     byte-swap immediates by nbytes
             * #else
             *     intel_pt_insn->rel = insn->immediate.value;
             * #endif
             */
            #[cfg(target_endian = "big")]
            {
                match (*insn).immediate.nbytes as c_int {
                    1 => {
                        (*intel_pt_insn).rel = (*insn).immediate.value;
                    }
                    2 => {
                        (*intel_pt_insn).rel = ((*insn).immediate.value as i16).swap_bytes() as c_int;
                    }
                    4 => {
                        (*intel_pt_insn).rel = ((*insn).immediate.value as u32).swap_bytes() as c_int;
                    }
                    _ => {
                        (*intel_pt_insn).rel = 0;
                    }
                }
            }
            #[cfg(not(target_endian = "big"))]
            {
                (*intel_pt_insn).rel = (*insn).immediate.value;
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn intel_pt_get_insn(
    buf: *const c_uchar,
    len: size_t,
    x86_64: c_int,
    intel_pt_insn: *mut intel_pt_insn,
) -> c_int {
    let mut insn: insn = unsafe { core::mem::zeroed() };
    let ret: c_int;

    unsafe {
        ret = insn_decode(
            &mut insn,
            buf,
            len,
            if x86_64 != 0 { INSN_MODE_64 } else { INSN_MODE_32 },
        );
        if ret < 0 || insn.length as size_t > len {
            return -1;
        }

        intel_pt_insn_decoder(&mut insn, intel_pt_insn);
        if (insn.length as usize) < INTEL_PT_INSN_BUF_SZ {
            memcpy(
                (*intel_pt_insn).buf.as_mut_ptr() as *mut c_void,
                buf as *const c_void,
                insn.length as size_t,
            );
        } else {
            memcpy(
                (*intel_pt_insn).buf.as_mut_ptr() as *mut c_void,
                buf as *const c_void,
                INTEL_PT_INSN_BUF_SZ,
            );
        }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch_is_uncond_branch(
    buf: *const c_uchar,
    len: size_t,
    x86_64: c_int,
) -> c_int {
    let mut in_: intel_pt_insn = unsafe { core::mem::zeroed() };
    unsafe {
        if intel_pt_get_insn(buf, len, x86_64, &mut in_) < 0 {
            return -1;
        }
        (in_.branch == intel_pt_insn_branch::INTEL_PT_BR_UNCONDITIONAL
            || in_.branch == intel_pt_insn_branch::INTEL_PT_BR_INDIRECT) as c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn dump_insn(
    x: *mut perf_insn,
    _ip: uint64_t,
    inbuf: *mut u8,
    inlen: c_int,
    lenp: *mut c_int,
) -> *const c_char {
    let mut insn: insn = unsafe { core::mem::zeroed() };
    let mut n: c_int;
    let mut i: c_int;
    let ret: c_int;

    unsafe {
        ret = insn_decode(
            &mut insn,
            inbuf,
            inlen as size_t,
            if (*x).is64bit { INSN_MODE_64 } else { INSN_MODE_32 },
        );

        if ret < 0 || insn.length > inlen {
            return c"<bad>".as_ptr();
        }
        if !lenp.is_null() {
            *lenp = insn.length;
        }
        n = scnprintf((*x).out.as_mut_ptr(), (*x).out.len(), c"insn: ".as_ptr());
        i = 0;
        while i < insn.length {
            n += scnprintf(
                (*x).out.as_mut_ptr().add(n as usize),
                (*x).out.len() - n as usize,
                c"%02x ".as_ptr(),
                *inbuf.add(i as usize) as c_uint,
            );
            i += 1;
        }
        (*x).out.as_ptr()
    }
}

#[no_mangle]
pub static branch_name: [*const c_char; 13] = [
    c"Other".as_ptr(),
    c"Call".as_ptr(),
    c"Ret".as_ptr(),
    c"Jcc".as_ptr(),
    c"Jmp".as_ptr(),
    c"Loop".as_ptr(),
    c"IRet".as_ptr(),
    c"Int".as_ptr(),
    c"Syscall".as_ptr(),
    c"Sysret".as_ptr(),
    c"VMentry".as_ptr(),
    c"Erets".as_ptr(),
    c"Eretu".as_ptr(),
];

#[no_mangle]
pub unsafe extern "C" fn intel_pt_insn_name(op: intel_pt_insn_op) -> *const c_char {
    unsafe { branch_name[op as usize] }
}

#[no_mangle]
pub unsafe extern "C" fn intel_pt_insn_desc(
    intel_pt_insn: *const intel_pt_insn,
    buf: *mut c_char,
    buf_len: size_t,
) -> c_int {
    unsafe {
        match (*intel_pt_insn).branch {
            intel_pt_insn_branch::INTEL_PT_BR_CONDITIONAL
            | intel_pt_insn_branch::INTEL_PT_BR_UNCONDITIONAL => {
                snprintf(
                    buf,
                    buf_len,
                    c"%s %s%d".as_ptr(),
                    intel_pt_insn_name((*intel_pt_insn).op),
                    if (*intel_pt_insn).rel > 0 {
                        c"+".as_ptr()
                    } else {
                        c"".as_ptr()
                    },
                    (*intel_pt_insn).rel,
                )
            }
            intel_pt_insn_branch::INTEL_PT_BR_NO_BRANCH
            | intel_pt_insn_branch::INTEL_PT_BR_INDIRECT => {
                snprintf(buf, buf_len, c"%s".as_ptr(), intel_pt_insn_name((*intel_pt_insn).op))
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn intel_pt_insn_type(op: intel_pt_insn_op) -> c_int {
    match op {
        intel_pt_insn_op::INTEL_PT_OP_OTHER => 0,
        intel_pt_insn_op::INTEL_PT_OP_CALL => PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL,
        intel_pt_insn_op::INTEL_PT_OP_RET => PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN,
        intel_pt_insn_op::INTEL_PT_OP_JCC => PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CONDITIONAL,
        intel_pt_insn_op::INTEL_PT_OP_JMP => PERF_IP_FLAG_BRANCH,
        intel_pt_insn_op::INTEL_PT_OP_LOOP => PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CONDITIONAL,
        intel_pt_insn_op::INTEL_PT_OP_IRET
        | intel_pt_insn_op::INTEL_PT_OP_ERETS
        | intel_pt_insn_op::INTEL_PT_OP_ERETU => {
            PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN | PERF_IP_FLAG_INTERRUPT
        }
        intel_pt_insn_op::INTEL_PT_OP_INT => {
            PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_INTERRUPT
        }
        intel_pt_insn_op::INTEL_PT_OP_SYSCALL => {
            PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_SYSCALLRET
        }
        intel_pt_insn_op::INTEL_PT_OP_SYSRET => {
            PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_RETURN | PERF_IP_FLAG_SYSCALLRET
        }
        intel_pt_insn_op::INTEL_PT_OP_VMENTRY => {
            PERF_IP_FLAG_BRANCH | PERF_IP_FLAG_CALL | PERF_IP_FLAG_VMENTRY
        }
    }
}
