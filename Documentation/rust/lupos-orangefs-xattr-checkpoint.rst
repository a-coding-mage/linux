OrangeFS xattr source checkpoint
===============================

Provenance and implemented scope
-------------------------------

This default-off, unbuilt checkpoint reconstructs ``fs/orangefs/xattr.c``
from the retained C and headers at Linux commit
``07b2f9d28c455307d0f626c3673f6a8aa1f3968d`` and composes it with the provider
integration at ``62a3294181556f17db863c9016524fa3068a374b``. The snapshot-02
source archive SHA-256 is
``4655a5cd12b36680ce81ea294ec717bc4510fe9e3343ae558dc21d71066f7d6d``.

Rust implements get, set, remove and list, cache lookup and expiry, flag/error
handling, list continuation, operation cleanup, callback ABI and handler
registration. C adapters retain kernel macros, static inlines and diagnostics;
configured declarations supply the real layouts. Original C and tests remain
unchanged, including original cache/protocol behavior.

``CONFIG_RUST_ORANGEFS_XATTR`` defaults to ``n``. The enabled proposal retains
the ``orangefs`` aggregate and ``xattr.o`` slot, selects Rust plus its C helper,
and provides explicit inspection targets. Private listing objects preserve
builtin/module membership and receive separate debug flags in both modes.

Original tests and remaining gates
---------------------------------

``Documentation/filesystems/orangefs.rst`` describes the external xfstests
``./check -pvfs2`` route, with real OrangeFS userspace/client services and
separate test/scratch filesystems. Its exact maintained cases and original C
helpers still need to be pinned and inspected before qualification. A shell
invocation alone does not establish original-C-test coverage.

The retained xattr sockfs tests exercise sockets on sockfs. The pathname-socket
test uses ordinary ``/tmp`` and a 4096-byte value, below OrangeFS's 8192-byte
protocol value limit; it is not a direct OrangeFS acceptance route. No dedicated
in-tree OrangeFS C suite was identified. No test execution is claimed.

Configured bindings, native Rust/C compilation and linking, selected/unselected
symbol ownership, builtin/module identity, selector transitions and inspection
outputs remain pending. Runtime qualification also needs VFS/ACL/fileattr
coverage, fresh/expired/negative caches, invalidation, page-token continuation,
size/probe boundaries, malformed replies, failures, timeouts and concurrency.
Generic xattr success cannot establish all those paths. Native ABI/BTF checks
and original-C-driven runtime comparison remain pending.
