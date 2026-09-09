/* SPDX-License-Identifier: GPL-2.0 */

pub const PROM_RESET: i32 = 0;
pub const PROM_EXEC: i32 = 1;
pub const PROM_RESTART: i32 = 2;
pub const PROM_REINIT: i32 = 3;
pub const PROM_REBOOT: i32 = 4;
pub const PROM_AUTOBOOT: i32 = 5;
pub const PROM_OPEN: i32 = 6;
pub const PROM_READ: i32 = 7;
pub const PROM_WRITE: i32 = 8;
pub const PROM_IOCTL: i32 = 9;
pub const PROM_CLOSE: i32 = 10;
pub const PROM_GETCHAR: i32 = 11;
pub const PROM_PUTCHAR: i32 = 12;
pub const PROM_SHOWCHAR: i32 = 13; /* XXX */
pub const PROM_GETS: i32 = 14; /* XXX */
pub const PROM_PUTS: i32 = 15; /* XXX */
pub const PROM_PRINTF: i32 = 16; /* XXX */

/* What are these for? */
pub const PROM_INITPROTO: i32 = 17; /* XXX */
pub const PROM_PROTOENABLE: i32 = 18; /* XXX */
pub const PROM_PROTODISABLE: i32 = 19; /* XXX */
pub const PROM_GETPKT: i32 = 20; /* XXX */
pub const PROM_PUTPKT: i32 = 21; /* XXX */

/* More PROM shit.  Probably has to do with VME RMW cycles??? */
pub const PROM_ORW_RMW: i32 = 22; /* XXX */
pub const PROM_ORH_RMW: i32 = 23; /* XXX */
pub const PROM_ORB_RMW: i32 = 24; /* XXX */
pub const PROM_ANDW_RMW: i32 = 25; /* XXX */
pub const PROM_ANDH_RMW: i32 = 26; /* XXX */
pub const PROM_ANDB_RMW: i32 = 27; /* XXX */

/* Cache handling stuff */
pub const PROM_FLUSHCACHE: i32 = 28; /* XXX */
pub const PROM_CLEARCACHE: i32 = 29; /* XXX */

/* Libc alike stuff */
pub const PROM_SETJMP: i32 = 30; /* XXX */
pub const PROM_LONGJMP: i32 = 31; /* XXX */
pub const PROM_BEVUTLB: i32 = 32; /* XXX */
pub const PROM_GETENV: i32 = 33; /* XXX */
pub const PROM_SETENV: i32 = 34; /* XXX */
pub const PROM_ATOB: i32 = 35; /* XXX */
pub const PROM_STRCMP: i32 = 36; /* XXX */
pub const PROM_STRLEN: i32 = 37; /* XXX */
pub const PROM_STRCPY: i32 = 38; /* XXX */
pub const PROM_STRCAT: i32 = 39; /* XXX */

/* Misc stuff */
pub const PROM_PARSER: i32 = 40; /* XXX */
pub const PROM_RANGE: i32 = 41; /* XXX */
pub const PROM_ARGVIZE: i32 = 42; /* XXX */
pub const PROM_HELP: i32 = 43; /* XXX */

/* Entry points for some PROM commands */
pub const PROM_DUMPCMD: i32 = 44; /* XXX */
pub const PROM_SETENVCMD: i32 = 45; /* XXX */
pub const PROM_UNSETENVCMD: i32 = 46; /* XXX */
pub const PROM_PRINTENVCMD: i32 = 47; /* XXX */
pub const PROM_BEVEXCEPT: i32 = 48; /* XXX */
pub const PROM_ENABLECMD: i32 = 49; /* XXX */
pub const PROM_DISABLECMD: i32 = 50; /* XXX */

pub const PROM_CLEARNOFAULT: i32 = 51; /* XXX */
pub const PROM_NOTIMPLEMENT: i32 = 52; /* XXX */

pub const PROM_NV_GET: i32 = 53; /* XXX */
pub const PROM_NV_SET: i32 = 54; /* XXX */

unsafe extern "C" {
    pub fn prom_getenv(name: *mut core::ffi::c_char) -> *mut core::ffi::c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
