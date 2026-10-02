Block crypto fallback source checkpoint
=======================================

Provenance and scope
--------------------

This default-off, unbuilt checkpoint reconstructs the provider from
``block/blk-crypto-fallback.c`` and configured-header interfaces retained at
Linux commit ``07b2f9d28c455307d0f626c3673f6a8aa1f3968d``. The original C
provider and tests remain unchanged. The source checkpoint archive SHA-256 is
``03d33993a7bddfddc58e623e71f6ce7b6ee541134bce46df0697789ceb7e1107``.

Rust implements keyslot programming/eviction, BIO preparation, bounce-page
allocation and completion, per-data-unit encryption/decryption, mode startup,
initialization and original error unwinding. C adapters retain kernel macros,
inline operations, synchronous request stack lifetimes and parameter metadata.
``CONFIG_RUST_BLK_CRYPTO_FALLBACK`` defaults to ``n``; the proposed enabled
rules retain the original object slot and select Rust plus its C adapters.

Original tests and pending gates
-------------------------------

The retained ``crypto/testmgr.c`` and ``crypto/testmgr.h`` provide original C
tests for ``xts(aes)``, ``essiv(cbc(aes),sha256)``,
``adiantum(xchacha12,aes)`` and ``xts(sm4)``. Use their original crypto manager
build rules and configured selftests. Those tests exercise the transforms,
not the fallback BIO provider; passing them would not establish provider parity.
No direct maintained in-tree C fallback suite was identified.

The fscrypt documentation describes external xfstests encryption runs. Any
later use must establish actual Rust fallback selection and use synthetic
guest disks. Such shell suites do not by themselves satisfy direct original-C
provider coverage. Qualification still needs source-verified C-driven checks
for BIO splitting, partial reads, DUN carry, keyslot/mode transitions, every
allocation/crypto failure, cleanup and concurrent completion/publication.

Configured bindings, native Rust/C compilation, selector-off/on object and
symbol ownership, callback/CFI/BTF/ABI checks, linking, original-C tests and
isolated guest runtime comparison are pending. No build, configuration, test
execution or runtime result is asserted by this source checkpoint.
