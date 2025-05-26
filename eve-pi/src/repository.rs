use crate::domain::{create_product_database, Character, Planet, Product};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use tracing::{debug, error, info};

/// Represents errors that can occur when working with repositories
#[derive(Debug)]
pub enum RepositoryError {
    /// Error that occurs when deserializing data
    DeserializationError(String),
    /// Error that occurs when a product is not found
    ProductNotFound(String),
    /// Error that occurs when data is invalid
    InvalidData(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::DeserializationError(msg) => {
                write!(f, "Failed to deserialize data: {}", msg)
            }
            RepositoryError::ProductNotFound(name) => write!(f, "Product not found: {}", name),
            RepositoryError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
        }
    }
}

impl Error for RepositoryError {}

/// Repository trait for accessing product data
pub trait ProductRepository {
    fn get_all_products(&self) -> Vec<Product>;
    fn get_product_by_name(&self, name: &str) -> Option<Product>;
    fn get_products_by_tier(&self, tier: crate::domain::ProductTier) -> Vec<Product>;
}

/// Repository trait for accessing planet data
pub trait PlanetRepository {
    fn get_all_planets(&self) -> Vec<Planet>;
    fn get_planet_by_id(&self, id: &str) -> Option<Planet>;
}

/// Repository trait for accessing character data
pub trait CharacterRepository {
    fn get_all_characters(&self) -> Vec<Character>;
    fn get_character_by_name(&self, name: &str) -> Option<Character>;
}

/// Combined repository trait for accessing all data
pub trait Repository: ProductRepository + PlanetRepository + CharacterRepository {}

/// Memory-based repository implementation
pub struct MemoryRepository {
    products: HashMap<String, Product>,
    planets: HashMap<String, Planet>,
    characters: HashMap<String, Character>,
}

impl MemoryRepository {
    /// Create a new empty repository
    pub fn new() -> Self {
        Self {
            products: create_product_database(),
            planets: HashMap::new(),
            characters: HashMap::new(),
        }
    }

    /// Load planets from JSON string
    pub fn load_planets(&mut self, json: &str) -> Result<(), RepositoryError> {
        info!("Loading planets from JSON (length: {})", json.len());
        debug!("JSON content: {}", json);

        // Try the simple approach first
        let planets: Vec<Planet> = serde_json::from_str(json).map_err(|e| {
            error!("Simple deserialization failed: {}", e);
            RepositoryError::DeserializationError(e.to_string())
        })?;

        info!("Successfully deserialized {} planets", planets.len());

        for (i, planet) in planets.iter().enumerate() {
            debug!("Processing planet {}: {:?}", i, planet);
            self.planets.insert(planet.id.clone(), planet.clone());
        }

        info!("Finished loading planets");
        Ok(())
    }

    /// Load characters from JSON string
    pub fn load_characters(&mut self, json: &str) -> Result<(), RepositoryError> {
        info!("Loading characters from JSON (length: {})", json.len());
        debug!("JSON content: {}", json);

        let characters: Vec<Character> = serde_json::from_str(json).map_err(|e| {
            error!("Failed to deserialize characters: {}", e);
            RepositoryError::DeserializationError(e.to_string())
        })?;

        info!("Successfully deserialized {} characters", characters.len());

        for (i, character) in characters.iter().enumerate() {
            debug!("Processing character {}: {:?}", i, character);
            self.characters
                .insert(character.name.clone(), character.clone());
        }

        info!("Finished loading characters");
        Ok(())
    }

    /// Load planets data directly from deserialized objects
    pub fn load_planets_data(&mut self, planets: Vec<Planet>) -> Result<(), RepositoryError> {
        info!("Loading {} planets from deserialized data", planets.len());

        for (i, planet) in planets.iter().enumerate() {
            debug!("Processing planet {}: {:?}", i, planet);
            self.planets.insert(planet.id.clone(), planet.clone());
        }

        info!("Finished loading planets data");
        Ok(())
    }

    /// Load characters data directly from deserialized objects
    pub fn load_characters_data(
        &mut self,
        characters: Vec<Character>,
    ) -> Result<(), RepositoryError> {
        info!(
            "Loading {} characters from deserialized data",
            characters.len()
        );

        for (i, character) in characters.iter().enumerate() {
            debug!("Processing character {}: {:?}", i, character);
            self.characters
                .insert(character.name.clone(), character.clone());
        }

        info!("Finished loading characters data");
        Ok(())
    }
}

impl ProductRepository for MemoryRepository {
    fn get_all_products(&self) -> Vec<Product> {
        self.products.values().cloned().collect()
    }

    fn get_product_by_name(&self, name: &str) -> Option<Product> {
        self.products.get(name).cloned()
    }

    fn get_products_by_tier(&self, tier: crate::domain::ProductTier) -> Vec<Product> {
        self.products
            .values()
            .filter(|p| p.tier == tier)
            .cloned()
            .collect()
    }
}

impl PlanetRepository for MemoryRepository {
    fn get_all_planets(&self) -> Vec<Planet> {
        self.planets.values().cloned().collect()
    }

    fn get_planet_by_id(&self, id: &str) -> Option<Planet> {
        self.planets.get(id).cloned()
    }
}

impl CharacterRepository for MemoryRepository {
    fn get_all_characters(&self) -> Vec<Character> {
        self.characters.values().cloned().collect()
    }

    fn get_character_by_name(&self, name: &str) -> Option<Character> {
        self.characters.get(name).cloned()
    }
}

impl Repository for MemoryRepository {}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_load_planets_basic() {
        let mut repo = MemoryRepository::new();

        let planets_json = r#"[
            {
                "id": "planet_1",
                "planet_type": "Barren",
                "resources": ["base_metals", "heavy_metals"]
            }
        ]"#;

        let result = repo.load_planets(planets_json);
        assert!(result.is_ok(), "Failed to load basic planets: {:?}", result);

        let planets = repo.get_all_planets();
        assert_eq!(planets.len(), 1);
        assert_eq!(planets[0].id, "planet_1");
    }

    #[traced_test]
    #[test]
    fn test_load_planets_from_frontend() {
        let mut repo = MemoryRepository::new();

        // This is the exact JSON that's being sent from the frontend
        let planets_json = r#"[{"id":"planet_1","planet_type":"Barren","resources":["base_metals","heavy_metals","noble_metals","chiral_structures"]},{"id":"planet_3","planet_type":"Temperate","resources":["aqueous_liquids","carbon_compounds","complex_organisms","micro_organisms","autotrophs"]},{"id":"planet_4","planet_type":"Gas","resources":["carbon_compounds","ionic_solutions","noble_gas","reactive_gas","suspended_plasma"]},{"id":"planet_5","planet_type":"Oceanic","resources":["aqueous_liquids","micro_organisms","planktic_colonies"]}]"#;

        let result = repo.load_planets(planets_json);
        assert!(
            result.is_ok(),
            "Failed to load frontend planets: {:?}",
            result
        );

        let planets = repo.get_all_planets();
        assert_eq!(planets.len(), 4);

        // Verify specific planets
        let planet_1 = repo.get_planet_by_id("planet_1").unwrap();
        assert_eq!(planet_1.planet_type, crate::domain::PlanetType::Barren);
        assert_eq!(planet_1.resources.len(), 4);

        let planet_3 = repo.get_planet_by_id("planet_3").unwrap();
        assert_eq!(planet_3.planet_type, crate::domain::PlanetType::Temperate);
        assert_eq!(planet_3.resources.len(), 5);
    }

    #[traced_test]
    #[test]
    fn test_planet_type_deserialization() {
        use crate::domain::{Planet, PlanetType};

        // Test individual planet type deserialization
        let test_cases = vec![
            ("Barren", PlanetType::Barren),
            ("Gas", PlanetType::Gas),
            ("Ice", PlanetType::Ice),
            ("Lava", PlanetType::Lava),
            ("Oceanic", PlanetType::Oceanic),
            ("Plasma", PlanetType::Plasma),
            ("Storm", PlanetType::Storm),
            ("Temperate", PlanetType::Temperate),
        ];

        for (json_str, expected_type) in test_cases {
            let planet_json = format!(
                r#"{{"id":"test","planet_type":"{}","resources":[]}}"#,
                json_str
            );

            let planet: Result<Planet, _> = serde_json::from_str(&planet_json);
            assert!(
                planet.is_ok(),
                "Failed to deserialize planet type {}: {:?}",
                json_str,
                planet
            );

            let planet = planet.unwrap();
            assert_eq!(planet.planet_type, expected_type);
        }
    }

    #[traced_test]
    #[test]
    fn test_load_characters_basic() {
        let mut repo = MemoryRepository::new();

        let characters_json = r#"[
            {
                "name": "test_character",
                "planets": 5,
                "skills": {
                    "command_center_upgrades": 5,
                    "interplanetary_consolidation": 5,
                    "remote_sensing": 4,
                    "planetary_production": 3,
                    "planetology": 2,
                    "advanced_planetology": 1
                }
            }
        ]"#;

        let result = repo.load_characters(characters_json);
        assert!(result.is_ok(), "Failed to load characters: {:?}", result);

        let characters = repo.get_all_characters();
        assert_eq!(characters.len(), 1);
        assert_eq!(characters[0].name, "test_character");
        assert_eq!(characters[0].skills.command_center_upgrades, 5);
        assert_eq!(characters[0].skills.remote_sensing, Some(4));
    }
}
