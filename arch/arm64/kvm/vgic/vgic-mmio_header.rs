/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2015, 2016 ARM Ltd.
 */

#[repr(C)]
pub struct vgic_register_region {
    pub reg_offset: ::core::ffi::c_uint,
    pub len: ::core::ffi::c_uint,
    pub bits_per_irq: ::core::ffi::c_uint,
    pub access_flags: ::core::ffi::c_uint,
    pub read: vgic_register_region_read,
    pub write: vgic_register_region_write,
    pub uaccess_read: Option<unsafe extern "C" fn(*mut kvm_vcpu, gpa_t, ::core::ffi::c_uint) -> ::core::ffi::c_ulong>,
    pub uaccess_write: vgic_register_region_uaccess_write,
}

#[repr(C)]
pub union vgic_register_region_read {
    pub read: Option<unsafe extern "C" fn(*mut kvm_vcpu, gpa_t, ::core::ffi::c_uint) -> ::core::ffi::c_ulong>,
    pub its_read: Option<unsafe extern "C" fn(*mut kvm, *mut vgic_its, gpa_t, ::core::ffi::c_uint) -> ::core::ffi::c_ulong>,
}

#[repr(C)]
pub union vgic_register_region_write {
    pub write: Option<unsafe extern "C" fn(*mut kvm_vcpu, gpa_t, ::core::ffi::c_uint, ::core::ffi::c_ulong)>,
    pub its_write: Option<unsafe extern "C" fn(*mut kvm, *mut vgic_its, gpa_t, ::core::ffi::c_uint, ::core::ffi::c_ulong)>,
}

#[repr(C)]
pub union vgic_register_region_uaccess_write {
    pub uaccess_write: Option<unsafe extern "C" fn(*mut kvm_vcpu, gpa_t, ::core::ffi::c_uint, ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    pub uaccess_its_write: Option<unsafe extern "C" fn(*mut kvm, *mut vgic_its, gpa_t, ::core::ffi::c_uint, ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
}

extern "C" {
    pub static kvm_io_gic_ops: kvm_io_device_ops;
}

pub const VGIC_ACCESS_8bit: ::core::ffi::c_uint = 1;
pub const VGIC_ACCESS_32bit: ::core::ffi::c_uint = 2;
pub const VGIC_ACCESS_64bit: ::core::ffi::c_uint = 4;

/* Generate a mask that covers the number of bytes required to address
 * up to 1024 interrupts, each represented by <bits> bits. This assumes
 * that <bits> is a power of two.
 */
#[macro_export]
macro_rules! VGIC_ADDR_IRQ_MASK { ($bits:expr) => { (($bits) * 1024 / 8) - 1 }; }

/* (addr & mask) gives us the _byte_ offset for the INT ID.
 * We multiply this by 8 the get the _bit_ offset, then divide this by
 * the number of bits to learn the actual INT ID.
 * But instead of a division (which requires a "long long div" implementation),
 * we shift by the binary logarithm of <bits>.
 * This assumes that <bits> is a power of two.
 */
#[macro_export]
macro_rules! VGIC_ADDR_TO_INTID { ($addr:expr, $bits:expr) => { (((($addr) & VGIC_ADDR_IRQ_MASK!($bits)) * 8) >> ilog2($bits)) }; }

#[macro_export]
macro_rules! REGISTER_DESC_WITH_BITS_PER_IRQ {
    ($off:expr, $rd:expr, $wr:expr, $ur:expr, $uw:expr, $bpi:expr, $acc:expr) => {
        vgic_register_region { reg_offset: $off, bits_per_irq: $bpi, len: $bpi * 1024 / 8, access_flags: $acc,
            read: $rd, write: $wr, uaccess_read: $ur, uaccess_write: $uw }
    };
}

#[macro_export]
macro_rules! REGISTER_DESC_WITH_LENGTH {
    ($off:expr, $rd:expr, $wr:expr, $length:expr, $acc:expr) => {
        vgic_register_region { reg_offset: $off, bits_per_irq: 0, len: $length, access_flags: $acc,
            read: $rd, write: $wr, uaccess_read: None, uaccess_write: vgic_register_region_uaccess_write { uaccess_write: None } }
    };
}

#[macro_export]
macro_rules! REGISTER_DESC_WITH_LENGTH_UACCESS {
    ($off:expr, $rd:expr, $wr:expr, $urd:expr, $uwr:expr, $length:expr, $acc:expr) => {
        vgic_register_region { reg_offset: $off, bits_per_irq: 0, len: $length, access_flags: $acc,
            read: $rd, write: $wr, uaccess_read: $urd, uaccess_write: $uwr }
    };
}

extern "C" {
    pub fn vgic_data_mmio_bus_to_host(val: *const ::core::ffi::c_void, len: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn vgic_data_host_to_mmio_bus(buf: *mut ::core::ffi::c_void, len: ::core::ffi::c_uint, data: ::core::ffi::c_ulong);
    pub fn extract_bytes(data: u64, offset: ::core::ffi::c_uint, num: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn update_64bit_reg(reg: u64, offset: ::core::ffi::c_uint, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong) -> u64;
    pub fn vgic_mmio_read_raz(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn vgic_mmio_read_rao(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn vgic_mmio_write_wi(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong);
    pub fn vgic_mmio_uaccess_write_wi(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn vgic_mmio_read_group(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn vgic_mmio_write_group(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong);
    pub fn vgic_mmio_read_enable(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn vgic_mmio_write_senable(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong);
    pub fn vgic_mmio_write_cenable(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong);
    pub fn vgic_uaccess_write_senable(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn vgic_uaccess_write_cenable(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn vgic_mmio_read_pending(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn vgic_uaccess_read_pending(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn vgic_mmio_write_spending(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong);
    pub fn vgic_mmio_write_cpending(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong);
    pub fn vgic_uaccess_write_spending(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn vgic_uaccess_write_cpending(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn vgic_mmio_read_active(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn vgic_uaccess_read_active(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn vgic_mmio_write_cactive(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong);
    pub fn vgic_mmio_write_sactive(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong);
    pub fn vgic_mmio_uaccess_write_cactive(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn vgic_mmio_uaccess_write_sactive(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong) -> ::core::ffi::c_int;
    pub fn vgic_mmio_read_priority(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn vgic_mmio_write_priority(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong);
    pub fn vgic_mmio_read_config(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint) -> ::core::ffi::c_ulong;
    pub fn vgic_mmio_write_config(vcpu: *mut kvm_vcpu, addr: gpa_t, len: ::core::ffi::c_uint, val: ::core::ffi::c_ulong);
    pub fn vgic_uaccess(vcpu: *mut kvm_vcpu, dev: *mut vgic_io_device, is_write: bool, offset: ::core::ffi::c_int, val: *mut u32) -> ::core::ffi::c_int;
    pub fn vgic_read_irq_line_level_info(vcpu: *mut kvm_vcpu, intid: u32) -> u32;
    pub fn vgic_write_irq_line_level_info(vcpu: *mut kvm_vcpu, intid: u32, val: u32);
    pub fn vgic_v2_init_dist_iodev(dev: *mut vgic_io_device) -> ::core::ffi::c_uint;
    pub fn vgic_v2_init_cpuif_iodev(dev: *mut vgic_io_device) -> ::core::ffi::c_uint;
    pub fn vgic_v3_init_dist_iodev(dev: *mut vgic_io_device) -> ::core::ffi::c_uint;
    pub fn vgic_sanitise_outer_cacheability(reg: u64) -> u64;
    pub fn vgic_sanitise_inner_cacheability(reg: u64) -> u64;
    pub fn vgic_sanitise_shareability(reg: u64) -> u64;
    pub fn vgic_sanitise_field(reg: u64, field_mask: u64, field_shift: ::core::ffi::c_int, sanitise_fn: Option<unsafe extern "C" fn(u64) -> u64>) -> u64;
}

/* Find the proper register handler entry given a certain address offset */
extern "C" {
    pub fn vgic_find_mmio_region(regions: *const vgic_register_region, nr_regions: ::core::ffi::c_int, offset: ::core::ffi::c_uint) -> *const vgic_register_region;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
