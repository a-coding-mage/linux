// SPDX-License-Identifier: GPL-2.0-or-later
/* (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation. 2005. */

//! Recursive-descent implementation of every production in dtc-parser.y.

use crate::dtc_header::*;
use crate::lexer::{Kind, Lexer, Token};
use crate::livetree::add_label;

struct Parser<'a> {
    lexer: Lexer,
    lookahead: Option<Token>,
    last: SourcePos,
    tree: DtInfo,
    diagnostics: &'a mut Diagnostics,
}

impl Parser<'_> {
    fn source(&mut self, prefix: &[u8], pos: &SourcePos, message: impl AsRef<[u8]>) {
        let position = self.lexer.sources.snapshot(pos);
        self.diagnostics.source(prefix, &position, message);
    }

    fn peek(&mut self) -> Result<Kind, Vec<u8>> {
        if self.lookahead.is_none() {
            self.lookahead = Some(self.lexer.next(self.diagnostics)?);
        }
        Ok(self.lookahead.as_ref().unwrap().kind.clone())
    }

    fn take(&mut self) -> Result<Token, Vec<u8>> {
        self.peek()?;
        let token = self.lookahead.take().unwrap();
        self.last = token.pos.clone();
        Ok(token)
    }

    fn syntax<T>(&mut self) -> Result<T, Vec<u8>> {
        self.peek()?;
        let pos = self.lookahead.as_ref().unwrap().pos.clone();
        self.source(b"Error", &pos, b"syntax error");
        Err(Vec::new())
    }

    fn require(&mut self, kind: Kind) -> Result<Token, Vec<u8>> {
        if self.peek()? != kind {
            return self.syntax();
        }
        self.take()
    }

    fn accept(&mut self, kind: Kind) -> Result<bool, Vec<u8>> {
        if self.peek()? == kind {
            self.take()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn reference(&mut self) -> Result<Token, Vec<u8>> {
        if matches!(self.peek()?, Kind::LabelRef | Kind::PathRef) {
            self.take()
        } else {
            self.syntax()
        }
    }

    fn labels(&mut self) -> Result<Vec<Token>, Vec<u8>> {
        let mut labels = Vec::new();
        while self.peek()? == Kind::Label {
            labels.push(self.take()?);
        }
        Ok(labels)
    }

    fn parse(&mut self) -> Result<(), Vec<u8>> {
        let mut headers = Vec::new();
        while self.peek()? == Kind::V1 {
            let start = self.take()?.pos;
            self.require(Kind::Char(b';'))?;
            let mut flags = DTSF_V1;
            if self.accept(Kind::Plugin)? {
                self.require(Kind::Char(b';'))?;
                flags |= DTSF_PLUGIN;
            }
            headers.push((flags, start.through(&self.last)));
        }
        if headers.is_empty() {
            return self.syntax();
        }
        self.tree.dtsflags = headers[0].0;
        if let Some((_, end)) = headers.last() {
            // headers is right-recursive: report mismatch locations in reduction order.
            for index in (0..headers.len().saturating_sub(1)).rev() {
                if headers[index].0 != headers[index + 1].0 {
                    self.source(
                        b"Error",
                        &headers[index + 1].1.through(end),
                        b"Header flags don't match earlier ones",
                    );
                }
            }
        }
        loop {
            let labels = self.labels()?;
            if self.peek()? != Kind::Memreserve {
                if !labels.is_empty() {
                    return self.syntax();
                }
                break;
            }
            self.take()?;
            let address = self.integer_prim()?.0;
            let size = self.integer_prim()?.0;
            self.require(Kind::Char(b';'))?;
            let mut reserve = Reserve {
                address,
                size,
                labels: Vec::new(),
            };
            for label in labels.into_iter().rev() {
                add_label(&mut reserve.labels, label.text);
            }
            self.tree.reserves.push(reserve);
        }
        if self.accept(Kind::Char(b'/'))? {
            self.tree.root = self.node()?;
        } else if matches!(self.peek()?, Kind::LabelRef | Kind::PathRef) {
            let reference = self.reference()?;
            let node = self.node()?;
            let pos = self.tree.nodes[node].srcpos[0].clone();
            if self.tree.dtsflags & DTSF_PLUGIN == 0 {
                self.source(
                    b"Error",
                    &pos,
                    format!("Label or path {} not found", display(&reference.text)),
                );
            } else if is_relative(&reference.text) {
                self.source(
                    b"Error",
                    &pos,
                    format!(
                        "Label-relative reference {} not supported in plugin",
                        display(&reference.text)
                    ),
                );
            }
            self.tree.root = self.tree.add_node(Node::default());
            self.tree.orphan(node, reference.text);
        } else {
            return self.syntax();
        }
        loop {
            match self.peek()? {
                Kind::Eof => break,
                Kind::Char(b'/') => {
                    self.take()?;
                    let node = self.node()?;
                    self.tree.merge_nodes(self.tree.root, node);
                }
                Kind::Label => {
                    let label = self.take()?;
                    let reference = self.reference()?;
                    let node = self.node()?;
                    if self.tree.dtsflags & DTSF_PLUGIN != 0 && is_relative(&reference.text) {
                        self.source(
                            b"Error",
                            &label.pos,
                            format!(
                                "Label-relative reference {} not supported in plugin",
                                display(&reference.text)
                            ),
                        );
                    }
                    if let Some(target) = self.tree.node_by_ref(&reference.text) {
                        add_label(&mut self.tree.nodes[target].labels, label.text);
                        self.tree.merge_nodes(target, node);
                    } else {
                        self.missing_reference(&reference);
                    }
                }
                Kind::LabelRef | Kind::PathRef => {
                    let reference = self.take()?;
                    let node = self.node()?;
                    if reference.kind == Kind::PathRef && self.tree.dtsflags & DTSF_PLUGIN != 0 {
                        if is_relative(&reference.text) {
                            self.source(
                                b"Error",
                                &reference.pos,
                                format!(
                                    "Label-relative reference {} not supported in plugin",
                                    display(&reference.text)
                                ),
                            );
                        }
                        self.tree.orphan(node, reference.text);
                    } else if let Some(target) = self.tree.node_by_ref(&reference.text) {
                        self.tree.merge_nodes(target, node);
                    } else if self.tree.dtsflags & DTSF_PLUGIN != 0 {
                        self.tree.orphan(node, reference.text);
                    } else {
                        self.missing_reference(&reference);
                    }
                }
                Kind::DeleteNode | Kind::Omit => {
                    let kind = self.take()?.kind;
                    let reference = self.reference()?;
                    self.require(Kind::Char(b';'))?;
                    if let Some(target) = self.tree.node_by_ref(&reference.text) {
                        if kind == Kind::DeleteNode {
                            self.tree.delete_node(target);
                        } else {
                            self.tree.nodes[target].omit_if_unused = true;
                        }
                    } else {
                        self.missing_reference(&reference);
                    }
                }
                _ => return self.syntax(),
            }
        }
        self.tree.boot_cpuid_phys = self.tree.guess_boot_cpuid();
        Ok(())
    }

    fn missing_reference(&mut self, reference: &Token) {
        self.source(
            b"Error",
            &reference.pos,
            format!("Label or path {} not found", display(&reference.text)),
        );
    }

    fn node(&mut self) -> Result<NodeId, Vec<u8>> {
        struct Frame {
            start: SourcePos,
            node: Node,
            seen_child: bool,
            labels: Vec<Token>,
        }
        let mut frames = vec![Frame {
            start: self.require(Kind::Char(b'{'))?.pos,
            node: Node::default(),
            seen_child: false,
            labels: Vec::new(),
        }];
        loop {
            if self.peek()? == Kind::Char(b'}') {
                self.take()?;
                self.require(Kind::Char(b';'))?;
                let mut frame = frames.pop().unwrap();
                frame.node.srcpos.push(
                    self.lexer
                        .sources
                        .snapshot(&frame.start.through(&self.last)),
                );
                for label in frame.labels.into_iter().rev() {
                    add_label(&mut frame.node.labels, label.text);
                }
                let id = self.tree.add_node(frame.node);
                let Some(parent) = frames.last_mut() else {
                    return Ok(id);
                };
                parent.node.children.push(id);
                parent.seen_child = true;
                continue;
            }
            let mut labels = Vec::new();
            let mut omit = false;
            let first = {
                self.peek()?;
                self.lookahead.as_ref().unwrap().pos.clone()
            };
            loop {
                match self.peek()? {
                    Kind::Label => labels.push(self.take()?),
                    Kind::Omit => {
                        self.take()?;
                        omit = true;
                    }
                    _ => break,
                }
            }
            let token = self.take()?;
            let mut property = None;
            let mut child = None;
            match token.kind {
                Kind::DeleteProperty if !omit => {
                    let name = self.require(Kind::Name)?.text;
                    self.require(Kind::Char(b';'))?;
                    property = Some(Property {
                        name,
                        deleted: true,
                        ..Property::default()
                    });
                }
                Kind::DeleteNode => {
                    let name = self.require(Kind::Name)?.text;
                    self.require(Kind::Char(b';'))?;
                    child = Some(self.tree.add_node(Node {
                        name,
                        deleted: true,
                        srcpos: vec![self.lexer.sources.snapshot(&token.pos.through(&self.last))],
                        ..Node::default()
                    }));
                }
                Kind::Name => {
                    if self.peek()? == Kind::Char(b'{') {
                        let start = self.take()?.pos;
                        frames.push(Frame {
                            start,
                            node: Node {
                                name: token.text,
                                omit_if_unused: omit,
                                ..Node::default()
                            },
                            seen_child: false,
                            labels,
                        });
                        continue;
                    } else {
                        if omit {
                            return self.syntax();
                        }
                        let data = if self.accept(Kind::Char(b'='))? {
                            self.property_data()?
                        } else {
                            Data::default()
                        };
                        self.require(Kind::Char(b';'))?;
                        property = Some(Property::new(
                            token.text,
                            data,
                            vec![self.lexer.sources.snapshot(&token.pos.through(&self.last))],
                        ));
                    }
                }
                _ => {
                    self.lookahead = Some(token);
                    return self.syntax();
                }
            }
            if let Some(mut property) = property {
                for label in labels.into_iter().rev() {
                    add_label(&mut property.labels, label.text);
                }
                if frames.last().unwrap().seen_child {
                    self.source(
                        b"Error",
                        &first.through(&self.last),
                        b"Properties must precede subnodes",
                    );
                    return Err(Vec::new());
                }
                frames.last_mut().unwrap().node.properties.push(property);
            } else if let Some(id) = child {
                for label in labels.into_iter().rev() {
                    add_label(&mut self.tree.nodes[id].labels, label.text);
                }
                self.tree.nodes[id].omit_if_unused |= omit;
                let frame = frames.last_mut().unwrap();
                frame.node.children.push(id);
                frame.seen_child = true;
            }
        }
    }

    fn property_data(&mut self) -> Result<Data, Vec<u8>> {
        let mut data = Data::default();
        loop {
            for label in self.labels()? {
                data.marker(MarkerKind::Label, Some(label.text));
            }
            match self.peek()? {
                Kind::String => {
                    let token = self.take()?;
                    let mut string = Data::default();
                    string.marker(MarkerKind::String, None);
                    string.bytes = token.text;
                    data.merge(string);
                }
                Kind::Bits | Kind::Char(b'<') => data.merge(self.array()?),
                Kind::Char(b'[') => {
                    self.take()?;
                    data.marker(MarkerKind::Uint8, None);
                    loop {
                        match self.peek()? {
                            Kind::Byte => data.bytes.push(self.take()?.value as u8),
                            Kind::Label => data.marker(MarkerKind::Label, Some(self.take()?.text)),
                            _ => break,
                        }
                    }
                    self.require(Kind::Char(b']'))?;
                }
                Kind::LabelRef | Kind::PathRef => {
                    let reference = self.take()?.text;
                    data.marker(MarkerKind::String, Some(reference.clone()));
                    data.marker(MarkerKind::RefPath, Some(reference));
                }
                Kind::Incbin => {
                    self.take()?;
                    self.require(Kind::Char(b'('))?;
                    let name = self.require(Kind::String)?.text;
                    let mut offset = 0;
                    let mut length = u64::MAX;
                    if self.accept(Kind::Char(b','))? {
                        offset = self.integer_prim()?.0;
                        self.require(Kind::Char(b','))?;
                        length = self.integer_prim()?.0;
                    }
                    self.require(Kind::Char(b')'))?;
                    let filename = name.split(|&b| b == 0).next().unwrap_or(b"");
                    let (_, bytes) = self.lexer.sources.open(filename)?;
                    if offset > i64::MAX as u64 {
                        return Err(format!(
                            "Couldn't seek to offset {} in \"{}\": Invalid argument",
                            offset,
                            display(filename)
                        )
                        .into_bytes());
                    }
                    let start = usize::try_from(offset)
                        .unwrap_or(usize::MAX)
                        .min(bytes.len());
                    let end = start
                        .saturating_add(usize::try_from(length).unwrap_or(usize::MAX))
                        .min(bytes.len());
                    let mut binary = Data::default();
                    binary.marker(MarkerKind::None, None);
                    binary.bytes.extend_from_slice(&bytes[start..end]);
                    data.merge(binary);
                }
                _ => return self.syntax(),
            }
            for label in self.labels()? {
                data.marker(MarkerKind::Label, Some(label.text));
            }
            if !self.accept(Kind::Char(b','))? {
                break;
            }
        }
        Ok(data)
    }

    fn array(&mut self) -> Result<Data, Vec<u8>> {
        let mut bits = 32;
        let mut kind = MarkerKind::Uint32;
        if self.accept(Kind::Bits)? {
            let number = self.require(Kind::Integer)?;
            self.require(Kind::Char(b'<'))?;
            match number.value {
                8 => {
                    bits = 8;
                    kind = MarkerKind::Uint8;
                }
                16 => {
                    bits = 16;
                    kind = MarkerKind::Uint16;
                }
                32 => {}
                64 => {
                    bits = 64;
                    kind = MarkerKind::Uint64;
                }
                _ => self.source(
                    b"Error",
                    &number.pos,
                    b"Array elements must be 8, 16, 32 or 64-bits",
                ),
            }
        } else {
            self.require(Kind::Char(b'<'))?;
        }
        let mut data = Data::default();
        data.marker(kind, None);
        loop {
            match self.peek()? {
                Kind::Integer | Kind::CharInteger | Kind::Char(b'(') => {
                    let (value, pos) = self.integer_prim()?;
                    if bits < 64 {
                        let mask = (1u64 << bits) - 1;
                        if value > mask && value | mask != u64::MAX {
                            self.diagnostics.raw(b"WARNING: ");
                            self.diagnostics.raw(pos.render());
                            self.diagnostics.raw(format!(
                                ": Value 0x{value:016x} truncated to 0x{:0width$x}\n",
                                value & mask,
                                width = bits / 4
                            ));
                        }
                    }
                    data.append_integer(value, bits);
                }
                Kind::LabelRef | Kind::PathRef => {
                    let reference = self.take()?;
                    if bits == 32 {
                        data.marker(MarkerKind::RefPhandle, Some(reference.text));
                    } else {
                        self.source(
                            b"Error",
                            &reference.pos,
                            b"References are only allowed in arrays with 32-bit elements.",
                        );
                    }
                    data.append_integer(u64::MAX >> (64 - bits), bits);
                }
                Kind::Label => data.marker(MarkerKind::Label, Some(self.take()?.text)),
                _ => break,
            }
        }
        self.require(Kind::Char(b'>'))?;
        Ok(data)
    }

    fn integer_prim(&mut self) -> Result<(u64, SourcePos), Vec<u8>> {
        match self.peek()? {
            Kind::Integer | Kind::CharInteger => {
                let token = self.take()?;
                Ok((token.value, token.pos))
            }
            Kind::Char(b'(') => {
                let first = self.take()?.pos;
                let value = self.expression(0)?.0;
                self.require(Kind::Char(b')'))?;
                Ok((value, first.through(&self.last)))
            }
            _ => self.syntax(),
        }
    }

    fn expression(&mut self, min_precedence: usize) -> Result<(u64, SourcePos), Vec<u8>> {
        enum Work {
            Start(usize),
            Continue(u64, SourcePos, usize),
            Unary(u8, SourcePos, usize),
            Parenthesis(SourcePos, usize),
            Binary(u64, SourcePos, Kind, usize),
            Yes(u64, SourcePos, usize),
            No(u64, u64, SourcePos, usize),
        }
        let mut stack = vec![Work::Start(min_precedence)];
        let mut result = (0, SourcePos::default());
        while let Some(work) = stack.pop() {
            match work {
                Work::Start(min) => match self.peek()? {
                    Kind::Char(op @ (b'-' | b'~' | b'!')) => {
                        stack.push(Work::Unary(op, self.take()?.pos, min));
                        stack.push(Work::Start(12));
                    }
                    Kind::Char(b'(') => {
                        stack.push(Work::Parenthesis(self.take()?.pos, min));
                        stack.push(Work::Start(0));
                    }
                    Kind::Integer | Kind::CharInteger => {
                        let token = self.take()?;
                        stack.push(Work::Continue(token.value, token.pos, min));
                    }
                    _ => return self.syntax(),
                },
                Work::Continue(left, pos, min) => {
                    // A unary RHS reduces before lexing another token.
                    if min >= 11 {
                        result = (left, pos);
                        continue;
                    }
                    let kind = self.peek()?;
                    let precedence = match kind {
                        Kind::Char(b'?') => 0,
                        Kind::Or => 1,
                        Kind::And => 2,
                        Kind::Char(b'|') => 3,
                        Kind::Char(b'^') => 4,
                        Kind::Char(b'&') => 5,
                        Kind::Eq | Kind::Ne => 6,
                        Kind::Char(b'<' | b'>') | Kind::Le | Kind::Ge => 7,
                        Kind::LShift | Kind::RShift => 8,
                        Kind::Char(b'+' | b'-') => 9,
                        Kind::Char(b'*' | b'/' | b'%') => 10,
                        _ => {
                            result = (left, pos);
                            continue;
                        }
                    };
                    if precedence < min {
                        result = (left, pos);
                        continue;
                    }
                    self.take()?;
                    if kind == Kind::Char(b'?') {
                        stack.push(Work::Yes(left, pos, min));
                        stack.push(Work::Start(0));
                    } else {
                        stack.push(Work::Binary(left, pos, kind, min));
                        stack.push(Work::Start(precedence + 1));
                    }
                }
                Work::Unary(op, first, min) => {
                    let value = match op {
                        b'-' => result.0.wrapping_neg(),
                        b'~' => !result.0,
                        _ => u64::from(result.0 == 0),
                    };
                    stack.push(Work::Continue(value, first.through(&result.1), min));
                }
                Work::Parenthesis(first, min) => {
                    self.require(Kind::Char(b')'))?;
                    stack.push(Work::Continue(result.0, first.through(&self.last), min));
                }
                Work::Yes(condition, first, min) => {
                    self.require(Kind::Char(b':'))?;
                    stack.push(Work::No(condition, result.0, first, min));
                    stack.push(Work::Start(0));
                }
                Work::No(condition, yes, first, min) => {
                    stack.push(Work::Continue(
                        if condition != 0 { yes } else { result.0 },
                        first.through(&result.1),
                        min,
                    ));
                }
                Work::Binary(left, first, kind, min) => {
                    let right = result.0;
                    let pos = first.through(&result.1);
                    let value = match kind {
                        Kind::Or => u64::from(left != 0 || right != 0),
                        Kind::And => u64::from(left != 0 && right != 0),
                        Kind::Char(b'|') => left | right,
                        Kind::Char(b'^') => left ^ right,
                        Kind::Char(b'&') => left & right,
                        Kind::Eq => u64::from(left == right),
                        Kind::Ne => u64::from(left != right),
                        Kind::Char(b'<') => u64::from(left < right),
                        Kind::Char(b'>') => u64::from(left > right),
                        Kind::Le => u64::from(left <= right),
                        Kind::Ge => u64::from(left >= right),
                        Kind::LShift => {
                            if right < 64 {
                                left << right
                            } else {
                                0
                            }
                        }
                        Kind::RShift => {
                            if right < 64 {
                                left >> right
                            } else {
                                0
                            }
                        }
                        Kind::Char(b'+') => left.wrapping_add(right),
                        Kind::Char(b'-') => left.wrapping_sub(right),
                        Kind::Char(b'*') => left.wrapping_mul(right),
                        Kind::Char(b'/' | b'%') => {
                            if right == 0 {
                                self.source(b"Error", &pos, b"Division by zero");
                                0
                            } else if kind == Kind::Char(b'/') {
                                left / right
                            } else {
                                left % right
                            }
                        }
                        _ => unreachable!(),
                    };
                    stack.push(Work::Continue(value, pos, min));
                }
            }
        }
        Ok(result)
    }
}

fn is_relative(reference: &[u8]) -> bool {
    !reference.starts_with(b"/") && reference.get(1..).is_some_and(|s| s.contains(&b'/'))
}

pub(crate) fn from_source(
    name: &[u8],
    options: &Options,
    diagnostics: &mut Diagnostics,
) -> Result<DtInfo, Vec<u8>> {
    let lexer = Lexer::new(name, options.include_paths.clone())?;
    let mut parser = Parser {
        lexer,
        lookahead: None,
        last: SourcePos::default(),
        tree: DtInfo::default(),
        diagnostics,
    };
    let result = parser.parse();
    parser.diagnostics.input_dependencies = parser.lexer.sources.dependencies.clone();
    if let Err(error) = result {
        return Err(if error.is_empty() {
            b"Unable to parse input tree\n".to_vec()
        } else {
            error
        });
    }
    if parser.diagnostics.source_error {
        return Err(b"Syntax error parsing input tree\n".to_vec());
    }
    parser.tree.dependencies = parser.lexer.sources.dependencies;
    parser.tree.initial_path = parser.lexer.sources.initial_path;
    Ok(parser.tree)
}
