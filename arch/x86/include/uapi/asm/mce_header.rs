/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Dependency intent from <linux/types.h> and <linux/ioctl.h> is preserved
 * through the Rust primitive types and ioctl values below.
 */

/*
 * Fields are zero when not available. Also, this struct is shared with
 * userspace mcelog and thus must keep existing fields at current offsets.
 * Only add new, shared fields to the end of the structure.
 * Do not add vendor-specific fields.
 */
#[repr(C)]
pub struct mce {
	pub status: u64,      /* Bank's MCi_STATUS MSR */
	pub misc: u64,        /* Bank's MCi_MISC MSR */
	pub addr: u64,        /* Bank's MCi_ADDR MSR */
	pub mcgstatus: u64,   /* Machine Check Global Status MSR */
	pub ip: u64,          /* Instruction Pointer when the error happened */
	pub tsc: u64,         /* CPU time stamp counter */
	pub time: u64,        /* Wall time_t when error was detected */
	pub cpuvendor: u8,    /* Kernel's X86_VENDOR enum */
	pub inject_flags: u8, /* Software inject flags */
	pub severity: u8,     /* Error severity */
	pub pad: u8,
	pub cpuid: u32,       /* CPUID 1 EAX */
	pub cs: u8,           /* Code segment */
	pub bank: u8,         /* Machine check bank reporting the error */
	pub cpu: u8,          /* CPU number; obsoleted by extcpu */
	pub finished: u8,     /* Entry is valid */
	pub extcpu: u32,      /* Linux CPU number that detected the error */
	pub socketid: u32,    /* CPU socket ID */
	pub apicid: u32,      /* CPU initial APIC ID */
	pub mcgcap: u64,      /* MCGCAP MSR: machine check capabilities of CPU */
	pub synd: u64,        /* MCA_SYND MSR: only valid on SMCA systems */
	pub ipid: u64,        /* MCA_IPID MSR: only valid on SMCA systems */
	pub ppin: u64,        /* Protected Processor Inventory Number */
	pub microcode: u32,   /* Microcode revision */
	pub kflags: u64,      /* Internal kernel use */
}

/* _IOR('M', nr, int), as defined by <linux/ioctl.h>. */
pub const MCE_GET_RECORD_LEN: u32 = 0x8004_4d01;
pub const MCE_GET_LOG_LEN: u32 = 0x8004_4d02;
pub const MCE_GETCLEAR_FLAGS: u32 = 0x8004_4d03;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
