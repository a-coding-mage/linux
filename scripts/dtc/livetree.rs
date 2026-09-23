// SPDX-License-Identifier: GPL-2.0-or-later
/* (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation. 2005. */

use crate::dtc_header::*;
use crate::srcpos::join_path;

pub(crate) fn add_label(labels: &mut Vec<Label>, name: Vec<u8>) {
    if let Some(label) = labels.iter_mut().find(|label| label.name == name) {
        label.deleted = false;
    } else {
        labels.insert(
            0,
            Label {
                name,
                deleted: false,
            },
        );
    }
}

impl Property {
    pub(crate) fn new(name: Vec<u8>, data: Data, srcpos: Vec<SourcePos>) -> Self {
        Self {
            name,
            data,
            srcpos,
            ..Self::default()
        }
    }

    pub(crate) fn delete(&mut self) {
        self.deleted = true;
        for label in &mut self.labels {
            label.deleted = true;
        }
    }

    pub(crate) fn cell(&self, index: usize) -> u32 {
        u32::from_be_bytes(
            self.data.bytes[index * 4..index * 4 + 4]
                .try_into()
                .expect("cell"),
        )
    }
}

impl Node {
    #[allow(dead_code)] // Canonical node helper, also available to compiler clients.
    pub(crate) fn unitname(&self) -> &[u8] {
        if self.basenamelen < self.name.len() {
            &self.name[self.basenamelen + 1..]
        } else {
            b""
        }
    }
}

impl DtInfo {
    pub(crate) fn add_node(&mut self, node: Node) -> NodeId {
        let id = self.nodes.len();
        for &child in &node.children {
            self.nodes[child].parent = Some(id);
        }
        self.nodes.push(node);
        id
    }

    pub(crate) fn add_child(&mut self, parent: NodeId, child: NodeId) {
        self.nodes[child].parent = Some(parent);
        self.nodes[parent].children.push(child);
    }

    pub(crate) fn property(&self, node: NodeId, name: &[u8]) -> Option<PropId> {
        self.nodes[node]
            .properties
            .iter()
            .position(|p| !p.deleted && p.name == name)
    }

    pub(crate) fn subnode(&self, node: NodeId, name: &[u8]) -> Option<NodeId> {
        self.nodes[node]
            .children
            .iter()
            .copied()
            .find(|&id| !self.nodes[id].deleted && self.nodes[id].name == name)
    }

    pub(crate) fn active_nodes(&self) -> Vec<NodeId> {
        self.subtree(self.root)
    }

    pub(crate) fn subtree(&self, root: NodeId) -> Vec<NodeId> {
        let mut result = Vec::new();
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            result.push(node);
            stack.extend(
                self.nodes[node]
                    .children
                    .iter()
                    .rev()
                    .copied()
                    .filter(|&id| !self.nodes[id].deleted),
            );
        }
        result
    }

    pub(crate) fn node_by_path_from(&self, mut node: NodeId, mut path: &[u8]) -> Option<NodeId> {
        loop {
            if path.is_empty() {
                return (!self.nodes[node].deleted).then_some(node);
            }
            while path.starts_with(b"/") {
                path = &path[1..];
            }
            match path.iter().position(|&x| x == b'/') {
                Some(slash) => {
                    node = self.subnode(node, &path[..slash])?;
                    path = &path[slash + 1..];
                }
                None => return self.subnode(node, path),
            }
        }
    }

    pub(crate) fn node_by_path(&self, path: &[u8]) -> Option<NodeId> {
        self.node_by_path_from(self.root, path)
    }

    pub(crate) fn node_by_label(&self, label: &[u8]) -> Option<NodeId> {
        self.active_nodes().into_iter().find(|&id| {
            self.nodes[id]
                .labels
                .iter()
                .any(|l| !l.deleted && l.name == label)
        })
    }

    pub(crate) fn node_by_phandle(&self, phandle: u32) -> Option<NodeId> {
        if !phandle_is_valid(phandle) {
            return None;
        }
        self.active_nodes()
            .into_iter()
            .find(|&id| !self.nodes[id].deleted && self.nodes[id].phandle == phandle)
    }

    pub(crate) fn node_by_ref(&self, reference: &[u8]) -> Option<NodeId> {
        if reference == b"/" {
            return Some(self.root);
        }
        if reference.starts_with(b"/") {
            return self.node_by_path(reference);
        }
        if let Some(slash) = reference.iter().position(|&b| b == b'/') {
            self.node_by_path_from(
                self.node_by_label(&reference[..slash])?,
                &reference[slash + 1..],
            )
        } else {
            self.node_by_label(reference)
        }
    }

    pub(crate) fn property_by_label(&self, label: &[u8]) -> Option<(NodeId, PropId)> {
        for node in self.active_nodes() {
            for (index, property) in self.nodes[node]
                .properties
                .iter()
                .enumerate()
                .filter(|(_, p)| !p.deleted)
            {
                if property
                    .labels
                    .iter()
                    .any(|l| !l.deleted && l.name == label)
                {
                    return Some((node, index));
                }
            }
        }
        None
    }

    pub(crate) fn marker_label(&self, label: &[u8]) -> Option<(NodeId, PropId, usize)> {
        for node in self.active_nodes() {
            for (index, property) in self.nodes[node]
                .properties
                .iter()
                .enumerate()
                .filter(|(_, p)| !p.deleted)
            {
                if let Some(marker) = property.data.markers.iter().position(|m| {
                    m.kind == MarkerKind::Label && m.reference.as_deref() == Some(label)
                }) {
                    return Some((node, index, marker));
                }
            }
        }
        None
    }

    pub(crate) fn node_phandle(&mut self, node: NodeId, format: u32) -> u32 {
        if phandle_is_valid(self.nodes[node].phandle) {
            return self.nodes[node].phandle;
        }
        while self.node_by_phandle(self.next_phandle).is_some() {
            self.next_phandle = self.next_phandle.wrapping_add(1);
        }
        self.nodes[node].phandle = self.next_phandle;
        for (name, flag) in [
            (b"linux,phandle".as_slice(), PHANDLE_LEGACY),
            (b"phandle".as_slice(), PHANDLE_EPAPR),
        ] {
            if format & flag == 0 || self.property(node, name).is_some() {
                continue;
            }
            let mut data = Data::default();
            data.marker(MarkerKind::Uint32, None);
            data.append_integer(u64::from(self.next_phandle), 32);
            self.nodes[node]
                .properties
                .push(Property::new(name.to_vec(), data, Vec::new()));
        }
        self.next_phandle
    }

    pub(crate) fn guess_boot_cpuid(&self) -> u32 {
        let Some(cpus) = self.node_by_path(b"/cpus") else {
            return 0;
        };
        let Some(&cpu) = self.nodes[cpus].children.first() else {
            return 0;
        };
        let Some(reg) = self.property(cpu, b"reg") else {
            return 0;
        };
        let property = &self.nodes[cpu].properties[reg];
        if property.data.bytes.len() == 4 {
            property.cell(0)
        } else {
            0
        }
    }

    pub(crate) fn delete_node(&mut self, root: NodeId) {
        for id in self.subtree(root) {
            self.nodes[id].deleted = true;
            for property in &mut self.nodes[id].properties {
                if !property.deleted {
                    property.delete();
                }
            }
            for label in &mut self.nodes[id].labels {
                label.deleted = true;
            }
        }
    }

    pub(crate) fn merge_nodes(&mut self, old: NodeId, new: NodeId) {
        enum Work {
            Merge(NodeId, NodeId),
            Child(NodeId, NodeId),
            Finish(NodeId, NodeId),
        }
        let mut pending = vec![Work::Merge(old, new)];
        while let Some(work) = pending.pop() {
            let (old, new) = match work {
                Work::Finish(old, new) => {
                    let positions = std::mem::take(&mut self.nodes[new].srcpos);
                    self.nodes[old].srcpos.extend(positions);
                    self.nodes[new].deleted = true;
                    continue;
                }
                Work::Child(old, child) => {
                    self.nodes[child].parent = None;
                    let found = self.nodes[old]
                        .children
                        .iter()
                        .copied()
                        .find(|&id| self.nodes[id].name == self.nodes[child].name);
                    if self.nodes[child].deleted {
                        if let Some(target) = found {
                            self.delete_node(target);
                        }
                    } else if let Some(target) = found {
                        pending.push(Work::Merge(target, child));
                    } else {
                        self.add_child(old, child);
                    }
                    continue;
                }
                Work::Merge(old, new) => (old, new),
            };
            self.nodes[old].deleted = false;
            let labels = std::mem::take(&mut self.nodes[new].labels);
            for label in labels {
                add_label(&mut self.nodes[old].labels, label.name);
            }
            let properties = std::mem::take(&mut self.nodes[new].properties);
            for mut property in properties {
                let found = self.nodes[old]
                    .properties
                    .iter()
                    .position(|p| p.name == property.name);
                if property.deleted {
                    if let Some(index) = found {
                        self.nodes[old].properties[index].delete();
                    }
                } else if let Some(index) = found {
                    let target = &mut self.nodes[old].properties[index];
                    for label in std::mem::take(&mut property.labels) {
                        add_label(&mut target.labels, label.name);
                    }
                    target.data = property.data;
                    target.srcpos = property.srcpos;
                    target.deleted = false;
                } else {
                    self.nodes[old].properties.push(property);
                }
            }
            let children = std::mem::take(&mut self.nodes[new].children);
            pending.push(Work::Finish(old, new));
            for child in children.into_iter().rev() {
                pending.push(Work::Child(old, child));
            }
        }
    }

    pub(crate) fn orphan(&mut self, child: NodeId, reference: Vec<u8>) {
        let mut data = Data::default();
        let name = if reference.starts_with(b"/") {
            data.marker(MarkerKind::String, Some(reference.clone()));
            data.bytes = reference;
            data.bytes.push(0);
            b"target-path".as_slice()
        } else {
            data.marker(MarkerKind::RefPhandle, Some(reference));
            data.append_integer(u64::from(u32::MAX), 32);
            b"target".as_slice()
        };
        self.nodes[child].name = b"__overlay__".to_vec();
        let node = self.add_node(Node {
            name: format!("fragment@{}", self.next_orphan_fragment).into_bytes(),
            properties: vec![Property::new(name.to_vec(), data, Vec::new())],
            children: vec![child],
            ..Node::default()
        });
        self.next_orphan_fragment += 1;
        self.add_child(self.root, node);
    }

    pub(crate) fn fill_fullpaths(&mut self) {
        for id in self.active_nodes() {
            self.nodes[id].basenamelen = self.nodes[id]
                .name
                .iter()
                .position(|&b| b == b'@')
                .unwrap_or(self.nodes[id].name.len());
            let prefix = self.nodes[id]
                .parent
                .map(|p| self.nodes[p].fullpath.as_slice())
                .unwrap_or(b"");
            self.nodes[id].fullpath = join_path(prefix, &self.nodes[id].name);
        }
    }

    pub(crate) fn sort_tree(&mut self) {
        self.reserves.sort_by_key(|r| (r.address, r.size));
        for id in 0..self.nodes.len() {
            self.nodes[id]
                .properties
                .sort_by(|a, b| a.name.cmp(&b.name));
            let mut children = std::mem::take(&mut self.nodes[id].children);
            children.sort_by(|&a, &b| self.nodes[a].name.cmp(&self.nodes[b].name));
            self.nodes[id].children = children;
        }
    }

    fn named_child(&mut self, parent: NodeId, name: &[u8]) -> NodeId {
        if let Some(id) = self.subnode(parent, name) {
            return id;
        }
        let child = self.add_node(Node {
            name: name.to_vec(),
            ..Node::default()
        });
        self.add_child(parent, child);
        child
    }

    pub(crate) fn append_to_property(
        &mut self,
        node: NodeId,
        name: &[u8],
        bytes: &[u8],
        kind: MarkerKind,
    ) {
        let index = self.property(node, name).unwrap_or_else(|| {
            self.nodes[node].properties.push(Property::new(
                name.to_vec(),
                Data::default(),
                Vec::new(),
            ));
            self.nodes[node].properties.len() - 1
        });
        let data = &mut self.nodes[node].properties[index].data;
        data.marker(kind, Some(name.to_vec()));
        data.bytes.extend_from_slice(bytes);
    }

    fn append_unique(&mut self, node: NodeId, name: &[u8], bytes: &[u8], strings: bool) -> bool {
        if let Some(index) = self.property(node, name) {
            let old = &self.nodes[node].properties[index].data.bytes;
            if strings {
                if !old.is_empty() && old.last() != Some(&0) {
                    return false;
                }
                if old.split_inclusive(|&b| b == 0).any(|s| s == bytes) {
                    return true;
                }
            } else {
                if old.len() % 4 != 0 {
                    return false;
                }
                if old.chunks_exact(4).any(|s| s == bytes) {
                    return true;
                }
            }
        }
        self.append_to_property(
            node,
            name,
            bytes,
            if strings {
                MarkerKind::String
            } else {
                MarkerKind::Uint32
            },
        );
        true
    }

    pub(crate) fn generate_labels_from_tree(
        &mut self,
        name: &[u8],
        options: &Options,
        diagnostics: &mut Diagnostics,
    ) {
        let Some(id) = self.subnode(self.root, name) else {
            return;
        };
        for property in self.nodes[id]
            .properties
            .clone()
            .into_iter()
            .filter(|p| !p.deleted)
        {
            let path = property.data.bytes.split(|&b| b == 0).next().unwrap_or(b"");
            if let Some(target) = self.node_by_path(path) {
                add_label(&mut self.nodes[target].labels, property.name);
            } else if options.quiet < 1 {
                diagnostics.raw(format!(
                    "Warning: Path {} referenced in property {}/{} missing",
                    display(path),
                    display(name),
                    display(&property.name)
                ));
            }
        }
    }

    pub(crate) fn generate_label_tree(
        &mut self,
        name: &[u8],
        allocate: bool,
        options: &Options,
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Vec<u8>> {
        if !self
            .active_nodes()
            .iter()
            .any(|&id| !self.nodes[id].labels.is_empty())
        {
            return Ok(());
        }
        let target = self.named_child(self.root, name);
        for node in self.active_nodes() {
            if self.nodes[node].labels.is_empty() {
                continue;
            }
            for label in self.nodes[node]
                .labels
                .clone()
                .into_iter()
                .filter(|l| !l.deleted)
            {
                if self.property(target, &label.name).is_some() {
                    diagnostics.raw(format!(
                        "WARNING: label {} already exists in /{}",
                        display(&label.name),
                        display(name)
                    ));
                    continue;
                }
                let data = Data::escape_string(&self.nodes[node].fullpath)?;
                self.nodes[target]
                    .properties
                    .push(Property::new(label.name, data, Vec::new()));
            }
            if allocate {
                self.node_phandle(node, options.phandle_format);
            }
        }
        Ok(())
    }

    fn references(&self, local: bool) -> Vec<(NodeId, PropId, Marker)> {
        let mut result = Vec::new();
        for id in self.active_nodes() {
            for (index, property) in self.nodes[id]
                .properties
                .iter()
                .enumerate()
                .filter(|(_, p)| !p.deleted)
            {
                for marker in &property.data.markers {
                    if marker.kind == MarkerKind::RefPhandle
                        && self
                            .node_by_ref(marker.reference.as_deref().unwrap_or(b""))
                            .is_some()
                            == local
                    {
                        result.push((id, index, marker.clone()));
                    }
                }
            }
        }
        result
    }

    pub(crate) fn generate_fixups_tree(
        &mut self,
        name: &[u8],
        diagnostics: &mut Diagnostics,
    ) -> Result<(), Vec<u8>> {
        let entries = self.references(false);
        if entries.is_empty() {
            return Ok(());
        }
        let target = self.named_child(self.root, name);
        let mut valid = true;
        for (node, property, marker) in entries {
            let reference = marker.reference.as_deref().unwrap_or(b"");
            if reference.contains(&b'/') {
                return Err(format!(
                    "Can't generate fixup for reference to path &{{{}}}\n",
                    display(reference)
                )
                .into_bytes());
            }
            let property_name = &self.nodes[node].properties[property].name;
            if property_name.contains(&b':') || self.nodes[node].fullpath.contains(&b':') {
                return Err(b"arguments should not contain ':'\n".to_vec());
            }
            let mut entry = self.nodes[node].fullpath.clone();
            entry.push(b':');
            entry.extend_from_slice(property_name);
            entry.extend_from_slice(format!(":{}\0", marker.offset).as_bytes());
            valid &= self.append_unique(target, reference, &entry, true);
        }
        if !valid {
            diagnostics.raw(format!(
                "Warning: Preexisting data in {} malformed, some content could not be added.\n",
                display(name)
            ));
        }
        Ok(())
    }

    pub(crate) fn generate_local_fixups_tree(
        &mut self,
        name: &[u8],
        diagnostics: &mut Diagnostics,
    ) {
        let entries = self.references(true);
        if entries.is_empty() {
            return;
        }
        let root = self.named_child(self.root, name);
        let mut valid = true;
        for (node, property, marker) in entries {
            let mut path = Vec::new();
            let mut current = node;
            while let Some(parent) = self.nodes[current].parent {
                path.push(self.nodes[current].name.clone());
                current = parent;
            }
            let mut target = root;
            for component in path.iter().rev() {
                target = self.named_child(target, component);
            }
            let property_name = self.nodes[node].properties[property].name.clone();
            valid &= self.append_unique(
                target,
                &property_name,
                &(marker.offset as u32).to_be_bytes(),
                false,
            );
        }
        if !valid {
            diagnostics.raw(format!(
                "Warning: Preexisting data in {} malformed, some content could not be added.\n",
                display(name)
            ));
        }
    }

    pub(crate) fn fixup_phandles(
        &mut self,
        name: &[u8],
        options: &Options,
        diagnostics: &mut Diagnostics,
    ) {
        let Some(id) = self.subnode(self.root, name) else {
            return;
        };
        for property in self.nodes[id]
            .properties
            .clone()
            .into_iter()
            .filter(|p| !p.deleted)
        {
            let malformed = format!(
                "Warning: Malformed fixup entry for label {}\n",
                display(&property.name)
            );
            for entry in property.data.bytes.split_inclusive(|&b| b == 0) {
                if entry.last() != Some(&0) {
                    if options.quiet < 1 {
                        diagnostics.raw(&malformed);
                    }
                    break;
                }
                let entry = &entry[..entry.len() - 1];
                let mut fields = entry.splitn(3, |&b| b == b':');
                let path = fields.next().unwrap_or(b"");
                let (Some(propname), Some(offset)) = (fields.next(), fields.next()) else {
                    if options.quiet < 1 {
                        diagnostics.raw(&malformed);
                    }
                    continue;
                };
                let Some(node) = self.node_by_path(path) else {
                    if options.quiet < 1 {
                        diagnostics.raw(format!(
                            "Warning: Label {} references non-existing node {}\n",
                            display(&property.name),
                            display(path)
                        ));
                    }
                    continue;
                };
                let Some(index) = self.property(node, propname) else {
                    if options.quiet < 1 {
                        diagnostics.raw(format!(
                            "Warning: Label {} references non-existing property {} in node {}\n",
                            display(&property.name),
                            display(&self.nodes[node].fullpath),
                            display(propname)
                        ));
                    }
                    continue;
                };
                let offset = crate::util::strtol(offset) as i64;
                if offset < 0
                    || offset as usize
                        > self.nodes[node].properties[index]
                            .data
                            .bytes
                            .len()
                            .saturating_sub(4)
                    || self.nodes[node].properties[index].data.bytes.len() < 4
                {
                    if options.quiet < 1 {
                        diagnostics.raw(format!("Warning: Label {} contains invalid offset for property {} in node {}\n", display(&property.name), display(propname), display(&self.nodes[node].fullpath)));
                    }
                    continue;
                }
                crate::treesource::property_add_marker(
                    &mut self.nodes[node].properties[index],
                    MarkerKind::RefPhandle,
                    offset as usize,
                    Some(property.name.clone()),
                );
            }
        }
    }

    pub(crate) fn local_fixup_phandles(
        &mut self,
        name: &[u8],
        options: &Options,
        diagnostics: &mut Diagnostics,
    ) {
        let Some(root) = self.subnode(self.root, name) else {
            return;
        };
        let mut stack = vec![(root, self.root, false)];
        while let Some((fixup, mut node, child)) = stack.pop() {
            if child {
                if let Some(target) = self.subnode(node, &self.nodes[fixup].name) {
                    node = target;
                } else {
                    if options.quiet < 1 {
                        diagnostics.raw(format!(
                            "Warning: node {}/{} referenced in __local_fixups__ missing\n",
                            display(&self.nodes[fixup].name),
                            display(&self.nodes[node].fullpath)
                        ));
                    }
                    continue;
                }
            }
            for property in self.nodes[fixup]
                .properties
                .clone()
                .into_iter()
                .filter(|p| !p.deleted)
            {
                let Some(index) = self.property(node, &property.name) else {
                    if options.quiet < 1 {
                        diagnostics.raw(format!(
                            "Warning: Property {} in {} referenced in __local_fixups__ missing\n",
                            display(&property.name),
                            display(&self.nodes[node].fullpath)
                        ));
                    }
                    continue;
                };
                if property.data.bytes.len() % 4 != 0 {
                    if options.quiet < 1 {
                        diagnostics.raw(format!(
                            "Warning: property {} in /__local_fixups__{} malformed\n",
                            display(&property.name),
                            display(&self.nodes[node].fullpath)
                        ));
                    }
                    continue;
                }
                for bytes in property.data.bytes.chunks_exact(4) {
                    crate::treesource::add_phandle_marker(
                        self,
                        node,
                        index,
                        u32::from_be_bytes(bytes.try_into().unwrap()) as usize,
                        options,
                        diagnostics,
                    );
                }
            }
            for &child in self.nodes[fixup].children.iter().rev() {
                if self.nodes[child].deleted {
                    continue;
                }
                stack.push((child, node, true));
            }
        }
    }
}
