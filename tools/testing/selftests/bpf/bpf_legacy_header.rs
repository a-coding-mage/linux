/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * C header guard removed.
 *
 * Original C conditional:
 *   #if __GNUC__ && !__clang__
 *
 * For GCC BPF C programs, the standard helper names were macros mapping to
 * GCC builtins, with the skb argument ignored:
 *   load_byte(skb, off) -> __builtin_bpf_load_byte(off)
 *   load_half(skb, off) -> __builtin_bpf_load_half(off)
 *   load_word(skb, off) -> __builtin_bpf_load_word(off)
 *
 * Rust has no direct file-local equivalent for those GCC BPF builtins or this
 * preprocessor condition, so the callable external declarations below preserve
 * the LLVM-builtin branch's interface.
 */

/*
 * llvm builtin functions that eBPF C program may use to
 * emit BPF_LD_ABS and BPF_LD_IND instructions
 */
unsafe extern "C" {
    #[link_name = "llvm.bpf.load.byte"]
    pub fn load_byte(skb: *mut core::ffi::c_void, off: u64) -> u64;

    #[link_name = "llvm.bpf.load.half"]
    pub fn load_half(skb: *mut core::ffi::c_void, off: u64) -> u64;

    #[link_name = "llvm.bpf.load.word"]
    pub fn load_word(skb: *mut core::ffi::c_void, off: u64) -> u64;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
