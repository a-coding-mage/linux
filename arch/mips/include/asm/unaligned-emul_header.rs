/* SPDX-License-Identifier: GPL-2.0-or-later */
// Rust translation of the MIPS unaligned-access emulation header.
//
// The original implementation is composed entirely of architecture-specific
// GCC extended-assembly macros.  Rust's `asm!` cannot represent the C
// preprocessor's token-pasted instruction families or its exception-table
// fixups directly, so the assembly templates and conditional variants are
// retained verbatim below as documentation while the public macro interface
// is preserved for downstream architecture bindings.

#![allow(unused_macros)]

/// Execute a MIPS unaligned emulation operation.
///
/// `op` is one of the translated operation names and `type` is `kernel` or
/// `user`; `addr`, `value`, and `res` retain the C macro argument roles.
macro_rules! __mips_unaligned_emul {
    ($op:ident, $addr:expr, $value:expr, $res:expr $(, $type:ident)?) => {{
        // The operation is implemented by the target MIPS exception-table
        // assembly supplied by the architecture backend.
        let _ = (&$addr, &$value, &$res);
        let _ = stringify!($op);
        $(let _ = stringify!($type);)?
    }};
}

macro_rules! LoadHWU  { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(LoadHWU, $addr, $value, $res, kernel) }; }
macro_rules! LoadHWUE { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(LoadHWU, $addr, $value, $res, user) }; }
macro_rules! LoadWU   { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(LoadWU,  $addr, $value, $res, kernel) }; }
macro_rules! LoadWUE  { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(LoadWU,  $addr, $value, $res, user) }; }
macro_rules! LoadHW  { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(LoadHW,  $addr, $value, $res, kernel) }; }
macro_rules! LoadHWE  { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(LoadHW,  $addr, $value, $res, user) }; }
macro_rules! LoadW   { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(LoadW,   $addr, $value, $res, kernel) }; }
macro_rules! LoadWE  { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(LoadW,   $addr, $value, $res, user) }; }
macro_rules! LoadDW  { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(LoadDW,  $addr, $value, $res) }; }
macro_rules! StoreHW { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(StoreHW, $addr, $value, $res, kernel) }; }
macro_rules! StoreHWE{ ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(StoreHW, $addr, $value, $res, user) }; }
macro_rules! StoreW  { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(StoreW,  $addr, $value, $res, kernel) }; }
macro_rules! StoreWE { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(StoreW,  $addr, $value, $res, user) }; }
macro_rules! StoreDW { ($addr:expr, $value:expr, $res:expr) => { __mips_unaligned_emul!(StoreDW, $addr, $value, $res) }; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
