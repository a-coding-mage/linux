/* SPDX-License-Identifier: GPL-2.0 */

/**
 * DOC: memblock kexec handover ABI
 *
 * Memblock can serialize its current memory reservations created with
 * reserve_mem command line option across kexec through KHO.
 * The post-KHO kernel can then consume these reservations and they are
 * guaranteed to have the same physical address.
 *
 * The state is serialized using Flattened Device Tree (FDT) format. Any
 * modification to the FDT structure, node properties, or the compatible
 * strings constitutes a breaking change. Such changes require incrementing
 * the version number in the relevant `_COMPATIBLE` string to prevent a new
 * kernel from misinterpreting data from an old kernel.
 *
 * Changes are allowed provided the compatibility version is incremented.
 * However, backward/forward compatibility is only guaranteed for kernels
 * supporting the same ABI version.
 *
 * FDT Structure Overview:
 *   The entire memblock state is encapsulated within a single KHO entry named
 *   "memblock".
 *   This entry contains an FDT with the following layout:
 *
 *   .. code-block:: none
 *
 *\t/ {
 *\t\tcompatible = "memblock-v1";
 *
 *\t\tn1 {
 *\t\t\tcompatible = "reserve-mem-v1";
 *\t\t\tstart = <0xc06b 0x4000000>;
 *\t\t\tsize = <0x04 0x00>;
 *\t\t};
 *\t};
 *
 * Main memblock node (/):
 *
 *   - compatible: "memblock-v1"
 *
 *     Identifies the overall memblock ABI version.
 *
 * reserved_mem node:
 *   These nodes describe all reserve_mem regions. The node name is the name
 *   defined by the user for a reserve_mem region.
 *
 *   - compatible: "reserve-mem-v1"
 *
 *     Identifies the ABI version of reserve_mem descriptions
 *
 *   - start: u64
 *
 *     Physical address of the reserved memory region.
 *
 *   - size: u64
 *
 *     size in bytes of the reserved memory region.
 */

/* Top level memblock FDT node name. */
pub const MEMBLOCK_KHO_FDT: &str = "memblock";

/* The compatible string for the memblock FDT root node. */
pub const MEMBLOCK_KHO_NODE_COMPATIBLE: &str = "memblock-v1";

/* The compatible string for the reserve_mem FDT nodes. */
pub const RESERVE_MEM_KHO_NODE_COMPATIBLE: &str = "reserve-mem-v1";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
