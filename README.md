# AI Translation CLI

A fast and beautiful translation tool built with Rust, featuring real-time streaming translation and an interactive TUI interface.

## Features

- **Fast & Small**: Binary size only 1.6MB, startup time < 100ms
- **Streaming Translation**: Real-time streaming output during translation
- **Beautiful TUI**: Chat-style interface with Ratatui
- **Multiple Modes**:
  - Interactive TUI mode (default)
  - Quick mode (`-q` flag) for one-shot translation
  - Pipe mode for command-line integration
- **Display Modes**: Toggle between translation-only and bilingual display with TAB key
- **Clipboard Support**: Copy translations with keyboard shortcuts (Ctrl+Y, 1-9)
- **Cross-platform**: Works on macOS, Linux, and Windows

## Installation

### Pre-built Binaries

Download pre-built binaries from the [Releases](https://github.com/your-username/ai-tran-cli/releases) page.

Available platforms:
- macOS (Intel & Apple Silicon)
- Linux (AMD64 & ARM64)
- Windows (x86 & ARM64)

### Build from Source

#### Quick Build (Current Platform)

```bash
git clone <repository-url>
cd ai-tran-cli
make build
```

Binary will be in `dist/ai-tran-cli-<platform>`.

#### Build for All Platforms

```bash
# Install cross-compilation targets
make install-targets

# Build all platforms
make all
```

Binaries will be in `dist/` directory.

#### Traditional Cargo Build

```bash
cargo build --release
```

The binary will be available at `target/release/ai-tran-cli`.

See [BUILD.md](BUILD.md) for detailed build instructions.

## Configuration

### Automatic Environment Variable Loading

The CLI **automatically reads** configuration from environment variables. No manual configuration is required if you set up a `.env` file.

Create a `.env` file in the project directory (or copy from `.env.example`):

```bash
cp .env.example .env
```

Edit `.env` with your API credentials:

```bash
OPENAI_API_KEY=sk-your-api-key-here
OPENAI_API_BASE=https://api.siliconflow.cn/v1
OPENAI_MODEL=deepseek-ai/DeepSeek-V3.2-Exp
TARGET_LANGUAGE=zh-CN
```

**The application will automatically:**
- ✅ Load `.env` file on startup
- ✅ Read API credentials from environment variables
- ✅ Use sensible defaults for optional settings
- ✅ Show clear error messages if API key is missing

### Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `OPENAI_API_KEY` | ✅ Yes | - | Your OpenAI-compatible API key |
| `OPENAI_API_BASE` | No | `https://api.openai.com/v1` | API endpoint URL |
| `OPENAI_MODEL` | No | `gpt-5-mini` | Model to use for translation |
| `TARGET_LANGUAGE` | No | `zh-CN` | Target language code |

For detailed configuration guide, see [doc/environment-setup.md](doc/environment-setup.md).

## Usage

### Interactive TUI Mode

Run without arguments to enter interactive mode:

```bash
ai-tran-cli
```

Type your text and press Enter to translate. The interface shows:
- Translation history with streaming updates
- Input box at the bottom
- Status bar with keyboard shortcuts

**Keyboard Shortcuts:**
- `Enter` - Send text for translation
- `TAB` - Toggle display mode (translation-only ↔ bilingual)
- `Ctrl+Y` - Copy latest translation to clipboard
- `1-9` - Copy translation #N to clipboard
- `Ctrl+C` - Clear translation history
- `ESC` - Quit

### Quick Mode

For one-shot translation without entering TUI:

```bash
echo "Hello, world!" | ai-tran-cli -q
# Output: 你好，世界！
```

Or pass text as stdin:

```bash
ai-tran-cli -q < input.txt
```

### Verbose Mode

Use `-v` or `--verbose` flag to print detailed debug information (with sensitive data masked):

```bash
echo "Hello" | ai-tran-cli -v -q
```

Output:
```
[VERBOSE] Loading environment variables from .env file
[VERBOSE] Initializing OpenAI provider
[VERBOSE] Provider: OpenAI
[VERBOSE] API Base: https://api.siliconflow.cn/v1
[VERBOSE] Model: deepseek-ai/DeepSeek-V3.2-Exp
[VERBOSE] API Key: sk-ttek**************zopq
[VERBOSE] Target Language: zh-CN
[VERBOSE] Translating text: Hello
[VERBOSE] HTTP Status: 200 OK
[VERBOSE] Translation result: 你好
你好
```

Verbose mode is useful for:
- Debugging API connection issues
- Verifying configuration is loaded correctly
- Troubleshooting translation errors
- Checking API request/response details

**Security**: API keys and sensitive data are automatically masked in verbose output.

### Pipe Mode

Pipe input automatically enters TUI with the text pre-submitted:

```bash
echo "Translate this" | ai-tran-cli
```

This starts the TUI and immediately begins translating "Translate this".

## Architecture

- **Language**: Rust
- **TUI Framework**: Ratatui 0.26 + Crossterm 0.27
- **Async Runtime**: Tokio
- **HTTP Client**: reqwest with streaming support
- **Clipboard**: arboard (cross-platform)
- **CLI Parsing**: clap 4.5

### Project Structure

```
ai-tran-cli/
├── src/
│   ├── app/           # Application state management
│   │   ├── app.rs     # App state and display modes
│   │   └── message.rs # Message data model
│   ├── providers/     # Translation providers
│   │   ├── mod.rs     # Provider trait
│   │   └── openai.rs  # OpenAI-compatible provider
│   ├── ui/            # TUI components
│   │   ├── chat.rs    # Chat area rendering
│   │   ├── input.rs   # Input box rendering
│   │   ├── layout.rs  # Layout management
│   │   └── statusbar.rs # Status bar
│   ├── events/        # Event handling
│   │   └── handler.rs # Keyboard event handlers
│   └── main.rs        # Entry point
├── doc/
│   └── prd.md         # Product requirements document
├── .env.example       # Environment template
└── Cargo.toml         # Project configuration
```

## Performance

- Binary size: 1.6MB (optimized with LTO and strip)
- Startup time: < 100ms
- Streaming: Real-time character-by-character output
- Memory: Minimal footprint with async/await

## Development

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Test quick mode

```bash
echo "test" | cargo run -- -q
```

### Release build

```bash
cargo build --release
```

## License

MIT

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.
