# figmaa

Interface design and prototyping in a native Rust app, with an AI agent built in.

Runs on macOS, Windows, Linux and FreeBSD, and in the browser via WebAssembly. Status: early alpha.

## Features

- **Layout:** frames, auto layout (wrap and grid), constraints, layout grids.
- **Components:** main components, instances with overrides, variants, component properties, an
  Assets panel.
- **Styles and variables:** paint, text, effect and grid styles; colour, number, string and boolean
  variables with modes.
- **Vectors:** vector networks, Pen and Pencil tools, boolean operations, variable-width strokes.
- **Paints and effects:** solid and gradient fills, image fills, blend modes, shadows, blurs, masks.
- **Text:** in-place editing, mixed styles, OpenType features, variable fonts, right-to-left text.
- **Prototyping:** connections, transitions, variables with conditionals, a presentation view.
- **Dev Mode:** inspect, measurements, annotations, code output (CSS, HTML, React and others).
- **Files:** `.figcraft` files on your own disk; SVG, PNG, JPG, WebP and PDF import and export.

## AI agent

The **Agent** tab in the right sidebar (desktop) is a chat that edits the open file. Describe a
change and the model makes it, aware of the layers you have selected.

- Runs through your own Claude Code login (`claude -p`) or any OpenAI-compatible endpoint,
  including a local one.
- Asks for your consent before it sends anything to a destination.
- Is confined to the document: a short list of tools, no file reads or writes, no network access
  for its tool calls, and text inside the file is treated as untrusted.
- Each batch of changes is its own undo step.

Details: [`docs/mcp.md`](docs/mcp.md#agent-tab).

## Automation

Every menu item and tool is a command, and the same commands run from the UI, the CLI, a JSON
control channel and an MCP server.

```sh
figcraft-cli commands                    # list every command
figcraft-cli run script.json --out a.figcraft
figcraft-cli mcp                         # MCP server for agents
```

See [`docs/cli.md`](docs/cli.md), [`docs/mcp.md`](docs/mcp.md) and
[`docs/control-protocol.md`](docs/control-protocol.md).

## Build and run

```sh
git clone https://github.com/its-ash/figmaa
cd figmaa
cargo run --release -p figcraft      # the desktop app
cargo test --workspace               # the tests
cargo xtask ci                       # fmt, clippy, tests and checks
cargo xtask wasm                     # the web build (needs trunk)
```

The crates and binaries still carry their original `figcraft` names for now.

## Privacy

- No account and no telemetry. Files stay on your disk.
- **Agent tab:** your messages, the selected layers' details and whatever the model reads with its
  tools go to the model you choose (Claude Code's provider or your endpoint). A local endpoint keeps
  it on your machine.
- **Live sharing** uploads the file to a server. The default is still the upstream public server
  (`collab.craftapps.dev`); set your own in Preferences, or share on the local network.

## License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE), at your option.

figmaa is a fork of [storytold/figcraft](https://github.com/storytold/figcraft) and is developed
independently; it is not affiliated with or endorsed by its authors. The original copyright notice,
required by both licences, is kept: Copyright (c) 2026 ArtCraft Team and the FigCraft contributors.
See [NOTICE](NOTICE) and [ATTRIBUTION.md](ATTRIBUTION.md).
