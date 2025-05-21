use crate::domain::{Character, Planet, Product, ProductionPlan, create_product_database};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// Represents errors that can occur when working with repositories
#[derive(Debug)]
pub enum RepositoryError {
    DeserializationError(String),
    ProductNotFound(String),
    InvalidData(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::DeserializationError(msg) => write!(f, "Failed to deserialize data: {}", msg),
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
        let planets: Vec<Planet> = serde_json::from_str(json)
            .map_err(|e| RepositoryError::DeserializationError(e.to_string()))?;
        
        for planet in planets {
            self.planets.insert(planet.id.clone(), planet);
        }
        
        Ok(())
    }

    /// Load characters from JSON string
    pub fn load_characters(&mut self, json: &str) -> Result<(), RepositoryError> {
        let characters: Vec<Character> = serde_json::from_str(json)
            .map_err(|e| RepositoryError::DeserializationError(e.to_string()))?;
        
        for character in characters {
            self.characters.insert(character.name.clone(), character);
        }
        
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