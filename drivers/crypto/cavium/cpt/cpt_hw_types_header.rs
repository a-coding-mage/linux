/* SPDX-License-Identifier: GPL-2.0-only */
/* Translation of cpt_hw_types.h. Bitfields are represented by their raw words
 * and mask/shift accessors, preserving the C register layout and semantics. */

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CptCompE {
    CPT_COMP_E_NOTDONE = 0x00,
    CPT_COMP_E_GOOD = 0x01,
    CPT_COMP_E_FAULT = 0x02,
    CPT_COMP_E_SWERR = 0x03,
    CPT_COMP_E_LAST_ENTRY = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_inst_s_s { pub words: [u64; 8] }
#[repr(C)]
pub union cpt_inst_s { pub u: [u64; 8], pub s: cpt_inst_s_s }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpt_res_s_s { pub words: [u64; 2] }
#[repr(C)]
pub union cpt_res_s { pub u: [u64; 2], pub s: cpt_res_s_s }

macro_rules! one_word_register {
    ($name:ident, $fields:ident) => {
        #[repr(C)]
        #[derive(Copy, Clone, Default)]
        pub struct $fields { pub word: u64 }
        #[repr(C)]
        pub union $name { pub u: u64, pub s: $fields }
    };
}

one_word_register!(cptx_pf_bist_status, cptx_pf_bist_status_s);
impl cptx_pf_bist_status_s { pub const BSTATUS_MASK: u64 = (1u64 << 30) - 1; pub fn bstatus(&self) -> u64 { self.word & Self::BSTATUS_MASK } }

one_word_register!(cptx_pf_constants, cptx_pf_constants_s);
impl cptx_pf_constants_s {
    pub fn vq(&self) -> u64 { self.word & 0xff }
    pub fn se(&self) -> u64 { (self.word >> 8) & 0xff }
    pub fn ae(&self) -> u64 { (self.word >> 16) & 0xff }
    pub fn grps(&self) -> u64 { (self.word >> 24) & 0xff }
    pub fn epcis(&self) -> u64 { (self.word >> 32) & 0xff }
}

one_word_register!(cptx_pf_exe_bist_status, cptx_pf_exe_bist_status_s);
impl cptx_pf_exe_bist_status_s { pub fn bstatus(&self) -> u64 { self.word & ((1u64 << 48) - 1) } }

one_word_register!(cptx_pf_qx_ctl, cptx_pf_qx_ctl_s);
impl cptx_pf_qx_ctl_s {
    pub fn pri(&self) -> u64 { self.word & 1 }
    pub fn grp(&self) -> u64 { (self.word >> 1) & 7 }
    pub fn iqb_ldwb(&self) -> u64 { (self.word >> 7) & 1 }
    pub fn inst_be(&self) -> u64 { (self.word >> 8) & 1 }
    pub fn inst_free(&self) -> u64 { (self.word >> 9) & 1 }
    pub fn cont_err(&self) -> u64 { (self.word >> 10) & 1 }
    pub fn size(&self) -> u64 { (self.word >> 32) & 0x1fff }
    pub fn aura(&self) -> u64 { (self.word >> 48) & 0xfff }
}

one_word_register!(cptx_vqx_saddr, cptx_vqx_saddr_s);
impl cptx_vqx_saddr_s { pub fn ptr(&self) -> u64 { (self.word >> 6) & ((1u64 << 43) - 1) } }

one_word_register!(cptx_vqx_misc_ena_w1s, cptx_vqx_misc_ena_w1s_s);
one_word_register!(cptx_vqx_misc_int, cptx_vqx_misc_int_s);
impl cptx_vqx_misc_ena_w1s_s { pub fn mbox(&self)->u64{self.word&1} pub fn dovf(&self)->u64{(self.word>>1)&1} pub fn irde(&self)->u64{(self.word>>2)&1} pub fn nwrp(&self)->u64{(self.word>>3)&1} pub fn swerr(&self)->u64{(self.word>>4)&1} }
impl cptx_vqx_misc_int_s { pub fn mbox(&self)->u64{self.word&1} pub fn dovf(&self)->u64{(self.word>>1)&1} pub fn irde(&self)->u64{(self.word>>2)&1} pub fn nwrp(&self)->u64{(self.word>>3)&1} pub fn swerr(&self)->u64{(self.word>>4)&1} }

one_word_register!(cptx_vqx_doorbell, cptx_vqx_doorbell_s);
impl cptx_vqx_doorbell_s { pub fn dbell_cnt(&self)->u64 { self.word & ((1u64<<20)-1) } }
one_word_register!(cptx_vqx_inprog, cptx_vqx_inprog_s);
impl cptx_vqx_inprog_s { pub fn inflight(&self)->u64 { self.word & 0xff } }
one_word_register!(cptx_vqx_done_ack, cptx_vqx_done_ack_s);
impl cptx_vqx_done_ack_s { pub fn done_ack(&self)->u64 { self.word & ((1u64<<20)-1) } }
one_word_register!(cptx_vqx_done, cptx_vqx_done_s);
impl cptx_vqx_done_s { pub fn done(&self)->u64 { self.word & ((1u64<<20)-1) } }
one_word_register!(cptx_vqx_done_wait, cptx_vqx_done_wait_s);
impl cptx_vqx_done_wait_s { pub fn num_wait(&self)->u64 { self.word & ((1u64<<20)-1) } pub fn time_wait(&self)->u64 { (self.word>>32)&0xffff } }
one_word_register!(cptx_vqx_done_ena_w1s, cptx_vqx_done_ena_w1s_s);
impl cptx_vqx_done_ena_w1s_s { pub fn done(&self)->u64 { self.word & 1 } }
one_word_register!(cptx_vqx_ctl, cptx_vqx_ctl_s);
impl cptx_vqx_ctl_s { pub fn ena(&self)->u64 { self.word & 1 } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
