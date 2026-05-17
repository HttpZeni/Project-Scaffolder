# Zen 🧘

A fast and simple CLI tool to scaffold new projects instantly.

> This is my first Rust project. I built it because I was tired of manually creating folders, setting up git, and doing the same repetitive steps every time I started a new project.

## Usage

```
zen create [language] [path]
```

## Examples

```
zen create rust my-project
zen create python C:\Coding\my-project
zen create javascript
```

## Supported Languages

| Language | Alias |
|----------|-------|
| Rust | rust |
| Python | python |
| C++ | cpp |
| JavaScript | javascript |
| TypeScript | typescript |
| C# | csharp |
| Go | go |
| Java | java |
| Kotlin | kotlin |
| Lua | lua |
| Ruby | ruby |
| PHP | php |
| Bash | bash |
| PowerShell | powershell |

## What Zen creates

```
my-project/
├── src/
│   └── main.[ext]
├── .gitignore
└── README.md
```

Git is initialized automatically.

## Commands

| Command | Description |
|---------|-------------|
| `zen create [language] [path]` | Create a new project |
| `zen help` | Show available commands |

## Built with

Rust