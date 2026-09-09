/* Translated from asm/octeon/octeon.h. */

pub const OCTEON_ARGV_MAX_ARGS: usize = 64;
pub const OCTEON_SERIAL_LEN: usize = 20;
pub const BOOT_FLAG_INIT_CORE: u32 = 1 << 0;
pub const OCTEON_BL_FLAG_DEBUG: u32 = 1 << 1;
pub const OCTEON_BL_FLAG_NO_MAGIC: u32 = 1 << 2;
pub const OCTEON_BL_FLAG_CONSOLE_UART1: u32 = 1 << 3;
pub const OCTEON_BL_FLAG_CONSOLE_PCI: u32 = 1 << 4;
pub const OCTEON_BL_FLAG_BREAK: u32 = 1 << 5;

extern "C" {
    pub fn octeon_bootmem_alloc_range_phys(size: u64, alignment: u64, min_addr: u64, max_addr: u64, do_locking: i32) -> u64;
    pub fn octeon_bootmem_alloc(size: u64, alignment: u64, do_locking: i32) -> *mut core::ffi::c_void;
    pub fn octeon_bootmem_alloc_range(size: u64, alignment: u64, min_addr: u64, max_addr: u64, do_locking: i32) -> *mut core::ffi::c_void;
    pub fn octeon_bootmem_alloc_named(size: u64, alignment: u64, name: *mut i8) -> *mut core::ffi::c_void;
    pub fn octeon_bootmem_alloc_named_range(size: u64, min_addr: u64, max_addr: u64, align: u64, name: *mut i8) -> *mut core::ffi::c_void;
    pub fn octeon_bootmem_alloc_named_address(size: u64, address: u64, name: *mut i8) -> *mut core::ffi::c_void;
    pub fn octeon_bootmem_free_named(name: *mut i8) -> i32;
    pub fn octeon_bootmem_lock();
    pub fn octeon_bootmem_unlock();
    pub fn octeon_is_simulation() -> i32;
    pub fn octeon_is_pci_host() -> i32;
    pub fn octeon_usb_is_ref_clk() -> i32;
    pub fn octeon_get_clock_rate() -> u64;
    pub fn octeon_get_io_clock_rate() -> u64;
    pub fn octeon_board_type_string() -> *const i8;
    pub fn octeon_get_pci_interrupts() -> *const i8;
    pub fn octeon_get_southbridge_interrupt() -> i32;
    pub fn octeon_get_boot_coremask() -> i32;
    pub fn octeon_get_boot_num_arguments() -> i32;
    pub fn octeon_get_boot_argument(arg: i32) -> *const i8;
    pub fn octeon_user_io_init();
    pub fn octeon_init_cvmcount();
    pub fn octeon_setup_delays();
    pub fn octeon_io_clk_delay(_: usize);
}

#[repr(C)]
pub struct octeon_boot_descriptor {
    pub desc_size: u32,
    pub desc_version: u32,
    pub stack_top: u64,
    pub heap_base: u64,
    pub heap_end: u64,
    pub entry_point: u64,
    pub desc_vaddr: u64,
    pub stack_size: u32,
    pub exception_base_addr: u32,
    pub argc: u32,
    pub heap_size: u32,
    pub argv: [u32; OCTEON_ARGV_MAX_ARGS],
    pub core_mask: u32,
    pub flags: u32,
    pub phy_mem_desc_addr: u32,
    pub dram_size: u32,
    pub eclock_hz: u32,
    pub debugger_flags_base_addr: u32,
    pub spi_clock_hz: u32,
    pub dclock_hz: u32,
    pub chip_rev_minor: u8,
    pub chip_rev_major: u8,
    pub chip_type: u16,
    pub board_rev_minor: u8,
    pub board_rev_major: u8,
    pub board_type: u16,
    pub unused1: [u64; 4],
    pub cvmx_desc_vaddr: u64,
}

#[repr(C)]
pub union octeon_cvmemctl {
    pub u64_: u64,
    /* C bitfields are represented by their containing register; use masks at call sites. */
    pub s: octeon_cvmemctl_bits,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct octeon_cvmemctl_bits { pub bits: u64 }

extern "C" {
    pub fn octeon_check_cpu_bist();
    pub fn octeon_prune_device_tree() -> i32;
    pub static __dtb_octeon_3xxx_begin: i8;
    pub static __dtb_octeon_68xx_begin: i8;
    pub fn octeon_setup_smp();
    pub fn octeon_ciu3_mbox_send(cpu: i32, mbox: u32);
    pub fn octeon_irq_ciu3_xlat(d: *mut core::ffi::c_void, node: *mut core::ffi::c_void, intspec: *const u32, intsize: u32, out_hwirq: *mut usize, out_type: *mut u32) -> i32;
    pub fn octeon_irq_ciu3_enable(data: *mut core::ffi::c_void);
    pub fn octeon_irq_ciu3_disable(data: *mut core::ffi::c_void);
    pub fn octeon_irq_ciu3_ack(data: *mut core::ffi::c_void);
    pub fn octeon_irq_ciu3_mask(data: *mut core::ffi::c_void);
    pub fn octeon_irq_ciu3_mask_ack(data: *mut core::ffi::c_void);
    pub fn octeon_irq_ciu3_mapx(d: *mut core::ffi::c_void, virq: u32, hw: usize, chip: *mut core::ffi::c_void) -> i32;
    pub fn octeon_mult_save(); pub fn octeon_mult_restore(); pub fn octeon_mult_save_end(); pub fn octeon_mult_restore_end();
    pub fn octeon_mult_save3(); pub fn octeon_mult_save3_end(); pub fn octeon_mult_save2(); pub fn octeon_mult_save2_end();
    pub fn octeon_mult_restore3(); pub fn octeon_mult_restore3_end(); pub fn octeon_mult_restore2(); pub fn octeon_mult_restore2_end();
    pub static mut octeon_bootinfo: *mut core::ffi::c_void;
    pub static mut octeon_bootloader_entry_addr: u64;
    pub static mut octeon_irq_setup_secondary: Option<unsafe extern "C" fn()>;
    pub fn octeon_fixup_irqs();
    pub static mut octeon_bootbus_sem: core::ffi::c_void;
    pub fn octeon_irq_get_block_domain(node: i32, block: u8) -> *mut core::ffi::c_void;
}

extern "C" {
    fn cvmx_write64_uint32(address: u64, value: u32);
    fn cvmx_read64_uint32(address: u64) -> u32;
}

/// Write a 32-bit value to the Octeon NPI register space.
#[inline]
pub unsafe fn octeon_npi_write32(address: u64, val: u32) {
    cvmx_write64_uint32(address ^ 4, val);
    cvmx_read64_uint32(address ^ 4);
}

/// Read a 32-bit value from the Octeon NPI register space.
#[inline]
pub unsafe fn octeon_npi_read32(address: u64) -> u32 {
    cvmx_read64_uint32(address ^ 4)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
