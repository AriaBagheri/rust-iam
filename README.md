
# Policy Validation System

A robust, flexible, and extensible policy validation framework for managing access control based on user actions and resources. This system evaluates policies using customizable matching logic and allows combining multiple policies into collections for advanced validation scenarios.

---

## Features

- **Extensible Policy Management**: Supports dynamic addition of policies to collections.
- **Fine-Grained Control**: Validates actions against resources using multiple attributes like region, service, account ID, etc.
- **Efficient Decision-Making**:
    - Explicit `Allow` and `Deny` rules.
    - Defaults to `NotSpecified` if no explicit match exists.
- **Customizable Matching**: Generic types allow flexible integration with any action or resource model.

---

## Getting Started

### Prerequisites

- **Rust**: Ensure you have Rust installed. You can download it from [rust-lang.org](https://www.rust-lang.org/).

### Installation

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd <repository-name>
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the tests:
   ```bash
   cargo test
   ```

---

## Usage

### Define Policies

Define your policies using the `Policy` struct and `MaybeEffect` enum:

- `Allow`: Grants access.
- `Deny`: Denies access (overrides `Allow`).
- `NotSpecified`: No explicit rule.

### Create a Policy Collection

Group policies together into a `PolicyCollection`:

```rust
let mut collection = PolicyCollection::<ActionType, PartitionType, ServiceType, RegionType, AccountIDType, ResourceType, ResourceIDType>(vec![policy]);
```

### Validate Actions

Validate an action against a resource:

```rust
let is_allowed = collection.validate(&action, &resource);

if is_allowed {
    println!("Access granted");
} else {
    println!("Access denied");
}
```

---

## API Reference

### `PolicyCollection`

A collection of policies for validating access control.

#### Methods:

1. **`extend`**: Add multiple policies to the collection.
   ```rust
   collection.extend(vec![another_policy]);
   ```

2. **`validate`**: Validate an action and resource against the collection.
   ```rust
   let result = collection.validate(&action, &resource);
   ```

---

## Examples

### Example: Basic Policy Validation

```rust
let allow_policy = Policy::new(|action, resource| {
    if action.matches(&true) && resource.matches(&true) {
        MaybeEffect::Allow
    } else {
        MaybeEffect::NotSpecified
    }
});

let deny_policy = Policy::new(|_, _| MaybeEffect::Deny);

let mut collection = PolicyCollection(vec![allow_policy]);
collection.extend(vec![deny_policy]);

let action = ActionType::SomeAction;
let resource = ResourceAbstract::new(PartitionType, ServiceType, RegionType, AccountIDType, ResourceType, ResourceIDType);

let result = collection.validate(&action, &resource);
println!("Access: {}", if result { "granted" } else { "denied" });
```

---

## Testing

Run the unit tests to ensure the framework works as expected:

```bash
cargo test
```

---

## Contributing

We welcome contributions! Follow these steps to contribute:

1. Fork the repository.
2. Create a new branch:
   ```bash
   git checkout -b feature/my-feature
   ```
3. Make your changes and commit them:
   ```bash
   git commit -m "Add my feature"
   ```
4. Push your branch:
   ```bash
   git push origin feature/my-feature
   ```
5. Open a pull request.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Contact

For questions or support, please open an issue or contact the maintainer.
