// SPDX-License-Identifier: GPL-2.0-or-later
//! Native C-ABI entry points for the allocation-free x86 instruction decoder.
//!
//! The C structure remains owned by its caller. Only this boundary accesses
//! its raw pointers and unions; the decoder operates on owned field values and
//! a bounded reader. No borrowed slice spans possibly unread instruction bytes.

use core::ffi::{c_int, c_void};
use core::ptr::{addr_of, addr_of_mut};
use kernel::bindings;

#[path = "insn.rs"]
#[allow(dead_code)] // Host users also need the full set of inline header helpers.
mod decoder;

use decoder::{DecodeResult, Decoder, Field, Input};

// Keep the original cursor's provenance, including when callers have rebased
// kaddr independently. Creating this reader never validates or accesses input;
// cached stages must still work with stale or inaccessible instruction pointers.
struct NativeInput {
    state: *mut bindings::insn,
    overlaps_state: bool,
    cursor: *const u8,
    end: *const u8,
}

impl Input for NativeInput {
    fn before_read(&self, decoded: &Decoder<Self>) {
        if !self.overlaps_state {
            return;
        }
        // SAFETY: only a C entry point constructs this reader, with an
        // exclusive writable native state. Publishing earlier changes before
        // each read also handles input bytes inside that same C structure.
        // No Rust reference aliases the native structure or instruction bytes.
        unsafe { store(self.state, decoded) };
    }

    fn read(&self, offset: usize, destination: &mut [u8]) -> bool {
        let Some(address) = (self.cursor as usize).checked_add(offset) else {
            return false;
        };
        let Some(end) = address.checked_add(destination.len()) else {
            return false;
        };
        if address == 0 || end > self.end as usize {
            return false;
        }
        let pointer = self.cursor.wrapping_add(offset);
        for (index, byte) in destination.iter_mut().enumerate() {
            // SAFETY: the C entry point requires every actually visited byte
            // to be readable. The whole requested span was validated against
            // end_kaddr before any access, as in C's validate_next().
            *byte = unsafe { pointer.add(index).read() };
        }
        true
    }
}

// SAFETY contracts for these private helpers are supplied by their C callers:
// the state is aligned and initialized, and its nine fields contain C integers.
unsafe fn load_field(pointer: *const bindings::insn_field) -> Field {
    Field {
        // SAFETY: all four bytes of the native value/bytes union are initialized.
        bytes: unsafe { (*pointer).__bindgen_anon_1.bytes },
        // SAFETY: these are ordinary initialized unsigned-byte fields.
        got: unsafe { (*pointer).got != 0 },
        nbytes: unsafe { (*pointer).nbytes },
        got_written: false,
    }
}

unsafe fn store_field(pointer: *mut bindings::insn_field, field: &Field) {
    // SAFETY: write individual members, never the struct's trailing padding.
    unsafe {
        addr_of_mut!((*pointer).__bindgen_anon_1.bytes).write(field.bytes);
        addr_of_mut!((*pointer).nbytes).write(field.nbytes);
        if field.got_written {
            addr_of_mut!((*pointer).got).write(1);
        }
    }
}

unsafe fn load(pointer: *mut bindings::insn) -> Decoder<NativeInput> {
    // SAFETY: the caller's initialized C struct is readable. No references to
    // it or to instruction memory survive these raw, field-wise reads.
    unsafe {
        let cursor = (*pointer).next_byte;
        let length_bias = (cursor as usize).wrapping_sub((*pointer).kaddr as usize);
        // Normal kernel callers use a separate input buffer: defer their
        // writeback until the stage returns. Only an overlapping read window
        // needs the before-read publication path.
        let state_begin = pointer as usize;
        let state_end = state_begin.saturating_add(core::mem::size_of::<bindings::insn>());
        let overlaps_state = (cursor as usize) < state_end
            && (*pointer).end_kaddr as usize > state_begin
            && (cursor as usize) < (*pointer).end_kaddr as usize;
        Decoder {
            prefixes: load_field(addr_of!((*pointer).prefixes)),
            rex_prefix: load_field(addr_of!((*pointer).rex_prefix)),
            vex_prefix: load_field(addr_of!((*pointer).__bindgen_anon_1.vex_prefix)),
            opcode: load_field(addr_of!((*pointer).opcode)),
            modrm: load_field(addr_of!((*pointer).modrm)),
            sib: load_field(addr_of!((*pointer).sib)),
            displacement: load_field(addr_of!((*pointer).displacement)),
            immediate1: load_field(addr_of!((*pointer).__bindgen_anon_2.immediate1)),
            immediate2: load_field(addr_of!((*pointer).__bindgen_anon_3.immediate2)),
            emulate_prefix_size: (*pointer).emulate_prefix_size as usize,
            attr: (*pointer).attr,
            opnd_bytes: (*pointer).opnd_bytes,
            addr_bytes: (*pointer).addr_bytes,
            length: (*pointer).length,
            x86_64: (*pointer).x86_64 != 0,
            bytes: NativeInput {
                state: pointer,
                overlaps_state,
                cursor,
                end: (*pointer).end_kaddr,
            },
            next: 0,
            length_bias,
        }
    }
}

unsafe fn store(pointer: *mut bindings::insn, decoded: &Decoder<NativeInput>) {
    // SAFETY: field-wise stores preserve padding, the original kaddr/end_kaddr,
    // and raw x86_64/got bytes that the selected C stage did not assign.
    unsafe {
        store_field(addr_of_mut!((*pointer).prefixes), &decoded.prefixes);
        store_field(addr_of_mut!((*pointer).rex_prefix), &decoded.rex_prefix);
        store_field(
            addr_of_mut!((*pointer).__bindgen_anon_1.vex_prefix),
            &decoded.vex_prefix,
        );
        store_field(addr_of_mut!((*pointer).opcode), &decoded.opcode);
        store_field(addr_of_mut!((*pointer).modrm), &decoded.modrm);
        store_field(addr_of_mut!((*pointer).sib), &decoded.sib);
        store_field(addr_of_mut!((*pointer).displacement), &decoded.displacement);
        store_field(
            addr_of_mut!((*pointer).__bindgen_anon_2.immediate1),
            &decoded.immediate1,
        );
        store_field(
            addr_of_mut!((*pointer).__bindgen_anon_3.immediate2),
            &decoded.immediate2,
        );
        addr_of_mut!((*pointer).emulate_prefix_size).write(decoded.emulate_prefix_size as c_int);
        addr_of_mut!((*pointer).attr).write(decoded.attr);
        addr_of_mut!((*pointer).opnd_bytes).write(decoded.opnd_bytes);
        addr_of_mut!((*pointer).addr_bytes).write(decoded.addr_bytes);
        addr_of_mut!((*pointer).length).write(decoded.length);
        addr_of_mut!((*pointer).next_byte).write(decoded.bytes.cursor.wrapping_add(decoded.next));
    }
}

fn status(result: DecodeResult) -> c_int {
    result.err().map_or(0, |error| error as c_int)
}

/// Initialize the caller's native instruction structure, including padding.
///
/// # Safety
///
/// insn must point to a writable, properly aligned native struct insn. Its
/// kaddr is retained without being read; subsequent getters require only the
/// bytes they actually visit. Overlap follows C's state-write/read ordering.
#[no_mangle]
pub unsafe extern "C" fn insn_init(
    insn: *mut bindings::insn,
    kaddr: *const c_void,
    buf_len: c_int,
    x86_64: c_int,
) {
    let length = buf_len.min(decoder::MAX_INSN_SIZE as c_int);
    let input = kaddr.cast::<u8>();
    // SAFETY: the caller supplies the complete writable C object. Zero bytes
    // are valid for all native integer, pointer, and union fields.
    unsafe {
        insn.write_bytes(0, 1);
        addr_of_mut!((*insn).kaddr).write(input);
        addr_of_mut!((*insn).end_kaddr).write(input.wrapping_offset(length as isize));
        addr_of_mut!((*insn).next_byte).write(input);
        // C assigns the int to an unsigned char, but tests the original int
        // for the address width. For example, 256 means x86_64=0, addr_bytes=8.
        addr_of_mut!((*insn).x86_64).write(x86_64 as u8);
        addr_of_mut!((*insn).opnd_bytes).write(4);
        addr_of_mut!((*insn).addr_bytes).write(if x86_64 != 0 { 8 } else { 4 });
    }
}

macro_rules! getter {
    ($name:ident, $method:ident, $($field:tt)+) => {
        /// Decode through this field, retaining partial results on failure.
        ///
        /// # Safety
        ///
        /// insn must be a writable, aligned, initialized native struct insn,
        /// with exclusive caller access. Each instruction byte actually read
        /// must be readable and within its declared bound. State writes that
        /// overlap input affect later reads. A cached field accesses no input.
        #[no_mangle]
        pub unsafe extern "C" fn $name(insn: *mut bindings::insn) -> c_int {
            // SAFETY: the caller supplies the initialized cached flag. Check
            // it before loading any cursor or other state, exactly as in C.
            if unsafe { (*insn).$($field)+ } != 0 {
                return 0;
            }
            // SAFETY: the caller supplies the initialized state and readable
            // instruction bytes requested by the bounded reader.
            let mut decoded = unsafe { load(insn) };
            let result = status(decoded.$method());
            // SAFETY: preserve all successful and partial changes on failure.
            unsafe { store(insn, &decoded) };
            result
        }
    };
}

getter!(insn_get_prefixes, get_prefixes, prefixes.got);
getter!(insn_get_opcode, get_opcode, opcode.got);
getter!(insn_get_modrm, get_modrm, modrm.got);
getter!(insn_get_sib, get_sib, sib.got);
getter!(insn_get_displacement, get_displacement, displacement.got);
getter!(
    insn_get_immediate,
    get_immediate,
    __bindgen_anon_2.immediate1.got
);
getter!(insn_get_length, get_length, length);

/// Report RIP-relative addressing, decoding ModRM if it is not cached.
///
/// # Safety
///
/// insn and any actually visited instruction bytes must satisfy the same
/// initialized and exclusive contract as insn_get_modrm.
#[no_mangle]
pub unsafe extern "C" fn insn_rip_relative(insn: *mut bindings::insn) -> c_int {
    // SAFETY: the initialized mode is readable even if instruction memory has
    // become inaccessible. A non-64-bit instruction never needs more fields.
    if unsafe { (*insn).x86_64 } == 0 {
        return 0;
    }
    // SAFETY: insn_get_modrm implements its own no-read cached-field path.
    if unsafe { insn_get_modrm(insn) } != 0 {
        return 0;
    }
    // SAFETY: a successful stage leaves this native field initialized.
    let field = unsafe { load_field(addr_of!((*insn).modrm)) };
    c_int::from(field.nbytes != 0 && field.bytes[0] & 0xc7 == 5)
}

/// Initialize and completely decode an instruction in the selected C mode.
///
/// # Safety
///
/// insn must be a writable, aligned native structure. Only instruction bytes
/// actually visited need to be readable; state/input overlap is permitted.
#[no_mangle]
pub unsafe extern "C" fn insn_decode(
    insn: *mut bindings::insn,
    kaddr: *const c_void,
    buf_len: c_int,
    mode: bindings::insn_mode,
) -> c_int {
    let x86_64 = if mode == bindings::insn_mode::INSN_MODE_KERN {
        cfg!(CONFIG_X86_64)
    } else {
        mode == bindings::insn_mode::INSN_MODE_64
    };
    // SAFETY: the caller supplies the same native state/input contract as the
    // two component C entry points, including their staged partial results.
    unsafe {
        insn_init(insn, kaddr, buf_len, c_int::from(x86_64));
        let result = insn_get_length(insn);
        if result != 0 {
            return result;
        }
        if (*insn).opcode.got != 0
            && (*insn).modrm.got != 0
            && (*insn).sib.got != 0
            && (*insn).displacement.got != 0
            && (*insn).__bindgen_anon_2.immediate1.got != 0
        {
            0
        } else {
            -22 // EINVAL
        }
    }
}

/// Look up the primary opcode attributes.
#[no_mangle]
pub extern "C" fn inat_get_opcode_attribute(opcode: u8) -> u32 {
    decoder::inat::inat_get_opcode_attribute(opcode)
}

/// Map a legacy prefix byte to its last-prefix identifier.
#[no_mangle]
pub extern "C" fn inat_get_last_prefix_id(last_prefix: u8) -> c_int {
    c_int::from(decoder::inat::inat_get_last_prefix_id(last_prefix))
}

// Out-of-range signed prefix IDs are invalid only if a variant is consulted.
// Use an invalid byte index rather than truncating e.g. 256 to valid prefix 0.
fn prefix_id(prefix: c_int) -> u8 {
    u8::try_from(prefix).unwrap_or(u8::MAX)
}

/// Look up an escaped opcode, checking any selected variant table.
#[no_mangle]
pub extern "C" fn inat_get_escape_attribute(opcode: u8, prefix: c_int, escape: u32) -> u32 {
    decoder::inat::inat_get_escape_attribute(opcode, prefix_id(prefix), escape)
}

/// Look up a ModRM group, retaining its common attributes.
#[no_mangle]
pub extern "C" fn inat_get_group_attribute(modrm: u8, prefix: c_int, group: u32) -> u32 {
    decoder::inat::inat_get_group_attribute(modrm, prefix_id(prefix), group)
}

/// Look up an AVX/EVEX opcode in its map and prefix variant.
#[no_mangle]
pub extern "C" fn inat_get_avx_attribute(opcode: u8, map: u8, prefix: u8) -> u32 {
    decoder::inat::inat_get_avx_attribute(opcode, map, prefix)
}

/// Look up an AMD XOP opcode in its map.
#[no_mangle]
pub extern "C" fn inat_get_xop_attribute(opcode: u8, map: u8) -> u32 {
    decoder::inat::inat_get_xop_attribute(opcode, map)
}
