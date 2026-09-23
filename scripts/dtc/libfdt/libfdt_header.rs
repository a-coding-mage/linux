// SPDX-License-Identifier: (GPL-2.0-or-later OR BSD-2-Clause)
//! Shared libfdt error codes and safe property views.
// Copyright (C) 2006 David Gibson, IBM Corporation.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub(crate) enum Error {
    NotFound = 1,
    Exists,
    NoSpace,
    BadOffset,
    BadPath,
    BadPhandle,
    BadState,
    Truncated,
    BadMagic,
    BadVersion,
    BadStructure,
    BadLayout,
    Internal,
    BadNCells,
    BadValue,
    BadOverlay,
    NoPhandles,
    BadFlags,
    Alignment,
}
impl Error {
    pub(crate) fn code(self) -> i32 {
        -(self as i32)
    }
    pub(crate) fn from_code(code: i32) -> Self {
        match code {
            -1 => Self::NotFound,
            -2 => Self::Exists,
            -3 => Self::NoSpace,
            -4 => Self::BadOffset,
            -5 => Self::BadPath,
            -6 => Self::BadPhandle,
            -7 => Self::BadState,
            -8 => Self::Truncated,
            -9 => Self::BadMagic,
            -10 => Self::BadVersion,
            -11 => Self::BadStructure,
            -12 => Self::BadLayout,
            -14 => Self::BadNCells,
            -15 => Self::BadValue,
            -16 => Self::BadOverlay,
            -17 => Self::NoPhandles,
            -18 => Self::BadFlags,
            -19 => Self::Alignment,
            _ => Self::Internal,
        }
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(super::strerror(self.code()))
    }
}
impl std::error::Error for Error {}
pub(crate) type Result<T> = std::result::Result<T, Error>;
pub(crate) const MAX_PHANDLE: u32 = 0xfffffffe;
pub(crate) const MAX_NCELLS: u32 = 4;
pub(crate) const CREATE_FLAG_NO_NAME_DEDUP: u32 = 1;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Property<'a> {
    pub(crate) offset: i32,
    pub(crate) name_offset: i32,
    pub(crate) data_offset: usize,
    pub(crate) data: &'a [u8],
}
