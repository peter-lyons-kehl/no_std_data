<!-- The following comments hides this section from being shown by
     https://peter-kehl.github.io/no_std_rna_slice_patterns_presentation.
-->
<!-- .slide: data-visibility="hidden" -->
You can see this text along with additional content and source code sections [in a
presentation](../no_std_rna_slice_patterns_presentation). That also shows any source code referred
to below.
<!-- Any comments in source starting with "presentation-" are anchors/delimiters for the above presentation.
-->
---

# Goal
[Examples](https://github.com/peter-kehl/no_std_rna_slice_patterns) of `no_std` (low
level/embedded-friendly) and mostly heapless & slice-handling patterns in Rust.

# Prerequisites
- [https://peter-kehl.github.io/no_std_rust_lib_presentation](no_std_rust_lib_presentation)
- `nightly` Rust (July 2022), but only for test purposes. The actual implementations work with `stable`.

# Disclaimers
## Disambiguation
This is not about DNA/RNA/genetic patterns in general or at any detail. Instead, it's a set of
implementations of [Exercism](https://exercism.org) > [Rust Track](https://exercism.org/tracks/rust) > [RNA Transcription exercise](https://exercism.org/tracks/rust/exercises/rna-transcription).

# Omitted & non-standard documentation
Big parts of these examples are repetitive. For brevity, similar items are docummented only once: at
their first occurrence (as in the above order of the examples).

Some code documentation comments include implementation details, or they refer to private
fields/functions. That's contrary to a general good practice of API design/documentation. However,
this allows the reader to easily navigate to Rust API references (for example, by Ctrl+click in VS
Code).

# Exercism-specific
Two implementations required a minor change to its Exercism's tests.

In order to upload this to Exercism, you'd need to
- incorporate 00_shared/src/lib.rs into `src/lib.rs` of your chosen solution,
- change your (chosen implementation) crate name back to `rna_transcription` in its `Cargo.toml` and
  in its `tests/rna-transcription.rs`; and
- rename the crate's directory to `rna-transcription`.

See also the [exercise source](https://github.com/exercism/rust/tree/main/exercises/practice/rna-transcription).
---

# Suggested order
These [examples](https://github.com/peter-kehl/no_std_rna_slice_patterns) are ordered by complexity or level of indirection/abstraction:

- 00_utils (shared utilities)
- 01_ok_heap-string
- 02_no_heap-array-const_limit-chars
- 03_no_heap-array-const_limit-bytes-wipe_on_clone
- 04_no_heap-array-const_limit-bytes-wipe_on_mut
- 05_no_heap-array-const_generic_exact-bytes
- 06_no_heap-array-const_generic_limit-bytes-wipe_on_clone
- 07_no_heap-array-const_generic_limit-bytes-wipe_on_mut
- 08_no_heap-slice-pass_in-bytes-wipe_on_drop
- 09_no_heap-slice-pass_in-bytes-wipe_on_mut
- 10_ok_heap-slices-box_dyn_trait-map
- 11_no_heap-slices-iterator_enum
- 12_no_heap-slices-iterator_impl
- 13_no_heap-eq_branch_iterators-dyn_trait
- 14_no_heap-eq_branch_iterators-matrix
- 15_no_heap-eq_iterator_to_generic_fn
- 16_no_heap-eq_dispatch_specialized
- 17_no_heap-eq_dispatch_universal

