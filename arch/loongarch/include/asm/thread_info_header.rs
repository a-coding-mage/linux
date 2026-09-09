/* SPDX-License-Identifier: GPL-2.0 */
/*
 * thread_info.h: LoongArch low-level thread information
 *
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

/* C header guards and __KERNEL__/__ASSEMBLER__ conditions omitted. */
/* Dependency supplied by the surrounding kernel translation unit:
 * asm/processor.h and asm-generic/thread_info_tif.h
 */

/*
 * low level task data that entry.S needs immediate access to
 * - this struct should fit entirely inside of one cache line
 * - this struct shares the supervisor stack pages
 * - if the contents of this structure are changed, the assembly constants
 *   must also be changed
 */
#[repr(C)]
pub struct thread_info {
	pub flags: usize,             /* low level flags */
	pub tp_value: usize,          /* thread pointer */
	pub cpu: u32,                 /* current CPU */
	pub preempt_count: i32,       /* 0 => preemptible, <0 => BUG */
	pub regs: *mut pt_regs,
	pub syscall: usize,           /* syscall number */
	pub syscall_work: usize,      /* SYSCALL_WORK_ flags */
}

/* macros/functions for gaining access to the thread information structure */
#[macro_export]
macro_rules! INIT_THREAD_INFO {
	($tsk:expr) => {
		$crate::thread_info {
			flags: _TIF_FIXADE,
			cpu: 0,
			preempt_count: INIT_PREEMPT_COUNT,
			tp_value: 0,
			regs: core::ptr::null_mut(),
			syscall: 0,
			syscall_work: 0,
		}
	};
}

/* C declaration: register unsigned long current_stack_pointer __asm__("$sp"); */
extern "C" {
	pub static mut current_stack_pointer: usize;
}

/* thread information allocation */
pub const THREAD_SIZE: usize = SZ_16K;
pub const THREAD_MASK: usize = THREAD_SIZE - 1usize;
pub const THREAD_SIZE_ORDER: usize = ilog2(THREAD_SIZE / PAGE_SIZE);

/*
 * thread information flags
 * - these are process state flags that various assembly files may need to
 *   access
 * - pending work-to-be-done flags are in LSW
 * - other flags in MSW
 *
 * Tell the generic TIF infrastructure which special bits loongarch supports
 */
/* #define HAVE_TIF_NEED_RESCHED_LAZY */
/* #define HAVE_TIF_RESTORE_SIGMASK */

/* Architecture specific bits */
pub const TIF_NOHZ: usize = 16;          /* in adaptive nohz mode */
pub const TIF_USEDFPU: usize = 17;       /* FPU was used by this task this quantum (SMP) */
pub const TIF_USEDSIMD: usize = 18;      /* SIMD has been used this quantum */
pub const TIF_FIXADE: usize = 19;        /* Fix address errors in software */
pub const TIF_LOGADE: usize = 20;        /* Log address errors to syslog */
pub const TIF_32BIT_REGS: usize = 21;    /* 32-bit general purpose registers */
pub const TIF_32BIT_ADDR: usize = 22;    /* 32-bit address space */
pub const TIF_LOAD_WATCH: usize = 23;    /* If set, load watch registers */
pub const TIF_SINGLESTEP: usize = 24;    /* Single Step */
pub const TIF_LSX_CTX_LIVE: usize = 25;  /* LSX context must be preserved */
pub const TIF_LASX_CTX_LIVE: usize = 26; /* LASX context must be preserved */
pub const TIF_USEDLBT: usize = 27;       /* LBT was used by this task this quantum (SMP) */
pub const TIF_LBT_CTX_LIVE: usize = 28;  /* LBT context must be preserved */

pub const _TIF_NOHZ: usize = 1usize << TIF_NOHZ;
pub const _TIF_USEDFPU: usize = 1usize << TIF_USEDFPU;
pub const _TIF_USEDSIMD: usize = 1usize << TIF_USEDSIMD;
pub const _TIF_FIXADE: usize = 1usize << TIF_FIXADE;
pub const _TIF_LOGADE: usize = 1usize << TIF_LOGADE;
pub const _TIF_32BIT_REGS: usize = 1usize << TIF_32BIT_REGS;
pub const _TIF_32BIT_ADDR: usize = 1usize << TIF_32BIT_ADDR;
pub const _TIF_LOAD_WATCH: usize = 1usize << TIF_LOAD_WATCH;
pub const _TIF_SINGLESTEP: usize = 1usize << TIF_SINGLESTEP;
pub const _TIF_LSX_CTX_LIVE: usize = 1usize << TIF_LSX_CTX_LIVE;
pub const _TIF_LASX_CTX_LIVE: usize = 1usize << TIF_LASX_CTX_LIVE;
pub const _TIF_USEDLBT: usize = 1usize << TIF_USEDLBT;
pub const _TIF_LBT_CTX_LIVE: usize = 1usize << TIF_LBT_CTX_LIVE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
