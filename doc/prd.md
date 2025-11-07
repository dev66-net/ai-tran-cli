# AI Translation CLI - Product Requirements Document

## 1. 产品概述

### 1.1 产品定位
一个基于Rust开发的高性能、小巧的命令行翻译工具，提供类似聊天界面的TUI体验，支持流式实时翻译输出。

### 1.2 核心价值
- **高性能**: Rust原生性能，二进制小巧 (~3-5MB)
- **流式体验**: 实时显示翻译结果，无需等待
- **交互友好**: 聊天式界面，符合现代用户习惯
- **功能丰富**: 剪贴板、显示模式、管道支持等

### 1.3 目标用户
- 开发者和技术人员
- 需要快速翻译的命令行用户
- 喜欢键盘操作的效率用户
- 需要脚本集成的自动化场景

---

## 2. 功能需求

### 2.1 核心功能

#### 2.1.1 TUI交互模式

**布局设计**:
```
┌─────────────────────────────────────────────────────┐
│ AI Translation CLI    [OpenAI GPT-4] [Mode: Both]  │
├─────────────────────────────────────────────────────┤
│                                                      │
│  [1] You: Hello world                                │
│      ✓ 你好世界                                      │
│                                                      │
│  [2] You: How are you?                               │
│      ⠋ 你好▊                    ← 流式实时输出       │
│                                                      │
│  [3] You: Good morning                               │
│      ✓ 早上好                                        │
│                                                      │
│  (可滚动查看历史记录)                                │
│                                                      │
├─────────────────────────────────────────────────────┤
│ > Type your text here...                        [?] │
└─────────────────────────────────────────────────────┘
 Ctrl+Y: Copy | TAB: Mode | Ctrl+C: Quit | ↑↓: Scroll
```

**关键特性**:
- 底部输入框，固定3行高度
- 顶部聊天区域，显示输入和翻译结果
- 每条消息带序号 [1], [2], [3]...
- 流式翻译时显示闪烁光标 ▊
- 支持滚动查看历史记录

---

#### 2.1.2 流式翻译

**流式输出效果**:
```
初始状态:
  You: How are you doing today?
  ⠋

逐字显示:
  You: How are you doing today?
  ⠋ 你▊

  You: How are you doing today?
  ⠋ 你今天▊

  You: How are you doing today?
  ⠋ 你今天过得▊

完成状态:
  You: How are you doing today?
  ✓ 你今天过得怎么样？
```

**技术要求**:
- 支持 SSE (Server-Sent Events) 流式API
- 实时更新UI，无需等待完整响应
- 显示加载动画和闪烁光标
- 自动滚动到底部

**支持的Provider**:
| Provider | 流式支持 | 状态 |
|----------|---------|------|
| OpenAI | ✅ | 已实现 |
| Azure OpenAI | ✅ | 已实现 |
| Claude | ✅ | 计划中 |
| Gemini | ✅ | 计划中 |

---

#### 2.1.3 剪贴板功能

**复制方式**:

| 快捷键 | 功能 | 使用场景 |
|--------|------|----------|
| `Ctrl+Y` | 复制最新翻译 | 最常用，刚翻译完直接复制 |
| `1-9` | 按序号复制 | 快速复制指定历史翻译 |
| `Ctrl+Shift+Y` | 复制全部历史 | 批量导出所有翻译结果 |

**交互流程**:
```
用户按下 Ctrl+Y:
1. 获取最新翻译结果
2. 复制到系统剪贴板
3. 显示通知: "✓ Copied to clipboard"
4. 通知3秒后自动消失

用户按下数字键 2:
1. 获取序号为[2]的翻译结果
2. 复制到系统剪贴板
3. 显示通知: "✓ Copied [2]"
```

**通知显示**:
```
┌─────────────────────────────────────────────────────┐
│ AI Translation CLI    [OpenAI GPT-4] [Mode: Both]  │
├─────────────────────────────────────────────────────┤
│             ✓ Copied to clipboard                    │  ← 3秒后消失
│  [1] You: Hello                                      │
│      ✓ 你好                                          │
```

---

#### 2.1.4 显示模式切换

**三种显示模式**:

##### 模式1：双语对照 (Bilingual) - 默认
```
[1] You: Hello world
    ✓ 你好世界

[2] You: Good morning
    ✓ 早上好
```

##### 模式2：仅翻译 (Translation Only)
```
[1] ✓ 你好世界

[2] ✓ 早上好
```

##### 模式3：仅原文 (Original Only)
```
[1] ✓ Hello world

[2] ✓ Good morning
```

**切换操作**:
- 按 `TAB` 键循环切换: Bilingual → Translation Only → Bilingual
- 状态栏显示当前模式
- 配置文件保存用户偏好

---

#### 2.1.5 管道模式

**场景1：无stdin，无参数** - 纯TUI模式
```bash
$ ai-tran-cli
# 进入空白TUI界面，等待用户输入
```

**场景2：有stdin，无参数** - TUI模式（自动提交）
```bash
$ echo "hello" | ai-tran-cli
# 进入TUI界面，自动提交 "hello" 并显示翻译
# 用户可以继续输入其他内容
```

**场景3：有stdin，有 -q 参数** - 快速模式
```bash
$ echo "hello" | ai-tran-cli -q
你好
# 翻译后立即退出，无TUI
```

**场景4：批量翻译**
```bash
$ cat texts.txt | ai-tran-cli -q
你好世界
早上好
你好吗
# 按行翻译，输出结果
```

---

### 2.2 高级功能

#### 2.2.1 配置管理

**配置文件位置**: `~/.config/ai-tran-cli/config.toml`

**配置示例**:
```toml
[provider]
name = "openai"
endpoint = "https://api.siliconflow.cn/v1/chat/completions"
model = "deepseek-ai/DeepSeek-V3.2-Exp"

[translation]
target_language = "zh-CN"
source_language = "auto"  # 自动检测

[display]
mode = "bilingual"  # bilingual, translation_only, original_only
show_line_numbers = true

[clipboard]
auto_copy_latest = false
quick_mode_auto_copy = true

[ui]
notification_duration = 3  # 秒
```

**API密钥存储**:
- 优先使用环境变量 `OPENAI_API_KEY`
- 次选使用 `.env` 文件
- 支持系统 keyring（未来）

---

#### 2.2.2 多语言支持

**目标语言**:
- 中文 (zh-CN)
- 英文 (en)
- 日文 (ja)
- 韩文 (ko)
- 西班牙语 (es)
- 法语 (fr)
- 德语 (de)

**语言检测**:
- 自动检测输入语言
- 智能选择目标语言（中文→英文，英文→中文）

---

## 3. 用户交互

### 3.1 键盘快捷键

| 快捷键 | 功能 | 类别 |
|--------|------|------|
| `Enter` | 提交翻译 | 输入 |
| `Backspace` | 删除字符 | 输入 |
| `↑` `↓` | 滚动消息 | 导航 |
| `Tab` | 切换显示模式 | 视图 |
| `Ctrl+Y` | 复制最新翻译 | 剪贴板 |
| `1-9` | 按序号复制 | 剪贴板 |
| `Ctrl+Shift+Y` | 复制全部历史 | 剪贴板 |
| `Ctrl+L` | 清除历史 | 操作 |
| `Ctrl+C` | 退出程序 | 系统 |

---

### 3.2 命令行参数

```bash
ai-tran-cli [OPTIONS] [COMMAND]

OPTIONS:
  -q, --quick              Quick mode: translate and exit
  -t, --target <LANG>      Target language [default: zh-CN]
  -p, --provider <NAME>    Provider name [default: openai]
  -m, --mode <MODE>        Display mode [default: bilingual]
  -v, --verbose            Verbose output
  -h, --help               Print help
  -V, --version            Print version

COMMANDS:
  config                   Configure provider and API keys
  list                     List available providers
  help                     Print this message or the help of subcommands
```

**使用示例**:
```bash
# 基础使用
ai-tran-cli

# 快速翻译
echo "hello" | ai-tran-cli -q

# 翻译成英文
echo "你好" | ai-tran-cli -q -t en

# 指定provider
ai-tran-cli -p claude

# 配置API密钥
ai-tran-cli config --provider openai --api-key sk-xxx

# 查看配置
ai-tran-cli config --show
```

---

## 4. 技术规格

### 4.1 技术栈

**语言和框架**:
- 语言: Rust 1.75+
- 异步运行时: Tokio 1.x
- TUI框架: Ratatui 0.26
- 终端控制: Crossterm 0.27

**核心依赖**:
```toml
[dependencies]
ratatui = "0.26"
crossterm = "0.27"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "stream"] }
eventsource-stream = "0.2"
futures = "0.3"
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
async-trait = "0.1"
arboard = "3.3"
dotenv = "0.15"
```

---

### 4.2 架构设计

**模块划分**:
```
src/
├── main.rs              # 入口，处理命令行参数
├── app/                 # 应用状态管理
│   ├── app.rs           # App 结构
│   └── message.rs       # Message 数据模型
├── ui/                  # TUI 界面
│   ├── chat.rs          # 聊天区域渲染
│   ├── input.rs         # 输入框渲染
│   ├── layout.rs        # 布局管理
│   └── statusbar.rs     # 状态栏
├── providers/           # 翻译 Provider
│   ├── trait.rs         # TranslationProvider trait
│   └── openai.rs        # OpenAI 实现
├── config/              # 配置管理
│   └── settings.rs      # 配置结构
├── events/              # 事件处理
│   └── handler.rs       # 键盘事件
└── utils/               # 工具函数
    └── clipboard.rs     # 剪贴板操作
```

---

### 4.3 数据模型

#### Message 结构
```rust
pub struct Message {
    pub id: usize,
    pub text: String,               // 用户输入
    pub translation: String,        // 翻译结果（支持增量更新）
    pub translation_complete: bool, // 是否完成
    pub status: MessageStatus,
    pub timestamp: DateTime<Utc>,
    pub provider: String,
}

pub enum MessageStatus {
    Pending,      // 等待翻译
    Streaming,    // 流式传输中
    Success,      // 翻译成功
    Error(String), // 翻译失败
}
```

#### App 状态
```rust
pub struct App {
    pub messages: Vec<Message>,
    pub input: String,
    pub scroll: usize,
    pub should_quit: bool,
    pub provider: Box<dyn TranslationProvider>,
    pub config: Config,
    pub display_mode: DisplayMode,
    pub notification: Option<(String, Instant)>,
    // ... 其他字段
}
```

---

### 4.4 API 集成

#### OpenAI Compatible API

**请求格式**:
```json
POST https://api.siliconflow.cn/v1/chat/completions
Authorization: Bearer sk-xxx
Content-Type: application/json

{
  "model": "deepseek-ai/DeepSeek-V3.2-Exp",
  "messages": [
    {
      "role": "system",
      "content": "You are a professional translator."
    },
    {
      "role": "user",
      "content": "Translate to Chinese: Hello world"
    }
  ],
  "stream": true
}
```

**流式响应格式**:
```
data: {"choices":[{"delta":{"content":"你"}}]}

data: {"choices":[{"delta":{"content":"好"}}]}

data: {"choices":[{"delta":{"content":"世界"}}]}

data: [DONE]
```

---

## 5. 性能指标

### 5.1 性能目标

| 指标 | 目标值 | 说明 |
|------|--------|------|
| 二进制大小 | < 5MB | Release模式，strip后 |
| 启动时间 | < 100ms | 冷启动到TUI显示 |
| 内存占用 | < 10MB | 运行时峰值 |
| 首token延迟 | < 500ms | API响应到首个字符显示 |
| UI刷新率 | 60 FPS | 流式更新时保持流畅 |

---

### 5.2 优化策略

**二进制优化**:
```toml
[profile.release]
opt-level = "z"     # 优化大小
lto = true          # Link Time Optimization
codegen-units = 1   # 更好的优化
strip = true        # 移除调试符号
```

**运行时优化**:
- 使用 tokio 单线程运行时（足够用）
- 按需加载配置
- 复用 HTTP 连接池

---

## 6. 用户场景

### 6.1 场景1：日常翻译
```
用户需求: 快速翻译一个英文句子

操作流程:
1. 运行 ai-tran-cli
2. 输入 "How are you?"
3. 按 Enter
4. 看到流式翻译结果 "你好吗？"
5. 按 Ctrl+Y 复制
6. 粘贴到其他应用
```

### 6.2 场景2：批量翻译
```
用户需求: 翻译文件中的所有句子

操作流程:
1. 准备 input.txt 文件
2. 运行 cat input.txt | ai-tran-cli -q > output.txt
3. 查看 output.txt 获取翻译结果
```

### 6.3 场景3：脚本集成
```bash
#!/bin/bash
# 自动翻译 commit message

git log -1 --pretty=%B | ai-tran-cli -q
```

---

## 7. 测试计划

### 7.1 功能测试

- [ ] TUI 界面正常显示
- [ ] 流式翻译实时更新
- [ ] 剪贴板复制功能
- [ ] 显示模式切换
- [ ] 管道输入处理
- [ ] 快速模式 (-q)
- [ ] 配置文件加载

### 7.2 性能测试

- [ ] 100条消息滚动性能
- [ ] 长文本翻译（1000字）
- [ ] 并发翻译请求
- [ ] 内存泄漏检测

### 7.3 兼容性测试

- [ ] macOS (测试优先)
- [ ] Linux
- [ ] Windows (可选)

---

## 8. 发布计划

### 8.1 版本规划

**v0.1.0 - MVP** (当前目标)
- ✅ 基础TUI界面
- ✅ 流式翻译
- ✅ 剪贴板功能
- ✅ OpenAI Provider

**v0.2.0** (计划中)
- 多Provider支持 (Claude, Gemini)
- 配置UI界面
- 翻译历史保存

**v1.0.0** (未来)
- 完整的错误处理
- 性能优化
- 完善的文档

---

### 8.2 发布清单

- [ ] 编写 README.md
- [ ] 编写 CHANGELOG.md
- [ ] 添加 LICENSE
- [ ] GitHub Release
- [ ] Crates.io 发布（可选）

---

## 9. 附录

### 9.1 术语表

| 术语 | 说明 |
|------|------|
| TUI | Terminal User Interface，终端用户界面 |
| SSE | Server-Sent Events，服务器推送事件 |
| Provider | 翻译服务提供商 |
| Streaming | 流式传输 |

### 9.2 参考项目

- [openai/codex](https://github.com/openai/codex) - UI参考
- [ratatui](https://github.com/ratatui-org/ratatui) - TUI框架
- [bat](https://github.com/sharkdp/bat) - Rust CLI最佳实践

---

**文档版本**: v1.0
**最后更新**: 2025-11-07
**作者**: AI Translation CLI Team
