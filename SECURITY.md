# Security Policy

## Supported versions

Security fixes go to the latest `0.x` release. Older `0.x` versions are not patched.

## Reporting a vulnerability

Please don't open a public issue for something you think is exploitable.

Use GitHub's [private vulnerability reporting][gh-pvr] on this repository instead: *Security* →
*Report a vulnerability*.

[gh-pvr]: https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability

Useful things to include: the version, platform and features you were using, something we can run to
reproduce it, and what you think the impact is. If secret material was involved, say so, but please
generate throwaway keys for the report. Never send real seeds, salts, private keys or passcodes.

## Threat model

Everything parsed from outside is assumed hostile: `qb64` and `qb2` streams, derivation, indexer
and counter codes, declared sizes and counts, SAD paths, public keys, signatures, digests and
ciphertexts.

With the `crypto` feature, the sensitive material is Ed25519 seeds, salts, the X25519 private keys
derived from them, decrypted seeds and salts, and the intermediate buffers used to derive any of
those.

Given that, the implementation is built to hold up against a few specific things.

Malformed input shouldn't be able to panic or abort the process, so crate-owned code contains no
`unsafe`, `unwrap`, `expect`, panicking macro, or indexing that can fail, and every rejection is a
typed error. Length arithmetic is checked, and parsers enforce explicit ceilings (for example
`MAX_DECODED_BYTES`, `MAX_RAW_MATERIAL_BYTES`, `MAX_SAD_PATH_COMPONENTS` and
`MAX_DERIVATION_PATH_BYTES`) before allocating from a size the input declares.

Encodings are strict: non-canonical Base64, wrong code families, size mismatches and trailing
material are rejected rather than quietly normalized.

Digest comparison and the seed-to-key match in `Encrypter::matches_seed_qb64` are constant-time.
Decryption reports a single `DecryptionFailed` error whatever the underlying cause, so it can't be
used as an oracle. X25519 keys are validated when constructed, and non-contributory points are
rejected.

Secrets are kept away from anywhere they might leak by accident: `Signer`, `Salt` and `Decrypter`
redact their `Debug` output, implement no `Display` or `Clone`, expose no raw-byte accessor, and
zero their owned buffers on drop.

## Randomness

`Signer::generate`, `Salt::generate` and encryption take their entropy from the operating system
through `getrandom`. A failure is returned as an error; nothing falls back to a seeded generator.
Deterministic construction from caller-supplied bytes exists for reproducible derivation and tests.

## Known limitations

Constructors that take borrowed secret bytes copy them. Whatever buffer you kept those bytes in is
yours to erase; this crate can't reach it.

Zeroization is best effort. It's a language-level guarantee at most, and it doesn't help against a
compromised OS, a debugger, a core dump, or pages swapped to disk.

`KeyDerivationProfile::Temporary` runs Argon2id with one iteration and 8 KiB of memory. It exists for
tests and must not protect real secrets.

secp256k1 and Ed448 material can be parsed and encoded for wire compatibility, but signatures over
those curves are not verified.

Constant-time behavior rests on `subtle`, `blake3` and the dalek crates. We haven't measured it with
dedicated tooling like `dudect`.

## Dependencies

Production dependencies have known licenses and active maintenance, with default features off, and
everything beyond `base64` and `zeroize` is only built with the `crypto` feature. Three are
first-party, written and maintained by this crate's author: `argon2id-p1`, `nacl-sealed-box` and
its nonce hash `blake2b-192`. Their outputs are checked byte for byte against the signify-ts
fixtures here. `cargo deny` and the resolved graph get reviewed before each release.
