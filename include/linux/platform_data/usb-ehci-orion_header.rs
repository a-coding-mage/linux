/*
 * This file is licensed under the terms of the GNU General Public
 * License version 2.  This program is licensed "as is" without any
 * warranty of any kind, whether express or implied.
 */

// Dependency intent: the original header includes <linux/mbus.h>.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum orion_ehci_phy_ver {
    EHCI_PHY_ORION,
    EHCI_PHY_DD,
    EHCI_PHY_KW,
    EHCI_PHY_NA,
}

#[repr(C)]
pub struct orion_ehci_data {
    pub phy_version: orion_ehci_phy_ver,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
