# AI-Based Coding Rules for Rust and Go Development (AIBD-RG)

*Based on the 7-step Vibe Coding Workflow and industry best practices*

## Overview

This document establishes the **AIBD-RG** (AI Boosted Development - Rust & Go) guidelines for maintainable AI-assisted development. These rules ensure consistent, high-quality code generation while preserving long-term maintainability.

## Core Principles

1. **Feature-Based Architecture**: Organize code by business capabilities, not technical layers
2. **Small, Focused Files**: One declaration per file for better AI token efficiency
3. **Clear Boundaries**: Enforce module separation with explicit dependency rules
4. **Type Safety First**: Leverage Rust’s ownership system and Go’s interfaces
5. **Documentation as Code**: Maintain living documentation alongside implementation

-----

## 1. Project Structure

### Rust Projects

```
src/
├── main.rs                    # Application entry point
├── lib.rs                     # Library root (if applicable)
├── features/                  # Feature-based organization
│   ├── user_management/       # Each feature is self-contained
│   │   ├── prd.md            # Feature requirements
│   │   ├── mod.rs            # Public API surface
│   │   ├── types.rs          # Public types
│   │   ├── service.rs        # Business logic
│   │   └── internal/         # Private implementation
│   │       ├── mod.rs
│   │       ├── validation.rs
│   │       └── persistence.rs
│   └── payment_processing/
│       └── ...
├── shared/                   # Shared utilities and types
│   ├── mod.rs
│   ├── types/
│   │   ├── mod.rs
│   │   ├── error.rs
│   │   └── result.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── logging.rs
│   │   └── config.rs
│   └── internal/            # Private shared code
│       └── ...
├── bin/                     # Binary targets
└── tests/                   # Integration tests
    └── features/
```

### Go Projects

```
.
├── cmd/                     # Application entry points
│   ├── server/
│   │   └── main.go
│   └── cli/
│       └── main.go
├── pkg/                     # Public library code
│   └── api/
├── internal/                # Private application code
│   ├── features/            # Feature-based organization
│   │   ├── user/           # Each feature is self-contained
│   │   │   ├── prd.md      # Feature requirements
│   │   │   ├── user.go     # Public API
│   │   │   ├── types.go    # Public types
│   │   │   ├── service.go  # Business logic
│   │   │   └── internal/   # Private implementation
│   │   │       ├── validation.go
│   │   │       └── repository.go
│   │   └── payment/
│   │       └── ...
│   ├── shared/             # Shared utilities
│   │   ├── types/
│   │   │   ├── error.go
│   │   │   └── result.go
│   │   ├── utils/
│   │   │   ├── logging.go
│   │   │   └── config.go
│   │   └── internal/       # Private shared code
│   └── tests/              # Test utilities
├── api/                    # API definitions (OpenAPI, protobuf)
├── docs/                   # Documentation
└── scripts/                # Build and deployment scripts
```

-----

## 2. Architecture Guidelines

### Feature Organization Rules

1. **Self-Contained Features**: Each feature owns its complete functionality
2. **Clear Boundaries**: Features can depend on `shared/` but not on other features directly
3. **Public API Surface**: Only expose what’s necessary through `mod.rs` (Rust) or package-level exports (Go)
4. **Internal Implementation**: Keep implementation details in `internal/` directories

### Dependency Rules

#### Rust

- Features in `src/features/` can import from `src/shared/` and other features’ public APIs
- Shared modules in `src/shared/` cannot import from `src/features/`
- Internal modules (`*/internal/`) are only accessible within their parent feature
- Use `pub(crate)` for crate-internal APIs, `pub` only for true public APIs

#### Go

- Features in `internal/features/` can import from `internal/shared/` and other features’ public APIs
- Shared modules in `internal/shared/` cannot import from `internal/features/`
- Internal packages (`*/internal/`) follow Go’s internal package visibility rules
- Use clear package naming and avoid circular dependencies

-----

## 3. File Organization

### One Declaration Per File Rule

#### Rust

```rust
// ❌ Don't: Multiple types in one file
// user_types.rs
pub struct User { ... }
pub struct UserProfile { ... }
pub struct UserSettings { ... }

// ✅ Do: One type per file
// user.rs
pub struct User { ... }

// user_profile.rs  
pub struct UserProfile { ... }

// user_settings.rs
pub struct UserSettings { ... }
```

#### Go

```go
// ❌ Don't: Multiple types in one file
// user_types.go
type User struct { ... }
type UserProfile struct { ... }
type UserSettings struct { ... }

// ✅ Do: One primary type per file
// user.go
type User struct { ... }

// profile.go
type UserProfile struct { ... }

// settings.go  
type UserSettings struct { ... }
```

### Import Organization

#### Rust

```rust
// Order: std, external crates, internal modules
use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::shared::types::UserId;
use crate::shared::error::AppError;
```

#### Go

```go
// Order: standard library, external packages, internal packages
import (
    "context"
    "fmt"
    "time"
    
    "github.com/gin-gonic/gin"
    "github.com/google/uuid"
    
    "myapp/internal/shared/types"
    "myapp/internal/shared/errors"
)
```

-----

## 4. Code Style Guidelines

### Rust Specific Rules

#### Async/Await Best Practices

```rust
// ❌ Don't: Async constructors
impl UserService {
    pub async fn new(repo: UserRepository) -> Self {
        let config = load_config().await;
        Self { repo, config }
    }
}

// ✅ Do: Separate construction from initialization
impl UserService {
    pub fn new(repo: UserRepository) -> Self {
        Self { repo, config: ## 8. CI/CD Integration and Tooling

### GitHub Actions Workflows

#### Rust CI Pipeline
```yaml
# .github/workflows/rust.yml
name: Rust CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable
      with:
        components: rustfmt, clippy
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
    
    - name: Check formatting
      run: cargo fmt --all -- --check
    
    - name: Run clippy
      run: cargo clippy --all-targets --all-features -- -D warnings
    
    - name: Run tests
      run: cargo test --all-features --verbose
    
    - name: Check for circular dependencies
      run: |
        cargo install cargo-deps
        cargo deps --no-transitive-deps | grep -q "cycle" && exit 1 || exit 0
    
    - name: Security audit
      run: |
        cargo install cargo-audit
        cargo audit
    
    - name: Generate documentation
      run: cargo doc --no-deps --all-features
    
    - name: Upload coverage reports
      uses: codecov/codecov-action@v3
      with:
        file: ./target/tarpaulin/cobertura.xml
```

#### Go CI Pipeline

```yaml
# .github/workflows/go.yml
name: Go CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Set up Go
      uses: actions/setup-go@v4
      with:
        go-version: '1.21'
    
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cache/go-build
          ~/go/pkg/mod
        key: ${{ runner.os }}-go-${{ hashFiles('**/go.sum') }}
    
    - name: Install dependencies
      run: go mod download
    
    - name: Verify dependencies
      run: go mod verify
    
    - name: Check formatting
      run: |
        if [ "$(gofmt -s -l . | wc -l)" -gt 0 ]; then
          echo "Code not formatted:"
          gofmt -s -l .
          exit 1
        fi
    
    - name: Run go vet
      run: go vet ./...
    
    - name: Install golangci-lint
      run: |
        curl -sSfL https://raw.githubusercontent.com/golangci/golangci-lint/master/install.sh | sh -s -- -b $(go env GOPATH)/bin v1.54.2
    
    - name: Run golangci-lint
      run: golangci-lint run
    
    - name: Run tests with coverage
      run: go test -race -coverprofile=coverage.out -covermode=atomic ./...
    
    - name: Check for circular dependencies
      run: |
        go list -json ./... | jq -r '.ImportPath + " " + (.Imports // [] | join(" "))' | \
          python3 -c "
        import sys
        from collections import defaultdict, deque
        
        graph = defaultdict(list)
        for line in sys.stdin:
            parts = line.strip().split()
            if len(parts) > 1:
                pkg = parts[0]
                for imp in parts[1:]:
                    if imp.startswith('$(go list -m)'):
                        graph[pkg].append(imp)
        
        def has_cycle(graph):
            WHITE, GRAY, BLACK = 0, 1, 2
            color = defaultdict(int)
            
            def dfs(node):
                color[node] = GRAY
                for neighbor in graph[node]:
                    if color[neighbor] == GRAY:
                        return True
                    if color[neighbor] == WHITE and dfs(neighbor):
                        return True
                color[node] = BLACK
                return False
            
            for node in graph:
                if color[node] == WHITE:
                    if dfs(node):
                        return True
            return False
        
        if has_cycle(graph):
            print('Circular dependency detected')
            sys.exit(1)
        "
    
    - name: Run gosec security scanner
      run: |
        go install github.com/securecodewarrior/gosec/v2/cmd/gosec@latest
        gosec ./...
    
    - name: Upload coverage to Codecov
      uses: codecov/codecov-action@v3
      with:
        file: ./coverage.out
```

### Semantic Versioning and Changelog Automation

#### Using git-cliff for Rust

```toml
# cliff.toml
[changelog]
header = """
# Changelog\n
All notable changes to this project will be documented in this file.\n
"""
body = """
{% if version %}\
    ## [{{ version | trim_start_matches(pat="v") }}] - {{ timestamp | date(format="%Y-%m-%d") }}
{% else %}\
    ## [unreleased]
{% endif %}\
{% for group, commits in commits | group_by(attribute="group") %}
    ### {{ group | upper_first }}
    {% for commit in commits %}
        - {{ commit.message | upper_first }}\
    {% endfor %}
{% endfor %}\n
"""

[git]
conventional_commits = true
filter_unconventional = true
split_commits = false
commit_preprocessors = [
    { pattern = '\((\w+)(?:\/)?(.*)\)', replace = "($1) $2"},
]
commit_parsers = [
    { message = "^feat", group = "Features"},
    { message = "^fix", group = "Bug Fixes"},
    { message = "^doc", group = "Documentation"},
    { message = "^perf", group = "Performance"},
    { message = "^refactor", group = "Refactoring"},
    { message = "^style", group = "Styling"},
    { message = "^test", group = "Testing"},
    { message = "^chore\(release\): prepare for", skip = true},
    { message = "^chore", group = "Miscellaneous Tasks"},
    { body = ".*security", group = "Security"},
]
```

#### GitHub Action for Release Automation

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0
    
    - name: Generate changelog
      run: |
        curl -L https://github.com/orhun/git-cliff/releases/latest/download/git-cliff-$(uname -s)-$(uname -m).tar.gz | tar xz
        ./git-cliff --current > CHANGELOG.md
    
    - name: Create Release
      uses: actions/create-release@v1
      env:
        GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
      with:
        tag_name: ${{ github.ref }}
        release_name: Release ${{ github.ref }}
        body_path: CHANGELOG.md
```

### Dependency Management and Security

#### Rust Security Tools

```bash
# Install security audit tools
cargo install cargo-audit cargo-outdated cargo-deny

# Regular security checks
cargo audit                    # Check for known vulnerabilities
cargo outdated                 # Check for outdated dependencies
cargo deny check              # Policy-based dependency management
```

#### cargo-deny Configuration

```toml
# deny.toml
[bans]
multiple-versions = "warn"
wildcards = "deny"
highlight = "all"

[[bans.deny]]
name = "openssl"
version = "*"
use-instead = "rustls"

[licenses]
unlicensed = "deny"
allow = [
    "MIT",
    "Apache-2.0",
    "Apache-2.0 WITH LLVM-exception",
    "BSD-2-Clause",
    "BSD-3-Clause",
    "ISC",
    "Unicode-DFS-2016",
]

[advisories]
vulnerability = "deny"
unmaintained = "warn"
yanked = "warn"
notice = "warn"
```

#### Go Security Tools

```bash
# Install security tools
go install github.com/securecodewarrior/gosec/v2/cmd/gosec@latest
go install golang.org/x/vuln/cmd/govulncheck@latest

# Regular security checks
gosec ./...           # Static security analyzer
govulncheck ./...     # Check for known vulnerabilities
go list -m all | nancy sleuth  # Check dependencies
```

-----

## 9. Security and Compliance

### Secure Coding Practices

#### Input Validation

**Rust Example:**

```rust
use validator::{Validate, ValidationError};

#[derive(Debug, Validate, Deserialize)]
pub struct CreateUserRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    
    #[validate(email)]
    pub email: String,
    
    #[validate(length(min = 8, max = 128))]
    #[validate(custom = "validate_password_strength")]
    pub password: String,
    
    #[validate(range(min = 18, max = 120))]
    pub age: Option<u8>,
}

fn validate_password_strength(password: &str) -> Result<(), ValidationError> {
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| "!@#$%^&*()".contains(c));
    
    if has_upper && has_lower && has_digit && has_special {
        Ok(())
    } else {
        Err(ValidationError::new("password_too_weak"))
    }
}

// Safe deserialization with size limits
pub fn deserialize_request<T>(body: &[u8]) -> Result<T, ValidationError> 
where 
    T: for<'a> Deserialize<'a> + Validate,
{
    const MAX_SIZE: usize = 1024 * 1024; // 1MB limit
    
    if body.len() > MAX_SIZE {
        return Err(ValidationError::new("request_too_large"));
    }
    
    let data: T = serde_json::from_slice(body)
        .map_err(|_| ValidationError::new("invalid_json"))?;
    
    data.validate()?;
    Ok(data)
}
```

**Go Example:**

```go
import (
    "context"
    "fmt"
    "net/mail"
    "regexp"
    "unicode"
)

type CreateUserRequest struct {
    Name     string `json:"name" validate:"required,min=1,max=100"`
    Email    string `json:"email" validate:"required,email"`
    Password string `json:"password" validate:"required,min=8,max=128"`
    Age      *int   `json:"age,omitempty" validate:"omitempty,min=18,max=120"`
}

func (r *CreateUserRequest) Validate() error {
    if len(r.Name) == 0 || len(r.Name) > 100 {
        return fmt.Errorf("name must be 1-100 characters")
    }
    
    if _, err := mail.ParseAddress(r.Email); err != nil {
        return fmt.Errorf("invalid email format")
    }
    
    if err := validatePasswordStrength(r.Password); err != nil {
        return err
    }
    
    if r.Age != nil && (*r.Age < 18 || *r.Age > 120) {
        return fmt.Errorf("age must be between 18 and 120")
    }
    
    return nil
}

func validatePasswordStrength(password string) error {
    if len(password) < 8 {
        return fmt.Errorf("password too short")
    }
    
    var hasUpper, hasLower, hasDigit, hasSpecial bool
    for _, char := range password {
        switch {
        case unicode.IsUpper(char):
            hasUpper = true
        case unicode.IsLower(char):
            hasLower = true
        case unicode.IsDigit(char):
            hasDigit = true
        case unicode.IsPunct(char) || unicode.IsSymbol(char):
            hasSpecial = true
        }
    }
    
    if !hasUpper || !hasLower || !hasDigit || !hasSpecial {
        return fmt.Errorf("password must contain uppercase, lowercase, digit, and special character")
    }
    
    return nil
}

// Safe JSON parsing with size limits
func ParseJSONRequest(ctx context.Context, body []byte, dest interface{}) error {
    const maxSize = 1024 * 1024 // 1MB limit
    
    if len(body) > maxSize {
        return fmt.Errorf("request body too large")
    }
    
    return json.Unmarshal(body, dest)
}
```

#### Avoiding Unsafe Patterns

**Rust: Forbid Unsafe Code**

```rust
// In Cargo.toml
[lints.rust]
unsafe_code = "forbid"

// Or with clippy
[lints.clippy]
undocumented_unsafe_blocks = "deny"
```

**Go: Secure HTTP Handling**

```go
import (
    "context"
    "crypto/tls"
    "net/http"
    "time"
)

// ✅ Secure HTTP server configuration
func NewSecureServer(handler http.Handler) *http.Server {
    return &http.Server{
        Addr:         ":8080",
        Handler:      handler,
        ReadTimeout:  15 * time.Second,
        WriteTimeout: 15 * time.Second,
        IdleTimeout:  60 * time.Second,
        TLSConfig: &tls.Config{
            MinVersion: tls.VersionTLS12,
            CipherSuites: []uint16{
                tls.TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
                tls.TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
                tls.TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305,
                tls.TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305,
            },
        },
    }
}

// ✅ Request timeout middleware
func WithTimeout(timeout time.Duration) func(http.Handler) http.Handler {
    return func(next http.Handler) http.Handler {
        return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
            ctx, cancel := context.WithTimeout(r.Context(), timeout)
            defer cancel()
            next.ServeHTTP(w, r.WithContext(ctx))
        })
    }
}
```

### Static Analysis Integration

#### Rust Security Analysis

```yaml
# .github/workflows/security.yml
name: Security Audit

on:
  schedule:
    - cron: '0 0 * * 0'  # Weekly
  workflow_dispatch:

jobs:
  security-audit:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Install Rust
      uses: dtolnay/rust-toolchain@stable
    
    - name: Install cargo-audit
      run: cargo install cargo-audit
    
    - name: Security Audit
      run: cargo audit --json | tee audit-results.json
    
    - name: Install cargo-deny
      run: cargo install cargo-deny
    
    - name: License and Security Policy Check
      run: cargo deny check
    
    - name: Upload security results
      uses: github/codeql-action/upload-sarif@v2
      if: always()
      with:
        sarif_file: audit-results.json
```

-----

## 10. Enhanced Anti-Patterns

### AI-Specific Anti-Patterns

#### Over-reliance Without Review

```rust
// ❌ Don't: Blindly accepting all AI-generated code
// AI generated this entire service without review:
pub struct UserService {
    // AI might hallucinate dependencies that don't exist
    repo: SomeNonExistentRepository,
    // Or use patterns inconsistent with your codebase
    logger: Box<dyn SomeGenericLogger>,
}

// ✅ Do: Review and validate AI suggestions
pub struct UserService {
    // Verify dependencies exist and follow project patterns
    repo: Arc<dyn UserRepository>,
    // Use established patterns from your codebase
    logger: Arc<dyn Logger>,
}
```

#### Prompting Without Context

```
❌ Bad Prompt:
"Add error handling to this function"

✅ Good Prompt:
"Add error handling to this function following our AIBD-RG patterns:
- Use our established AppError types
- Follow the Result<T, E> pattern
- Add proper error context with thiserror
- Ensure errors map to our domain model

Here's how we handle errors in similar functions: [provide examples]"
```

#### Ignoring AI Hallucinations

```rust
// ❌ AI might suggest non-existent methods
impl User {
    pub fn validate(&self) -> ValidationResult {
        // AI invents a method that doesn't exist in your ValidationResult
        self.email.check_domain_reputation()
    }
}

// ✅ Always verify AI suggestions compile and make sense
impl User {
    pub fn validate(&self) -> Result<(), ValidationError> {
        // Use actual methods from your codebase
        Email::validate(&self.email)?;
        Ok(())
    }
}
```

### Architectural Anti-Patterns

#### Feature Boundary Violations

```rust
// ❌ Don't: Direct imports between feature internals
// In src/features/payment/internal/processor.rs
use crate::features::user_management::internal::repository::UserRepo;

// ✅ Do: Use public APIs and dependency injection
// In src/features/payment/service.rs
use crate::features::user_management::UserService;

pub struct PaymentService {
    user_service: Arc<dyn UserServiceTrait>,
}
```

#### Circular Dependencies

```go
// ❌ Don't: Create circular dependencies
// internal/features/user/service.go
import "myapp/internal/features/payment"

// internal/features/payment/service.go  
import "myapp/internal/features/user"

// ✅ Do: Use dependency inversion
// internal/features/user/service.go
type PaymentService interface {
    ProcessPayment(ctx context.Context, userID UserID, amount Money) error
}

type UserService struct {
    paymentSvc PaymentService // Interface, not concrete type
}
```

### Code Quality Anti-Patterns

#### Inconsistent Error Handling

```rust
// ❌ Don't: Mix error handling styles
pub fn create_user(data: UserData) -> Option<User> { // Sometimes Option
    // ...
}

pub fn update_user(data: UserData) -> Result<User, String> { // Sometimes Result<T, String>
    // ...
}

pub fn delete_user(id: UserId) -> Result<(), Box<dyn Error>> { // Sometimes Box<dyn Error>
    // ...
}

// ✅ Do: Use consistent error types
pub fn create_user(data: UserData) -> UserResult<User> {
    // ...
}

pub fn update_user(data: UserData) -> UserResult<User> {
    // ...
}

pub fn delete_user(id: UserId) -> UserResult<()> {
    // ...
}
```

#### Mixing Abstraction Levels

```go
// ❌ Don't: Mix high-level business logic with low-level details
func (s *UserService) CreateUser(ctx context.Context, data CreateUserData) (*User, error) {
    // High-level business logic
    if err := s.validateBusinessRules(data); err != nil {
        return nil, err
    }
    
    // Low-level database details mixed in
    query := "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id"
    var id int64
    err := s.db.QueryRowContext(ctx, query, data.Name, data.Email, hashedPassword).Scan(&id)
    
    // More business logic
    s.sendWelcomeEmail(user.Email)
    
    return user, nil
}

// ✅ Do: Separate concerns with clear boundaries
func (s *UserService) CreateUser(ctx context.Context, data CreateUserData) (*User, error) {
    // Business logic layer
    if err := s.validateBusinessRules(data); err != nil {
        return nil, err
    }
    
    user, err := s.buildUser(data)
    if err != nil {
        return nil, err
    }
    
    // Delegate to repository layer
    createdUser, err := s.repo.Create(ctx, user)
    if err != nil {
        return nil, err
    }
    
    // Business logic continues
    s.emailService.SendWelcomeEmail(ctx, createdUser.Email)
    
    return createdUser, nil
}
```

### Testing Anti-Patterns

#### Testing Implementation Instead of Behavior

```rust
// ❌ Don't: Test internal implementation details
#[test]
fn test_user_service_calls_repository_create() {
    let mut mock_repo = MockUserRepository::new();
    mock_repo.expect_create()
        .times(1)
        .returning(|_| Ok(()));
    
    let service = UserService::new(Arc::new(mock_repo));
    service.create_user(test_data()).unwrap();
    
    // This test breaks when you refactor internal implementation
}

// ✅ Do: Test business behavior and outcomes
#[test]
fn test_create_user_with_valid_data_succeeds() {
    let service = create_test_user_service();
    let user_data = CreateUserData {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        password: "secure123!".to_string(),
    };
    
    let result = service.create_user(user_data).await;
    
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email.to_string(), "john@example.com");
    // Verify user can be retrieved (behavior)
    let retrieved = service.get_user_by_email(&user.email).await.unwrap();
    assert_eq!(retrieved.id, user.id);
}
```

-----

## Conclusion

These enhanced AIBD-RG guidelines provide a comprehensive framework for maintainable AI-assisted development in Rust and Go. The key improvements include:

- **Enhanced AI Interaction**: Better prompting strategies, session management, and hallucination detection
- **Language-Specific Deep Dives**: Async patterns, generics, context propagation, and type safety
- **Comprehensive Tooling**: CI/CD integration, security scanning, and dependency management
- **Security-First Approach**: Input validation, secure defaults, and static analysis integration
- **Expanded Anti-Patterns**: AI-specific pitfalls, architectural violations, and testing mistakes

Remember that successful AI-assisted development isn’t just about speed—it’s about maintaining quality, security, and long-term maintainability while leveraging AI to amplify your capabilities as a developer.

The framework evolves with your team’s needs, so regularly review and update these guidelines based on your experience and changing requirements. }
}

```
pub async fn initialize(&mut self) -> AppResult<()> {
    self.config = Some(load_config().await?);
    Ok(())
}
```

}

// ✅ Use tokio::spawn safely with proper error handling
pub async fn process_users_batch(&self, users: Vec<UserId>) -> AppResult<Vec<ProcessResult>> {
let handles: Vec<_> = users
.into_iter()
.map(|user_id| {
let service = Arc::clone(&self.service);
tokio::spawn(async move {
service.process_user(user_id).await
})
})
.collect();

```
let mut results = Vec::new();
for handle in handles {
    match handle.await {
        Ok(result) => results.push(result?),
        Err(join_err) => return Err(AppError::TaskJoinError(join_err)),
    }
}
Ok(results)
```

}

```
#### Newtype Wrappers for Domain Safety
```rust
// ✅ Use newtypes extensively for domain modeling
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(Uuid);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HashedPassword(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(DateTime<Utc>);

// Implement domain-specific methods
impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
    
    pub fn from_str(s: &str) -> Result<Self, ValidationError> {
        Uuid::parse_str(s)
            .map(Self)
            .map_err(|_| ValidationError::InvalidUserId)
    }
}

impl Email {
    pub fn new(value: String) -> Result<Self, ValidationError> {
        if Self::is_valid(&value) {
            Ok(Self(value))
        } else {
            Err(ValidationError::InvalidEmail)
        }
    }
    
    fn is_valid(email: &str) -> bool {
        // Email validation logic
        email.contains('@') && email.len() > 3
    }
    
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
}

// ❌ Don't: Using primitive types in business logic
pub fn send_welcome_email(user_id: String, email: String) -> Result<(), EmailError> {
    // Easy to mix up parameters
}

// ✅ Do: Type-safe parameters
pub fn send_welcome_email(user_id: UserId, email: Email) -> Result<(), EmailError> {
    // Impossible to mix up parameters
}
```

#### Result vs Option Usage Patterns

```rust
// ✅ Use Result<T, E> when you need failure context
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::PasswordTooShort);
    }
    if !password.chars().any(|c| c.is_ascii_punctuation()) {
        return Err(ValidationError::PasswordNeedsSpecialChar);
    }
    Ok(())
}

// ✅ Use Option<T> when absence is a valid state
pub fn find_user_by_email(&self, email: &Email) -> Option<User> {
    self.users.get(email).cloned()
}

// ✅ Convert between Result and Option when needed
pub fn get_user_by_id(&self, id: UserId) -> Result<User, UserError> {
    self.find_user_by_id(id)
        .ok_or(UserError::NotFound { id })
}

// ✅ Use Result for fallible operations even if error is rare
pub fn parse_timestamp(input: &str) -> Result<Timestamp, ParseError> {
    DateTime::parse_from_rfc3339(input)
        .map(|dt| Timestamp(dt.with_timezone(&Utc)))
        .map_err(ParseError::InvalidTimestamp)
}
```

### Go Specific Rules

#### Context Propagation Best Practices

```go
// ✅ Always pass context.Context as the first parameter
func (s *UserService) CreateUser(ctx context.Context, data CreateUserData) (*User, error) {
    // Use context for timeouts, cancellation, and request-scoped values
    ctx, cancel := context.WithTimeout(ctx, 30*time.Second)
    defer cancel()
    
    return s.repo.Create(ctx, data)
}

// ✅ Propagate context through the entire call chain
func (s *UserService) ProcessBatch(ctx context.Context, users []UserID) error {
    for _, userID := range users {
        select {
        case <-ctx.Done():
            return ctx.Err() // Handle cancellation
        default:
            if err := s.processUser(ctx, userID); err != nil {
                return err
            }
        }
    }
    return nil
}

// ✅ Use context for request-scoped values sparingly
func (s *UserService) GetCurrentUser(ctx context.Context) (*User, error) {
    userID, ok := ctx.Value("user_id").(UserID)
    if !ok {
        return nil, ErrNoUserInContext
    }
    return s.repo.GetByID(ctx, userID)
}
```

#### Go Generics (1.18+) for Reusable Logic

```go
// ✅ Use generics for type-safe collections and utilities
type Repository[T any, ID comparable] interface {
    GetByID(ctx context.Context, id ID) (*T, error)
    Create(ctx context.Context, entity *T) error
    Update(ctx context.Context, entity *T) error
    Delete(ctx context.Context, id ID) error
}

// ✅ Generic pagination utility
type PageRequest[T any] struct {
    Limit  int
    Offset int
    Filter T
}

type PageResponse[T any] struct {
    Items      []T
    Total      int
    HasMore    bool
    NextOffset *int
}

func Paginate[T any](ctx context.Context, req PageRequest[T], 
    fetcher func(ctx context.Context, limit, offset int, filter T) ([]T, int, error),
) (*PageResponse[T], error) {
    items, total, err := fetcher(ctx, req.Limit, req.Offset, req.Filter)
    if err != nil {
        return nil, err
    }
    
    hasMore := req.Offset+len(items) < total
    var nextOffset *int
    if hasMore {
        next := req.Offset + req.Limit
        nextOffset = &next
    }
    
    return &PageResponse[T]{
        Items:      items,
        Total:      total,
        HasMore:    hasMore,
        NextOffset: nextOffset,
    }, nil
}

// ✅ Type-safe result wrapper
type Result[T any] struct {
    Value T
    Error error
}

func NewResult[T any](value T, err error) Result[T] {
    return Result[T]{Value: value, Error: err}
}

func (r Result[T]) IsOk() bool {
    return r.Error == nil
}

func (r Result[T]) Unwrap() T {
    if r.Error != nil {
        panic("called Unwrap on error result")
    }
    return r.Value
}
```

#### Interface Versioning and Dependency Injection

```go
// ✅ Version interfaces explicitly
type UserRepositoryV1 interface {
    GetByID(ctx context.Context, id UserID) (*User, error)
    Create(ctx context.Context, user *User) error
}

type UserRepositoryV2 interface {
    UserRepositoryV1 // Embed previous version
    GetByEmail(ctx context.Context, email string) (*User, error)
    BatchCreate(ctx context.Context, users []*User) error
}

// ✅ Use dependency injection with interfaces
type UserService struct {
    repo   UserRepositoryV2
    email  EmailService
    logger Logger
    cache  Cache[UserID, *User]
}

// ✅ Constructor pattern with options
type UserServiceOption func(*UserService)

func WithCache[T any](cache Cache[UserID, T]) UserServiceOption {
    return func(s *UserService) {
        s.cache = cache
    }
}

func WithLogger(logger Logger) UserServiceOption {
    return func(s *UserService) {
        s.logger = logger
    }
}

func NewUserService(repo UserRepositoryV2, email EmailService, opts ...UserServiceOption) *UserService {
    s := &UserService{
        repo:   repo,
        email:  email,
        logger: &NoOpLogger{}, // Default
    }
    
    for _, opt := range opts {
        opt(s)
    }
    
    return s
}

// ✅ Interface segregation - keep interfaces focused
type UserReader interface {
    GetByID(ctx context.Context, id UserID) (*User, error)
    GetByEmail(ctx context.Context, email string) (*User, error)
}

type UserWriter interface {
    Create(ctx context.Context, user *User) error
    Update(ctx context.Context, user *User) error
    Delete(ctx context.Context, id UserID) error
}

// Compose for full repository
type UserRepository interface {
    UserReader
    UserWriter
}
```

-----

## 5. Documentation Standards

### Feature-Level Documentation (PRD Template)

Each feature must include a `prd.md` file using this template:

```markdown
# Feature Name: [Feature Name]

## Objective
[1-2 sentences describing the business purpose and value of this feature]

## Business Context
[Background information about why this feature is needed, what problem it solves, and how it fits into the broader application]

## User Stories
[List the primary user stories in the format: "As a [role], I want [goal] so that [reason]"]
- As a [role], I want [goal] so that [reason]
- As a [role], I want [goal] so that [reason]
- As a [role], I want [goal] so that [reason]

## Requirements

### Functional Requirements
[What the system must do - specific behaviors and capabilities]
- [Requirement 1]
- [Requirement 2]
- [Requirement 3]

### Non-Functional Requirements
[How the system should perform - quality attributes]
- Performance: [response times, throughput requirements]
- Security: [authentication, authorization, data protection]
- Reliability: [uptime, error handling, recovery]
- Scalability: [concurrent users, data volume limits]

## Acceptance Criteria
[Specific, testable conditions that define when the feature is complete]
- [ ] [Testable criterion 1]
- [ ] [Testable criterion 2]
- [ ] [Testable criterion 3]
- [ ] [Error handling criterion]
- [ ] [Edge case criterion]

## Out of Scope
[Explicitly list what is NOT included to prevent scope creep]
- [Excluded functionality 1]
- [Excluded functionality 2]
- [Future enhancement that's not part of this iteration]

## Dependencies
[List dependencies on other features, external services, or infrastructure]
- Internal: [other features this depends on]
- External: [third-party services, APIs, databases]
- Infrastructure: [deployment requirements, environment needs]

## API Design (if applicable)
[High-level API structure if this feature exposes endpoints]
```

POST /api/v1/[resource]
GET /api/v1/[resource]/{id}
PUT /api/v1/[resource]/{id}
DELETE /api/v1/[resource]/{id}

```
## Data Model (if applicable)
[Key data structures and relationships]
```

[Entity] {
field1: type
field2: type
relationships: []
}

```
## Security Considerations
[Security implications and requirements]
- Authentication: [how users are authenticated]
- Authorization: [permission model]
- Data Protection: [sensitive data handling]
- Input Validation: [validation requirements]

## Testing Strategy
[Approach for testing this feature]
- Unit Tests: [what will be unit tested]
- Integration Tests: [integration scenarios]
- End-to-End Tests: [user journey tests]
- Performance Tests: [if applicable]

## Rollout Plan
[How the feature will be deployed and enabled]
- [ ] Development environment
- [ ] Staging environment  
- [ ] Production deployment
- [ ] Feature flags (if applicable)
- [ ] Monitoring and alerts

## Success Metrics
[How success will be measured]
- [Metric 1]: [target value]
- [Metric 2]: [target value]
- [User feedback criteria]
```

### Code-Level Documentation Standards

#### Documentation Generation

Both Rust and Go have excellent built-in documentation tools:

```bash
# Rust: Generate and serve documentation
cargo doc --open --no-deps

# Go: Generate and serve documentation  
go doc -http=:6060
# Or for module documentation
go doc ./...
```

#### Public API Documentation Requirements

All public APIs must include:

1. **Purpose**: What the function/type does
2. **Parameters**: Description of each parameter
3. **Returns**: What is returned and when
4. **Errors**: Possible error conditions
5. **Examples**: Realistic usage examples
6. **Safety**: Any safety considerations (especially for Rust)

### Code-Level Documentation

#### Rust Documentation

```rust
/// Service for managing user accounts and authentication.
/// 
/// This service handles user registration, authentication, profile updates,
/// and account management operations. It coordinates with the user repository
/// and email service to provide complete user lifecycle management.
/// 
/// # Examples
/// 
/// ```rust
/// let service = UserService::new(repo, email_service);
/// let user = service.create_user(create_data).await?;
/// ```
pub struct UserService {
    repository: Arc<dyn UserRepository>,
    email_service: Arc<dyn EmailService>,
}

impl UserService {
    /// Creates a new user account with the provided data.
    /// 
    /// # Arguments
    /// 
    /// * `data` - User creation data including name, email, and password
    /// 
    /// # Returns
    /// 
    /// Returns the created user on success, or an error if:
    /// - Email is already registered
    /// - Password doesn't meet requirements
    /// - Email validation fails
    /// 
    /// # Errors
    /// 
    /// * `UserError::EmailAlreadyExists` - Email is already registered
    /// * `UserError::InvalidPassword` - Password doesn't meet requirements
    /// * `UserError::ValidationFailed` - Input validation failed
    pub async fn create_user(&self, data: CreateUserData) -> UserResult<User> {
        // Implementation
    }
}
```

#### Go Documentation

```go
// UserService provides user account management and authentication services.
//
// This service handles user registration, authentication, profile updates,
// and account management operations. It coordinates with the user repository
// and email service to provide complete user lifecycle management.
type UserService struct {
    repo         UserRepository
    emailService EmailService
    logger       Logger
}

// NewUserService creates a new UserService with the provided dependencies.
func NewUserService(repo UserRepository, emailService EmailService, logger Logger) *UserService {
    return &UserService{
        repo:         repo,
        emailService: emailService,
        logger:       logger,
    }
}

// CreateUser creates a new user account with the provided data.
//
// The function validates the input data, checks for email uniqueness,
// hashes the password, and persists the user to the repository.
// An activation email is sent to the user's email address.
//
// Returns the created user on success, or an error if:
//   - Email is already registered
//   - Password doesn't meet requirements  
//   - Email validation fails
//
// Example:
//   user, err := service.CreateUser(ctx, CreateUserData{
//       Name:     "John Doe",
//       Email:    "john@example.com", 
//       Password: "secure123!",
//   })
func (s *UserService) CreateUser(ctx context.Context, data CreateUserData) (*User, error) {
    // Implementation
}
```

-----

## 6. Testing Guidelines

### Test Organization

#### Rust

```rust
// tests/features/user_management/user_service_test.rs
use crate::features::user_management::{UserService, CreateUserData};
use crate::shared::test::*;

#[tokio::test]
async fn test_create_user_success() {
    // Arrange
    let service = create_test_user_service().await;
    let data = CreateUserData {
        name: "John Doe".to_string(),
        email: "john@example.com".to_string(),
        password: "secure123!".to_string(),
    };
    
    // Act
    let result = service.create_user(data).await;
    
    // Assert
    assert!(result.is_ok());
    let user = result.unwrap();
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.email.to_string(), "john@example.com");
}

#[tokio::test]
async fn test_create_user_duplicate_email_fails() {
    // Test implementation mapping to acceptance criteria
}
```

#### Go

```go
// internal/features/user/service_test.go
func TestUserService_CreateUser_Success(t *testing.T) {
    // Arrange
    service := createTestUserService(t)
    data := CreateUserData{
        Name:     "John Doe",
        Email:    "john@example.com", 
        Password: "secure123!",
    }
    
    // Act
    user, err := service.CreateUser(context.Background(), data)
    
    // Assert
    require.NoError(t, err)
    assert.Equal(t, "John Doe", user.Name)
    assert.Equal(t, "john@example.com", string(user.Email))
}

func TestUserService_CreateUser_DuplicateEmail_Fails(t *testing.T) {
    // Test implementation mapping to acceptance criteria
}
```

### Test Utilities

#### Rust

```rust
// src/shared/test.rs
pub struct TestUserService {
    pub service: UserService,
    pub repo: Arc<MockUserRepository>,
}

pub async fn create_test_user_service() -> TestUserService {
    let repo = Arc::new(MockUserRepository::new());
    let email_service = Arc::new(MockEmailService::new());
    let service = UserService::new(repo.clone(), email_service);
    
    TestUserService { service, repo }
}

pub fn create_test_user() -> User {
    User {
        id: UserId::new(),
        name: "Test User".to_string(),
        email: Email::new("test@example.com".to_string()).unwrap(),
        created_at: chrono::Utc::now(),
    }
}
```

#### Go

```go
// internal/shared/test/helpers.go
func CreateTestUserService(t *testing.T) *UserService {
    repo := &MockUserRepository{}
    emailService := &MockEmailService{}
    logger := &MockLogger{}
    
    return NewUserService(repo, emailService, logger)
}

func CreateTestUser() *User {
    return &User{
        ID:        NewUserID(),
        Name:      "Test User",
        Email:     "test@example.com",
        CreatedAt: time.Now(),
    }
}
```

-----

## 7. AI Assistant Interaction Guidelines

### Effective Prompting

When working with AI assistants, use these patterns:

#### ❌ Bad Prompts vs ✅ Good Prompts

**❌ Bad: Vague and context-free**

```
Write a user service in Rust.
```

**✅ Good: Specific with context and guidelines**

```
Create a user service for our Rust application following AIBD-RG guidelines:

Context: This is part of our user management feature that handles registration and authentication.
Requirements: 
- Use our established error types (UserError)
- Follow the Repository pattern
- Include proper async/await patterns
- Add comprehensive documentation

Please create:
1. The service struct and implementation
2. Appropriate error types
3. Unit tests for core functionality

Refer to existing code in src/features/user_management/ for patterns.
```

**❌ Bad: No architectural context**

```
Fix this error in my Go code.
```

**✅ Good: Clear context and expectations**

```
I'm getting a circular dependency error in our Go application. Here's the error:

[paste error]

This is in our user management feature. Please:
1. Analyze the dependency chain
2. Suggest a refactoring that follows our AIBD-RG guidelines
3. Ensure we maintain proper feature boundaries
4. Keep the Repository interface pattern intact

The affected files are in internal/features/user/.
```

#### Feature Creation Template

```
I'd like to create a new feature for our [Rust/Go] application following AIBD-RG guidelines:

Feature: [Feature Name]
Business Context: [Why this feature exists]
Key Capabilities:
- [Capability 1]
- [Capability 2] 
- [Capability 3]

Dependencies:
- Existing features: [list any]
- External services: [list any]
- Shared utilities needed: [list any]

Please follow our AIBD-RG guidelines and:
1. First create the directory structure
2. Draft the PRD file with acceptance criteria
3. Then implement the public API surface
4. Finally implement the internal logic

Start with just the structure and PRD - don't implement yet.
```

### AI Session Management

#### Session Logging

Maintain a development log for traceability:

```markdown
# AI Development Session Log

## Session: 2024-12-12 - Payment Feature Implementation

### Context Shared:
- AIBD-RG guidelines
- Existing user management patterns
- Database schema for payments

### Prompts Used:
1. "Create payment feature structure and PRD"
2. "Implement PaymentService following established patterns"
3. "Add tests for payment validation logic"
4. "Review code for AIBD-RG compliance"

### Generated Artifacts:
- internal/features/payment/prd.md
- internal/features/payment/service.go
- internal/features/payment/types.go
- internal/features/payment/service_test.go

### Issues Found:
- Missing context propagation in service methods
- Error types not following established patterns

### Fixes Applied:
- Added context.Context as first parameter
- Updated error types to match UserError pattern
```

#### Context Management

```
At the start of each session, provide:

"Here's the context for our development session:

1. Project: [Name and brief description]
2. Current feature: [What we're working on]
3. Recent changes: [What was done in last session]
4. Today's goals: [What we want to accomplish]
5. Guidelines: Follow our AIBD-RG guidelines
6. Existing patterns: [Point to similar implemented features]

Please acknowledge you understand the context before we proceed."
```

### Continuous Reinforcement

Throughout development sessions:

- **Start of session**: “Remember to follow our AIBD-RG guidelines throughout this session”
- **Before major changes**: “Ensure this follows our [specific rule, e.g., ‘one declaration per file rule’]”
- **After generation**: “Review this code for AIBD-RG compliance and suggest improvements”
- **End of session**: “Summarize what we built and confirm it follows our architectural patterns”

### AI Hallucination Detection

Watch for these common AI mistakes:

- **Inconsistent APIs**: AI inventing methods that don’t exist
- **Wrong import paths**: AI assuming dependencies that aren’t there
- **Pattern mixing**: AI switching between different architectural styles mid-session
- **Outdated syntax**: AI using deprecated language features

**Validation prompts:**

```
"Before we proceed, please verify:
1. Are all the imports and dependencies actually available?
2. Does this code compile with the current project setup?
3. Are we following the same patterns as [reference existing code]?
4. Have you introduced any new concepts not in our guidelines?"
```

-----

## 8. Enforcement Tools

### Rust Tooling

#### Cargo.toml Configuration

```toml
[workspace]
members = ["src/*"]

[dependencies]
# Core dependencies
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }

[dev-dependencies]  
tokio-test = "0.4"
mockall = "0.11"

[lints.rust]
unsafe_code = "forbid"
unused_imports = "warn"
dead_code = "warn"

[lints.clippy]
all = "warn"
pedantic = "warn"
nursery = "warn"
```

#### Pre-commit Hooks

```bash
#!/bin/sh
# .git/hooks/pre-commit

# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features -- -D warnings

# Run tests
cargo test

# Check for circular dependencies
cargo-deps --no-transitive-deps | grep -q "cycle" && exit 1

exit 0
```

### Go Tooling

#### Go Module Configuration

```go
// go.mod
module myapp

go 1.21

require (
    github.com/gin-gonic/gin v1.9.1
    github.com/google/uuid v1.3.0
    github.com/stretchr/testify v1.8.4
)
```

#### Makefile for Enforcement

```makefile
.PHONY: lint test fmt vet check-cycles

fmt:
	go fmt ./...

vet:
	go vet ./...

lint:
	golangci-lint run

test:
	go test -race -coverprofile=coverage.out ./...

check-cycles:
	go list -json ./... | jq -r '.ImportPath + " " + (.Imports // [] | join(" "))' | \
		python3 scripts/check_cycles.py

check: fmt vet lint test check-cycles

pre-commit: check
```

#### golangci-lint Configuration

```yaml
# .golangci.yml
linters-settings:
  gocyclo:
    min-complexity: 10
  govet:
    check-shadowing: true
  misspell:
    locale: US

linters:
  enable:
    - gocyclo
    - govet
    - ineffassign
    - misspell
    - gofmt
    - goimports
    - deadcode
    - varcheck
    - structcheck
    - unused

run:
  timeout: 5m
  skip-dirs:
    - vendor
```

-----

## 9. 7-Step AIBD-RG Workflow

1. **Start with Boilerplate**: Use pre-configured project template with proper structure and tooling
2. **Share Guidelines**: Begin each AI session with: “Follow our AIBD-RG guidelines”
3. **Define Features Architecturally**: Request directory structure and PRD files before implementation
4. **Generate Implementation**: Let AI create code following established patterns
5. **Implement Tests**: Map test cases to acceptance criteria in PRD files
6. **Validate with Tools**: Run linters, formatters, and tests; fix issues with AI assistance
7. **Iterate**: Repeat for each feature, building on previous work

### Complete Workflow Example: Adding Payment Processing Feature

Here’s a complete walkthrough of all 7 steps with actual commands and AI interactions:

#### Step 1: Start with Boilerplate

```bash
# For new projects:
cargo new myapp --lib  # or: go mod init myapp
cd myapp

# For existing projects, ensure you're in project root:
pwd  # Should show your project directory
git status  # Ensure clean working directory
```

#### Step 2: Share Guidelines with AI

**AI Chat Interaction:**

```
🧑: "I'm starting a new development session for our Rust/Go project. 

Please follow our AIBD-RG guidelines throughout this session:
- Feature-based architecture with one declaration per file
- Use established error handling patterns  
- Include comprehensive documentation
- Map tests to acceptance criteria
- Maintain clear dependency boundaries

Ready to begin?"

🤖: "Understood! I'll follow your AIBD-RG guidelines throughout our session..."
```

#### Step 3: Define Features Architecturally

**AI Chat Interaction:**

```
🧑: "I'd like to create a payment processing feature following AIBD-RG guidelines:

Feature: Payment Processing
Business Context: Enable users to make payments for premium features
Key Capabilities:
- Validate payment methods (credit cards, bank transfers)
- Process payments securely
- Handle payment failures and retries
- Store payment history

Dependencies:
- User Management feature (for user validation)
- External payment gateway (Stripe/PayPal)

Please follow our AIBD-RG guidelines:
1. First create the directory structure
2. Draft the PRD file with acceptance criteria
3. Don't implement yet - just structure and requirements"

🤖: "I'll create the payment processing feature structure following AIBD-RG..."
```

**Commands after AI generates structure:**

```bash
# Verify the AI created proper directory structure
tree src/features/payment_processing/  # Rust
# or
tree internal/features/payment/        # Go

# Review the generated PRD file
cat src/features/payment_processing/prd.md  # Rust
# or  
cat internal/features/payment/prd.md        # Go
```

#### Step 4: Generate Implementation

**AI Chat Interaction:**

```
🧑: "Great structure! Now implement the payment processing feature based on the PRD, following our AIBD-RG guidelines:

- Use our established error handling patterns (PaymentError types)
- Follow the Repository pattern for data access
- Include comprehensive documentation for all public APIs
- Use proper async patterns (Rust) or context propagation (Go)
- Keep one declaration per file
- Implement the public API surface first, then internal logic"

🤖: "I'll implement the payment processing feature following your established patterns..."
```

**Commands to verify generated code:**

```bash
# Check that files were created in the right places
find src/features/payment_processing -name "*.rs" | head -10  # Rust
find internal/features/payment -name "*.go" | head -10       # Go

# Quick compilation check
cargo check  # Rust
go build ./... # Go
```

#### Step 5: Implement Tests

**AI Chat Interaction:**

```
🧑: "Now let's ensure we have comprehensive test coverage. Please implement tests for all acceptance criteria listed in our payment processing PRD:

- Map each test directly to an acceptance criterion  
- Use our established testing patterns and utilities
- Include both success and failure scenarios
- Test error handling thoroughly
- Follow our naming conventions for test functions

Refer to existing tests in user_management feature for patterns."

🤖: "I'll create comprehensive tests mapping to the acceptance criteria in the PRD..."
```

**Commands to run initial tests:**

```bash
# Run just the new tests to see if they work
cargo test payment_processing  # Rust
go test ./internal/features/payment/...  # Go
```

#### Step 6: Validate with Tools

```bash
# Run full validation suite
# For Rust projects:
cargo fmt --all                           # Format code
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo test                                # Run all tests
cargo check                               # Final compilation check

# For Go projects:
make check  # Runs: fmt, vet, lint, test, check-cycles

# If using the Makefile from our documentation:
# This runs: go fmt, go vet, golangci-lint, go test, cycle detection
```

**If validation fails, go back to AI:**

```
🧑: "The validation failed with these specific errors:

[paste the actual error messages]

Please fix these issues while maintaining our AIBD-RG guidelines. Focus on:
- Fixing the linting violations
- Ensuring all tests pass
- Maintaining our architectural patterns"
```

#### Step 7: Commit with Conventional Messages

```bash
# Stage the changes
git add .

# Check what we're committing
git diff --cached --stat

# Commit with conventional message format
git commit -m "feat(payment-processing): implement secure payment validation and processing

- Add PaymentService with comprehensive validation logic
- Implement support for credit cards and bank transfers
- Add PaymentError types following established error patterns  
- Include PaymentRepository with proper async patterns
- Add complete test coverage for all acceptance criteria
- Integrate with external payment gateway abstraction"

# Push to remote
git push origin feature/payment-processing
```

### Summary of Complete Workflow

```bash
# Step 1: Project setup (one-time)
cargo new myapp --lib && cd myapp

# Step 2-5: AI interactions (chat-based, no commands)
# - Share AIBD-RG guidelines  
# - Define feature architecture
# - Generate implementation
# - Create comprehensive tests

# Step 6: Validation (repeat until passing)
cargo check && cargo test && cargo clippy  # Rust
make check                                  # Go

# Step 7: Clean commit
git add . && git commit -m "feat(feature-name): description"
```

**Key Point:** Steps 2-5 happen primarily through AI chat interactions, while steps 1, 6, and 7 use command-line tools. The validation commands in step 6 create a feedback loop with the AI to ensure quality.

-----

## 10. Common Anti-Patterns to Avoid

### Organizational Anti-Patterns

- ❌ Mixing business logic in main.rs/main.go
- ❌ Creating “god modules” with multiple responsibilities
- ❌ Circular dependencies between features
- ❌ Exposing internal implementation details

### Code Anti-Patterns

#### Rust

- ❌ Overusing `unwrap()` instead of proper error handling
- ❌ Using `String` everywhere instead of domain-specific types
- ❌ Ignoring clippy warnings about ownership patterns
- ❌ Creating deeply nested Result/Option chains

#### Go

- ❌ Using `panic()` for business logic errors
- ❌ Ignoring returned errors
- ❌ Creating interfaces with too many methods
- ❌ Using `interface{}` instead of generic types or specific interfaces

### Documentation Anti-Patterns

- ❌ Missing or outdated PRD files
- ❌ Code comments that repeat what code does instead of why
- ❌ Missing examples in public API documentation
- ❌ Test files that don’t map to acceptance criteria

-----

## Conclusion

These AIBD-RG guidelines provide a framework for maintainable AI-assisted development in Rust and Go. By following these rules, you can leverage AI coding assistance while maintaining code quality, consistency, and long-term maintainability.

Remember to:

- Continuously reinforce guidelines with AI assistants
- Use enforcement tools to catch deviations early
- Map tests directly to business requirements
- Keep features self-contained and boundaries clear
- Prioritize type safety and proper error handling

The key is not just generating code faster, but generating *better* code that remains comprehensible and maintainable as your project grows.