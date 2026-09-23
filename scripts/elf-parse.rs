// SPDX-License-Identifier: GPL-2.0-only

//! Checked, endian-independent ELF section access for host tools.

/// A section header, decoded independently of the host's word size.
#[derive(Clone, Copy)]
#[allow(dead_code)] // Different host tools consume different header fields.
pub(crate) struct Section {
    name: u32,
    pub(crate) index: usize,
    pub(crate) kind: u32,
    pub(crate) flags: u64,
    pub(crate) address: u64,
    pub(crate) offset: u64,
    pub(crate) size: u64,
    pub(crate) link: u32,
    pub(crate) info: u32,
    pub(crate) alignment: u64,
    pub(crate) entry_size: u64,
}

/// A decoded ELF symbol, with its original section index.
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(crate) struct Symbol {
    pub(crate) name: u32,
    pub(crate) value: u64,
    pub(crate) size: u64,
    pub(crate) kind: u8,
    pub(crate) binding: u8,
    pub(crate) visibility: u8,
    pub(crate) section: u16,
}

/// A relocation, including its explicit addend when present.
#[derive(Clone, Copy)]
#[allow(dead_code)]
pub(crate) struct Relocation {
    pub(crate) offset: u64,
    pub(crate) symbol: u32,
    pub(crate) kind: u32,
    pub(crate) addend: Option<i64>,
}

/// An ELF image whose section-header table has been checked for bounds.
pub(crate) struct ElfFile<'a> {
    data: &'a [u8],
    little_endian: bool,
    is_64: bool,
    section_offset: usize,
    section_size: usize,
    section_count: usize,
    string_index: usize,
}

fn bytes(data: &[u8], offset: usize, length: usize) -> Result<&[u8], String> {
    let end = offset.checked_add(length).ok_or("ELF offset overflow")?;
    data.get(offset..end)
        .ok_or_else(|| "truncated ELF file".into())
}

fn integer(data: &[u8], offset: usize, size: usize, little: bool) -> Result<u64, String> {
    let value = bytes(data, offset, size)?;
    Ok(if little {
        value
            .iter()
            .rev()
            .fold(0, |n, byte| (n << 8) | u64::from(*byte))
    } else {
        value.iter().fold(0, |n, byte| (n << 8) | u64::from(*byte))
    })
}

fn host_offset(value: u64) -> Result<usize, String> {
    value
        .try_into()
        .map_err(|_| "ELF offset exceeds host address space".into())
}

impl<'a> ElfFile<'a> {
    /// Parse an ELF image, accepting only the specified ELF file types.
    pub(crate) fn parse(data: &'a [u8], types: u32) -> Result<Self, String> {
        Self::parse_types(data, Some(types))
    }

    /// Parse metadata independently of the ELF object's file type.
    #[allow(dead_code)] // Only metadata-only consumers accept arbitrary types.
    pub(crate) fn parse_any(data: &'a [u8]) -> Result<Self, String> {
        Self::parse_types(data, None)
    }

    fn parse_types(data: &'a [u8], types: Option<u32>) -> Result<Self, String> {
        bytes(data, 0, 16)?;
        let little_endian = match data[5] {
            1 => true,
            2 => false,
            n => return Err(format!("unrecognized ELF data encoding {n}:")),
        };
        if &data[..4] != b"\x7fELF" || data[6] != 1 {
            return Err("unrecognized ELF file".into());
        }
        let read = |offset, size| integer(data, offset, size, little_endian);
        let kind = read(16, 2)?;
        if types.is_some_and(|types| kind >= 32 || types & (1 << kind) == 0) {
            return Err("Invalid ELF type file".into());
        }
        let is_64 = match data[4] {
            1 => false,
            2 => true,
            n => return Err(format!("unrecognized ELF class {n}")),
        };
        let (header_size, section_size, table_offset, sizes_offset) = if is_64 {
            (64, 64, 40, 52)
        } else {
            (52, 40, 32, 40)
        };
        bytes(data, 0, header_size)?;
        if read(sizes_offset, 2)? != header_size as u64
            || read(sizes_offset + 6, 2)? != section_size as u64
        {
            return Err("unrecognized ET_EXEC/ET_DYN file:".into());
        }
        let section_offset = host_offset(read(table_offset, if is_64 { 8 } else { 4 })?)?;
        let mut elf = Self {
            data,
            little_endian,
            is_64,
            section_offset,
            section_size,
            section_count: read(sizes_offset + 8, 2)? as usize,
            string_index: read(sizes_offset + 10, 2)? as usize,
        };
        if section_offset == 0 {
            if elf.section_count != 0 || elf.string_index != 0 {
                return Err("missing ELF section table:".into());
            }
            return Ok(elf);
        }
        // Extended section counts and string-table indices live in section 0.
        let first = elf.read_section(0)?;
        if elf.section_count == 0 {
            elf.section_count = host_offset(first.size)?;
        }
        if elf.string_index == 0xffff {
            elf.string_index = first.link as usize;
        }
        let table_size = elf
            .section_count
            .checked_mul(section_size)
            .ok_or("ELF section table size overflow:")?;
        bytes(data, section_offset, table_size)?;
        if elf.string_index >= elf.section_count {
            return Err("invalid ELF section name table index:".into());
        }
        Ok(elf)
    }

    fn read_section(&self, index: usize) -> Result<Section, String> {
        let start = index
            .checked_mul(self.section_size)
            .and_then(|offset| self.section_offset.checked_add(offset))
            .ok_or("ELF section offset overflow:")?;
        let header = bytes(self.data, start, self.section_size)?;
        let read = |offset, size| integer(header, offset, size, self.little_endian);
        let (offset, size, link, width) = if self.is_64 {
            (24, 32, 40, 8)
        } else {
            (16, 20, 24, 4)
        };
        Ok(Section {
            name: read(0, 4)? as u32,
            index,
            kind: read(4, 4)? as u32,
            flags: read(8, width)?,
            address: read(if self.is_64 { 16 } else { 12 }, width)?,
            offset: read(offset, width)?,
            size: read(size, width)?,
            link: read(link, 4)? as u32,
            info: read(link + 4, 4)? as u32,
            alignment: read(if self.is_64 { 48 } else { 32 }, width)?,
            entry_size: read(if self.is_64 { 56 } else { 36 }, width)?,
        })
    }

    /// Return a section's file contents. NOBITS sections have no stored data.
    pub(crate) fn section_data(&self, section: &Section) -> Result<&'a [u8], String> {
        if section.kind == 8 {
            return Ok(&[]);
        }
        bytes(
            self.data,
            host_offset(section.offset)?,
            host_offset(section.size)?,
        )
    }

    /// Find a named section, validating string-table references along the way.
    #[allow(dead_code)] // Some consumers decode the complete section list.
    pub(crate) fn find_section(&self, name: &[u8]) -> Result<Option<Section>, String> {
        if self.section_count == 0 || self.string_index == 0 {
            return Ok(None);
        }
        let strings = self.section_data(&self.read_section(self.string_index)?)?;
        for index in 0..self.section_count {
            let section = self.read_section(index)?;
            let tail = strings
                .get(section.name as usize..)
                .ok_or("invalid ELF section name offset:")?;
            let end = tail
                .iter()
                .position(|&byte| byte == 0)
                .ok_or("unterminated ELF section name:")?;
            if &tail[..end] == name {
                return Ok(Some(section));
            }
        }
        Ok(None)
    }
}

// These accessors are shared by table-editing tools; the tracepoint checker
// intentionally only needs section_data and find_section above.
#[allow(dead_code)]
impl<'a> ElfFile<'a> {
    /// Target architecture identifier from e_machine.
    pub(crate) fn machine(&self) -> Result<u16, String> {
        Ok(self.read_integer(18, 2)? as u16)
    }

    /// Width of target addresses in bytes.
    pub(crate) fn word_size(&self) -> usize {
        if self.is_64 {
            8
        } else {
            4
        }
    }

    /// Whether integers in this image use little-endian encoding.
    pub(crate) fn little_endian(&self) -> bool {
        self.little_endian
    }

    /// Read a target-endian integer at a checked file offset.
    pub(crate) fn read_integer(&self, offset: u64, width: usize) -> Result<u64, String> {
        if !matches!(width, 1 | 2 | 4 | 8) {
            return Err("invalid ELF integer width:".into());
        }
        integer(self.data, host_offset(offset)?, width, self.little_endian)
    }

    /// Return a checked section header by index.
    pub(crate) fn section(&self, index: usize) -> Result<Section, String> {
        if index >= self.section_count {
            return Err("invalid ELF section index:".into());
        }
        self.read_section(index)
    }

    /// Decode all section headers, including the null section.
    pub(crate) fn sections(&self) -> Result<Vec<Section>, String> {
        (0..self.section_count)
            .map(|index| self.section(index))
            .collect()
    }

    /// Decode a symbol table, validating its entry size and file extent.
    pub(crate) fn symbols(&self, table: &Section) -> Result<Vec<Symbol>, String> {
        let width = if self.is_64 { 24 } else { 16 };
        if table.entry_size < width || table.size % table.entry_size != 0 {
            return Err("invalid ELF symbol table entry size:".into());
        }
        let data = self.section_data(table)?;
        let stride = host_offset(table.entry_size)?;
        data.chunks_exact(stride)
            .map(|entry| {
                let read = |offset, width| integer(entry, offset, width, self.little_endian);
                Ok(Symbol {
                    name: read(0, 4)? as u32,
                    value: read(if self.is_64 { 8 } else { 4 }, self.word_size())?,
                    size: read(if self.is_64 { 16 } else { 8 }, self.word_size())?,
                    kind: entry[if self.is_64 { 4 } else { 12 }] & 0x0f,
                    binding: entry[if self.is_64 { 4 } else { 12 }] >> 4,
                    visibility: entry[if self.is_64 { 5 } else { 13 }] & 3,
                    section: read(if self.is_64 { 6 } else { 14 }, 2)? as u16,
                })
            })
            .collect()
    }

    /// Return a NUL-terminated string inside a checked string table.
    pub(crate) fn string(&self, table: &Section, offset: u32) -> Result<&'a [u8], String> {
        let tail = self
            .section_data(table)?
            .get(offset as usize..)
            .ok_or("invalid ELF string offset:")?;
        let end = tail
            .iter()
            .position(|&byte| byte == 0)
            .ok_or("unterminated ELF string:")?;
        Ok(&tail[..end])
    }

    /// Return a checked name from the section-name string table.
    pub(crate) fn section_name(&self, section: &Section) -> Result<&'a [u8], String> {
        self.string(&self.section(self.string_index)?, section.name)
    }

    /// Resolve SHN_XINDEX while retaining reserved indices outside the ordinary
    /// section-number space, as required by modpost's symbol search.
    pub(crate) fn symbol_section_index(
        &self,
        table: &Section,
        index: usize,
        symbol: &Symbol,
    ) -> Result<u32, String> {
        if symbol.section == 0xffff {
            let extended = self
                .sections()?
                .into_iter()
                .find(|section| section.kind == 18 && section.link as usize == table.index)
                .ok_or("missing ELF extended symbol indices:")?;
            return Ok(integer(
                self.section_data(&extended)?,
                index.checked_mul(4).ok_or("ELF symbol index overflow:")?,
                4,
                self.little_endian,
            )? as u32);
        }
        Ok(if symbol.section >= 0xff00 {
            u32::from(symbol.section).wrapping_sub(0x10000)
        } else {
            u32::from(symbol.section)
        })
    }

    /// Decode REL or RELA entries, including the MIPS64 mixed-width r_info.
    pub(crate) fn relocations(&self, section: &Section) -> Result<Vec<Relocation>, String> {
        let explicit = match section.kind {
            4 => true,
            9 => false,
            _ => return Err("invalid ELF relocation section type:".into()),
        };
        let word = self.word_size();
        let minimum = word * if explicit { 3 } else { 2 };
        let stride = host_offset(section.entry_size)?;
        if stride < minimum || section.size % section.entry_size != 0 {
            return Err("invalid ELF relocation entry size:".into());
        }
        let mips64 = self.is_64 && self.machine()? == 8;
        self.section_data(section)?
            .chunks_exact(stride)
            .map(|entry| {
                let read = |offset, width| integer(entry, offset, width, self.little_endian);
                let info = read(word, word)?;
                let (symbol, kind) = if mips64 {
                    (read(word, 4)? as u32, u32::from(entry[word + 7]))
                } else if self.is_64 {
                    ((info >> 32) as u32, info as u32)
                } else {
                    ((info >> 8) as u32, (info & 0xff) as u32)
                };
                Ok(Relocation {
                    offset: read(0, word)?,
                    symbol,
                    kind,
                    addend: if explicit {
                        let value = read(word * 2, word)?;
                        Some(if self.is_64 {
                            value as i64
                        } else {
                            i64::from(value as i32)
                        })
                    } else {
                        None
                    },
                })
            })
            .collect()
    }

    /// Resolve ordinary or SHN_XINDEX symbol indices, rejecting reserved values.
    pub(crate) fn symbol_section(
        &self,
        table: &Section,
        index: usize,
        symbol: &Symbol,
    ) -> Result<Section, String> {
        let section = if symbol.section == 0xffff {
            let extended = self
                .sections()?
                .into_iter()
                .find(|section| section.kind == 18 && section.link as usize == table.index)
                .ok_or("missing ELF extended symbol indices:")?;
            let data = self.section_data(&extended)?;
            integer(
                data,
                index.checked_mul(4).ok_or("ELF symbol index overflow:")?,
                4,
                self.little_endian,
            )? as usize
        } else {
            if symbol.section >= 0xff00 || symbol.section == 0 {
                return Err("invalid ELF symbol section index:".into());
            }
            symbol.section as usize
        };
        self.section(section)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
