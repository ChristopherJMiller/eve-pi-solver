# EVE Online Planetary Interaction (PI) Solver - Implementation Plan

## Overview

This plan outlines the approach for converting the EVE Online Planetary Interaction solver from Prolog to Rust. The goal is to create an efficient, well-tested library that can be compiled to WebAssembly (WASM) and used in web applications to determine optimal production plans for manufacturing products in EVE Online's planetary industry system.

## Core Domain Concepts

1. **Products**: Items at different tiers (P0-P4) with manufacturing requirements
2. **Planets**: Different planet types with available resources
3. **Characters**: In-game characters with skills and planet limits
4. **Factory Configurations**: Various setups for manufacturing chains
5. **Production Plan**: The final output showing character/planet assignments and production chains

## Module Structure

### 1. Core Domain Module (`domain.rs`)

- Define core structs and enums for all domain concepts
- Implement basic relationships between domain objects
- Pure business logic with no I/O dependencies

```rust
// Key types
pub enum ProductTier { P0, P1, P2, P3, P4 }
pub enum PlanetType { Barren, Gas, Ice, Lava, Oceanic, Plasma, Storm, Temperate }
pub struct Product { /* ... */ }
pub struct Planet { /* ... */ }
pub struct Character { /* ... */ }
pub struct FactorySetup { /* ... */ }
```

### 2. Data Repository Module (`repository.rs`)

- Responsible for managing game data
- Implement data access traits to allow for mockable test implementations
- Support for client-provided data (from JavaScript)

```rust
pub trait ProductRepository {
    fn get_all_products(&self) -> Vec<Product>;
    fn get_product_by_name(&self, name: &str) -> Option<Product>;
    // etc.
}

pub trait PlanetRepository {
    fn get_available_planets(&self) -> Vec<Planet>;
    // etc.
}

// In-memory implementation for use in WASM
pub struct MemoryRepository {
    // Implementation that holds data in memory (received from JS)
}
```

### 3. Factory Configuration Module (`factory.rs`)

- Implement the logic for determining valid factory configurations
- Port the factory_type and factory_planet predicates from Prolog
- Validate planet/resource combinations

```rust
pub struct FactoryConfiguration {
    pub start_tier: ProductTier,
    pub end_tier: ProductTier,
    pub imported_inputs: Vec<Product>,
    pub mined_inputs: Vec<Product>,
    pub outputs: Vec<Product>,
}

pub fn find_valid_factory_configurations(
    planet_type: PlanetType,
    target_product: &Product,
) -> Vec<FactoryConfiguration> {
    // Implementation...
}
```

### 4. Production Plan Solver Module (`solver.rs`)

- Core algorithm for generating production plans
- Define data structures for representing solution state
- Implement backtracking search algorithms (similar to Prolog's approach)

```rust
pub struct ProductionPlan {
    pub assignments: Vec<PlanetAssignment>,
}

pub struct PlanetAssignment {
    pub character: Character,
    pub planet: Planet,
    pub imported_inputs: Vec<Product>,
    pub mined_inputs: Vec<Product>,
    pub output: Product,
}

pub fn solve_production_plan(
    target_product: &Product,
    available_planets: &[Planet],
    available_characters: &[Character],
) -> Option<ProductionPlan> {
    // Implementation...
}
```

### 5. WASM Binding Module (`wasm.rs`)

- Provides JavaScript-friendly API for the library
- Handles serialization/deserialization of data between JS and Rust
- Implements exported functions with wasm-bindgen

```rust
#[wasm_bindgen]
pub struct PiSolver {
    repository: Box<dyn Repository>,
}

#[wasm_bindgen]
impl PiSolver {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Initialize solver
    }

    #[wasm_bindgen]
    pub fn load_data(&mut self, planets_json: &str, characters_json: &str) -> Result<(), JsValue> {
        // Parse JSON and populate repository
    }

    #[wasm_bindgen]
    pub fn solve(&self, target_product: &str) -> Result<JsValue, JsValue> {
        // Solve and return JSON plan
    }
}
```

### 6. Library Entry Point (`lib.rs`)

- Configures the library
- Exports public API
- Sets up WASM integration

```rust
mod domain;
mod repository;
mod factory;
mod solver;
mod wasm;

// Re-export the WASM API
pub use wasm::PiSolver;
```

## Key Algorithms

### 1. Factory Configuration Validation

- Determine if a planet can support a specific factory type
- Validate that required resources are available on the planet
- Ensure production chains are valid (all inputs can be produced)

### 2. Production Plan Generation

- Implement a backtracking search algorithm to find valid assignments
- Use heuristics to prioritize promising assignments
- Track character planet assignments to enforce limits
- Handle special cases (like P4 products with direct P0 resource requirements)

### 3. Resource Optimization

- Minimize the number of planets needed
- Optimize for shortest production chains
- Balance production across characters

## Data Model Relationships

```
Character (1) --< Planet Assignments (*)
Planet Assignment (*) --< Factory Configuration (1)
Factory Configuration (*) --< Products (*)
Products (*) --< Manufacturing Chain (1)
Planet Types (*) --< Available Resources (*)
```

## Testing Strategy

### 1. Unit Tests

- Test individual functions in isolation
- Mock dependencies for predictable testing
- Focus on correctness of algorithms

```rust
#[test]
fn test_factory_configuration_validation() {
    // Test setup...
    assert!(is_valid_factory(planet, config));
}
```

### 2. Integration Tests

- Test multiple modules working together
- Verify data loading and transformation
- Ensure proper interaction between components

```rust
#[test]
fn test_solver_with_repository() {
    let repo = MockRepository::new();
    let solver = Solver::new(repo);
    let plan = solver.solve("broadcast_node");
    assert!(plan.is_valid());
}
```

### 3. Acceptance Tests

- Test against the examples provided
- Verify that output matches expected results
- Test with different target products

```rust
#[test]
fn test_example_nano_factory() {
    // Load example data...
    let result = solve_production_plan("nano_factory", planets, characters);
    assert_eq!(result, expected_output);
}
```

### 4. WASM-specific Tests

- Test the JS API functionality
- Verify proper data serialization/deserialization
- Ensure error handling works across the language boundary

```rust
#[wasm_bindgen_test]
fn test_wasm_api() {
    let solver = PiSolver::new();
    let result = solver.load_data(planets_json, characters_json);
    assert!(result.is_ok());
}
```

### 5. Property-Based Tests

- Generate random valid inputs
- Verify that solutions maintain invariants
- Test edge cases and boundaries

```rust
#[test]
fn test_solver_properties() {
    // Use proptest or similar to generate valid inputs
    // Verify that solutions are valid
}
```

## Implementation Phases

1. **Phase 1**: Core domain model and repository

   - Implement basic structs and enums
   - Set up core game data structures
   - Basic test harness

2. **Phase 2**: Factory configuration logic

   - Implement validation of factory setups
   - Port the rules from Prolog
   - Test with example data

3. **Phase 3**: Production plan solver

   - Implement backtracking search
   - Handle character and planet constraints
   - Generate valid production plans

4. **Phase 4**: WASM binding and JavaScript API

   - Add wasm-bindgen integration
   - Create JavaScript-friendly API
   - Handle serialization and error handling

5. **Phase 5**: Optimization and Web Demo

   - Improve algorithm efficiency
   - Add heuristics for better plans
   - Create a simple web demo to showcase functionality

## WASM Build Configuration

1. **Dependencies**:

   - `wasm-bindgen` for JavaScript interop
   - `serde` and `serde_json` for serialization
   - `web-sys` for any browser API interactions
   - `wasm-bindgen-test` for WASM-specific testing

2. **Cargo.toml Configuration**:

```toml
[package]
name = "eve-pi-solver"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
wasm-bindgen = "0.2"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
web-sys = { version = "0.3", features = ["console"] }

[dev-dependencies]
wasm-bindgen-test = "0.3"
```

3. **Build Process**:
   - Use `wasm-pack build` to compile to WASM
   - Generated files will include JavaScript glue code
   - Package can be published to npm

## Performance Considerations

1. Use efficient data structures for lookups (HashMap, HashSet)
2. Implement caching for expensive operations
3. Keep the WASM bundle size small (use `wee_alloc` as allocator)
4. Minimize data crossing the JS/WASM boundary
5. Benchmark critical algorithms and optimize hot spots

## Error Handling Strategy

1. Use Rust's Result type for recoverable errors
2. Map errors to JavaScript-friendly representations
3. Provide clear error messages for common issues
4. Include context information for debugging
5. Graceful fallback when optimal solutions can't be found

## JavaScript Integration

1. Provide a simple, promise-based API for JavaScript consumers
2. Handle async operations appropriately
3. Provide TypeScript definitions for better developer experience
4. Include examples for common web frameworks (React, Vue, etc.)
5. Document memory management considerations for WASM

## Example JavaScript Usage

```javascript
import { PiSolver } from "eve-pi-solver";

async function generatePlan() {
  const solver = new PiSolver();

  // Load data from API or static files
  const planets = await fetch("/planets.json").then((r) => r.json());
  const characters = await fetch("/characters.json").then((r) => r.json());

  // Load data into solver
  await solver.load_data(JSON.stringify(planets), JSON.stringify(characters));

  // Generate a production plan
  const plan = await solver.solve("nano_factory");

  // Display results
  console.log(JSON.parse(plan));
}
```

This plan provides a roadmap for implementing a robust, efficient EVE Online PI solver in Rust, designed to be compiled to WebAssembly for use in web applications.
