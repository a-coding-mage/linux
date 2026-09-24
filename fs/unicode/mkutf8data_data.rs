// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2014 SGI.
//! Read, expand and UTF-8 encode the Unicode Character Database mappings.

use super::io::{self, Result};
use super::model::{encode, Database, HANGUL, LIMIT};
use std::ffi::{CStr, CString};

pub(super) struct Inputs {
    pub age: CString,
    pub ccc: CString,
    pub prop: CString,
    pub data: CString,
    pub fold: CString,
    pub norm: CString,
    pub test: CString,
    pub output: CString,
    pub verbose: i32,
}

impl Default for Inputs {
    fn default() -> Self {
        Self {
            age: c"DerivedAge.txt".into(),
            ccc: c"DerivedCombiningClass.txt".into(),
            prop: c"DerivedCoreProperties.txt".into(),
            data: c"UnicodeData.txt".into(),
            fold: c"CaseFolding.txt".into(),
            norm: c"NormalizationCorrections.txt".into(),
            test: c"NormalizationTest.txt".into(),
            output: c"utf8data.c".into(),
            verbose: 0,
        }
    }
}

fn log(verbose: i32, text: String) {
    if verbose > 0 {
        crate::output(text.as_bytes());
    }
}
fn valid_age(a: [u32; 3]) -> bool {
    a[0] <= 65535 && a[1] <= 255 && a[2] <= 255
}
fn age(a: [u32; 3]) -> u32 {
    (a[0] << 16) | (a[1] << 8) | a[2]
}

fn age_header(line: &CStr) -> Option<([u32; 3], bool)> {
    let (n, [a, b, c, _]) = io::numbers(line, c"# Age=V%d_%d_%d");
    if n == 3 {
        return Some(([a, b, c], true));
    }
    let (n, [a, b, _, _]) = io::numbers(line, c"# Age=V%d_%d");
    (n == 2).then_some(([a, b, 0], false))
}

fn age_init(input: &Inputs, db: &mut Database) -> Result<()> {
    let mut lines = io::read(&input.age, input.verbose)?;
    let mut count = 0;
    for line in lines.by_ref() {
        if let Some(([a, b, c], three)) = age_header(&line) {
            count += 1;
            log(
                input.verbose - 1,
                if three {
                    format!(" Age V{}_{}_{}\n", a as i32, b as i32, c as i32)
                } else {
                    format!(" Age V{}_{}\n", a as i32, b as i32)
                },
            );
            if !valid_age([a, b, c]) {
                return Err(io::line_error(&input.age, &line));
            }
        }
    }
    log(input.verbose - 1, format!("{count} age entries\n"));
    if count == 0 || count > 255 {
        return Err(io::file_error(&input.age));
    }
    db.ages.push(0);
    let mut count = 0i32;
    lines.rewind();
    for line in lines {
        if let Some(([a, b, c], three)) = age_header(&line) {
            db.ages.push(age([a, b, c]));
            let gen = db.ages.len() - 1;
            log(
                input.verbose - 1,
                if three {
                    format!(" Age V{}_{}_{} = gen {gen}\n", a as i32, b as i32, c as i32)
                } else {
                    format!(" Age V{}_{} = {gen}\n", a as i32, b as i32)
                },
            );
            if !valid_age([a, b, c]) {
                return Err(io::line_error(&input.age, &line));
            }
            continue;
        }
        let (n, [first, last, _, _]) = io::numbers(&line, c"%X..%X ; %d.%d #");
        let gen = db.ages.len() as i32 - 1;
        if n == 4 {
            if first as usize >= LIMIT || last as usize >= LIMIT {
                return Err(io::line_error(&input.age, &line));
            }
            for code in first..=last {
                db.records[code as usize].generation = gen;
            }
            count = count.wrapping_add(1u32.wrapping_add(last).wrapping_sub(first) as i32);
            log(
                input.verbose - 1,
                format!("  {first:X}..{last:X} gen {gen}\n"),
            );
        } else {
            let (n, [code, _, _, _]) = io::numbers(&line, c"%X ; %d.%d #");
            if n != 3 {
                continue;
            }
            if code as usize >= LIMIT {
                return Err(io::line_error(&input.age, &line));
            }
            db.records[code as usize].generation = gen;
            count = count.wrapping_add(1);
            log(input.verbose - 1, format!("  {code:X} gen {gen}\n"));
        }
    }
    db.max_age = *db.ages.last().expect("validated ages");
    log(
        input.verbose - 1,
        " Removing surrogate block D800..DFFF\n".into(),
    );
    for record in &mut db.records[0xd800..=0xdfff] {
        record.generation = -1;
    }
    found(input.verbose, count, &input.age)
}

fn found(verbose: i32, count: i32, path: &CStr) -> Result<()> {
    log(verbose, format!("Found {count} entries\n"));
    if count == 0 {
        Err(io::file_error(path))
    } else {
        Ok(())
    }
}

fn ccc_init(input: &Inputs, db: &mut Database) -> Result<()> {
    let mut count = 0i32;
    for line in io::read(&input.ccc, input.verbose)? {
        let (n, [first, last, value, _]) = io::numbers(&line, c"%X..%X ; %d #");
        if n == 3 {
            if first as usize >= LIMIT || last as usize >= LIMIT {
                return Err(io::line_error(&input.ccc, &line));
            }
            for code in first..=last {
                db.records[code as usize].ccc = value as i32;
                count = count.wrapping_add(1);
            }
            log(
                input.verbose - 1,
                format!(" {first:X}..{last:X} ccc {}\n", value as i32),
            );
        } else {
            let (n, [code, value, _, _]) = io::numbers(&line, c"%X ; %d #");
            if n != 2 {
                continue;
            }
            if code as usize >= LIMIT {
                return Err(io::line_error(&input.ccc, &line));
            }
            db.records[code as usize].ccc = value as i32;
            count = count.wrapping_add(1);
            log(
                input.verbose - 1,
                format!(" {code:X} ccc {}\n", value as i32),
            );
        }
    }
    found(input.verbose, count, &input.ccc)
}

fn print_mapping(verbose: i32, code: u32, mapping: &[u32]) {
    if verbose <= 1 {
        return;
    }
    let mut text = format!(" {code:X} ->");
    for code in mapping {
        text.push_str(&format!(" {code:X}"));
    }
    text.push('\n');
    crate::output(text.as_bytes());
}

fn nfdi_init(input: &Inputs, db: &mut Database) -> Result<()> {
    const IGNORED: [&[u8]; 16] = [
        b"font",
        b"noBreak",
        b"initial",
        b"medial",
        b"final",
        b"isolated",
        b"circle",
        b"super",
        b"sub",
        b"vertical",
        b"wide",
        b"narrow",
        b"small",
        b"square",
        b"fraction",
        b"compat",
    ];
    let mut count = 0;
    for line in io::read(&input.data, input.verbose)? {
        let Some((code, text)) = io::decomposition(&line) else {
            continue;
        };
        if code as usize >= LIMIT {
            return Err(io::line_error(&input.data, &line));
        }
        let mut mapping = text.to_bytes();
        if mapping.first() == Some(&b'<') {
            let Some(end) = mapping.iter().position(|&b| b == b'>') else {
                return Err(io::line_error(&input.data, &line));
            };
            if IGNORED.contains(&&mapping[1..end]) {
                continue;
            }
            mapping = &mapping[end + 1..];
        }
        let mapping = io::mapping(mapping).ok_or_else(|| io::line_error(&input.data, &line))?;
        print_mapping(input.verbose, code, &mapping);
        db.records[code as usize].utf32_nfdi = Some(mapping);
        count += 1;
    }
    found(input.verbose, count, &input.data)
}

fn casefold_init(input: &Inputs, db: &mut Database) -> Result<()> {
    let mut count = 0;
    for line in io::read(&input.fold, input.verbose)? {
        let Some((code, status, text)) = io::casefold(&line) else {
            continue;
        };
        if code as usize >= LIMIT {
            return Err(io::line_error(&input.fold, &line));
        }
        if status != b'C' && status != b'F' {
            continue;
        }
        let mut mapping = text.to_bytes();
        if mapping.first() == Some(&b'<') {
            let Some(end) = mapping.iter().position(|&b| b == b' ') else {
                return Err(io::line_error(&input.fold, &line));
            };
            mapping = &mapping[end + 1..];
        }
        let mapping = io::mapping(mapping).ok_or_else(|| io::line_error(&input.fold, &line))?;
        print_mapping(input.verbose, code, &mapping);
        db.records[code as usize].utf32_cf = Some(mapping);
        count += 1;
    }
    found(input.verbose, count, &input.fold)
}

fn ignore_init(input: &Inputs, db: &mut Database) -> Result<()> {
    let mut count = 0;
    for line in io::read(&input.prop, input.verbose)? {
        let range = io::property(&line, true);
        let is_range = range.is_some();
        let Some((first, last, property)) = range.or_else(|| io::property(&line, false)) else {
            continue;
        };
        if property.to_bytes() != b"Default_Ignorable_Code_Point" {
            continue;
        }
        if first as usize >= LIMIT || last as usize >= LIMIT {
            return Err(io::line_error(&input.prop, &line));
        }
        for code in first..=last {
            db.records[code as usize].utf32_nfdi = Some(Vec::new());
            db.records[code as usize].utf32_cf = Some(Vec::new());
            count += 1;
        }
        log(
            input.verbose - 1,
            if is_range {
                format!(" {first:X}..{last:X} Default_Ignorable_Code_Point\n")
            } else {
                format!(" {first:X} Default_Ignorable_Code_Point\n")
            },
        );
    }
    found(input.verbose, count, &input.prop)
}

fn corrections_init(input: &Inputs, db: &mut Database) -> Result<()> {
    let mut lines = io::read(&input.norm, input.verbose)?;
    // The C validation pass precedes every mapping and verbose entry.
    for line in lines.by_ref() {
        if let Some((code, _, _, version)) = io::correction(&line) {
            if code as usize >= LIMIT || !valid_age(version) {
                return Err(io::line_error(&input.norm, &line));
            }
        }
    }
    let mut count = 0;
    lines.rewind();
    for line in lines {
        let Some((code, old, new, version)) = io::correction(&line) else {
            continue;
        };
        let mut record = db.records[code as usize].clone();
        record.correction = age(version);
        record.utf32_nfdi =
            Some(io::mapping(old.to_bytes()).ok_or_else(|| io::line_error(&input.norm, &line))?);
        db.records.push(record);
        if input.verbose > 1 {
            crate::output(format!(" {code:X} -> ").as_bytes());
            crate::output(old.to_bytes());
            crate::output(b" -> ");
            crate::output(new.to_bytes());
            crate::output(format!(" V{}_{}_{}\n", version[0], version[1], version[2]).as_bytes());
        }
        count += 1;
    }
    found(input.verbose, count, &input.norm)
}

fn hangul_decompose(input: &Inputs, db: &mut Database) {
    log(input.verbose, "Decomposing hangul\n".into());
    for code in 0xac00..=0xd7a3 {
        let index = code - 0xac00;
        let mut mapping = vec![0x1100 + index / 588, 0x1161 + (index % 588) / 28];
        if index % 28 != 0 {
            mapping.push(0x11a7 + index % 28);
        }
        let record = &mut db.records[code as usize];
        assert!(record.utf32_nfdi.is_none() && record.utf32_cf.is_none());
        record.utf32_nfdi = Some(mapping.clone());
        record.utf32_cf = Some(mapping.clone());
        record.nfdi = Some(vec![HANGUL]);
        print_mapping(input.verbose, code, &mapping);
    }
    log(input.verbose, "Created 11172 entries\n".into());
}

fn decompose(input: &Inputs, db: &mut Database, casefold: bool) -> Result<()> {
    let name = if casefold { "nfdicf" } else { "nfdi" };
    log(input.verbose, format!("Decomposing {name}\n"));
    let mut count = 0;
    for code in 0..LIMIT {
        let mapping = if casefold {
            &db.records[code].utf32_cf
        } else {
            &db.records[code].utf32_nfdi
        };
        let Some(mut mapping) = mapping.clone() else {
            continue;
        };
        // Invalid cyclic input made the C loop forever or overwrite its fixed
        // stack buffer. Track states so such input gets a finite, safe error.
        let mut seen = std::collections::HashSet::new();
        loop {
            if !seen.insert(mapping.clone()) {
                return Err(
                    format!("Error: cyclic {name} decomposition at {code:X}\n").into_bytes()
                );
            }
            let mut expanded = Vec::new();
            let mut changed = false;
            for &point in &mapping {
                let replacement = if casefold {
                    &db.records[point as usize].utf32_cf
                } else {
                    &db.records[point as usize].utf32_nfdi
                };
                if let Some(replacement) = replacement {
                    expanded.extend(replacement);
                    changed = true;
                } else {
                    expanded.push(point);
                }
            }
            if !changed {
                break;
            }
            // Unicode guarantees at most18codepoints; this is the original C
            // stack-buffer capacity, not a reduced supported Unicode domain.
            if expanded.len() > 18 {
                return Err(
                    format!("Error: oversized {name} decomposition at {code:X}\n").into_bytes(),
                );
            }
            mapping = expanded;
        }
        if casefold {
            db.records[code].utf32_cf = Some(mapping.clone());
        } else {
            db.records[code].utf32_nfdi = Some(mapping.clone());
            if db.records[code].utf32_cf.is_none() {
                db.records[code].utf32_cf = Some(mapping.clone());
            }
        }
        print_mapping(input.verbose, code as u32, &mapping);
        count += 1;
    }
    log(input.verbose, format!("Processed {count} entries\n"));
    Ok(())
}

fn utf8_init(db: &mut Database) {
    for record in &mut db.records {
        if record.nfdi.is_some() {
            assert_eq!(record.nfdi.as_deref(), Some([HANGUL].as_slice()));
            continue;
        }
        let convert = |mapping: &[u32]| {
            let mut out = Vec::new();
            for &code in mapping {
                encode(code, &mut out);
            }
            out
        };
        record.nfdi = record.utf32_nfdi.as_deref().map(convert);
        let cf = record.utf32_cf.as_deref().map(convert);
        if cf != record.nfdi {
            record.cf = cf;
        }
    }
}

pub(super) fn load(input: &Inputs) -> Result<Database> {
    let mut db = Database::new();
    age_init(input, &mut db)?;
    ccc_init(input, &mut db)?;
    nfdi_init(input, &mut db)?;
    casefold_init(input, &mut db)?;
    ignore_init(input, &mut db)?;
    corrections_init(input, &mut db)?;
    hangul_decompose(input, &mut db);
    decompose(input, &mut db, false)?;
    decompose(input, &mut db, true)?;
    utf8_init(&mut db);
    Ok(db)
}
