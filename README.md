# Zen

A fast and simple CLI tool to scaffold new projects instantly.

> This is my first Rust project. I built it because I was tired of manually creating folders, setting up git, and doing the same repetitive steps every time I started a new project.

## Usage

```
zen create [language] [path]
```

## Examples

```bash
zen create rust my-project
zen create python C:\Coding\my-project
zen create javascript
```

## Supported Languages

| Language   | Alias        |
|------------|--------------|
| Rust       | rust         |
| Python     | python       |
| C++        | cpp          |
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

| Command                        | Description                        |
|--------------------------------|------------------------------------|
| `zen create [language] [path]` | Scaffold a new project             |
| `zen list`                     | List all supported languages       |
| `zen help`                     | Show available commands            |

## Built with

Rust