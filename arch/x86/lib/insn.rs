// SPDX-License-Identifier: GPL-2.0-or-later
//! Safe, allocation-free x86 instruction analysis shared by kernel and tools.
// Copyright (C) IBM Corporation, 2002, 2004, 2009.

#[path = "../include/asm/insn_header.rs"]
mod definitions;
#[path = "../include/asm/emulate_prefix_header.rs"]
mod emulate;
#[path = "inat.rs"]
pub(crate) mod inat;

#[allow(unused_imports)] // Re-export the complete decoder API for shared users.
pub(crate) use definitions::*;
use inat::*;

impl<R: Input> Decoder<R> {
    fn peek(&self, offset: usize) -> Result<u8, DecodeError> {
        let offset = self.next.checked_add(offset).ok_or(DecodeError::NoData)?;
        let mut byte = [0];
        self.bytes.before_read(self);
        if self.bytes.read(offset, &mut byte) {
            Ok(byte[0])
        } else {
            Err(DecodeError::NoData)
        }
    }
    fn take_byte(&mut self) -> Result<u8, DecodeError> {
        let value = self.peek(0)?;
        self.next += 1;
        Ok(value)
    }
    fn take_int(&mut self, size: u8, signed: bool) -> Result<i32, DecodeError> {
        let end = self
            .next
            .checked_add(size as usize)
            .ok_or(DecodeError::NoData)?;
        let mut bytes = [0; 4];
        self.bytes.before_read(self);
        if !matches!(size, 1 | 2 | 4) || !self.bytes.read(self.next, &mut bytes[..size as usize]) {
            return Err(DecodeError::NoData);
        }
        let value = match (size, signed) {
            (1, true) => bytes[0] as i8 as i32,
            (1, false) => bytes[0] as i32,
            (2, true) => i16::from_le_bytes([bytes[0], bytes[1]]) as i32,
            (2, false) => u16::from_le_bytes([bytes[0], bytes[1]]) as i32,
            (4, _) => i32::from_le_bytes(bytes),
            _ => return Err(DecodeError::NoData),
        };
        self.next = end;
        Ok(value)
    }
    fn get_emulate_prefix(&mut self) {
        for prefix in [
            &emulate::__XEN_EMULATE_PREFIX,
            &emulate::__KVM_EMULATE_PREFIX,
        ] {
            // Check in C's order: stop at the first mismatch or missing byte,
            // rather than copying a speculative complete prefix up front.
            if prefix
                .iter()
                .enumerate()
                .all(|(offset, byte)| self.peek(offset) == Ok(*byte))
            {
                self.emulate_prefix_size = prefix.len();
                self.next += prefix.len();
                break;
            }
        }
    }

    pub(crate) fn get_prefixes(&mut self) -> DecodeResult {
        if self.prefixes.got {
            return Ok(());
        }
        self.get_emulate_prefix();
        let (mut unique, mut last) = (0usize, 0u8);
        let mut byte = self.peek(0)?;
        let mut attr = inat_get_opcode_attribute(byte);
        while inat_is_legacy_prefix(attr) {
            if !self.prefixes.bytes[..unique].contains(&byte) {
                if unique == 4 {
                    break;
                }
                self.prefixes.bytes[unique] = byte;
                unique += 1;
                if inat_is_address_size_prefix(attr) {
                    self.addr_bytes ^= if self.x86_64 { 12 } else { 6 };
                } else if inat_is_operand_size_prefix(attr) {
                    self.opnd_bytes ^= 6;
                }
            }
            self.prefixes.nbytes = self.prefixes.nbytes.wrapping_add(1);
            self.next += 1;
            last = byte;
            byte = self.peek(0)?;
            attr = inat_get_opcode_attribute(byte);
        }
        if last != 0 && last != self.prefixes.bytes[3] {
            let old = self.prefixes.bytes[3];
            if old != 0 {
                for item in &mut self.prefixes.bytes[..unique] {
                    if *item == last {
                        *item = old;
                    }
                }
            }
            self.prefixes.bytes[3] = last;
        }
        if self.x86_64 {
            byte = self.peek(0)?;
            attr = inat_get_opcode_attribute(byte);
            if inat_is_rex_prefix(attr) {
                self.rex_prefix.set(byte as i32, 1);
                self.next += 1;
                if byte & 8 != 0 {
                    self.opnd_bytes = 8;
                }
            } else if inat_is_rex2_prefix(attr) {
                self.rex_prefix.bytes[0] = byte;
                byte = self.peek(1)?;
                self.rex_prefix.bytes[1] = byte;
                self.rex_prefix.nbytes = 2;
                self.next += 2;
                if byte & 8 != 0 {
                    self.opnd_bytes = 8;
                }
                self.rex_prefix.mark_got();
                self.vex_prefix.mark_got();
                self.prefixes.mark_got();
                return Ok(());
            }
        }
        self.rex_prefix.mark_got();
        byte = self.peek(0)?;
        // Retain the preceding attribute: a REX followed by VEX is not parsed
        // as a VEX prefix by the original decoder.
        if inat_is_vex_prefix(attr) || inat_is_xop_prefix(attr) {
            let second = self.peek(1)?;
            let pop = inat_is_xop_prefix(attr) && (second >> 3) & 7 == 0;
            if !pop && (self.x86_64 || second >> 6 == 3) {
                self.vex_prefix.bytes[0] = byte;
                self.vex_prefix.bytes[1] = second;
                if inat_is_evex_prefix(attr) {
                    self.vex_prefix.bytes[2] = self.peek(2)?;
                    let last = self.peek(3)?;
                    self.vex_prefix.bytes[3] = last;
                    self.vex_prefix.nbytes = 4;
                    self.next += 4;
                    if self.x86_64 && last & 0x80 != 0 {
                        self.opnd_bytes = 8;
                    }
                } else if inat_is_vex3_prefix(attr) || inat_is_xop_prefix(attr) {
                    let last = self.peek(2)?;
                    self.vex_prefix.bytes[2] = last;
                    self.vex_prefix.nbytes = 3;
                    self.next += 3;
                    if self.x86_64 && last & 0x80 != 0 {
                        self.opnd_bytes = 8;
                    }
                } else {
                    self.vex_prefix.bytes[2] = second & 0x7f;
                    self.vex_prefix.nbytes = 2;
                    self.next += 2;
                }
            }
        }
        self.vex_prefix.mark_got();
        self.prefixes.mark_got();
        Ok(())
    }

    pub(crate) fn get_opcode(&mut self) -> DecodeResult {
        if self.opcode.got {
            return Ok(());
        }
        self.get_prefixes()?;
        let mut byte = self.take_byte()?;
        self.opcode.bytes[0] = byte;
        self.opcode.nbytes = 1;
        if self.is_avx_or_xop() {
            if self.avx_is_xop() {
                self.attr = inat_get_xop_attribute(byte, self.xop_map_bits());
                if !inat_accept_xop(self.attr) {
                    self.attr = 0;
                    return Err(DecodeError::Invalid);
                }
            } else {
                let prefix = self.vex_p_bits();
                self.attr = inat_get_avx_attribute(byte, self.vex_m_bits(), prefix);
                if inat_evex_scalable(self.attr) && !self.vex_w_bit() && prefix == INAT_PFX_OPNDSZ {
                    self.opnd_bytes = 2;
                }
                if (inat_must_evex(self.attr) && !self.is_evex())
                    || (!inat_accept_vex(self.attr) && !inat_is_group(self.attr))
                {
                    self.attr = 0;
                    return Err(DecodeError::Invalid);
                }
            }
        } else if self.is_rex2() {
            self.attr = if self.rex2_m_bit() {
                let prefix = self.last_prefix_id();
                inat_get_escape_attribute(byte, prefix, inat_get_opcode_attribute(0x0f))
            } else {
                inat_get_opcode_attribute(byte)
            };
        } else {
            self.attr = inat_get_opcode_attribute(byte);
            if self.x86_64 && inat_is_invalid64(self.attr) {
                self.attr &= INAT_INV64;
            }
            while inat_is_escape(self.attr) {
                byte = self.take_byte()?;
                let field = self
                    .opcode
                    .bytes
                    .get_mut(self.opcode.nbytes as usize)
                    .ok_or(DecodeError::Invalid)?;
                *field = byte;
                self.opcode.nbytes += 1;
                let prefix = self.last_prefix_id();
                self.attr = inat_get_escape_attribute(byte, prefix, self.attr);
            }
            if inat_must_vex(self.attr) {
                self.attr = 0;
                return Err(DecodeError::Invalid);
            }
        }
        self.opcode.mark_got();
        Ok(())
    }

    pub(crate) fn get_modrm(&mut self) -> DecodeResult {
        if self.modrm.got {
            return Ok(());
        }
        self.get_opcode()?;
        if inat_has_modrm(self.attr) {
            let byte = self.take_byte()?;
            self.modrm.set(byte as i32, 1);
            if inat_is_group(self.attr) {
                let prefix = self.last_prefix_id();
                self.attr = inat_get_group_attribute(byte, prefix, self.attr);
                if self.is_avx_or_xop()
                    && !inat_accept_vex(self.attr)
                    && !inat_accept_xop(self.attr)
                {
                    self.attr = 0;
                    return Err(DecodeError::Invalid);
                }
            }
        }
        if self.x86_64 && inat_is_force64(self.attr) {
            self.opnd_bytes = 8;
        }
        self.modrm.mark_got();
        Ok(())
    }
    pub(crate) fn rip_relative(&mut self) -> bool {
        self.x86_64
            && self.get_modrm().is_ok()
            && self.modrm.nbytes != 0
            && self.modrm.bytes[0] & 0xc7 == 5
    }
    pub(crate) fn get_sib(&mut self) -> DecodeResult {
        if self.sib.got {
            return Ok(());
        }
        self.get_modrm()?;
        let modrm = self.modrm.bytes[0];
        if self.modrm.nbytes != 0 && self.addr_bytes != 2 && modrm >> 6 != 3 && modrm & 7 == 4 {
            let byte = self.take_byte()?;
            self.sib.set(byte as i32, 1);
        }
        self.sib.mark_got();
        Ok(())
    }
    pub(crate) fn get_displacement(&mut self) -> DecodeResult {
        if self.displacement.got {
            return Ok(());
        }
        self.get_sib()?;
        if self.modrm.nbytes != 0 {
            let (mode, rm, base) = (
                self.modrm.bytes[0] >> 6,
                self.modrm.bytes[0] & 7,
                self.sib.bytes[0] & 7,
            );
            let size = match mode {
                3 => 0,
                1 => 1,
                _ if self.addr_bytes == 2 => {
                    if (mode == 0 && rm == 6) || mode == 2 {
                        2
                    } else {
                        0
                    }
                }
                _ if (mode == 0 && rm == 5) || mode == 2 || (mode == 0 && base == 5) => 4,
                _ => 0,
            };
            if size != 0 {
                let value = self.take_int(size, true)?;
                self.displacement.set(value, size);
            }
        }
        self.displacement.mark_got();
        Ok(())
    }

    fn get_moffset(&mut self) -> DecodeResult {
        match self.addr_bytes {
            2 | 4 => {
                let value = self.take_int(self.addr_bytes, true)?;
                self.immediate1.set(value, self.addr_bytes);
            }
            8 => {
                let first = self.take_int(4, true)?;
                self.immediate1.set(first, 4);
                let second = self.take_int(4, true)?;
                self.immediate2.set(second, 4);
            }
            _ => return Err(DecodeError::NoData),
        }
        self.immediate1.mark_got();
        self.immediate2.mark_got();
        Ok(())
    }
    fn get_immv32(&mut self) -> DecodeResult {
        let size = match self.opnd_bytes {
            2 => 2,
            4 | 8 => 4,
            _ => return Err(DecodeError::NoData),
        };
        let value = self.take_int(size, true)?;
        self.immediate1.set(value, size);
        Ok(())
    }
    fn get_immv(&mut self) -> DecodeResult {
        match self.opnd_bytes {
            2 | 4 => {
                let value = self.take_int(self.opnd_bytes, true)?;
                self.immediate1.set(value, self.opnd_bytes);
            }
            8 => {
                let first = self.take_int(4, true)?;
                self.immediate1.set(first, 4);
                let second = self.take_int(4, true)?;
                self.immediate2.set(second, 4);
            }
            _ => return Err(DecodeError::NoData),
        }
        self.immediate1.mark_got();
        self.immediate2.mark_got();
        Ok(())
    }
    fn get_immptr(&mut self) -> DecodeResult {
        if !matches!(self.opnd_bytes, 2 | 4) {
            return Err(DecodeError::NoData);
        }
        let first = self.take_int(self.opnd_bytes, true)?;
        self.immediate1.set(first, self.opnd_bytes);
        let second = self.take_int(2, false)?;
        self.immediate2.set(second, 2);
        self.immediate1.mark_got();
        self.immediate2.mark_got();
        Ok(())
    }

    pub(crate) fn get_immediate(&mut self) -> DecodeResult {
        if self.immediate1.got {
            return Ok(());
        }
        self.get_displacement()?;
        if inat_has_moffset(self.attr) {
            self.get_moffset()?;
        } else if inat_has_immediate(self.attr) {
            match inat_immediate_size(self.attr) {
                INAT_IMM_BYTE | INAT_IMM_WORD | INAT_IMM_DWORD => {
                    let size = match inat_immediate_size(self.attr) {
                        INAT_IMM_BYTE => 1,
                        INAT_IMM_WORD => 2,
                        _ => 4,
                    };
                    let value = self.take_int(size, true)?;
                    self.immediate1.set(value, size);
                }
                INAT_IMM_QWORD => {
                    let first = self.take_int(4, true)?;
                    self.immediate1.set(first, 4);
                    let second = self.take_int(4, true)?;
                    self.immediate2.set(second, 4);
                }
                INAT_IMM_PTR => self.get_immptr()?,
                INAT_IMM_VWORD32 => self.get_immv32()?,
                INAT_IMM_VWORD => self.get_immv()?,
                _ => return Err(DecodeError::NoData),
            }
            if inat_has_second_immediate(self.attr) {
                let second = self.take_int(1, true)?;
                self.immediate2.set(second, 1);
            }
        }
        self.immediate1.mark_got();
        Ok(())
    }
    pub(crate) fn get_length(&mut self) -> DecodeResult {
        if self.length != 0 {
            return Ok(());
        }
        self.get_immediate()?;
        self.length = self.length_bias.wrapping_add(self.next) as u8;
        Ok(())
    }
    pub(crate) fn complete(&self) -> bool {
        self.opcode.got
            && self.modrm.got
            && self.sib.got
            && self.displacement.got
            && self.immediate1.got
    }
    pub(crate) fn decode(&mut self) -> DecodeResult {
        self.get_length()?;
        if self.complete() {
            Ok(())
        } else {
            Err(DecodeError::Invalid)
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
