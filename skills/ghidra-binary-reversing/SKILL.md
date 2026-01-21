---
name: ghidra-binary-reversing
description: Use the pyghidra-mcp Ghidra headless MCP server with the mcporter CLI to reverse engineer binaries (import programs, list functions/symbols, decompile, xrefs, bytes, and graphs).
---

# Ghidra Binary Reversing (pyghidra-mcp)

Use this skill when a task requires binary reverse engineering with Ghidra via the **pyghidra-mcp** MCP server. The server exposes Ghidra capabilities (importing binaries, listing functions/symbols, decompiling, cross-references, etc.) as MCP tools.

## Quick start (mcporter/cli required)

1. **Start the server (recommended: streamable HTTP)**
   - `uvx pyghidra-mcp --project-path /path/to/ghidra-projects` (stdio)
   - `uvx pyghidra-mcp --transport streamable-http --project-path /path/to/ghidra-projects` (HTTP)
   - The server defaults to streamable HTTP and listens on `http://127.0.0.1:8000/mcp`; SSE is legacy.

2. **Discover tools with mcporter**
   - Stdio:
     ```bash
     npx mcporter list --stdio "uvx pyghidra-mcp --project-path /path/to/ghidra-projects"
     ```
   - HTTP:
     ```bash
     npx mcporter list --http-url http://127.0.0.1:8000/mcp
     ```
   - `mcporter list` shows tool names and JSON schemas.

3. **Call tools with mcporter**
   - Example (stdio):
     ```bash
     npx mcporter call --stdio "uvx pyghidra-mcp --project-path /path/to/ghidra-projects" \
       pyghidra_mcp.list_project_binaries
     ```
   - Example (HTTP):
     ```bash
     npx mcporter call --http-url http://127.0.0.1:8000/mcp \
       pyghidra_mcp.list_project_binaries
     ```
   - `mcporter call` accepts JSON arguments after the tool name when required.

## Core workflow

1. **Import and enumerate**
   - Use MCP tools like `list_project_binaries` and `list_project_program_info` to confirm the binary is loaded and see architecture/format details.

2. **Map the surface area**
   - Enumerate functions and symbols: `search_functions_by_name`, `search_symbols_by_name`.
   - Enumerate imports/exports: `list_imports`, `list_exports`.

3. **Pivot into code**
   - Decompile: `decompile_function` for high-value functions.
   - Cross-references: `list_cross_references` to locate callers/uses.

4. **Deep inspection**
   - Read bytes with `read_memory_bytes` when verifying constants or structures.
   - Build call graphs via `get_call_graph` for high-level flow.

## Notes & tips

- Prefer **streamable HTTP** for tool calls when possible; use stdio only when HTTP is not feasible.
- Always run `mcporter list` first to confirm the exact tool names and argument schemas before calling.
- If binaries are large or analysis is slow, scope to specific functions via name searches before decompiling.

## Docker option (when local Ghidra install is unavailable)

You can also run the server in Docker, mounting a binaries directory into the container and using the same mcporter HTTP flow.
