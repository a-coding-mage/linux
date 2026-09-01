// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2019 ARM Limited */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

// Translated from dependencies included by the C source:
// <ctype.h>, <string.h>, and "testcases.h".
// The concrete values for these constants are supplied by the surrounding
// translated repository headers.
const FPSIMD_MAGIC: u32 = /* TODO: from testcases.h */ 0;
const ESR_MAGIC: u32 = /* TODO: from testcases.h */ 0;
const POE_MAGIC: u32 = /* TODO: from testcases.h */ 0;
const TPIDR2_MAGIC: u32 = /* TODO: from testcases.h */ 0;
const SVE_MAGIC: u32 = /* TODO: from testcases.h */ 0;
const ZA_MAGIC: u32 = /* TODO: from testcases.h */ 0;
const ZT_MAGIC: u32 = /* TODO: from testcases.h */ 0;
const FPMR_MAGIC: u32 = /* TODO: from testcases.h */ 0;
const GCS_MAGIC: u32 = /* TODO: from testcases.h */ 0;
const EXTRA_MAGIC: u32 = /* TODO: from testcases.h */ 0;
const KSFT_BAD_MAGIC: u32 = /* TODO: from testcases.h */ 0;

const FPSIMD_CTX: c_int = /* TODO: from testcases.h */ 0;
const SVE_CTX: c_int = /* TODO: from testcases.h */ 0;
const ZA_CTX: c_int = /* TODO: from testcases.h */ 0;
const ZT_CTX: c_int = /* TODO: from testcases.h */ 0;
const FPMR_CTX: c_int = /* TODO: from testcases.h */ 0;
const GCS_CTX: c_int = /* TODO: from testcases.h */ 0;
const EXTRA_CTX: c_int = /* TODO: from testcases.h */ 0;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct _aarch64_ctx {
    pub magic: u32,
    pub size: u32,
}

#[repr(C)]
pub struct fpsimd_context {
    pub head: _aarch64_ctx,
}

#[repr(C)]
pub struct esr_context {
    pub head: _aarch64_ctx,
}

#[repr(C)]
pub struct poe_context {
    pub head: _aarch64_ctx,
}

#[repr(C)]
pub struct tpidr2_context {
    pub head: _aarch64_ctx,
}

#[repr(C)]
pub struct sve_context {
    pub head: _aarch64_ctx,
    pub vl: u16,
}

#[repr(C)]
pub struct za_context {
    pub head: _aarch64_ctx,
    pub vl: u16,
}

#[repr(C)]
pub struct zt_context {
    pub head: _aarch64_ctx,
    pub nregs: u16,
}

#[repr(C)]
pub struct fpmr_context {
    pub head: _aarch64_ctx,
}

#[repr(C)]
pub struct gcs_context {
    pub head: _aarch64_ctx,
}

#[repr(C)]
pub struct extra_context {
    pub head: _aarch64_ctx,
    pub datap: u64,
    pub size: u32,
}

#[repr(C)]
pub struct mcontext_t {
    pub __reserved: [u8; 0],
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_mcontext: mcontext_t,
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn isalnum(c: c_int) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;

    fn GET_RESV_NEXT_HEAD(head: *mut _aarch64_ctx) -> *mut _aarch64_ctx;
    fn SVE_SIG_CONTEXT_SIZE(vq: usize) -> usize;
    fn ZA_SIG_CONTEXT_SIZE(vq: usize) -> usize;
    fn ZT_SIG_CONTEXT_SIZE(nregs: u16) -> u32;
    fn sve_vq_from_vl(vl: u16) -> usize;
    fn sve_vl_valid(vl: u16) -> bool;
    fn get_terminator(
        shead: *mut _aarch64_ctx,
        resv_sz: usize,
        offset: *mut usize,
    ) -> *mut _aarch64_ctx;
    fn get_header(
        shead: *mut _aarch64_ctx,
        magic: u32,
        resv_sz: usize,
        offset: *mut usize,
    ) -> *mut _aarch64_ctx;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn validate_extra_context(
    extra: *mut extra_context,
    err: *mut *mut c_char,
    extra_data: *mut *mut c_void,
    extra_size: *mut usize,
) -> bool {
    let mut term: *mut _aarch64_ctx;

    if extra.is_null() || err.is_null() {
        return false;
    }

    fprintf(stderr, c"Validating EXTRA...\n".as_ptr());
    term = GET_RESV_NEXT_HEAD(ptr::addr_of_mut!((*extra).head));
    if term.is_null() || (*term).magic != 0 || (*term).size != 0 {
        *err = c"Missing terminator after EXTRA context".as_ptr() as *mut c_char;
        return false;
    }
    if ((*extra).datap & 0x0f_u64) != 0 {
        *err = c"Extra DATAP misaligned".as_ptr() as *mut c_char;
    } else if (((*extra).size as c_ulong) & 0x0f_c_ulong) != 0 {
        *err = c"Extra SIZE misaligned".as_ptr() as *mut c_char;
    } else if (*extra).datap != term as u64 + 0x10_u64 {
        *err = c"Extra DATAP misplaced (not contiguous)".as_ptr() as *mut c_char;
    }
    if !(*err).is_null() {
        return false;
    }

    *extra_data = (*extra).datap as *mut c_void;
    *extra_size = (*extra).size as usize;

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn validate_sve_context(sve: *mut sve_context, err: *mut *mut c_char) -> bool {
    /* Size will be rounded up to a multiple of 16 bytes */
    let regs_size: usize = ((SVE_SIG_CONTEXT_SIZE(sve_vq_from_vl((*sve).vl)) + 15) / 16) * 16;

    if sve.is_null() || err.is_null() {
        return false;
    }

    /* Either a bare sve_context or a sve_context followed by regs data */
    if (*sve).head.size != mem::size_of::<sve_context>() as u32 && (*sve).head.size != regs_size as u32 {
        *err = c"bad size for SVE context".as_ptr() as *mut c_char;
        return false;
    }

    if !sve_vl_valid((*sve).vl) {
        *err = c"SVE VL invalid".as_ptr() as *mut c_char;

        return false;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn validate_za_context(za: *mut za_context, err: *mut *mut c_char) -> bool {
    /* Size will be rounded up to a multiple of 16 bytes */
    let regs_size: usize = ((ZA_SIG_CONTEXT_SIZE(sve_vq_from_vl((*za).vl)) + 15) / 16) * 16;

    if za.is_null() || err.is_null() {
        return false;
    }

    /* Either a bare za_context or a za_context followed by regs data */
    if (*za).head.size != mem::size_of::<za_context>() as u32 && (*za).head.size != regs_size as u32 {
        *err = c"bad size for ZA context".as_ptr() as *mut c_char;
        return false;
    }

    if !sve_vl_valid((*za).vl) {
        *err = c"SME VL in ZA context invalid".as_ptr() as *mut c_char;

        return false;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn validate_zt_context(zt: *mut zt_context, err: *mut *mut c_char) -> bool {
    if zt.is_null() || err.is_null() {
        return false;
    }

    /* If the context is present there should be at least one register */
    if (*zt).nregs == 0 {
        *err = c"no registers".as_ptr() as *mut c_char;
        return false;
    }

    /* Size should agree with the number of registers */
    if (*zt).head.size != ZT_SIG_CONTEXT_SIZE((*zt).nregs) {
        *err = c"register count does not match size".as_ptr() as *mut c_char;
        return false;
    }

    true
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn validate_reserved(
    uc: *mut ucontext_t,
    mut resv_sz: usize,
    err: *mut *mut c_char,
) -> bool {
    let mut terminated: bool = false;
    let mut offs: usize = 0;
    let mut flags: c_int = 0;
    let mut new_flags: c_int;
    let mut i: c_int;
    let mut extra: *mut extra_context = ptr::null_mut();
    let mut sve: *mut sve_context = ptr::null_mut();
    let mut za: *mut za_context = ptr::null_mut();
    let mut zt: *mut zt_context = ptr::null_mut();
    let mut head: *mut _aarch64_ctx =
        ptr::addr_of_mut!((*uc).uc_mcontext.__reserved) as *mut _aarch64_ctx;
    let mut extra_data: *mut c_void = ptr::null_mut();
    let mut extra_sz: usize = 0;
    let mut magic: [c_char; 4] = [0; 4];

    if err.is_null() {
        return false;
    }
    /* Walk till the end terminator verifying __reserved contents */
    while !head.is_null() && !terminated && offs < resv_sz {
        if ((head as u64) & 0x0f_u64) != 0 {
            *err = c"Misaligned HEAD".as_ptr() as *mut c_char;
            return false;
        }

        new_flags = 0;

        match (*head).magic {
            0 => {
                if (*head).size != 0 {
                    *err = c"Bad size for terminator".as_ptr() as *mut c_char;
                } else if !extra_data.is_null() {
                    /* End of main data, walking the extra data */
                    head = extra_data as *mut _aarch64_ctx;
                    resv_sz = extra_sz;
                    offs = 0;

                    extra_data = ptr::null_mut();
                    extra_sz = 0;
                    continue;
                } else {
                    terminated = true;
                }
            }
            FPSIMD_MAGIC => {
                if (flags & FPSIMD_CTX) != 0 {
                    *err = c"Multiple FPSIMD_MAGIC".as_ptr() as *mut c_char;
                } else if (*head).size != mem::size_of::<fpsimd_context>() as u32 {
                    *err = c"Bad size for fpsimd_context".as_ptr() as *mut c_char;
                }
                new_flags |= FPSIMD_CTX;
            }
            ESR_MAGIC => {
                if (*head).size != mem::size_of::<esr_context>() as u32 {
                    *err = c"Bad size for esr_context".as_ptr() as *mut c_char;
                }
            }
            POE_MAGIC => {
                if (*head).size != mem::size_of::<poe_context>() as u32 {
                    *err = c"Bad size for poe_context".as_ptr() as *mut c_char;
                }
            }
            TPIDR2_MAGIC => {
                if (*head).size != mem::size_of::<tpidr2_context>() as u32 {
                    *err = c"Bad size for tpidr2_context".as_ptr() as *mut c_char;
                }
            }
            SVE_MAGIC => {
                if (flags & SVE_CTX) != 0 {
                    *err = c"Multiple SVE_MAGIC".as_ptr() as *mut c_char;
                }
                /* Size is validated in validate_sve_context() */
                sve = head as *mut sve_context;
                new_flags |= SVE_CTX;
            }
            ZA_MAGIC => {
                if (flags & ZA_CTX) != 0 {
                    *err = c"Multiple ZA_MAGIC".as_ptr() as *mut c_char;
                }
                /* Size is validated in validate_za_context() */
                za = head as *mut za_context;
                new_flags |= ZA_CTX;
            }
            ZT_MAGIC => {
                if (flags & ZT_CTX) != 0 {
                    *err = c"Multiple ZT_MAGIC".as_ptr() as *mut c_char;
                }
                /* Size is validated in validate_za_context() */
                zt = head as *mut zt_context;
                new_flags |= ZT_CTX;
            }
            FPMR_MAGIC => {
                if (flags & FPMR_CTX) != 0 {
                    *err = c"Multiple FPMR_MAGIC".as_ptr() as *mut c_char;
                } else if (*head).size != mem::size_of::<fpmr_context>() as u32 {
                    *err = c"Bad size for fpmr_context".as_ptr() as *mut c_char;
                }
                new_flags |= FPMR_CTX;
            }
            GCS_MAGIC => {
                if (flags & GCS_CTX) != 0 {
                    *err = c"Multiple GCS_MAGIC".as_ptr() as *mut c_char;
                }
                if (*head).size != mem::size_of::<gcs_context>() as u32 {
                    *err = c"Bad size for gcs_context".as_ptr() as *mut c_char;
                }
                new_flags |= GCS_CTX;
            }
            EXTRA_MAGIC => {
                if (flags & EXTRA_CTX) != 0 {
                    *err = c"Multiple EXTRA_MAGIC".as_ptr() as *mut c_char;
                } else if (*head).size != mem::size_of::<extra_context>() as u32 {
                    *err = c"Bad size for extra_context".as_ptr() as *mut c_char;
                }
                new_flags |= EXTRA_CTX;
                extra = head as *mut extra_context;
            }
            KSFT_BAD_MAGIC => {
                /*
                 * This is a BAD magic header defined
                 * artificially by a testcase and surely
                 * unknown to the Kernel parse_user_sigframe().
                 * It MUST cause a Kernel induced SEGV
                 */
                *err = c"BAD MAGIC !".as_ptr() as *mut c_char;
            }
            _ => {
                /*
                 * A still unknown Magic: potentially freshly added
                 * to the Kernel code and still unknown to the
                 * tests.  Magic numbers are supposed to be allocated
                 * as somewhat meaningful ASCII strings so try to
                 * print as such as well as the raw number.
                 */
                memcpy(
                    magic.as_mut_ptr() as *mut c_void,
                    ptr::addr_of!((*head).magic) as *const c_void,
                    mem::size_of_val(&magic),
                );
                i = 0;
                while (i as usize) < mem::size_of_val(&magic) {
                    if isalnum(magic[i as usize] as c_int) == 0 {
                        magic[i as usize] = b'?' as c_char;
                    }
                    i += 1;
                }

                fprintf(
                    stdout,
                    c"SKIP Unknown MAGIC: 0x%X (%c%c%c%c) - Is KSFT arm64/signal up to date ?\n"
                        .as_ptr(),
                    (*head).magic,
                    magic[3] as c_int,
                    magic[2] as c_int,
                    magic[1] as c_int,
                    magic[0] as c_int,
                );
            }
        }

        if !(*err).is_null() {
            return false;
        }

        offs += (*head).size as usize;
        if resv_sz < offs + mem::size_of::<_aarch64_ctx>() {
            *err = c"HEAD Overrun".as_ptr() as *mut c_char;
            return false;
        }

        if (new_flags & EXTRA_CTX) != 0 {
            if !validate_extra_context(extra, err, &mut extra_data, &mut extra_sz) {
                return false;
            }
        }
        if (new_flags & SVE_CTX) != 0 {
            if !validate_sve_context(sve, err) {
                return false;
            }
        }
        if (new_flags & ZA_CTX) != 0 {
            if !validate_za_context(za, err) {
                return false;
            }
        }
        if (new_flags & ZT_CTX) != 0 {
            if !validate_zt_context(zt, err) {
                return false;
            }
        }

        flags |= new_flags;

        head = GET_RESV_NEXT_HEAD(head);
    }

    if terminated && (flags & FPSIMD_CTX) == 0 {
        *err = c"Missing FPSIMD".as_ptr() as *mut c_char;
        return false;
    }

    if terminated && (flags & ZT_CTX) != 0 && (flags & ZA_CTX) == 0 {
        *err = c"ZT context but no ZA context".as_ptr() as *mut c_char;
        return false;
    }

    true
}

/*
 * This function walks through the records inside the provided reserved area
 * trying to find enough space to fit @need_sz bytes: if not enough space is
 * available and an extra_context record is present, it throws away the
 * extra_context record.
 *
 * It returns a pointer to a new header where it is possible to start storing
 * our need_sz bytes.
 *
 * @shead: points to the start of reserved area
 * @need_sz: needed bytes
 * @resv_sz: reserved area size in bytes
 * @offset: if not null, this will be filled with the offset of the return
 *	    head pointer from @shead
 *
 * @return: pointer to a new head where to start storing need_sz bytes, or
 *	    NULL if space could not be made available.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_starting_head(
    shead: *mut _aarch64_ctx,
    need_sz: usize,
    resv_sz: usize,
    offset: *mut usize,
) -> *mut _aarch64_ctx {
    let mut offs: usize = 0;
    let mut head: *mut _aarch64_ctx;

    head = get_terminator(shead, resv_sz, &mut offs);
    /* not found a terminator...no need to update offset if any */
    if head.is_null() {
        return head;
    }
    if resv_sz - offs < need_sz {
        fprintf(
            stderr,
            c"Low on space:%zd. Discarding extra_context.\n".as_ptr(),
            resv_sz - offs,
        );
        head = get_header(shead, EXTRA_MAGIC, resv_sz, &mut offs);
        if head.is_null() || resv_sz - offs < need_sz {
            fprintf(stderr, c"Failed to reclaim space on sigframe.\n".as_ptr());
            return ptr::null_mut();
        }
    }

    fprintf(stderr, c"Available space:%zd\n".as_ptr(), resv_sz - offs);
    if !offset.is_null() {
        *offset = offs;
    }
    head
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
