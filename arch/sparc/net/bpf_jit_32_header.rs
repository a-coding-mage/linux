/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Conventions:
 *  %g1 : temporary
 *  %g2 : Secondary temporary used by SKB data helper stubs.
 *  %g3 : packet offset passed into SKB data helper stubs.
 *  %o0 : pointer to skb (first argument given to JIT function)
 *  %o1 : BPF A accumulator
 *  %o2 : BPF X accumulator
 *  %o3 : Holds saved %o7 so we can call helper functions without needing
 *        to allocate a register window.
 *  %o4 : skb->len - skb->data_len
 *  %o5 : skb->data
 */

/* C preprocessor constants from the non-assembler branch. */
pub const G0: u32 = 0x00;
pub const G1: u32 = 0x01;
pub const G3: u32 = 0x03;
pub const G6: u32 = 0x06;
pub const O0: u32 = 0x08;
pub const O1: u32 = 0x09;
pub const O2: u32 = 0x0a;
pub const O3: u32 = 0x0b;
pub const O4: u32 = 0x0c;
pub const O5: u32 = 0x0d;
pub const SP: u32 = 0x0e;
pub const O7: u32 = 0x0f;
pub const FP: u32 = 0x1e;

pub const r_SKB: u32 = O0;
pub const r_A: u32 = O1;
pub const r_X: u32 = O2;
pub const r_saved_O7: u32 = O3;
pub const r_HEADLEN: u32 = O4;
pub const r_SKB_DATA: u32 = O5;
pub const r_TMP: u32 = G1;
/* G2 is referenced by the source but is not defined there. */
pub const r_TMP2: u32 = G2;
pub const r_OFF: u32 = G3;

/* Assembly code in arch/sparc/net/bpf_jit_asm_32.S. */
unsafe extern "C" {
    pub static mut bpf_jit_load_word: [u32; 0];
    pub static mut bpf_jit_load_half: [u32; 0];
    pub static mut bpf_jit_load_byte: [u32; 0];
    pub static mut bpf_jit_load_byte_msh: [u32; 0];
    pub static mut bpf_jit_load_word_positive_offset: [u32; 0];
    pub static mut bpf_jit_load_half_positive_offset: [u32; 0];
    pub static mut bpf_jit_load_byte_positive_offset: [u32; 0];
    pub static mut bpf_jit_load_byte_msh_positive_offset: [u32; 0];
    pub static mut bpf_jit_load_word_negative_offset: [u32; 0];
    pub static mut bpf_jit_load_half_negative_offset: [u32; 0];
    pub static mut bpf_jit_load_byte_negative_offset: [u32; 0];
    pub static mut bpf_jit_load_byte_msh_negative_offset: [u32; 0];
}

/* In the assembler branch these aliases expand to the following registers:
 * r_SKB=%o0, r_A=%o1, r_X=%o2, r_saved_O7=%o3, r_HEADLEN=%o4,
 * r_SKB_DATA=%o5, r_TMP=%g1, r_TMP2=%g2, r_OFF=%g3.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
