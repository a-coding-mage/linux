/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Rust representation of the Nios II assembler macros from asm-macros.h.
 * The macro bodies intentionally retain the original assembler source.
 */

macro_rules! ANDI32 {
    ($reg1:tt, $reg2:tt, $mask:expr) => {{
        if ($mask & 0xffff) != 0 {
            if ($mask & 0xffff0000) != 0 {
                concat!("movhi ", stringify!($reg1), ", %hi(", stringify!($mask), ")\n",
                        "movui ", stringify!($reg1), ", %lo(", stringify!($mask), ")\n",
                        "and ", stringify!($reg1), ", ", stringify!($reg1), ", ", stringify!($reg2))
            } else {
                concat!("andi ", stringify!($reg1), ", ", stringify!($reg2), ", %lo(", stringify!($mask), ")")
            }
        } else {
            concat!("andhi ", stringify!($reg1), ", ", stringify!($reg2), ", %hi(", stringify!($mask), ")")
        }
    }};
}

macro_rules! ORI32 {
    ($reg1:tt, $reg2:tt, $mask:expr) => {{
        if ($mask & 0xffff) != 0 {
            if ($mask & 0xffff0000) != 0 {
                concat!("orhi ", stringify!($reg1), ", ", stringify!($reg2), ", %hi(", stringify!($mask), ")\n",
                        "ori ", stringify!($reg1), ", ", stringify!($reg2), ", %lo(", stringify!($mask), ")")
            } else {
                concat!("ori ", stringify!($reg1), ", ", stringify!($reg2), ", %lo(", stringify!($mask), ")")
            }
        } else {
            concat!("orhi ", stringify!($reg1), ", ", stringify!($reg2), ", %hi(", stringify!($mask), ")")
        }
    }};
}

macro_rules! XORI32 {
    ($reg1:tt, $reg2:tt, $mask:expr) => {{
        if ($mask & 0xffff) != 0 {
            if ($mask & 0xffff0000) != 0 {
                concat!("xorhi ", stringify!($reg1), ", ", stringify!($reg2), ", %hi(", stringify!($mask), ")\n",
                        "xori ", stringify!($reg1), ", ", stringify!($reg1), ", %lo(", stringify!($mask), ")")
            } else {
                concat!("xori ", stringify!($reg1), ", ", stringify!($reg2), ", %lo(", stringify!($mask), ")")
            }
        } else {
            concat!("xorhi ", stringify!($reg1), ", ", stringify!($reg2), ", %hi(", stringify!($mask), ")")
        }
    }};
}

macro_rules! BT {
    ($reg1:tt, $reg2:tt, $bit:expr) => {{
        if $bit > 31 { concat!(".err") }
        else if $bit < 16 { concat!("andi ", stringify!($reg1), ", ", stringify!($reg2), ", (1 << ", stringify!($bit), ")") }
        else { concat!("andhi ", stringify!($reg1), ", ", stringify!($reg2), ", (1 << (", stringify!($bit), " - 16))") }
    }};
}

macro_rules! BTBZ { ($r1:tt, $r2:tt, $b:expr, $l:tt) => { (BT!($r1, $r2, $b), concat!("\nbeq ", stringify!($r1), ", r0, ", stringify!($l))) }; }
macro_rules! BTBNZ { ($r1:tt, $r2:tt, $b:expr, $l:tt) => { (BT!($r1, $r2, $b), concat!("\nbne ", stringify!($r1), ", r0, ", stringify!($l))) }; }

macro_rules! BTC { ($r1:tt, $r2:tt, $b:expr) => { if $b < 16 { concat!("andi ", stringify!($r1), ", ", stringify!($r2), ", (1 << ", stringify!($b), ")\nxori ", stringify!($r2), ", ", stringify!($r2), ", (1 << ", stringify!($b), ")") } else { concat!("andhi ", stringify!($r1), ", ", stringify!($r2), ", (1 << (", stringify!($b), " - 16))\nxorhi ", stringify!($r2), ", ", stringify!($r2), ", (1 << (", stringify!($b), " - 16))") } }; }
macro_rules! BTS { ($r1:tt, $r2:tt, $b:expr) => { if $b < 16 { concat!("andi ", stringify!($r1), ", ", stringify!($r2), ", (1 << ", stringify!($b), ")\nori ", stringify!($r2), ", ", stringify!($r2), ", (1 << ", stringify!($b), ")") } else { concat!("andhi ", stringify!($r1), ", ", stringify!($r2), ", (1 << (", stringify!($b), " - 16))\norhi ", stringify!($r2), ", ", stringify!($r2), ", (1 << (", stringify!($b), " - 16))") } }; }
macro_rules! BTR { ($r1:tt, $r2:tt, $b:expr) => { if $b < 16 { concat!("andi ", stringify!($r1), ", ", stringify!($r2), ", (1 << ", stringify!($b), ")\nandi ", stringify!($r2), ", ", stringify!($r2), ", %lo(~(1 << ", stringify!($b), "))") } else { concat!("andhi ", stringify!($r1), ", ", stringify!($r2), ", (1 << (", stringify!($b), " - 16))\nandhi ", stringify!($r2), ", ", stringify!($r2), ", %lo(~(1 << (", stringify!($b), " - 16)))") } }; }

macro_rules! BTCBZ { ($r1:tt, $r2:tt, $b:expr, $l:tt) => { (BTC!($r1, $r2, $b), concat!("\nbeq ", stringify!($r1), ", r0, ", stringify!($l))) }; }
macro_rules! BTCBNZ { ($r1:tt, $r2:tt, $b:expr, $l:tt) => { (BTC!($r1, $r2, $b), concat!("\nbne ", stringify!($r1), ", r0, ", stringify!($l))) }; }
macro_rules! BTSBZ { ($r1:tt, $r2:tt, $b:expr, $l:tt) => { (BTS!($r1, $r2, $b), concat!("\nbeq ", stringify!($r1), ", r0, ", stringify!($l))) }; }
macro_rules! BTSBNZ { ($r1:tt, $r2:tt, $b:expr, $l:tt) => { (BTS!($r1, $r2, $b), concat!("\nbne ", stringify!($r1), ", r0, ", stringify!($l))) }; }
macro_rules! BTRBZ { ($r1:tt, $r2:tt, $b:expr, $l:tt) => { (BTR!($r1, $r2, $b), concat!("\nbne ", stringify!($r1), ", r0, ", stringify!($l))) }; }
macro_rules! BTRBNZ { ($r1:tt, $r2:tt, $b:expr, $l:tt) => { (BTR!($r1, $r2, $b), concat!("\nbne ", stringify!($r1), ", r0, ", stringify!($l))) }; }

macro_rules! TSTBZ { ($r1:tt, $r2:tt, $m:expr, $l:tt) => { (ANDI32!($r1, $r2, $m), concat!("\nbeq ", stringify!($r1), ", r0, ", stringify!($l))) }; }
macro_rules! TSTBNZ { ($r1:tt, $r2:tt, $m:expr, $l:tt) => { (ANDI32!($r1, $r2, $m), concat!("\nbne ", stringify!($r1), ", r0, ", stringify!($l))) }; }
macro_rules! PUSH { ($r:tt) => { concat!("addi sp, sp, -4\nstw ", stringify!($r), ", 0(sp)") }; }
macro_rules! POP { ($r:tt) => { concat!("ldw ", stringify!($r), ", 0(sp)\naddi sp, sp, 4") }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
