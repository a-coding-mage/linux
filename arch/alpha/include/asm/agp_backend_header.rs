/* SPDX-License-Identifier: GPL-2.0 */

// The C header uses kernel-provided `u32`, `dma_addr_t`, and `off_t` types.
// They are intentionally referenced here as external dependencies.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alpha_agp_mode_bits {
    pub raw: u32,
}

impl alpha_agp_mode_bits {
    pub const fn new(raw: u32) -> Self {
        Self { raw }
    }

    pub const fn rate(self) -> u32 { self.raw & 0x7 }
    pub const fn reserved0(self) -> u32 { (self.raw >> 3) & 0x1 }
    pub const fn fw(self) -> u32 { (self.raw >> 4) & 0x1 }
    pub const fn fourgb(self) -> u32 { (self.raw >> 5) & 0x1 }
    pub const fn reserved1(self) -> u32 { (self.raw >> 6) & 0x3 }
    pub const fn enable(self) -> u32 { (self.raw >> 8) & 0x1 }
    pub const fn sba(self) -> u32 { (self.raw >> 9) & 0x1 }
    pub const fn reserved2(self) -> u32 { (self.raw >> 10) & 0x3fff }
    pub const fn rq(self) -> u32 { (self.raw >> 24) & 0xff }
}

#[repr(C)]
pub union alpha_agp_mode {
    pub bits: alpha_agp_mode_bits,
    pub lw: u32,
}

#[repr(C)]
pub struct alpha_agp_info {
    pub hose: *mut pci_controller,
    pub aperture: alpha_agp_aperture,
    pub capability: alpha_agp_mode,
    pub mode: alpha_agp_mode,
    pub private: *mut core::ffi::c_void,
    pub ops: *mut alpha_agp_ops,
}

#[repr(C)]
pub struct alpha_agp_aperture {
    pub bus_base: dma_addr_t,
    pub size: usize,
    pub sysdata: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct alpha_agp_ops {
    pub setup: Option<unsafe extern "C" fn(info: *mut alpha_agp_info) -> i32>,
    pub cleanup: Option<unsafe extern "C" fn(info: *mut alpha_agp_info)>,
    pub configure: Option<unsafe extern "C" fn(info: *mut alpha_agp_info) -> i32>,
    pub bind: Option<unsafe extern "C" fn(info: *mut alpha_agp_info, pg_start: isize, mem: *mut agp_memory) -> i32>,
    pub unbind: Option<unsafe extern "C" fn(info: *mut alpha_agp_info, pg_start: isize, mem: *mut agp_memory) -> i32>,
    pub translate: Option<unsafe extern "C" fn(info: *mut alpha_agp_info, addr: dma_addr_t) -> usize>,
}

// Opaque types and kernel scalar types supplied by other headers.
pub enum pci_controller {}
pub enum agp_memory {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
