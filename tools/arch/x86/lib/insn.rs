// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * x86 instruction analysis
 *
 * Copyright (C) IBM Corporation, 2002, 2004, 2009
 */

// Translated from arch/x86/lib/insn.c.  C include dependencies are expected to
// provide the Rust equivalents of struct insn, struct insn_field, inat helpers,
// x86 bit macros, errno constants, emulate-prefix constants, and insn_mode.

use core::mem::size_of;
use core::ptr;

unsafe fn validate_next<T>(insn: *const insn, n: usize) -> bool {
    ((*insn).next_byte as usize).wrapping_add(size_of::<T>()).wrapping_add(n)
        <= (*insn).end_kaddr as usize
}

unsafe fn __get_next_u8(insn: *mut insn) -> insn_byte_t {
    let r = ptr::read_unaligned((*insn).next_byte as *const u8);
    (*insn).next_byte = (*insn).next_byte.add(size_of::<u8>());
    r as insn_byte_t
}

unsafe fn __get_next_i8(insn: *mut insn) -> i8 {
    let r = ptr::read_unaligned((*insn).next_byte as *const i8);
    (*insn).next_byte = (*insn).next_byte.add(size_of::<i8>());
    r
}

unsafe fn __get_next_i16(insn: *mut insn) -> i16 {
    let r = i16::from_le(ptr::read_unaligned((*insn).next_byte as *const i16));
    (*insn).next_byte = (*insn).next_byte.add(size_of::<i16>());
    r
}

unsafe fn __get_next_u16(insn: *mut insn) -> u16 {
    let r = u16::from_le(ptr::read_unaligned((*insn).next_byte as *const u16));
    (*insn).next_byte = (*insn).next_byte.add(size_of::<u16>());
    r
}

unsafe fn __get_next_i32(insn: *mut insn) -> i32 {
    let r = i32::from_le(ptr::read_unaligned((*insn).next_byte as *const i32));
    (*insn).next_byte = (*insn).next_byte.add(size_of::<i32>());
    r
}

unsafe fn __peek_nbyte_next_u8(insn: *const insn, n: usize) -> insn_byte_t {
    ptr::read_unaligned(((*insn).next_byte as *const u8).add(n)) as insn_byte_t
}

unsafe fn get_next_u8(insn: *mut insn) -> Result<insn_byte_t, i32> {
    if !validate_next::<u8>(insn, 0) {
        return Err(-ENODATA);
    }
    Ok(__get_next_u8(insn))
}

unsafe fn get_next_i8(insn: *mut insn) -> Result<i8, i32> {
    if !validate_next::<i8>(insn, 0) {
        return Err(-ENODATA);
    }
    Ok(__get_next_i8(insn))
}

unsafe fn get_next_i16(insn: *mut insn) -> Result<i16, i32> {
    if !validate_next::<i16>(insn, 0) {
        return Err(-ENODATA);
    }
    Ok(__get_next_i16(insn))
}

unsafe fn get_next_u16(insn: *mut insn) -> Result<u16, i32> {
    if !validate_next::<u16>(insn, 0) {
        return Err(-ENODATA);
    }
    Ok(__get_next_u16(insn))
}

unsafe fn get_next_i32(insn: *mut insn) -> Result<i32, i32> {
    if !validate_next::<i32>(insn, 0) {
        return Err(-ENODATA);
    }
    Ok(__get_next_i32(insn))
}

unsafe fn peek_nbyte_next_u8(insn: *const insn, n: usize) -> Result<insn_byte_t, i32> {
    if !validate_next::<u8>(insn, n) {
        return Err(-ENODATA);
    }
    Ok(__peek_nbyte_next_u8(insn, n))
}

unsafe fn peek_next_u8(insn: *const insn) -> Result<insn_byte_t, i32> {
    peek_nbyte_next_u8(insn, 0)
}

/**
 * insn_init() - initialize struct insn
 * @insn:	&struct insn to be initialized
 * @kaddr:	address (in kernel memory) of instruction (or copy thereof)
 * @buf_len:	length of the insn buffer at @kaddr
 * @x86_64:	!0 for 64-bit kernel or 64-bit app
 */
pub unsafe extern "C" fn insn_init(
    insn: *mut insn,
    kaddr: *const core::ffi::c_void,
    mut buf_len: i32,
    x86_64: i32,
) {
    /*
     * Instructions longer than MAX_INSN_SIZE (15 bytes) are invalid
     * even if the input buffer is long enough to hold them.
     */
    if buf_len > MAX_INSN_SIZE {
        buf_len = MAX_INSN_SIZE;
    }

    ptr::write_bytes(insn as *mut u8, 0, size_of::<insn>());
    (*insn).kaddr = kaddr as *const u8;
    (*insn).end_kaddr = (kaddr as *const u8).add(buf_len as usize);
    (*insn).next_byte = kaddr as *const u8;
    (*insn).x86_64 = x86_64;
    (*insn).opnd_bytes = 4;
    if x86_64 != 0 {
        (*insn).addr_bytes = 8;
    } else {
        (*insn).addr_bytes = 4;
    }
}

static xen_prefix: [insn_byte_t; XEN_EMULATE_PREFIX_LEN] = __XEN_EMULATE_PREFIX;
static kvm_prefix: [insn_byte_t; KVM_EMULATE_PREFIX_LEN] = __KVM_EMULATE_PREFIX;

unsafe fn __insn_get_emulate_prefix(
    insn: *mut insn,
    prefix: *const insn_byte_t,
    len: usize,
) -> i32 {
    let mut i: usize = 0;

    while i < len {
        if peek_nbyte_next_u8(insn, i).unwrap_or_default() != *prefix.add(i) {
            return 0;
        }
        i += 1;
    }

    (*insn).emulate_prefix_size = len;
    (*insn).next_byte = (*insn).next_byte.add(len);

    1
}

unsafe fn insn_get_emulate_prefix(insn: *mut insn) {
    if __insn_get_emulate_prefix(insn, xen_prefix.as_ptr(), xen_prefix.len()) != 0 {
        return;
    }

    __insn_get_emulate_prefix(insn, kvm_prefix.as_ptr(), kvm_prefix.len());
}

/**
 * insn_get_prefixes - scan x86 instruction prefix bytes
 * @insn:	&struct insn containing instruction
 *
 * Populates the @insn->prefixes bitmap, and updates @insn->next_byte
 * to point to the (first) opcode.  No effect if @insn->prefixes.got
 * is already set.
 *
 * * Returns:
 * 0:  on success
 * < 0: on error
 */
pub unsafe extern "C" fn insn_get_prefixes(insn: *mut insn) -> i32 {
    let prefixes: *mut insn_field = &mut (*insn).prefixes;
    let mut attr: insn_attr_t;
    let mut b: insn_byte_t;
    let mut lb: insn_byte_t;
    let mut i: i32;
    let mut nb: i32;

    if (*prefixes).got != 0 {
        return 0;
    }

    insn_get_emulate_prefix(insn);

    nb = 0;
    lb = 0;
    b = match peek_next_u8(insn) {
        Ok(v) => v,
        Err(e) => return e,
    };
    attr = inat_get_opcode_attribute(b);
    while inat_is_legacy_prefix(attr) != 0 {
        /* Skip if same prefix */
        i = 0;
        while i < nb {
            if (*prefixes).bytes[i as usize] == b {
                break;
            }
            i += 1;
        }
        if i == nb {
            if nb == 4 {
                /* Invalid instruction */
                break;
            }
            (*prefixes).bytes[nb as usize] = b;
            nb += 1;
            if inat_is_address_size_prefix(attr) != 0 {
                /* address size switches 2/4 or 4/8 */
                if (*insn).x86_64 != 0 {
                    (*insn).addr_bytes ^= 12;
                } else {
                    (*insn).addr_bytes ^= 6;
                }
            } else if inat_is_operand_size_prefix(attr) != 0 {
                /* oprand size switches 2/4 */
                (*insn).opnd_bytes ^= 6;
            }
        }
        (*prefixes).nbytes += 1;
        (*insn).next_byte = (*insn).next_byte.add(1);
        lb = b;
        b = match peek_next_u8(insn) {
            Ok(v) => v,
            Err(e) => return e,
        };
        attr = inat_get_opcode_attribute(b);
    }
    /* Set the last prefix */
    if lb != 0 && lb != (*insn).prefixes.bytes[3] {
        if (*insn).prefixes.bytes[3] != 0 {
            /* Swap the last prefix */
            b = (*insn).prefixes.bytes[3];
            i = 0;
            while i < nb {
                if (*prefixes).bytes[i as usize] == lb {
                    insn_set_byte(prefixes, i, b);
                }
                i += 1;
            }
        }
        insn_set_byte(&mut (*insn).prefixes, 3, lb);
    }

    /* Decode REX prefix */
    if (*insn).x86_64 != 0 {
        b = match peek_next_u8(insn) {
            Ok(v) => v,
            Err(e) => return e,
        };
        attr = inat_get_opcode_attribute(b);
        if inat_is_rex_prefix(attr) != 0 {
            insn_field_set(&mut (*insn).rex_prefix, b as i64, 1);
            (*insn).next_byte = (*insn).next_byte.add(1);
            if X86_REX_W(b) != 0 {
                /* REX.W overrides opnd_size */
                (*insn).opnd_bytes = 8;
            }
        } else if inat_is_rex2_prefix(attr) != 0 {
            insn_set_byte(&mut (*insn).rex_prefix, 0, b);
            b = match peek_nbyte_next_u8(insn, 1) {
                Ok(v) => v,
                Err(e) => return e,
            };
            insn_set_byte(&mut (*insn).rex_prefix, 1, b);
            (*insn).rex_prefix.nbytes = 2;
            (*insn).next_byte = (*insn).next_byte.add(2);
            if X86_REX_W(b) != 0 {
                /* REX.W overrides opnd_size */
                (*insn).opnd_bytes = 8;
            }
            (*insn).rex_prefix.got = 1;
            (*insn).vex_prefix.got = 1;
            (*prefixes).got = 1;
            return 0;
        }
    }
    (*insn).rex_prefix.got = 1;

    /* Decode VEX/XOP prefix */
    b = match peek_next_u8(insn) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if inat_is_vex_prefix(attr) != 0 || inat_is_xop_prefix(attr) != 0 {
        let mut b2: insn_byte_t = match peek_nbyte_next_u8(insn, 1) {
            Ok(v) => v,
            Err(e) => return e,
        };

        if inat_is_xop_prefix(attr) != 0 && X86_MODRM_REG(b2) == 0 {
            /* Grp1A.0 is always POP Ev */
        } else if (*insn).x86_64 == 0 && X86_MODRM_MOD(b2) != 3 {
            /*
             * In 32-bits mode, if the [7:6] bits (mod bits of
             * ModRM) on the second byte are not 11b, it is
             * LDS or LES or BOUND.
             */
        } else {
            insn_set_byte(&mut (*insn).vex_prefix, 0, b);
            insn_set_byte(&mut (*insn).vex_prefix, 1, b2);
            if inat_is_evex_prefix(attr) != 0 {
                b2 = match peek_nbyte_next_u8(insn, 2) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                insn_set_byte(&mut (*insn).vex_prefix, 2, b2);
                b2 = match peek_nbyte_next_u8(insn, 3) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                insn_set_byte(&mut (*insn).vex_prefix, 3, b2);
                (*insn).vex_prefix.nbytes = 4;
                (*insn).next_byte = (*insn).next_byte.add(4);
                if (*insn).x86_64 != 0 && X86_VEX_W(b2) != 0 {
                    /* VEX.W overrides opnd_size */
                    (*insn).opnd_bytes = 8;
                }
            } else if inat_is_vex3_prefix(attr) != 0 || inat_is_xop_prefix(attr) != 0 {
                b2 = match peek_nbyte_next_u8(insn, 2) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                insn_set_byte(&mut (*insn).vex_prefix, 2, b2);
                (*insn).vex_prefix.nbytes = 3;
                (*insn).next_byte = (*insn).next_byte.add(3);
                if (*insn).x86_64 != 0 && X86_VEX_W(b2) != 0 {
                    /* VEX.W/XOP.W overrides opnd_size */
                    (*insn).opnd_bytes = 8;
                }
            } else {
                /*
                 * For VEX2, fake VEX3-like byte#2.
                 * Makes it easier to decode vex.W, vex.vvvv,
                 * vex.L and vex.pp. Masking with 0x7f sets vex.W == 0.
                 */
                insn_set_byte(&mut (*insn).vex_prefix, 2, b2 & 0x7f);
                (*insn).vex_prefix.nbytes = 2;
                (*insn).next_byte = (*insn).next_byte.add(2);
            }
        }
    }
    (*insn).vex_prefix.got = 1;

    (*prefixes).got = 1;

    0
}

/**
 * insn_get_opcode - collect opcode(s)
 * @insn:	&struct insn containing instruction
 *
 * Populates @insn->opcode, updates @insn->next_byte to point past the
 * opcode byte(s), and set @insn->attr (except for groups).
 * If necessary, first collects any preceding (prefix) bytes.
 * Sets @insn->opcode.value = opcode1.  No effect if @insn->opcode.got
 * is already 1.
 *
 * Returns:
 * 0:  on success
 * < 0: on error
 */
pub unsafe extern "C" fn insn_get_opcode(insn: *mut insn) -> i32 {
    let opcode: *mut insn_field = &mut (*insn).opcode;
    let mut pfx_id: i32;
    let mut ret: i32;
    let mut op: insn_byte_t;

    if (*opcode).got != 0 {
        return 0;
    }

    ret = insn_get_prefixes(insn);
    if ret != 0 {
        return ret;
    }

    /* Get first opcode */
    op = match get_next_u8(insn) {
        Ok(v) => v,
        Err(e) => return e,
    };
    insn_set_byte(opcode, 0, op);
    (*opcode).nbytes = 1;

    /* Check if there is VEX/XOP prefix or not */
    if insn_is_avx_or_xop(insn) != 0 {
        let m: insn_byte_t;
        let p: insn_byte_t;

        /* XOP prefix has different encoding */
        if avx_insn_is_xop(insn) != 0 {
            m = insn_xop_map_bits(insn);
            (*insn).attr = inat_get_xop_attribute(op, m);
            if inat_accept_xop((*insn).attr) == 0 {
                (*insn).attr = 0;
                return -EINVAL;
            }
            /* XOP has only 1 byte for opcode */
            (*opcode).got = 1;
            return 0;
        }

        m = insn_vex_m_bits(insn);
        p = insn_vex_p_bits(insn);
        (*insn).attr = inat_get_avx_attribute(op, m, p);
        /* SCALABLE EVEX uses p bits to encode operand size */
        if inat_evex_scalable((*insn).attr) != 0 && insn_vex_w_bit(insn) == 0 && p == INAT_PFX_OPNDSZ {
            (*insn).opnd_bytes = 2;
        }
        if (inat_must_evex((*insn).attr) != 0 && insn_is_evex(insn) == 0)
            || (inat_accept_vex((*insn).attr) == 0 && inat_is_group((*insn).attr) == 0)
        {
            /* This instruction is bad */
            (*insn).attr = 0;
            return -EINVAL;
        }
        /* VEX has only 1 byte for opcode */
        (*opcode).got = 1;
        return 0;
    }

    /* Check if there is REX2 prefix or not */
    if insn_is_rex2(insn) != 0 {
        if insn_rex2_m_bit(insn) != 0 {
            /* map 1 is escape 0x0f */
            let esc_attr: insn_attr_t = inat_get_opcode_attribute(0x0f);

            pfx_id = insn_last_prefix_id(insn);
            (*insn).attr = inat_get_escape_attribute(op, pfx_id, esc_attr);
        } else {
            (*insn).attr = inat_get_opcode_attribute(op);
        }
        (*opcode).got = 1;
        return 0;
    }

    (*insn).attr = inat_get_opcode_attribute(op);
    if (*insn).x86_64 != 0 && inat_is_invalid64((*insn).attr) != 0 {
        /* This instruction is invalid, like UD2. Stop decoding. */
        (*insn).attr &= INAT_INV64;
    }

    while inat_is_escape((*insn).attr) != 0 {
        /* Get escaped opcode */
        op = match get_next_u8(insn) {
            Ok(v) => v,
            Err(e) => return e,
        };
        (*opcode).bytes[(*opcode).nbytes as usize] = op;
        (*opcode).nbytes += 1;
        pfx_id = insn_last_prefix_id(insn);
        (*insn).attr = inat_get_escape_attribute(op, pfx_id, (*insn).attr);
    }

    if inat_must_vex((*insn).attr) != 0 {
        /* This instruction is bad */
        (*insn).attr = 0;
        return -EINVAL;
    }

    (*opcode).got = 1;
    0
}

/**
 * insn_get_modrm - collect ModRM byte, if any
 * @insn:	&struct insn containing instruction
 *
 * Populates @insn->modrm and updates @insn->next_byte to point past the
 * ModRM byte, if any.  If necessary, first collects the preceding bytes
 * (prefixes and opcode(s)).  No effect if @insn->modrm.got is already 1.
 *
 * Returns:
 * 0:  on success
 * < 0: on error
 */
pub unsafe extern "C" fn insn_get_modrm(insn: *mut insn) -> i32 {
    let modrm: *mut insn_field = &mut (*insn).modrm;
    let pfx_id: insn_byte_t;
    let mod_byte: insn_byte_t;
    let ret: i32;

    if (*modrm).got != 0 {
        return 0;
    }

    ret = insn_get_opcode(insn);
    if ret != 0 {
        return ret;
    }

    if inat_has_modrm((*insn).attr) != 0 {
        mod_byte = match get_next_u8(insn) {
            Ok(v) => v,
            Err(e) => return e,
        };
        insn_field_set(modrm, mod_byte as i64, 1);
        if inat_is_group((*insn).attr) != 0 {
            pfx_id = insn_last_prefix_id(insn) as insn_byte_t;
            (*insn).attr = inat_get_group_attribute(mod_byte, pfx_id, (*insn).attr);
            if insn_is_avx_or_xop(insn) != 0
                && inat_accept_vex((*insn).attr) == 0
                && inat_accept_xop((*insn).attr) == 0
            {
                /* Bad insn */
                (*insn).attr = 0;
                return -EINVAL;
            }
        }
    }

    if (*insn).x86_64 != 0 && inat_is_force64((*insn).attr) != 0 {
        (*insn).opnd_bytes = 8;
    }

    (*modrm).got = 1;
    0
}

/**
 * insn_rip_relative() - Does instruction use RIP-relative addressing mode?
 * @insn:	&struct insn containing instruction
 *
 * If necessary, first collects the instruction up to and including the
 * ModRM byte.  No effect if @insn->x86_64 is 0.
 */
pub unsafe extern "C" fn insn_rip_relative(insn: *mut insn) -> i32 {
    let modrm: *mut insn_field = &mut (*insn).modrm;
    let ret: i32;

    if (*insn).x86_64 == 0 {
        return 0;
    }

    ret = insn_get_modrm(insn);
    if ret != 0 {
        return 0;
    }
    /*
     * For rip-relative instructions, the mod field (top 2 bits)
     * is zero and the r/m field (bottom 3 bits) is 0x5.
     */
    ((*modrm).nbytes != 0 && ((*modrm).bytes[0] & 0xc7) == 0x5) as i32
}

/**
 * insn_get_sib() - Get the SIB byte of instruction
 * @insn:	&struct insn containing instruction
 *
 * If necessary, first collects the instruction up to and including the
 * ModRM byte.
 *
 * Returns:
 * 0: if decoding succeeded
 * < 0: otherwise.
 */
pub unsafe extern "C" fn insn_get_sib(insn: *mut insn) -> i32 {
    let modrm: insn_byte_t;
    let ret: i32;

    if (*insn).sib.got != 0 {
        return 0;
    }

    ret = insn_get_modrm(insn);
    if ret != 0 {
        return ret;
    }

    if (*insn).modrm.nbytes != 0 {
        modrm = (*insn).modrm.bytes[0];
        if (*insn).addr_bytes != 2 && X86_MODRM_MOD(modrm) != 3 && X86_MODRM_RM(modrm) == 4 {
            let v = match get_next_u8(insn) {
                Ok(v) => v,
                Err(e) => return e,
            };
            insn_field_set(&mut (*insn).sib, v as i64, 1);
        }
    }
    (*insn).sib.got = 1;

    0
}

/**
 * insn_get_displacement() - Get the displacement of instruction
 * @insn:	&struct insn containing instruction
 *
 * If necessary, first collects the instruction up to and including the
 * SIB byte.
 * Displacement value is sign-expanded.
 *
 * * Returns:
 * 0: if decoding succeeded
 * < 0: otherwise.
 */
pub unsafe extern "C" fn insn_get_displacement(insn: *mut insn) -> i32 {
    let mod_byte: insn_byte_t;
    let rm: insn_byte_t;
    let base: insn_byte_t;
    let ret: i32;

    if (*insn).displacement.got != 0 {
        return 0;
    }

    ret = insn_get_sib(insn);
    if ret != 0 {
        return ret;
    }

    if (*insn).modrm.nbytes != 0 {
        /*
         * Interpreting the modrm byte:
         * mod = 00 - no displacement fields (exceptions below)
         * mod = 01 - 1-byte displacement field
         * mod = 10 - displacement field is 4 bytes, or 2 bytes if
         * 	address size = 2 (0x67 prefix in 32-bit mode)
         * mod = 11 - no memory operand
         *
         * If address size = 2...
         * mod = 00, r/m = 110 - displacement field is 2 bytes
         *
         * If address size != 2...
         * mod != 11, r/m = 100 - SIB byte exists
         * mod = 00, SIB base = 101 - displacement field is 4 bytes
         * mod = 00, r/m = 101 - rip-relative addressing, displacement
         * 	field is 4 bytes
         */
        mod_byte = X86_MODRM_MOD((*insn).modrm.value as insn_byte_t);
        rm = X86_MODRM_RM((*insn).modrm.value as insn_byte_t);
        base = X86_SIB_BASE((*insn).sib.value as insn_byte_t);
        if mod_byte != 3 {
            if mod_byte == 1 {
                let v = match get_next_i8(insn) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                insn_field_set(&mut (*insn).displacement, v as i64, 1);
            } else if (*insn).addr_bytes == 2 {
                if (mod_byte == 0 && rm == 6) || mod_byte == 2 {
                    let v = match get_next_i16(insn) {
                        Ok(v) => v,
                        Err(e) => return e,
                    };
                    insn_field_set(&mut (*insn).displacement, v as i64, 2);
                }
            } else if (mod_byte == 0 && rm == 5) || mod_byte == 2 || (mod_byte == 0 && base == 5) {
                let v = match get_next_i32(insn) {
                    Ok(v) => v,
                    Err(e) => return e,
                };
                insn_field_set(&mut (*insn).displacement, v as i64, 4);
            }
        }
    }
    (*insn).displacement.got = 1;
    0
}

/* Decode moffset16/32/64. Return 0 if failed */
unsafe fn __get_moffset(insn: *mut insn) -> i32 {
    match (*insn).addr_bytes {
        2 => {
            let v = match get_next_i16(insn) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            insn_field_set(&mut (*insn).moffset1, v as i64, 2);
        }
        4 => {
            let v = match get_next_i32(insn) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            insn_field_set(&mut (*insn).moffset1, v as i64, 4);
        }
        8 => {
            let v1 = match get_next_i32(insn) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            insn_field_set(&mut (*insn).moffset1, v1 as i64, 4);
            let v2 = match get_next_i32(insn) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            insn_field_set(&mut (*insn).moffset2, v2 as i64, 4);
        }
        _ => {
            /* opnd_bytes must be modified manually */
            return 0;
        }
    }
    (*insn).moffset1.got = 1;
    (*insn).moffset2.got = 1;

    1
}

/* Decode imm v32(Iz). Return 0 if failed */
unsafe fn __get_immv32(insn: *mut insn) -> i32 {
    match (*insn).opnd_bytes {
        2 => {
            let v = match get_next_i16(insn) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            insn_field_set(&mut (*insn).immediate, v as i64, 2);
        }
        4 | 8 => {
            let v = match get_next_i32(insn) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            insn_field_set(&mut (*insn).immediate, v as i64, 4);
        }
        _ => {
            /* opnd_bytes must be modified manually */
            return 0;
        }
    }

    1
}

/* Decode imm v64(Iv/Ov), Return 0 if failed */
unsafe fn __get_immv(insn: *mut insn) -> i32 {
    match (*insn).opnd_bytes {
        2 => {
            let v = match get_next_i16(insn) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            insn_field_set(&mut (*insn).immediate1, v as i64, 2);
        }
        4 => {
            let v = match get_next_i32(insn) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            insn_field_set(&mut (*insn).immediate1, v as i64, 4);
            (*insn).immediate1.nbytes = 4;
        }
        8 => {
            let v1 = match get_next_i32(insn) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            insn_field_set(&mut (*insn).immediate1, v1 as i64, 4);
            let v2 = match get_next_i32(insn) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            insn_field_set(&mut (*insn).immediate2, v2 as i64, 4);
        }
        _ => {
            /* opnd_bytes must be modified manually */
            return 0;
        }
    }
    (*insn).immediate1.got = 1;
    (*insn).immediate2.got = 1;

    1
}

/* Decode ptr16:16/32(Ap) */
unsafe fn __get_immptr(insn: *mut insn) -> i32 {
    match (*insn).opnd_bytes {
        2 => {
            let v = match get_next_i16(insn) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            insn_field_set(&mut (*insn).immediate1, v as i64, 2);
        }
        4 => {
            let v = match get_next_i32(insn) {
                Ok(v) => v,
                Err(_) => return 0,
            };
            insn_field_set(&mut (*insn).immediate1, v as i64, 4);
        }
        8 => {
            /* ptr16:64 is not exist (no segment) */
            return 0;
        }
        _ => {
            /* opnd_bytes must be modified manually */
            return 0;
        }
    }
    let v = match get_next_u16(insn) {
        Ok(v) => v,
        Err(_) => return 0,
    };
    insn_field_set(&mut (*insn).immediate2, v as i64, 2);
    (*insn).immediate1.got = 1;
    (*insn).immediate2.got = 1;

    1
}

/**
 * insn_get_immediate() - Get the immediate in an instruction
 * @insn:	&struct insn containing instruction
 *
 * If necessary, first collects the instruction up to and including the
 * displacement bytes.
 * Basically, most of immediates are sign-expanded. Unsigned-value can be
 * computed by bit masking with ((1 << (nbytes * 8)) - 1)
 *
 * Returns:
 * 0:  on success
 * < 0: on error
 */
pub unsafe extern "C" fn insn_get_immediate(insn: *mut insn) -> i32 {
    let ret: i32;

    if (*insn).immediate.got != 0 {
        return 0;
    }

    ret = insn_get_displacement(insn);
    if ret != 0 {
        return ret;
    }

    if inat_has_moffset((*insn).attr) != 0 {
        if __get_moffset(insn) == 0 {
            return -ENODATA;
        }
        (*insn).immediate.got = 1;
        return 0;
    }

    if inat_has_immediate((*insn).attr) == 0 {
        (*insn).immediate.got = 1;
        return 0;
    }

    match inat_immediate_size((*insn).attr) {
        INAT_IMM_BYTE => {
            let v = match get_next_i8(insn) {
                Ok(v) => v,
                Err(e) => return e,
            };
            insn_field_set(&mut (*insn).immediate, v as i64, 1);
        }
        INAT_IMM_WORD => {
            let v = match get_next_i16(insn) {
                Ok(v) => v,
                Err(e) => return e,
            };
            insn_field_set(&mut (*insn).immediate, v as i64, 2);
        }
        INAT_IMM_DWORD => {
            let v = match get_next_i32(insn) {
                Ok(v) => v,
                Err(e) => return e,
            };
            insn_field_set(&mut (*insn).immediate, v as i64, 4);
        }
        INAT_IMM_QWORD => {
            let v1 = match get_next_i32(insn) {
                Ok(v) => v,
                Err(e) => return e,
            };
            insn_field_set(&mut (*insn).immediate1, v1 as i64, 4);
            let v2 = match get_next_i32(insn) {
                Ok(v) => v,
                Err(e) => return e,
            };
            insn_field_set(&mut (*insn).immediate2, v2 as i64, 4);
        }
        INAT_IMM_PTR => {
            if __get_immptr(insn) == 0 {
                return -ENODATA;
            }
        }
        INAT_IMM_VWORD32 => {
            if __get_immv32(insn) == 0 {
                return -ENODATA;
            }
        }
        INAT_IMM_VWORD => {
            if __get_immv(insn) == 0 {
                return -ENODATA;
            }
        }
        _ => {
            /* Here, insn must have an immediate, but failed */
            return -ENODATA;
        }
    }
    if inat_has_second_immediate((*insn).attr) != 0 {
        let v = match get_next_i8(insn) {
            Ok(v) => v,
            Err(e) => return e,
        };
        insn_field_set(&mut (*insn).immediate2, v as i64, 1);
    }
    (*insn).immediate.got = 1;
    0
}

/**
 * insn_get_length() - Get the length of instruction
 * @insn:	&struct insn containing instruction
 *
 * If necessary, first collects the instruction up to and including the
 * immediates bytes.
 *
 * Returns:
 *  - 0 on success
 *  - < 0 on error
*/
pub unsafe extern "C" fn insn_get_length(insn: *mut insn) -> i32 {
    let ret: i32;

    if (*insn).length != 0 {
        return 0;
    }

    ret = insn_get_immediate(insn);
    if ret != 0 {
        return ret;
    }

    (*insn).length = ((*insn).next_byte as usize).wrapping_sub((*insn).kaddr as usize) as u8;

    0
}

/* Ensure this instruction is decoded completely */
unsafe fn insn_complete(insn: *mut insn) -> i32 {
    ((*insn).opcode.got != 0
        && (*insn).modrm.got != 0
        && (*insn).sib.got != 0
        && (*insn).displacement.got != 0
        && (*insn).immediate.got != 0) as i32
}

/**
 * insn_decode() - Decode an x86 instruction
 * @insn:	&struct insn to be initialized
 * @kaddr:	address (in kernel memory) of instruction (or copy thereof)
 * @buf_len:	length of the insn buffer at @kaddr
 * @m:		insn mode, see enum insn_mode
 *
 * Returns:
 * 0: if decoding succeeded
 * < 0: otherwise.
 */
pub unsafe extern "C" fn insn_decode(
    insn: *mut insn,
    kaddr: *const core::ffi::c_void,
    buf_len: i32,
    m: insn_mode,
) -> i32 {
    let ret: i32;

    const INSN_MODE_KERN: insn_mode = -1 as insn_mode; /* __ignore_sync_check__ mode is only valid in the kernel */

    if m == INSN_MODE_KERN {
        insn_init(insn, kaddr, buf_len, IS_ENABLED_CONFIG_X86_64);
    } else {
        insn_init(insn, kaddr, buf_len, (m == INSN_MODE_64) as i32);
    }

    ret = insn_get_length(insn);
    if ret != 0 {
        return ret;
    }

    if insn_complete(insn) != 0 {
        return 0;
    }

    -EINVAL
}
