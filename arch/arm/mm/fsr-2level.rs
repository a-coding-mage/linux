// SPDX-License-Identifier: GPL-2.0

static mut fsr_info: [fsr_info; 32] = [
	/*
	 * The following are the standard ARMv3 and ARMv4 aborts.  ARMv5
	 * defines these to be "precise" aborts.
	 */
	fsr_info { fn_: do_bad, sig: SIGSEGV, code: 0, name: "vector exception" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: BUS_ADRALN, name: "alignment exception" },
	fsr_info { fn_: do_bad, sig: SIGKILL, code: 0, name: "terminal exception" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: BUS_ADRALN, name: "alignment exception" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on linefetch" },
	fsr_info { fn_: do_translation_fault, sig: SIGSEGV, code: SEGV_MAPERR, name: "section translation fault" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on linefetch" },
	fsr_info { fn_: do_page_fault, sig: SIGSEGV, code: SEGV_MAPERR, name: "page translation fault" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on non-linefetch" },
	fsr_info { fn_: do_bad, sig: SIGSEGV, code: SEGV_ACCERR, name: "section domain fault" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on non-linefetch" },
	fsr_info { fn_: do_bad, sig: SIGSEGV, code: SEGV_ACCERR, name: "page domain fault" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on translation" },
	fsr_info { fn_: do_sect_fault, sig: SIGSEGV, code: SEGV_ACCERR, name: "section permission fault" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on translation" },
	fsr_info { fn_: do_page_fault, sig: SIGSEGV, code: SEGV_ACCERR, name: "page permission fault" },
	/*
	 * The following are "imprecise" aborts, which are signalled by bit
	 * 10 of the FSR, and may not be recoverable.  These are only
	 * supported if the CPU abort handler supports bit 10.
	 */
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 16" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 17" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 18" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 19" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "lock abort" }, // xscale
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 21" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: BUS_OBJERR, name: "imprecise external abort" }, // xscale
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 23" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "dcache parity error" }, // xscale
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 25" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 26" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 27" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 28" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 29" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 30" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 31" },
];

static mut ifsr_info: [fsr_info; 32] = [
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 0" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 1" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "debug event" },
	fsr_info { fn_: do_bad, sig: SIGSEGV, code: SEGV_ACCERR, name: "section access flag fault" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 4" },
	fsr_info { fn_: do_translation_fault, sig: SIGSEGV, code: SEGV_MAPERR, name: "section translation fault" },
	fsr_info { fn_: do_bad, sig: SIGSEGV, code: SEGV_ACCERR, name: "page access flag fault" },
	fsr_info { fn_: do_page_fault, sig: SIGSEGV, code: SEGV_MAPERR, name: "page translation fault" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on non-linefetch" },
	fsr_info { fn_: do_bad, sig: SIGSEGV, code: SEGV_ACCERR, name: "section domain fault" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 10" },
	fsr_info { fn_: do_bad, sig: SIGSEGV, code: SEGV_ACCERR, name: "page domain fault" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on translation" },
	fsr_info { fn_: do_sect_fault, sig: SIGSEGV, code: SEGV_ACCERR, name: "section permission fault" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "external abort on translation" },
	fsr_info { fn_: do_page_fault, sig: SIGSEGV, code: SEGV_ACCERR, name: "page permission fault" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 16" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 17" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 18" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 19" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 20" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 21" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 22" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 23" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 24" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 25" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 26" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 27" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 28" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 29" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 30" },
	fsr_info { fn_: do_bad, sig: SIGBUS, code: 0, name: "unknown 31" },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
