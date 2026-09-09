// Translated from linux/linkmode.h.
// Dependencies supplied by the Linux bitmap and ethtool headers are referenced
// here but are not implemented in this translation unit.

extern "C" {
    fn bitmap_zero(dst: *mut libc::c_ulong, nbits: libc::c_uint);
    fn bitmap_fill(dst: *mut libc::c_ulong, nbits: libc::c_uint);
    fn bitmap_copy(
        dst: *mut libc::c_ulong,
        src: *const libc::c_ulong,
        nbits: libc::c_uint,
    );
    fn bitmap_and(
        dst: *mut libc::c_ulong,
        a: *const libc::c_ulong,
        b: *const libc::c_ulong,
        nbits: libc::c_uint,
    );
    fn bitmap_or(
        dst: *mut libc::c_ulong,
        a: *const libc::c_ulong,
        b: *const libc::c_ulong,
        nbits: libc::c_uint,
    );
    fn bitmap_empty(src: *const libc::c_ulong, nbits: libc::c_uint) -> bool;
    fn bitmap_andnot(
        dst: *mut libc::c_ulong,
        src1: *const libc::c_ulong,
        src2: *const libc::c_ulong,
        nbits: libc::c_uint,
    ) -> bool;
    fn test_bit(nr: libc::c_ulong, addr: *const libc::c_ulong) -> bool;
    fn __set_bit(nr: libc::c_ulong, addr: *mut libc::c_ulong);
    fn __clear_bit(nr: libc::c_ulong, addr: *mut libc::c_ulong);
    fn __assign_bit(nr: libc::c_ulong, addr: *mut libc::c_ulong, value: bool);
    fn bitmap_equal(
        src1: *const libc::c_ulong,
        src2: *const libc::c_ulong,
        nbits: libc::c_uint,
    ) -> libc::c_int;
    fn bitmap_intersects(
        src1: *const libc::c_ulong,
        src2: *const libc::c_ulong,
        nbits: libc::c_uint,
    ) -> libc::c_int;
    fn bitmap_subset(
        src1: *const libc::c_ulong,
        src2: *const libc::c_ulong,
        nbits: libc::c_uint,
    ) -> libc::c_int;
}

#[inline]
pub unsafe fn linkmode_zero(dst: *mut libc::c_ulong) {
    bitmap_zero(dst, __ETHTOOL_LINK_MODE_MASK_NBITS);
}

#[inline]
pub unsafe fn linkmode_fill(dst: *mut libc::c_ulong) {
    bitmap_fill(dst, __ETHTOOL_LINK_MODE_MASK_NBITS);
}

#[inline]
pub unsafe fn linkmode_copy(dst: *mut libc::c_ulong, src: *const libc::c_ulong) {
    bitmap_copy(dst, src, __ETHTOOL_LINK_MODE_MASK_NBITS);
}

#[inline]
pub unsafe fn linkmode_and(
    dst: *mut libc::c_ulong,
    a: *const libc::c_ulong,
    b: *const libc::c_ulong,
) {
    bitmap_and(dst, a, b, __ETHTOOL_LINK_MODE_MASK_NBITS);
}

#[inline]
pub unsafe fn linkmode_or(
    dst: *mut libc::c_ulong,
    a: *const libc::c_ulong,
    b: *const libc::c_ulong,
) {
    bitmap_or(dst, a, b, __ETHTOOL_LINK_MODE_MASK_NBITS);
}

#[inline]
pub unsafe fn linkmode_empty(src: *const libc::c_ulong) -> bool {
    bitmap_empty(src, __ETHTOOL_LINK_MODE_MASK_NBITS)
}

#[inline]
pub unsafe fn linkmode_andnot(
    dst: *mut libc::c_ulong,
    src1: *const libc::c_ulong,
    src2: *const libc::c_ulong,
) -> bool {
    bitmap_andnot(dst, src1, src2, __ETHTOOL_LINK_MODE_MASK_NBITS)
}

#[inline]
pub unsafe fn linkmode_test_bit(nr: libc::c_ulong, addr: *const libc::c_ulong) -> bool {
    test_bit(nr, addr)
}

#[inline]
pub unsafe fn linkmode_set_bit(nr: libc::c_ulong, addr: *mut libc::c_ulong) {
    __set_bit(nr, addr);
}

#[inline]
pub unsafe fn linkmode_clear_bit(nr: libc::c_ulong, addr: *mut libc::c_ulong) {
    __clear_bit(nr, addr);
}

#[inline]
pub unsafe fn linkmode_mod_bit(nr: libc::c_ulong, addr: *mut libc::c_ulong, value: bool) {
    __assign_bit(nr, addr, value);
}

#[inline]
pub unsafe fn linkmode_set_bit_array(
    array: *const libc::c_int,
    array_size: libc::c_int,
    addr: *mut libc::c_ulong,
) {
    let mut i: libc::c_int = 0;
    while i < array_size {
        linkmode_set_bit(*array.add(i as usize) as libc::c_ulong, addr);
        i += 1;
    }
}

#[inline]
pub unsafe fn linkmode_equal(
    src1: *const libc::c_ulong,
    src2: *const libc::c_ulong,
) -> libc::c_int {
    bitmap_equal(src1, src2, __ETHTOOL_LINK_MODE_MASK_NBITS)
}

#[inline]
pub unsafe fn linkmode_intersects(
    src1: *const libc::c_ulong,
    src2: *const libc::c_ulong,
) -> libc::c_int {
    bitmap_intersects(src1, src2, __ETHTOOL_LINK_MODE_MASK_NBITS)
}

#[inline]
pub unsafe fn linkmode_subset(
    src1: *const libc::c_ulong,
    src2: *const libc::c_ulong,
) -> libc::c_int {
    bitmap_subset(src1, src2, __ETHTOOL_LINK_MODE_MASK_NBITS)
}

extern "C" {
    pub fn linkmode_resolve_pause(
        local_adv: *const libc::c_ulong,
        partner_adv: *const libc::c_ulong,
        tx_pause: *mut bool,
        rx_pause: *mut bool,
    );

    pub fn linkmode_set_pause(
        advertisement: *mut libc::c_ulong,
        tx: bool,
        rx: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
