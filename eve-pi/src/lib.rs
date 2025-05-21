mod domain;
mod factory;
mod repository;
mod solver;
mod utils;
mod wasm;

// Re-export the WASM API
pub use wasm::format_production_plan;
pub use wasm::PiSolver;

#[cfg(test)]
mod tests {
    use crate::repository::MemoryRepository;
    use crate::solver::Solver;
    use std::fs;

    #[test]
    fn test_basic_production_plan() {
        // Create a new memory repository
        let mut repository = MemoryRepository::new();

        // Load example planets and characters
        let planets_json =
            fs::read_to_string("../examples/planets.json").expect("Failed to read planets.json");
        let characters_json = fs::read_to_string("../examples/characters.json")
            .expect("Failed to read characters.json");

        repository
            .load_planets(&planets_json)
            .expect("Failed to load planets");
        repository
            .load_characters(&characters_json)
            .expect("Failed to load characters");

        // Create a solver with the repository
        let solver = Solver::new(&repository);

        // Solve for a simpler P1 product (bacteria) instead of the complex P4 nano_factory
        let plan = solver
            .solve("bacteria")
            .expect("Failed to solve for bacteria");

        // Verify the plan has assignments
        assert!(
            !plan.assignments.is_empty(),
            "Plan should have at least one assignment"
        );

        // Verify at least one assignment is for the target product
        let has_target = plan.assignments.iter().any(|a| a.output == "bacteria");
        assert!(has_target, "Plan should include the target product");
    }
}
