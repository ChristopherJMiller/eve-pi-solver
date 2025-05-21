use wasm_bindgen::prelude::*;
use crate::domain::ProductionPlan;
use crate::repository::{MemoryRepository, Repository};
use crate::solver::{Solver, SolverError};
use std::sync::Mutex;

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
        
        Self {
            repository: Mutex::new(MemoryRepository::new()),
        }
    }
    
    /// Load planet data from a JSON string
    #[wasm_bindgen]
    pub fn load_planets(&self, json: &str) -> Result<(), JsValue> {
        let mut repo = self.repository.lock().map_err(|_| {
            JsValue::from_str("Failed to lock repository")
        })?;
        
        repo.load_planets(json).map_err(|err| {
            JsValue::from_str(&format!("Failed to load planets: {}", err))
        })
    }
    
    /// Load character data from a JSON string
    #[wasm_bindgen]
    pub fn load_characters(&self, json: &str) -> Result<(), JsValue> {
        let mut repo = self.repository.lock().map_err(|_| {
            JsValue::from_str("Failed to lock repository")
        })?;
        
        repo.load_characters(json).map_err(|err| {
            JsValue::from_str(&format!("Failed to load characters: {}", err))
        })
    }
    
    /// Solve for a production plan for the target product
    #[wasm_bindgen]
    pub fn solve(&self, target_product: &str) -> Result<JsValue, JsValue> {
        let repo = self.repository.lock().map_err(|_| {
            JsValue::from_str("Failed to lock repository")
        })?;
        
        let solver = Solver::new(&*repo);
        let plan = solver.solve(target_product).map_err(|err| {
            JsValue::from_str(&format!("Failed to solve: {:?}", err))
        })?;
        
        // Convert the plan to JSON for JavaScript
        let plan_json = serde_json::to_string(&plan).map_err(|err| {
            JsValue::from_str(&format!("Failed to serialize plan: {}", err))
        })?;
        
        Ok(JsValue::from_str(&plan_json))
    }
}

/// Export helper function to convert a production plan to a simpler JavaScript format
#[wasm_bindgen]
pub fn format_production_plan(plan_json: &str) -> Result<JsValue, JsValue> {
    let plan: ProductionPlan = serde_json::from_str(plan_json).map_err(|err| {
        JsValue::from_str(&format!("Failed to parse plan: {}", err))
    })?;
    
    // Create a JavaScript-friendly structure
    let assignments = plan.assignments.iter().map(|assignment| {
        let mut obj = js_sys::Object::new();
        
        js_sys::Reflect::set(&obj, &JsValue::from_str("character"), &JsValue::from_str(&assignment.character)).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("planet"), &JsValue::from_str(&assignment.planet)).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("type"), &JsValue::from_str(&format!("{:?}", assignment.planet_type))).unwrap();
        js_sys::Reflect::set(&obj, &JsValue::from_str("output"), &JsValue::from_str(&assignment.output)).unwrap();
        
        // Convert imported inputs to JS array
        let imported = js_sys::Array::new();
        for input in &assignment.imported_inputs {
            imported.push(&JsValue::from_str(input));
        }
        js_sys::Reflect::set(&obj, &JsValue::from_str("import"), &imported).unwrap();
        
        // Convert mined inputs to JS array
        let mined = js_sys::Array::new();
        for input in &assignment.mined_inputs {
            mined.push(&JsValue::from_str(input));
        }
        js_sys::Reflect::set(&obj, &JsValue::from_str("mine"), &mined).unwrap();
        
        obj
    }).collect::<js_sys::Array>();
    
    // Create the final object
    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &JsValue::from_str("plan"), &assignments).unwrap();
    
    Ok(JsValue::from(result))
} 