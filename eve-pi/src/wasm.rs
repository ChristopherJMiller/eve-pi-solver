use crate::domain::ProductionPlan;
use crate::repository::{MemoryRepository, Repository};
use crate::solver::{Solver, SolverError};
use std::sync::Mutex;
use tracing::{debug, error, info, warn};
use wasm_bindgen::prelude::*;

// Use `wee_alloc` as the global allocator to reduce code size
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Wrap a repository in a Mutex since JavaScript is single-threaded
#[wasm_bindgen]
pub struct PiSolver {
    repository: Mutex<MemoryRepository>,
}

#[wasm_bindgen]
impl PiSolver {
    /// Create a new PiSolver instance
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        // Set up panic hook for better error messages
        crate::utils::set_panic_hook();

        // Initialize tracing
        crate::utils::init_tracing();

        info!("PiSolver instance created");

        Self {
            repository: Mutex::new(MemoryRepository::new()),
        }
    }

    /// Load planet data from JavaScript objects
    #[wasm_bindgen]
    pub fn load_planets(&self, planets_js: JsValue) -> Result<(), JsValue> {
        info!("WASM: Starting load_planets");

        let mut repo = self.repository.lock().map_err(|_| {
            error!("WASM: Failed to lock repository");
            JsValue::from_str("Failed to lock repository")
        })?;

        info!("WASM: Successfully locked repository, deserializing planets");

        // Deserialize JavaScript objects directly using serde-wasm-bindgen
        let planets: Vec<crate::domain::Planet> = serde_wasm_bindgen::from_value(planets_js)
            .map_err(|err| {
                error!("WASM: Failed to deserialize planets: {:?}", err);
                JsValue::from_str(&format!("Failed to deserialize planets: {:?}", err))
            })?;

        info!("WASM: Successfully deserialized planets, calling repo.load_planets_data");

        repo.load_planets_data(planets).map_err(|err| {
            error!("WASM: repo.load_planets_data failed: {}", err);
            JsValue::from_str(&format!("Failed to load planets: {}", err))
        })?;

        info!("WASM: load_planets completed successfully");
        Ok(())
    }

    /// Load character data from JavaScript objects
    #[wasm_bindgen]
    pub fn load_characters(&self, characters_js: JsValue) -> Result<(), JsValue> {
        info!("WASM: Starting load_characters");

        let mut repo = self.repository.lock().map_err(|_| {
            error!("WASM: Failed to lock repository for characters");
            JsValue::from_str("Failed to lock repository")
        })?;

        info!("WASM: Successfully locked repository for characters, deserializing characters");

        // Deserialize JavaScript objects directly using serde-wasm-bindgen
        let characters: Vec<crate::domain::Character> =
            serde_wasm_bindgen::from_value(characters_js).map_err(|err| {
                error!("WASM: Failed to deserialize characters: {:?}", err);
                JsValue::from_str(&format!("Failed to deserialize characters: {:?}", err))
            })?;

        info!("WASM: Successfully deserialized characters, calling repo.load_characters_data");

        info!("WASM: characters: {:?}", characters);

        repo.load_characters_data(characters).map_err(|err| {
            error!("WASM: Failed to load characters: {}", err);
            JsValue::from_str(&format!("Failed to load characters: {}", err))
        })?;

        info!("WASM: load_characters completed successfully");
        Ok(())
    }

    /// Solve for a production plan for the target product
    #[wasm_bindgen]
    pub fn solve(&self, target_product: String) -> Result<JsValue, JsValue> {
        info!("WASM: Starting solve for product: {}", target_product);

        let repo = self.repository.lock().map_err(|_| {
            error!("WASM: Failed to lock repository for solving");
            JsValue::from_str("Failed to lock repository")
        })?;

        info!("WASM: Successfully locked repository for solving");

        let solver = Solver::new(&*repo);
        let plan = solver.solve(&target_product).map_err(|err| {
            error!("WASM: Failed to solve: {:?}", err);
            JsValue::from_str(&format!("Failed to solve: {:?}", err))
        })?;

        info!("WASM: Successfully solved, converting to JavaScript object");

        // Convert the plan directly to a JavaScript object using serde-wasm-bindgen
        serde_wasm_bindgen::to_value(&plan).map_err(|err| {
            error!("WASM: Failed to serialize plan: {:?}", err);
            JsValue::from_str(&format!("Failed to serialize plan: {:?}", err))
        })
    }
}

/// Export helper function to convert a production plan to a simpler JavaScript format
#[wasm_bindgen]
pub fn format_production_plan(plan_js: JsValue) -> Result<JsValue, JsValue> {
    let plan: ProductionPlan = serde_wasm_bindgen::from_value(plan_js)
        .map_err(|err| JsValue::from_str(&format!("Failed to deserialize plan: {:?}", err)))?;

    // Create a simplified JavaScript-friendly structure
    let simplified_plan = plan
        .assignments
        .iter()
        .map(|assignment| {
            serde_json::json!({
                "character": assignment.character,
                "planet": assignment.planet,
                "type": format!("{:?}", assignment.planet_type),
                "output": assignment.output,
                "import": assignment.imported_inputs,
                "mine": assignment.mined_inputs
            })
        })
        .collect::<Vec<_>>();

    let result = serde_json::json!({
        "plan": simplified_plan
    });

    // Convert back to JsValue using serde-wasm-bindgen
    serde_wasm_bindgen::to_value(&result).map_err(|err| {
        JsValue::from_str(&format!("Failed to serialize simplified plan: {:?}", err))
    })
}
