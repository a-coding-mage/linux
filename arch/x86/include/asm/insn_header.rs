// SPDX-License-Identifier: GPL-2.0-or-later
//! Owned x86 instruction fields and a bounded, borrowed decoding cursor.
// Copyright (C) IBM Corporation, 2009.

use super::inat;

pub(crate) const MAX_INSN_SIZE: usize = 15;
pub(crate) const POP_SS_OPCODE: u8 = 0x1f;
pub(crate) const MOV_SREG_OPCODE: u8 = 0x8e;
pub(crate) const X86_VEX2_M: u8 = 1;
pub(crate) const X86_VEX_M_MAX: u8 = 0x1f;
pub(crate) const X86_XOP_M_MIN: u8 = 0x08;
pub(crate) const X86_XOP_M_MAX: u8 = 0x1f;

// Native equivalents of the field-extraction macros, keeping each mask's
// numeric value (rather than collapsing single-bit masks into booleans).
pub(crate) const fn modrm_mod(byte: u8) -> u8 {
    byte >> 6
}
pub(crate) const fn modrm_reg(byte: u8) -> u8 {
    (byte >> 3) & 7
}
pub(crate) const fn modrm_rm(byte: u8) -> u8 {
    byte & 7
}
pub(crate) const fn sib_scale(byte: u8) -> u8 {
    byte >> 6
}
pub(crate) const fn sib_index(byte: u8) -> u8 {
    (byte >> 3) & 7
}
pub(crate) const fn sib_base(byte: u8) -> u8 {
    byte & 7
}
pub(crate) const fn rex2_m(byte: u8) -> u8 {
    byte & 0x80
}
pub(crate) const fn rex2_r(byte: u8) -> u8 {
    byte & 0x40
}
pub(crate) const fn rex2_x(byte: u8) -> u8 {
    byte & 0x20
}
pub(crate) const fn rex2_b(byte: u8) -> u8 {
    byte & 0x10
}
pub(crate) const fn rex_w(byte: u8) -> u8 {
    byte & 8
}
pub(crate) const fn rex_r(byte: u8) -> u8 {
    byte & 4
}
pub(crate) const fn rex_x(byte: u8) -> u8 {
    byte & 2
}
pub(crate) const fn rex_b(byte: u8) -> u8 {
    byte & 1
}
pub(crate) const fn vex_w(byte: u8) -> u8 {
    byte & 0x80
}
pub(crate) const fn vex_r(byte: u8) -> u8 {
    byte & 0x80
}
pub(crate) const fn vex_x(byte: u8) -> u8 {
    byte & 0x40
}
pub(crate) const fn vex_b(byte: u8) -> u8 {
    byte & 0x20
}
pub(crate) const fn vex_l(byte: u8) -> u8 {
    byte & 4
}
pub(crate) const fn evex_m(byte: u8) -> u8 {
    byte & 7
}
pub(crate) const fn vex3_m(byte: u8) -> u8 {
    byte & 0x1f
}
pub(crate) const fn vex_v(byte: u8) -> u8 {
    (byte >> 3) & 0xf
}
pub(crate) const fn vex_p(byte: u8) -> u8 {
    byte & 3
}
pub(crate) const fn xop_r(byte: u8) -> u8 {
    byte & 0x80
}
pub(crate) const fn xop_x(byte: u8) -> u8 {
    byte & 0x40
}
pub(crate) const fn xop_b(byte: u8) -> u8 {
    byte & 0x20
}
pub(crate) const fn xop_m(byte: u8) -> u8 {
    byte & 0x1f
}
pub(crate) const fn xop_w(byte: u8) -> u8 {
    byte & 0x80
}
pub(crate) const fn xop_v(byte: u8) -> u8 {
    byte & 0x78
}
pub(crate) const fn xop_l(byte: u8) -> u8 {
    byte & 4
}
pub(crate) const fn xop_p(byte: u8) -> u8 {
    byte & 3
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct Field {
    pub(crate) bytes: [u8; 4],
    pub(crate) got: bool,
    pub(crate) nbytes: u8,
    // The C flag is an unsigned byte, not _Bool. Track actual assignments
    // separately so an untouched nonzero flag retains its original value.
    pub(crate) got_written: bool,
}

impl Field {
    pub(crate) fn value(&self) -> i32 {
        i32::from_le_bytes(self.bytes)
    }
    pub(crate) fn set(&mut self, value: i32, nbytes: u8) {
        self.bytes = value.to_le_bytes();
        self.nbytes = nbytes;
    }
    pub(crate) fn mark_got(&mut self) {
        self.got = true;
        self.got_written = true;
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Mode {
    Bits32,
    Bits64,
    // The caller supplies the target kernel configuration, not the host width.
    Kernel { x86_64: bool },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub(crate) enum DecodeError {
    Invalid = -22,
    NoData = -61,
}
pub(crate) type DecodeResult = Result<(), DecodeError>;

/// Copy only the requested bytes of an instruction, after checking its bound.
///
/// A failed read leaves the destination untouched. Implementations must not
/// read beyond the requested span: a kernel instruction may border an unmapped
/// page, and a cached decoding stage need not access instruction memory at all.
pub(crate) trait Input {
    // A native adapter can publish preceding field changes before a read,
    // preserving C when instruction input aliases the output structure.
    fn before_read(&self, _decoded: &Decoder<Self>)
    where
        Self: Sized,
    {
    }
    fn read(&self, offset: usize, destination: &mut [u8]) -> bool;
}

impl Input for &[u8] {
    fn read(&self, offset: usize, destination: &mut [u8]) -> bool {
        let Some(end) = offset.checked_add(destination.len()) else {
            return false;
        };
        let Some(bytes) = self.get(offset..end) else {
            return false;
        };
        destination.copy_from_slice(bytes);
        true
    }
}

// Existing host users retain their borrowed-slice API and lifetime.
pub(crate) type Instruction<'a> = Decoder<&'a [u8]>;

#[derive(Clone, Debug)]
pub(crate) struct Decoder<R> {
    pub(crate) prefixes: Field,
    pub(crate) rex_prefix: Field,
    pub(crate) vex_prefix: Field,
    pub(crate) opcode: Field,
    pub(crate) modrm: Field,
    pub(crate) sib: Field,
    pub(crate) displacement: Field,
    pub(crate) immediate1: Field,
    pub(crate) immediate2: Field,
    pub(crate) emulate_prefix_size: usize,
    pub(crate) attr: u32,
    pub(crate) opnd_bytes: u8,
    pub(crate) addr_bytes: u8,
    pub(crate) length: u8,
    pub(crate) x86_64: bool,
    pub(crate) bytes: R,
    pub(crate) next: usize,
    // A native caller can rebase kaddr independently of its next-byte cursor.
    // Keep its wrapping pointer difference separate from bounded read indices.
    pub(crate) length_bias: usize,
}

impl<'a> Instruction<'a> {
    pub(crate) fn new(bytes: &'a [u8], mode: Mode) -> Self {
        let x86_64 = match mode {
            Mode::Bits32 => false,
            Mode::Bits64 => true,
            Mode::Kernel { x86_64 } => x86_64,
        };
        Self {
            prefixes: Field::default(),
            rex_prefix: Field::default(),
            vex_prefix: Field::default(),
            opcode: Field::default(),
            modrm: Field::default(),
            sib: Field::default(),
            displacement: Field::default(),
            immediate1: Field::default(),
            immediate2: Field::default(),
            emulate_prefix_size: 0,
            attr: 0,
            opnd_bytes: 4,
            addr_bytes: if x86_64 { 8 } else { 4 },
            length: 0,
            x86_64,
            bytes: &bytes[..bytes.len().min(MAX_INSN_SIZE)],
            next: 0,
            length_bias: 0,
        }
    }
}

impl<R: Input> Decoder<R> {
    // These names alias C unions without maintaining duplicate field values.
    pub(crate) fn immediate(&self) -> &Field {
        &self.immediate1
    }
    pub(crate) fn moffset1(&self) -> &Field {
        &self.immediate1
    }
    pub(crate) fn moffset2(&self) -> &Field {
        &self.immediate2
    }
    pub(crate) fn xop_prefix(&self) -> &Field {
        &self.vex_prefix
    }

    pub(crate) fn get_attribute(&mut self) -> u32 {
        let _ = self.get_modrm();
        self.attr
    }
    pub(crate) fn is_rex2(&mut self) -> bool {
        if !self.prefixes.got {
            let _ = self.get_prefixes();
        }
        self.rex_prefix.nbytes == 2
    }
    pub(crate) fn rex2_m_bit(&self) -> bool {
        self.rex_prefix.bytes[1] & 0x80 != 0
    }
    pub(crate) fn is_avx_or_xop(&mut self) -> bool {
        if !self.prefixes.got {
            let _ = self.get_prefixes();
        }
        self.vex_prefix.value() != 0
    }
    pub(crate) fn is_evex(&mut self) -> bool {
        if !self.prefixes.got {
            let _ = self.get_prefixes();
        }
        self.vex_prefix.nbytes == 4
    }
    pub(crate) fn avx_is_xop(&self) -> bool {
        inat::inat_is_xop_prefix(inat::inat_get_opcode_attribute(self.vex_prefix.bytes[0]))
    }
    pub(crate) fn is_xop(&mut self) -> bool {
        self.is_avx_or_xop() && self.avx_is_xop()
    }
    pub(crate) fn has_emulate_prefix(&self) -> bool {
        self.emulate_prefix_size != 0
    }
    pub(crate) fn vex_m_bits(&self) -> u8 {
        match self.vex_prefix.nbytes {
            2 => 1,
            3 => self.vex_prefix.bytes[1] & 0x1f,
            _ => self.vex_prefix.bytes[1] & 7,
        }
    }
    pub(crate) fn vex_p_bits(&self) -> u8 {
        self.vex_prefix.bytes[if self.vex_prefix.nbytes == 2 { 1 } else { 2 }] & 3
    }
    pub(crate) fn vex_w_bit(&self) -> bool {
        self.vex_prefix.nbytes >= 3 && self.vex_prefix.bytes[2] & 0x80 != 0
    }
    pub(crate) fn xop_map_bits(&self) -> u8 {
        if self.vex_prefix.nbytes < 3 {
            0
        } else {
            self.vex_prefix.bytes[1] & 0x1f
        }
    }
    pub(crate) fn xop_p_bits(&self) -> u8 {
        self.vex_prefix.bytes[2] & 3
    }
    pub(crate) fn last_prefix_id(&mut self) -> u8 {
        if self.is_avx_or_xop() {
            if self.avx_is_xop() {
                self.xop_p_bits()
            } else {
                self.vex_p_bits()
            }
        } else if self.prefixes.bytes[3] != 0 {
            inat::inat_get_last_prefix_id(self.prefixes.bytes[3])
        } else {
            0
        }
    }
    pub(crate) fn offset_rex_prefix(&self) -> usize {
        self.prefixes.nbytes as usize
    }
    pub(crate) fn offset_vex_prefix(&self) -> usize {
        self.offset_rex_prefix() + self.rex_prefix.nbytes as usize
    }
    pub(crate) fn offset_opcode(&self) -> usize {
        self.offset_vex_prefix() + self.vex_prefix.nbytes as usize
    }
    pub(crate) fn offset_modrm(&self) -> usize {
        self.offset_opcode() + self.opcode.nbytes as usize
    }
    pub(crate) fn offset_sib(&self) -> usize {
        self.offset_modrm() + self.modrm.nbytes as usize
    }
    pub(crate) fn offset_displacement(&self) -> usize {
        self.offset_sib() + self.sib.nbytes as usize
    }
    pub(crate) fn offset_immediate(&self) -> usize {
        self.offset_displacement() + self.displacement.nbytes as usize
    }
    pub(crate) fn iter_prefixes(&self) -> impl Iterator<Item = u8> + '_ {
        self.prefixes
            .bytes
            .iter()
            .copied()
            .take_while(|&byte| byte != 0)
    }
    pub(crate) fn masking_exception(&self) -> bool {
        self.opcode.bytes[0] == POP_SS_OPCODE
            || (self.opcode.bytes[0] == MOV_SREG_OPCODE && (self.modrm.bytes[0] >> 3) & 7 == 2)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
