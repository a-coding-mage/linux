/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/* audit.h -- Auditing support */
/* C header translated to Rust; external Linux types and ELF constants remain dependencies. */

pub const AUDIT_GET: u32 = 1000;
pub const AUDIT_SET: u32 = 1001;
pub const AUDIT_LIST: u32 = 1002;
pub const AUDIT_ADD: u32 = 1003;
pub const AUDIT_DEL: u32 = 1004;
pub const AUDIT_USER: u32 = 1005;
pub const AUDIT_LOGIN: u32 = 1006;
pub const AUDIT_WATCH_INS: u32 = 1007;
pub const AUDIT_WATCH_REM: u32 = 1008;
pub const AUDIT_WATCH_LIST: u32 = 1009;
pub const AUDIT_SIGNAL_INFO: u32 = 1010;
pub const AUDIT_ADD_RULE: u32 = 1011;
pub const AUDIT_DEL_RULE: u32 = 1012;
pub const AUDIT_LIST_RULES: u32 = 1013;
pub const AUDIT_TRIM: u32 = 1014;
pub const AUDIT_MAKE_EQUIV: u32 = 1015;
pub const AUDIT_TTY_GET: u32 = 1016;
pub const AUDIT_TTY_SET: u32 = 1017;
pub const AUDIT_SET_FEATURE: u32 = 1018;
pub const AUDIT_GET_FEATURE: u32 = 1019;
pub const AUDIT_FIRST_USER_MSG: u32 = 1100;
pub const AUDIT_USER_AVC: u32 = 1107;
pub const AUDIT_USER_TTY: u32 = 1124;
pub const AUDIT_LAST_USER_MSG: u32 = 1199;
pub const AUDIT_FIRST_USER_MSG2: u32 = 2100;
pub const AUDIT_LAST_USER_MSG2: u32 = 2999;
pub const AUDIT_DAEMON_START: u32 = 1200;
pub const AUDIT_DAEMON_END: u32 = 1201;
pub const AUDIT_DAEMON_ABORT: u32 = 1202;
pub const AUDIT_DAEMON_CONFIG: u32 = 1203;
pub const AUDIT_SYSCALL: u32 = 1300;
pub const AUDIT_PATH: u32 = 1302;
pub const AUDIT_IPC: u32 = 1303;
pub const AUDIT_SOCKETCALL: u32 = 1304;
pub const AUDIT_CONFIG_CHANGE: u32 = 1305;
pub const AUDIT_SOCKADDR: u32 = 1306;
pub const AUDIT_CWD: u32 = 1307;
pub const AUDIT_EXECVE: u32 = 1309;
pub const AUDIT_IPC_SET_PERM: u32 = 1311;
pub const AUDIT_MQ_OPEN: u32 = 1312;
pub const AUDIT_MQ_SENDRECV: u32 = 1313;
pub const AUDIT_MQ_NOTIFY: u32 = 1314;
pub const AUDIT_MQ_GETSETATTR: u32 = 1315;
pub const AUDIT_KERNEL_OTHER: u32 = 1316;
pub const AUDIT_FD_PAIR: u32 = 1317;
pub const AUDIT_OBJ_PID: u32 = 1318;
pub const AUDIT_TTY: u32 = 1319;
pub const AUDIT_EOE: u32 = 1320;
pub const AUDIT_BPRM_FCAPS: u32 = 1321;
pub const AUDIT_CAPSET: u32 = 1322;
pub const AUDIT_MMAP: u32 = 1323;
pub const AUDIT_NETFILTER_PKT: u32 = 1324;
pub const AUDIT_NETFILTER_CFG: u32 = 1325;
pub const AUDIT_SECCOMP: u32 = 1326;
pub const AUDIT_PROCTITLE: u32 = 1327;
pub const AUDIT_FEATURE_CHANGE: u32 = 1328;
pub const AUDIT_REPLACE: u32 = 1329;
pub const AUDIT_KERN_MODULE: u32 = 1330;
pub const AUDIT_FANOTIFY: u32 = 1331;
pub const AUDIT_TIME_INJOFFSET: u32 = 1332;
pub const AUDIT_TIME_ADJNTPVAL: u32 = 1333;
pub const AUDIT_BPF: u32 = 1334;
pub const AUDIT_EVENT_LISTENER: u32 = 1335;
pub const AUDIT_URINGOP: u32 = 1336;
pub const AUDIT_OPENAT2: u32 = 1337;
pub const AUDIT_DM_CTRL: u32 = 1338;
pub const AUDIT_DM_EVENT: u32 = 1339;
pub const AUDIT_AVC: u32 = 1400;
pub const AUDIT_SELINUX_ERR: u32 = 1401;
pub const AUDIT_AVC_PATH: u32 = 1402;
pub const AUDIT_MAC_POLICY_LOAD: u32 = 1403;
pub const AUDIT_MAC_STATUS: u32 = 1404;
pub const AUDIT_MAC_CONFIG_CHANGE: u32 = 1405;
pub const AUDIT_MAC_UNLBL_ALLOW: u32 = 1406;
pub const AUDIT_MAC_CIPSOV4_ADD: u32 = 1407;
pub const AUDIT_MAC_CIPSOV4_DEL: u32 = 1408;
pub const AUDIT_MAC_MAP_ADD: u32 = 1409;
pub const AUDIT_MAC_MAP_DEL: u32 = 1410;
pub const AUDIT_MAC_IPSEC_ADDSA: u32 = 1411;
pub const AUDIT_MAC_IPSEC_DELSA: u32 = 1412;
pub const AUDIT_MAC_IPSEC_ADDSPD: u32 = 1413;
pub const AUDIT_MAC_IPSEC_DELSPD: u32 = 1414;
pub const AUDIT_MAC_IPSEC_EVENT: u32 = 1415;
pub const AUDIT_MAC_UNLBL_STCADD: u32 = 1416;
pub const AUDIT_MAC_UNLBL_STCDEL: u32 = 1417;
pub const AUDIT_MAC_CALIPSO_ADD: u32 = 1418;
pub const AUDIT_MAC_CALIPSO_DEL: u32 = 1419;
pub const AUDIT_IPE_ACCESS: u32 = 1420;
pub const AUDIT_IPE_CONFIG_CHANGE: u32 = 1421;
pub const AUDIT_IPE_POLICY_LOAD: u32 = 1422;
pub const AUDIT_LANDLOCK_ACCESS: u32 = 1423;
pub const AUDIT_LANDLOCK_DOMAIN: u32 = 1424;
pub const AUDIT_MAC_TASK_CONTEXTS: u32 = 1425;
pub const AUDIT_MAC_OBJ_CONTEXTS: u32 = 1426;
pub const AUDIT_FIRST_KERN_ANOM_MSG: u32 = 1700;
pub const AUDIT_LAST_KERN_ANOM_MSG: u32 = 1799;
pub const AUDIT_ANOM_PROMISCUOUS: u32 = 1700;
pub const AUDIT_ANOM_ABEND: u32 = 1701;
pub const AUDIT_ANOM_LINK: u32 = 1702;
pub const AUDIT_ANOM_CREAT: u32 = 1703;
pub const AUDIT_INTEGRITY_DATA: u32 = 1800;
pub const AUDIT_INTEGRITY_METADATA: u32 = 1801;
pub const AUDIT_INTEGRITY_STATUS: u32 = 1802;
pub const AUDIT_INTEGRITY_HASH: u32 = 1803;
pub const AUDIT_INTEGRITY_PCR: u32 = 1804;
pub const AUDIT_INTEGRITY_RULE: u32 = 1805;
pub const AUDIT_INTEGRITY_EVM_XATTR: u32 = 1806;
pub const AUDIT_INTEGRITY_POLICY_RULE: u32 = 1807;
pub const AUDIT_INTEGRITY_USERSPACE: u32 = 1808;
pub const AUDIT_KERNEL: u32 = 2000;

pub const AUDIT_FILTER_USER: u32 = 0x00;
pub const AUDIT_FILTER_TASK: u32 = 0x01;
pub const AUDIT_FILTER_ENTRY: u32 = 0x02;
pub const AUDIT_FILTER_WATCH: u32 = 0x03;
pub const AUDIT_FILTER_EXIT: u32 = 0x04;
pub const AUDIT_FILTER_EXCLUDE: u32 = 0x05;
pub const AUDIT_FILTER_TYPE: u32 = AUDIT_FILTER_EXCLUDE;
pub const AUDIT_FILTER_FS: u32 = 0x06;
pub const AUDIT_FILTER_URING_EXIT: u32 = 0x07;
pub const AUDIT_NR_FILTERS: u32 = 8;
pub const AUDIT_FILTER_PREPEND: u32 = 0x10;
pub const AUDIT_NEVER: u32 = 0;
pub const AUDIT_POSSIBLE: u32 = 1;
pub const AUDIT_ALWAYS: u32 = 2;
pub const AUDIT_MAX_FIELDS: usize = 64;
pub const AUDIT_MAX_KEY_LEN: usize = 256;
pub const AUDIT_BITMASK_SIZE: usize = 64;
#[inline] pub const fn AUDIT_WORD(nr: u32) -> u32 { nr / 32 }
#[inline] pub const fn AUDIT_BIT(nr: u32) -> u32 { 1u32 << (nr - AUDIT_WORD(nr) * 32) }
pub const AUDIT_SYSCALL_CLASSES: u32 = 16;
pub const AUDIT_CLASS_DIR_WRITE: u32 = 0;
pub const AUDIT_CLASS_DIR_WRITE_32: u32 = 1;
pub const AUDIT_CLASS_CHATTR: u32 = 2;
pub const AUDIT_CLASS_CHATTR_32: u32 = 3;
pub const AUDIT_CLASS_READ: u32 = 4;
pub const AUDIT_CLASS_READ_32: u32 = 5;
pub const AUDIT_CLASS_WRITE: u32 = 6;
pub const AUDIT_CLASS_WRITE_32: u32 = 7;
pub const AUDIT_CLASS_SIGNAL: u32 = 8;
pub const AUDIT_CLASS_SIGNAL_32: u32 = 9;
pub const AUDIT_UNUSED_BITS: u32 = 0x07FFFC00;

pub const AUDIT_COMPARE_UID_TO_OBJ_UID: u32 = 1;
pub const AUDIT_COMPARE_GID_TO_OBJ_GID: u32 = 2;
pub const AUDIT_COMPARE_EUID_TO_OBJ_UID: u32 = 3;
pub const AUDIT_COMPARE_EGID_TO_OBJ_GID: u32 = 4;
pub const AUDIT_COMPARE_AUID_TO_OBJ_UID: u32 = 5;
pub const AUDIT_COMPARE_SUID_TO_OBJ_UID: u32 = 6;
pub const AUDIT_COMPARE_SGID_TO_OBJ_GID: u32 = 7;
pub const AUDIT_COMPARE_FSUID_TO_OBJ_UID: u32 = 8;
pub const AUDIT_COMPARE_FSGID_TO_OBJ_GID: u32 = 9;
pub const AUDIT_COMPARE_UID_TO_AUID: u32 = 10;
pub const AUDIT_COMPARE_UID_TO_EUID: u32 = 11;
pub const AUDIT_COMPARE_UID_TO_FSUID: u32 = 12;
pub const AUDIT_COMPARE_UID_TO_SUID: u32 = 13;
pub const AUDIT_COMPARE_AUID_TO_FSUID: u32 = 14;
pub const AUDIT_COMPARE_AUID_TO_SUID: u32 = 15;
pub const AUDIT_COMPARE_AUID_TO_EUID: u32 = 16;
pub const AUDIT_COMPARE_EUID_TO_SUID: u32 = 17;
pub const AUDIT_COMPARE_EUID_TO_FSUID: u32 = 18;
pub const AUDIT_COMPARE_SUID_TO_FSUID: u32 = 19;
pub const AUDIT_COMPARE_GID_TO_EGID: u32 = 20;
pub const AUDIT_COMPARE_GID_TO_FSGID: u32 = 21;
pub const AUDIT_COMPARE_GID_TO_SGID: u32 = 22;
pub const AUDIT_COMPARE_EGID_TO_FSGID: u32 = 23;
pub const AUDIT_COMPARE_EGID_TO_SGID: u32 = 24;
pub const AUDIT_COMPARE_SGID_TO_FSGID: u32 = 25;
pub const AUDIT_MAX_FIELD_COMPARE: u32 = AUDIT_COMPARE_SGID_TO_FSGID;

pub const AUDIT_PID: u32 = 0; pub const AUDIT_UID: u32 = 1; pub const AUDIT_EUID: u32 = 2;
pub const AUDIT_SUID: u32 = 3; pub const AUDIT_FSUID: u32 = 4; pub const AUDIT_GID: u32 = 5;
pub const AUDIT_EGID: u32 = 6; pub const AUDIT_SGID: u32 = 7; pub const AUDIT_FSGID: u32 = 8;
pub const AUDIT_LOGINUID: u32 = 9; pub const AUDIT_PERS: u32 = 10; pub const AUDIT_ARCH: u32 = 11;
pub const AUDIT_MSGTYPE: u32 = 12; pub const AUDIT_SUBJ_USER: u32 = 13; pub const AUDIT_SUBJ_ROLE: u32 = 14;
pub const AUDIT_SUBJ_TYPE: u32 = 15; pub const AUDIT_SUBJ_SEN: u32 = 16; pub const AUDIT_SUBJ_CLR: u32 = 17;
pub const AUDIT_PPID: u32 = 18; pub const AUDIT_OBJ_USER: u32 = 19; pub const AUDIT_OBJ_ROLE: u32 = 20;
pub const AUDIT_OBJ_TYPE: u32 = 21; pub const AUDIT_OBJ_LEV_LOW: u32 = 22; pub const AUDIT_OBJ_LEV_HIGH: u32 = 23;
pub const AUDIT_LOGINUID_SET: u32 = 24; pub const AUDIT_SESSIONID: u32 = 25; pub const AUDIT_FSTYPE: u32 = 26;
pub const AUDIT_DEVMAJOR: u32 = 100; pub const AUDIT_DEVMINOR: u32 = 101; pub const AUDIT_INODE: u32 = 102;
pub const AUDIT_EXIT: u32 = 103; pub const AUDIT_SUCCESS: u32 = 104; pub const AUDIT_WATCH: u32 = 105;
pub const AUDIT_PERM: u32 = 106; pub const AUDIT_DIR: u32 = 107; pub const AUDIT_FILETYPE: u32 = 108;
pub const AUDIT_OBJ_UID: u32 = 109; pub const AUDIT_OBJ_GID: u32 = 110; pub const AUDIT_FIELD_COMPARE: u32 = 111;
pub const AUDIT_EXE: u32 = 112; pub const AUDIT_SADDR_FAM: u32 = 113;
pub const AUDIT_ARG0: u32 = 200; pub const AUDIT_ARG1: u32 = AUDIT_ARG0 + 1;
pub const AUDIT_ARG2: u32 = AUDIT_ARG0 + 2; pub const AUDIT_ARG3: u32 = AUDIT_ARG0 + 3;
pub const AUDIT_FILTERKEY: u32 = 210;
pub const AUDIT_NEGATE: u32 = 0x80000000;
pub const AUDIT_BIT_MASK: u32 = 0x08000000; pub const AUDIT_LESS_THAN: u32 = 0x10000000;
pub const AUDIT_GREATER_THAN: u32 = 0x20000000; pub const AUDIT_NOT_EQUAL: u32 = 0x30000000;
pub const AUDIT_EQUAL: u32 = 0x40000000; pub const AUDIT_BIT_TEST: u32 = AUDIT_BIT_MASK | AUDIT_EQUAL;
pub const AUDIT_LESS_THAN_OR_EQUAL: u32 = AUDIT_LESS_THAN | AUDIT_EQUAL;
pub const AUDIT_GREATER_THAN_OR_EQUAL: u32 = AUDIT_GREATER_THAN | AUDIT_EQUAL;
pub const AUDIT_OPERATORS: u32 = AUDIT_EQUAL | AUDIT_NOT_EQUAL | AUDIT_BIT_MASK;

#[repr(i32)] pub enum AuditOperator { Audit_equal, Audit_not_equal, Audit_bitmask, Audit_bittest, Audit_lt, Audit_gt, Audit_le, Audit_ge, Audit_bad }
pub const AUDIT_STATUS_ENABLED: u32 = 0x0001; pub const AUDIT_STATUS_FAILURE: u32 = 0x0002;
pub const AUDIT_STATUS_PID: u32 = 0x0004; pub const AUDIT_STATUS_RATE_LIMIT: u32 = 0x0008;
pub const AUDIT_STATUS_BACKLOG_LIMIT: u32 = 0x0010; pub const AUDIT_STATUS_BACKLOG_WAIT_TIME: u32 = 0x0020;
pub const AUDIT_STATUS_LOST: u32 = 0x0040; pub const AUDIT_STATUS_BACKLOG_WAIT_TIME_ACTUAL: u32 = 0x0080;
pub const AUDIT_FEATURE_BITMAP_BACKLOG_LIMIT: u32 = 1; pub const AUDIT_FEATURE_BITMAP_BACKLOG_WAIT_TIME: u32 = 2;
pub const AUDIT_FEATURE_BITMAP_EXECUTABLE_PATH: u32 = 4; pub const AUDIT_FEATURE_BITMAP_EXCLUDE_EXTEND: u32 = 8;
pub const AUDIT_FEATURE_BITMAP_SESSIONID_FILTER: u32 = 16; pub const AUDIT_FEATURE_BITMAP_LOST_RESET: u32 = 32;
pub const AUDIT_FEATURE_BITMAP_FILTER_FS: u32 = 64;
pub const AUDIT_FEATURE_BITMAP_ALL: u32 = 127;
pub const AUDIT_VERSION_LATEST: u32 = AUDIT_FEATURE_BITMAP_ALL;
pub const AUDIT_VERSION_BACKLOG_LIMIT: u32 = AUDIT_FEATURE_BITMAP_BACKLOG_LIMIT;
pub const AUDIT_VERSION_BACKLOG_WAIT_TIME: u32 = AUDIT_FEATURE_BITMAP_BACKLOG_WAIT_TIME;
pub const AUDIT_FAIL_SILENT: u32 = 0; pub const AUDIT_FAIL_PRINTK: u32 = 1; pub const AUDIT_FAIL_PANIC: u32 = 2;
pub const __AUDIT_ARCH_CONVENTION_MASK: u32 = 0x30000000;
pub const __AUDIT_ARCH_CONVENTION_MIPS64_N32: u32 = 0x20000000;
pub const __AUDIT_ARCH_64BIT: u32 = 0x80000000; pub const __AUDIT_ARCH_LE: u32 = 0x40000000;

pub const AUDIT_ARCH_AARCH64: u32 = EM_AARCH64 | __AUDIT_ARCH_64BIT | __AUDIT_ARCH_LE;
pub const AUDIT_ARCH_ALPHA: u32 = EM_ALPHA | __AUDIT_ARCH_64BIT | __AUDIT_ARCH_LE;
pub const AUDIT_ARCH_ARCOMPACT: u32 = EM_ARCOMPACT | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_ARCOMPACTBE: u32 = EM_ARCOMPACT;
pub const AUDIT_ARCH_ARCV2: u32 = EM_ARCV2 | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_ARCV2BE: u32 = EM_ARCV2;
pub const AUDIT_ARCH_ARM: u32 = EM_ARM | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_ARMEB: u32 = EM_ARM;
pub const AUDIT_ARCH_C6X: u32 = EM_TI_C6000 | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_C6XBE: u32 = EM_TI_C6000;
pub const AUDIT_ARCH_CRIS: u32 = EM_CRIS | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_CSKY: u32 = EM_CSKY | __AUDIT_ARCH_LE;
pub const AUDIT_ARCH_FRV: u32 = EM_FRV; pub const AUDIT_ARCH_H8300: u32 = EM_H8_300; pub const AUDIT_ARCH_HEXAGON: u32 = EM_HEXAGON;
pub const AUDIT_ARCH_I386: u32 = EM_386 | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_IA64: u32 = EM_IA_64 | __AUDIT_ARCH_64BIT | __AUDIT_ARCH_LE;
pub const AUDIT_ARCH_M32R: u32 = EM_M32R; pub const AUDIT_ARCH_M68K: u32 = EM_68K; pub const AUDIT_ARCH_MICROBLAZE: u32 = EM_MICROBLAZE;
pub const AUDIT_ARCH_MIPS: u32 = EM_MIPS; pub const AUDIT_ARCH_MIPSEL: u32 = EM_MIPS | __AUDIT_ARCH_LE;
pub const AUDIT_ARCH_MIPS64: u32 = EM_MIPS | __AUDIT_ARCH_64BIT;
pub const AUDIT_ARCH_MIPS64N32: u32 = EM_MIPS | __AUDIT_ARCH_64BIT | __AUDIT_ARCH_CONVENTION_MIPS64_N32;
pub const AUDIT_ARCH_MIPSEL64: u32 = EM_MIPS | __AUDIT_ARCH_64BIT | __AUDIT_ARCH_LE;
pub const AUDIT_ARCH_MIPSEL64N32: u32 = EM_MIPS | __AUDIT_ARCH_64BIT | __AUDIT_ARCH_LE | __AUDIT_ARCH_CONVENTION_MIPS64_N32;
pub const AUDIT_ARCH_NDS32: u32 = EM_NDS32 | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_NDS32BE: u32 = EM_NDS32;
pub const AUDIT_ARCH_NIOS2: u32 = EM_ALTERA_NIOS2 | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_OPENRISC: u32 = EM_OPENRISC;
pub const AUDIT_ARCH_PARISC: u32 = EM_PARISC; pub const AUDIT_ARCH_PARISC64: u32 = EM_PARISC | __AUDIT_ARCH_64BIT;
pub const AUDIT_ARCH_PPC: u32 = EM_PPC; pub const AUDIT_ARCH_PPC64: u32 = EM_PPC64 | __AUDIT_ARCH_64BIT;
pub const AUDIT_ARCH_PPC64LE: u32 = EM_PPC64 | __AUDIT_ARCH_64BIT | __AUDIT_ARCH_LE;
pub const AUDIT_ARCH_RISCV32: u32 = EM_RISCV | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_RISCV64: u32 = EM_RISCV | __AUDIT_ARCH_64BIT | __AUDIT_ARCH_LE;
pub const AUDIT_ARCH_S390: u32 = EM_S390; pub const AUDIT_ARCH_S390X: u32 = EM_S390 | __AUDIT_ARCH_64BIT;
pub const AUDIT_ARCH_SH: u32 = EM_SH; pub const AUDIT_ARCH_SHEL: u32 = EM_SH | __AUDIT_ARCH_LE;
pub const AUDIT_ARCH_SH64: u32 = EM_SH | __AUDIT_ARCH_64BIT; pub const AUDIT_ARCH_SHEL64: u32 = EM_SH | __AUDIT_ARCH_64BIT | __AUDIT_ARCH_LE;
pub const AUDIT_ARCH_SPARC: u32 = EM_SPARC; pub const AUDIT_ARCH_SPARC64: u32 = EM_SPARCV9 | __AUDIT_ARCH_64BIT;
pub const AUDIT_ARCH_TILEGX: u32 = EM_TILEGX | __AUDIT_ARCH_64BIT | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_TILEGX32: u32 = EM_TILEGX | __AUDIT_ARCH_LE;
pub const AUDIT_ARCH_TILEPRO: u32 = EM_TILEPRO | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_UNICORE: u32 = EM_UNICORE | __AUDIT_ARCH_LE;
pub const AUDIT_ARCH_X86_64: u32 = EM_X86_64 | __AUDIT_ARCH_64BIT | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_XTENSA: u32 = EM_XTENSA;
pub const AUDIT_ARCH_LOONGARCH32: u32 = EM_LOONGARCH | __AUDIT_ARCH_LE; pub const AUDIT_ARCH_LOONGARCH64: u32 = EM_LOONGARCH | __AUDIT_ARCH_64BIT | __AUDIT_ARCH_LE;
pub const AUDIT_PERM_EXEC: u32 = 1; pub const AUDIT_PERM_WRITE: u32 = 2; pub const AUDIT_PERM_READ: u32 = 4; pub const AUDIT_PERM_ATTR: u32 = 8;
pub const AUDIT_MESSAGE_TEXT_MAX: usize = 8560;

#[repr(i32)] pub enum AuditNlgrps { AUDIT_NLGRP_NONE, AUDIT_NLGRP_READLOG, __AUDIT_NLGRP_MAX }
pub const AUDIT_NLGRP_MAX: i32 = __AUDIT_NLGRP_MAX as i32 - 1;

#[repr(C)] pub union AuditStatusVersion { pub version: u32, pub feature_bitmap: u32 }
#[repr(C)] pub struct audit_status {
    pub mask: u32, pub enabled: u32, pub failure: u32, pub pid: u32, pub rate_limit: u32,
    pub backlog_limit: u32, pub lost: u32, pub backlog: u32, pub version: AuditStatusVersion,
    pub backlog_wait_time: u32, pub backlog_wait_time_actual: u32,
}
pub const AUDIT_FEATURE_VERSION: u32 = 1;
#[repr(C)] pub struct audit_features { pub vers: u32, pub mask: u32, pub features: u32, pub lock: u32 }
pub const AUDIT_FEATURE_ONLY_UNSET_LOGINUID: u32 = 0;
pub const AUDIT_FEATURE_LOGINUID_IMMUTABLE: u32 = 1;
pub const AUDIT_LAST_FEATURE: u32 = AUDIT_FEATURE_LOGINUID_IMMUTABLE;
#[inline] pub const fn audit_feature_valid(x: i32) -> bool { x >= 0 && (x as u32) <= AUDIT_LAST_FEATURE }
#[inline] pub const fn AUDIT_FEATURE_TO_MASK(x: u32) -> u32 { 1u32 << (x & 31) }
#[repr(C)] pub struct audit_tty_status { pub enabled: u32, pub log_passwd: u32 }
pub const AUDIT_UID_UNSET: u32 = u32::MAX; pub const AUDIT_SID_UNSET: u32 = u32::MAX;
#[repr(C)] pub struct audit_rule_data {
    pub flags: u32, pub action: u32, pub field_count: u32,
    pub mask: [u32; AUDIT_BITMASK_SIZE], pub fields: [u32; AUDIT_MAX_FIELDS],
    pub values: [u32; AUDIT_MAX_FIELDS], pub fieldflags: [u32; AUDIT_MAX_FIELDS],
    pub buflen: u32, pub buf: [core::ffi::c_char; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
