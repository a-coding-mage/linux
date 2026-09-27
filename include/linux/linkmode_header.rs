// Translated from linux/linkmode.h.
// Dependencies supplied by the Linux bitmap and ethtool headers are referenced
// here but are not implemented in this translation unit.

extern "C" {
    fn bitmap_zero(dst: *mut kernel::ffi::c_ulong, nbits: kernel::ffi::c_uint);
    fn bitmap_fill(dst: *mut kernel::ffi::c_ulong, nbits: kernel::ffi::c_uint);
    fn bitmap_copy(
        dst: *mut kernel::ffi::c_ulong,
        src: *const kernel::ffi::c_ulong,
        nbits: kernel::ffi::c_uint,
    );
    fn bitmap_and(
        dst: *mut kernel::ffi::c_ulong,
        a: *const kernel::ffi::c_ulong,
        b: *const kernel::ffi::c_ulong,
        nbits: kernel::ffi::c_uint,
    );
    fn bitmap_or(
        dst: *mut kernel::ffi::c_ulong,
        a: *const kernel::ffi::c_ulong,
        b: *const kernel::ffi::c_ulong,
        nbits: kernel::ffi::c_uint,
    );
    fn bitmap_empty(src: *const kernel::ffi::c_ulong, nbits: kernel::ffi::c_uint) -> bool;
    fn bitmap_andnot(
        dst: *mut kernel::ffi::c_ulong,
        src1: *const kernel::ffi::c_ulong,
        src2: *const kernel::ffi::c_ulong,
        nbits: kernel::ffi::c_uint,
    ) -> bool;
    fn test_bit(nr: kernel::ffi::c_ulong, addr: *const kernel::ffi::c_ulong) -> bool;
    fn __set_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong);
    fn __clear_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong);
    fn __assign_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong, value: bool);
    fn bitmap_equal(
        src1: *const kernel::ffi::c_ulong,
        src2: *const kernel::ffi::c_ulong,
        nbits: kernel::ffi::c_uint,
    ) -> kernel::ffi::c_int;
    fn bitmap_intersects(
        src1: *const kernel::ffi::c_ulong,
        src2: *const kernel::ffi::c_ulong,
        nbits: kernel::ffi::c_uint,
    ) -> kernel::ffi::c_int;
    fn bitmap_subset(
        src1: *const kernel::ffi::c_ulong,
        src2: *const kernel::ffi::c_ulong,
        nbits: kernel::ffi::c_uint,
    ) -> kernel::ffi::c_int;
}

#[inline]
pub unsafe fn linkmode_zero(dst: *mut kernel::ffi::c_ulong) {
    bitmap_zero(dst, __ETHTOOL_LINK_MODE_MASK_NBITS);
}

#[inline]
pub unsafe fn linkmode_fill(dst: *mut kernel::ffi::c_ulong) {
    bitmap_fill(dst, __ETHTOOL_LINK_MODE_MASK_NBITS);
}

#[inline]
pub unsafe fn linkmode_copy(dst: *mut kernel::ffi::c_ulong, src: *const kernel::ffi::c_ulong) {
    bitmap_copy(dst, src, __ETHTOOL_LINK_MODE_MASK_NBITS);
}

#[inline]
pub unsafe fn linkmode_and(
    dst: *mut kernel::ffi::c_ulong,
    a: *const kernel::ffi::c_ulong,
    b: *const kernel::ffi::c_ulong,
) {
    bitmap_and(dst, a, b, __ETHTOOL_LINK_MODE_MASK_NBITS);
}

#[inline]
pub unsafe fn linkmode_or(
    dst: *mut kernel::ffi::c_ulong,
    a: *const kernel::ffi::c_ulong,
    b: *const kernel::ffi::c_ulong,
) {
    bitmap_or(dst, a, b, __ETHTOOL_LINK_MODE_MASK_NBITS);
}

#[inline]
pub unsafe fn linkmode_empty(src: *const kernel::ffi::c_ulong) -> bool {
    bitmap_empty(src, __ETHTOOL_LINK_MODE_MASK_NBITS)
}

#[inline]
pub unsafe fn linkmode_andnot(
    dst: *mut kernel::ffi::c_ulong,
    src1: *const kernel::ffi::c_ulong,
    src2: *const kernel::ffi::c_ulong,
) -> bool {
    bitmap_andnot(dst, src1, src2, __ETHTOOL_LINK_MODE_MASK_NBITS)
}

#[inline]
pub unsafe fn linkmode_test_bit(nr: kernel::ffi::c_ulong, addr: *const kernel::ffi::c_ulong) -> bool {
    test_bit(nr, addr)
}

#[inline]
pub unsafe fn linkmode_set_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) {
    __set_bit(nr, addr);
}

#[inline]
pub unsafe fn linkmode_clear_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) {
    __clear_bit(nr, addr);
}

#[inline]
pub unsafe fn linkmode_mod_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong, value: bool) {
    __assign_bit(nr, addr, value);
}

#[inline]
pub unsafe fn linkmode_set_bit_array(
    array: *const kernel::ffi::c_int,
    array_size: kernel::ffi::c_int,
    addr: *mut kernel::ffi::c_ulong,
) {
    let mut i: kernel::ffi::c_int = 0;
    while i < array_size {
        linkmode_set_bit(*array.add(i as usize) as kernel::ffi::c_ulong, addr);
        i += 1;
    }
}

#[inline]
pub unsafe fn linkmode_equal(
    src1: *const kernel::ffi::c_ulong,
    src2: *const kernel::ffi::c_ulong,
) -> kernel::ffi::c_int {
    bitmap_equal(src1, src2, __ETHTOOL_LINK_MODE_MASK_NBITS)
}

#[inline]
pub unsafe fn linkmode_intersects(
    src1: *const kernel::ffi::c_ulong,
    src2: *const kernel::ffi::c_ulong,
) -> kernel::ffi::c_int {
    bitmap_intersects(src1, src2, __ETHTOOL_LINK_MODE_MASK_NBITS)
}

#[inline]
pub unsafe fn linkmode_subset(
    src1: *const kernel::ffi::c_ulong,
    src2: *const kernel::ffi::c_ulong,
) -> kernel::ffi::c_int {
    bitmap_subset(src1, src2, __ETHTOOL_LINK_MODE_MASK_NBITS)
}

extern "C" {
    pub fn linkmode_resolve_pause(
        local_adv: *const kernel::ffi::c_ulong,
        partner_adv: *const kernel::ffi::c_ulong,
        tx_pause: *mut bool,
        rx_pause: *mut bool,
    );

    pub fn linkmode_set_pause(
        advertisement: *mut kernel::ffi::c_ulong,
        tx: bool,
        rx: bool,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
