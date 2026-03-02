<div align="center">
  <h1>Infolyt</h1>
  <p><strong>Complete filesystem intelligence — index, analyse, organise.</strong></p>

  [![CI](https://github.com/infolyt/infolyt/actions/workflows/ci.yml/badge.svg)](https://github.com/infolyt/infolyt/actions/workflows/ci.yml)
  [![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
  [![Crates.io](https://img.shields.io/crates/v/infolyt.svg)](https://crates.io/crates/infolyt)
  [![Discord](https://img.shields.io/discord/000000000?label=Discord&logo=discord)](https://discord.gg/infolyt)
</div>

## What is Infolyt?

Infolyt is a fast, cross-platform file intelligence tool.
It indexes every file on your system, detects true file types
via magic bytes, computes content hashes, finds duplicates,
and generates smart organisation recommendations.

## Features

- ⚡ Scans 500,000+ files/minute using parallel processing
- 🔍 True file type detection via magic bytes (not extensions)  
- 🔐 Multi-tier hashing: XXH3 partial → BLAKE3 full
- 🧠 Intelligent organisation recommendations
- 🗂️ Safe quarantine-before-delete workflow
- 🖥️ Native GUI (Tauri) + full-featured CLI
- 🌍 Cross-platform: Windows, macOS, Linux

## Quick Start

\`\`\`bash
# Install
cargo install infolyt

# Scan your home directory
infolyt scan ~/

# Find duplicates
infolyt duplicates list

# Get organisation recommendations
infolyt recommend
\`\`\`

## License

Apache 2.0 — see [LICENSE](LICENSE).
Infolyt Pro is available under a commercial license.