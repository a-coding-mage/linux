BPF token source checkpoint
===========================

Provenance and scope
--------------------

This default-off, unbuilt checkpoint reconstructs ``kernel/bpf/token.c`` using
the original C and headers at Linux commit
``07b2f9d28c455307d0f626c3673f6a8aa1f3968d``. The final v2 source archive
SHA-256 is
``03ddb7473bc4f684f1ae8fbb76cf9a55853b8d4b89c7a1324247e050df0c9882``.
V2 includes the two anonymous-union field-path corrections. Original C sources,
tests and existing BPF LPM integration remain unchanged.

Rust implements token creation and unwind, namespace/capability and LSM
policy, descriptor lookup, mask checks, information copying, fdinfo, release,
reference lifetime and deferred free callbacks. Configured C declarations
supply layouts and constants; C adapters retain kernel macros, static inlines
and registration metadata. ``CONFIG_RUST_BPF_TOKEN`` defaults to ``n``; the
enabled proposal selects Rust for the existing ``token.o`` slot plus its C
adapter and configured bindings.

Original tests and pending gates
-------------------------------

Build the retained C BPF selftests through their original Makefile with strict
build behavior, original C BPF programs, libbpf and bpftool-generated skeletons.
The entry point is ``tools/testing/selftests/bpf/prog_tests/token.c`` and its
``serial_test_token()`` suite, selected by ``test_progs -t token``. Compare
original-C and Rust providers with the same source, configuration and tools.
Do not count skips or incomplete objects as passes.

The planned isolated guest needs active BPF LSM, BTF, user/mount namespaces,
bpffs delegation, capability and descriptor-transfer support, XDP/freplace/
struct_ops functionality and original runner prerequisites. The kallsyms case
temporarily adjusts security-related guest sysctls; it must remain guest-only.
Coverage still needs C-driven checks of malformed descriptors/attributes,
creation failures, all information-copy boundaries, masks, failure cleanup,
retained references and concurrent final puts.

Original token-info assertions, child assertion reporting and an inode-failure
ownership path remain separate, runtime-unconfirmed baseline concerns. Original
C and assertions are preserved; full logs and executed assertions must accompany
later results. Any semantic or test repair needs a separate change.

Token-specific configured bindings, native Rust/C compilation, selector-off/on
symbol ownership, linking, callback/CFI/BTF/ABI, original C selftest build and
isolated runtime comparison remain pending. No build or test run is claimed.
