/* SPDX-License-Identifier: GPL-2.0 */

/*
 * DOMAIN(name, num, index_kind, is_physical)
 *
 * @name:     An all caps token, suitable for use in generating an enum
 *            member and appending to an event name in sysfs.
 *
 * @num:      The number corresponding to the domain as given in
 *            documentation. We assume the catalog domain and the hcall
 *            domain have the same numbering (so far they do), but this
 *            may need to be changed in the future.
 *
 * @index_kind: A stringifiable token describing the meaning of the index
 *              within the given domain. Must fit the parsing rules of the
 *              perf sysfs api.
 *
 * @is_physical: True if the domain is physical, false otherwise (if virtual).
 *
 * Note: The terms PHYS_CHIP, PHYS_CORE, VCPU correspond to physical chip,
 *       physical core and virtual processor in 24x7 Counters specifications.
 */

// Each entry preserves the C DOMAIN(name, num, index_kind, is_physical) data.
pub const PHYS_CHIP: (u8, &str, bool) = (0x01, "chip", true);
pub const PHYS_CORE: (u8, &str, bool) = (0x02, "core", true);
pub const VCPU_HOME_CORE: (u8, &str, bool) = (0x03, "vcpu", false);
pub const VCPU_HOME_CHIP: (u8, &str, bool) = (0x04, "vcpu", false);
pub const VCPU_HOME_NODE: (u8, &str, bool) = (0x05, "vcpu", false);
pub const VCPU_REMOTE_NODE: (u8, &str, bool) = (0x06, "vcpu", false);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
