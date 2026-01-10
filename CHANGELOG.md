# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Nothing yet

### Changed
- Nothing yet

### Fixed
- Nothing yet

---

## [0.1.0] - 2026-01-09

### Added
- **Core CLI Commands**:
  - `forge new <name>` - Create new project (interactive mode)
  - `forge new <name> --arch <template>` - Create project with specific template
  - `forge list` - List all available architecture templates
  - `forge add-template <key>` - Create custom local template
  - `forge remove-template <key>` - Remove custom local template
- **Command-line Flags**:
  - `--arch <template>` - Select architecture template
  - `--git-init` - Initialize git repository after project creation
  - `--no-readme` - Skip README.md generation
  - `--force` - Overwrite existing project directory
  - `--version` / `-v` - Display version information
- **34+ Built-in Architecture Templates**:
  - Backend: REST API, GraphQL, FastAPI, Django, NestJS, Go, Spring Boot
  - Frontend: React, Next.js, Vue, Angular, Svelte
  - Fullstack: MERN, Monorepo (Nx, Turborepo)
  - Mobile: React Native, Flutter
  - Cloud/DevOps: Kubernetes, Serverless, Terraform, Microservices, API Gateway
  - Advanced: Clean Architecture, DDD, Event-Driven, CQRS, GraphQL Federation
  - Specialized: Machine Learning, Data Science, CLI (Rust), Electron, Chrome Extension, Blockchain DApp
- **Custom Templates System**:
  - Local template storage at `~/.config/forge/templates/local_templates.json`
  - Interactive template creation with auto-opening editor
  - Template validation (prevents path traversal and absolute paths)
  - User templates override built-in templates with same key
- **Editor Integration**:
  - Respects `$FORGE_EDITOR`, `$VISUAL`, and `$EDITOR` environment variables
  - WSL path conversion for VS Code integration
  - Fallback chain: nano → vim → VS Code → notepad (Windows)
- **Template Engine**:
  - Tera-based file template system for dynamic content generation
  - Pre-configured templates for `.gitignore`, `.env.example`, `package.json`
  - Support for nested folder structures (e.g., `k8s/overlays/dev`)
- **Project Generator**:
  - Automatic folder structure creation from template definitions
  - File generation with variable substitution (project name, etc.)
  - Conditional `.gitignore` creation
  - Optional README.md with template structure documentation
  - Optional git repository initialization
- **Interactive Mode**:
  - Dialoguer-based template selection when `--arch` is not provided
  - Colored output with status indicators (✓, →, ●)
- **Cross-platform Support**:
  - Linux x86_64 builds
  - Windows x86_64 builds
  - WSL (Windows Subsystem for Linux) support
  - Automated installers with PATH configuration

### Infrastructure
- **CI/CD Pipeline**:
  - GitHub Actions workflow for continuous integration
  - Automated testing with `cargo test`
  - Code quality checks with `cargo clippy` (zero warnings)
  - Format verification with `cargo fmt`
- **Release Automation**:
  - Automated binary builds for Linux and Windows
  - SHA256 checksum generation
  - GitHub Releases integration with release workflow
- **Installation Scripts**:
  - `install.sh` - Unix/Linux/macOS/WSL installer
  - `install.ps1` - Windows PowerShell installer
  - Automatic PATH configuration in shell configs
  - Binary installation to `~/.local/bin/forge`

### Testing
- Unit tests for template loading and validation
- Unit tests for project generation
- Permission error handling tests (Unix)
- Test coverage for custom template management

### Documentation
- Comprehensive README with quick start guide
- 34+ templates documented with descriptions
- Custom templates guide with JSON examples
- WSL integration instructions
- CI/CD examples (GitHub Actions, GitLab CI, Jenkins)
- Editor configuration guide
- Development setup and contribution guidelines
- `CHANGELOG.md` following Keep a Changelog format
- MIT License

---

## Links

- **Repository**: https://github.com/iamjuaness/ForgeArch
- **Issues**: https://github.com/iamjuaness/ForgeArch/issues
- **Releases**: https://github.com/iamjuaness/ForgeArch/releases

---

Generated on: 2026-01-09
