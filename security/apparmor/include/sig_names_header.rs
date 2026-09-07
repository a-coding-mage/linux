// Dependencies: signal constants from linux/signal.h and "signal.h"

// Provide a mapping of arch signal to internal signal # for mediation.
const fn make_sig_map() -> [i32; MAXMAPPED_SIG as usize] {
    let mut map = [0; MAXMAPPED_SIG as usize];
    map[0] = MAXMAPPED_SIG;
    map[SIGHUP as usize] = 1;
    map[SIGINT as usize] = 2;
    map[SIGQUIT as usize] = 3;
    map[SIGILL as usize] = 4;
    map[SIGTRAP as usize] = 5;
    map[SIGABRT as usize] = 6;
    map[SIGBUS as usize] = 7;
    map[SIGFPE as usize] = 8;
    map[SIGKILL as usize] = 9;
    map[SIGUSR1 as usize] = 10;
    map[SIGSEGV as usize] = 11;
    map[SIGUSR2 as usize] = 12;
    map[SIGPIPE as usize] = 13;
    map[SIGALRM as usize] = 14;
    map[SIGTERM as usize] = 15;
    // #ifdef SIGSTKFLT
    #[cfg(target_os = "linux")]
    { map[SIGSTKFLT as usize] = 16; }
    // #endif
    map[SIGCHLD as usize] = 17;
    map[SIGCONT as usize] = 18;
    map[SIGSTOP as usize] = 19;
    map[SIGTSTP as usize] = 20;
    map[SIGTTIN as usize] = 21;
    map[SIGTTOU as usize] = 22;
    map[SIGURG as usize] = 23;
    map[SIGXCPU as usize] = 24;
    map[SIGXFSZ as usize] = 25;
    map[SIGVTALRM as usize] = 26;
    map[SIGPROF as usize] = 27;
    map[SIGWINCH as usize] = 28;
    map[SIGIO as usize] = 29;
    map[SIGPWR as usize] = 30;
    // #ifdef SIGSYS
    map[SIGSYS as usize] = 31;
    // #endif
    // #ifdef SIGEMT
    map[SIGEMT as usize] = 32;
    // #endif
    // #if defined(SIGLOST) && SIGPWR != SIGLOST
    map[SIGLOST as usize] = 33;
    // #endif
    // #if defined(SIGUNUSED) && defined(SIGLOST) && defined(SIGSYS) && SIGLOST != SIGSYS
    map[SIGUNUSED as usize] = 34;
    // #endif
    map
}

static SIG_MAP: [i32; MAXMAPPED_SIG as usize] = make_sig_map();

// This table is ordered post sig_map[sig] mapping.
static SIG_NAMES: [&str; MAXMAPPED_SIGNAME as usize] = [
    "unknown", "hup", "int", "quit", "ill", "trap", "abrt", "bus", "fpe", "kill",
    "usr1", "segv", "usr2", "pipe", "alrm", "term", "stkflt", "chld", "cont", "stop",
    "stp", "ttin", "ttou", "urg", "xcpu", "xfsz", "vtalrm", "prof", "winch", "io",
    "pwr", "sys", "emt", "lost", "unused", "exists", // always last existence test
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
