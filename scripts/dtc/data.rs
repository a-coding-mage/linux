// SPDX-License-Identifier: GPL-2.0-or-later
/* (C) Copyright David Gibson <dwg@au1.ibm.com>, IBM Corporation. 2005. */

use crate::dtc_header::{Data, Marker, MarkerKind};

impl Data {
    pub(crate) fn marker(&mut self, kind: MarkerKind, reference: Option<Vec<u8>>) {
        self.markers.push(Marker {
            kind,
            offset: self.bytes.len(),
            reference,
        });
    }

    pub(crate) fn merge(&mut self, mut other: Self) {
        for marker in &mut other.markers {
            marker.offset += self.bytes.len();
        }
        self.bytes.append(&mut other.bytes);
        self.markers.append(&mut other.markers);
    }

    pub(crate) fn append_integer(&mut self, value: u64, bits: usize) {
        assert!(matches!(bits, 8 | 16 | 32 | 64));
        self.bytes
            .extend_from_slice(&value.to_be_bytes()[8 - bits / 8..]);
    }

    pub(crate) fn insert_at_marker(&mut self, index: usize, bytes: &[u8]) {
        let offset = self.markers[index].offset;
        self.bytes.splice(offset..offset, bytes.iter().copied());
        for marker in &mut self.markers[index + 1..] {
            marker.offset += bytes.len();
        }
    }

    #[allow(dead_code)] // The complete data API also serves out-of-tree DTC users.
    pub(crate) fn insert_data(&mut self, index: usize, other: &Self) {
        let offset = self.markers[index].offset;
        self.insert_at_marker(index, &other.bytes);
        let markers = other.markers.iter().cloned().map(|mut marker| {
            marker.offset += offset;
            marker
        });
        self.markers.splice(index + 1..index + 1, markers);
    }

    pub(crate) fn is_one_string(&self) -> bool {
        self.bytes.last() == Some(&0) && !self.bytes[..self.bytes.len() - 1].contains(&0)
    }

    pub(crate) fn escape_string(input: &[u8]) -> Result<Self, Vec<u8>> {
        let mut data = Self::default();
        data.marker(MarkerKind::String, None);
        let mut index = 0;
        while let Some(&byte) = input.get(index) {
            index += 1;
            if byte != b'\\' {
                data.bytes.push(byte);
                continue;
            }
            let escaped = input.get(index).copied().unwrap_or(0);
            index += 1;
            let value = match escaped {
                b'a' => 7,
                b'b' => 8,
                b't' => 9,
                b'n' => 10,
                b'v' => 11,
                b'f' => 12,
                b'r' => 13,
                b'0'..=b'7' | b'x' => {
                    let radix = if escaped == b'x' {
                        16
                    } else {
                        index -= 1;
                        8
                    };
                    let max = if radix == 16 { 2 } else { 3 };
                    let mut value = 0u16;
                    let start = index;
                    while index < input.len() && index - start < max {
                        let Some(digit) = (input[index] as char).to_digit(radix) else {
                            break;
                        };
                        value = value * radix as u16 + digit as u16;
                        index += 1;
                    }
                    if index == start {
                        return Err(b"\\x used with no following hex digits\n".to_vec());
                    }
                    value as u8
                }
                _ => escaped,
            };
            data.bytes.push(value);
        }
        data.bytes.push(0);
        Ok(data)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
