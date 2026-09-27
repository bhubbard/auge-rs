# auge-rs

[![macOS 26+](https://img.shields.io/badge/macOS-26%2B%20(Tahoe)-000000?logo=apple&logoColor=white)](https://developer.apple.com/macos/)
[![Rust 2021](https://img.shields.io/badge/rust-edition%202021-orange.svg)](Cargo.toml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GitHub Pages](https://img.shields.io/badge/docs-GitHub%20Pages-blue.svg)](https://bhubbard.github.io/auge-rs/)
[![Coverage Report](https://img.shields.io/badge/coverage-HTML%20Report-brightgreen.svg)](https://bhubbard.github.io/auge-rs/)
[![100% On-Device](https://img.shields.io/badge/privacy-100%25%20on--device-green)](https://developer.apple.com/documentation/vision)

**Apple's on-device Vision framework from the command line ported to native Rust.**  
Fast on-device OCR, image classification, barcode & QR code detection, face detection, human detection, horizon leveling, animal recognition, and document parsing.

Ported directly from [Arthur-Ficial/auge](https://github.com/Arthur-Ficial/auge).

---

## ⚡ What is this

Every Mac ships with Apple's [Vision framework](https://developer.apple.com/documentation/vision) — a powerful on-device computer vision engine. `auge-rs` wraps it in a UNIX CLI with sub-millisecond execution, pipe-friendly design, and structured output formats.

- **100% On-Device**: Zero network calls, zero API keys, zero cloud fees, zero tracking.
- **OCR**: Extract text, line details, and bounding boxes from screenshots, scans, images, and piped buffers.
- **Classification**: Identify image categories and tags across 1,000+ taxonomy labels.
- **Barcodes & QR Codes**: Instant detection and decoding.
- **Faces & Humans**: Detect bounding boxes, face counts, and human postures.
- **Horizon & Rectangles**: Detect horizon tilt angle in degrees and perspective rectangles.
- **Text Cleaning**: Optional `--clean` post-pass to fix end-of-line hyphenation and smart quotes.
- **Format Flexibility**: Output in `plain`, Markdown (`--md`), standard JSON schema v2 (`--json`), and newline-delimited JSON (`--ndjson`).
- **Flexible Inputs**: File paths, stdin pipes, and direct clipboard images (`--clipboard`).

---

## 🛠️ CLI Usage Examples

```bash
# OCR text extraction
auge --ocr screenshot.png

# Image classification
auge --classify photo.jpg

# Barcode and QR code scanning
auge --barcode qr.png

# Detect faces and humans
auge --faces portrait.jpg
auge --humans scene.png

# Format as Markdown or JSON
auge --classify --md image.png
auge --ocr --json receipt.png

# Stream from clipboard or stdin
pbpaste | auge --ocr
cat image.png | auge --classify
```

---

## 🧪 Testing

```bash
cargo test --all-targets
```

---

## 📄 License

MIT License. See [LICENSE](LICENSE) for details.
