/* SPDX-License-Identifier: GPL-2.0-or-later */

// Dependency intent: __u32 and __u64 are supplied by linux/types.h.

const IOC_NRBITS: u32 = 8;
const IOC_TYPEBITS: u32 = 8;
const IOC_SIZEBITS: u32 = 14;
const IOC_NRSHIFT: u32 = 0;
const IOC_TYPESHIFT: u32 = IOC_NRSHIFT + IOC_NRBITS;
const IOC_SIZESHIFT: u32 = IOC_TYPESHIFT + IOC_TYPEBITS;
const IOC_DIRSHIFT: u32 = IOC_SIZESHIFT + IOC_SIZEBITS;
const IOC_WRITE: u32 = 1;
const IOC_READ: u32 = 2;

const fn ioc(dir: u32, ty: u32, nr: u32, size: u32) -> u32 {
    (dir << IOC_DIRSHIFT)
        | (ty << IOC_TYPESHIFT)
        | (nr << IOC_NRSHIFT)
        | (size << IOC_SIZESHIFT)
}

const fn iowr<T>(nr: u32) -> u32 {
    ioc(IOC_READ | IOC_WRITE, b'g' as u32, nr, core::mem::size_of::<T>() as u32)
}

const fn iow<T>(nr: u32) -> u32 {
    ioc(IOC_WRITE, b'g' as u32, nr, core::mem::size_of::<T>() as u32)
}

const fn ior<T>(nr: u32) -> u32 {
    ioc(IOC_READ, b'g' as u32, nr, core::mem::size_of::<T>() as u32)
}

const fn io(nr: u32) -> u32 {
    ioc(0, b'g' as u32, nr, 0)
}

pub const GUP_TEST_MAX_PAGES_TO_DUMP: usize = 8;
pub const GUP_TEST_FLAG_DUMP_PAGES_USE_PIN: u32 = 0x1;

#[repr(C)]
pub struct gup_test {
    pub get_delta_usec: u64,
    pub put_delta_usec: u64,
    pub addr: u64,
    pub size: u64,
    pub nr_pages_per_call: u32,
    pub gup_flags: u32,
    pub test_flags: u32,
    /*
     * Each non-zero entry is the number of the page (1-based: first page is
     * page 1, so that zero entries mean "do nothing") from the .addr base.
     */
    pub which_pages: [u32; GUP_TEST_MAX_PAGES_TO_DUMP],
}

#[repr(C)]
pub struct pin_longterm_test {
    pub addr: u64,
    pub size: u64,
    pub flags: u32,
}

pub const GUP_FAST_BENCHMARK: u32 = iowr::<gup_test>(1);
pub const PIN_FAST_BENCHMARK: u32 = ioc(IOC_READ | IOC_WRITE, b'g' as u32, 2, core::mem::size_of::<gup_test>() as u32);
pub const PIN_LONGTERM_BENCHMARK: u32 = ioc(IOC_READ | IOC_WRITE, b'g' as u32, 3, core::mem::size_of::<gup_test>() as u32);
pub const GUP_BASIC_TEST: u32 = ioc(IOC_READ | IOC_WRITE, b'g' as u32, 4, core::mem::size_of::<gup_test>() as u32);
pub const PIN_BASIC_TEST: u32 = ioc(IOC_READ | IOC_WRITE, b'g' as u32, 5, core::mem::size_of::<gup_test>() as u32);
pub const DUMP_USER_PAGES_TEST: u32 = ioc(IOC_READ | IOC_WRITE, b'g' as u32, 6, core::mem::size_of::<gup_test>() as u32);
pub const PIN_LONGTERM_TEST_START: u32 = iow::<pin_longterm_test>(7);
pub const PIN_LONGTERM_TEST_STOP: u32 = io(8);
pub const PIN_LONGTERM_TEST_READ: u32 = iow::<u64>(9);

pub const PIN_LONGTERM_TEST_FLAG_USE_WRITE: u32 = 1;
pub const PIN_LONGTERM_TEST_FLAG_USE_FAST: u32 = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
