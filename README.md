# Serena JetBrains Symbol Bug - Minimal Reproduction

## Bug Report: KeyError in `jet_brains_get_symbols_overview` with impl blocks

### Description
The `jet_brains_get_symbols_overview` tool returns a `KeyError: 'name_path'` when called on a Rust file containing an `impl` block for a struct. Files without impl blocks work correctly.

### How to Reproduce

This repository contains two minimal Rust files to demonstrate the bug:

#### File 1: `src/lib_without_impl.rs` (Works correctly ✓)
Contains a simple struct and function WITHOUT an impl block.

```rust
pub struct Person {
    pub name: String,
    pub age: u32,
}

pub fn greet(person: &Person) -> String {
    format!("Hello, {}!", person.name)
}
```

**Tool call:**
```
mcp__serena__jet_brains_get_symbols_overview(relative_path="src/lib_without_impl.rs")
```

**Result:**
```json
{"STRUCT_ITEM": ["Person"], "FUNCTION": ["greet"]}
```

✓ **Status:** Works as expected

---

#### File 2: `src/lib_with_impl.rs` (Fails with KeyError ✗)
Contains the same struct BUT with an impl block added.

```rust
pub struct Person {
    pub name: String,
    pub age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }

    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

pub fn display_person(person: &Person) -> String {
    format!("{} is {} years old", person.name, person.age)
}
```

**Tool call:**
```
mcp__serena__jet_brains_get_symbols_overview(relative_path="src/lib_with_impl.rs")
```

**Result:**
```
Error executing tool: KeyError - 'name_path'
```

✗ **Status:** Fails with KeyError

### Key Findings
- Files **without** impl blocks: Tool works correctly
- Files **with** impl blocks: Tool crashes with `KeyError: 'name_path'`
- The presence of an impl block triggers the bug, regardless of the struct definition

### Root Cause Analysis
The tool appears to be missing or incorrectly handling the `name_path` field when processing impl blocks. The internal data structure returned for files with impl blocks is missing this required key, causing a KeyError when the tool tries to access it.

### Environment
- **OS:** macOS 26.2 (25C56)
- **IDE:** IntelliJ IDEA Ultimate 2025.3.1
- **Rust plugin version**: 253.29346.139
- **Serena Version:** 2023.2.5 and 2023.2.6-SNAPSHOT
- **Rust Version:** 1.92.0
- **Claude Code Version**: 2.1.5

### Logs

```
INFO  2026-01-12 13:05:45,858 [MainThread] serena.cli:start_mcp_server:229 - Initializing Serena MCP server
INFO  2026-01-12 13:05:45,858 [MainThread] serena.cli:start_mcp_server:230 - Storing logs in /Users/kristof/.serena/logs/2026-01-12/mcp_20260112-130545.txt
INFO  2026-01-12 13:05:45,859 [MainThread] serena.config.serena_config:from_config_file:486 - Loading Serena configuration from /Users/kristof/.serena/serena_config.yml
INFO  2026-01-12 13:05:45,894 [MainThread] serena.agent:__init__:237 - Will record tool usage statistics with token count estimator: CHAR_COUNT.
INFO  2026-01-12 13:05:45,909 [MainThread] serena.agent:__init__:241 - Starting Serena server (version=0.1.4, process id=28228, parent process id=27996; language backend=JETBRAINS)
INFO  2026-01-12 13:05:45,909 [MainThread] serena.agent:__init__:245 - Configuration file: /Users/kristof/.serena/serena_config.yml
INFO  2026-01-12 13:05:45,910 [MainThread] serena.agent:__init__:246 - Available projects: ai-skill-framework, fixest_benchmarks, hiring-coding-task, impact-study, power-interests, pydvl, pydvl, pyfixest, serena, serena-jetbrains-symbol-bug, tfl-jhub-deploy-identity, tfl-pydata2024-pydvl, tfl-training-explainable-ai, tfl-training-trustworthy-engineering, tfl-website, twai-pipeline
INFO  2026-01-12 13:05:45,910 [MainThread] serena.agent:__init__:247 - Loaded tools (39): read_file, create_text_file, list_dir, find_file, replace_content, delete_lines, replace_lines, insert_at_line, search_for_pattern, restart_language_server, get_symbols_overview, find_symbol, find_referencing_symbols, replace_symbol_body, insert_after_symbol, insert_before_symbol, rename_symbol, write_memory, read_memory, list_memories, delete_memory, edit_memory, execute_shell_command, open_dashboard, activate_project, remove_project, switch_modes, get_current_config, check_onboarding_performed, onboarding, think_about_collected_information, think_about_task_adherence, think_about_whether_you_are_done, summarize_changes, prepare_for_new_conversation, initial_instructions, jet_brains_find_symbol, jet_brains_find_referencing_symbols, jet_brains_get_symbols_overview
INFO  2026-01-12 13:05:45,910 [MainThread] serena.config.serena_config:start:360 - Loading project instance for RegisteredProject[project_root=/Users/kristof/Projects/serena-jetbrains-symbol-bug, project_config=ProjectConfig[project_name=serena-jetbrains-symbol-bug]] starting ...
INFO  2026-01-12 13:05:45,910 [MainThread] serena.util.file_system:start:360 - Loading of .gitignore files starting ...
INFO  2026-01-12 13:05:45,911 [MainThread] serena.util.file_system:_load_gitignore_files:148 - Processing .gitignore file: /Users/kristof/Projects/serena-jetbrains-symbol-bug/.serena/.gitignore
INFO  2026-01-12 13:05:45,911 [MainThread] serena.util.file_system:_load_gitignore_files:148 - Processing .gitignore file: /Users/kristof/Projects/serena-jetbrains-symbol-bug/.idea/.gitignore
INFO  2026-01-12 13:05:45,912 [MainThread] serena.util.file_system:stop:367 - Loading of .gitignore files completed in 0.001 seconds
INFO  2026-01-12 13:05:45,912 [MainThread] serena.config.serena_config:stop:367 - Loading project instance for RegisteredProject[project_root=/Users/kristof/Projects/serena-jetbrains-symbol-bug, project_config=ProjectConfig[project_name=serena-jetbrains-symbol-bug]] completed in 0.002 seconds
INFO  2026-01-12 13:05:45,912 [MainThread] serena.agent:load_project_from_path_or_name:589 - Found registered project 'serena-jetbrains-symbol-bug' at path /Users/kristof/Projects/serena-jetbrains-symbol-bug
INFO  2026-01-12 13:05:45,912 [MainThread] serena.agent:_single_project_context_tool_inclusion_definitions:384 - Applying tool inclusion/exclusion definitions for single-project context based on project 'serena-jetbrains-symbol-bug'
INFO  2026-01-12 13:05:45,913 [MainThread] serena.agent:apply:137 - SerenaAgentContext[name='claude-code'] excluded 5 tools: create_text_file, read_file, execute_shell_command, prepare_for_new_conversation, replace_content
INFO  2026-01-12 13:05:45,913 [MainThread] serena.agent:apply:137 - ToolInclusionDefinition(excluded_tools=['activate_project', 'get_current_config'], included_optional_tools=(), fixed_tools=()) excluded 2 tools: activate_project, get_current_config
INFO  2026-01-12 13:05:45,913 [MainThread] serena.agent:apply:135 - SerenaAgentMode[name='jetbrains'] included 3 tools: jet_brains_find_symbol, jet_brains_find_referencing_symbols, jet_brains_get_symbols_overview
INFO  2026-01-12 13:05:45,913 [MainThread] serena.agent:apply:137 - SerenaAgentMode[name='jetbrains'] excluded 3 tools: find_symbol, find_referencing_symbols, get_symbols_overview
INFO  2026-01-12 13:05:45,914 [MainThread] serena.agent:__init__:274 - Number of exposed tools: 21
INFO  2026-01-12 13:05:45,922 [MainThread] serena.agent:_update_active_tools:525 - Active tools (21): check_onboarding_performed, delete_memory, edit_memory, find_file, initial_instructions, insert_after_symbol, insert_before_symbol, jet_brains_find_referencing_symbols, jet_brains_find_symbol, jet_brains_get_symbols_overview, list_dir, list_memories, onboarding, read_memory, rename_symbol, replace_symbol_body, search_for_pattern, think_about_collected_information, think_about_task_adherence, think_about_whether_you_are_done, write_memory
INFO  2026-01-12 13:05:45,923 [MainThread] serena.agent:load_project_from_path_or_name:589 - Found registered project 'serena-jetbrains-symbol-bug' at path /Users/kristof/Projects/serena-jetbrains-symbol-bug
INFO  2026-01-12 13:05:45,923 [MainThread] serena.agent:_activate_project:562 - Activating serena-jetbrains-symbol-bug at /Users/kristof/Projects/serena-jetbrains-symbol-bug
INFO  2026-01-12 13:05:45,923 [MainThread] serena.agent:_update_active_tools:525 - Active tools (21): check_onboarding_performed, delete_memory, edit_memory, find_file, initial_instructions, insert_after_symbol, insert_before_symbol, jet_brains_find_referencing_symbols, jet_brains_find_symbol, jet_brains_get_symbols_overview, list_dir, list_memories, onboarding, read_memory, rename_symbol, replace_symbol_body, search_for_pattern, think_about_collected_information, think_about_task_adherence, think_about_whether_you_are_done, write_memory
INFO  2026-01-12 13:05:45,927 [MainThread] serena.dashboard:run_in_thread:591 - Starting dashboard (listen_address=127.0.0.1, port=24283)
INFO  2026-01-12 13:05:45,927 [MainThread] serena.agent:__init__:311 - Serena web dashboard started at http://127.0.0.1:24283/dashboard/index.html
INFO  2026-01-12 13:05:45,930 [MainThread] serena.agent:create_system_prompt:492 - Generating system prompt with available_tools=(see exposed tools), available_markers={'ToolMarkerCanEdit', 'ToolMarkerDoesNotRequireActiveProject', 'WriteMemoryTool', 'InsertBeforeSymbolTool', 'DeleteMemoryTool', 'EditMemoryTool', 'JetBrainsFindReferencingSymbolsTool', 'RenameSymbolTool', 'ToolMarkerSymbolicEdit', 'ReplaceSymbolBodyTool', 'InitialInstructionsTool', 'ToolMarkerOptional', 'JetBrainsFindSymbolTool', 'ToolMarkerSymbolicRead', 'JetBrainsGetSymbolsOverviewTool', 'InsertAfterSymbolTool'}
INFO  2026-01-12 13:05:45,932 [MainThread] serena.agent:create_system_prompt:504 - System prompt:
You are a professional coding agent. 
You have access to semantic coding tools upon which you rely heavily for all your work.
You operate in a resource-efficient and intelligent manner, always keeping in mind to not read or generate
content that is not needed for the task at hand.

Some tasks may require you to understand the architecture of large parts of the codebase, while for others,
it may be enough to read a small set of symbols or a single file.
You avoid reading entire files unless it is absolutely necessary, instead relying on intelligent step-by-step 
acquisition of information. Once you have read a full file, it does not make
sense to analyse it with the symbolic read tools; you already have the information.

You can achieve intelligent reading of code by using the symbolic tools for getting an overview of symbols and
the relations between them, and then only reading the bodies of symbols that are necessary to complete the task at hand. 
You can use the standard tools like list_dir, find_file and search_for_pattern if you need to.
Where appropriate, you pass the `relative_path` parameter to restrict the search to a specific file or directory.

If you are unsure about a symbol's name or location, you can use the `search_for_pattern` tool, which allows fast
and flexible search for patterns in the codebase. In this way, you can first find candidates for symbols or files,
and then proceed with the symbolic tools.



Symbols are identified by their `name_path` and `relative_path` (see the description of the `find_symbol` tool).
You can get information about the symbols in a file by using the `get_symbols_overview` tool or use the `find_symbol` to search. 
You only read the bodies of symbols when you need to (e.g. if you want to fully understand or edit it).
For example, if you are working with Python code and already know that you need to read the body of the constructor of the class Foo, you can directly
use `find_symbol` with name path pattern `Foo/__init__` and `include_body=True`. If you don't know yet which methods in `Foo` you need to read or edit,
you can use `find_symbol` with name path pattern `Foo`, `include_body=False` and `depth=1` to get all (top-level) methods of `Foo` before proceeding
to read the desired methods with `include_body=True`.
You can understand relationships between symbols by using the `find_referencing_symbols` tool.



You generally have access to memories and it may be useful for you to read them.
You infer whether memories are relevant based on their names.


The context and modes of operation are described below. These determine how to interact with your user
and which kinds of interactions are expected of you.

Context description:
You are running in a CLI coding agent context where file operations, basic (line-based) edits and reads 
as well as shell commands are handled by your own, internal tools.

If Serena's tools can be used to achieve your task, you should prioritize them.
In particular, it is important that you avoid reading entire source code files unless it is strictly necessary!
Instead, for exploring and reading code in a token-efficient manner, use Serena's overview and symbolic search tools.
For non-code files or for reads where you don't know the symbol's name path, you can use the pattern search tool.

Modes descriptions:

You are operating in interactive mode. You should engage with the user throughout the task, asking for clarification
whenever anything is unclear, insufficiently specified, or ambiguous.

Break down complex tasks into smaller steps and explain your thinking at each stage. When you're uncertain about
a decision, present options to the user and ask for guidance rather than making assumptions.

Focus on providing informative results for intermediate steps, such that the user can follow along with your progress and
provide feedback as needed.

You are operating in editing mode. You can edit files with the provided tools.
You adhere to the project's code style and patterns.

Use symbolic editing tools whenever possible for precise code modifications.
If no explicit editing task has yet been provided, wait for the user to provide one. Do not be overly eager.

When writing new code, think about where it belongs best. Don't generate new files if you don't plan on actually
properly integrating them into the codebase.

You have two main approaches for editing code: (a) editing at the symbol level and (b) file-based editing.
The symbol-based approach is appropriate if you need to adjust an entire symbol, e.g. a method, a class, a function, etc.
It is not appropriate if you need to adjust just a few lines of code within a larger symbol.

**Symbolic editing**
Use symbolic retrieval tools to identify the symbols you need to edit.
If you need to replace the definition of a symbol, use the `replace_symbol_body` tool.
If you want to add some new code at the end of the file, use the `insert_after_symbol` tool with the last top-level symbol in the file. 
Similarly, you can use `insert_before_symbol` with the first top-level symbol in the file to insert code at the beginning of a file.
You can understand relationships between symbols by using the `find_referencing_symbols` tool. If not explicitly requested otherwise by the user,
you make sure that when you edit a symbol, the change is either backward-compatible or you find and update all references as needed.
The `find_referencing_symbols` tool will give you code snippets around the references as well as symbolic information.
You can assume that all symbol editing tools are reliable, so you never need to verify the results if the tools return without error.




You have hereby read the 'Serena Instructions Manual' and do not need to read it again.

The project with name 'serena-jetbrains-symbol-bug' at /Users/kristof/Projects/serena-jetbrains-symbol-bug is activated.
Programming languages: rust; file encoding: utf-8
INFO  2026-01-12 13:05:45,934 [MainThread] serena.cli:start_mcp_server:257 - Starting MCP server …
INFO  2026-01-12 13:05:45,955 [MainThread] serena.mcp:_set_mcp_tools:259 - Starting MCP server with 21 tools: ['list_dir', 'find_file', 'search_for_pattern', 'replace_symbol_body', 'insert_after_symbol', 'insert_before_symbol', 'rename_symbol', 'write_memory', 'read_memory', 'list_memories', 'delete_memory', 'edit_memory', 'check_onboarding_performed', 'onboarding', 'think_about_collected_information', 'think_about_task_adherence', 'think_about_whether_you_are_done', 'initial_instructions', 'jet_brains_find_symbol', 'jet_brains_find_referencing_symbols', 'jet_brains_get_symbols_overview']
INFO  2026-01-12 13:05:45,955 [MainThread] serena.mcp:server_lifespan:334 - MCP server lifetime setup complete
INFO  2026-01-12 13:05:45,959 [MainThread] mcp.server.lowlevel.server:_handle_request:709 - Processing request of type ListToolsRequest
INFO  2026-01-12 13:05:45,960 [MainThread] mcp.server.lowlevel.server:_handle_request:709 - Processing request of type ListPromptsRequest
INFO  2026-01-12 13:05:45,961 [MainThread] mcp.server.lowlevel.server:_handle_request:709 - Processing request of type ListResourcesRequest
INFO  2026-01-12 13:06:16,589 [MainThread] mcp.server.lowlevel.server:_handle_request:709 - Processing request of type CallToolRequest
INFO  2026-01-12 13:06:16,589 [MainThread] serena.task_executor:issue_task:192 - Scheduling Task-1:JetBrainsGetSymbolsOverviewTool
INFO  2026-01-12 13:06:16,603 [SerenaAgentTaskExecutor] serena.task_executor:_process_task_queue:123 - Starting execution of Task-1:JetBrainsGetSymbolsOverviewTool
INFO  2026-01-12 13:06:16,603 [Task-1:JetBrainsGetSymbolsOverviewTool] serena.task_executor:start:360 - Task-1:JetBrainsGetSymbolsOverviewTool starting ...
INFO  2026-01-12 13:06:16,603 [Task-1:JetBrainsGetSymbolsOverviewTool] serena.tools.tools_base:_log_tool_application:205 - jet_brains_get_symbols_overview: relative_path='src/lib_without_impl.rs', depth=0, max_answer_chars=-1
INFO  2026-01-12 13:06:16,611 [Task-1:JetBrainsGetSymbolsOverviewTool] serena.tools.jetbrains_plugin_client:from_project:72 - Found JetBrains IDE service at port 24227 for project /Users/kristof/Projects/serena-jetbrains-symbol-bug
INFO  2026-01-12 13:06:16,661 [Task-1:JetBrainsGetSymbolsOverviewTool] serena.tools.tools_base:task:278 - Result: {"STRUCT_ITEM": ["Person"], "FUNCTION": ["greet"]}
INFO  2026-01-12 13:06:16,661 [Task-1:JetBrainsGetSymbolsOverviewTool] serena.task_executor:stop:367 - Task-1:JetBrainsGetSymbolsOverviewTool completed in 0.058 seconds
INFO  2026-01-12 13:06:16,663 [MainThread] mcp.server.lowlevel.server:_handle_request:709 - Processing request of type CallToolRequest
INFO  2026-01-12 13:06:16,663 [MainThread] serena.task_executor:issue_task:192 - Scheduling Task-2:JetBrainsGetSymbolsOverviewTool
INFO  2026-01-12 13:06:16,766 [SerenaAgentTaskExecutor] serena.task_executor:_process_task_queue:123 - Starting execution of Task-2:JetBrainsGetSymbolsOverviewTool
INFO  2026-01-12 13:06:16,766 [Task-2:JetBrainsGetSymbolsOverviewTool] serena.task_executor:start:360 - Task-2:JetBrainsGetSymbolsOverviewTool starting ...
INFO  2026-01-12 13:06:16,766 [Task-2:JetBrainsGetSymbolsOverviewTool] serena.tools.tools_base:_log_tool_application:205 - jet_brains_get_symbols_overview: relative_path='src/lib_with_impl.rs', depth=0, max_answer_chars=-1
ERROR 2026-01-12 13:06:16,775 [Task-2:JetBrainsGetSymbolsOverviewTool] serena.tools.tools_base:task:274 - Error executing tool: 'name_path'
Traceback (most recent call last):
  File "/Users/kristof/.cache/uv/archive-v0/6pa01dO_aUmh3XwwkxcR_/lib/python3.12/site-packages/serena/tools/tools_base.py", line 249, in task
    result = apply_fn(**kwargs)
             ^^^^^^^^^^^^^^^^^^
  File "/Users/kristof/.cache/uv/archive-v0/6pa01dO_aUmh3XwwkxcR_/lib/python3.12/site-packages/serena/tools/jetbrains_tools.py", line 157, in apply
    compact_symbols = self._transform_symbols_to_compact_format(symbols)
                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  File "/Users/kristof/.cache/uv/archive-v0/6pa01dO_aUmh3XwwkxcR_/lib/python3.12/site-packages/serena/tools/jetbrains_tools.py", line 121, in _transform_symbols_to_compact_format
    name_path = symbol["name_path"]
                ~~~~~~^^^^^^^^^^^^^
KeyError: 'name_path'
INFO  2026-01-12 13:06:16,776 [Task-2:JetBrainsGetSymbolsOverviewTool] serena.tools.tools_base:task:278 - Result: Error executing tool: KeyError - 'name_path'
INFO  2026-01-12 13:06:16,776 [Task-2:JetBrainsGetSymbolsOverviewTool] serena.task_executor:stop:367 - Task-2:JetBrainsGetSymbolsOverviewTool completed in 0.010 seconds

```