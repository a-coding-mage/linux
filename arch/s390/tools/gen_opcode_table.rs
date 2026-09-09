/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Generate opcode table initializers for the in-kernel disassembler.
 *
 *    Copyright IBM Corp. 2017
 *
 */

const STRING_SIZE_MAX: usize = 20;

struct InsnType {
    byte: u8,
    mask: u8,
    format: &'static [&'static str],
}

struct Insn {
    type_: &'static InsnType,
    opcode: String,
    name: String,
    upper: String,
    format: String,
    name_len: usize,
}

struct InsnGroup {
    type_: &'static InsnType,
    offset: i32,
    count: i32,
    opcode: String,
}

struct InsnFormat {
    format: String,
    type_: i32,
}

struct GenOpcode {
    insn: Vec<Insn>,
    nr: i32,
    group: Vec<InsnGroup>,
    nr_groups: i32,
}

/*
 * Table of instruction format types. Each opcode is defined with
 * at least one byte (two nibbles), three nibbles, or two bytes (four
 * nibbles).
 * The byte member of each instruction format type entry defines
 * within which byte of an instruction the third (and fourth) nibble
 * of an opcode can be found. The mask member is the and-mask that
 * needs to be applied on this byte in order to get the third (and
 * fourth) nibble of the opcode.
 * The format array defines all instruction formats (as defined in the
 * Principles of Operation) which have the same position of the opcode
 * nibbles.
 * A special case are instruction formats with 1-byte opcodes. In this
 * case the byte member always is zero, so that the mask is applied on
 * the (only) byte that contains the opcode.
 */
static INSN_TYPE_TABLE: [InsnType; 4] = [
    InsnType { byte: 0, mask: 0xff, format: &["MII", "RR", "RS", "RSI", "RX", "SI", "SMI", "SS"] },
    InsnType { byte: 1, mask: 0x0f, format: &["RI", "RIL", "SSF"] },
    InsnType { byte: 1, mask: 0xff, format: &["E", "IE", "RRE", "RRF", "RRR", "S", "SIL", "SSE"] },
    InsnType { byte: 5, mask: 0xff, format: &["RIE", "RIS", "RRS", "RSE", "RSL", "RSY", "RXE", "RXF", "RXY", "SIY", "VRI", "VRR", "VRS", "VRV", "VRX", "VSI"] },
];

fn insn_format_to_type(format: &str) -> &'static InsnType {
    let base_format = format.split('_').next().unwrap_or(format);
    for insn_type in &INSN_TYPE_TABLE {
        for entry in insn_type.format {
            if *entry == base_format {
                return insn_type;
            }
        }
    }
    std::process::exit(1);
}

fn read_instructions(desc: &mut GenOpcode) {
    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    for line in input.split_whitespace().collect::<Vec<_>>().chunks(3) {
        if line.len() != 3 {
            std::process::exit(1);
        }
        let opcode = line[0].to_string();
        let name = line[1].to_string();
        let format = line[2].to_string();
        let type_ = insn_format_to_type(&format);
        let name_len = name.len();
        let upper = name.to_uppercase();
        desc.nr += 1;
        desc.insn.push(Insn { type_, opcode, name, upper, format, name_len });
    }
}

fn print_formats(desc: &mut GenOpcode) {
    desc.insn.sort_by(|a, b| a.format.cmp(&b.format));
    let mut format = String::new();
    let mut count = 0;
    println!("enum {{");
    for insn in &desc.insn {
        if format == insn.format { continue; }
        count += 1;
        format = insn.format.clone();
        println!("\tINSTR_{},", format);
    }
    println!("}}; /* {} */\n", count);
}

fn print_insn_name(name: &str) {
    print!("{{");
    for byte in name.bytes() { print!(" '{}',", byte as char); }
    print!(" }}");
}

fn print_long_insn(desc: &mut GenOpcode) {
    desc.insn.sort_by(|a, b| a.name.cmp(&b.name));
    let count = desc.insn.iter().filter(|insn| insn.name_len >= 6).count();
    println!("enum {{");
    for insn in &desc.insn {
        if insn.name_len >= 6 { println!("\tLONG_INSN_{},", insn.upper); }
    }
    println!("}}; /* {} */\n", count);
    println!("#define LONG_INSN_INITIALIZER {{ \\");
    for insn in &desc.insn {
        if insn.name_len < 6 { continue; }
        print!("\t[LONG_INSN_{}] = ", insn.upper);
        print_insn_name(&insn.name);
        println!(", \\");
    }
    println!("}}\n");
}

fn print_opcode(insn: &Insn, nr: i32) {
    let opcode = if insn.type_.byte != 0 { &insn.opcode[2..] } else { &insn.opcode };
    print!("\t[{nr:4}] = {{ .opfrag = 0x{opcode}, .format = INSTR_{}, ", insn.format);
    if insn.name_len < 6 {
        print!(".name =  ");
        print_insn_name(&insn.name);
    } else {
        print!(".offset = LONG_INSN_{}", insn.upper);
    }
    println!(" }}, \\");
}

fn add_to_group(desc: &mut GenOpcode, insn: &Insn, offset: i32) {
    if let Some(group) = desc.group.last_mut() {
        if group.opcode[..2] == insn.opcode[..2] || group.type_.byte == 0 {
            group.count += 1;
            return;
        }
    }
    desc.nr_groups += 1;
    desc.group.push(InsnGroup { opcode: insn.opcode[..2].to_string(), type_: insn.type_, offset, count: 1 });
}

fn print_opcode_table(desc: &mut GenOpcode) {
    desc.insn.sort_by(|a, b| a.opcode.cmp(&b.opcode));
    println!("#define OPCODE_TABLE_INITIALIZER {{ \\");
    let mut offset = 0;
    let mut opcode = String::new();
    for i in 0..desc.insn.len() {
        if desc.insn[i].type_.byte == 0 { continue; }
        let insn = &desc.insn[i];
        add_to_group(desc, insn, offset);
        if opcode != insn.opcode[..2] {
            opcode = insn.opcode[..2].to_string();
            println!("\t/* {:.2} */ \\", opcode);
        }
        print_opcode(insn, offset);
        offset += 1;
    }
    println!("\t/* 1-byte opcode instructions */ \\");
    for i in 0..desc.insn.len() {
        if desc.insn[i].type_.byte != 0 { continue; }
        let insn = &desc.insn[i];
        add_to_group(desc, insn, offset);
        print_opcode(insn, offset);
        offset += 1;
    }
    println!("}}\n");
}

fn print_opcode_table_offsets(desc: &GenOpcode) {
    println!("#define OPCODE_OFFSET_INITIALIZER {{ \\");
    for group in &desc.group {
        println!("\t{{ .opcode = 0x{}, .mask = 0x{:02x}, .byte = {}, .offset = {}, .count = {} }}, \\", group.opcode, group.type_.mask, group.type_.byte, group.offset, group.count);
    }
    println!("}}\n");
}

fn main() {
    let mut desc = GenOpcode { insn: Vec::new(), nr: 0, group: Vec::new(), nr_groups: 0 };
    read_instructions(&mut desc);
    println!("#ifndef __S390_GENERATED_DIS_DEFS_H__");
    println!("#define __S390_GENERATED_DIS_DEFS_H__");
    println!("/*");
    println!(" * DO NOT MODIFY.");
    println!(" *");
    println!(" * This file was generated by gen_opcode_table.c");
    println!(" */\n");
    print_formats(&mut desc);
    print_long_insn(&mut desc);
    print_opcode_table(&mut desc);
    print_opcode_table_offsets(&desc);
    println!("#endif");
    std::process::exit(0);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
