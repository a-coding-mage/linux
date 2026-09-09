// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel/Rust translation.

unsafe fn zl3073x_pin_check_freq(
    zldev: *mut zl3073x_dev,
    dir: dpll_pin_direction,
    id: u8,
    freq: u64,
) -> bool {
    if freq > U32_MAX as u64 {
        return {
            dev_warn((*zldev).dev, "Unsupported frequency %llu Hz in firmware node\n", freq);
            false
        };
    }

    if dir == DPLL_PIN_DIRECTION_INPUT {
        let rc = zl3073x_ref_freq_factorize(freq, core::ptr::null_mut(), core::ptr::null_mut());
        if rc != 0 {
            dev_warn((*zldev).dev, "Unsupported frequency %llu Hz in firmware node\n", freq);
            return false;
        }
    } else {
        let out = zl3073x_output_pin_out_get(id);
        let synth = zl3073x_dev_out_synth_get(zldev, out);
        let synth_freq = zl3073x_dev_synth_freq_get(zldev, synth);
        if synth_freq % freq as u32 != 0 {
            dev_warn((*zldev).dev, "Unsupported frequency %llu Hz in firmware node\n", freq);
            return false;
        }
    }

    true
}

unsafe fn zl3073x_prop_pin_package_label_set(
    zldev: *mut zl3073x_dev,
    props: *mut zl3073x_pin_props,
    dir: dpll_pin_direction,
    id: u8,
) {
    let (prefix, is_diff) = if dir == DPLL_PIN_DIRECTION_INPUT {
        let r#ref = zl3073x_input_pin_ref_get(id);
        ("REF", zl3073x_dev_ref_is_diff(zldev, r#ref))
    } else {
        let out = zl3073x_output_pin_out_get(id);
        ("OUT", zl3073x_dev_out_is_diff(zldev, out))
    };
    let suffix = if !is_diff {
        if zl3073x_is_p_pin(id) { "P" } else { "N" }
    } else { "" };

    snprintf((*props).package_label.as_mut_ptr(), core::mem::size_of_val(&(*props).package_label),
             "%s%u%s", prefix, id / 2, suffix);
    (*props).dpll_props.package_label = (*props).package_label.as_ptr();
}

unsafe fn zl3073x_prop_pin_fwnode_get(
    zldev: *mut zl3073x_dev,
    props: *mut zl3073x_pin_props,
    dir: dpll_pin_direction,
    id: u8,
) -> i32 {
    let node_name = if dir == DPLL_PIN_DIRECTION_INPUT { "input-pins" } else { "output-pins" };
    let pins_node = device_get_named_child_node((*zldev).dev, node_name);
    if pins_node.is_null() {
        dev_dbg((*zldev).dev, "'%s' sub-node is missing\n", node_name);
        return -ENOENT;
    }

    let mut pin_node = core::ptr::null_mut();
    fwnode_for_each_child_node(pins_node, |node| {
        let mut reg = 0u32;
        if fwnode_property_read_u32(node, "reg", &mut reg) == 0 && id as u32 == reg {
            pin_node = node;
            true
        } else { false }
    });
    fwnode_handle_put(pins_node);
    (*props).fwnode = pin_node;
    dev_dbg((*zldev).dev, "Firmware node for %s %sfound\n", (*props).package_label.as_ptr(),
            if pin_node.is_null() { "NOT " } else { "" });
    if pin_node.is_null() { -ENOENT } else { 0 }
}

pub unsafe fn zl3073x_pin_props_get(
    zldev: *mut zl3073x_dev, dir: dpll_pin_direction, index: u8,
) -> *mut zl3073x_pin_props {
    let mut props = kzalloc_obj::<zl3073x_pin_props>();
    if props.is_null() { return ERR_PTR(-ENOMEM); }
    let curr_freq;
    if dir == DPLL_PIN_DIRECTION_INPUT {
        (*props).dpll_props.r#type = DPLL_PIN_TYPE_EXT;
        (*props).dpll_props.capabilities = DPLL_PIN_CAPABILITIES_PRIORITY_CAN_CHANGE |
            DPLL_PIN_CAPABILITIES_STATE_CAN_CHANGE;
        curr_freq = zl3073x_dev_ref_freq_get(zldev, index);
    } else {
        (*props).dpll_props.r#type = DPLL_PIN_TYPE_GNSS;
        let out = zl3073x_output_pin_out_get(index);
        let synth = zl3073x_dev_out_synth_get(zldev, out);
        let f = 2 * zl3073x_dev_synth_freq_get(zldev, synth);
        (*props).dpll_props.phase_gran = if f != 0 { div_u64(PSEC_PER_SEC, f) } else { 1 };
        curr_freq = zl3073x_dev_output_pin_freq_get(zldev, index);
    }
    (*props).dpll_props.phase_range.min = S32_MIN;
    (*props).dpll_props.phase_range.max = S32_MAX;
    zl3073x_prop_pin_package_label_set(zldev, props, dir, index);
    let mut rc = zl3073x_prop_pin_fwnode_get(zldev, props, dir, index);
    let mut freqs: *mut u64 = core::ptr::null_mut();
    let mut num_freqs = 0i32;
    if rc == 0 {
        fwnode_property_read_string((*props).fwnode, "label", &mut (*props).dpll_props.board_label);
        let mut r#type = core::ptr::null();
        if fwnode_property_read_string((*props).fwnode, "connection-type", &mut r#type) == 0 {
            if strcmp(r#type, "ext") == 0 { (*props).dpll_props.r#type = DPLL_PIN_TYPE_EXT; }
            else if strcmp(r#type, "gnss") == 0 { (*props).dpll_props.r#type = DPLL_PIN_TYPE_GNSS; }
            else if strcmp(r#type, "int") == 0 { (*props).dpll_props.r#type = DPLL_PIN_TYPE_INT_OSCILLATOR; }
            else if strcmp(r#type, "synce") == 0 { (*props).dpll_props.r#type = DPLL_PIN_TYPE_SYNCE_ETH_PORT; }
            else if strcmp(r#type, "mux") == 0 { (*props).dpll_props.r#type = DPLL_PIN_TYPE_MUX; }
            else { dev_warn((*zldev).dev, "Unknown or unsupported pin type '%s'\n", r#type); }
        }
        (*props).esync_control = fwnode_property_read_bool((*props).fwnode, "esync-control");
        num_freqs = fwnode_property_count_u64((*props).fwnode, "supported-frequencies-hz");
        if num_freqs > 0 {
            freqs = kcalloc(num_freqs as usize, core::mem::size_of::<u64>(), GFP_KERNEL);
            if freqs.is_null() { rc = -ENOMEM; }
            else { fwnode_property_read_u64_array((*props).fwnode, "supported-frequencies-hz", freqs, num_freqs); }
        } else { num_freqs = 0; }
    }
    if rc != 0 && !freqs.is_null() { kfree(freqs); }
    if rc != 0 && (*props).fwnode.is_null() { /* no firmware node is valid */ }
    let ranges = kzalloc_objs::<dpll_pin_frequency>((num_freqs + 1) as usize);
    if ranges.is_null() { kfree(freqs); fwnode_handle_put((*props).fwnode); kfree(props); return ERR_PTR(-ENOMEM); }
    (*ranges) = DPLL_PIN_FREQUENCY(curr_freq);
    let mut j = 1usize;
    for i in 0..num_freqs as usize {
        let freq = *freqs.add(i);
        if freq != curr_freq && zl3073x_pin_check_freq(zldev, dir, index, freq) {
            *ranges.add(j) = DPLL_PIN_FREQUENCY(freq); j += 1;
        }
    }
    kfree(freqs);
    (*props).dpll_props.freq_supported = ranges;
    (*props).dpll_props.freq_supported_num = j;
    props
}

pub unsafe fn zl3073x_pin_props_put(props: *mut zl3073x_pin_props) {
    kfree((*props).dpll_props.freq_supported);
    if !(*props).fwnode.is_null() { fwnode_handle_put((*props).fwnode); }
    kfree(props);
}

pub unsafe fn zl3073x_prop_dpll_type_get(zldev: *mut zl3073x_dev, index: u8) -> dpll_type {
    let mut types: [*const i8; ZL3073X_MAX_CHANNELS] = [core::ptr::null(); ZL3073X_MAX_CHANNELS];
    let count = device_property_read_string_array((*zldev).dev, "dpll-types", types.as_mut_ptr(), types.len());
    if index as isize >= count { return DPLL_TYPE_PPS; }
    if strcmp(types[index as usize], "pps") == 0 { DPLL_TYPE_PPS }
    else if strcmp(types[index as usize], "eec") == 0 { DPLL_TYPE_EEC }
    else { dev_info((*zldev).dev, "Unknown DPLL type '%s', using default\n", types[index as usize]); DPLL_TYPE_PPS }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
