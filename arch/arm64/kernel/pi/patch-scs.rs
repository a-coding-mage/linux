// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2022 - Google LLC
 * Author: Ard Biesheuvel <ardb@google.com>
 */

use core::mem::offset_of;
use core::ptr;

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    fn le32_to_cpup(p: *const u32) -> u32;
    fn cpu_to_le32(v: u32) -> u32;
    fn strcmp(a: *const u8, b: *const u8) -> i32;
}

const EDYNSCS_INVALID_FDE_AUGM_DATA_SIZE: i32 = -1;
const EDYNSCS_INVALID_CFA_OPCODE: i32 = -2;
const EDYNSCS_INVALID_CIE_HEADER: i32 = -3;
const EDYNSCS_INVALID_CIE_SDATA_SIZE: i32 = -4;

pub static mut dynamic_scs_is_enabled: bool = false;

//
// This minimal DWARF CFI parser is partially based on the code in
// arch/arc/kernel/unwind.c, and on the document below:
// https://refspecs.linuxbase.org/LSB_4.0.0/LSB-Core-generic/LSB-Core-generic/ehframechpt.html
//

const DW_CFA_nop: u8 = 0x00;
const DW_CFA_advance_loc1: u8 = 0x02;
const DW_CFA_advance_loc2: u8 = 0x03;
const DW_CFA_advance_loc4: u8 = 0x04;
const DW_CFA_offset_extended: u8 = 0x05;
const DW_CFA_restore_extended: u8 = 0x06;
const DW_CFA_same_value: u8 = 0x08;
const DW_CFA_remember_state: u8 = 0x0a;
const DW_CFA_restore_state: u8 = 0x0b;
const DW_CFA_def_cfa: u8 = 0x0c;
const DW_CFA_def_cfa_register: u8 = 0x0d;
const DW_CFA_def_cfa_offset: u8 = 0x0e;
const DW_CFA_def_cfa_offset_sf: u8 = 0x13;
const DW_CFA_negate_ra_state: u8 = 0x2d;
const DW_EH_PE_sdata4: u8 = 0x0b;
const DW_EH_PE_sdata8: u8 = 0x0c;
const DW_EH_PE_pcrel: u8 = 0x10;

const PACIASP: u32 = 0xd503233f;
const AUTIASP: u32 = 0xd50323bf;
const SCS_PUSH: u32 = 0xf800865e;
const SCS_POP: u32 = 0xf85f8e5e;

#[inline(always)]
unsafe fn scs_patch_loc(loc: u64) {
    let insn = le32_to_cpup(loc as *const u32);
    match insn {
        PACIASP => ptr::write(loc as *mut u32, cpu_to_le32(SCS_PUSH)),
        AUTIASP => ptr::write(loc as *mut u32, cpu_to_le32(SCS_POP)),
        _ => {
            /*
             * While the DW_CFA_negate_ra_state directive is guaranteed to
             * appear right after a PACIASP/AUTIASP instruction, it may
             * also appear after a DW_CFA_restore_state directive that
             * restores a state that is only partially accurate, and is
             * followed by DW_CFA_negate_ra_state directive to toggle the
             * PAC bit again. So we permit other instructions here, and
             * ignore them.
             */
            return;
        }
    }
    // Cache maintenance assembly is configuration-dependent in the kernel.
    // CONFIG_ARM64_WORKAROUND_CLEAN_CACHE / ALTERNATIVE are supplied externally.
}

/*
 * Skip one uleb128/sleb128 encoded quantity from the opcode stream. All bytes
 * except the last one have bit #7 set.
 */
#[inline(always)]
unsafe fn skip_xleb128(opcode: &mut *const u8, mut size: i32) -> i32 {
    let mut c;
    loop {
        c = **opcode;
        *opcode = opcode.add(1);
        size -= 1;
        if c & (1 << 7) == 0 { break; }
    }
    size
}

#[repr(C)]
pub struct EhFrame {
    pub size: u32,
    pub cie_id_or_pointer: u32,
    pub data: [u8; 0],
}

#[inline(always)]
unsafe fn scs_handle_fde_frame(frame: *const EhFrame, code_alignment_factor: i32,
                               use_sdata8: bool, dry_run: bool) -> i32 {
    let mut size = (*frame).size as i32 - offset_of!(EhFrame, data) as i32 + 4;
    let mut loc = frame as u64 + offset_of!(EhFrame, data) as u64 - 8;
    let mut opcode = (frame as *const u8).add(16);
    if use_sdata8 {
        let initial = (frame as *const u8).add(8) as *const i64;
        loc = initial as u64 + ptr::read_unaligned(initial) as u64;
        opcode = opcode.add(16);
        size -= 8;
    }
    if *opcode & (1 << 7) != 0 { return EDYNSCS_INVALID_FDE_AUGM_DATA_SIZE; }
    let l = *opcode as i32;
    opcode = opcode.add(1 + l as usize);
    size -= l + 1;
    while size > 0 {
        size -= 1;
        match *opcode { 
            DW_CFA_nop | DW_CFA_remember_state | DW_CFA_restore_state => { opcode = opcode.add(1); }
            DW_CFA_advance_loc1 => { opcode=opcode.add(1); loc += *opcode as u64 * code_alignment_factor as u64; opcode=opcode.add(1); size-=1; }
            DW_CFA_advance_loc2 => { opcode=opcode.add(1); loc += *opcode as u64 * code_alignment_factor as u64; opcode=opcode.add(1); loc += (*opcode as u64)<<8; opcode=opcode.add(1); size-=2; }
            DW_CFA_advance_loc4 => { opcode=opcode.add(1); for shift in [0,8,16,24] { loc += (*opcode as u64)<<shift; opcode=opcode.add(1); } size-=4; }
            DW_CFA_def_cfa | DW_CFA_offset_extended => { opcode=opcode.add(1); size=skip_xleb128(&mut opcode,size); opcode=opcode.add(0); size=skip_xleb128(&mut opcode,size); }
            DW_CFA_def_cfa_offset | DW_CFA_def_cfa_offset_sf | DW_CFA_def_cfa_register | DW_CFA_same_value | DW_CFA_restore_extended | 0x80..=0xbf => { opcode=opcode.add(1); size=skip_xleb128(&mut opcode,size); }
            DW_CFA_negate_ra_state => { opcode=opcode.add(1); if !dry_run { scs_patch_loc(loc-4); } }
            0x40..=0x7f => { loc += ((*opcode & 0x3f) as u64) * code_alignment_factor as u64; opcode=opcode.add(1); }
            0xc0..=0xff => { opcode=opcode.add(1); }
            _ => return EDYNSCS_INVALID_CFA_OPCODE,
        }
    }
    0
}

pub unsafe fn scs_patch(eh_frame: *const u8, mut size: i32, skip_dry_run: bool) -> i32 {
    let mut code_alignment_factor = 1;
    let mut fde_use_sdata8 = false;
    let mut p = eh_frame;
    while size > 4 {
        let frame = p as *const EhFrame;
        if (*frame).size == 0 || (*frame).size == u32::MAX || (*frame).size > size as u32 { break; }
        if (*frame).cie_id_or_pointer == 0 {
            let aug = p.add(9);
            if strcmp(aug, b"zR\0".as_ptr()) != 0 { return EDYNSCS_INVALID_CIE_HEADER; }
            let fields = p.add(12);
            if *fields & 0x80 != 0 || *fields.add(1) & 0x80 != 0 || *fields.add(2) != 30 || *fields.add(3) != 1 { return EDYNSCS_INVALID_CIE_HEADER; }
            code_alignment_factor = *fields as i32;
            match *fields.add(4) { x if x == (DW_EH_PE_pcrel|DW_EH_PE_sdata4) => fde_use_sdata8=false, x if x == (DW_EH_PE_pcrel|DW_EH_PE_sdata8) => fde_use_sdata8=true, _ => return EDYNSCS_INVALID_CIE_SDATA_SIZE }
        } else {
            let ret = scs_handle_fde_frame(frame, code_alignment_factor, fde_use_sdata8, !skip_dry_run);
            if ret != 0 { return ret; }
            if !skip_dry_run { scs_handle_fde_frame(frame, code_alignment_factor, fde_use_sdata8, false); }
        }
        let n = 4 + (*frame).size as i32; p=p.add(n as usize); size-=n;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
