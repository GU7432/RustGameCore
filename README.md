# RustGameCore

一個以 Rust 撰寫的遊戲核心/底層庫範例，包含對外 C header（由 `cbindgen` 產生）以便與其他語言或系統整合。

> 注意：此 README 依據目前專案結構與常見慣例撰寫。請在必要時補上授權資訊與詳細的專案描述。

## 目標

- 提供遊戲邏輯與核心元件的 Rust 實作。
- 可編譯成 Rust library 並由 `cbindgen` 產生 C header（`include/rustcore.h`）以供外部呼叫。

## 目錄簡介

- `src/` - Rust 原始碼，包含 `lib.rs`（函式庫）與 `main.rs`（範例或執行檔）。
- `include/` - 預期的 C 標頭檔位置（由 `cbindgen` 或手動維護）。
- `puzzle/` - 子模組範例。
- `Cargo.toml` - Rust 專案設定。
- `cbindgen.toml` - `cbindgen` 設定（若有）。

## 快速開始

在 Linux 下，先確保已安裝 Rust toolchain（rustup / cargo / rustc）：

```bash
# 安裝或更新 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update
```

建置專案（debug）：

```bash
cargo build
```

建置（release）：

```bash
cargo build --release
```

若專案包含可執行檔（`src/main.rs`），可直接執行：

```bash
cargo run
```

或執行 release 版：

```bash
cargo run --release
```

測試：

```bash
cargo test
```

## 產生 C 標頭（若需要）

專案包含 `cbindgen.toml`，可以使用 `cbindgen` 產生 C header（請先安裝 `cbindgen`）：

```bash
cargo install --force cbindgen
cbindgen --config cbindgen.toml --output include/rustcore.h 
```


## 常見指令總覽

- 建置：`cargo build` 或 `cargo build --release`
- 執行：`cargo run`
- 產生 C header：`cbindgen --config cbindgen.toml --output include/rustcore.h`



