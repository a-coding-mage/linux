// SPDX-License-Identifier: GPL-2.0
// DEBUG is undefined in the original source.

// Dependencies supplied by the surrounding kernel translation unit.
#[allow(non_camel_case_types)]
pub type __be32 = u32;
pub type u32 = core::ffi::c_uint;
pub type c_ulong = core::ffi::c_ulong;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

extern "C" {
    fn of_read_number(cell: *const __be32, size: u32) -> c_ulong;
    fn of_get_property(
        dn: *const device_node,
        name: *const core::ffi::c_char,
        lenp: *mut u32,
    ) -> *const __be32;
    fn of_n_addr_cells(dn: *const device_node) -> u32;
    fn of_n_size_cells(dn: *const device_node) -> u32;
}

pub unsafe fn of_parse_dma_window(
    dn: *mut device_node,
    mut dma_window: *const __be32,
    busno: *mut c_ulong,
    phys: *mut c_ulong,
    size: *mut c_ulong,
) {
    let cells: u32;
    let mut prop: *const __be32;

    /* busno is always one cell */
    *busno = of_read_number(dma_window, 1);
    dma_window = dma_window.add(1);

    prop = of_get_property(
        dn,
        b"ibm,#dma-address-cells\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null_mut(),
    );
    if prop.is_null() {
        prop = of_get_property(
            dn,
            b"#address-cells\0".as_ptr() as *const core::ffi::c_char,
            core::ptr::null_mut(),
        );
    }

    cells = if !prop.is_null() {
        of_read_number(prop, 1) as u32
    } else {
        of_n_addr_cells(dn)
    };
    *phys = of_read_number(dma_window, cells);

    dma_window = dma_window.add(cells as usize);

    prop = of_get_property(
        dn,
        b"ibm,#dma-size-cells\0".as_ptr() as *const core::ffi::c_char,
        core::ptr::null_mut(),
    );
    let cells = if !prop.is_null() {
        of_read_number(prop, 1) as u32
    } else {
        of_n_size_cells(dn)
    };
    *size = of_read_number(dma_window, cells);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
