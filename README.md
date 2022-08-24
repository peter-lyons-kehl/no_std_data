# Disambiguation
This is not about DNA/RNA/genetic patterns in general or at any detail. It's a set of
implementations of Exercism's Rust Track "RNA Transcription" exercise.

# no_std
`no_std` Rust

# Suggested order

# Omitted documentation

# Exercism
One implementation required a minor change to its Exercism's tests.

# Disclaimers/Side notes
- In order to upload this to Exercism, you'd need to
 - incorporate 00_shared/src/lib.rs into src/lib.rs of your chosen solution,
 - change your (chosen implementation) crate name back to `rna_transcription` in its `Cargo.toml`
   and in its `tests/rna-transcription.rs`; and
 - (probably need to) rename the crate's directory to `rna-transcription`.
 
- Some code doc comments include implementation details, or they refer to private fields/functions.
  That's contrary to a general good practice of API design/documentation. However, this allows the
  reader to easily navigate to Rust API references (for example, by Ctrl+click in VS Code).
