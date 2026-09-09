/* SPDX-License-Identifier: GPL-2.0 */
/*
 * MCTP per-net structures
 */

// External kernel types and helpers are supplied by other translated files.

pub const MCTP_BINDS_BITS: usize = 7;

#[repr(C)]
pub struct netns_mctp {
	/* Only updated under RTNL, entries freed via RCU */
	pub routes: list_head,

	/* Bound sockets: hash table of sockets, keyed by
	 * (type, src_eid, dest_eid).
	 * Specific src_eid/dest_eid entries also have an entry for
	 * MCTP_ADDR_ANY. This list is updated from non-atomic contexts
	 * (under bind_lock), and read (under rcu) in packet rx.
	 */
	pub bind_lock: mutex,
	pub binds: [hlist_head; 1usize << MCTP_BINDS_BITS],

	/* tag allocations. This list is read and updated from atomic contexts,
	 * but elements are free()ed after a RCU grace-period
	 */
	pub keys_lock: spinlock_t,
	pub keys: hlist_head,

	/* MCTP network */
	pub default_net: u32,

	/* neighbour table */
	pub neigh_lock: mutex,
	pub neighbours: list_head,
}

extern "C" {
	fn hash_32(value: u32, bits: u32) -> u32;
}

#[inline]
pub unsafe fn mctp_bind_hash(type_: u8, local_addr: u8, peer_addr: u8) -> u32 {
	unsafe {
		hash_32(
			type_ as u32
				| (local_addr as u32) << 8
				| (peer_addr as u32) << 16,
			MCTP_BINDS_BITS as u32,
		)
	}
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
