// SPDX-License-Identifier: GPL-2.0
/* Converted from tools/testing/selftests/bpf/verifier/ringbuf.c */

/* C dependencies removed from executable Rust:
 * #include <linux/bpf.h>
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 */

#[repr(C)]
pub struct MapRingbuf {
    /* __uint(type, BPF_MAP_TYPE_RINGBUF); */
    pub type_: u32,
    /* __uint(max_entries, 4096); */
    pub max_entries: u32,
}

/* SEC(".maps") */
#[no_mangle]
pub static mut map_ringbuf: MapRingbuf = MapRingbuf {
    type_: BPF_MAP_TYPE_RINGBUF,
    max_entries: 4096,
};

extern "C" {
    static BPF_MAP_TYPE_RINGBUF: u32;

    fn bpf_ringbuf_reserve(...);
    fn bpf_ringbuf_submit(...);
    fn bpf_fib_lookup(...);
}

/* SEC("socket")
 * __description("ringbuf: invalid reservation offset 1")
 * __failure
 * __msg("R1 must have zero offset when passed to release func")
 * __failure_unpriv
 * __naked
 */
#[no_mangle]
pub unsafe extern "C" fn ringbuf_invalid_reservation_offset_1() {
    core::arch::asm!(
        "/* reserve 8 byte ringbuf memory */",
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r1 = {map_ringbuf} ll",
        "r2 = 8",
        "r3 = 0",
        "call {bpf_ringbuf_reserve}",
        "/* store a pointer to the reserved memory in R6 */",
        "r6 = r0",
        "/* check whether the reservation was successful */",
        "if r0 == 0 goto 1f",
        "/* spill R6(mem) into the stack */",
        "*(u64*)(r10 - 8) = r6",
        "/* fill it back in R7 */",
        "r7 = *(u64*)(r10 - 8)",
        "/* should be able to access *(R7) = 0 */",
        "r1 = 0",
        "*(u64*)(r7 + 0) = r1",
        "/* submit the reserved ringbuf memory */",
        "r1 = r7",
        "/* add invalid offset to reserved ringbuf memory */",
        "r1 += 0xcafe",
        "r2 = 0",
        "call {bpf_ringbuf_submit}",
        "1:",
        "r0 = 0",
        "exit",
        bpf_ringbuf_reserve = sym bpf_ringbuf_reserve,
        bpf_ringbuf_submit = sym bpf_ringbuf_submit,
        map_ringbuf = sym map_ringbuf,
        options(noreturn)
    );
}

/* SEC("socket")
 * __description("ringbuf: invalid reservation offset 2")
 * __failure
 * __msg("R7 min value is outside of the allowed memory range")
 * __failure_unpriv
 * __naked
 */
#[no_mangle]
pub unsafe extern "C" fn ringbuf_invalid_reservation_offset_2() {
    core::arch::asm!(
        "/* reserve 8 byte ringbuf memory */",
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r1 = {map_ringbuf} ll",
        "r2 = 8",
        "r3 = 0",
        "call {bpf_ringbuf_reserve}",
        "/* store a pointer to the reserved memory in R6 */",
        "r6 = r0",
        "/* check whether the reservation was successful */",
        "if r0 == 0 goto 1f",
        "/* spill R6(mem) into the stack */",
        "*(u64*)(r10 - 8) = r6",
        "/* fill it back in R7 */",
        "r7 = *(u64*)(r10 - 8)",
        "/* add invalid offset to reserved ringbuf memory */",
        "r7 += 0xcafe",
        "/* should be able to access *(R7) = 0 */",
        "r1 = 0",
        "*(u64*)(r7 + 0) = r1",
        "/* submit the reserved ringbuf memory */",
        "r1 = r7",
        "r2 = 0",
        "call {bpf_ringbuf_submit}",
        "1:",
        "r0 = 0",
        "exit",
        bpf_ringbuf_reserve = sym bpf_ringbuf_reserve,
        bpf_ringbuf_submit = sym bpf_ringbuf_submit,
        map_ringbuf = sym map_ringbuf,
        options(noreturn)
    );
}

/* SEC("xdp")
 * __description("ringbuf: check passing rb mem to helpers")
 * __success
 * __retval(0)
 * __naked
 */
#[no_mangle]
pub unsafe extern "C" fn passing_rb_mem_to_helpers() {
    core::arch::asm!(
        "r6 = r1",
        "/* reserve 8 byte ringbuf memory */",
        "r1 = 0",
        "*(u64*)(r10 - 8) = r1",
        "r1 = {map_ringbuf} ll",
        "r2 = 8",
        "r3 = 0",
        "call {bpf_ringbuf_reserve}",
        "r7 = r0",
        "/* check whether the reservation was successful */",
        "if r0 != 0 goto 1f",
        "exit",
        "1:",
        "/* pass allocated ring buffer memory to fib lookup */",
        "r1 = r6",
        "r2 = r0",
        "r3 = 8",
        "r4 = 0",
        "call {bpf_fib_lookup}",
        "/* submit the ringbuf memory */",
        "r1 = r7",
        "r2 = 0",
        "call {bpf_ringbuf_submit}",
        "r0 = 0",
        "exit",
        bpf_fib_lookup = sym bpf_fib_lookup,
        bpf_ringbuf_reserve = sym bpf_ringbuf_reserve,
        bpf_ringbuf_submit = sym bpf_ringbuf_submit,
        map_ringbuf = sym map_ringbuf,
        options(noreturn)
    );
}

/* SEC("license") */
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";
