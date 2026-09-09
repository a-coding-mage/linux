// SPDX-License-Identifier: GPL-2.0

static mut fsr_info: [struct fsr_info; 32] = [
	/*
	 * The following are the standard ARMv3 and ARMv4 aborts.  ARMv5
	 * defines these to be "precise" aborts.
	 */
	struct fsr_info { fn_: do_bad, sig: SIGSEGV, code: 0, name: "vector exception" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: BUS_ADRALN, name: "alignment exception" },
	struct fsr_info { fn_: do_bad, sig: SIGKILL, code: 0, name: "terminal exception" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: BUS_ADRALN, name: "alignment exception" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on linefetch" },
	struct fsr_info { fn_: do_translation_fault, sig: SIGSEGV, code: SEGV_MAPERR, name: "section translation fault" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on linefetch" },
	struct fsr_info { fn_: do_page_fault, sig: SIGSEGV, code: SEGV_MAPERR, name: "page translation fault" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on non-linefetch" },
	struct fsr_info { fn_: do_bad, sig: SIGSEGV, code: SEGV_ACCERR, name: "section domain fault" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on non-linefetch" },
	struct fsr_info { fn_: do_bad, sig: SIGSEGV, code: SEGV_ACCERR, name: "page domain fault" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on translation" },
	struct fsr_info { fn_: do_sect_fault, sig: SIGSEGV, code: SEGV_ACCERR, name: "section permission fault" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on translation" },
	struct fsr_info { fn_: do_page_fault, sig: SIGSEGV, code: SEGV_ACCERR, name: "page permission fault" },
	/*
	 * The following are "imprecise" aborts, which are signalled by bit
	 * 10 of the FSR, and may not be recoverable.  These are only
	 * supported if the CPU abort handler supports bit 10.
	 */
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 16" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 17" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 18" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 19" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "lock abort" }, // xscale
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 21" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: BUS_OBJERR, name: "imprecise external abort" }, // xscale
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 23" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "dcache parity error" }, // xscale
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 25" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 26" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 27" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 28" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 29" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 30" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 31" },
];

static mut ifsr_info: [struct fsr_info; 32] = [
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 0" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 1" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "debug event" },
	struct fsr_info { fn_: do_bad, sig: SIGSEGV, code: SEGV_ACCERR, name: "section access flag fault" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 4" },
	struct fsr_info { fn_: do_translation_fault, sig: SIGSEGV, code: SEGV_MAPERR, name: "section translation fault" },
	struct fsr_info { fn_: do_bad, sig: SIGSEGV, code: SEGV_ACCERR, name: "page access flag fault" },
	struct fsr_info { fn_: do_page_fault, sig: SIGSEGV, code: SEGV_MAPERR, name: "page translation fault" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on non-linefetch" },
	struct fsr_info { fn_: do_bad, sig: SIGSEGV, code: SEGV_ACCERR, name: "section domain fault" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 10" },
	struct fsr_info { fn_: do_bad, sig: SIGSEGV, code: SEGV_ACCERR, name: "page domain fault" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on translation" },
	struct fsr_info { fn_: do_sect_fault, sig: SIGSEGV, code: SEGV_ACCERR, name: "section permission fault" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on translation" },
	struct fsr_info { fn_: do_page_fault, sig: SIGSEGV, code: SEGV_ACCERR, name: "page permission fault" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 16" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 17" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 18" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 19" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 20" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 21" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 22" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 23" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 24" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 25" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 26" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 27" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 28" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 29" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 30" },
	struct fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 31" },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
