# networking-components/networking-components-mcp-server.rs#2 — docs: add AGENTS.md and fleet sops env layout

head: chore/agents-md-and-sops-env  base: main  author: ORESoftware  updated: 2026-08-27T19:14:36Z
dir: /Users/maca5/codes/.claude-fleet/scratch/merge/networking-components_networking-components-mcp-server.rs__2

## conflicted files
- justfile

## base (main) last 8 commits
14f9858 DEN-965: harden networking-components MCP provider parity
55d6e0a Merge pull request #3 from networking-components/agent/den-390-f2e-generated-env
a8834cc Merge pull request #4 from networking-components/agent/ores-sops-ensure-dec-20260828b
36117a3 Refuse unguarded env/dec mkdir before ores-sops.
446fbd7 DEN-390 Consume generated runtime env readers.
5667f06 DEN-390 Generate flags-2-env runtime env readers.
0efdbad fix: recover from oversized MCP stdio frames
17452cc feat: specialize the hardened MCP server for networking-components

## head (chore/agents-md-and-sops-env) last 8 commits
c20c9b4 Replace yanked chacha20 0.10.1 with 0.10.2.
bc16ddc Run just env-check in primary CI and ignore env/dec/.
f048b15 docs: add AGENTS.md and fleet sops env layout
0efdbad fix: recover from oversized MCP stdio frames
17452cc feat: specialize the hardened MCP server for networking-components
b54877c feat: establish hardened organization MCP template

## merge-base: 0efdbad3f97d60788a5af991204c5710d423d284

## PR diff stat (merge-base..head)
 .envrc                              |   7 +
 .github/workflows/ci.yml            |  10 +
 .github/workflows/secrets-audit.yml |  32 +++
 .gitignore                          |  18 ++
 .just/dotenv.py                     |  87 +++++++
 .just/env.just                      | 461 ++++++++++++++++++++++++++++++++++++
 .sops.yaml                          |  27 ++-
 AGENTS.md                           |  88 +++++++
 Cargo.lock                          |   4 +-
 env/README.md                       |  28 +++
 env/enc/prod.env.enc                |  10 +
 flake.nix                           |  13 +-
 justfile                            |  76 +++---
 13 files changed, 823 insertions(+), 38 deletions(-)

## base diff stat (merge-base..base)
 .cli-flags.toml                 |   18 +
 .github/workflows/ci.yml        |   13 +-
 Cargo.lock                      | 1575 +++++++++++++++++++++++++++++++++++++--
 Cargo.toml                      |   11 +-
 README.md                       |   74 +-
 generated/dart/env.dart         |    9 +
 generated/dart/runtime.dart     |  172 +++++
 generated/gleam/env.gleam       |   12 +
 generated/gleam/runtime.gleam   |   42 ++
 generated/rust/env.rs           |   18 +
 generated/rust/runtime.rs       |  196 +++++
 generated/typescript/env.ts     |   13 +
 generated/typescript/runtime.ts |  193 +++++
 justfile                        |    4 +-
 mcp-fleet-profile.json          |  893 ++++++++++++++++++++++
 src/http.rs                     |   10 +
 src/main.rs                     |   27 +-
 src/spec.rs                     |   64 ++
 tests/stdio_parity.rs           |  260 +++++++
 19 files changed, 3498 insertions(+), 106 deletions(-)

## merge output
Auto-merging .github/workflows/ci.yml
Auto-merging Cargo.lock
Auto-merging justfile
CONFLICT (content): Merge conflict in justfile
Automatic merge failed; fix conflicts and then commit the result.
