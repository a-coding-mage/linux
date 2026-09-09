/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency intent preserved from <linux/types.h> and <linux/pkt_cls.h>.

pub const TC_EM_TEXT_ALGOSIZ: usize = 16;

#[repr(C)]
pub struct tcf_em_text {
    pub algo: [::core::ffi::c_char; TC_EM_TEXT_ALGOSIZ],
    pub from_offset: u16,
    pub to_offset: u16,
    pub pattern_len: u16,
    // C bit-fields: from_layer occupies bits 0..=3 and to_layer bits 4..=7.
    pub layers: u8,
    pub pad: u8,
}

impl tcf_em_text {
    #[inline]
    pub fn from_layer(&self) -> u8 {
        self.layers & 0x0f
    }

    #[inline]
    pub fn to_layer(&self) -> u8 {
        (self.layers >> 4) & 0x0f
    }

    #[inline]
    pub fn set_from_layer(&mut self, value: u8) {
        self.layers = (self.layers & 0xf0) | (value & 0x0f);
    }

    #[inline]
    pub fn set_to_layer(&mut self, value: u8) {
        self.layers = (self.layers & 0x0f) | ((value & 0x0f) << 4);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
