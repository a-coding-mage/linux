// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * x86 decoder sanity test - based on test_get_insn.c
 *
 * Copyright (C) IBM Corporation, 2009
 * Copyright (C) Hitachi, Ltd., 2011
 */
//! Exercise the bounded x86 decoder with reproducible random or supplied bytes.

// Both decoder test programs share helpers and the complete decoder API, but
// exercise different subsets of it.
#[allow(dead_code)]
mod insn_test_common;

use common::decoder::{Instruction, Mode, MAX_INSN_SIZE};
use insn_test_common as common;
use std::ffi::OsString;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::os::unix::ffi::OsStrExt;

struct Config {
    program: Vec<u8>,
    verbose: u32,
    mode: Mode,
    seed: u32,
    start: u64,
    end: u64,
    input: Option<Box<dyn BufRead>>,
}

fn usage(program: &[u8], error: Option<&str>) -> Vec<u8> {
    let mut output = Vec::new();
    if let Some(error) = error {
        output.extend_from_slice(program);
        output.extend_from_slice(format!(": Error: {error}\n\n").as_bytes());
    }
    output.extend_from_slice(b"Usage: ");
    output.extend_from_slice(program);
    output.extend_from_slice(
        b" [-y|-n|-v] [-s seed[,no]] [-m max] [-i input]\n\
\t-y\t64bit mode\n\
\t-n\t32bit mode\n\
\t-v\tVerbosity(-vv dumps any decoded result)\n\
\t-s\tGive a random seed (and iteration number)\n\
\t-m\tGive a maximum iteration number\n\
\t-i\tGive an input file with decoded binary\n",
    );
    output
}

fn parse(args: &[OsString]) -> Result<Config, Vec<u8>> {
    let program = common::program(args).as_bytes();
    let fail = |message| usage(program, Some(message));
    let mut config = Config {
        program: program.to_vec(),
        verbose: 0,
        mode: Mode::Bits32,
        seed: 0,
        start: 0,
        end: 10_000,
        input: None,
    };
    let mut set_seed = false;
    for option in common::Options::new(args, b"ynvs:m:i:") {
        let (option, value) = option.map_err(|mut error| {
            error.extend_from_slice(&usage(program, None));
            error
        })?;
        match option {
            b'y' => config.mode = Mode::Bits64,
            b'n' => config.mode = Mode::Bits32,
            b'v' => config.verbose = config.verbose.saturating_add(1),
            b'i' => {
                let path = value.expect("option requires an argument");
                config.input = Some(if path.as_bytes() == b"-" {
                    Box::new(BufReader::new(io::stdin()))
                } else {
                    Box::new(BufReader::new(
                        File::open(path).map_err(|_| fail("Failed to open input file"))?,
                    ))
                });
            }
            b's' => {
                let value = value.expect("option requires an argument");
                let mut bytes = value.as_bytes();
                let (seed, mut end) = common::unsigned(bytes, 0);
                config.seed = seed as u32;
                if bytes.get(end) == Some(&b',') {
                    bytes = &bytes[end + 1..];
                    (config.start, end) = common::unsigned(bytes, 0);
                }
                if end == 0 || end != bytes.len() {
                    return Err(fail("Failed to parse seed"));
                }
                set_seed = true;
            }
            b'm' => {
                let value = value.expect("option requires an argument");
                let bytes = value.as_bytes();
                let (number, end) = common::unsigned(bytes, 0);
                if end == 0 || end != bytes.len() {
                    return Err(fail("Failed to parse max_iter"));
                }
                config.end = number;
            }
            _ => unreachable!("only declared options are returned"),
        }
    }
    if config.end < config.start {
        return Err(fail("Max iteration number must be bigger than iter-num"));
    }
    if set_seed && config.input.is_some() {
        return Err(fail("Don't use input file (-i) with random seed (-s)"));
    }
    if !set_seed && config.input.is_none() {
        let mut seed = [0; 4];
        let read = File::open("/dev/urandom").and_then(|mut file| file.read(&mut seed));
        if !matches!(read, Ok(4)) {
            return Err(fail("Failed to open /dev/urandom"));
        }
        config.seed = u32::from_ne_bytes(seed);
    }
    Ok(config)
}

// Reproduce glibc's shared srand()/random() additive-feedback sequence without
// an FFI dependency, so old Linux-host seed/iteration reproductions still work.
struct Random {
    state: [u32; 31],
    front: usize,
    rear: usize,
}

impl Random {
    fn new(seed: u32) -> Self {
        let mut state = [0; 31];
        state[0] = if seed == 0 { 1 } else { seed };
        let mut word = state[0] as i32 as i64;
        for value in state.iter_mut().skip(1) {
            word = 16_807 * (word % 127_773) - 2_836 * (word / 127_773);
            if word < 0 {
                word += 2_147_483_647;
            }
            *value = word as u32;
        }
        let mut random = Self {
            state,
            front: 3,
            rear: 0,
        };
        for _ in 0..310 {
            random.next();
        }
        random
    }

    fn next(&mut self) -> u32 {
        let value = self.state[self.front].wrapping_add(self.state[self.rear]);
        self.state[self.front] = value;
        self.front = (self.front + 1) % 31;
        self.rear = (self.rear + 1) % 31;
        value >> 1
    }

    fn fill(&mut self, buffer: &mut [u8]) {
        let mut pairs = buffer.chunks_exact_mut(2);
        for pair in &mut pairs {
            pair.copy_from_slice(&(self.next() as u16).to_ne_bytes());
        }
        for byte in pairs.into_remainder() {
            *byte = self.next() as u8;
        }
    }
}

fn read_next(reader: &mut dyn BufRead, buffer: &mut [u8]) -> usize {
    // The C tool treats both a read error and a final unterminated fgets chunk
    // as end-of-input, and returns the last index rather than the byte count.
    let Some(line) = common::fgets(reader, 256).ok().flatten() else {
        return 0;
    };
    if line.eof {
        return 0;
    }
    let mut input = common::cstr(&line.bytes);
    for (index, byte) in buffer.iter_mut().enumerate() {
        let (value, end) = common::unsigned(input, 16);
        *byte = value as u8;
        input = &input[end..];
        if input.first() != Some(&b' ') {
            return index;
        }
    }
    buffer.len()
}

fn dump_stream(
    output: &mut Vec<u8>,
    message: &[u8],
    iteration: u64,
    insn: &Instruction<'_>,
    config: &Config,
) {
    output.extend_from_slice(message);
    output.extend_from_slice(b":\n");
    common::dump_insn(output, insn);
    output.extend_from_slice(b"You can reproduce this with below command(s);\n $ echo ");
    for byte in insn.bytes {
        output.extend_from_slice(format!(" {byte:02x}").as_bytes());
    }
    output.extend_from_slice(b" | ");
    output.extend_from_slice(&config.program);
    output.extend_from_slice(b" -i -\n");
    if config.input.is_none() {
        output.extend_from_slice(b"Or \n $ ");
        output.extend_from_slice(&config.program);
        output.extend_from_slice(format!(" -s 0x{:x},{iteration}\n", config.seed).as_bytes());
    }
}

fn run(mut config: Config, stdout: &mut dyn Write, stderr: &mut dyn Write) -> io::Result<i32> {
    let mut random = Random::new(config.seed);
    // The C tool leaves an initial short input's tail uninitialized. Initialize
    // it safely, while preserving the previous instruction's tail on later
    // short lines. The second half retains the original NOP stop bytes.
    let mut buffer = [0; MAX_INSN_SIZE * 2];
    buffer[MAX_INSN_SIZE..].fill(0x90);
    let (mut instructions, mut errors) = (0i32, 0i32);
    let mut output = Vec::new();
    for iteration in 0..config.end {
        let bytes = &mut buffer[..MAX_INSN_SIZE];
        if let Some(input) = &mut config.input {
            if read_next(input.as_mut(), bytes) == 0 {
                break;
            }
        } else {
            random.fill(bytes);
        }
        if iteration < config.start {
            continue;
        }
        let mut insn = Instruction::new(&buffer, config.mode);
        let result = insn.decode();
        output.clear();
        if insn.next == 0 || insn.next > MAX_INSN_SIZE {
            dump_stream(
                &mut output,
                b"Error: Found an access violation",
                iteration,
                &insn,
                &config,
            );
            stderr.write_all(&output)?;
            errors = errors.wrapping_add(1);
        } else if config.verbose > 0 && result.is_err() {
            dump_stream(
                &mut output,
                b"Info: Found an undecodable input",
                iteration,
                &insn,
                &config,
            );
            stdout.write_all(&output)?;
        } else if config.verbose >= 2 {
            common::dump_insn(&mut output, &insn);
            stdout.write_all(&output)?;
        }
        instructions = instructions.wrapping_add(1);
    }
    output.clear();
    output.extend_from_slice(b"  ");
    output.extend_from_slice(&config.program);
    let status = if errors == 0 { "success" } else { "failure" };
    let source = if config.input.is_some() {
        "given"
    } else {
        "random"
    };
    output.extend_from_slice(format!(": {status}: Decoded and checked {instructions} {source} instructions with {errors} errors (seed:0x{:x})\n", config.seed).as_bytes());
    if errors == 0 {
        stdout.write_all(&output)?;
    } else {
        stderr.write_all(&output)?;
    }
    stdout.flush()?;
    stderr.flush()?;
    Ok(i32::from(errors != 0))
}

fn main() {
    let args: Vec<_> = std::env::args_os().collect();
    let mut stdout = BufWriter::new(io::stdout().lock());
    let mut stderr = io::stderr().lock();
    let status = match parse(&args) {
        Ok(config) => match run(config, &mut stdout, &mut stderr) {
            Ok(status) => status,
            Err(error) => {
                let _ = stderr.write_all(&common::error(common::program(&args).as_bytes(), &error));
                1
            }
        },
        Err(message) => {
            let _ = stderr.write_all(&message);
            1
        }
    };
    std::process::exit(status);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
