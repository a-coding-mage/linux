// SPDX-License-Identifier: GPL-2.0-or-later
// (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation. 2007.
//! Ordered device-tree validation, reference resolution, and semantic fixups.

use crate::dtc_header::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Unchecked,
    Prerequisite,
    Passed,
    Failed,
}

struct Check {
    name: &'static str,
    warn: bool,
    error: bool,
    prerequisites: Vec<&'static str>,
    status: Status,
}

const PROVIDERS: &[(&str, &str, &str, bool)] = &[
    ("clocks", "clocks", "#clock-cells", false),
    ("cooling_device", "cooling-device", "#cooling-cells", false),
    ("dmas", "dmas", "#dma-cells", false),
    ("hwlocks", "hwlocks", "#hwlock-cells", false),
    (
        "interrupts_extended",
        "interrupts-extended",
        "#interrupt-cells",
        false,
    ),
    ("io_channels", "io-channels", "#io-channel-cells", false),
    ("iommus", "iommus", "#iommu-cells", false),
    ("mboxes", "mboxes", "#mbox-cells", false),
    ("msi_parent", "msi-parent", "#msi-cells", true),
    ("mux_controls", "mux-controls", "#mux-control-cells", false),
    ("phys", "phys", "#phy-cells", false),
    (
        "power_domains",
        "power-domains",
        "#power-domain-cells",
        false,
    ),
    ("pwms", "pwms", "#pwm-cells", false),
    ("resets", "resets", "#reset-cells", false),
    ("sound_dai", "sound-dai", "#sound-dai-cells", false),
    (
        "thermal_sensors",
        "thermal-sensors",
        "#thermal-sensor-cells",
        false,
    ),
];

fn definitions() -> Vec<Check> {
    let mut checks = Vec::new();
    let mut add = |name, level, prerequisites: &[&'static str]| {
        checks.push(Check {
            name,
            warn: level == 1,
            error: level == 2,
            prerequisites: prerequisites.to_vec(),
            status: Status::Unchecked,
        })
    };
    add("duplicate_node_names", 2, &[]);
    add("duplicate_property_names", 2, &[]);
    add("node_name_chars", 2, &[]);
    add("node_name_format", 2, &["node_name_chars"]);
    add("node_name_not_empty", 2, &["node_name_chars"]);
    add("property_name_chars", 2, &[]);
    add("name_is_string", 2, &[]);
    add("name_properties", 2, &["name_is_string"]);
    add("node_name_vs_property_name", 1, &["node_name_chars"]);
    add("duplicate_label", 2, &[]);
    add("explicit_phandles", 2, &[]);
    add(
        "phandle_references",
        2,
        &["duplicate_node_names", "explicit_phandles"],
    );
    add("path_references", 2, &["duplicate_node_names"]);
    add(
        "omit_unused_nodes",
        2,
        &["phandle_references", "path_references"],
    );
    add("address_cells_is_cell", 1, &[]);
    add("size_cells_is_cell", 1, &[]);
    add("device_type_is_string", 1, &[]);
    add("model_is_string", 1, &[]);
    add("status_is_string", 1, &[]);
    add("label_is_string", 1, &[]);
    add("compatible_is_string_list", 1, &[]);
    add("names_is_string_list", 1, &[]);
    add("property_name_chars_strict", 0, &[]);
    add("node_name_chars_strict", 0, &[]);
    add(
        "addr_size_cells",
        1,
        &["address_cells_is_cell", "size_cells_is_cell"],
    );
    add("reg_format", 1, &["addr_size_cells"]);
    add("ranges_format", 1, &["addr_size_cells"]);
    add("dma_ranges_format", 1, &["addr_size_cells"]);
    add("unit_address_vs_reg", 1, &[]);
    add(
        "unit_address_format",
        1,
        &["node_name_format", "pci_bridge", "simple_bus_bridge"],
    );
    add(
        "pci_bridge",
        1,
        &["device_type_is_string", "addr_size_cells"],
    );
    add("pci_device_reg", 1, &["reg_format", "pci_bridge"]);
    add("pci_device_bus_num", 1, &["reg_format", "pci_bridge"]);
    add(
        "simple_bus_bridge",
        1,
        &["addr_size_cells", "compatible_is_string_list"],
    );
    add("simple_bus_reg", 1, &["reg_format", "simple_bus_bridge"]);
    add("i2c_bus_bridge", 1, &["addr_size_cells"]);
    add("i2c_bus_reg", 1, &["reg_format", "i2c_bus_bridge"]);
    add("spi_bus_bridge", 1, &["addr_size_cells"]);
    add("spi_bus_reg", 1, &["reg_format", "spi_bus_bridge"]);
    add("avoid_default_addr_size", 1, &["addr_size_cells"]);
    add(
        "avoid_unnecessary_addr_size",
        1,
        &["avoid_default_addr_size"],
    );
    add("unique_unit_address", 1, &["avoid_default_addr_size"]);
    add(
        "unique_unit_address_if_enabled",
        0,
        &["avoid_default_addr_size"],
    );
    add("obsolete_chosen_interrupt_controller", 1, &[]);
    add("chosen_node_is_root", 1, &[]);
    add("chosen_node_bootargs", 1, &[]);
    add("chosen_node_stdout_path", 1, &[]);
    for (property, cell) in [
        ("clocks_property", "clocks_is_cell"),
        ("cooling_device_property", "cooling_device_is_cell"),
        ("dmas_property", "dmas_is_cell"),
        ("hwlocks_property", "hwlocks_is_cell"),
        (
            "interrupts_extended_property",
            "interrupts_extended_is_cell",
        ),
        ("io_channels_property", "io_channels_is_cell"),
        ("iommus_property", "iommus_is_cell"),
        ("mboxes_property", "mboxes_is_cell"),
        ("msi_parent_property", "msi_parent_is_cell"),
        ("mux_controls_property", "mux_controls_is_cell"),
        ("phys_property", "phys_is_cell"),
        ("power_domains_property", "power_domains_is_cell"),
        ("pwms_property", "pwms_is_cell"),
        ("resets_property", "resets_is_cell"),
        ("sound_dai_property", "sound_dai_is_cell"),
        ("thermal_sensors_property", "thermal_sensors_is_cell"),
    ] {
        add(property, 1, &[cell, "phandle_references"]);
        add(cell, 1, &[]);
    }
    add("deprecated_gpio_property", 0, &[]);
    add("gpios_property", 1, &["phandle_references"]);
    // The C declaration passes phandle_references as data, not as a prerequisite.
    add("interrupts_property", 1, &[]);
    add("interrupt_provider", 1, &["interrupts_extended_is_cell"]);
    add(
        "interrupt_map",
        1,
        &[
            "phandle_references",
            "addr_size_cells",
            "interrupt_provider",
        ],
    );
    add("alias_paths", 1, &[]);
    add("graph_nodes", 1, &[]);
    add("graph_port", 1, &["graph_nodes"]);
    add("graph_endpoint", 1, &["graph_nodes"]);
    add("always_fail", 0, &[]);
    checks
}

fn bytes_format(template: &str, arguments: &[&[u8]]) -> Vec<u8> {
    let mut result = Vec::new();
    for (index, part) in template.split("{}").enumerate() {
        if index != 0 {
            result.extend_from_slice(arguments[index - 1]);
        }
        result.extend_from_slice(part.as_bytes());
    }
    result
}

macro_rules! msg {
    ($template:literal $(, $argument:expr)* $(,)?) => {
        bytes_format($template, &[$($argument),*])
    };
}

fn c_string(bytes: &[u8]) -> &[u8] {
    &bytes[..bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len())]
}

fn unit_name(node: &Node) -> &[u8] {
    node.name.get(node.basenamelen + 1..).unwrap_or(&[])
}

fn base_name(node: &Node) -> &[u8] {
    &node.name[..node.basenamelen]
}
fn addr_cells(node: &Node) -> i32 {
    if node.addr_cells == -1 {
        2
    } else {
        node.addr_cells
    }
}
fn size_cells(node: &Node) -> i32 {
    if node.size_cells == -1 {
        1
    } else {
        node.size_cells
    }
}
fn multiple(length: usize, size: i64) -> bool {
    if size == 0 {
        length == 0
    } else {
        (length as i64) % size == 0
    }
}
fn cell(property: &Property, index: usize) -> Option<u32> {
    let bytes = property
        .data
        .bytes
        .get(index.checked_mul(4)?..index.checked_mul(4)?.checked_add(4)?)?;
    Some(u32::from_be_bytes(bytes.try_into().ok()?))
}
fn prop<'a>(dti: &'a DtInfo, node: NodeId, name: &[u8]) -> Option<&'a Property> {
    dti.property(node, name)
        .map(|index| &dti.nodes[node].properties[index])
}
fn compatible(dti: &DtInfo, node: NodeId, name: &[u8]) -> bool {
    prop(dti, node, b"compatible")
        .is_some_and(|p| p.data.bytes.split(|&b| b == 0).any(|s| s == name))
}
fn gpio(name: &[u8]) -> bool {
    !name.ends_with(b",nr-gpios")
        && (name == b"gpios"
            || name == b"gpio"
            || name.ends_with(b"-gpios")
            || name.ends_with(b"-gpio"))
}

struct Checker<'a> {
    options: &'a Options,
    diagnostics: &'a mut Diagnostics,
    checks: Vec<Check>,
}

impl Checker<'_> {
    fn string(&mut self, dti: &DtInfo, c: usize, n: NodeId, name: &[u8], list: bool) {
        let Some(p) = dti.property(n, name) else {
            return;
        };
        let data = &dti.nodes[n].properties[p].data;
        let good = if list {
            data.bytes.is_empty() || data.bytes.last() == Some(&0)
        } else {
            data.is_one_string()
        };
        if !good {
            self.fail(
                dti,
                c,
                n,
                Some(p),
                if list {
                    "property is not a string list"
                } else {
                    "property is not a string"
                },
            );
        }
    }

    fn single_cell(&mut self, dti: &DtInfo, c: usize, n: NodeId, name: &[u8]) {
        if let Some(p) = dti.property(n, name) {
            if dti.nodes[n].properties[p].data.bytes.len() != 4 {
                self.fail(dti, c, n, Some(p), "property is not a single cell");
            }
        }
    }

    fn label_description(dti: &DtInfo, n: NodeId, p: Option<PropId>, marker: bool) -> Vec<u8> {
        let mut text = Vec::new();
        if marker {
            text.extend_from_slice(b"value of ");
        }
        if let Some(p) = p {
            text.extend_from_slice(&msg!("'{}' in ", &dti.nodes[n].properties[p].name));
        }
        text.extend_from_slice(&dti.nodes[n].fullpath);
        text
    }

    fn duplicate_label(
        &mut self,
        dti: &DtInfo,
        c: usize,
        n: NodeId,
        p: Option<PropId>,
        marker: Option<usize>,
        label: &[u8],
    ) {
        let other = dti
            .node_by_label(label)
            .map(|n| (n, None, None))
            .or_else(|| {
                dti.property_by_label(label)
                    .map(|(n, p)| (n, Some(p), None))
            })
            .or_else(|| {
                dti.marker_label(label)
                    .map(|(n, p, m)| (n, Some(p), Some(m)))
            });
        if let Some((on, op, om)) = other {
            if (on, op, om) != (n, p, marker) {
                self.fail(
                    dti,
                    c,
                    n,
                    None,
                    msg!(
                        "Duplicate label '{}' on {} and {}",
                        label,
                        &Self::label_description(dti, n, p, marker.is_some()),
                        &Self::label_description(dti, on, op, om.is_some())
                    ),
                );
            }
        }
    }

    fn phandle_property(&mut self, dti: &DtInfo, c: usize, n: NodeId, name: &[u8]) -> u32 {
        let Some(p) = dti.property(n, name) else {
            return 0;
        };
        let prop = &dti.nodes[n].properties[p];
        if prop.data.bytes.len() != 4 {
            self.fail(
                dti,
                c,
                n,
                Some(p),
                msg!(
                    "bad length ({}) {} property",
                    prop.data.bytes.len().to_string().as_bytes(),
                    &prop.name
                ),
            );
            return 0;
        }
        if let Some(marker) = prop
            .data
            .markers
            .iter()
            .find(|m| m.kind == MarkerKind::RefPhandle)
        {
            if marker.reference.as_deref().and_then(|r| dti.node_by_ref(r)) != Some(n) {
                self.fail(
                    dti,
                    c,
                    n,
                    None,
                    msg!("{} is a reference to another node", &prop.name),
                );
            }
            return 0;
        }
        let value = cell(prop, 0).unwrap_or(0);
        if !phandle_is_valid(value) {
            self.fail(
                dti,
                c,
                n,
                Some(p),
                msg!(
                    "bad value (0x{}) in {} property",
                    format!("{value:x}").as_bytes(),
                    &prop.name
                ),
            );
            return 0;
        }
        value
    }

    fn references(&mut self, dti: &mut DtInfo, c: usize, n: NodeId, phandle: bool) {
        let kind = if phandle {
            MarkerKind::RefPhandle
        } else {
            MarkerKind::RefPath
        };
        let mut p = 0;
        while p < dti.nodes[n].properties.len() {
            if dti.nodes[n].properties[p].deleted {
                p += 1;
                continue;
            }
            let count = dti.nodes[n].properties[p].data.markers.len();
            for m in 0..count {
                let marker = dti.nodes[n].properties[p].data.markers[m].clone();
                if marker.kind != kind {
                    continue;
                }
                let reference = marker.reference.as_deref().unwrap_or(&[]);
                let Some(target) = dti.node_by_ref(reference) else {
                    if !phandle || dti.dtsflags & DTSF_PLUGIN == 0 {
                        self.fail(
                            dti,
                            c,
                            n,
                            None,
                            msg!(
                                "Reference to non-existent node or label \"{}\"\n",
                                reference
                            ),
                        );
                    } else if let Some(bytes) = dti.nodes[n].properties[p]
                        .data
                        .bytes
                        .get_mut(marker.offset..marker.offset.saturating_add(4))
                    {
                        bytes.copy_from_slice(&u32::MAX.to_be_bytes());
                    }
                    continue;
                };
                if phandle {
                    let value = dti.node_phandle(target, self.options.phandle_format);
                    if let Some(bytes) = dti.nodes[n].properties[p]
                        .data
                        .bytes
                        .get_mut(marker.offset..marker.offset.saturating_add(4))
                    {
                        bytes.copy_from_slice(&value.to_be_bytes());
                    } else {
                        self.fail(
                            dti,
                            c,
                            n,
                            Some(p),
                            "phandle reference exceeds property data",
                        );
                    }
                } else {
                    let mut path = dti.nodes[target].fullpath.clone();
                    path.push(0);
                    if marker.offset <= dti.nodes[n].properties[p].data.bytes.len() {
                        dti.nodes[n].properties[p].data.insert_at_marker(m, &path);
                    } else {
                        self.fail(dti, c, n, Some(p), "path reference exceeds property data");
                    }
                }
                dti.nodes[target].is_referenced = true;
            }
            p += 1;
        }
    }

    fn node(&mut self, dti: &mut DtInfo, c: usize, n: NodeId) {
        let check = self.checks[c].name;
        let node = &dti.nodes[n];
        match check {
            "always_fail" => self.fail(dti, c, n, None, "always_fail check"),
            "duplicate_node_names" => {
                for (index, &child) in node.children.iter().enumerate() {
                    if dti.nodes[child].deleted {
                        continue;
                    }
                    for &other in &node.children[index + 1..] {
                        if dti.nodes[child].name == dti.nodes[other].name {
                            self.fail(dti, c, other, None, "Duplicate node name");
                        }
                    }
                }
            }
            "duplicate_property_names" => {
                for (p, prop) in node
                    .properties
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| !p.deleted)
                {
                    for other in node.properties[p + 1..].iter().filter(|p| !p.deleted) {
                        if prop.name == other.name {
                            self.fail(dti, c, n, Some(p), "Duplicate property name");
                        }
                    }
                }
            }
            "node_name_chars" | "node_name_chars_strict" => {
                let strict = check.ends_with("strict");
                let allowed = if strict {
                    b",-".as_slice()
                } else {
                    b",._+-@".as_slice()
                };
                if let Some(index) = node
                    .name
                    .iter()
                    .position(|b| !b.is_ascii_alphanumeric() && !allowed.contains(b))
                {
                    if !strict || index < node.basenamelen {
                        self.fail(
                            dti,
                            c,
                            n,
                            None,
                            bytes_format(
                                if strict {
                                    "Character '{}' not recommended in node name"
                                } else {
                                    "Bad character '{}' in node name"
                                },
                                &[&node.name[index..index + 1]],
                            ),
                        );
                    }
                }
            }
            "node_name_format" => {
                if unit_name(node).contains(&b'@') {
                    self.fail(dti, c, n, None, "multiple '@' characters in node name");
                }
            }
            "node_name_not_empty" => {
                if node.basenamelen == 0 && node.parent.is_some() {
                    self.fail(dti, c, n, None, "Empty node name");
                }
            }
            "node_name_vs_property_name" => {
                if node
                    .parent
                    .is_some_and(|p| dti.property(p, &node.name).is_some())
                {
                    self.fail(dti, c, n, None, "node name and property name conflict");
                }
            }
            "unit_address_vs_reg" => {
                if dti.subnode(n, b"__overlay__").is_some() {
                    return;
                }
                let has_reg = dti.property(n, b"reg").is_some()
                    || prop(dti, n, b"ranges").is_some_and(|p| !p.data.bytes.is_empty());
                if has_reg && unit_name(node).is_empty() {
                    self.fail(
                        dti,
                        c,
                        n,
                        None,
                        "node has a reg or ranges property, but no unit name",
                    );
                }
                if !has_reg && !unit_name(node).is_empty() {
                    self.fail(
                        dti,
                        c,
                        n,
                        None,
                        "node has a unit name, but no reg or ranges property",
                    );
                }
            }
            "property_name_chars" | "property_name_chars_strict" => {
                let strict = check.ends_with("strict");
                let allowed = if strict {
                    b",-".as_slice()
                } else {
                    b",._+*#?-".as_slice()
                };
                for (p, prop) in node
                    .properties
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| !p.deleted)
                {
                    if strict && prop.name == b"device_type" {
                        continue;
                    }
                    let mut name = prop.name.as_slice();
                    let mut bad = name
                        .iter()
                        .position(|b| !b.is_ascii_alphanumeric() && !allowed.contains(b));
                    if strict {
                        if let Some(index) = bad {
                            if name[index] == b'#' && (index == 0 || name[index - 1] == b',') {
                                name = &name[index + 1..];
                                bad = name.iter().position(|b| {
                                    !b.is_ascii_alphanumeric() && !allowed.contains(b)
                                });
                            }
                        }
                    }
                    if let Some(index) = bad {
                        self.fail(
                            dti,
                            c,
                            n,
                            Some(p),
                            bytes_format(
                                if strict {
                                    "Character '{}' not recommended in property name"
                                } else {
                                    "Bad character '{}' in property name"
                                },
                                &[&name[index..index + 1]],
                            ),
                        );
                    }
                }
            }
            "duplicate_label" => {
                for label in node.labels.iter().filter(|l| !l.deleted) {
                    self.duplicate_label(dti, c, n, None, None, &label.name);
                }
                for (p, prop) in node
                    .properties
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| !p.deleted)
                {
                    for label in prop.labels.iter().filter(|l| !l.deleted) {
                        self.duplicate_label(dti, c, n, Some(p), None, &label.name);
                    }
                    for (m, marker) in prop
                        .data
                        .markers
                        .iter()
                        .enumerate()
                        .filter(|(_, m)| m.kind == MarkerKind::Label)
                    {
                        self.duplicate_label(
                            dti,
                            c,
                            n,
                            Some(p),
                            Some(m),
                            marker.reference.as_deref().unwrap_or(&[]),
                        );
                    }
                }
            }
            "explicit_phandles" => {
                let mut value = self.phandle_property(dti, c, n, b"phandle");
                let linux = self.phandle_property(dti, c, n, b"linux,phandle");
                if value == 0 && linux == 0 {
                    return;
                }
                if value != 0 && linux != 0 && value != linux {
                    self.fail(
                        dti,
                        c,
                        n,
                        None,
                        "mismatching 'phandle' and 'linux,phandle' properties",
                    );
                }
                if value == 0 {
                    value = linux;
                }
                if let Some(other) = dti.node_by_phandle(value).filter(|&other| other != n) {
                    self.fail(
                        dti,
                        c,
                        n,
                        None,
                        msg!(
                            "duplicated phandle 0x{} (seen before at {})",
                            format!("{value:x}").as_bytes(),
                            &dti.nodes[other].fullpath
                        ),
                    );
                } else {
                    dti.nodes[n].phandle = value;
                }
            }
            "name_properties" => {
                if let Some(p) = node.properties.iter().position(|p| p.name == b"name") {
                    let value = &node.properties[p].data.bytes;
                    if value.len() != node.basenamelen + 1
                        || value.get(..node.basenamelen) != Some(base_name(node))
                    {
                        self.fail(
                            dti,
                            c,
                            n,
                            None,
                            msg!(
                                "\"name\" property is incorrect (\"{}\" instead of base node name)",
                                c_string(value)
                            ),
                        );
                    } else {
                        dti.nodes[n].properties.remove(p);
                    }
                }
            }
            "phandle_references" => self.references(dti, c, n, true),
            "path_references" => self.references(dti, c, n, false),
            "omit_unused_nodes" => {
                if !(self.options.generate_symbols && !node.labels.is_empty())
                    && node.omit_if_unused
                    && !node.is_referenced
                {
                    dti.delete_node(n);
                }
            }
            "name_is_string" => self.string(dti, c, n, b"name", false),
            "device_type_is_string" => self.string(dti, c, n, b"device_type", false),
            "model_is_string" => self.string(dti, c, n, b"model", false),
            "status_is_string" => self.string(dti, c, n, b"status", false),
            "label_is_string" => self.string(dti, c, n, b"label", false),
            "compatible_is_string_list" => self.string(dti, c, n, b"compatible", true),
            "names_is_string_list" => {
                for p in node
                    .properties
                    .iter()
                    .filter(|p| !p.deleted && p.name.ends_with(b"-names"))
                {
                    self.string(dti, c, n, &p.name, true);
                }
            }
            "address_cells_is_cell" => self.single_cell(dti, c, n, b"#address-cells"),
            "size_cells_is_cell" => self.single_cell(dti, c, n, b"#size-cells"),
            "addr_size_cells" => {
                let addr = prop(dti, n, b"#address-cells")
                    .and_then(|p| cell(p, 0))
                    .map(|v| v as i32)
                    .unwrap_or(-1);
                let size = prop(dti, n, b"#size-cells")
                    .and_then(|p| cell(p, 0))
                    .map(|v| v as i32)
                    .unwrap_or(-1);
                dti.nodes[n].addr_cells = addr;
                dti.nodes[n].size_cells = size;
            }
            "alias_paths" => {
                if node.name != b"aliases" {
                    return;
                }
                for (p, prop) in node
                    .properties
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| !p.deleted)
                {
                    if prop.name == b"phandle" || prop.name == b"linux,phandle" {
                        continue;
                    }
                    if dti.dtsflags & DTSF_PLUGIN == 0
                        && (prop.data.bytes.is_empty()
                            || dti.node_by_path(c_string(&prop.data.bytes)).is_none())
                    {
                        self.fail(
                            dti,
                            c,
                            n,
                            Some(p),
                            msg!(
                                "aliases property is not a valid node ({})",
                                if prop.data.bytes.is_empty() {
                                    b"(null)".as_slice()
                                } else {
                                    c_string(&prop.data.bytes)
                                }
                            ),
                        );
                        continue;
                    }
                    if prop
                        .name
                        .iter()
                        .any(|b| !(b.is_ascii_lowercase() || b.is_ascii_digit() || *b == b'-'))
                    {
                        self.fail(
                            dti,
                            c,
                            n,
                            None,
                            "aliases property name must include only lowercase and '-'",
                        );
                    }
                }
            }
            "reg_format" | "ranges_format" | "dma_ranges_format" => {
                self.reg_ranges(dti, c, n, check)
            }
            "pci_bridge"
            | "pci_device_reg"
            | "pci_device_bus_num"
            | "simple_bus_bridge"
            | "simple_bus_reg"
            | "i2c_bus_bridge"
            | "i2c_bus_reg"
            | "spi_bus_bridge"
            | "spi_bus_reg"
            | "unit_address_format" => self.bus(dti, c, n, check),
            "avoid_default_addr_size"
            | "avoid_unnecessary_addr_size"
            | "unique_unit_address"
            | "unique_unit_address_if_enabled"
            | "obsolete_chosen_interrupt_controller"
            | "chosen_node_is_root"
            | "chosen_node_bootargs"
            | "chosen_node_stdout_path" => self.style(dti, c, n, check),
            "gpios_property"
            | "deprecated_gpio_property"
            | "interrupts_property"
            | "interrupt_provider"
            | "interrupt_map" => self.interrupt_gpio(dti, c, n, check),
            "graph_nodes" | "graph_port" | "graph_endpoint" => self.graph(dti, c, n, check),
            _ => {
                for &(stem, property, cell_name, optional) in PROVIDERS {
                    if check.strip_suffix("_is_cell") == Some(stem) {
                        self.single_cell(dti, c, n, cell_name.as_bytes());
                        return;
                    }
                    if check.strip_suffix("_property") == Some(stem) {
                        if let Some(p) = dti.property(n, property.as_bytes()) {
                            self.phandle_args(dti, c, n, p, cell_name.as_bytes(), optional);
                        }
                        return;
                    }
                }
                unreachable!("registered check has an implementation");
            }
        }
    }

    fn index(&self, name: &str) -> usize {
        self.checks
            .iter()
            .position(|c| c.name == name)
            .expect("registered check")
    }

    fn reg_ranges(&mut self, dti: &DtInfo, c: usize, n: NodeId, check: &str) {
        let name = match check {
            "reg_format" => b"reg".as_slice(),
            "ranges_format" => b"ranges",
            _ => b"dma-ranges",
        };
        let Some(p) = dti.property(n, name) else {
            return;
        };
        let node = &dti.nodes[n];
        let length = node.properties[p].data.bytes.len();
        let Some(parent) = node.parent.map(|p| &dti.nodes[p]) else {
            self.fail(
                dti,
                c,
                n,
                if check == "reg_format" { None } else { Some(p) },
                msg!("Root node has a \"{}\" property", name),
            );
            return;
        };
        let pa = addr_cells(parent);
        let ps = size_cells(parent);
        if check == "reg_format" {
            if length == 0 {
                self.fail(dti, c, n, Some(p), "property is empty");
            }
            if !multiple(length, i64::from(pa.wrapping_add(ps).wrapping_mul(4))) {
                self.fail(dti,c,n,Some(p),format!("property has invalid length ({length} bytes) (#address-cells == {pa}, #size-cells == {ps})"));
            }
            return;
        }
        let ca = addr_cells(node);
        let cs = size_cells(node);
        if length == 0 {
            if pa != ca {
                self.fail(
                    dti,
                    c,
                    n,
                    Some(p),
                    msg!(
                        "empty \"{}\" property but its #address-cells ({}) differs from {} ({})",
                        name,
                        ca.to_string().as_bytes(),
                        &parent.fullpath,
                        pa.to_string().as_bytes()
                    ),
                );
            }
            if ps != cs {
                self.fail(
                    dti,
                    c,
                    n,
                    Some(p),
                    msg!(
                        "empty \"{}\" property but its #size-cells ({}) differs from {} ({})",
                        name,
                        cs.to_string().as_bytes(),
                        &parent.fullpath,
                        ps.to_string().as_bytes()
                    ),
                );
            }
        } else if !multiple(
            length,
            i64::from(pa.wrapping_add(ca).wrapping_add(cs).wrapping_mul(4)),
        ) {
            self.fail(dti,c,n,Some(p),msg!("\"{}\" property has invalid length ({} bytes) (parent #address-cells == {}, child #address-cells == {}, #size-cells == {})",name,length.to_string().as_bytes(),pa.to_string().as_bytes(),ca.to_string().as_bytes(),cs.to_string().as_bytes()));
        }
    }

    fn style(&mut self, dti: &DtInfo, c: usize, n: NodeId, check: &str) {
        let node = &dti.nodes[n];
        match check {
            "avoid_default_addr_size" => {
                let Some(parent) = node.parent.map(|p| &dti.nodes[p]) else {
                    return;
                };
                if dti.property(n, b"reg").is_none() && dti.property(n, b"ranges").is_none() {
                    return;
                }
                if parent.addr_cells == -1 {
                    self.fail(dti, c, n, None, "Relying on default #address-cells value");
                }
                if parent.size_cells == -1 {
                    self.fail(dti, c, n, None, "Relying on default #size-cells value");
                }
            }
            "avoid_unnecessary_addr_size" => {
                if node.parent.is_none()
                    || node.addr_cells < 0
                    || node.size_cells < 0
                    || node.children.is_empty()
                    || dti.property(n, b"ranges").is_some()
                    || dti.property(n, b"dma-ranges").is_some()
                {
                    return;
                }
                if node
                    .children
                    .iter()
                    .filter(|&&child| !dti.nodes[child].deleted)
                    .any(|&child| {
                        dti.property(child, b"reg").is_some()
                            || dti.property(child, b"ranges").is_some()
                    })
                {
                    return;
                }
                self.fail(dti,c,n,None,"unnecessary #address-cells/#size-cells without \"ranges\", \"dma-ranges\" or child \"reg\" or \"ranges\" property");
            }
            "unique_unit_address" | "unique_unit_address_if_enabled" => {
                if node.addr_cells < 0 || node.size_cells < 0 {
                    return;
                }
                let disabled = |child| {
                    prop(dti, child, b"status")
                        .is_some_and(|p| c_string(&p.data.bytes) == b"disabled")
                };
                let skip_disabled = check == "unique_unit_address_if_enabled";
                for (index, &a) in node.children.iter().enumerate() {
                    if dti.nodes[a].deleted
                        || unit_name(&dti.nodes[a]).is_empty()
                        || skip_disabled && disabled(a)
                    {
                        continue;
                    }
                    for &b in &node.children[..index] {
                        if dti.nodes[b].deleted || skip_disabled && disabled(b) {
                            continue;
                        }
                        if unit_name(&dti.nodes[a]) == unit_name(&dti.nodes[b]) {
                            self.fail(
                                dti,
                                c,
                                b,
                                None,
                                msg!(
                                    "duplicate unit-address (also used in node {})",
                                    &dti.nodes[a].fullpath
                                ),
                            );
                        }
                    }
                }
            }
            "obsolete_chosen_interrupt_controller" => {
                if n != dti.root {
                    return;
                }
                if let Some(chosen) = dti.node_by_path(b"/chosen") {
                    if let Some(p) = dti.property(chosen, b"interrupt-controller") {
                        self.fail_owner(
                            dti,
                            c,
                            n,
                            Some((chosen, p)),
                            b"/chosen has obsolete \"interrupt-controller\" property",
                        );
                    }
                }
            }
            "chosen_node_is_root" => {
                if node.name == b"chosen" && node.parent != Some(dti.root) {
                    self.fail(dti, c, n, None, "chosen node must be at root node");
                }
            }
            "chosen_node_bootargs" => {
                if node.name == b"chosen" {
                    self.string(dti, c, n, b"bootargs", false);
                }
            }
            "chosen_node_stdout_path" => {
                if node.name != b"chosen" {
                    return;
                }
                let name = if dti.property(n, b"stdout-path").is_some() {
                    b"stdout-path".as_slice()
                } else if let Some(p) = dti.property(n, b"linux,stdout-path") {
                    self.fail(dti, c, n, Some(p), "Use 'stdout-path' instead");
                    b"linux,stdout-path"
                } else {
                    return;
                };
                self.string(dti, c, n, name, false);
            }
            _ => unreachable!(),
        }
    }

    fn bus(&mut self, dti: &mut DtInfo, c: usize, n: NodeId, check: &str) {
        match check {
            "pci_bridge" => {
                if !prop(dti, n, b"device_type").is_some_and(|p| c_string(&p.data.bytes) == b"pci")
                {
                    return;
                }
                dti.nodes[n].bus = Some("PCI");
                let node = &dti.nodes[n];
                if base_name(node) != b"pci" && base_name(node) != b"pcie" {
                    self.fail(dti, c, n, None, "node name is not \"pci\" or \"pcie\"");
                }
                if dti.property(n, b"ranges").is_none() {
                    self.fail(
                        dti,
                        c,
                        n,
                        None,
                        "missing ranges for PCI bridge (or not a bridge)",
                    );
                }
                if addr_cells(node) != 3 {
                    self.fail(dti, c, n, None, "incorrect #address-cells for PCI bridge");
                }
                if size_cells(node) != 2 {
                    self.fail(dti, c, n, None, "incorrect #size-cells for PCI bridge");
                }
                if let Some(p) = dti.property(n, b"bus-range") {
                    let property = &node.properties[p];
                    if property.data.bytes.len() != 8 {
                        self.fail(dti, c, n, Some(p), "value must be 2 cells");
                        return;
                    }
                    if cell(property, 0) > cell(property, 1) {
                        self.fail(
                            dti,
                            c,
                            n,
                            Some(p),
                            "1st cell must be less than or equal to 2nd cell",
                        );
                    }
                    if cell(property, 1).unwrap_or(0) > 255 {
                        self.fail(
                            dti,
                            c,
                            n,
                            Some(p),
                            "maximum bus number must be less than 256",
                        );
                    }
                }
            }
            "simple_bus_bridge" => {
                if compatible(dti, n, b"simple-bus") {
                    dti.nodes[n].bus = Some("simple-bus");
                }
            }
            "i2c_bus_bridge" => {
                let name = base_name(&dti.nodes[n]);
                if name != b"i2c-bus" && name != b"i2c-arb" {
                    if name != b"i2c"
                        || dti.nodes[n].children.iter().any(|&child| {
                            !dti.nodes[child].deleted && base_name(&dti.nodes[child]) == b"i2c-bus"
                        })
                    {
                        return;
                    }
                }
                dti.nodes[n].bus = Some("i2c-bus");
                let node = &dti.nodes[n];
                if node.children.is_empty() {
                    return;
                }
                if addr_cells(node) != 1 {
                    self.fail(dti, c, n, None, "incorrect #address-cells for I2C bus");
                }
                if size_cells(node) != 0 {
                    self.fail(dti, c, n, None, "incorrect #size-cells for I2C bus");
                }
            }
            "spi_bus_bridge" => {
                if base_name(&dti.nodes[n]) == b"spi" {
                    dti.nodes[n].bus = Some("spi-bus");
                } else {
                    if addr_cells(&dti.nodes[n]) != 1 || size_cells(&dti.nodes[n]) != 0 {
                        return;
                    }
                    let spi = dti.nodes[n]
                        .children
                        .iter()
                        .filter(|&&child| !dti.nodes[child].deleted)
                        .any(|&child| {
                            dti.nodes[child]
                                .properties
                                .iter()
                                .any(|p| !p.deleted && p.name.starts_with(b"spi-"))
                        });
                    if spi {
                        dti.nodes[n].bus = Some("spi-bus");
                    }
                    if dti.nodes[n].bus == Some("spi-bus") && dti.property(n, b"reg").is_some() {
                        self.fail(dti, c, n, None, "node name for SPI buses should be 'spi'");
                    }
                }
                let node = &dti.nodes[n];
                if node.bus != Some("spi-bus") || node.children.is_empty() {
                    return;
                }
                let expected = if dti.property(n, b"spi-slave").is_some() {
                    0
                } else {
                    1
                };
                if addr_cells(node) != expected {
                    self.fail(dti, c, n, None, "incorrect #address-cells for SPI bus");
                }
                if size_cells(node) != 0 {
                    self.fail(dti, c, n, None, "incorrect #size-cells for SPI bus");
                }
            }
            "unit_address_format" => {
                let node = &dti.nodes[n];
                if node.parent.is_some_and(|p| dti.nodes[p].bus.is_some()) {
                    return;
                }
                let mut unit = unit_name(node);
                if unit.starts_with(b"0x") {
                    self.fail(dti, c, n, None, "unit name should not have leading \"0x\"");
                    unit = &unit[2..];
                }
                if unit.first() == Some(&b'0') && unit.get(1).is_some_and(u8::is_ascii_hexdigit) {
                    self.fail(dti, c, n, None, "unit name should not have leading 0s");
                }
            }
            _ => self.bus_reg(dti, c, n, check),
        }
    }

    fn bus_reg(&mut self, dti: &DtInfo, c: usize, n: NodeId, check: &str) {
        let node = &dti.nodes[n];
        let Some(parent_id) = node.parent else {
            return;
        };
        let parent = &dti.nodes[parent_id];
        let expected_bus = match check {
            "pci_device_reg" | "pci_device_bus_num" => "PCI",
            "simple_bus_reg" => "simple-bus",
            "i2c_bus_reg" => "i2c-bus",
            _ => "spi-bus",
        };
        if parent.bus != Some(expected_bus) {
            return;
        }
        let register = dti.property(n, b"reg");
        if check == "simple_bus_reg" {
            let values = if let Some(p) = register {
                Some((&node.properties[p], 0usize))
            } else {
                prop(dti, n, b"ranges")
                    .filter(|p| !p.data.bytes.is_empty())
                    .map(|p| (p, addr_cells(node).max(0) as usize))
            };
            let Some((property, start)) = values else {
                if parent.parent.is_some() && node.bus != Some("simple-bus") {
                    self.fail(dti, c, n, None, "missing or empty reg/ranges property");
                }
                return;
            };
            let mut value = 0u64;
            for index in 0..addr_cells(parent).max(0) as usize {
                let Some(word) = cell(property, start + index) else {
                    self.fail(
                        dti,
                        c,
                        n,
                        register,
                        "reg/ranges property is too short for its address cells",
                    );
                    return;
                };
                value = (value << 32) | u64::from(word);
            }
            let expected = format!("{value:x}");
            if unit_name(node) != expected.as_bytes() {
                self.fail(
                    dti,
                    c,
                    n,
                    None,
                    msg!(
                        "simple-bus unit address format error, expected \"{}\"",
                        expected.as_bytes()
                    ),
                );
            }
            return;
        }
        if check == "spi_bus_reg" && dti.property(parent_id, b"spi-slave").is_some() {
            return;
        }
        let Some(p) = register else {
            if expected_bus != "PCI" {
                self.fail(dti, c, n, None, "missing or empty reg property");
            }
            return;
        };
        let property = &node.properties[p];
        let Some(value) = cell(property, 0) else {
            self.fail(dti, c, n, None, "missing or empty reg property");
            return;
        };
        match check {
            "pci_device_bus_num" => {
                let bus = (value & 0x00ff0000) >> 16;
                let range = dti.property(parent_id, b"bus-range");
                let (min, max) = range
                    .map(|r| {
                        (
                            cell(&parent.properties[r], 0).unwrap_or(0),
                            cell(&parent.properties[r], 1).unwrap_or(0),
                        )
                    })
                    .unwrap_or((0, 0));
                if bus < min || bus > max {
                    self.fail_owner(
                        dti,
                        c,
                        n,
                        range.map(|r| (parent_id, r)),
                        format!("PCI bus number {bus} out of range, expected ({min} - {max})")
                            .as_bytes(),
                    );
                }
            }
            "pci_device_reg" => {
                if cell(property, 1).unwrap_or(0) != 0 || cell(property, 2).unwrap_or(0) != 0 {
                    self.fail(
                        dti,
                        c,
                        n,
                        Some(p),
                        "PCI reg config space address cells 2 and 3 must be 0",
                    );
                }
                if value & 0xff000000 != 0 {
                    self.fail(
                        dti,
                        c,
                        n,
                        Some(p),
                        "PCI reg address is not configuration space",
                    );
                }
                if value & 0xff != 0 {
                    self.fail(
                        dti,
                        c,
                        n,
                        Some(p),
                        "PCI reg config space address register number must be 0",
                    );
                }
                let device = (value & 0xf800) >> 11;
                let function = (value & 0x700) >> 8;
                if function == 0 && unit_name(node) == format!("{device:x}").as_bytes() {
                    return;
                }
                let expected = format!("{device:x},{function:x}");
                if unit_name(node) != expected.as_bytes() {
                    self.fail(
                        dti,
                        c,
                        n,
                        None,
                        msg!(
                            "PCI unit address format error, expected \"{}\"",
                            expected.as_bytes()
                        ),
                    );
                }
            }
            "i2c_bus_reg" => {
                let expected = format!("{:x}", value & !(1 << 30));
                if unit_name(node) != expected.as_bytes() {
                    self.fail(
                        dti,
                        c,
                        n,
                        None,
                        msg!(
                            "I2C bus unit address format error, expected \"{}\"",
                            expected.as_bytes()
                        ),
                    );
                }
                for bytes in property.data.bytes.chunks_exact(4) {
                    let value =
                        u32::from_be_bytes(bytes.try_into().expect("one cell")) & !(1 << 30);
                    if value & (1 << 31) != 0 {
                        if value & !(1 << 31) > 0x3ff {
                            self.fail(
                                dti,
                                c,
                                n,
                                Some(p),
                                format!(
                                    "I2C address must be less than 10-bits, got \"0x{value:x}\""
                                ),
                            );
                        }
                    } else if value > 0x7f {
                        self.fail(dti,c,n,Some(p),format!("I2C address must be less than 7-bits, got \"0x{value:x}\". Set I2C_TEN_BIT_ADDRESS for 10 bit addresses or fix the property"));
                    }
                }
            }
            "spi_bus_reg" => {
                let expected = format!("{value:x}");
                if unit_name(node) != expected.as_bytes() {
                    self.fail(
                        dti,
                        c,
                        n,
                        None,
                        msg!(
                            "SPI bus unit address format error, expected \"{}\"",
                            expected.as_bytes()
                        ),
                    );
                }
            }
            _ => unreachable!(),
        }
    }

    fn configure(&mut self, index: usize, warn: bool, error: bool, enable: bool) {
        let c = &self.checks[index];
        let change = if enable {
            (warn && !c.warn) || (error && !c.error)
        } else {
            (warn && c.warn) || (error && c.error)
        };
        if change {
            let related: Vec<usize> = if enable {
                c.prerequisites
                    .iter()
                    .map(|name| self.index(name))
                    .collect()
            } else {
                self.checks
                    .iter()
                    .enumerate()
                    .filter_map(|(i, other)| other.prerequisites.contains(&c.name).then_some(i))
                    .collect()
            };
            for next in related {
                self.configure(next, warn, error, enable);
            }
        }
        let c = &mut self.checks[index];
        if warn {
            c.warn = enable;
        }
        if error {
            c.error = enable;
        }
    }

    fn phandle_args(
        &mut self,
        dti: &DtInfo,
        c: usize,
        n: NodeId,
        p: PropId,
        cell_name: &[u8],
        optional: bool,
    ) {
        let property = &dti.nodes[n].properties[p];
        let length = property.data.bytes.len();
        if length % 4 != 0 {
            self.fail(
                dti,
                c,
                n,
                Some(p),
                format!("property size ({length}) is invalid, expected multiple of 4"),
            );
            return;
        }
        let mut index = 0usize;
        while index < length / 4 {
            let handle = cell(property, index).unwrap_or(0);
            if !phandle_is_valid(handle) {
                if dti.dtsflags & DTSF_PLUGIN != 0 {
                    break;
                }
                index += 1;
                continue;
            }
            if !property.data.markers.is_empty()
                && !property
                    .data
                    .markers
                    .iter()
                    .any(|m| m.kind == MarkerKind::RefPhandle && m.offset == index * 4)
            {
                self.fail(
                    dti,
                    c,
                    n,
                    Some(p),
                    format!("cell {index} is not a phandle reference"),
                );
            }
            let Some(provider) = dti.node_by_phandle(handle) else {
                self.fail(
                    dti,
                    c,
                    n,
                    Some(p),
                    format!("Could not get phandle node for (cell {index})"),
                );
                break;
            };
            let cells = match prop(dti, provider, cell_name).and_then(|p| cell(p, 0)) {
                Some(value) => value,
                None if optional => 0,
                None => {
                    self.fail(dti,c,n,None,msg!("Missing property '{}' in node {} or bad phandle (referred from {}[{}])",cell_name,&dti.nodes[provider].fullpath,&property.name,index.to_string().as_bytes()));
                    break;
                }
            };
            let expected = (index as u64 + u64::from(cells) + 1) * 4;
            if expected > length as u64 || expected <= index as u64 {
                self.fail(
                    dti,
                    c,
                    n,
                    Some(p),
                    format!("property size ({length}) too small for cell size {cells}"),
                );
                break;
            }
            index += cells as usize + 1;
        }
    }

    fn irq_provider(dti: &DtInfo, n: NodeId) -> bool {
        dti.property(n, b"interrupt-controller").is_some()
            || dti.property(n, b"interrupt-map").is_some()
    }

    fn interrupt_gpio(&mut self, dti: &DtInfo, c: usize, n: NodeId, check: &str) {
        let node = &dti.nodes[n];
        match check {
            "gpios_property" => {
                if dti.property(n, b"gpio-hog").is_some() {
                    return;
                }
                for (p, property) in node
                    .properties
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| !p.deleted)
                {
                    if gpio(&property.name) {
                        self.phandle_args(dti, c, n, p, b"#gpio-cells", false);
                    }
                }
            }
            "deprecated_gpio_property" => {
                for (p, property) in node
                    .properties
                    .iter()
                    .enumerate()
                    .filter(|(_, p)| !p.deleted)
                {
                    if gpio(&property.name) && property.name.ends_with(b"gpio") {
                        self.fail(
                            dti,
                            c,
                            n,
                            Some(p),
                            "'[*-]gpio' is deprecated, use '[*-]gpios' instead",
                        );
                    }
                }
            }
            "interrupt_provider" => {
                let provider = Self::irq_provider(dti, n);
                let cells = dti.property(n, b"#interrupt-cells").is_some();
                if provider && !cells {
                    self.fail(
                        dti,
                        c,
                        n,
                        None,
                        "Missing '#interrupt-cells' in interrupt provider",
                    );
                }
                if !provider && cells {
                    self.fail(
                        dti,
                        c,
                        n,
                        None,
                        "'#interrupt-cells' found, but node is not an interrupt provider",
                    );
                }
            }
            "interrupts_property" => self.interrupts(dti, c, n),
            "interrupt_map" => self.interrupt_map(dti, c, n),
            _ => unreachable!(),
        }
    }

    fn interrupts(&mut self, dti: &DtInfo, c: usize, n: NodeId) {
        let Some(p) = dti.property(n, b"interrupts") else {
            return;
        };
        let length = dti.nodes[n].properties[p].data.bytes.len();
        if length % 4 != 0 {
            self.fail(
                dti,
                c,
                n,
                Some(p),
                format!("size ({length}) is invalid, expected multiple of 4"),
            );
        }
        let mut parent = Some(n);
        let mut irq_node = None;
        while let Some(current) = parent {
            if current != n && Self::irq_provider(dti, current) {
                irq_node = Some(current);
                break;
            }
            if let Some(pp) = dti.property(current, b"interrupt-parent") {
                let handle = cell(&dti.nodes[current].properties[pp], 0).unwrap_or(0);
                if !phandle_is_valid(handle) {
                    if dti.dtsflags & DTSF_PLUGIN != 0 {
                        return;
                    }
                    self.fail(dti, c, current, Some(pp), "Invalid phandle");
                    break;
                }
                let Some(target) = dti.node_by_phandle(handle) else {
                    self.fail(dti, c, current, Some(pp), "Bad phandle");
                    return;
                };
                if !Self::irq_provider(dti, target) {
                    self.fail(
                        dti,
                        c,
                        target,
                        None,
                        "Missing interrupt-controller or interrupt-map property",
                    );
                }
                irq_node = Some(target);
                break;
            }
            parent = dti.nodes[current].parent;
        }
        let Some(irq_node) = irq_node else {
            self.fail(dti, c, n, None, "Missing interrupt-parent");
            return;
        };
        let Some(cp) = dti.property(irq_node, b"#interrupt-cells") else {
            return;
        };
        let cells = cell(&dti.nodes[irq_node].properties[cp], 0).unwrap_or(0);
        let size = u64::from(cells) * 4;
        if !multiple(length, size as i64) {
            self.fail_owner(
                dti,
                c,
                n,
                Some((irq_node, cp)),
                format!("size is ({length}), expected multiple of {}", size as i32).as_bytes(),
            );
        }
    }

    fn interrupt_map(&mut self, dti: &DtInfo, c: usize, n: NodeId) {
        let Some(p) = dti.property(n, b"interrupt-map") else {
            return;
        };
        let node = &dti.nodes[n];
        if node.addr_cells < 0 {
            self.fail(
                dti,
                c,
                n,
                None,
                "Missing '#address-cells' in interrupt-map provider",
            );
            return;
        }
        let irqcells = prop(dti, n, b"#interrupt-cells")
            .and_then(|p| cell(p, 0))
            .unwrap_or(0);
        let size = addr_cells(node) as u64 + u64::from(irqcells);
        if let Some(mask) = dti.property(n, b"interrupt-map-mask") {
            let length = node.properties[mask].data.bytes.len();
            if length as u64 != size * 4 {
                self.fail(
                    dti,
                    c,
                    n,
                    Some(mask),
                    format!("property size ({length}) is invalid, expected {}", size * 4),
                );
            }
        }
        let map = &node.properties[p];
        let length = map.data.bytes.len();
        if length % 4 != 0 {
            self.fail(
                dti,
                c,
                n,
                Some(p),
                format!("property size ({length}) is invalid, expected multiple of 4"),
            );
            return;
        }
        let map_cells = (length / 4) as u64;
        let mut index = 0u64;
        while index < map_cells {
            if index + size >= map_cells {
                self.fail(
                    dti,
                    c,
                    n,
                    Some(p),
                    format!(
                        "property size ({length}) too small, expected > {}",
                        (index + size) * 4
                    ),
                );
                break;
            }
            index += size;
            let handle = cell(map, index as usize).unwrap_or(0);
            if !phandle_is_valid(handle) {
                if dti.dtsflags & DTSF_PLUGIN == 0 {
                    self.fail(
                        dti,
                        c,
                        n,
                        Some(p),
                        format!("Cell {index} is not a phandle({})", handle as i32),
                    );
                }
                break;
            }
            let Some(provider) = dti.node_by_phandle(handle) else {
                self.fail(
                    dti,
                    c,
                    n,
                    Some(p),
                    format!(
                        "Could not get phandle({}) node for (cell {index})",
                        handle as i32
                    ),
                );
                break;
            };
            let Some(count) = prop(dti, provider, b"#interrupt-cells").and_then(|p| cell(p, 0))
            else {
                self.fail(dti,c,n,None,msg!("Missing property '#interrupt-cells' in node {} or bad phandle (referred from interrupt-map[{}])",&dti.nodes[provider].fullpath,index.to_string().as_bytes()));
                break;
            };
            let mut parent_size = u64::from(count);
            if let Some(address) = prop(dti, provider, b"#address-cells").and_then(|p| cell(p, 0)) {
                parent_size += u64::from(address);
            } else {
                self.fail(
                    dti,
                    c,
                    n,
                    Some(p),
                    msg!(
                        "Missing property '#address-cells' in node {}, using 0 as fallback",
                        &dti.nodes[provider].fullpath
                    ),
                );
            }
            index += 1 + parent_size;
            if index > map_cells {
                self.fail(
                    dti,
                    c,
                    n,
                    Some(p),
                    format!("property size ({length}) mismatch, expected {}", index * 4),
                );
            }
        }
    }

    fn graph_reg(&mut self, dti: &DtInfo, c: usize, n: NodeId) {
        let Some(p) = dti.property(n, b"reg") else {
            return;
        };
        let node = &dti.nodes[n];
        let property = &node.properties[p];
        if property.data.bytes.len() != 4 {
            self.fail(dti, c, n, None, "graph node malformed 'reg' property");
            return;
        }
        let expected = format!("{:x}", cell(property, 0).unwrap_or(0));
        if unit_name(node) != expected.as_bytes() {
            self.fail(
                dti,
                c,
                n,
                None,
                msg!(
                    "graph node unit address error, expected \"{}\"",
                    expected.as_bytes()
                ),
            );
        }
        if let Some(parent) = node.parent.map(|p| &dti.nodes[p]) {
            if parent.addr_cells != 1 {
                self.fail(
                    dti,
                    c,
                    n,
                    dti.property(n, b"#address-cells"),
                    format!(
                        "graph node '#address-cells' is {}, must be 1",
                        parent.addr_cells
                    ),
                );
            }
            if parent.size_cells != 0 {
                self.fail(
                    dti,
                    c,
                    n,
                    dti.property(n, b"#size-cells"),
                    format!(
                        "graph node '#size-cells' is {}, must be 0",
                        parent.size_cells
                    ),
                );
            }
        }
    }

    fn remote_endpoint(&mut self, dti: &DtInfo, c: usize, n: NodeId) -> Option<NodeId> {
        let p = dti.property(n, b"remote-endpoint")?;
        let handle = cell(&dti.nodes[n].properties[p], 0).unwrap_or(0);
        if !phandle_is_valid(handle) {
            return None;
        }
        let node = dti.node_by_phandle(handle);
        if node.is_none() {
            self.fail(dti, c, n, Some(p), "graph phandle is not valid");
        }
        node
    }

    fn graph(&mut self, dti: &mut DtInfo, c: usize, n: NodeId, check: &str) {
        match check {
            "graph_nodes" => {
                for child in dti.nodes[n].children.clone() {
                    if dti.nodes[child].deleted
                        || !(base_name(&dti.nodes[child]) == b"endpoint"
                            || dti.property(child, b"remote-endpoint").is_some())
                    {
                        continue;
                    }
                    let Some(parent) = dti.nodes[n].parent else {
                        self.fail(dti,c,n,None,msg!("root node contains endpoint node '{}', potentially misplaced remote-endpoint property",&dti.nodes[child].name));
                        continue;
                    };
                    dti.nodes[n].bus = Some("graph-port");
                    if dti.nodes[parent].bus.is_none()
                        && (dti.nodes[parent].name == b"ports" || dti.property(n, b"reg").is_some())
                    {
                        dti.nodes[parent].bus = Some("graph-ports");
                    }
                    break;
                }
            }
            "graph_port" => {
                if dti.nodes[n].bus != Some("graph-port") {
                    return;
                }
                self.graph_reg(dti, c, n);
                if dti.dtsflags & DTSF_PLUGIN == 0 && base_name(&dti.nodes[n]) != b"port" {
                    self.fail(dti, c, n, None, "graph port node name should be 'port'");
                }
            }
            "graph_endpoint" => {
                if !dti.nodes[n]
                    .parent
                    .is_some_and(|p| dti.nodes[p].bus == Some("graph-port"))
                {
                    return;
                }
                self.graph_reg(dti, c, n);
                if dti.dtsflags & DTSF_PLUGIN != 0 {
                    return;
                }
                if base_name(&dti.nodes[n]) != b"endpoint" {
                    self.fail(
                        dti,
                        c,
                        n,
                        None,
                        "graph endpoint node name should be 'endpoint'",
                    );
                }
                if let Some(remote) = self.remote_endpoint(dti, c, n) {
                    if self.remote_endpoint(dti, c, remote) != Some(n) {
                        self.fail(
                            dti,
                            c,
                            n,
                            None,
                            msg!(
                                "graph connection to node '{}' is not bidirectional",
                                &dti.nodes[remote].fullpath
                            ),
                        );
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn options(&mut self) -> Result<(), i32> {
        for (warn, error, arg) in &self.options.checks {
            let disable = arg.starts_with(b"no-") || arg.starts_with(b"no_");
            let name = if disable { &arg[3..] } else { arg.as_slice() };
            let Some(index) = self.checks.iter().position(|c| c.name.as_bytes() == name) else {
                self.diagnostics
                    .raw(msg!("FATAL ERROR: Unrecognized check name \"{}\"\n", name));
                return Err(1);
            };
            self.configure(index, *warn, *error, !disable);
        }
        Ok(())
    }

    fn message(
        &mut self,
        dti: &DtInfo,
        check: usize,
        node: Option<NodeId>,
        property: Option<(NodeId, PropId)>,
        message: &[u8],
    ) {
        let c = &self.checks[check];
        if !(c.warn && self.options.quiet < 1 || c.error && self.options.quiet < 2) {
            return;
        }
        let p = property.map(|(owner, index)| &dti.nodes[owner].properties[index]);
        let n = node.map(|id| &dti.nodes[id]);
        let positions = p
            .filter(|p| !p.srcpos.is_empty())
            .map(|p| &p.srcpos)
            .or_else(|| n.map(|n| &n.srcpos));
        if let Some(pos) = positions.and_then(|p| p.first()) {
            self.diagnostics.raw(pos.render());
        } else if dti.outname == b"-" {
            self.diagnostics.raw(b"<stdout>");
        } else {
            self.diagnostics.raw(&dti.outname);
        }
        self.diagnostics.raw(if c.error {
            b": ERROR (".as_slice()
        } else {
            b": Warning (".as_slice()
        });
        self.diagnostics.raw(c.name);
        self.diagnostics.raw(b"): ");
        if let Some(n) = n {
            self.diagnostics.raw(&n.fullpath);
            if let Some(p) = p {
                self.diagnostics.raw(b":");
                self.diagnostics.raw(&p.name);
            }
            self.diagnostics.raw(b": ");
        }
        self.diagnostics.raw(message);
        self.diagnostics.raw(b"\n");
        if p.is_none() {
            if let Some(n) = n {
                for pos in n.srcpos.iter().skip(1) {
                    self.diagnostics.raw(b"  also defined at ");
                    self.diagnostics.raw(pos.render());
                    self.diagnostics.raw(b"\n");
                }
            }
        }
    }

    fn fail(
        &mut self,
        dti: &DtInfo,
        check: usize,
        node: NodeId,
        property: Option<PropId>,
        message: impl AsRef<[u8]>,
    ) {
        self.fail_owner(
            dti,
            check,
            node,
            property.map(|p| (node, p)),
            message.as_ref(),
        );
    }

    fn fail_owner(
        &mut self,
        dti: &DtInfo,
        check: usize,
        node: NodeId,
        property: Option<(NodeId, PropId)>,
        message: &[u8],
    ) {
        self.checks[check].status = Status::Failed;
        self.message(dti, check, Some(node), property, message);
    }

    fn walk(&mut self, dti: &mut DtInfo, check: usize, node: NodeId) {
        self.node(dti, check, node);
        for child in dti.nodes[node].children.clone() {
            if !dti.nodes[child].deleted {
                self.walk(dti, check, child);
            }
        }
    }

    fn run(&mut self, dti: &mut DtInfo, check: usize) -> bool {
        let mut error = false;
        if self.checks[check].status == Status::Unchecked {
            for prerequisite in self.checks[check].prerequisites.clone() {
                let index = self.index(prerequisite);
                error = error || self.run(dti, index);
                if self.checks[index].status != Status::Passed {
                    self.checks[check].status = Status::Prerequisite;
                    self.message(
                        dti,
                        check,
                        None,
                        None,
                        &msg!("Failed prerequisite '{}'", prerequisite.as_bytes()),
                    );
                }
            }
            if self.checks[check].status == Status::Unchecked {
                self.walk(dti, check, dti.root);
                if self.checks[check].status == Status::Unchecked {
                    self.checks[check].status = Status::Passed;
                }
            }
        }
        error || self.checks[check].status != Status::Passed && self.checks[check].error
    }
}

pub(crate) fn validate_check_options(
    options: &Options,
    diagnostics: &mut Diagnostics,
) -> Result<(), i32> {
    Checker {
        options,
        diagnostics,
        checks: definitions(),
    }
    .options()
}

pub(crate) fn process_checks(
    dti: &mut DtInfo,
    options: &Options,
    diagnostics: &mut Diagnostics,
) -> Result<(), i32> {
    let mut checker = Checker {
        options,
        diagnostics,
        checks: definitions(),
    };
    checker.options()?;
    let mut error = false;
    for index in 0..checker.checks.len() {
        if checker.checks[index].warn || checker.checks[index].error {
            error = error || checker.run(dti, index);
        }
    }
    if error {
        if !options.force {
            checker
                .diagnostics
                .raw(b"ERROR: Input tree has errors, aborting (use -f to force output)\n");
            return Err(2);
        }
        if options.quiet < 3 {
            checker
                .diagnostics
                .raw(b"Warning: Input tree has errors, output forced\n");
        }
    }
    Ok(())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
