/* SPDX-License-Identifier: GPL-2.0 */

// struct device is supplied by an external dependency.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

pub const DSA_MAX_PORTS: usize = 12;

#[repr(C)]
pub struct dsa_chip_data {
    /*
     * Reference to network devices
     */
    pub netdev: [*mut device; DSA_MAX_PORTS],

    /* set to size of eeprom if supported by the switch */
    pub eeprom_len: i32,

    /*
     * The names of the switch's ports.  Use "cpu" to
     * designate the switch port that the cpu is connected to,
     * "dsa" to indicate that this port is a DSA link to
     * another switch, NULL to indicate the port is unused,
     * or any other string to indicate this is a physical port.
     */
    pub port_names: [*mut i8; DSA_MAX_PORTS],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
