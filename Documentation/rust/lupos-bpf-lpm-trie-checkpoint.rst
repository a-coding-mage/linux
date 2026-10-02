Lupos BPF LPM trie source checkpoint
==================================

This is an UNBUILT source reconstruction for Lupos issue #34 (master #28).
It is not accepted runtime parity and cannot close the issue.  The original
C provider and original C tests are retained.  CONFIG_RUST_BPF_LPM_TRIE is
default-off and depends on BPF_SYSCALL and RUST.

Source provenance
-----------------

The seven-file v3 candidate was reconstructed against
2b54ad9f34b04db3801ed2a938b3f841edea27f8.  Its sealed source.patch SHA-256 is
59f2f8f9e24028a25377dd6c9d2ff2d787da8a357df483b38957713f5075cd52.
This checkpoint is rebased onto 3ac6d2749d1d278f3b6c42c2bccb45329096b645;
the existing Unicode normalization include in rust/Makefile is preserved.

The source design received independent review.  V2 corrects intentionally
concurrent published-node flags and entry-count accesses with aligned
same-width relaxed atomic operations at the shared access sites.  The
source synchronization blocker identified in v1 is resolved.  This source
review does not establish configured bindings, native ABI or runtime parity.
V3 adds only the two original C copyright notices to the reviewed V2 Rust
source; it makes no behavioral change. V1 and V2 payloads remain preserved.

Implemented callback bodies
---------------------------

The Rust source supplies all eight provider callbacks, with C macro/inline
adapters and the original map_ops/BTF registration schema:

* map_alloc: lupos_trie_alloc
* map_free: lupos_trie_free
* map_lookup_elem: lupos_trie_lookup_elem
* map_update_elem: lupos_trie_update_elem
* map_delete_elem: lupos_trie_delete_elem
* map_get_next_key: lupos_trie_get_next_key
* map_check_btf: lupos_trie_check_btf
* map_mem_usage: lupos_trie_mem_usage

Generic batch dispatch remains the original shared C subsystem service.
Native bindings must be generated from the full selected BPF-capable kernel
configuration.  The private trie declarations are derived from retained C;
compile-time layout assertions still require a real configured build.

Original test plan
------------------

Build separate C-default and Rust-enabled kernels with otherwise matched
configurations and canonical Kbuild.  Record source, configuration, binding,
object, kernel and test-binary hashes.  Verify exactly one trie_map_ops,
the Rust callback owner when enabled, and the original lpm_trie BTF type.

Run the complete unmodified tools/testing/selftests/bpf/test_maps binary:
its runner provides no CLI filter.  Keep BPF_STRICT_BUILD=1 and the canonical
C flags (including -g, default -O0, -rdynamic, -std=gnu11, -Wall, -Werror,
frame pointers, and the existing include/link settings).  Use the original
map_tests/lpm_trie_map_basic_ops.c, lpm_trie_map_get_next_key.c and
lpm_trie_map_batch_ops.c inputs; neighboring Rust test translations do not
replace the original C suite.

The basic suite checks widths 1..16, deterministic reference lookups before
and after deletion, IPv4/IPv6 and long lookup prefixes, branch/ancestor/leaf
deletion, empty/absent keys, postorder iteration, concurrent operations,
update flags/capacity and repeated deletion during integer iteration.
The next-key suite covers /0../32 and eight readers using up to 65,536
iterations each.  Batch tests cover original generic dispatch, key/value
association, chunks 1..9 and empty-map ENOENT.

Run canonical test_progs -t map_ptr for positive BTF-backed map creation and
typed map-pointer/BTF-ID access.  The original benchs/bench_lpm_trie_map.c,
progs/lpm_trie_bench.c and progs/lpm_trie_map.c provide supplemental eBPF
program-path and performance coverage.  Retain exit status, complete
pass/fail/skip counts and kernel diagnostics for each matched kernel.

Unproven coverage
-----------------

No binding generation, compilation, installation, VM run or LPM test was
performed for this source checkpoint.  Native ABI, C/R runtime parity,
architecture reachability and performance remain unproven.  The cited
original tests do not directly assert exact map_mem_usage bytes, exhaust
allocation-attribute rejection, force allocator/stack-init failures or
resource-spinlock EDEADLK/ETIMEDOUT, or prove every RCU/concurrency execution.
Map close exercises destruction without proving all reclamation timings.
Negative BTF key kinds and complete native layout are also not established
by the positive map_ptr test.  These gaps must not be reported as passes.
