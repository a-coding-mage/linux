/*
 *\tFILE    \tbitfield.h
 *
 *\tVersion \t1.1
 *\tAuthor  \tCopyright (c) Marc A. Viredaz, 1998
 *\t        \tDEC Western Research Laboratory, Palo Alto, CA
 *\tDate    \tApril 1998 (April 1997)
 *\tSystem  \tAdvanced RISC Machine (ARM)
 *\tLanguage\tC or ARM Assembly
 *\tPurpose \tDefinition of macros to operate on bit fields.
 */

/* UData is an unsigned-long conversion in C.  The assembly variant is a
 * no-op; this Rust header represents the C translation. */
macro_rules! UData {
    ($data:expr) => {
        ($data as u32)
    };
}

/* Encode a bit field from its size and shift value. */
macro_rules! Fld {
    ($size:expr, $shft:expr) => {
        (($size << 16) + $shft)
    };
}

/* Return the size of an encoded bit field. */
macro_rules! FSize {
    ($field:expr) => {
        ($field >> 16)
    };
}

/* Return the shift value of an encoded bit field. */
macro_rules! FShft {
    ($field:expr) => {
        ($field & 0x0000_FFFF)
    };
}

/* Return the mask for an encoded bit field. */
macro_rules! FMsk {
    ($field:expr) => {
        ((UData!(1) << FSize!($field)) - 1) << FShft!($field)
    };
}

/* Return the mask aligned on bit 0 for an encoded bit field. */
macro_rules! FAlnMsk {
    ($field:expr) => {
        (UData!(1) << FSize!($field)) - 1
    };
}

/* Return the first bit of an encoded bit field. */
macro_rules! F1stBit {
    ($field:expr) => {
        UData!(1) << FShft!($field)
    };
}

/* Insert a value into an encoded bit field. */
macro_rules! FInsrt {
    ($value:expr, $field:expr) => {
        UData!($value) << FShft!($field)
    };
}

/* Extract a value from an encoded bit field. */
macro_rules! FExtr {
    ($data:expr, $field:expr) => {
        (UData!($data) >> FShft!($field)) & FAlnMsk!($field)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
