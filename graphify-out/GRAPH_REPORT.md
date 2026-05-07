# Graph Report - src  (2026-05-07)

## Corpus Check
- 87 files ¡¤ ~52,249 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 1405 nodes ¡¤ 2666 edges ¡¤ 51 communities detected
- Extraction: 72% EXTRACTED ¡¤ 28% INFERRED ¡¤ 0% AMBIGUOUS ¡¤ INFERRED: 752 edges (avg confidence: 0.8)
- Token cost: 0 input ¡¤ 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_Agent Service|Agent Service]]
- [[_COMMUNITY_Chat Messaging|Chat Messaging]]
- [[_COMMUNITY_App Config & API|App Config & API]]
- [[_COMMUNITY_Auto Dream & Plugins|Auto Dream & Plugins]]
- [[_COMMUNITY_JavaScript Bridge|JavaScript Bridge]]
- [[_COMMUNITY_Localization|Localization]]
- [[_COMMUNITY_Skill Executor|Skill Executor]]
- [[_COMMUNITY_API Client & State|API Client & State]]
- [[_COMMUNITY_MCP Protocol & Prompts|MCP Protocol & Prompts]]
- [[_COMMUNITY_Voice & Stress Tests|Voice & Stress Tests]]
- [[_COMMUNITY_REPL|REPL]]
- [[_COMMUNITY_Magic Docs Service|Magic Docs Service]]
- [[_COMMUNITY_API Client|API Client]]
- [[_COMMUNITY_Context Management|Context Management]]
- [[_COMMUNITY_Built-in Skills|Built-in Skills]]
- [[_COMMUNITY_Plugin System|Plugin System]]
- [[_COMMUNITY_App Models & Main|App Models & Main]]
- [[_COMMUNITY_MCP Manager|MCP Manager]]
- [[_COMMUNITY_Hook System|Hook System]]
- [[_COMMUNITY_Sandbox Isolation|Sandbox Isolation]]
- [[_COMMUNITY_Session Consolidation|Session Consolidation]]
- [[_COMMUNITY_Session Management|Session Management]]
- [[_COMMUNITY_Project Initialization|Project Initialization]]
- [[_COMMUNITY_Built-in Tools|Built-in Tools]]
- [[_COMMUNITY_Note Editing|Note Editing]]
- [[_COMMUNITY_Transport Layer|Transport Layer]]
- [[_COMMUNITY_Git Operations|Git Operations]]
- [[_COMMUNITY_Syntax Highlighting|Syntax Highlighting]]
- [[_COMMUNITY_Command Registry|Command Registry]]
- [[_COMMUNITY_Sampling|Sampling]]
- [[_COMMUNITY_CLI Commands|CLI Commands]]
- [[_COMMUNITY_History Tracking|History Tracking]]
- [[_COMMUNITY_Skill Module|Skill Module]]
- [[_COMMUNITY_MCP Configuration|MCP Configuration]]
- [[_COMMUNITY_Agent System|Agent System]]
- [[_COMMUNITY_Command Execution Tool|Command Execution Tool]]
- [[_COMMUNITY_File Edit Tool|File Edit Tool]]
- [[_COMMUNITY_File Read Tool|File Read Tool]]
- [[_COMMUNITY_File Write Tool|File Write Tool]]
- [[_COMMUNITY_List Files Tool|List Files Tool]]
- [[_COMMUNITY_Search Tool|Search Tool]]
- [[_COMMUNITY_Voice Input|Voice Input]]
- [[_COMMUNITY_JS Bridge Types|JS Bridge Types]]
- [[_COMMUNITY_JS Bridge Types|JS Bridge Types]]
- [[_COMMUNITY_JS Bridge Types|JS Bridge Types]]
- [[_COMMUNITY_GUI Message|GUI Message]]
- [[_COMMUNITY_Library Root|Library Root]]
- [[_COMMUNITY_CLI Arguments|CLI Arguments]]
- [[_COMMUNITY_CLI Commands Module|CLI Commands Module]]
- [[_COMMUNITY_Settings Config|Settings Config]]
- [[_COMMUNITY_Web Module|Web Module]]

## God Nodes (most connected - your core abstractions)
1. `home_dir()` - 31 edges
2. `Theme` - 29 edges
3. `JsBridge` - 28 edges
4. `ChatPanel` - 24 edges
5. `MemoryManager` - 20 edges
6. `PluginManager` - 19 edges
7. `Cli` - 18 edges
8. `McpServer` - 18 edges
9. `GitOperationsTool` - 17 edges
10. `NoteEditTool` - 17 edges

## Surprising Connections (you probably didn't know these)
- `main()` --calls--> `init()`  [INFERRED]
  src\web\main.rs ¡ú src\i18n\mod.rs
- `main()` --calls--> `run_gui()`  [INFERRED]
  src\web\main.rs ¡ú src\gui\app.rs
- `main()` --calls--> `start_server()`  [INFERRED]
  src\web\main.rs ¡ú src\web\server.rs
- `start()` --calls--> `init()`  [INFERRED]
  src\wasm\mod.rs ¡ú src\i18n\mod.rs

## Communities

### Community 0 - "Agent Service"
Cohesion: 0.03
Nodes (14): AgentsService, HistoryManager, LoadedPlugin, PluginLoader, PluginModule, MemoryManager, PluginRegistry, Resource (+6 more)

### Community 1 - "Chat Messaging"
Cohesion: 0.05
Nodes (16): Attachment, ChatMessage, ChatPanel, ContentPart, MessageRole, split_by_code_blocks(), split_inline_code(), SettingsPanel (+8 more)

### Community 2 - "App Config & API"
Cohesion: 0.04
Nodes (30): ApiConfig, ClaudeCodeApp, run_gui(), Cli, AppState, get_categories(), get_featured(), get_plugin() (+22 more)

### Community 3 - "Auto Dream & Plugins"
Cohesion: 0.04
Nodes (28): AutoDreamConfig, AutoDreamService, AutoDreamStatus, ConsolidatedInsight, ConsolidationState, MemoryEntry, EventListener, JsConvert (+20 more)

### Community 4 - "JavaScript Bridge"
Cohesion: 0.03
Nodes (14): JsBridge, String, ClaudeCodeWasm, TerminalApp, BrowserStorage, Database, GetRequest, IndexedDbStorage (+6 more)

### Community 5 - "Localization"
Cohesion: 0.04
Nodes (23): available_locales(), is_locale_available(), load_locale(), ChatMessageJs, init(), Language, Locale, start() (+15 more)

### Community 6 - "Skill Executor"
Cohesion: 0.06
Nodes (10): SkillExecutor, Tool, ToolError, ToolOutput, ToolRegistry, SkillRegistry, Task, TaskManagementTool (+2 more)

### Community 7 - "API Client & State"
Cohesion: 0.05
Nodes (23): AppState, ChatMessage, ChatRequest, ChatResponse, Choice, Conversation, Delta, MemoryState (+15 more)

### Community 8 - "MCP Protocol & Prompts"
Cohesion: 0.08
Nodes (7): McpMessage, Prompt, PromptArgument, PromptContent, PromptManager, PromptMessage, McpServer

### Community 9 - "Voice & Stress Tests"
Cohesion: 0.12
Nodes (10): run_stress_test(), StressTestResult, StressTestRunner, test_autodream_basic(), test_stress_result_finalization(), RecordingState, VoiceBackend, VoiceConfig (+2 more)

### Community 10 - "REPL"
Cohesion: 0.09
Nodes (25): Repl, test_repl_creation(), clear_screen(), format_inline_styles(), highlight_code_line(), init_terminal(), print_claude_message(), print_code_block() (+17 more)

### Community 11 - "Magic Docs Service"
Cohesion: 0.09
Nodes (7): MagicDocHeader, MagicDocInfo, MagicDocsConfig, MagicDocsService, MagicDocsStatus, ServiceManager, ServiceStatus

### Community 12 - "API Client"
Cohesion: 0.11
Nodes (6): ChatMessage, ChatResponse, Usage, WasmApiClient, LocaleAssets, LocaleLoader

### Community 13 - "Context Management"
Cohesion: 0.1
Nodes (7): ContextEntry, ContextManager, ContextPriority, ContextSource, ContextStats, ContextSummary, ContextWindow

### Community 14 - "Built-in Skills"
Cohesion: 0.06
Nodes (6): BuildSkill, BuiltinSkills, CommitSkill, DocumentSkill, ReviewSkill, TestSkill

### Community 15 - "Plugin System"
Cohesion: 0.09
Nodes (5): PluginCommandDef, PluginInfo, PluginManager, PluginManifest, PluginStatus

### Community 16 - "App Models & Main"
Cohesion: 0.08
Nodes (17): main(), ApiResponse, FeaturedPlugins, HealthResponse, InstallRequest, InstallResponse, MarketplaceStats, Plugin (+9 more)

### Community 17 - "MCP Manager"
Cohesion: 0.12
Nodes (5): McpError, McpManager, McpServerConnection, McpServerInfo, SessionManager

### Community 18 - "Hook System"
Cohesion: 0.11
Nodes (6): Hook, HookContext, HookHandlerType, HookManager, HookPoint, HookResult

### Community 19 - "Sandbox Isolation"
Cohesion: 0.11
Nodes (4): IsolationConfig, PluginSandbox, SandboxViolation, ViolationType

### Community 20 - "Session Consolidation"
Cohesion: 0.13
Nodes (6): ConsolidationConfig, ConsolidationEngine, ConsolidationResult, MemoryEntry, MemoryStatus, MemoryType

### Community 21 - "Session Management"
Cohesion: 0.11
Nodes (5): Session, SessionInfo, SessionManager, SessionMessage, SessionStatus

### Community 22 - "Project Initialization"
Cohesion: 0.14
Nodes (4): ProjectConfig, ProjectInitializer, ProjectTemplate, TemplateFile

### Community 23 - "Built-in Tools"
Cohesion: 0.13
Nodes (7): BuiltinCommandExecutor, BuiltinFileReadExecutor, BuiltinFileWriteExecutor, BuiltinSearchExecutor, McpTool, ToolExecutor, ToolRegistry

### Community 24 - "Note Editing"
Cohesion: 0.18
Nodes (3): Note, NoteEditTool, NoteFormat

### Community 25 - "Transport Layer"
Cohesion: 0.12
Nodes (5): StdioTransport, TcpTransport, Transport, TransportConfig, WebSocketTransport

### Community 26 - "Git Operations"
Cohesion: 0.25
Nodes (1): GitOperationsTool

### Community 27 - "Syntax Highlighting"
Cohesion: 0.24
Nodes (8): boost_brightness(), CodeHighlighter, format_code_block(), get_simple_code_style(), get_syntax_set(), get_theme_name(), get_theme_set(), syntect_style_to_egui_format()

### Community 28 - "Command Registry"
Cohesion: 0.16
Nodes (5): BuiltinCommandHandler, CommandHandler, CommandHandlerType, CommandRegistry, PluginCommand

### Community 29 - "Sampling"
Cohesion: 0.15
Nodes (6): PendingRequest, SamplingContent, SamplingMessage, SamplingRequest, SamplingResponse, SamplingUsage

### Community 30 - "CLI Commands"
Cohesion: 0.18
Nodes (10): CliArgs, Commands, ConfigCommands, MagicDocsCommands, McpCommands, MemoryCommands, PluginCommands, ServiceCommands (+2 more)

### Community 31 - "History Tracking"
Cohesion: 0.2
Nodes (4): HistoryEntry, HistoryFilter, HistoryStats, HistoryType

### Community 32 - "Skill Module"
Cohesion: 0.22
Nodes (6): Skill, SkillCategory, SkillContext, SkillError, SkillParams, SkillResult

### Community 33 - "MCP Configuration"
Cohesion: 0.29
Nodes (2): McpConfig, McpServerStatus

### Community 34 - "Agent System"
Cohesion: 0.25
Nodes (6): AgentDefinition, AgentMessage, AgentSession, AgentStatus, AgentStatusReport, AgentType

### Community 35 - "Command Execution Tool"
Cohesion: 0.32
Nodes (1): ExecuteCommandTool

### Community 36 - "File Edit Tool"
Cohesion: 0.32
Nodes (1): FileEditTool

### Community 37 - "File Read Tool"
Cohesion: 0.32
Nodes (1): FileReadTool

### Community 38 - "File Write Tool"
Cohesion: 0.32
Nodes (1): FileWriteTool

### Community 39 - "List Files Tool"
Cohesion: 0.32
Nodes (1): ListFilesTool

### Community 40 - "Search Tool"
Cohesion: 0.32
Nodes (1): SearchTool

### Community 41 - "Voice Input"
Cohesion: 0.5
Nodes (1): VoiceInput

### Community 42 - "JS Bridge Types"
Cohesion: 0.67
Nodes (1): bool

### Community 43 - "JS Bridge Types"
Cohesion: 0.67
Nodes (1): f64

### Community 44 - "JS Bridge Types"
Cohesion: 0.67
Nodes (1): i32

### Community 45 - "GUI Message"
Cohesion: 1.0
Nodes (1): GuiMessage

### Community 46 - "Library Root"
Cohesion: 1.0
Nodes (0): 

### Community 47 - "CLI Arguments"
Cohesion: 1.0
Nodes (0): 

### Community 48 - "CLI Commands Module"
Cohesion: 1.0
Nodes (0): 

### Community 49 - "Settings Config"
Cohesion: 1.0
Nodes (0): 

### Community 50 - "Web Module"
Cohesion: 1.0
Nodes (0): 

## Knowledge Gaps
- **137 isolated node(s):** `TemplateFile`, `CacheEntry`, `SshSession`, `SshStatus`, `SshCommandResult` (+132 more)
  These have ¡Ü1 connection - possible missing edges or undocumented components.
- **Thin community `GUI Message`** (2 nodes): `GuiMessage`, `mod.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Library Root`** (1 nodes): `lib.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `CLI Arguments`** (1 nodes): `args.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `CLI Commands Module`** (1 nodes): `commands.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Settings Config`** (1 nodes): `settings.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Web Module`** (1 nodes): `mod.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SamplingManager` connect `Agent Service` to `JavaScript Bridge`, `Sampling`?**
  _High betweenness centrality (0.034) - this node is a cross-community bridge._
- **Why does `ApiClient` connect `App Config & API` to `REPL`, `API Client & State`?**
  _High betweenness centrality (0.031) - this node is a cross-community bridge._
- **Why does `JsBridge` connect `JavaScript Bridge` to `Auto Dream & Plugins`?**
  _High betweenness centrality (0.031) - this node is a cross-community bridge._
- **Are the 28 inferred relationships involving `home_dir()` (e.g. with `.default()` and `.new()`) actually correct?**
  _`home_dir()` has 28 INFERRED edges - model-reasoned connections that need verification._
- **What connects `TemplateFile`, `CacheEntry`, `SshSession` to the rest of the system?**
  _137 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Agent Service` be split into smaller, more focused modules?**
  _Cohesion score 0.03 - nodes in this community are weakly interconnected._
- **Should `Chat Messaging` be split into smaller, more focused modules?**
  _Cohesion score 0.05 - nodes in this community are weakly interconnected._