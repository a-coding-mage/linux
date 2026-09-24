// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2014 SGI.
//! Compressed, shared UTF-8 tries, using arena indices rather than owning pointers.

use super::model::{encode, hangul, is_hangul, Database, Record, HANGUL, LIMIT};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Child {
    Empty,
    Node(usize),
    Leaf(usize),
}

impl Child {
    fn node(self) -> Option<usize> {
        if let Self::Node(i) = self {
            Some(i)
        } else {
            None
        }
    }
    fn exists(self) -> bool {
        self != Self::Empty
    }
}

#[derive(Clone)]
struct Node {
    parent: Option<usize>,
    children: [Child; 2],
    bit: u8,
    next_byte: bool,
    key_bits: u32,
    key_mask: u32,
    marked: bool,
    index: usize,
    offset: Option<usize>,
    size: usize,
}

pub(super) struct Tree {
    root: Child,
    nodes: Vec<Node>,
    free: Vec<usize>,
    leaf_indices: Vec<usize>,
    next: Option<usize>,
    pub max_age: u32,
    pub casefold: bool,
    pub index: usize,
    historical: bool,
}

pub(super) struct Forest {
    pub trees: Vec<Tree>,
    pub bytes: Vec<u8>,
}

pub(super) struct Leaf {
    pub generation: u8,
    pub ccc: u8,
    pub mapping: Option<Vec<u8>>,
}

fn message(verbose: i32, text: String) {
    if verbose > 0 {
        crate::output(text.as_bytes());
    }
}

impl Tree {
    pub(super) fn name(&self) -> &'static str {
        if self.casefold {
            "nfdicf"
        } else {
            "nfdi"
        }
    }

    fn new(max_age: u32, casefold: bool) -> Self {
        Self {
            root: Child::Empty,
            nodes: Vec::new(),
            free: Vec::new(),
            leaf_indices: vec![0; LIMIT],
            next: None,
            max_age,
            casefold,
            index: 0,
            historical: false,
        }
    }

    fn allocate(&mut self, parent: Option<usize>) -> usize {
        let (bit, next_byte) = match parent {
            None => (7, false),
            Some(i) if self.nodes[i].bit & 7 == 0 => (self.nodes[i].bit + 15, true),
            Some(i) => (self.nodes[i].bit - 1, false),
        };
        let node = Node {
            parent,
            children: [Child::Empty; 2],
            bit,
            next_byte,
            key_bits: 0,
            key_mask: 0,
            marked: false,
            index: 0,
            offset: None,
            size: 4,
        };
        if let Some(i) = self.free.pop() {
            self.nodes[i] = node;
            i
        } else {
            let i = self.nodes.len();
            self.nodes.push(node);
            i
        }
    }

    fn equal(&self, db: &Database, left: usize, right: usize) -> bool {
        let (l, r) = (&db.records[left], &db.records[right]);
        l.generation == r.generation
            && l.ccc == r.ccc
            && if self.casefold && (l.cf.is_some() || r.cf.is_some()) {
                l.cf == r.cf
            } else {
                l.nfdi == r.nfdi
            }
    }

    fn insert(&mut self, key: &[u8], leaf: usize, db: &Database) {
        if self.root == Child::Empty {
            self.root = Child::Node(self.allocate(None));
        }
        let mut id = self.root.node().expect("UTF-8 tree root must be a node");
        let mut byte = 0;
        for remaining in (0..key.len() * 8).rev() {
            if self.nodes[id].next_byte {
                byte += 1;
            }
            let right = usize::from(key[byte] & (1 << (self.nodes[id].bit & 7)) != 0);
            if remaining == 0 {
                self.nodes[id].children[right] = Child::Leaf(leaf);
                break;
            }
            if self.nodes[id].children[right] == Child::Empty {
                let child = self.allocate(Some(id));
                self.nodes[id].children[right] = Child::Node(child);
            }
            id = self.nodes[id].children[right]
                .node()
                .expect("ordered UTF-8 insertion");
        }

        let mut current = Some(id);
        while let Some(i) = current {
            let node = &self.nodes[i];
            if node.next_byte {
                break;
            }
            let [Child::Leaf(left), Child::Leaf(right)] = node.children else {
                break;
            };
            if !self.equal(db, left, right) {
                break;
            }
            let bit = node.bit;
            let parent = node.parent;
            if let Some(p) = parent {
                let side = usize::from(self.nodes[p].children[1] == Child::Node(i));
                assert_eq!(self.nodes[p].children[side], Child::Node(i));
                self.nodes[p].children[side] = Child::Leaf(left);
                if self.nodes[p].children[1 - side].exists() {
                    self.nodes[p].key_mask = 0;
                    self.nodes[p].key_bits = 0;
                } else {
                    self.nodes[p].key_mask |= 1 << bit;
                    if side == 1 {
                        self.nodes[p].key_bits |= 1 << bit;
                    }
                }
            } else {
                self.root = Child::Leaf(left);
            }
            self.free.push(i);
            current = parent;
        }
        while let Some(i) = current {
            let Some(p) = self.nodes[i].parent else { break };
            if self.nodes[i].key_mask == 0 || self.nodes[p].children.iter().all(|c| c.exists()) {
                self.nodes[p].key_mask = 0;
                self.nodes[p].key_bits = 0;
            } else {
                assert_eq!(self.nodes[p].key_mask & self.nodes[i].key_mask, 0);
                self.nodes[p].key_mask |= self.nodes[i].key_mask | (1 << self.nodes[p].bit);
                self.nodes[p].key_bits |= self.nodes[i].key_bits;
                if self.nodes[p].children[1].exists() {
                    self.nodes[p].key_bits |= 1 << self.nodes[p].bit;
                }
            }
            current = Some(p);
        }
    }

    fn singleton_leaf(&self, mut i: usize) -> usize {
        loop {
            let [left, right] = self.nodes[i].children;
            if let Child::Leaf(l) = left {
                return l;
            }
            if let Child::Leaf(r) = right {
                return r;
            }
            i = left
                .node()
                .or_else(|| right.node())
                .expect("nonempty singleton path");
        }
    }

    fn prune(&mut self, db: &Database, verbose: i32) {
        message(
            verbose,
            format!("Pruning {}_{:x}\n", self.name(), self.max_age),
        );
        let mut current = self.root.node();
        let (mut left_mask, mut right_mask, mut count) = (0u32, 0u32, 0);
        while let Some(mut i) = current {
            let node = &self.nodes[i];
            if !node.next_byte {
                if let [Child::Node(left), Child::Node(right)] = node.children {
                    if self.nodes[left].key_mask != 0
                        && self.nodes[left].key_mask == self.nodes[right].key_mask
                        && self.nodes[left].key_bits == self.nodes[right].key_bits
                        && self.equal(db, self.singleton_leaf(left), self.singleton_leaf(right))
                    {
                        let parent = node.parent.expect("UTF-8 root cannot be pruned");
                        let side = usize::from(self.nodes[parent].children[1] == Child::Node(i));
                        assert_eq!(self.nodes[parent].children[side], Child::Node(i));
                        self.nodes[parent].children[side] = Child::Node(left);
                        self.nodes[left].parent = Some(parent);
                        self.nodes[left].key_mask |= 1 << self.nodes[i].bit;
                        self.nodes[i].children[0] = Child::Empty;
                        let mut discard = Some(i);
                        while let Some(j) = discard {
                            let bit = 1 << self.nodes[j].bit;
                            left_mask &= !bit;
                            right_mask &= !bit;
                            discard = self.nodes[j].children[0]
                                .node()
                                .or_else(|| self.nodes[j].children[1].node());
                            self.free.push(j);
                            // C counts only links freed before the terminal
                            // singleton (which it leaks). Reclaim that last
                            // arena slot too, but preserve the diagnostic count.
                            if discard.is_some() {
                                count += 1;
                            }
                        }
                        i = parent;
                        left_mask &= !(1 << self.nodes[i].bit);
                        right_mask &= !(1 << self.nodes[i].bit);
                        loop {
                            if self.nodes[i].children.iter().all(|c| c.exists()) {
                                break;
                            }
                            for child in self.nodes[i].children {
                                if let Child::Node(j) = child {
                                    self.nodes[i].key_mask |= self.nodes[j].key_mask;
                                    self.nodes[i].key_bits |= self.nodes[j].key_bits;
                                }
                            }
                            self.nodes[i].key_mask |= 1 << self.nodes[i].bit;
                            i = self.nodes[i]
                                .parent
                                .expect("singleton chain ends below root");
                            left_mask &= !(1 << self.nodes[i].bit);
                            right_mask &= !(1 << self.nodes[i].bit);
                        }
                    }
                }
            }
            let bit = 1 << self.nodes[i].bit;
            if left_mask & bit == 0 && self.nodes[i].children[0].node().is_some() {
                left_mask |= bit;
                current = self.nodes[i].children[0].node();
            } else if right_mask & bit == 0 && self.nodes[i].children[1].node().is_some() {
                right_mask |= bit;
                current = self.nodes[i].children[1].node();
            } else {
                left_mask &= !bit;
                right_mask &= !bit;
                current = self.nodes[i].parent;
            }
        }
        message(verbose, format!("Pruned {count} nodes\n"));
    }

    fn leaf_mark(&self, r: &Record) -> bool {
        if self.historical {
            r.correction != 0
        } else if self.casefold {
            r.cf.is_some()
        } else {
            true
        }
    }

    fn mark_ancestors(&mut self, mut current: Option<usize>) -> usize {
        let mut count = 0;
        while let Some(i) = current {
            if self.nodes[i].marked {
                break;
            }
            self.nodes[i].marked = true;
            count += 1;
            current = self.nodes[i].parent;
        }
        count
    }

    fn mark(&mut self, db: &Database, verbose: i32) {
        message(
            verbose,
            format!("Marking {}_{:x}\n", self.name(), self.max_age),
        );
        let mut marked = 0;
        for second in [false, true] {
            let mut stack = vec![(self.root, None, 0)];
            while let Some((child, parent, side)) = stack.pop() {
                match child {
                    Child::Empty => (),
                    Child::Leaf(l) => {
                        if self.leaf_mark(&db.records[l]) {
                            marked += self.mark_ancestors(parent);
                        }
                    }
                    Child::Node(i) => {
                        if second {
                            if let Some(p) = parent {
                                if !self.nodes[i].marked
                                    && self.nodes[p].marked
                                    && (side == 0 || !self.nodes[p].children[0].exists())
                                {
                                    self.nodes[i].marked = true;
                                    marked += 1;
                                }
                            }
                        }
                        stack.push((self.nodes[i].children[1], Some(i), 1));
                        stack.push((self.nodes[i].children[0], Some(i), 0));
                    }
                }
            }
        }
        message(verbose, format!("Marked {marked} nodes\n"));
    }

    fn mapping<'a>(&self, record: &'a Record) -> Option<&'a [u8]> {
        if self.casefold {
            record.cf.as_deref().or(record.nfdi.as_deref())
        } else {
            record.nfdi.as_deref()
        }
    }

    fn leaf_bytes(&self, record: &Record) -> Vec<u8> {
        let mut bytes = vec![record.generation as u8];
        if is_hangul(record.code) {
            bytes.extend([255, HANGUL]);
        } else if let Some(mapping) = self.mapping(record) {
            bytes.push(255);
            bytes.extend(mapping);
            bytes.push(0);
        } else {
            bytes.push(record.ccc as u8);
        }
        bytes
    }

    fn leaf_size(&self, record: &Record) -> usize {
        2 + if is_hangul(record.code) {
            1
        } else {
            self.mapping(record).map_or(0, |s| s.len() + 1)
        }
    }

    fn index(&mut self, db: &Database, mut offset: usize, verbose: i32) -> usize {
        offset = offset.next_multiple_of(64);
        self.index = offset;
        message(
            verbose,
            format!("Indexing {}_{:x}: {offset}\n", self.name(), self.max_age),
        );
        let mut stack = vec![self.root];
        while let Some(child) = stack.pop() {
            match child {
                Child::Empty => (),
                Child::Leaf(l) => {
                    self.leaf_indices[db.records[l].code as usize] = offset;
                    offset += self.leaf_size(&db.records[l]);
                }
                Child::Node(i) => {
                    if self.nodes[i].marked {
                        self.nodes[i].index = offset;
                        offset += self.nodes[i].size;
                        stack.extend(self.nodes[i].children.into_iter().rev());
                    }
                }
            }
        }
        offset = offset.next_multiple_of(16);
        message(verbose, format!("Final index {offset}\n"));
        offset
    }

    fn mark_subtree(&mut self, start: usize) -> usize {
        let mut stack = vec![start];
        let mut changed = 0;
        while let Some(i) = stack.pop() {
            if self.nodes[i].marked {
                continue;
            }
            self.nodes[i].marked = true;
            self.nodes[i].index =
                self.nodes[self.nodes[i].parent.expect("non-root forwarded subtree")].index;
            changed += 1;
            stack.extend(
                self.nodes[i]
                    .children
                    .into_iter()
                    .rev()
                    .filter_map(Child::node),
            );
        }
        changed
    }

    fn emit(&self, db: &Database, output: &mut [u8], verbose: i32) {
        message(
            verbose,
            format!("Emitting {}_{:x}\n", self.name(), self.max_age),
        );
        let mut index = self.index;
        let (mut leaves, mut leaf_bytes, mut nodes) = (0, 0, [0; 4]);
        let mut stack = vec![self.root];
        while let Some(child) = stack.pop() {
            match child {
                Child::Empty => (),
                Child::Leaf(l) => {
                    let bytes = self.leaf_bytes(&db.records[l]);
                    output[index..index + bytes.len()].copy_from_slice(&bytes);
                    index += bytes.len();
                    leaves += 1;
                    leaf_bytes += bytes.len();
                }
                Child::Node(i) => {
                    let node = &self.nodes[i];
                    if !node.marked {
                        continue;
                    }
                    assert_eq!(node.index, index);
                    let offset = node.offset.expect("sized trie node");
                    let mut byte = (node.bit & 7) | if node.next_byte { 8 } else { 0 };
                    if node.children.iter().all(|c| c.exists()) {
                        if node.children[0].node().is_some() {
                            byte |= 128;
                        }
                        if node.children[1].node().is_some() {
                            byte |= 64;
                        }
                        let len = if offset <= 255 {
                            1
                        } else if offset <= 65535 {
                            2
                        } else {
                            3
                        };
                        nodes[len] += 1;
                        byte |= (len as u8) << 4;
                        output[index] = byte;
                        index += 1;
                        for k in 0..len {
                            output[index] = (offset >> (8 * k)) as u8;
                            index += 1;
                        }
                    } else {
                        let side = usize::from(node.children[1].exists());
                        if side == 1 {
                            byte |= 64;
                        }
                        if node.children[side].node().is_some() {
                            byte |= 128;
                        }
                        nodes[0] += 1;
                        output[index] = byte;
                        index += 1;
                    }
                    stack.extend(node.children.into_iter().rev());
                }
            }
        }
        message(
            verbose,
            format!(
                "Emitted {leaves} ({leaf_bytes}) leaves {} ({}+{}+{}+{}) nodes {} total\n",
                nodes.iter().sum::<usize>(),
                nodes[0],
                nodes[1],
                nodes[2],
                nodes[3],
                index - self.index
            ),
        );
    }
}

impl Forest {
    pub(super) fn dump(&self, db: &Database) {
        for index in [self.trees.len() - 1, self.trees.len() - 2] {
            let tree = &self.trees[index];
            let address = |child: Child| match child {
                Child::Empty => "(nil)".to_string(),
                Child::Node(i) => format!("{:p}", &tree.nodes[i]),
                Child::Leaf(i) => format!("{:p}", &db.records[i]),
            };
            crate::output(
                format!(
                    "{}_{:x} root {}\n",
                    tree.name(),
                    tree.max_age,
                    address(tree.root)
                )
                .as_bytes(),
            );
            let (mut nodes, mut leaves, mut singletons) = (0, 0, 0);
            let mut stack = vec![(tree.root, 1usize)];
            while let Some((child, indent)) = stack.pop() {
                match child {
                    Child::Empty => (),
                    Child::Node(i) => {
                        let node = &tree.nodes[i];
                        nodes += 1;
                        if !node.children.iter().all(|c| c.exists()) {
                            singletons += 1;
                        }
                        crate::output(format!("{:indent$}node @ {} bitnum {} nextbyte {} left {} right {} mask {:x} bits {:x}\n", "", address(child), node.bit, u8::from(node.next_byte), address(node.children[0]), address(node.children[1]), node.key_mask, node.key_bits).as_bytes());
                        stack.push((node.children[1], indent + 1));
                        stack.push((node.children[0], indent + 1));
                    }
                    Child::Leaf(i) => {
                        leaves += 1;
                        let record = &db.records[i];
                        crate::output(
                            format!(
                                "{:indent$}leaf @ {} code {:X} ccc {} gen {}",
                                "",
                                address(child),
                                record.code,
                                record.ccc,
                                record.generation
                            )
                            .as_bytes(),
                        );
                        if tree.casefold && record.cf.is_some() {
                            crate::output(b" nfdicf \"");
                            crate::output(record.cf.as_deref().unwrap());
                            crate::output(b"\"");
                        } else if let Some(mapping) = &record.nfdi {
                            crate::output(b" nfdi \"");
                            crate::output(if mapping.first() == Some(&HANGUL) {
                                b"HANGUL SYLLABLE"
                            } else {
                                mapping
                            });
                            crate::output(b"\"");
                        }
                        crate::output(b"\n");
                    }
                }
            }
            crate::output(
                format!("nodes {nodes} leaves {leaves} singletons {singletons}\n").as_bytes(),
            );
        }
    }

    fn size(&mut self, tree: usize, db: &Database, verbose: i32) -> usize {
        message(
            verbose,
            format!(
                "Sizing {}_{:x}\n",
                self.trees[tree].name(),
                self.trees[tree].max_age
            ),
        );
        let mut changed = 0;
        let mut stack = Vec::new();
        if let Some(root) = self.trees[tree].root.node() {
            stack.push((root, 0u32, 0u32));
        }
        while let Some((i, path_bits, path_mask)) = stack.pop() {
            let node = self.trees[tree].nodes[i].clone();
            if !node.marked {
                continue;
            }
            let offset = if !node.children.iter().all(|c| c.exists()) {
                0
            } else {
                match node.children[1] {
                    Child::Leaf(l) => {
                        self.trees[tree].leaf_indices[db.records[l].code as usize] - node.index
                    }
                    Child::Node(mut right) => {
                        let mut owner = tree;
                        let mut next = self.trees[tree].next;
                        while !self.trees[owner].nodes[right].marked {
                            let next_tree = next.expect("unmarked node must have a fallback tree");
                            let mut n = self.trees[next_tree].root.node().expect("fallback root");
                            loop {
                                let other = &self.trees[next_tree].nodes[n];
                                if other.bit == node.bit {
                                    break;
                                }
                                let bit = 1 << other.bit;
                                if path_mask & bit == 0 {
                                    break;
                                }
                                let Some(child) =
                                    other.children[usize::from(path_bits & bit != 0)].node()
                                else {
                                    break;
                                };
                                n = child;
                            }
                            if self.trees[next_tree].nodes[n].bit != node.bit {
                                break;
                            }
                            right = self.trees[next_tree].nodes[n].children[1]
                                .node()
                                .expect("corresponding right node");
                            owner = next_tree;
                            next = self.trees[owner].next;
                        }
                        if !self.trees[owner].nodes[right].marked {
                            changed += self.trees[owner].mark_subtree(right);
                        }
                        self.trees[owner].nodes[right]
                            .index
                            .checked_sub(node.index)
                            .expect("forward-only sharing")
                    }
                    Child::Empty => unreachable!(),
                }
            };
            assert!(offset <= 0xffffff);
            let size = if !node.children.iter().all(|c| c.exists()) {
                1
            } else if offset <= 255 {
                2
            } else if offset <= 65535 {
                3
            } else {
                4
            };
            if node.size != size || node.offset != Some(offset) {
                self.trees[tree].nodes[i].size = size;
                self.trees[tree].nodes[i].offset = Some(offset);
                changed += 1;
            }
            let bit = 1 << node.bit;
            if let Some(right) = node.children[1].node() {
                stack.push((right, path_bits | bit, path_mask | bit));
            }
            if let Some(left) = node.children[0].node() {
                stack.push((left, path_bits, path_mask | bit));
            }
        }
        message(verbose, format!("Found {changed} changes\n"));
        changed
    }

    pub(super) fn lookup(&self, tree: usize, input: &[u8]) -> Option<Leaf> {
        if input.is_empty() {
            return None;
        }
        let mut offset = self.trees.get(tree)?.index;
        let mut byte = 0;
        loop {
            let node = *self.bytes.get(offset)?;
            let len = usize::from((node & 48) >> 4);
            if node & 8 != 0 {
                byte += 1;
            }
            let right = input.get(byte)? & (1 << (node & 7)) != 0;
            let internal;
            if len != 0 {
                if right {
                    let mut jump = 0usize;
                    for k in 0..len {
                        jump |= usize::from(*self.bytes.get(offset + k + 1)?) << (8 * k);
                    }
                    offset += jump;
                    internal = node & 64 != 0;
                } else {
                    offset += len + 1;
                    internal = node & 128 != 0;
                }
            } else {
                if right != (node & 64 != 0) {
                    return None;
                }
                offset += 1;
                internal = node & 128 != 0;
            }
            if !internal {
                break;
            }
        }
        let mut generation = *self.bytes.get(offset)?;
        let ccc = *self.bytes.get(offset + 1)?;
        let mapping = if ccc == 255 {
            if *self.bytes.get(offset + 2)? == HANGUL {
                let s = input.get(byte.checked_sub(2)?..=byte)?;
                let code = (u32::from(s[0] & 15) << 12)
                    | (u32::from(s[1] & 63) << 6)
                    | u32::from(s[2] & 63);
                generation = 2;
                Some(hangul(code))
            } else {
                let start = offset + 2;
                let length = self.bytes.get(start..)?.iter().position(|&b| b == 0)?;
                Some(self.bytes[start..start + length].to_vec())
            }
        } else {
            None
        };
        Some(Leaf {
            generation,
            ccc,
            mapping,
        })
    }
}

pub(super) fn build(db: &Database, verbose: i32) -> Forest {
    let mut thresholds: Vec<u32> = db.records[LIMIT..]
        .iter()
        .map(|r| r.correction)
        .filter(|&a| a != 0)
        .collect();
    thresholds.push(u32::MAX);
    thresholds.sort_unstable();
    thresholds.dedup();
    let mut forest = Forest {
        trees: Vec::new(),
        bytes: Vec::new(),
    };
    for threshold in thresholds {
        let age = db
            .ages
            .iter()
            .copied()
            .take_while(|&age| age < threshold)
            .last()
            .expect("age zero precedes corrections");
        forest.trees.push(Tree::new(age, true));
        forest.trees.push(Tree::new(age, false));
    }
    let latest = forest.trees.len() - 2;
    for i in 0..latest {
        forest.trees[i].next = Some(latest + (i & 1));
        forest.trees[i].historical = true;
    }
    forest.trees[latest].next = Some(latest + 1);
    let mut key = Vec::with_capacity(4);
    for tree in &mut forest.trees {
        message(
            verbose,
            format!("Populating {}_{:x}\n", tree.name(), tree.max_age),
        );
        for code in 0..LIMIT {
            if db.records[code].generation < 0 {
                continue;
            }
            key.clear();
            encode(code as u32, &mut key);
            tree.insert(&key, db.record_at_age(code, tree.max_age), db);
        }
    }
    for tree in &mut forest.trees {
        tree.prune(db, verbose);
    }
    for tree in &mut forest.trees {
        tree.mark(db, verbose);
    }
    let size = loop {
        let mut size = 0;
        for tree in &mut forest.trees {
            size = tree.index(db, size, verbose);
        }
        let mut changed = 0;
        for i in 0..forest.trees.len() {
            changed += forest.size(i, db, verbose);
        }
        if changed == 0 {
            break size;
        }
    };
    forest.bytes.resize(size, 0);
    for tree in &forest.trees {
        tree.emit(db, &mut forest.bytes, verbose);
    }
    for tree in &forest.trees {
        message(
            verbose,
            format!("{}_{:x} idx {}\n", tree.name(), tree.max_age, tree.index),
        );
    }
    forest
}
