/*
 * Derived from (and probably identical to):
 * ftl.h 1.7 1999/10/25 20:23:17
 *
 * The contents of this file are subject to the Mozilla Public License
 * Version 1.1 (the "License"); you may not use this file except in
 * compliance with the License. You may obtain a copy of the License
 * at http://www.mozilla.org/MPL/
 *
 * Software distributed under the License is distributed on an "AS IS"
 * basis, WITHOUT WARRANTY OF ANY KIND, either express or implied. See
 * the License for the specific language governing rights and
 * limitations under the License.
 *
 * The initial developer of the original code is David A. Hinds
 * <dahinds@users.sourceforge.net>.  Portions created by David A. Hinds
 * are Copyright (C) 1999 David A. Hinds.  All Rights Reserved.
 *
 * Alternatively, the contents of this file may be used under the
 * terms of the GNU General Public License version 2 (the "GPL"), in
 * which case the provisions of the GPL are applicable instead of the
 * above.  If you wish to allow the use of your version of this file
 * only under the terms of the GPL and not to allow others to use
 * your version of this file under the MPL, indicate your decision by
 * deleting the provisions above and replace them with the notice and
 * other provisions required by the GPL.  If you do not delete the
 * provisions above, a recipient may use your version of this file
 * under either the MPL or the GPL.
 */

#[repr(C)]
pub struct erase_unit_header_t {
    pub LinkTargetTuple: [u8; 5],
    pub DataOrgTuple: [u8; 10],
    pub NumTransferUnits: u8,
    pub EraseCount: u32,
    pub LogicalEUN: u16,
    pub BlockSize: u8,
    pub EraseUnitSize: u8,
    pub FirstPhysicalEUN: u16,
    pub NumEraseUnits: u16,
    pub FormattedSize: u32,
    pub FirstVMAddress: u32,
    pub NumVMPages: u16,
    pub Flags: u8,
    pub Code: u8,
    pub SerialNumber: u32,
    pub AltEUHOffset: u32,
    pub BAMOffset: u32,
    pub Reserved: [u8; 12],
    pub EndTuple: [u8; 2],
}

/* Flags in erase_unit_header_t */
pub const HIDDEN_AREA: u8 = 0x01;
pub const REVERSE_POLARITY: u8 = 0x02;
pub const DOUBLE_BAI: u8 = 0x04;

/* Definitions for block allocation information */

#[inline]
pub const fn BLOCK_FREE(b: u32) -> bool {
    b == 0xffff_ffff
}

#[inline]
pub const fn BLOCK_DELETED(b: u32) -> bool {
    b == 0 || b == 0xffff_fffe
}

#[inline]
pub const fn BLOCK_TYPE(b: u32) -> u32 {
    b & 0x7f
}

#[inline]
pub const fn BLOCK_ADDRESS(b: u32) -> u32 {
    b & !0x7f
}

#[inline]
pub const fn BLOCK_NUMBER(b: u32) -> u32 {
    b >> 9
}

pub const BLOCK_CONTROL: u32 = 0x30;
pub const BLOCK_DATA: u32 = 0x40;
pub const BLOCK_REPLACEMENT: u32 = 0x60;
pub const BLOCK_BAD: u32 = 0x70;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
