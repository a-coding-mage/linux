/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Linux Socket Filter Data Structures
 */

// Dependencies supplied by the corresponding Linux headers:
// linux/compiler.h, linux/types.h, linux/bpf_common.h

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
pub struct sock_filter {
	pub code: __u16, /* Actual filter code */
	pub jt: __u8,   /* Jump true */
	pub jf: __u8,   /* Jump false */
	pub k: __u32,   /* Generic multiuse field */
}

#[repr(C)]
pub struct sock_fprog {
	pub len: u16, /* Number of filter blocks */
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
#[macro_export]
macro_rules! BPF_STMT {
	($code:expr, $k:expr) => {
		sock_filter {
			code: ($code) as u16,
			jt: 0,
			jf: 0,
			k: $k,
		}
	};
}

#[macro_export]
macro_rules! BPF_JUMP {
	($code:expr, $k:expr, $jt:expr, $jf:expr) => {
		sock_filter {
			code: ($code) as u16,
			jt: $jt,
			jf: $jf,
			k: $k,
		}
	};
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
pub const SKF_AD_PROTOCOL: i32 = 0;
pub const SKF_AD_PKTTYPE: i32 = 4;
pub const SKF_AD_IFINDEX: i32 = 8;
pub const SKF_AD_NLATTR: i32 = 12;
pub const SKF_AD_NLATTR_NEST: i32 = 16;
pub const SKF_AD_MARK: i32 = 20;
pub const SKF_AD_QUEUE: i32 = 24;
pub const SKF_AD_HATYPE: i32 = 28;
pub const SKF_AD_RXHASH: i32 = 32;
pub const SKF_AD_CPU: i32 = 36;
pub const SKF_AD_ALU_XOR_X: i32 = 40;
pub const SKF_AD_VLAN_TAG: i32 = 44;
pub const SKF_AD_VLAN_TAG_PRESENT: i32 = 48;
pub const SKF_AD_PAY_OFFSET: i32 = 52;
pub const SKF_AD_RANDOM: i32 = 56;
pub const SKF_AD_VLAN_TPID: i32 = 60;
pub const SKF_AD_MAX: i32 = 64;

pub const SKF_NET_OFF: i32 = -0x100000;
pub const SKF_LL_OFF: i32 = -0x200000;

pub const BPF_NET_OFF: i32 = SKF_NET_OFF;
pub const BPF_LL_OFF: i32 = SKF_LL_OFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
