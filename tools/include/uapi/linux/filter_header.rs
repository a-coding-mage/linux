/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Linux Socket Filter Data Structures
 */

/* Includes from the C header:
 * <linux/types.h>
 * <linux/bpf_common.h>
 */

/*
 * Current version of the filter code architecture.
 */
pub const BPF_MAJOR_VERSION: u32 = 1;
pub const BPF_MINOR_VERSION: u32 = 1;

/*
 *	Try and keep these values and structures similar to BSD, especially
 *	the BPF code definitions which need to match so you can share filters
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_filter {
    /* Actual filter code */
    pub code: u16,
    /* Jump true */
    pub jt: u8,
    /* Jump false */
    pub jf: u8,
    /* Generic multiuse field */
    pub k: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_fprog {
    /* Number of filter blocks */
    pub len: ::std::os::raw::c_ushort,
    pub filter: *mut sock_filter,
}

/* ret - BPF_K and BPF_X also apply */
#[inline]
pub const fn BPF_RVAL(code: u32) -> u32 {
    code & 0x18
}

pub const BPF_A: u32 = 0x10;

/* misc */
#[inline]
pub const fn BPF_MISCOP(code: u32) -> u32 {
    code & 0xf8
}

pub const BPF_TAX: u32 = 0x00;
pub const BPF_TXA: u32 = 0x80;

/*
 * Macros for filter block array initializers.
 */
#[inline]
pub const fn BPF_STMT(code: u32, k: u32) -> sock_filter {
    sock_filter {
        code: code as ::std::os::raw::c_ushort,
        jt: 0,
        jf: 0,
        k,
    }
}

#[inline]
pub const fn BPF_JUMP(code: u32, k: u32, jt: u8, jf: u8) -> sock_filter {
    sock_filter {
        code: code as ::std::os::raw::c_ushort,
        jt,
        jf,
        k,
    }
}

/*
 * Number of scratch memory words for: BPF_ST and BPF_STX
 */
pub const BPF_MEMWORDS: u32 = 16;

/* RATIONALE. Negative offsets are invalid in BPF.
   We use them to reference ancillary data.
   Unlike introduction new instructions, it does not break
   existing compilers/optimizers.
 */
pub const SKF_AD_OFF: i32 = -0x1000;
pub const SKF_AD_PROTOCOL: u32 = 0;
pub const SKF_AD_PKTTYPE: u32 = 4;
pub const SKF_AD_IFINDEX: u32 = 8;
pub const SKF_AD_NLATTR: u32 = 12;
pub const SKF_AD_NLATTR_NEST: u32 = 16;
pub const SKF_AD_MARK: u32 = 20;
pub const SKF_AD_QUEUE: u32 = 24;
pub const SKF_AD_HATYPE: u32 = 28;
pub const SKF_AD_RXHASH: u32 = 32;
pub const SKF_AD_CPU: u32 = 36;
pub const SKF_AD_ALU_XOR_X: u32 = 40;
pub const SKF_AD_VLAN_TAG: u32 = 44;
pub const SKF_AD_VLAN_TAG_PRESENT: u32 = 48;
pub const SKF_AD_PAY_OFFSET: u32 = 52;
pub const SKF_AD_RANDOM: u32 = 56;
pub const SKF_AD_VLAN_TPID: u32 = 60;
pub const SKF_AD_MAX: u32 = 64;

pub const SKF_NET_OFF: i32 = -0x100000;
pub const SKF_LL_OFF: i32 = -0x200000;

pub const BPF_NET_OFF: i32 = SKF_NET_OFF;
pub const BPF_LL_OFF: i32 = SKF_LL_OFF;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
