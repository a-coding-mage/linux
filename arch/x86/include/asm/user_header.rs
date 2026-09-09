/* SPDX-License-Identifier: GPL-2.0 */

// CONFIG_X86_32 selects asm/user_32.h; otherwise asm/user_64.h.
// The declarations supplied by those headers and asm/types.h are external dependencies.

#[repr(C)]
pub struct user_ymmh_regs {
    /* 16 * 16 bytes for each YMMH-reg */
    pub ymmh_space: [__u32; 64],
}

#[repr(C)]
pub struct user_xstate_header {
    pub xfeatures: __u64,
    pub reserved1: [__u64; 2],
    pub reserved2: [__u64; 5],
}

/*
 * The structure layout of user_xstateregs, used for exporting the
 * extended register state through ptrace and core-dump (NT_X86_XSTATE note)
 * interfaces will be same as the memory layout of xsave used by the processor
 * (except for the bytes 464..511, which can be used by the software) and hence
 * the size of this structure varies depending on the features supported by the
 * processor and OS. The size of the structure that users need to use can be
 * obtained by doing:
 *     cpuid_count(0xd, 0, &eax, &ptrace_xstateregs_struct_size, &ecx, &edx);
 * i.e., cpuid.(eax=0xd,ecx=0).ebx will be the size that user (debuggers, etc.)
 * need to use.
 *
 * For now, only the first 8 bytes of the software usable bytes[464..471] will
 * be used and will be set to OS enabled xstate mask (which is same as the
 * 64bit mask returned by the xgetbv's xCR0). Users (analyzing core dump
 * remotely, etc.) can use this mask as well as the mask saved in the
 * xstate_hdr bytes and interpret what states the processor/OS supports
 * and what states are in modified/initialized conditions for the
 * particular process/thread.
 *
 * Also when the user modifies certain state FP/SSE/etc through the
 * ptrace interface, they must ensure that the header.xfeatures
 * bytes[512..519] of the memory layout are updated correspondingly.
 * i.e., for example when FP state is modified to a non-init state,
 * header.xfeatures's bit 0 must be set to '1', when SSE is modified to
 * non-init state, header.xfeatures's bit 1 must to be set to '1', etc.
 */
pub const USER_XSTATE_FX_SW_WORDS: usize = 6;
pub const USER_XSTATE_XCR0_WORD: usize = 0;

#[repr(C)]
pub struct user_xstateregs_i387 {
    pub fpx_space: [__u64; 58],
    pub xstate_fx_sw: [__u64; USER_XSTATE_FX_SW_WORDS],
}

#[repr(C)]
pub struct user_xstateregs {
    pub i387: user_xstateregs_i387,
    pub header: user_xstate_header,
    pub ymmh: user_ymmh_regs,
    /* further processor state extensions go here */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
