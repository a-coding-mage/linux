// Translated from testing/selftests/bpf/progs/btf__core_reloc_bitfields.c.
// Depends on the Rust equivalent of "core_reloc_types.h" for `core_reloc_bitfields`.

#[no_mangle]
pub extern "C" fn f(x: core_reloc_bitfields) {
    let _ = x;
}
