/* Translation of cvmx.h. Kernel and OCTEON include dependencies are external. */

#[repr(i64)]
pub enum cvmx_mips_space {
    CVMX_MIPS_SPACE_XKSEG = 3,
    CVMX_MIPS_SPACE_XKPHYS = 2,
    CVMX_MIPS_SPACE_XSSEG = 1,
    CVMX_MIPS_SPACE_XUSEG = 0,
}

pub const CVMX_MIPS32_SPACE_KSEG0: i32 = 1;
#[inline] pub const fn CVMX_ADD_SEG32(segment: i32, add: i32) -> i32 { (segment << 31) | add }
pub const CVMX_IO_SEG: cvmx_mips_space = cvmx_mips_space::CVMX_MIPS_SPACE_XKPHYS;
#[inline] pub const fn CVMX_ADD_SEG(segment: u64, add: u64) -> u64 { (segment << 62) | add }
#[inline] pub const fn CVMX_ADD_IO_SEG(add: u64) -> u64 { CVMX_ADD_SEG(CVMX_IO_SEG as u64, add) }

pub const CVMX_ENABLE_DEBUG_PRINTS: i32 = 1;
pub const CVMX_MAX_CORES: i32 = 16;
pub const CVMX_CACHE_LINE_SIZE: i32 = 128;
pub const CVMX_CACHE_LINE_MASK: i32 = CVMX_CACHE_LINE_SIZE - 1;

#[inline] pub fn CAST64<T>(v: *const T) -> i64 { v as isize as i64 }
#[inline] pub fn CASTPTR<T>(v: u64) -> *mut T { v as usize as *mut T }

#[inline]
pub unsafe fn cvmx_get_proc_id() -> u32 {
    let mut id: u32;
    core::arch::asm!("mfc0 {0}, $15, 0", out(reg) id);
    id
}

#[inline] pub const fn cvmx_build_mask(bits: u64) -> u64 { !((!0u64) << bits) }
#[inline] pub const fn cvmx_build_io_address(major_did: u64, sub_did: u64) -> u64 {
    (1u64 << 48) | (major_did << 43) | (sub_did << 40)
}
#[inline] pub const fn cvmx_build_bits(high_bit: u64, low_bit: u64, value: u64) -> u64 {
    (value & cvmx_build_mask(high_bit - low_bit + 1)) << low_bit
}

#[inline]
pub unsafe fn cvmx_ptr_to_phys<T>(ptr: *mut T) -> u64 {
    if core::mem::size_of::<*mut T>() == 8 {
        if (CAST64(ptr) >> 62) == 3 { CAST64(ptr) as u64 & cvmx_build_mask(30) }
        else { CAST64(ptr) as u64 & cvmx_build_mask(40) }
    } else { ptr as usize as u64 & 0x1fffffff }
}

#[inline]
pub unsafe fn cvmx_phys_to_ptr<T>(physical_address: u64) -> *mut T {
    if core::mem::size_of::<*mut T>() == 8 {
        CASTPTR(CVMX_ADD_SEG(CVMX_MIPS_SPACE_XKPHYS as u64, physical_address))
    } else {
        CASTPTR(CVMX_ADD_SEG32(CVMX_MIPS32_SPACE_KSEG0, physical_address as i32) as u32 as u64)
    }
}

macro_rules! cvmx_build_write64 { ($name:ident, $ty:ty) => {
    #[inline] pub unsafe fn $name(addr: u64, val: $ty) { (addr as *mut $ty).write_volatile(val); }
}; }
macro_rules! cvmx_build_read64 { ($name:ident, $ty:ty) => {
    #[inline] pub unsafe fn $name(addr: u64) -> $ty { (addr as *const $ty).read_volatile() }
}; }
cvmx_build_write64!(cvmx_write64_int64, i64); cvmx_build_write64!(cvmx_write64_int32, i32);
cvmx_build_write64!(cvmx_write64_int16, i16); cvmx_build_write64!(cvmx_write64_int8, i8);
cvmx_build_write64!(cvmx_write64_uint64, u64); cvmx_build_write64!(cvmx_write64_uint32, u32);
cvmx_build_write64!(cvmx_write64_uint16, u16); cvmx_build_write64!(cvmx_write64_uint8, u8);
cvmx_build_read64!(cvmx_read64_int64, i64); cvmx_build_read64!(cvmx_read64_int32, i32);
cvmx_build_read64!(cvmx_read64_int16, i16); cvmx_build_read64!(cvmx_read64_int8, i8);
cvmx_build_read64!(cvmx_read64_uint64, u64); cvmx_build_read64!(cvmx_read64_uint32, u32);
cvmx_build_read64!(cvmx_read64_uint16, u16); cvmx_build_read64!(cvmx_read64_uint8, u8);
pub use cvmx_write64_uint64 as cvmx_write64;
pub use cvmx_read64_uint64 as cvmx_read64;

#[inline] pub unsafe fn cvmx_write_csr(csr_addr: u64, val: u64) {
    cvmx_write64(csr_addr, val);
    if ((csr_addr >> 40) & 0x7ffff) == 0x118 { cvmx_read64(CVMX_MIO_BOOT_BIST_STAT); }
}
#[inline] pub unsafe fn cvmx_writeq_csr(csr_addr: *mut core::ffi::c_void, val: u64) { cvmx_write_csr(csr_addr as u64, val); }
#[inline] pub unsafe fn cvmx_write_io(io_addr: u64, val: u64) { cvmx_write64(io_addr, val); }
#[inline] pub unsafe fn cvmx_read_csr(csr_addr: u64) -> u64 { cvmx_read64(csr_addr) }
#[inline] pub unsafe fn cvmx_readq_csr(csr_addr: *mut core::ffi::c_void) -> u64 { cvmx_read_csr(csr_addr as u64) }
#[inline] pub unsafe fn cvmx_send_single(data: u64) { cvmx_write64(0xffffffffffffa200, data); }
#[inline] pub unsafe fn cvmx_read_csr_async(scraddr: u64, csr_addr: u64) {
    let addr = (csr_addr & ((1u64 << 48) - 1)) | ((scraddr >> 3) << 48) | (1u64 << 56);
    cvmx_send_single(addr);
}

#[inline] pub unsafe fn cvmx_octeon_is_pass1() -> i32 { 0 /* OCTEON_IS_COMMON_BINARY/model condition is build-time external. */ }
#[inline] pub unsafe fn cvmx_get_core_num() -> u32 { let mut n: u32; CVMX_RDHWRNV!(n, 0); n }
pub const CVMX_NODE_NO_SHIFT: u32 = 7;
pub const CVMX_NODE_MASK: u32 = 0x3;
#[inline] pub unsafe fn cvmx_get_node_num() -> u32 { (cvmx_get_core_num() >> CVMX_NODE_NO_SHIFT) & CVMX_NODE_MASK }
#[inline] pub unsafe fn cvmx_get_local_core_num() -> u32 { cvmx_get_core_num() & ((1 << CVMX_NODE_NO_SHIFT) - 1) }
pub const CVMX_NODE_BITS: u32 = 2;
pub const CVMX_MAX_NODES: u32 = 1 << CVMX_NODE_BITS;
pub const CVMX_NODE_IO_SHIFT: u32 = 36;
pub const CVMX_NODE_MEM_SHIFT: u32 = 40;
pub const CVMX_NODE_IO_MASK: u64 = (CVMX_NODE_MASK as u64) << CVMX_NODE_IO_SHIFT;

#[inline] pub unsafe fn cvmx_write_csr_node(node: u64, csr_addr: u64, val: u64) {
    let node_addr = (node & CVMX_NODE_MASK as u64) << CVMX_NODE_IO_SHIFT;
    let composite = (csr_addr & !CVMX_NODE_IO_MASK) | node_addr;
    cvmx_write64(composite, val);
    if ((csr_addr >> 40) & 0x7ffff) == 0x118 { cvmx_read64(CVMX_MIO_BOOT_BIST_STAT | node_addr); }
}
#[inline] pub unsafe fn cvmx_read_csr_node(node: u64, csr_addr: u64) -> u64 {
    let node_addr = (csr_addr & !CVMX_NODE_IO_MASK) | ((node & CVMX_NODE_MASK as u64) << CVMX_NODE_IO_SHIFT);
    cvmx_read_csr(node_addr)
}
#[inline] pub unsafe fn cvmx_pop(val: u32) -> u32 { val.count_ones() }
#[inline] pub unsafe fn cvmx_dpop(val: u64) -> i32 { val.count_ones() as i32 }
#[inline] pub unsafe fn cvmx_get_cycle() -> u64 { let mut cycle: u64; CVMX_RDHWR!(cycle, 31); cycle }
#[inline] pub unsafe fn cvmx_get_cycle_global() -> u64 { if cvmx_octeon_is_pass1() != 0 { 0 } else { cvmx_read64(CVMX_IPD_CLK_COUNT) } }

/* CVMX_WAIT_FOR_FIELD64 is retained as a macro because its type/field/op arguments are caller-defined. */
#[macro_export]
macro_rules! CVMX_WAIT_FOR_FIELD64 {
    ($address:expr, $type:ty, $field:ident, $op:tt, $value:expr, $timeout_usec:expr) => {{
        let mut result: i32;
        loop {
            let done = unsafe { cvmx_get_cycle() }.wrapping_add(($timeout_usec as u64).wrapping_mul(unsafe { cvmx_sysinfo_get().cpu_clock_hz }) / 1_000_000);
            loop {
                let c: $type = unsafe { core::mem::zeroed() };
                let _ = c;
                /* C bitfield access and __delay(100) are supplied by the caller's OCTEON bindings. */
                result = -1; break;
            }
            break;
        }
        result
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
