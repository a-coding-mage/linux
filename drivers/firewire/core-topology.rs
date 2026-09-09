// SPDX-License-Identifier: GPL-2.0-or-later
/* Incremental bus scan, based on bus topology */

// Kernel and local C dependencies are supplied by the surrounding translation.

unsafe fn fw_node_create(sid: u32, port_count: i32, color: i32) -> *mut fw_node {
    let node = kzalloc_flex::<fw_node>(port_count);
    if node.is_null() { return core::ptr::null_mut(); }
    (*node).color = color;
    (*node).node_id = LOCAL_BUS | phy_packet_self_id_get_phy_id(sid);
    (*node).link_on = phy_packet_self_id_zero_get_link_active(sid);
    (*node).phy_speed = phy_packet_self_id_zero_get_scode(sid);
    (*node).initiated_reset = phy_packet_self_id_zero_get_initiated_reset(sid);
    (*node).port_count = port_count;
    kref_init(&mut (*node).kref);
    INIT_LIST_HEAD(&mut (*node).link);
    node
}

unsafe fn update_hop_count(node: *mut fw_node) {
    let mut depths = [-1i32, -1i32];
    let mut max_child_hops = 0;
    for i in 0..(*node).port_count {
        let child = (*node).ports.add(i as usize).read();
        if child.is_null() { continue; }
        if (*child).max_hops > max_child_hops { max_child_hops = (*child).max_hops; }
        if (*child).max_depth > depths[0] {
            depths[1] = depths[0]; depths[0] = (*child).max_depth;
        } else if (*child).max_depth > depths[1] { depths[1] = (*child).max_depth; }
    }
    (*node).max_depth = depths[0] + 1;
    (*node).max_hops = core::cmp::max(max_child_hops, depths[0] + depths[1] + 2);
}

unsafe fn fw_node(l: *mut list_head) -> *mut fw_node { list_entry(l, offset_of_link()) }

type fw_node_callback_t = unsafe fn(*mut fw_card, *mut fw_node, *mut fw_node);

unsafe fn for_each_fw_node(card: *mut fw_card, root: *mut fw_node, callback: fw_node_callback_t) {
    for_each_fw_node_impl(card, root, callback);
}

unsafe fn free_fw_node(_card: *mut fw_card, node: *mut fw_node, _parent: *mut fw_node) { kfree(node); }

unsafe fn build_tree(card: *mut fw_card, sid: *const u32, self_id_count: i32, generation: u32) -> *mut fw_node {
    let mut enumerator = self_id_sequence_enumerator { cursor: sid, quadlet_count: self_id_count };
    let (mut node, mut child, mut local_node, mut irm_node): (*mut fw_node, *mut fw_node, *mut fw_node, *mut fw_node) = (core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    let mut stack = list_head_zero();
    INIT_LIST_HEAD(&mut stack);
    let mut stack_depth = 0;
    let mut phy_id = 0;
    let mut gap_count = phy_packet_self_id_zero_get_gap_count(*sid);
    let mut beta_repeaters_present = false;
    while enumerator.quadlet_count > 0 {
        let mut child_port_count = 0; let mut parent_port_count = 0; let mut total_port_count = 0;
        let mut quadlet_count = 0; let mut port_index; let mut h;
        let self_id_sequence = self_id_sequence_enumerator_next(&mut enumerator, &mut quadlet_count);
        if IS_ERR(self_id_sequence) { if PTR_ERR(self_id_sequence) != -ENODATA { fw_err(card, "inconsistent extended self IDs: %ld\n", PTR_ERR(self_id_sequence)); goto_error(card, &mut stack); return core::ptr::null_mut(); } break; }
        let port_capacity = self_id_sequence_get_port_capacity(quadlet_count);
        trace_self_id_sequence((*card).index, self_id_sequence, quadlet_count, generation);
        for port_index in 0..port_capacity { match self_id_sequence_get_port_status(self_id_sequence, quadlet_count, port_index) {
            PHY_PACKET_SELF_ID_PORT_STATUS_CHILD => child_port_count += 1,
            PHY_PACKET_SELF_ID_PORT_STATUS_PARENT => parent_port_count += 1,
            PHY_PACKET_SELF_ID_PORT_STATUS_NCONN => total_port_count += 1, _ => {}
        }}
        total_port_count += child_port_count + parent_port_count;
        if (enumerator.quadlet_count == 0 && parent_port_count != 0) || (enumerator.quadlet_count > 0 && parent_port_count != 1) { fw_err(card, "parent port inconsistency for node %d: parent_count=%d\n", phy_id, parent_port_count); goto_error(card, &mut stack); return core::ptr::null_mut(); }
        if phy_id != phy_packet_self_id_get_phy_id(*self_id_sequence) { fw_err(card, "PHY ID mismatch in self ID: %d != %d\n", phy_id, phy_packet_self_id_get_phy_id(*self_id_sequence)); goto_error(card, &mut stack); return core::ptr::null_mut(); }
        if child_port_count > stack_depth { fw_err(card, "topology stack underflow\n"); goto_error(card, &mut stack); return core::ptr::null_mut(); }
        h = &mut stack; for _ in 0..child_port_count { h = (*h).prev; } child = fw_node(h);
        node = fw_node_create(*self_id_sequence, total_port_count as i32, (*card).color);
        if node.is_null() { fw_err(card, "out of memory while building topology\n"); goto_error(card, &mut stack); return core::ptr::null_mut(); }
        if phy_id == ((*card).node_id & 0x3f) { local_node = node; }
        if phy_packet_self_id_zero_get_contender(*self_id_sequence) { irm_node = node; }
        for port_index in 0..total_port_count { match self_id_sequence_get_port_status(self_id_sequence, quadlet_count, port_index) {
            PHY_PACKET_SELF_ID_PORT_STATUS_PARENT => (*node).color = port_index as i32,
            PHY_PACKET_SELF_ID_PORT_STATUS_CHILD => { (*node).ports.add(port_index as usize).write(child); (*child).ports.add((*child).color as usize).write(node); (*child).color = (*card).color; child = fw_node((*child).link.next); }, _ => {}
        }}
        __list_del((*h).prev, &mut stack); list_add_tail(&mut (*node).link, &mut stack); stack_depth += 1 - child_port_count as i32;
        if (*node).phy_speed == SCODE_BETA && parent_port_count + child_port_count > 1 { beta_repeaters_present = true; }
        if phy_packet_self_id_zero_get_gap_count(*self_id_sequence) != gap_count { gap_count = GAP_COUNT_MISMATCHED; }
        update_hop_count(node); phy_id += 1;
    }
    (*card).root_node = node; (*card).irm_node = irm_node; (*card).gap_count = gap_count; (*card).beta_repeaters_present = beta_repeaters_present; local_node
}

unsafe fn return_error(_card: *mut fw_card) {}
unsafe fn goto_error(card: *mut fw_card, stack: &mut list_head) { (*card).color += 1; let mut node = fw_node(stack.next); while node != fw_node(stack) { for_each_fw_node(card, node, free_fw_node); node = fw_node(stack.next); } }

unsafe fn for_each_fw_node_impl(card: *mut fw_card, root: *mut fw_node, callback: fw_node_callback_t) {
    let mut list = list_head_zero(); INIT_LIST_HEAD(&mut list); fw_node_get(root); list_add_tail(&mut (*root).link, &mut list);
    let mut parent = core::ptr::null_mut(); let mut node = root;
    while !list_entry_is_head(node, &mut list) {
        (*node).color = (*card).color;
        for i in 0..(*node).port_count { let child = (*node).ports.add(i as usize).read(); if child.is_null() { continue; } if (*child).color == (*card).color { parent = child; } else { fw_node_get(child); list_add_tail(&mut (*child).link, &mut list); } }
        callback(card, node, parent); node = fw_node((*node).link.next);
    }
    let mut n = fw_node(list.next); while !list_entry_is_head(n, &mut list) { let next = fw_node((*n).link.next); fw_node_put(n); n = next; }
}

unsafe fn report_found_node(card: *mut fw_card, node: *mut fw_node, parent: *mut fw_node) {
    let b_path = (*node).phy_speed == SCODE_BETA;
    if !parent.is_null() { (*node).max_speed = core::cmp::min((*parent).max_speed, (*node).phy_speed); (*node).b_path = (*parent).b_path && b_path; } else { (*node).max_speed = (*node).phy_speed; (*node).b_path = b_path; }
    fw_node_event(card, node, FW_NODE_CREATED); (*card).bm_retries = 0;
}

unsafe fn move_tree(node0: *mut fw_node, node1: *mut fw_node, port: i32) {
    let tree = (*node1).ports.add(port as usize).read(); (*node0).ports.add(port as usize).write(tree);
    for i in 0..(*tree).port_count { if (*tree).ports.add(i as usize).read() == node1 { (*tree).ports.add(i as usize).write(node0); break; } }
}

unsafe fn update_tree(card: *mut fw_card, root: *mut fw_node) {
    let mut node0 = (*card).local_node; let mut node1 = root;
    while !node0.is_null() {
        if (*node0).link_on && !(*node1).link_on { fw_node_event(card, node0, FW_NODE_LINK_OFF); } else if !(*node0).link_on && (*node1).link_on { fw_node_event(card, node0, FW_NODE_LINK_ON); } else if (*node1).initiated_reset && (*node1).link_on { fw_node_event(card, node0, FW_NODE_INITIATED_RESET); } else { fw_node_event(card, node0, FW_NODE_UPDATED); }
        (*node0).node_id = (*node1).node_id; (*node0).color = (*card).color; (*node0).link_on = (*node1).link_on; (*node0).initiated_reset = (*node1).initiated_reset; (*node0).max_hops = (*node1).max_hops; (*node1).color = (*card).color;
        if (*card).root_node == node1 { (*card).root_node = node0; } if (*card).irm_node == node1 { (*card).irm_node = node0; }
        for i in 0..(*node0).port_count { let a = (*node0).ports.add(i as usize).read(); let b = (*node1).ports.add(i as usize).read(); if a.is_null() && !b.is_null() { move_tree(node0,node1,i); for_each_fw_node_impl(card,a,report_found_node); } else if !a.is_null() && b.is_null() { for_each_fw_node_impl(card,a,report_lost_node); (*node0).ports.add(i as usize).write(core::ptr::null_mut()); } }
        node0 = fw_node((*node0).link.next); node1 = fw_node((*node1).link.next);
    }
}
unsafe fn free_topology_node(card: *mut fw_card, node: *mut fw_node, parent: *mut fw_node) { free_fw_node(card, node, parent); }

pub unsafe fn fw_destroy_nodes(card: *mut fw_card) {
    lockdep_assert_held(&(*card).lock); (*card).color += 1;
    if !(*card).local_node.is_null() { for_each_fw_node(card, (*card).local_node, report_lost_node); }
    (*card).local_node = core::ptr::null_mut();
}

unsafe fn report_lost_node(card: *mut fw_card, node: *mut fw_node, _parent: *mut fw_node) { fw_node_event(card, node, FW_NODE_DESTROYED); fw_node_put(node); (*card).bm_retries = 0; }

// Additional declarations are supplied by the surrounding kernel translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
