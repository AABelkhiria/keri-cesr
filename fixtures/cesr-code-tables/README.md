# CESR derivation code-table fixtures

- Upstream: `https://github.com/WebOfTrust/signify-ts.git`
- Reference commit: `91a04a5edd804425f42be75ffbf034f8d14e5cb2`
- Reference describe: `0.4.0-24-g91a04a5`
- Source: `src/keri/core/matter.ts`
- Tests: `test/core/matter.test.ts`
- Fixture schema: `v1`
- Generated: `2026-08-04`
- Binary encoding: not applicable; this fixture contains ASCII hard codes and integer size metadata.

Generate from the repository root after building the pinned TypeScript checkout:

```bash
npm ci --prefix tools/reference-vectors
node tools/reference-vectors/cesr_code_tables.mjs
```

`v1.json` records every `Matter.Sizes` entry in declaration order, computed fixed raw sizes, exact
membership in the five exported codices, all `Matter.Hards` selector widths, the upstream `Sizage`
constructor property order, and a deterministic unsupported-code rejection. Rust tests consume the
checked-in JSON and do not invoke Node.
