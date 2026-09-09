/* SPDX-License-Identifier: GPL-2.0 */
// Source dependency: <linux/phy.h>

// Opaque declarations supplied by other translation units.
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mii_bus {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mdiobb_ops {
    pub owner: *mut module,

    /* Set the Management Data Clock high if level is one,
     * low if level is zero.
     */
    pub set_mdc: Option<unsafe extern "C" fn(ctrl: *mut mdiobb_ctrl, level: i32)>,

    /* Configure the Management Data I/O pin as an input if
     * "output" is zero, or an output if "output" is one.
     */
    pub set_mdio_dir: Option<unsafe extern "C" fn(ctrl: *mut mdiobb_ctrl, output: i32)>,

    /* Set the Management Data I/O pin high if value is one,
     * low if "value" is zero.  This may only be called
     * when the MDIO pin is configured as an output.
     */
    pub set_mdio_data: Option<unsafe extern "C" fn(ctrl: *mut mdiobb_ctrl, value: i32)>,

    /* Retrieve the state Management Data I/O pin. */
    pub get_mdio_data: Option<unsafe extern "C" fn(ctrl: *mut mdiobb_ctrl) -> i32>,
}

#[repr(C)]
pub struct mdiobb_ctrl {
    pub ops: *const mdiobb_ops,
    pub override_op_c22: u32,
    pub op_c22_read: u8,
    pub op_c22_write: u8,
}

extern "C" {
    pub fn mdiobb_read_c22(bus: *mut mii_bus, phy: i32, reg: i32) -> i32;
    pub fn mdiobb_write_c22(bus: *mut mii_bus, phy: i32, reg: i32, val: u16) -> i32;
    pub fn mdiobb_read_c45(bus: *mut mii_bus, devad: i32, phy: i32, reg: i32) -> i32;
    pub fn mdiobb_write_c45(
        bus: *mut mii_bus,
        devad: i32,
        phy: i32,
        reg: i32,
        val: u16,
    ) -> i32;
}

/* The returned bus is not yet registered with the phy layer. */
extern "C" {
    pub fn alloc_mdio_bitbang(ctrl: *mut mdiobb_ctrl) -> *mut mii_bus;
}

/* The bus must already have been unregistered. */
extern "C" {
    pub fn free_mdio_bitbang(bus: *mut mii_bus);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
