/*
 * Copyright (C) 2000 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Licensed under the GPL
 */

macro_rules! UPT_SYSCALL_ARG1 {
    ($r:expr) => { UPT_BX!($r) };
}

macro_rules! UPT_SYSCALL_ARG2 {
    ($r:expr) => { UPT_CX!($r) };
}

macro_rules! UPT_SYSCALL_ARG3 {
    ($r:expr) => { UPT_DX!($r) };
}

macro_rules! UPT_SYSCALL_ARG4 {
    ($r:expr) => { UPT_SI!($r) };
}

macro_rules! UPT_SYSCALL_ARG5 {
    ($r:expr) => { UPT_DI!($r) };
}

macro_rules! UPT_SYSCALL_ARG6 {
    ($r:expr) => { UPT_BP!($r) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
