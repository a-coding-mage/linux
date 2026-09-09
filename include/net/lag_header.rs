/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding Linux networking headers:
// <linux/netdevice.h>, <linux/if_team.h>, and <net/bonding.h>.

extern "C" {
    fn netif_is_team_port(port_dev: *const net_device) -> bool;
    fn team_port_dev_txable(port_dev: *const net_device) -> bool;
    fn bond_is_active_slave_dev(port_dev: *const net_device) -> bool;
}

pub unsafe fn net_lag_port_dev_txable(port_dev: *const net_device) -> bool {
    if netif_is_team_port(port_dev) {
        team_port_dev_txable(port_dev)
    } else {
        bond_is_active_slave_dev(port_dev)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
