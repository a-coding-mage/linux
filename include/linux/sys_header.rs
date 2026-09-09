/* SPDX-License-Identifier: GPL-2.0 */

/*
 * This file is no longer used or needed
 */

/*
 * These are system calls that will be removed at some time
 * due to newer versions existing..
 * (please be careful - ibcs2 may need some of these).
 */
/*
 * C preprocessor condition: `notdef` was not enabled in the source.
 * The following legacy syscall aliases are therefore intentionally inactive:
 *
 * #define _sys_waitpid  _sys_old_syscall  /* _sys_wait4 */
 * #define _sys_olduname _sys_old_syscall  /* _sys_newuname */
 * #define _sys_uname    _sys_old_syscall  /* _sys_newuname */
 * #define _sys_stat     _sys_old_syscall  /* _sys_newstat */
 * #define _sys_fstat    _sys_old_syscall  /* _sys_newfstat */
 * #define _sys_lstat    _sys_old_syscall  /* _sys_newlstat */
 * #define _sys_signal   _sys_old_syscall  /* _sys_sigaction */
 * #define _sys_sgetmask _sys_old_syscall  /* _sys_sigprocmask */
 * #define _sys_ssetmask _sys_old_syscall  /* _sys_sigprocmask */
 */

/*
 * These are system calls that haven't been implemented yet
 * but have an entry in the table for future expansion..
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
