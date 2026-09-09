/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright IBM Corp. 2008
 *
 * Authors: Hollis Blanchard <hollisb@us.ibm.com>
 */

// Dependency: u32 is supplied by the surrounding kernel translation.

#[inline]
pub fn get_op(inst: u32) -> u32 {
    inst >> 26
}

#[inline]
pub fn get_xop(inst: u32) -> u32 {
    (inst >> 1) & 0x3ff
}

#[inline]
pub fn get_sprn(inst: u32) -> u32 {
    ((inst >> 16) & 0x1f) | ((inst >> 6) & 0x3e0)
}

#[inline]
pub fn get_dcrn(inst: u32) -> u32 {
    ((inst >> 16) & 0x1f) | ((inst >> 6) & 0x3e0)
}

#[inline]
pub fn get_tmrn(inst: u32) -> u32 {
    ((inst >> 16) & 0x1f) | ((inst >> 6) & 0x3e0)
}

#[inline]
pub fn get_rt(inst: u32) -> u32 {
    (inst >> 21) & 0x1f
}

#[inline]
pub fn get_rs(inst: u32) -> u32 {
    (inst >> 21) & 0x1f
}

#[inline]
pub fn get_ra(inst: u32) -> u32 {
    (inst >> 16) & 0x1f
}

#[inline]
pub fn get_rb(inst: u32) -> u32 {
    (inst >> 11) & 0x1f
}

#[inline]
pub fn get_rc(inst: u32) -> u32 {
    inst & 0x1
}

#[inline]
pub fn get_ws(inst: u32) -> u32 {
    (inst >> 11) & 0x1f
}

#[inline]
pub fn get_d(inst: u32) -> u32 {
    inst & 0xffff
}

#[inline]
pub fn get_oc(inst: u32) -> u32 {
    (inst >> 11) & 0x7fff
}

#[inline]
pub fn get_tx_or_sx(inst: u32) -> u32 {
    inst & 0x1
}

#[inline]
pub fn is_xform(inst: u32) -> bool {
    get_op(inst) == 31
}

#[inline]
pub fn is_dsform(inst: u32) -> bool {
    get_op(inst) >= 56
}

/*
 * Create a DSISR value from the instruction
 */
#[inline]
pub fn make_dsisr(instr: u32) -> u32 {
    let mut dsisr: u32;

    /* bits  6:15 --> 22:31 */
    dsisr = (instr & 0x03ff0000) >> 16;

    if is_xform(instr) {
        /* bits 29:30 --> 15:16 */
        dsisr |= (instr & 0x00000006) << 14;
        /* bit     25 -->    17 */
        dsisr |= (instr & 0x00000040) << 8;
        /* bits 21:24 --> 18:21 */
        dsisr |= (instr & 0x00000780) << 3;
    } else {
        /* bit      5 -->    17 */
        dsisr |= (instr & 0x04000000) >> 12;
        /* bits  1: 4 --> 18:21 */
        dsisr |= (instr & 0x78000000) >> 17;
        /* bits 30:31 --> 12:13 */
        if is_dsform(instr) {
            dsisr |= (instr & 0x00000003) << 18;
        }
    }

    dsisr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
