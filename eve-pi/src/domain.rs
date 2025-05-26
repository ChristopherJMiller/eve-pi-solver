use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents the tier of a product in the production chain
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ProductTier {
    P0, // Raw materials
    P1, // Basic processed materials
    P2, // Refined commodities
    P3, // Specialized commodities
    P4, // Advanced commodities
}

/// Represents the type of planet in EVE Online
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlanetType {
    Barren,
    Gas,
    Ice,
    Lava,
    Oceanic,
    Plasma,
    Storm,
    Temperate,
}

/// Represents a product in the planetary production chain
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub tier: ProductTier,
    pub ingredients: Vec<String>, // Names of products required to produce this product
}

impl Product {
    /// Create a new product
    pub fn new(name: String, tier: ProductTier, ingredients: Vec<String>) -> Self {
        Self {
            name,
            tier,
            ingredients,
        }
    }

    /// Create a P0 raw material (no ingredients)
    pub fn new_raw_material(name: String) -> Self {
        Self {
            name,
            tier: ProductTier::P0,
            ingredients: Vec::new(),
        }
    }
}

/// Represents a planet in EVE Online
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Planet {
    pub id: String,
    pub planet_type: PlanetType,
    pub resources: Vec<String>, // Names of P0 resources available on this planet
}

/// Represents character skills for planetary industry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterSkills {
    pub command_center_upgrades: u8,
    pub interplanetary_consolidation: u8,
    #[serde(default)]
    pub remote_sensing: Option<u8>,
    #[serde(default)]
    pub planetary_production: Option<u8>,
    #[serde(default)]
    pub planetology: Option<u8>,
    #[serde(default)]
    pub advanced_planetology: Option<u8>,
}

/// Represents a character in EVE Online
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub planets: usize,          // Number of planets the character can manage
    pub skills: CharacterSkills, // Skill levels for different planetary skills
}

/// Represents a factory configuration for a planet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactoryConfiguration {
    pub start_tier: ProductTier,
    pub end_tier: ProductTier,
    pub imported_inputs: Vec<String>, // Names of products that need to be imported
    pub mined_inputs: Vec<String>,    // Names of products that can be mined on the planet
    pub outputs: Vec<String>,         // Names of products that can be produced
}

/// Represents an assignment of a planet to produce a specific product
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanetAssignment {
    pub character: String, // Character name
    pub planet: String,    // Planet ID
    pub planet_type: PlanetType,
    pub imported_inputs: Vec<String>, // Products imported to this planet
    pub mined_inputs: Vec<String>,    // Products mined on this planet
    pub output: String,               // Product being produced
}

/// Represents a complete production plan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionPlan {
    pub assignments: Vec<PlanetAssignment>,
}

/// Specialized products in P4 tier that require direct P0 mining
pub fn requires_p4_mined(product_name: &str) -> bool {
    matches!(
        product_name,
        "nano_factory" | "organic_mortar_applicators" | "sterile_conduit"
    )
}

/// Maps each P0 resource to the planet types it can be found on
pub fn planet_resource_map() -> HashMap<&'static str, Vec<PlanetType>> {
    let mut map = HashMap::new();

    map.insert(
        "aqueous_liquids",
        vec![PlanetType::Oceanic, PlanetType::Temperate],
    );
    map.insert("autotrophs", vec![PlanetType::Temperate]);
    map.insert(
        "base_metals",
        vec![PlanetType::Barren, PlanetType::Lava, PlanetType::Plasma],
    );
    map.insert(
        "carbon_compounds",
        vec![PlanetType::Gas, PlanetType::Temperate],
    );
    map.insert("complex_organisms", vec![PlanetType::Temperate]);
    map.insert("felsic_magma", vec![PlanetType::Lava]);
    map.insert(
        "heavy_metals",
        vec![PlanetType::Barren, PlanetType::Lava, PlanetType::Plasma],
    );
    map.insert("ionic_solutions", vec![PlanetType::Gas, PlanetType::Storm]);
    map.insert(
        "micro_organisms",
        vec![PlanetType::Oceanic, PlanetType::Temperate],
    );
    map.insert("noble_gas", vec![PlanetType::Gas, PlanetType::Ice]);
    map.insert("noble_metals", vec![PlanetType::Barren, PlanetType::Plasma]);
    map.insert("non_cs_crystals", vec![PlanetType::Ice, PlanetType::Plasma]);
    map.insert("planktic_colonies", vec![PlanetType::Oceanic]);
    map.insert("reactive_gas", vec![PlanetType::Gas, PlanetType::Storm]);
    map.insert(
        "suspended_plasma",
        vec![PlanetType::Gas, PlanetType::Plasma, PlanetType::Storm],
    );

    map
}

// Define the product database
pub fn create_product_database() -> HashMap<String, Product> {
    let mut products = HashMap::new();

    // P0 Products (raw materials)
    let p0_names = vec![
        "aqueous_liquids",
        "autotrophs",
        "base_metals",
        "carbon_compounds",
        "complex_organisms",
        "felsic_magma",
        "heavy_metals",
        "ionic_solutions",
        "micro_organisms",
        "noble_gas",
        "noble_metals",
        "non_cs_crystals",
        "planktic_colonies",
        "reactive_gas",
        "suspended_plasma",
    ];

    for name in p0_names {
        products.insert(
            name.to_string(),
            Product::new_raw_material(name.to_string()),
        );
    }

    // P1 Products (basic processed materials)
    let p1_products = vec![
        ("bacteria", vec!["micro_organisms"]),
        ("biofuels", vec!["carbon_compounds"]),
        ("biomass", vec!["planktic_colonies"]),
        ("chiral_structures", vec!["non_cs_crystals"]),
        ("electrolytes", vec!["ionic_solutions"]),
        ("industrial_fibers", vec!["autotrophs"]),
        ("oxidizing_compound", vec!["reactive_gas"]),
        ("oxygen", vec!["noble_gas"]),
        ("plasmoids", vec!["suspended_plasma"]),
        ("precious_metals", vec!["noble_metals"]),
        ("proteins", vec!["complex_organisms"]),
        ("reactive_metals", vec!["base_metals"]),
        ("silicon", vec!["felsic_magma"]),
        ("toxic_metals", vec!["heavy_metals"]),
        ("water", vec!["aqueous_liquids"]),
    ];

    for (name, ingredients) in p1_products {
        products.insert(
            name.to_string(),
            Product::new(
                name.to_string(),
                ProductTier::P1,
                ingredients.iter().map(|s| s.to_string()).collect(),
            ),
        );
    }

    // P2 Products (simplified list for brevity)
    let p2_products = vec![
        ("biocells", vec!["precious_metals", "biofuels"]),
        (
            "construction_blocks",
            vec!["toxic_metals", "reactive_metals"],
        ),
        (
            "consumer_electronics",
            vec!["chiral_structures", "toxic_metals"],
        ),
        ("coolant", vec!["water", "electrolytes"]),
        ("enriched_uranium", vec!["toxic_metals", "precious_metals"]),
        ("fertilizer", vec!["proteins", "bacteria"]),
        ("livestock", vec!["biofuels", "proteins"]),
        (
            "mechanical_parts",
            vec!["precious_metals", "reactive_metals"],
        ),
        ("microfiber_shielding", vec!["silicon", "industrial_fibers"]),
        (
            "miniature_electronics",
            vec!["silicon", "chiral_structures"],
        ),
        ("nanites", vec!["reactive_metals", "bacteria"]),
        ("oxides", vec!["oxygen", "oxidizing_compound"]),
        (
            "polyaramids",
            vec!["industrial_fibers", "oxidizing_compound"],
        ),
        ("polytextiles", vec!["industrial_fibers", "biofuels"]),
        ("rocket_fuel", vec!["electrolytes", "plasmoids"]),
        ("silicate_glass", vec!["silicon", "oxidizing_compound"]),
        ("superconductors", vec!["water", "plasmoids"]),
        ("supertensile_plastics", vec!["oxygen", "biomass"]),
        ("synthetic_oil", vec!["oxygen", "electrolytes"]),
        ("test_cultures", vec!["water", "bacteria"]),
        ("viral_agent", vec!["biomass", "bacteria"]),
    ];

    for (name, ingredients) in p2_products {
        products.insert(
            name.to_string(),
            Product::new(
                name.to_string(),
                ProductTier::P2,
                ingredients.iter().map(|s| s.to_string()).collect(),
            ),
        );
    }

    // P3 Products (simplified for brevity)
    let p3_products = vec![
        (
            "biotech_research_reports",
            vec!["nanites", "livestock", "construction_blocks"],
        ),
        (
            "camera_drones",
            vec!["silicate_glass", "rocket_fuel", "mechanical_parts"],
        ),
        ("condensates", vec!["oxides", "coolant", "precious_metals"]),
        (
            "cryoprotectant_solution",
            vec!["synthetic_oil", "fertilizer", "polytextiles"],
        ),
        (
            "data_chips",
            vec!["nanites", "silicate_glass", "consumer_electronics"],
        ),
        (
            "gel_matrix_biopaste",
            vec!["oxides", "biocells", "industrial_fibers"],
        ),
        (
            "guidance_systems",
            vec![
                "consumer_electronics",
                "mechanical_parts",
                "miniature_electronics",
            ],
        ),
        (
            "hazmat_detection_systems",
            vec!["industrial_fibers", "oxides", "microfiber_shielding"],
        ),
        (
            "hermetic_membranes",
            vec!["polytextiles", "silicate_glass", "coolant"],
        ),
        (
            "high_tech_transmitters",
            vec![
                "chiral_structures",
                "miniature_electronics",
                "superconductors",
            ],
        ),
        (
            "industrial_explosives",
            vec!["fertilizer", "polytextiles", "reactive_metals"],
        ),
        (
            "neocoms",
            vec!["biocells", "construction_blocks", "microfiber_shielding"],
        ),
        (
            "nuclear_reactors",
            vec![
                "enriched_uranium",
                "microfiber_shielding",
                "consumer_electronics",
            ],
        ),
        (
            "planetary_vehicles",
            vec!["rocket_fuel", "consumer_electronics", "mechanical_parts"],
        ),
        (
            "robotics",
            vec![
                "mechanical_parts",
                "consumer_electronics",
                "precious_metals",
            ],
        ),
        (
            "smartfab_units",
            vec!["construction_blocks", "miniature_electronics", "nanites"],
        ),
        (
            "supercomputers",
            vec!["coolant", "consumer_electronics", "miniature_electronics"],
        ),
        (
            "synthetic_synapses",
            vec!["supertensile_plastics", "test_cultures", "biocells"],
        ),
        (
            "transcranial_microcontrollers",
            vec!["biocells", "nanites", "silicate_glass"],
        ),
        (
            "ukomi_super_conductors",
            vec!["synthetic_oil", "superconductors", "coolant"],
        ),
        ("vaccines", vec!["livestock", "viral_agent"]),
    ];

    for (name, ingredients) in p3_products {
        products.insert(
            name.to_string(),
            Product::new(
                name.to_string(),
                ProductTier::P3,
                ingredients.iter().map(|s| s.to_string()).collect(),
            ),
        );
    }

    // P4 Products
    let p4_products = vec![
        (
            "broadcast_node",
            vec!["neocoms", "data_chips", "high_tech_transmitters"],
        ),
        (
            "integrity_response_drones",
            vec![
                "gel_matrix_biopaste",
                "hazmat_detection_systems",
                "planetary_vehicles",
            ],
        ),
        (
            "nano_factory",
            vec![
                "industrial_explosives",
                "ukomi_super_conductors",
                "reactive_metals",
            ],
        ),
        (
            "organic_mortar_applicators",
            vec!["condensates", "robotics", "bacteria"],
        ),
        (
            "recursive_computing_module",
            vec![
                "synthetic_synapses",
                "guidance_systems",
                "transcranial_microcontrollers",
            ],
        ),
        (
            "self_harmonizing_power_core",
            vec!["camera_drones", "nuclear_reactors", "hermetic_membranes"],
        ),
        (
            "sterile_conduit",
            vec!["smartfab_units", "vaccines", "water"],
        ),
        (
            "wetware_mainframe",
            vec![
                "supercomputers",
                "biotech_research_reports",
                "cryoprotectant_solution",
            ],
        ),
    ];

    for (name, ingredients) in p4_products {
        products.insert(
            name.to_string(),
            Product::new(
                name.to_string(),
                ProductTier::P4,
                ingredients.iter().map(|s| s.to_string()).collect(),
            ),
        );
    }

    products
}
