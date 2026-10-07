# CESR qualified-material fixtures

- Schema version: 1
- Upstream: `https://github.com/WebOfTrust/signify-ts.git`
- Reference commit: `91a04a5edd804425f42be75ffbf034f8d14e5cb2`
- Generated: 2026-08-04
- Generator: `node tools/reference-vectors/cesr_matter.mjs`
- Reference sources: `src/keri/core/matter.ts`
- Reference tests: `test/core/matter.test.ts`, with variable construction examples corroborated by
  `test/core/bexter.test.ts`

Run `npm ci --prefix tools/reference-vectors`, then run the generator from the repository root.
The generator uses deterministic arithmetic byte sequences and records raw and qualified-binary
fields as lowercase hexadecimal. `qb64` is emitted directly by the pinned `Matter` implementation.
Because that implementation explicitly rejects `qb2` input, `qb2_hex` is derived from its exact
`qb64` using the pinned reference's `decodeBase64Url` helper. Its variable-size parser is defective:
it recovers code/raw bytes but leaves `size` undefined, cannot re-encode, and cannot safely delimit a
stream. Those exact observations are recorded; Rust instead validates the soft size and produces a
round-trippable canonical value.

Successful fixed cases cover every fixed code in `Matter.Sizes`. Variable cases cover empty matter,
both `A` and `B` variable families, all three lead sizes, and both small and large code forms.
Rejected cases cover empty, unsupported, truncated, short-raw, non-zero alignment-bit, and non-zero
lead-byte inputs. Rust adds bounded variable parsing and qualified-binary parsing as safe protocol
completion beyond the unfinished reference operations.
