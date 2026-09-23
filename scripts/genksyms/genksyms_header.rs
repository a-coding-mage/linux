// SPDX-License-Identifier: GPL-2.0-or-later
// Generate kernel symbol version hashes.
// Copyright 1996, 1997 Linux International.
// Original implementation: Richard Henderson <rth@tamu.edu>, based on work
// by Bjorn Ekwall <bj0rn@blox.se>. This file was part of Linux modutils.

/// Genksyms keeps C tag names separate from ordinary identifiers.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) enum Kind {
    #[default]
    Normal,
    Typedef,
    Enum,
    Struct,
    Union,
    EnumConst,
}

impl Kind {
    pub(super) fn namespace(self) -> bool {
        matches!(self, Self::Enum | Self::Struct | Self::Union)
    }
    pub(super) fn tag(self) -> Option<u8> {
        match self {
            Self::Normal => None,
            Self::Typedef => Some(b't'),
            Self::Enum => Some(b'e'),
            Self::Struct => Some(b's'),
            Self::Union => Some(b'u'),
            Self::EnumConst => Some(b'E'),
        }
    }
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Normal => "",
            Self::Typedef => "typedef",
            Self::Enum => "enum",
            Self::Struct => "struct",
            Self::Union => "union",
            Self::EnumConst => "enum constant",
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct Word {
    pub(super) text: Vec<u8>,
    pub(super) kind: Kind,
}

impl Word {
    pub(super) fn plain(text: impl AsRef<[u8]>) -> Self {
        Self {
            text: text.as_ref().to_vec(),
            kind: Kind::Normal,
        }
    }
    pub(super) fn print(&self, output: &mut Vec<u8>) {
        if let Some(tag) = self.kind.tag() {
            output.extend_from_slice(&[tag, b'#']);
        }
        output.extend_from_slice(&self.text);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Status {
    Unchanged,
    Defined,
    Modified,
}

pub(super) struct Symbol {
    pub(super) name: Vec<u8>,
    pub(super) kind: Kind,
    pub(super) definition: Vec<Word>,
    pub(super) external: bool,
    pub(super) declared: bool,
    pub(super) status: Status,
    pub(super) override_version: bool,
    pub(super) visited: bool,
}
