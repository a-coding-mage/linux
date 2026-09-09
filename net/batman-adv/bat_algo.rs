// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) B.A.T.M.A.N. contributors:
 *
 * Marek Lindner, Simon Wunderlich
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, corresponding to the C includes.

pub static mut batadv_routing_algo: [u8; 20] = *b"BATMAN_IV\0\0\0\0\0\0\0\0\0\0\0";
static mut batadv_algo_list: hlist_head = hlist_head::default();

/**
 * batadv_algo_init() - Initialize batman-adv algorithm management data
 *  structures
 */
pub unsafe fn batadv_algo_init() {
	INIT_HLIST_HEAD(&raw mut batadv_algo_list);
}

/**
 * batadv_algo_get() - Search for algorithm with specific name
 * @name: algorithm name to find
 *
 * Return: Pointer to batadv_algo_ops on success, NULL otherwise
 */
pub unsafe fn batadv_algo_get(name: *const c_char) -> *mut batadv_algo_ops {
	let mut bat_algo_ops: *mut batadv_algo_ops = core::ptr::null_mut();
	let mut bat_algo_ops_tmp: *mut batadv_algo_ops;

	// hlist_for_each_entry(bat_algo_ops_tmp, &batadv_algo_list, list)
	for bat_algo_ops_tmp in hlist_for_each_entry(&raw mut batadv_algo_list) {
		if strcmp((*bat_algo_ops_tmp).name, name) != 0 {
			continue;
		}

		bat_algo_ops = bat_algo_ops_tmp;
		break;
	}

	bat_algo_ops
}

/**
 * batadv_algo_register() - Register callbacks for a mesh algorithm
 * @bat_algo_ops: mesh algorithm callbacks to add
 *
 * Return: 0 on success or negative error number in case of failure
 */
pub unsafe fn batadv_algo_register(bat_algo_ops: *mut batadv_algo_ops) -> c_int {
	let bat_algo_ops_tmp = batadv_algo_get((*bat_algo_ops).name);
	if !bat_algo_ops_tmp.is_null() {
		pr_info!("Trying to register already registered routing algorithm: %s\n",
			(*bat_algo_ops).name);
		return -EEXIST;
	}

	/* all algorithms must implement all ops (for now) */
	if (*bat_algo_ops).iface.enable.is_none()
		|| (*bat_algo_ops).iface.disable.is_none()
		|| (*bat_algo_ops).iface.update_mac.is_none()
		|| (*bat_algo_ops).iface.primary_set.is_none()
		|| (*bat_algo_ops).neigh.cmp.is_none()
		|| (*bat_algo_ops).neigh.is_similar_or_better.is_none()
	{
		pr_info!("Routing algo '%s' does not implement required ops\n",
			(*bat_algo_ops).name);
		return -EINVAL;
	}

	INIT_HLIST_NODE(&raw mut (*bat_algo_ops).list);
	hlist_add_head(&raw mut (*bat_algo_ops).list, &raw mut batadv_algo_list);

	0
}

/**
 * batadv_algo_select() - Select algorithm of mesh interface
 * @bat_priv: the bat priv with all the mesh interface information
 * @name: name of the algorithm to select
 *
 * The algorithm callbacks for the mesh interface will be set when the algorithm
 * with the correct name was found. Any previous selected algorithm will not be
 * deinitialized and the new selected algorithm will also not be initialized.
 * It is therefore not allowed to call batadv_algo_select outside the creation
 * function of the mesh interface.
 *
 * Return: 0 on success or negative error number in case of failure
 */
pub unsafe fn batadv_algo_select(bat_priv: *mut batadv_priv, name: *const c_char) -> c_int {
	let bat_algo_ops = batadv_algo_get(name);
	if bat_algo_ops.is_null() {
		return -EINVAL;
	}

	(*bat_priv).algo_ops = bat_algo_ops;
	0
}

/**
 * batadv_param_set_ra() - Validate and store routing_algo module parameter
 * @val: new value for the routing_algo module parameter
 * @kp: kernel parameter description used to store the value
 *
 * Check that the requested algorithm is known to batman-adv and then store
 * the name as the new default routing algorithm.
 *
 * Return: 0 on success or negative error number in case of failure
 */
unsafe fn batadv_param_set_ra(val: *const c_char, kp: *const kernel_param) -> c_int {
	let mut algo_name = val as *mut c_char;
	let name_len = strlen(algo_name);
	if name_len > 0 && *algo_name.add(name_len - 1) == b'\n' as c_char {
		*algo_name.add(name_len - 1) = b'\0' as c_char;
	}

	let bat_algo_ops = batadv_algo_get(algo_name);
	if bat_algo_ops.is_null() {
		pr_err!("Routing algorithm '%s' is not supported\n", algo_name);
		return -EINVAL;
	}

	param_set_copystring(algo_name, kp)
}

static batadv_param_ops_ra: kernel_param_ops = kernel_param_ops {
	set: Some(batadv_param_set_ra),
	get: Some(param_get_string),
};

static mut batadv_param_string_ra: kparam_string = kparam_string {
	maxlen: core::mem::size_of::<[u8; 20]>(),
	string: unsafe { &raw mut batadv_routing_algo },
};

// module_param_cb(routing_algo, &batadv_param_ops_ra, &batadv_param_string_ra, 0644);

/**
 * batadv_algo_dump_entry() - fill in information about one supported routing
 *  algorithm
 */
unsafe fn batadv_algo_dump_entry(msg: *mut sk_buff, portid: u32, seq: u32,
					 bat_algo_ops: *mut batadv_algo_ops) -> c_int {
	let hdr = genlmsg_put(msg, portid, seq, &raw mut batadv_netlink_family,
					 NLM_F_MULTI, BATADV_CMD_GET_ROUTING_ALGOS);
	if hdr.is_null() {
		return -EMSGSIZE;
	}

	if nla_put_string(msg, BATADV_ATTR_ALGO_NAME, (*bat_algo_ops).name) != 0 {
		genlmsg_cancel(msg, hdr);
		return -EMSGSIZE;
	}

	genlmsg_end(msg, hdr);
	0
}

/**
 * batadv_algo_dump() - fill in information about supported routing algorithms
 */
pub unsafe fn batadv_algo_dump(msg: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
	let portid = NETLINK_CB((*cb).skb).portid;
	let mut skip = (*cb).args[0];
	let mut i = 0;

	for bat_algo_ops in hlist_for_each_entry(&raw mut batadv_algo_list) {
		if i += 1; i - 1 < skip {
			continue;
		}

		if batadv_algo_dump_entry(msg, portid, (*(*cb).nlh).nlmsg_seq,
						  bat_algo_ops) != 0 {
			i -= 1;
			break;
		}
	}

	(*cb).args[0] = i;
	(*msg).len as c_int
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
