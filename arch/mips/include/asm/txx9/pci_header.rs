/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependency supplied by <linux/pci.h>.
#[repr(C)]
pub struct pci_controller {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub static mut txx9_primary_pcic: pci_controller;

    pub fn txx9_alloc_pci_controller(
        pcic: *mut pci_controller,
        mem_base: ::kernel::ffi::c_ulong,
        mem_size: ::kernel::ffi::c_ulong,
        io_base: ::kernel::ffi::c_ulong,
        io_size: ::kernel::ffi::c_ulong,
    ) -> *mut pci_controller;

    pub fn txx9_pci66_check(
        hose: *mut pci_controller,
        top_bus: ::kernel::ffi::c_int,
        current_bus: ::kernel::ffi::c_int,
    ) -> ::kernel::ffi::c_int;

    // The original declaration uses the kernel __initdata annotation.
    pub static mut txx9_pci_mem_high: ::kernel::ffi::c_int;

    pub static mut txx9_pci_option: ::kernel::ffi::c_int;

    pub static mut txx9_pci_err_action: txx9_pci_err_action;

    pub static mut txx9_board_pcibios_setup:
        Option<unsafe extern "C" fn(str: *mut ::kernel::ffi::c_char) -> *mut ::kernel::ffi::c_char>;

    pub fn txx9_pcibios_setup(
        str_: *mut ::kernel::ffi::c_char,
    ) -> *mut ::kernel::ffi::c_char;
}

pub const TXX9_PCI_OPT_PICMG: ::kernel::ffi::c_int = 0x0002;
pub const TXX9_PCI_OPT_CLK_33: ::kernel::ffi::c_int = 0x0008;
pub const TXX9_PCI_OPT_CLK_66: ::kernel::ffi::c_int = 0x0010;
pub const TXX9_PCI_OPT_CLK_MASK: ::kernel::ffi::c_int =
    TXX9_PCI_OPT_CLK_33 | TXX9_PCI_OPT_CLK_66;
pub const TXX9_PCI_OPT_CLK_AUTO: ::kernel::ffi::c_int = TXX9_PCI_OPT_CLK_MASK;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum txx9_pci_err_action {
    TXX9_PCI_ERR_REPORT,
    TXX9_PCI_ERR_IGNORE,
    TXX9_PCI_ERR_PANIC,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
