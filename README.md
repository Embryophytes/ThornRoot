[![StandWithUkraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/badges/StandWithUkraine.svg)](https://github.com/vshymanskyy/StandWithUkraine/blob/main/docs/README.md)
[![Stand With Ukraine](https://raw.githubusercontent.com/vshymanskyy/StandWithUkraine/main/banner2-direct.svg)](https://vshymanskyy.github.io/StandWithUkraine/)

# ThornRoot

ThornRoot is a foundational library designed for integration into the larger **BlackThorn** project. This library provides the core structures and custom logic for manipulating database schemas.

## Overview

ThornRoot offers a set of container structures with custom event-handling logic, enabling actions such as:
- Adding tables to a schema
- Updating existing tables
- Adding columns to tables
- Updating columns in tables

The primary goal of ThornRoot is to serve as a general-purpose tool for generating database migrations based on these structures. While full migration versioning is not yet implemented, the current version supports generating SQL code for creating tables and defining relationships.

## Installation

To add ThornRoot to your Rust project, use the following command:

```bash
cargo add thorn_root
```

Alternatively, add this to your `Cargo.toml`:

```toml
[dependencies]
thorn_root = "0.1.0"
```

## Features

- **Schema Manipulation**:
  - Define and manage database schemas programmatically.
  - Handle schema-related events such as table and column additions or updates.
- **Code Generation**:
  - Generate SQL scripts for table creation.
  - Generate SQL scripts for defining relationships between tables.

## Future Enhancements

ThornRoot is actively under development, and several key features are planned for future releases:
1. **Migration Versioning**:
   - Implement migration versioning to track changes and apply schema updates incrementally.
2. **Serialization Support**:
   - Add methods for serializing container structures to enable session storage for the BlackThorn frontend application.
3. **Enhanced Event Logic**:
   - Expand the event-handling logic for more complex schema manipulations.

## Integration with BlackThorn

As part of the BlackThorn project, ThornRoot plays a crucial role in managing the backend database schema. The library's focus on flexibility and customizability ensures seamless integration with the larger project's goals.

## Contributing

We welcome contributions! Feel free to submit issues, feature requests, or pull requests to help improve ThornRoot.

## License

ThornRoot is licensed under the [MIT License](LICENSE).

## Acknowledgements

Special thanks to **Thorn developer**


