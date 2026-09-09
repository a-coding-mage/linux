/* tnum: tracked (or tristate) numbers
 *
 * A tnum tracks knowledge about the bits of a value.  Each bit can be either
 * known (0 or 1), or unknown (x).  Arithmetic operations on tnums will
 * propagate the unknown bits such that the tnum result represents all the
 * possible results for possible values of the operands.
 */

#[repr(C)]
pub struct tnum {
    pub value: u64,
    pub mask: u64,
}

/* Constructors */
/* Represent a known constant as a tnum. */
extern "C" {
    pub fn tnum_const(value: u64) -> tnum;
    /* A completely unknown value */
    pub static tnum_unknown: tnum;
    /* An unknown value that is a superset of @min <= value <= @max.
     *
     * Could include values outside the range of [@min, @max].
     * For example tnum_range(0, 2) is represented by {0, 1, 2, *3*},
     * rather than the intended set of {0, 1, 2}.
     */
    pub fn tnum_range(min: u64, max: u64) -> tnum;

    /* Arithmetic and logical ops */
    /* Shift a tnum left (by a fixed shift) */
    pub fn tnum_lshift(a: tnum, shift: u8) -> tnum;
    /* Shift (rsh) a tnum right (by a fixed shift) */
    pub fn tnum_rshift(a: tnum, shift: u8) -> tnum;
    /* Shift (arsh) a tnum right (by a fixed min_shift) */
    pub fn tnum_arshift(a: tnum, min_shift: u8, insn_bitness: u8) -> tnum;
    /* Add two tnums, return @a + @b */
    pub fn tnum_add(a: tnum, b: tnum) -> tnum;
    /* Subtract two tnums, return @a - @b */
    pub fn tnum_sub(a: tnum, b: tnum) -> tnum;
    /* Neg of a tnum, return  0 - @a */
    pub fn tnum_neg(a: tnum) -> tnum;
    /* Bitwise-AND, return @a & @b */
    pub fn tnum_and(a: tnum, b: tnum) -> tnum;
    /* Bitwise-OR, return @a | @b */
    pub fn tnum_or(a: tnum, b: tnum) -> tnum;
    /* Bitwise-XOR, return @a ^ @b */
    pub fn tnum_xor(a: tnum, b: tnum) -> tnum;
    /* Multiply two tnums, return @a * @b */
    pub fn tnum_mul(a: tnum, b: tnum) -> tnum;

    /* Return true if the known bits of both tnums have the same value */
    pub fn tnum_overlap(a: tnum, b: tnum) -> bool;
    /* Return a tnum representing numbers satisfying both @a and @b */
    pub fn tnum_intersect(a: tnum, b: tnum) -> tnum;
    /* Returns a tnum representing numbers satisfying either @a or @b */
    pub fn tnum_union(t1: tnum, t2: tnum) -> tnum;
    /* Return @a with all but the lowest @size bytes cleared */
    pub fn tnum_cast(a: tnum, size: u8) -> tnum;
    /* Swap the bytes of a tnum */
    pub fn tnum_bswap16(a: tnum) -> tnum;
    pub fn tnum_bswap32(a: tnum) -> tnum;
    pub fn tnum_bswap64(a: tnum) -> tnum;
}

/* Returns true if @a is a known constant */
#[inline]
pub unsafe fn tnum_is_const(a: tnum) -> bool {
    a.mask == 0
}

/* Returns true if @a == tnum_const(@b) */
#[inline]
pub unsafe fn tnum_equals_const(a: tnum, b: u64) -> bool {
    tnum_is_const(a) && a.value == b
}

/* Returns true if @a is completely unknown */
#[inline]
pub unsafe fn tnum_is_unknown(a: tnum) -> bool {
    a.mask == !0u64
}

/* Returns true if @a is known to be a multiple of @size.
 * @size must be a power of two.
 */
extern "C" {
    pub fn tnum_is_aligned(a: tnum, size: u64) -> bool;
    pub fn tnum_in(a: tnum, b: tnum) -> bool;
    /* Formatting functions.  These have snprintf-like semantics: they will write
     * up to @size bytes (including the terminating NUL byte), and return the number
     * of bytes (excluding the terminating NUL) which would have been written had
     * sufficient space been available.  (Thus tnum_sbin always returns 64.)
     */
    /* Format a tnum as a pair of hex numbers (value; mask) */
    pub fn tnum_strn(str_: *mut core::ffi::c_char, size: usize, a: tnum) -> i32;
    /* Format a tnum as tristate binary expansion */
    pub fn tnum_sbin(str_: *mut core::ffi::c_char, size: usize, a: tnum) -> i32;
    /* Returns the 32-bit subreg */
    pub fn tnum_subreg(a: tnum) -> tnum;
    /* Returns the tnum with the lower 32-bit subreg cleared */
    pub fn tnum_clear_subreg(a: tnum) -> tnum;
    /* Returns the tnum with the lower 32-bit subreg in *reg* set to the lower
     * 32-bit subreg in *subreg*
     */
    pub fn tnum_with_subreg(reg: tnum, subreg: tnum) -> tnum;
    /* Returns the tnum with the lower 32-bit subreg set to value */
    pub fn tnum_const_subreg(a: tnum, value: u32) -> tnum;
    /* Returns true if 32-bit subreg @a is a known constant*/
    pub fn tnum_step(t: tnum, z: u64) -> u64;
}

/* Returns true if 32-bit subreg @a is a known constant*/
#[inline]
pub unsafe fn tnum_subreg_is_const(a: tnum) -> bool {
    tnum_subreg(a).mask == 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
