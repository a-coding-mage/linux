// Dependencies: signal constants from linux/signal.h and "signal.h"
// MAXMAPPED_SIG and MAXMAPPED_SIGNAME constants
// Signal constants: SIGHUP, SIGINT, SIGQUIT, SIGILL, SIGTRAP, SIGABRT, SIGBUS, SIGFPE,
// SIGKILL, SIGUSR1, SIGSEGV, SIGUSR2, SIGPIPE, SIGALRM, SIGTERM, SIGSTKFLT (conditional),
// SIGCHLD, SIGCONT, SIGSTOP, SIGTSTP, SIGTTIN, SIGTTOU, SIGURG, SIGXCPU, SIGXFSZ,
// SIGVTALRM, SIGPROF, SIGWINCH, SIGIO, SIGPWR, SIGSYS (conditional), SIGEMT (conditional),
// SIGLOST (conditional), SIGUNUSED (conditional)

// provide a mapping of arch signal to internal signal # for mediation
// those that are always an alias SIGCLD for SIGCLHD and SIGPOLL for SIGIO
// map to the same entry those that may/or may not get a separate entry
pub const SIG_MAP: [i32; MAXMAPPED_SIG as usize] = [
    MAXMAPPED_SIG,   // [0] existence test
    1,               // [SIGHUP]
    2,               // [SIGINT]
    3,               // [SIGQUIT]
    4,               // [SIGILL]
    5,               // [SIGTRAP] -, 5, -
    6,               // [SIGABRT] SIGIOT: -, 6, -
    7,               // [SIGBUS] 10, 7, 10
    8,               // [SIGFPE]
    9,               // [SIGKILL]
    10,              // [SIGUSR1] 30, 10, 16
    11,              // [SIGSEGV]
    12,              // [SIGUSR2] 31, 12, 17
    13,              // [SIGPIPE]
    14,              // [SIGALRM]
    15,              // [SIGTERM]
    #[cfg(target_os = "linux")]
    16,              // [SIGSTKFLT] -, 16, -
    17,              // [SIGCHLD] 20, 17, 18. SIGCHLD -, -, 18
    18,              // [SIGCONT] 19, 18, 25
    19,              // [SIGSTOP] 17, 19, 23
    20,              // [SIGTSTP] 18, 20, 24
    21,              // [SIGTTIN] 21, 21, 26
    22,              // [SIGTTOU] 22, 22, 27
    23,              // [SIGURG] 16, 23, 21
    24,              // [SIGXCPU] 24, 24, 30
    25,              // [SIGXFSZ] 25, 25, 31
    26,              // [SIGVTALRM] 26, 26, 28
    27,              // [SIGPROF] 27, 27, 29
    28,              // [SIGWINCH] 28, 28, 20
    29,              // [SIGIO] SIGPOLL: 23, 29, 22
    30,              // [SIGPWR] 29, 30, 19. SIGINFO 29, -, -
    #[cfg(any(target_os = "linux", target_os = "unix"))]
    31,              // [SIGSYS] 12, 31, 12. often SIG LOST/UNUSED
    #[cfg(any(target_os = "bsd", target_os = "macos"))]
    32,              // [SIGEMT] 7, -, 7
    #[cfg(target_arch = "sparc")]
    33,              // [SIGLOST] unused on Linux
    #[cfg(all(target_os = "linux", target_arch = "sparc"))]
    34,              // [SIGUNUSED] -, 31, -
];

// this table is ordered post sig_map[sig] mapping
pub const SIG_NAMES: &[&str] = &[
    "unknown",
    "hup",
    "int",
    "quit",
    "ill",
    "trap",
    "abrt",
    "bus",
    "fpe",
    "kill",
    "usr1",
    "segv",
    "usr2",
    "pipe",
    "alrm",
    "term",
    "stkflt",
    "chld",
    "cont",
    "stop",
    "stp",
    "ttin",
    "ttou",
    "urg",
    "xcpu",
    "xfsz",
    "vtalrm",
    "prof",
    "winch",
    "io",
    "pwr",
    "sys",
    "emt",
    "lost",
    "unused",
    "exists",        // always last existence test mapped to MAXMAPPED_SIG
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
