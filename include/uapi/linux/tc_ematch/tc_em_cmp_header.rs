/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

#[repr(C)]
pub struct tcf_em_cmp {
    pub val: u32,
    pub mask: u32,
    pub off: u16,
    /* C bit-fields: align:4, flags:4. */
    pub align_flags: u8,
    /* C bit-fields: layer:4, opnd:4. */
    pub layer_opnd: u8,
}

impl tcf_em_cmp {
    #[inline]
    pub const fn align(&self) -> u8 {
        self.align_flags & 0x0f
    }

    #[inline]
    pub fn set_align(&mut self, value: u8) {
        self.align_flags = (self.align_flags & 0xf0) | (value & 0x0f);
    }

    #[inline]
    pub const fn flags(&self) -> u8 {
        (self.align_flags >> 4) & 0x0f
    }

    #[inline]
    pub fn set_flags(&mut self, value: u8) {
        self.align_flags = (self.align_flags & 0x0f) | ((value & 0x0f) << 4);
    }

    #[inline]
    pub const fn layer(&self) -> u8 {
        self.layer_opnd & 0x0f
    }

    #[inline]
    pub fn set_layer(&mut self, value: u8) {
        self.layer_opnd = (self.layer_opnd & 0xf0) | (value & 0x0f);
    }

    #[inline]
    pub const fn opnd(&self) -> u8 {
        (self.layer_opnd >> 4) & 0x0f
    }

    #[inline]
    pub fn set_opnd(&mut self, value: u8) {
        self.layer_opnd = (self.layer_opnd & 0x0f) | ((value & 0x0f) << 4);
    }
}

pub const TCF_EM_ALIGN_U8: u32 = 1;
pub const TCF_EM_ALIGN_U16: u32 = 2;
pub const TCF_EM_ALIGN_U32: u32 = 4;

pub const TCF_EM_CMP_TRANS: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
