<div align="center">

# 🔨 ForgeArch

**Universal Project Scaffolding Tool**

[![Release](https://img.shields.io/github/v/release/iamjuaness/ForgeArch?style=flat-square)](https://github.com/iamjuaness/ForgeArch/releases/latest)
[![CI](https://img.shields.io/github/actions/workflow/status/iamjuaness/ForgeArch/ci.yml?branch=main&style=flat-square&label=CI)](https://github.com/iamjuaness/ForgeArch/actions)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](LICENSE)
[![Platform](https://img.shields.io/badge/platform-Linux%20%7C%20Windows-lightgrey?style=flat-square)](https://github.com/iamjuaness/ForgeArch/releases)

Bootstrap projects from battle-tested architecture templates in seconds.

[Quick Start](#-quick-start) •
[Installation](#-installation) •
[Usage](#-usage) •
[Templates](#-templates) •
[Custom Templates](#-custom-templates) •
[Contributing](#-contributing)

</div>

---

## 📋 Overview

**ForgeArch** (CLI: `forge`) is a fast, zero-dependency project scaffolding tool that generates production-ready project structures from embedded templates. Perfect for backend APIs, React frontends, monorepos, CLI tools, microservices, and infrastructure setups.

### ✨ Key Features

- 🚀 **Fast & Lightweight** — Single binary, no runtime dependencies
- 📦 **34+ Built-in Templates** — Modern architectures ready to use
- 🎯 **Interactive & Scriptable** — Perfect for both manual use and CI/CD
- 🔒 **Safe Operations** — Prevents accidental overwrites unless forced
- 🎨 **Custom Templates** — Create and manage your own private templates
- 🔧 **Git Integration** — Optional repository initialization
- 🌐 **WSL Support** — Seamless integration with Windows Subsystem for Linux

---

## 🚀 Quick Start

```bash
# Install (Linux/macOS/WSL)
curl -fsSL https://raw.githubusercontent.com/iamjuaness/ForgeArch/master/install.sh | sh

# Activate in current session
source ~/.bashrc  # or ~/.zshrc for zsh

# Create your first project
forge new my-api --arch backend-api --git-init

# List all available templates
forge list
```

---

## 📥 Installation

### Automated Installation

**Linux / macOS / WSL**
```bash
curl -fsSL https://raw.githubusercontent.com/iamjuaness/ForgeArch/master/install.sh | sh

# Activate forge in your current terminal
source ~/.bashrc    # For bash
source ~/.zshrc     # For zsh
```

**Windows (PowerShell)**
```powershell
iwr https://raw.githubusercontent.com/iamjuaness/ForgeArch/master/install.ps1 | iex

# Activate forge in your current terminal
$env:Path = "$env:Path;$env:USERPROFILE\.local\bin"
```

The installer will:
- ✅ Detect your platform automatically
- ✅ Download the correct binary
- ✅ Install to `~/.local/bin/forge`
- ✅ Configure your PATH automatically
- ✅ Verify the installation

### Manual Installation

1. **Download** the latest release for your platform:
   - [Linux (x86_64)](https://github.com/iamjuaness/ForgeArch/releases/download/v0.1.0/forge_arch-x86_64-unknown-linux-gnu.tar.gz)
   - [Windows (x86_64)](https://github.com/iamjuaness/ForgeArch/releases/download/v0.1.0/forge_arch.exe-x86_64-pc-windows-msvc.zip)

2. **Extract** the archive
3. **Move** the binary to a directory in your PATH:
   ```bash
   # Linux/macOS/WSL
   mkdir -p ~/.local/bin
   mv forge ~/.local/bin/
   export PATH="$HOME/.local/bin:$PATH"

   # Windows (as Administrator)
   move forge.exe "C:\Program Files\ForgeArch\"
   ```

4. **Verify** installation:
   ```bash
   forge --version
   # or
   forge -v
   ```

### Build from Source

Requires [Rust](https://rustup.rs/) 1.70+

```bash
git clone https://github.com/iamjuaness/ForgeArch.git
cd ForgeArch
cargo build --release
# Binary at target/release/forge_arch
cp target/release/forge_arch ~/.local/bin/forge
```

---

## 📖 Usage

### Basic Commands

```bash
forge new <project-name>                    # Interactive mode
forge new <name> --arch <template>          # Non-interactive
forge list                                  # Show available templates
forge add-template <key>                    # Create custom template
forge remove-template <key>                 # Remove custom template
forge --help                                # Show all options
forge --version | forge -v                  # Show version
```

### Command Reference

| Command | Description |
|---------|-------------|
| `forge new <name>` | Create a new project (interactive if no --arch) |
| `forge list` | List all available architecture templates |
| `forge add-template <key>` | Create a new custom local template |
| `forge remove-template <key>` | Remove a custom local template |
| `forge --version` or `-v` | Display version information |
| `forge --help` | Show help information |

### Common Options

| Option | Description |
|--------|-------------|
| `--arch <template>` | Choose architecture template |
| `--git-init` | Initialize git repository after creation |
| `--no-readme` | Skip README.md generation |
| `--force` | Overwrite existing directory |

### Examples

**Interactive project creation:**
```bash
forge new my-project
# Prompts you to select a template from the list
```

**Backend API with Git initialization:**
```bash
forge new my-api --arch backend-api --git-init
```

**Microservices architecture:**
```bash
forge new my-services --arch microservices --git-init
```

**Next.js frontend:**
```bash
forge new my-app --arch nextjs-app
```

**Kubernetes application:**
```bash
forge new k8s-app --arch kubernetes-app --git-init
```

**Force overwrite existing project:**
```bash
forge new existing-project --arch monorepo --force
```

**Machine Learning project:**
```bash
forge new ml-project --arch machine-learning
```

**List all available templates:**
```bash
forge list
```

---

## 🏗️ Built-in Templates

ForgeArch includes **34 production-ready templates**:

### Backend Frameworks
| Template | Description |
|----------|-------------|
| `backend-api` | RESTful API with layered architecture |
| `backend-graphql` | GraphQL API with schema-first approach |
| `fastapi-backend` | Modern Python API with FastAPI |
| `django-backend` | Full-featured Django web application |
| `nestjs-backend` | Enterprise Node.js framework with TypeScript |
| `go-microservice` | Go microservice with clean architecture |
| `spring-boot` | Java Spring Boot REST API |

### Frontend Frameworks
| Template | Description |
|----------|-------------|
| `frontend-react` | React app with modern tooling |
| `nextjs-app` | Next.js 14+ with App Router |
| `vue-frontend` | Vue 3 with Composition API |
| `angular-frontend` | Enterprise Angular application |
| `svelte-frontend` | SvelteKit modern framework |

### Fullstack & Monorepos
| Template | Description |
|----------|-------------|
| `fullstack-mern` | MongoDB / Express / React / Node |
| `monorepo` | Multi-package workspace structure |
| `nx-monorepo` | Nx workspace with apps and libraries |
| `turborepo` | High-performance monorepo with Turborepo |

### Mobile Development
| Template | Description |
|----------|-------------|
| `mobile-react-native` | Cross-platform React Native app |
| `flutter-app` | Flutter mobile application |

### Cloud & DevOps
| Template | Description |
|----------|-------------|
| `serverless` | AWS Lambda / Serverless Framework |
| `microservices` | Microservices repository scaffold |
| `kubernetes-app` | Containerized app with K8s manifests |
| `infrastructure-terraform` | Terraform IaC with modules |
| `api-gateway` | API gateway with rate limiting |

### Advanced Architectures
| Template | Description |
|----------|-------------|
| `clean-architecture` | Hexagonal/Clean architecture with DDD |
| `event-driven` | Event-driven with message brokers |
| `ddd-bounded-contexts` | Domain-Driven Design structure |
| `cqrs-event-sourcing` | CQRS + Event Sourcing pattern |
| `graphql-federation` | Federated GraphQL with subgraphs |

### Specialized
| Template | Description |
|----------|-------------|
| `machine-learning` | ML/AI project with MLOps structure |
| `python-data-science` | Data science project (notebooks, data, src) |
| `cli-rust` | Rust command-line application |
| `electron-desktop` | Cross-platform desktop app with Electron |
| `chrome-extension` | Browser extension with manifest V3 |
| `blockchain-dapp` | Decentralized app with smart contracts |

---

## 🎨 Custom Templates

Create and manage your own private templates stored locally on your machine.

### Create a Custom Template

```bash
# Create a new template skeleton
forge add-template my-backend

# This opens an editor with a template skeleton to customize
```

### Template File Location

Templates are stored at:
- **Linux/macOS/WSL**: `~/.config/forge/templates/local_templates.json`
- **Windows**: `%APPDATA%\forge\templates\local_templates.json`

### Template Structure

```json
{
  "my-backend": {
    "name": "My Custom Backend",
    "description": "Custom API with my preferred structure",
    "structure": [
      "src/controllers",
      "src/services",
      "src/models",
      "src/middleware",
      "tests/unit",
      "tests/integration",
      "docs"
    ],
    "files": {
      ".gitignore": "backend",
      ".env.example": "backend",
      "Dockerfile": "backend"
    }
  }
}
```

### Template Fields

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Human-friendly template name |
| `description` | string | Short description of the template |
| `structure` | array | List of folders to create (supports nested paths) |
| `files` | object | Mapping of filename → template kind |

### File Template Kinds

Available file template kinds:
- `backend` — Backend-specific files (.gitignore, .env.example, Dockerfile)
- `frontend` — Frontend-specific files (.gitignore, .env.example)
- `monorepo` — Monorepo files (package.json, .gitignore)

### Edit Custom Template

```bash
# Manually edit with your preferred editor
nano ~/.config/forge/templates/local_templates.json
vim ~/.config/forge/templates/local_templates.json
code ~/.config/forge/templates/local_templates.json
```

### Use Custom Template

```bash
forge new my-project --arch my-backend
```

### Remove Custom Template

```bash
forge remove-template my-backend
```

### Editor Configuration

ForgeArch respects your editor preferences:

```bash
# Set preferred editor (priority order)
export FORGE_EDITOR="nano"     # Highest priority
export VISUAL="vim"            # Second priority
export EDITOR="code --wait"    # Third priority

# Add to your shell config
echo 'export FORGE_EDITOR="nano"' >> ~/.bashrc
```

**Default editor priority:**
1. `$FORGE_EDITOR`
2. `$VISUAL`
3. `$EDITOR`
4. `nano` (WSL/Linux)
5. `vim`
6. `code --wait` (VS Code)
7. `notepad.exe` (Windows)

### WSL Editor Tips

If using VS Code in WSL, ForgeArch automatically converts paths:

```bash
# VS Code opens the correct WSL file
forge add-template my-template
```

For terminal editors in WSL:
```bash
export FORGE_EDITOR="nano"  # Recommended for WSL
```

---

## 🛠️ Development

### Prerequisites

- Rust 1.70+
- Cargo

### Setup

```bash
git clone https://github.com/iamjuaness/ForgeArch.git
cd ForgeArch
cargo build
```

### Run Tests

```bash
cargo test                    # Run all tests
cargo test --verbose          # Verbose output
cargo clippy                  # Lint code
cargo fmt                     # Format code
```

### Run Locally

```bash
cargo run -- new test-project --arch backend-api
cargo run -- list
cargo run -- add-template custom
```

### Project Structure

```
ForgeArch/
├── src/
│   ├── main.rs           # CLI entry point
│   ├── generator.rs      # Project generation logic
│   └── templates.rs      # Template loading and management
├── templates/
│   ├── architectures.json              # Built-in templates (34+)
│   └── file_templates/
│       ├── backend/
│       ├── frontend/
│       └── monorepo/
├── Cargo.toml
└── README.md
```

---

## 🤝 Contributing

Contributions are welcome! Here's how you can help:

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Adding New Templates

To add a new built-in template:

1. Edit `templates/architectures.json`
2. Add your template structure:
   ```json
   {
     "your-template": {
       "name": "Your Template Name",
       "description": "Template description",
       "structure": ["src", "tests"],
       "files": {".gitignore": "backend"}
     }
   }
   ```
3. Test: `cargo run -- new test --arch your-template`
4. Submit a PR

### Code Style

- Follow Rust conventions
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add tests for new features

---

## 📄 License

This project is licensed under the **MIT License** - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- Inspired by `create-react-app`, `cargo-generate`, and `yeoman`
- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Uses [Tera](https://tera.netlify.app/) for templating
- Uses [Clap](https://github.com/clap-rs/clap) for CLI parsing

---

## 📞 Support

- 🐛 **Report Bugs**: [GitHub Issues](https://github.com/iamjuaness/ForgeArch/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/iamjuaness/ForgeArch/discussions)
- 📧 **Email**: juane.cardonav@gmail.com
- 🌟 **Star** this repo if you find it useful!

---

<div align="center">

**[⬆ Back to Top](#-forgearch)**

Made with ❤️ by [iamjuaness](https://github.com/iamjuaness)

⭐ Star us on GitHub — it helps!

</div>
