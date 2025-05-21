use crate::domain::{
    planet_resource_map, requires_p4_mined, FactoryConfiguration, PlanetType, ProductTier,
};
use crate::repository::{ProductRepository, Repository};
use std::collections::HashSet;
use std::error::Error;
use std::fmt;

/// Error type for factory operations
#[derive(Debug)]
pub enum FactoryError {
    ProductNotFound(String),
    InvalidProductTier {
        product: String,
        expected: ProductTier,
        actual: ProductTier,
    },
    MissingIngredients {
        product: String,
        missing: Vec<String>,
    },
    RequiresMining(String),
    DoesNotRequireMining(String),
    NoMinableResource,
    InputOutputMismatch,
    PlanetCannotMine {
        planet_type: PlanetType,
        resource: String,
    },
}

impl fmt::Display for FactoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FactoryError::ProductNotFound(product) => write!(f, "Product not found: {}", product),
            FactoryError::InvalidProductTier {
                product,
                expected,
                actual,
            } => {
                write!(
                    f,
                    "Product {} has incorrect tier: expected {:?}, got {:?}",
                    product, expected, actual
                )
            }
            FactoryError::MissingIngredients { product, missing } => {
                write!(
                    f,
                    "Product {} is missing ingredients: {:?}",
                    product, missing
                )
            }
            FactoryError::RequiresMining(product) => {
                write!(
                    f,
                    "Product {} requires mining which is not supported",
                    product
                )
            }
            FactoryError::DoesNotRequireMining(product) => {
                write!(f, "Product {} does not require mining", product)
            }
            FactoryError::NoMinableResource => {
                write!(f, "No minable resource found in the production chain")
            }
            FactoryError::InputOutputMismatch => {
                write!(f, "Number of inputs does not match number of outputs")
            }
            FactoryError::PlanetCannotMine {
                planet_type,
                resource,
            } => {
                write!(
                    f,
                    "Planet type {:?} cannot mine resource {}",
                    planet_type, resource
                )
            }
        }
    }
}

impl Error for FactoryError {}

/// Find valid factory configurations for P4 production without mining requirements
fn factory_type_p2_to_p4_without_mining(
    repository: &dyn ProductRepository,
    output: &str,
) -> Result<FactoryConfiguration, FactoryError> {
    // Check if this is a special P4 product that requires mining
    if requires_p4_mined(output) {
        return Err(FactoryError::RequiresMining(output.to_string()));
    }

    // Get the P4 product
    let p4_product = repository
        .get_product_by_name(output)
        .ok_or_else(|| FactoryError::ProductNotFound(output.to_string()))?;

    if p4_product.tier != ProductTier::P4 {
        return Err(FactoryError::InvalidProductTier {
            product: output.to_string(),
            expected: ProductTier::P4,
            actual: p4_product.tier,
        });
    }

    // Get all P3 products needed for manufacturing
    let mut imported_inputs = HashSet::new();
    for ingredient in &p4_product.ingredients {
        let p3_product = repository
            .get_product_by_name(ingredient)
            .ok_or_else(|| FactoryError::ProductNotFound(ingredient.to_string()))?;

        if p3_product.tier != ProductTier::P3 {
            return Err(FactoryError::InvalidProductTier {
                product: ingredient.to_string(),
                expected: ProductTier::P3,
                actual: p3_product.tier,
            });
        }
        imported_inputs.insert(ingredient.as_str());
    }

    Ok(FactoryConfiguration {
        start_tier: ProductTier::P2,
        end_tier: ProductTier::P4,
        imported_inputs: imported_inputs.into_iter().map(String::from).collect(),
        mined_inputs: Vec::new(),
        outputs: vec![output.to_string()],
    })
}

/// Find valid factory configurations for P4 production with mining requirements
fn factory_type_p2_to_p4_with_mining(
    repository: &dyn ProductRepository,
    output: &str,
) -> Result<FactoryConfiguration, FactoryError> {
    // Get the P4 product
    let p4_product = repository
        .get_product_by_name(output)
        .ok_or_else(|| FactoryError::ProductNotFound(output.to_string()))?;

    // Check if this is a special P4 product that requires mining
    if !requires_p4_mined(output) {
        return Err(FactoryError::DoesNotRequireMining(output.to_string()));
    }

    if p4_product.tier != ProductTier::P4 {
        return Err(FactoryError::InvalidProductTier {
            product: output.to_string(),
            expected: ProductTier::P4,
            actual: p4_product.tier,
        });
    }

    // Get all P3 products needed for manufacturing
    let mut all_inputs = HashSet::new();
    for ingredient in &p4_product.ingredients {
        match repository.get_product_by_name(ingredient) {
            Some(p3_product) => {
                if p3_product.tier != ProductTier::P3 {
                    continue;
                }
                all_inputs.insert(ingredient.clone());

                // Also add all P1/P2 ingredients recursively
                for p3_ingredient in &p3_product.ingredients {
                    all_inputs.insert(p3_ingredient.clone());

                    if let Some(p2_product) = repository.get_product_by_name(p3_ingredient) {
                        for p2_ingredient in &p2_product.ingredients {
                            all_inputs.insert(p2_ingredient.clone());

                            if let Some(p1_product) = repository.get_product_by_name(p2_ingredient)
                            {
                                for p1_ingredient in &p1_product.ingredients {
                                    all_inputs.insert(p1_ingredient.clone());
                                }
                            }
                        }
                    }
                }
            }
            None => return Err(FactoryError::ProductNotFound(ingredient.to_string())),
        }
    }

    // Find a P0 material that can be mined
    for input in &all_inputs {
        if let Some(product) = repository.get_product_by_name(input) {
            if product.tier == ProductTier::P0 {
                let mined_input = input.clone();

                // Remove this from the imported inputs
                let imported_inputs: Vec<String> = all_inputs
                    .iter()
                    .filter(|x| **x != mined_input)
                    .cloned()
                    .collect();

                return Ok(FactoryConfiguration {
                    start_tier: ProductTier::P2,
                    end_tier: ProductTier::P4,
                    imported_inputs,
                    mined_inputs: vec![mined_input],
                    outputs: vec![output.to_string()],
                });
            }
        }
    }

    Err(FactoryError::NoMinableResource)
}

/// Find valid factory configurations for P0 to P2 direct production
fn factory_type_p0_to_p2(
    repository: &dyn ProductRepository,
    output: &str,
) -> Result<FactoryConfiguration, FactoryError> {
    // Get the P2 product
    let p2_product = repository
        .get_product_by_name(output)
        .ok_or_else(|| FactoryError::ProductNotFound(output.to_string()))?;

    if p2_product.tier != ProductTier::P2 {
        return Err(FactoryError::InvalidProductTier {
            product: output.to_string(),
            expected: ProductTier::P2,
            actual: p2_product.tier,
        });
    }

    // Get the P1 ingredients
    let mut p1_ingredients = Vec::new();
    for ingredient in &p2_product.ingredients {
        let p1_product = repository
            .get_product_by_name(ingredient)
            .ok_or_else(|| FactoryError::ProductNotFound(ingredient.to_string()))?;

        if p1_product.tier != ProductTier::P1 {
            return Err(FactoryError::InvalidProductTier {
                product: ingredient.to_string(),
                expected: ProductTier::P1,
                actual: p1_product.tier,
            });
        }
        p1_ingredients.push(p1_product);
    }

    // Get the P0 ingredients
    let mut mined_inputs = Vec::new();
    for p1_product in &p1_ingredients {
        for ingredient in &p1_product.ingredients {
            let p0_product = repository
                .get_product_by_name(ingredient)
                .ok_or_else(|| FactoryError::ProductNotFound(ingredient.to_string()))?;

            if p0_product.tier != ProductTier::P0 {
                return Err(FactoryError::InvalidProductTier {
                    product: ingredient.to_string(),
                    expected: ProductTier::P0,
                    actual: p0_product.tier,
                });
            }
            mined_inputs.push(ingredient.clone());
        }
    }

    Ok(FactoryConfiguration {
        start_tier: ProductTier::P0,
        end_tier: ProductTier::P2,
        imported_inputs: Vec::new(),
        mined_inputs,
        outputs: vec![output.to_string()],
    })
}

/// Find valid factory configurations for P1 to P2 production
fn factory_type_p1_to_p2(
    repository: &dyn ProductRepository,
    imports: &[&str],
    outputs: &[&str],
) -> Result<FactoryConfiguration, FactoryError> {
    let imports_set: HashSet<&str> = imports.iter().copied().collect();

    // Verify all outputs are P2 products
    for output in outputs {
        let product = repository
            .get_product_by_name(output)
            .ok_or_else(|| FactoryError::ProductNotFound((*output).to_string()))?;

        if product.tier != ProductTier::P2 {
            return Err(FactoryError::InvalidProductTier {
                product: (*output).to_string(),
                expected: ProductTier::P2,
                actual: product.tier,
            });
        }

        // Check that all ingredients for this product are available
        let ingredients_set: HashSet<&str> =
            product.ingredients.iter().map(|s| s.as_str()).collect();

        if !ingredients_set.is_subset(&imports_set) {
            let missing: Vec<String> = ingredients_set
                .difference(&imports_set)
                .map(|&s| s.to_string())
                .collect();

            return Err(FactoryError::MissingIngredients {
                product: (*output).to_string(),
                missing,
            });
        }
    }

    Ok(FactoryConfiguration {
        start_tier: ProductTier::P1,
        end_tier: ProductTier::P2,
        imported_inputs: imports.iter().map(|&s| s.to_string()).collect(),
        mined_inputs: Vec::new(),
        outputs: outputs.iter().map(|&s| s.to_string()).collect(),
    })
}

/// Find valid factory configurations for P0 to P1 direct production
fn factory_type_p0_to_p1(
    repository: &dyn ProductRepository,
    mined_inputs: &[&str],
    outputs: &[&str],
) -> Result<FactoryConfiguration, FactoryError> {
    // Check that number of inputs matches number of outputs
    if mined_inputs.len() != outputs.len() {
        return Err(FactoryError::InputOutputMismatch);
    }

    // Verify each P0 input and P1 output pair
    for (i, mined_input) in mined_inputs.iter().enumerate() {
        let p0_product = repository
            .get_product_by_name(mined_input)
            .ok_or_else(|| FactoryError::ProductNotFound((*mined_input).to_string()))?;

        if p0_product.tier != ProductTier::P0 {
            return Err(FactoryError::InvalidProductTier {
                product: (*mined_input).to_string(),
                expected: ProductTier::P0,
                actual: p0_product.tier,
            });
        }

        let p1_product = repository
            .get_product_by_name(outputs[i])
            .ok_or_else(|| FactoryError::ProductNotFound(outputs[i].to_string()))?;

        if p1_product.tier != ProductTier::P1 {
            return Err(FactoryError::InvalidProductTier {
                product: outputs[i].to_string(),
                expected: ProductTier::P1,
                actual: p1_product.tier,
            });
        }

        // Check that this P1 product requires this P0 input
        if p1_product.ingredients.len() != 1 || p1_product.ingredients[0] != *mined_input {
            return Err(FactoryError::MissingIngredients {
                product: outputs[i].to_string(),
                missing: vec![(*mined_input).to_string()],
            });
        }
    }

    Ok(FactoryConfiguration {
        start_tier: ProductTier::P0,
        end_tier: ProductTier::P1,
        imported_inputs: Vec::new(),
        mined_inputs: mined_inputs.iter().map(|&s| s.to_string()).collect(),
        outputs: outputs.iter().map(|&s| s.to_string()).collect(),
    })
}

/// Check if a planet can support mining specific resources
fn valid_planet_for_mining(
    planet_type: PlanetType,
    mined_inputs: &[&str],
) -> Result<(), FactoryError> {
    let resource_map = planet_resource_map();

    for input in mined_inputs {
        if let Some(valid_planet_types) = resource_map.get(input) {
            if !valid_planet_types.contains(&planet_type) {
                return Err(FactoryError::PlanetCannotMine {
                    planet_type,
                    resource: (*input).to_string(),
                });
            }
        } else {
            return Err(FactoryError::ProductNotFound((*input).to_string()));
        }
    }

    Ok(())
}

/// Find valid factory configurations for a specific planet type and target product
pub fn find_valid_factory_configurations(
    repository: &dyn Repository,
    planet_type: PlanetType,
    target_product: &str,
) -> Vec<FactoryConfiguration> {
    let mut configurations = Vec::new();

    // Try P4 production without mining
    match factory_type_p2_to_p4_without_mining(repository, target_product) {
        Ok(config) => configurations.push(config),
        Err(_) => {} // Silently ignore errors, just means this type isn't valid
    }

    // Try P4 production with mining
    match factory_type_p2_to_p4_with_mining(repository, target_product) {
        Ok(config) => {
            // Check if this planet type supports the required mining
            let mined_inputs: Vec<&str> = config.mined_inputs.iter().map(|s| s.as_str()).collect();
            if valid_planet_for_mining(planet_type, &mined_inputs).is_ok() {
                configurations.push(config);
            }
        }
        Err(_) => {} // Silently ignore errors, just means this type isn't valid
    }

    // Try P0 to P2 direct production
    match factory_type_p0_to_p2(repository, target_product) {
        Ok(config) => {
            // Check if this planet type supports the required mining
            let mined_inputs: Vec<&str> = config.mined_inputs.iter().map(|s| s.as_str()).collect();
            if valid_planet_for_mining(planet_type, &mined_inputs).is_ok() {
                configurations.push(config);
            }
        }
        Err(_) => {} // Silently ignore errors, just means this type isn't valid
    }

    // Try P1 to P2 production if target is a P2 product
    if let Some(product) = repository.get_product_by_name(target_product) {
        if product.tier == ProductTier::P2 {
            // Get P1 ingredients for this P2 product
            let p1_ingredients: Vec<&str> =
                product.ingredients.iter().map(|s| s.as_str()).collect();

            // Try importing all P1 ingredients to produce this P2 product
            match factory_type_p1_to_p2(repository, &p1_ingredients, &[target_product]) {
                Ok(config) => configurations.push(config),
                Err(_) => {} // Silently ignore errors
            }
        }

        // Try P0 to P1 production if target is a P1 product
        if product.tier == ProductTier::P1 && product.ingredients.len() == 1 {
            // Get the P0 ingredient for this P1 product
            let p0_ingredient = product.ingredients[0].as_str();

            // Verify this is a P0 product
            if let Some(p0_product) = repository.get_product_by_name(p0_ingredient) {
                if p0_product.tier == ProductTier::P0 {
                    // Check if planet supports mining this resource
                    if valid_planet_for_mining(planet_type, &[p0_ingredient]).is_ok() {
                        match factory_type_p0_to_p1(repository, &[p0_ingredient], &[target_product])
                        {
                            Ok(config) => configurations.push(config),
                            Err(_) => {} // Silently ignore errors
                        }
                    }
                }
            }
        }
    }

    configurations
}

/// Determine if a planet can support a factory for a specific product
pub fn factory_planet(
    repository: &dyn Repository,
    planet_type: PlanetType,
    target_product: &str,
) -> Vec<FactoryConfiguration> {
    find_valid_factory_configurations(repository, planet_type, target_product)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{PlanetType, ProductTier};
    use crate::repository::MemoryRepository;

    // Helper function to find a P4 product that doesn't require mining
    fn find_p4_product_without_mining(repo: &MemoryRepository) -> Option<String> {
        let p4_products = repo.get_products_by_tier(ProductTier::P4);
        for product in p4_products {
            if !requires_p4_mined(&product.name) {
                return Some(product.name);
            }
        }
        None
    }

    // Helper function to find a P4 product that requires mining
    fn find_p4_product_with_mining(repo: &MemoryRepository) -> Option<String> {
        let p4_products = repo.get_products_by_tier(ProductTier::P4);
        for product in p4_products {
            if requires_p4_mined(&product.name) {
                return Some(product.name);
            }
        }
        None
    }

    // Helper function to find a valid P1 product that can be mined on a specific planet type
    fn find_valid_p1_for_planet(
        repo: &MemoryRepository,
        planet_type: PlanetType,
    ) -> Option<String> {
        let p1_products = repo.get_products_by_tier(ProductTier::P1);

        for p1 in p1_products {
            if p1.ingredients.len() != 1 {
                continue;
            }

            let p0_name = &p1.ingredients[0];
            if let Some(p0) = repo.get_product_by_name(p0_name) {
                if p0.tier == ProductTier::P0
                    && valid_planet_for_mining(planet_type, &[p0_name.as_str()]).is_ok()
                {
                    return Some(p1.name);
                }
            }
        }
        None
    }

    #[test]
    fn test_factory_type_p2_to_p4_without_mining() {
        let repo = MemoryRepository::new();

        // Get all P4 products that don't require mining
        let p4_products = repo
            .get_products_by_tier(ProductTier::P4)
            .into_iter()
            .filter(|p| !requires_p4_mined(&p.name))
            .collect::<Vec<_>>();

        if p4_products.is_empty() {
            println!("Skipping test - no P4 products without mining requirement found");
            return;
        }

        println!(
            "Testing {} P4 products without mining requirement",
            p4_products.len()
        );
        let mut success_count = 0;

        // Test each P4 product that doesn't require mining
        for p4_product in &p4_products {
            let result = factory_type_p2_to_p4_without_mining(&repo, &p4_product.name);

            match result {
                Ok(config) => {
                    success_count += 1;

                    // Verify the configuration
                    assert_eq!(config.start_tier, ProductTier::P2);
                    assert_eq!(config.end_tier, ProductTier::P4);
                    assert!(config.mined_inputs.is_empty());
                    assert_eq!(config.outputs, vec![p4_product.name.clone()]);
                    assert!(!config.imported_inputs.is_empty());
                }
                Err(err) => {
                    println!("Unexpected error for {}: {:?}", p4_product.name, err);
                }
            }
        }

        // Ensure at least one product was successfully tested
        assert!(
            success_count > 0,
            "Expected at least one successful P4 product configuration"
        );
        println!(
            "Successfully tested {}/{} P4 products without mining",
            success_count,
            p4_products.len()
        );

        // Test with P4 products that require mining (should return Err)
        let p4_products_with_mining = repo
            .get_products_by_tier(ProductTier::P4)
            .into_iter()
            .filter(|p| requires_p4_mined(&p.name))
            .collect::<Vec<_>>();

        if !p4_products_with_mining.is_empty() {
            println!(
                "Testing {} P4 products with mining requirement",
                p4_products_with_mining.len()
            );

            for p4_product in &p4_products_with_mining {
                let result = factory_type_p2_to_p4_without_mining(&repo, &p4_product.name);
                assert!(
                    result.is_err(),
                    "Expected Err for a P4 product requiring mining: {}",
                    p4_product.name
                );

                if let Err(err) = result {
                    match err {
                        FactoryError::RequiresMining(_) => {} // Expected error
                        _ => panic!(
                            "Expected RequiresMining error for {}, got {:?}",
                            p4_product.name, err
                        ),
                    }
                }
            }
        }

        // Test with non-existent product
        let result = factory_type_p2_to_p4_without_mining(&repo, "nonexistent_product");
        assert!(result.is_err());
        if let Err(err) = result {
            match err {
                FactoryError::ProductNotFound(_) => {} // Expected error
                _ => panic!("Expected ProductNotFound error, got {:?}", err),
            }
        }
    }

    #[test]
    fn test_factory_type_p2_to_p4_with_mining() {
        let repo = MemoryRepository::new();

        // Get all P4 products that require mining
        let p4_products_with_mining = repo
            .get_products_by_tier(ProductTier::P4)
            .into_iter()
            .filter(|p| requires_p4_mined(&p.name))
            .collect::<Vec<_>>();

        if p4_products_with_mining.is_empty() {
            println!("Skipping test - no P4 products with mining requirement found");
            return;
        }

        print!(
            "Testing {} P4 products with mining requirement",
            p4_products_with_mining.len()
        );
        let mut success_count = 0;

        // Test each P4 product that requires mining
        for p4_product in &p4_products_with_mining {
            let result = factory_type_p2_to_p4_with_mining(&repo, &p4_product.name);

            match result {
                Ok(config) => {
                    success_count += 1;

                    // Verify the configuration
                    assert_eq!(config.start_tier, ProductTier::P2);
                    assert_eq!(config.end_tier, ProductTier::P4);
                    assert!(!config.mined_inputs.is_empty());
                    assert_eq!(config.outputs, vec![p4_product.name.clone()]);
                }
                Err(err) => {
                    println!("Unexpected error for {}: {:?}", p4_product.name, err);
                }
            }
        }

        // Ensure at least one product was successfully tested
        assert!(
            success_count > 0,
            "Expected at least one successful P4 product configuration"
        );
        println!(
            "Successfully tested {}/{} P4 products with mining",
            success_count,
            p4_products_with_mining.len()
        );

        // Test with P4 products that don't require mining (should return Err)
        let p4_products_without_mining = repo
            .get_products_by_tier(ProductTier::P4)
            .into_iter()
            .filter(|p| !requires_p4_mined(&p.name))
            .collect::<Vec<_>>();

        if !p4_products_without_mining.is_empty() {
            println!(
                "Testing {} P4 products without mining requirement",
                p4_products_without_mining.len()
            );

            for p4_product in &p4_products_without_mining {
                let result = factory_type_p2_to_p4_with_mining(&repo, &p4_product.name);
                assert!(
                    result.is_err(),
                    "Expected Err for a P4 product not requiring mining: {}",
                    p4_product.name
                );

                if let Err(err) = result {
                    match err {
                        FactoryError::DoesNotRequireMining(_) => {} // Expected error
                        _ => panic!(
                            "Expected DoesNotRequireMining error for {}, got {:?}",
                            p4_product.name, err
                        ),
                    }
                }
            }
        }

        // Test with non-existent product
        let result = factory_type_p2_to_p4_with_mining(&repo, "nonexistent_product");
        assert!(result.is_err());
        if let Err(err) = result {
            match err {
                FactoryError::ProductNotFound(_) => {} // Expected error
                _ => panic!("Expected ProductNotFound error, got {:?}", err),
            }
        }
    }

    #[test]
    fn test_factory_type_p0_to_p2() {
        let repo = MemoryRepository::new();

        // Get all P2 products from the repository
        let p2_products = repo.get_products_by_tier(ProductTier::P2);

        if p2_products.is_empty() {
            println!("Skipping test - no P2 products found in repository");
            return;
        }

        println!(
            "Testing {} P2 products for P0 to P2 production",
            p2_products.len()
        );
        let mut success_count = 0;
        let mut invalid_count = 0;

        // Test each P2 product
        for p2_product in &p2_products {
            let result = factory_type_p0_to_p2(&repo, &p2_product.name);

            match result {
                Ok(config) => {
                    success_count += 1;

                    // Verify the configuration
                    assert_eq!(config.start_tier, ProductTier::P0);
                    assert_eq!(config.end_tier, ProductTier::P2);
                    assert!(!config.mined_inputs.is_empty());
                    assert_eq!(config.outputs, vec![p2_product.name.clone()]);
                }
                Err(err) => {
                    // Some P2 products may legitimately not be directly producible from P0
                    // So we only track this as invalid if there's an unexpected error
                    match err {
                        FactoryError::InvalidProductTier { .. } => {
                            panic!(
                                "Got unexpected InvalidProductTier for a P2 product: {}",
                                p2_product.name
                            );
                        }
                        _ => {
                            invalid_count += 1;
                            // This is normal for P2 products that can't be directly made from P0
                        }
                    }
                }
            }
        }

        println!(
            "P0 to P2 production: {} success, {} not applicable",
            success_count, invalid_count
        );

        // Some P2 products should be producible directly from P0
        // But we don't assert this as it depends on the specific product database
        if success_count == 0 {
            println!("Warning: No P2 products can be directly produced from P0");
        }

        // Test with P1 products (should return Err)
        let p1_products = repo.get_products_by_tier(ProductTier::P1);
        let test_count = p1_products.len().min(3); // Test at most 3 P1 products

        if !p1_products.is_empty() {
            println!("Testing {} P1 products (expected to fail)", test_count);

            for p1_product in p1_products.iter().take(test_count) {
                let result = factory_type_p0_to_p2(&repo, &p1_product.name);
                assert!(
                    result.is_err(),
                    "P1 product should not have P0 to P2 factory config: {}",
                    p1_product.name
                );

                if let Err(err) = result {
                    match err {
                        FactoryError::InvalidProductTier { .. } => {} // Expected error
                        _ => panic!(
                            "Expected InvalidProductTier error for {}, got {:?}",
                            p1_product.name, err
                        ),
                    }
                }
            }
        }

        // Test with non-existent product
        let result = factory_type_p0_to_p2(&repo, "nonexistent_product");
        assert!(result.is_err());
        if let Err(err) = result {
            match err {
                FactoryError::ProductNotFound(_) => {} // Expected error
                _ => panic!("Expected ProductNotFound error, got {:?}", err),
            }
        }
    }

    #[test]
    fn test_factory_type_p1_to_p2() {
        let repo = MemoryRepository::new();

        // Get all P2 products
        let p2_products = repo.get_products_by_tier(ProductTier::P2);

        if p2_products.is_empty() {
            println!("Skipping test - no P2 products found in repository");
            return;
        }

        println!(
            "Testing {} P2 products for P1 to P2 production",
            p2_products.len()
        );
        let mut success_count = 0;

        // Test each P2 product
        for p2_product in &p2_products {
            // Get P1 ingredients for this P2 product
            let p1_ingredients: Vec<&str> =
                p2_product.ingredients.iter().map(|s| s.as_str()).collect();

            // Check if all ingredients exist and are P1 products
            let all_p1 = p1_ingredients.iter().all(|name| {
                if let Some(product) = repo.get_product_by_name(name) {
                    product.tier == ProductTier::P1
                } else {
                    false
                }
            });

            if all_p1 && !p1_ingredients.is_empty() {
                let result = factory_type_p1_to_p2(&repo, &p1_ingredients, &[&p2_product.name]);

                match result {
                    Ok(config) => {
                        success_count += 1;

                        // Verify the configuration
                        assert_eq!(config.start_tier, ProductTier::P1);
                        assert_eq!(config.end_tier, ProductTier::P2);
                        assert!(config.mined_inputs.is_empty());
                        assert_eq!(config.outputs, vec![p2_product.name.clone()]);

                        // Test with missing P1 input if there are multiple ingredients
                        if p1_ingredients.len() > 1 {
                            // Try with only the first ingredient
                            let partial_ingredients = &[p1_ingredients[0]];
                            let result = factory_type_p1_to_p2(
                                &repo,
                                partial_ingredients,
                                &[&p2_product.name],
                            );

                            assert!(
                                result.is_err(),
                                "Factory should not be possible with missing ingredients for {}",
                                p2_product.name
                            );

                            if let Err(err) = result {
                                match err {
                                    FactoryError::MissingIngredients { .. } => {} // Expected error
                                    _ => panic!(
                                        "Expected MissingIngredients error for {}, got {:?}",
                                        p2_product.name, err
                                    ),
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!("Unexpected error for {}: {:?}", p2_product.name, err);
                    }
                }
            }
        }

        assert!(
            success_count > 0,
            "Expected at least one successful P1 to P2 configuration"
        );
        println!(
            "Successfully tested {} P2 products with P1 ingredients",
            success_count
        );

        // Test with wrong P2 output (trying to use ingredients from one product to make another)
        // Find two different P2 products to test this scenario
        if p2_products.len() >= 2 {
            let p2_product1 = &p2_products[0];
            let p2_product2 = &p2_products[1];

            // Get P1 ingredients for the first P2 product
            let p1_ingredients1: Vec<&str> =
                p2_product1.ingredients.iter().map(|s| s.as_str()).collect();

            // Check if all ingredients exist and are P1 products
            let all_p1 = p1_ingredients1.iter().all(|name| {
                if let Some(product) = repo.get_product_by_name(name) {
                    product.tier == ProductTier::P1
                } else {
                    false
                }
            });

            if all_p1 && !p1_ingredients1.is_empty() {
                // Try to use ingredients from product1 to make product2
                let result = factory_type_p1_to_p2(&repo, &p1_ingredients1, &[&p2_product2.name]);

                // This might work for some products if they share ingredients, so we don't assert failure
                println!(
                    "Cross-product test: Using {} ingredients to make {}: {}",
                    p2_product1.name,
                    p2_product2.name,
                    if result.is_ok() {
                        "succeeded (shared ingredients)"
                    } else {
                        "failed (expected)"
                    }
                );
            }
        }

        // Test with non-existent products
        let result = factory_type_p1_to_p2(&repo, &["nonexistent_product"], &["mechanical_parts"]);
        assert!(result.is_err());
        if let Err(err) = result {
            match err {
                FactoryError::ProductNotFound(_) => {} // Expected error
                _ => panic!("Expected ProductNotFound error, got {:?}", err),
            }
        }
    }

    #[test]
    fn test_factory_type_p0_to_p1() {
        let repo = MemoryRepository::new();

        // Get all P1 products
        let p1_products = repo.get_products_by_tier(ProductTier::P1);

        if p1_products.is_empty() {
            println!("Skipping test - no P1 products found in repository");
            return;
        }

        println!(
            "Testing {} P1 products for P0 to P1 production",
            p1_products.len()
        );
        let mut success_count = 0;
        let mut p1_with_single_p0 = 0;

        // Test each P1 product
        for p1_product in &p1_products {
            // Check if this P1 has exactly one P0 ingredient
            if p1_product.ingredients.len() == 1 {
                p1_with_single_p0 += 1;
                let p0_name = &p1_product.ingredients[0];

                // Verify this is a P0 product
                if let Some(p0_product) = repo.get_product_by_name(p0_name) {
                    if p0_product.tier == ProductTier::P0 {
                        // Test with valid P0 input and P1 output
                        let result =
                            factory_type_p0_to_p1(&repo, &[p0_name.as_str()], &[&p1_product.name]);

                        match result {
                            Ok(config) => {
                                success_count += 1;

                                // Verify the configuration
                                assert_eq!(config.start_tier, ProductTier::P0);
                                assert_eq!(config.end_tier, ProductTier::P1);
                                assert_eq!(config.mined_inputs, vec![p0_name.clone()]);
                                assert_eq!(config.outputs, vec![p1_product.name.clone()]);
                                assert!(config.imported_inputs.is_empty());
                            }
                            Err(err) => {
                                println!("Unexpected error for {}: {:?}", p1_product.name, err);
                            }
                        }
                    }
                }
            }
        }

        assert!(
            success_count > 0,
            "Expected at least one successful P0 to P1 configuration"
        );
        println!(
            "Successfully tested {}/{} P1 products with single P0 ingredient",
            success_count, p1_with_single_p0
        );

        // Test with multiple P0 inputs and P1 outputs
        // First, find at least 2 valid P1 products with single P0 ingredients
        let valid_p1_products: Vec<_> = p1_products
            .iter()
            .filter(|p| {
                p.ingredients.len() == 1
                    && repo
                        .get_product_by_name(&p.ingredients[0])
                        .map_or(false, |p0| p0.tier == ProductTier::P0)
            })
            .take(2)
            .collect();

        if valid_p1_products.len() >= 2 {
            let p1_product1 = &valid_p1_products[0];
            let p1_product2 = &valid_p1_products[1];
            let p0_name1 = &p1_product1.ingredients[0];
            let p0_name2 = &p1_product2.ingredients[0];

            // Test with multiple P0 inputs and P1 outputs
            let result = factory_type_p0_to_p1(
                &repo,
                &[p0_name1.as_str(), p0_name2.as_str()],
                &[&p1_product1.name, &p1_product2.name],
            );

            assert!(
                result.is_ok(),
                "Factory configuration should work for multiple inputs/outputs"
            );

            if let Ok(config) = result {
                assert_eq!(
                    config.mined_inputs,
                    vec![p0_name1.clone(), p0_name2.clone()]
                );
                assert_eq!(
                    config.outputs,
                    vec![p1_product1.name.clone(), p1_product2.name.clone()]
                );
            }

            // Test with mismatched input and output counts
            let result = factory_type_p0_to_p1(
                &repo,
                &[p0_name1.as_str(), p0_name2.as_str()],
                &[&p1_product1.name],
            );

            assert!(
                result.is_err(),
                "Mismatched input and output counts should fail"
            );

            if let Err(err) = result {
                match err {
                    FactoryError::InputOutputMismatch => {} // Expected error
                    _ => panic!("Expected InputOutputMismatch error, got {:?}", err),
                }
            }
        } else {
            println!("Skipping multiple input test - couldn't find enough valid P1 products");
        }

        // Test with incorrect P1 product for P0 input
        // Find two P1 products where the second doesn't use the first's P0 input
        let valid_p1_pairs: Vec<_> = p1_products
            .iter()
            .filter(|p1| {
                p1.ingredients.len() == 1
                    && repo
                        .get_product_by_name(&p1.ingredients[0])
                        .map_or(false, |p0| p0.tier == ProductTier::P0)
            })
            .collect::<Vec<_>>()
            .windows(2)
            .filter(|pair| pair[0].ingredients[0] != pair[1].ingredients[0])
            .map(|pair| (pair[0], pair[1]))
            .take(1)
            .collect();

        if !valid_p1_pairs.is_empty() {
            let (p1a, p1b) = valid_p1_pairs[0];
            let p0_name = &p1a.ingredients[0];

            // Try to use P0 from first product to make second product
            let result = factory_type_p0_to_p1(&repo, &[p0_name.as_str()], &[&p1b.name]);

            assert!(
                result.is_err(),
                "Incorrect P1 product for P0 input should fail"
            );

            if let Err(err) = result {
                match err {
                    FactoryError::MissingIngredients { .. } => {} // Expected error
                    _ => panic!("Expected MissingIngredients error, got {:?}", err),
                }
            }
        } else {
            println!("Skipping incorrect input test - couldn't find suitable P1 product pairs");
        }

        // Test with non-existent products
        let result = factory_type_p0_to_p1(&repo, &["nonexistent_product"], &["water"]);
        assert!(result.is_err());
        if let Err(err) = result {
            match err {
                FactoryError::ProductNotFound(_) => {} // Expected error
                _ => panic!("Expected ProductNotFound error, got {:?}", err),
            }
        }
    }

    #[test]
    fn test_valid_planet_for_mining() {
        // Test with valid planet type and resource
        let result = valid_planet_for_mining(PlanetType::Oceanic, &["aqueous_liquids"]);
        assert!(result.is_ok());

        // Test with valid planet type and multiple resources
        let result = valid_planet_for_mining(PlanetType::Gas, &["reactive_gas", "noble_gas"]);
        assert!(result.is_ok());

        // Test with incompatible planet type and resource
        let result = valid_planet_for_mining(PlanetType::Barren, &["aqueous_liquids"]);
        assert!(result.is_err());

        // Test with mixed compatible and incompatible resources
        let result = valid_planet_for_mining(PlanetType::Gas, &["reactive_gas", "aqueous_liquids"]);
        assert!(result.is_err());

        // Test with non-existent resource
        let result = valid_planet_for_mining(PlanetType::Gas, &["nonexistent_resource"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_find_valid_factory_configurations() {
        let repo = MemoryRepository::new();

        // Get all planet types
        let planet_types = vec![
            PlanetType::Barren,
            PlanetType::Gas,
            PlanetType::Ice,
            PlanetType::Lava,
            PlanetType::Oceanic,
            PlanetType::Plasma,
            PlanetType::Storm,
            PlanetType::Temperate,
        ];

        // Test P1 products on each planet type
        println!("Testing P1 products for each planet type");
        let p1_products = repo.get_products_by_tier(ProductTier::P1);

        for planet_type in &planet_types {
            let mut success_count = 0;

            // For each planet type, find all P1 products that can be produced
            for p1_product in &p1_products {
                let configs =
                    find_valid_factory_configurations(&repo, *planet_type, &p1_product.name);

                if !configs.is_empty() {
                    success_count += 1;

                    // Verify at least one config has this product as output
                    assert!(
                        configs.iter().any(|c| c.outputs.contains(&p1_product.name)),
                        "Expected config to include {} as output",
                        p1_product.name
                    );
                }
            }

            println!(
                "Planet type {:?}: {} of {} P1 products can be produced",
                planet_type,
                success_count,
                p1_products.len()
            );
        }

        // Test P2 products (should have P1->P2 configurations regardless of planet type)
        println!("\nTesting P2 products on all planet types");
        let p2_products = repo.get_products_by_tier(ProductTier::P2);

        let test_planet_type = PlanetType::Barren; // Arbitrary planet type for P2 tests
        let mut p2_success = 0;

        for p2_product in &p2_products {
            let configs =
                find_valid_factory_configurations(&repo, test_planet_type, &p2_product.name);

            if !configs.is_empty() {
                p2_success += 1;
                assert!(
                    configs.iter().any(|c| c.outputs.contains(&p2_product.name)),
                    "Expected config to include {} as output",
                    p2_product.name
                );
            }
        }

        println!(
            "{} of {} P2 products can be produced on any planet type",
            p2_success,
            p2_products.len()
        );

        // Test P4 products
        println!("\nTesting P4 products on various planet types");
        let p4_products = repo.get_products_by_tier(ProductTier::P4);
        let p4_without_mining = p4_products
            .iter()
            .filter(|p| !requires_p4_mined(&p.name))
            .count();
        let p4_with_mining = p4_products
            .iter()
            .filter(|p| requires_p4_mined(&p.name))
            .count();

        println!(
            "P4 products: {} total, {} without mining, {} with mining",
            p4_products.len(),
            p4_without_mining,
            p4_with_mining
        );

        // Test P4 products without mining on different planet types
        for p4_product in p4_products.iter().filter(|p| !requires_p4_mined(&p.name)) {
            let mut found_config = false;

            // Should work on any planet type
            let configs =
                find_valid_factory_configurations(&repo, test_planet_type, &p4_product.name);
            if !configs.is_empty() {
                found_config = true;
                assert!(
                    configs.iter().any(|c| c.outputs.contains(&p4_product.name)),
                    "Expected config to include {} as output",
                    p4_product.name
                );
            }

            assert!(
                found_config,
                "Expected at least one valid configuration for P4 product without mining: {}",
                p4_product.name
            );
        }

        // Test P4 products with mining on specific planet types
        for p4_product in p4_products.iter().filter(|p| requires_p4_mined(&p.name)) {
            let mut found_any_config = false;

            // Try on each planet type - at least one should work
            for planet_type in &planet_types {
                let configs =
                    find_valid_factory_configurations(&repo, *planet_type, &p4_product.name);
                if !configs.is_empty()
                    && configs.iter().any(|c| c.outputs.contains(&p4_product.name))
                {
                    found_any_config = true;
                    println!(
                        "P4 product with mining {} can be produced on {:?}",
                        p4_product.name, planet_type
                    );
                    break;
                }
            }

            assert!(
                found_any_config,
                "Expected at least one planet type to support P4 product with mining: {}",
                p4_product.name
            );
        }
    }

    #[test]
    fn test_factory_planet() {
        let repo = MemoryRepository::new();
        
        // Get all planet types
        let planet_types = vec![
            PlanetType::Barren, 
            PlanetType::Gas, 
            PlanetType::Ice, 
            PlanetType::Lava, 
            PlanetType::Oceanic, 
            PlanetType::Plasma, 
            PlanetType::Storm, 
            PlanetType::Temperate
        ];
        
        // Get all products by tier
        let p1_products = repo.get_products_by_tier(ProductTier::P1);
        let p2_products = repo.get_products_by_tier(ProductTier::P2);
        let p4_products = repo.get_products_by_tier(ProductTier::P4);
        
        println!("Testing factory_planet function with all product combinations");
        println!("Products to test: {} P1, {} P2, {} P4", 
                 p1_products.len(), p2_products.len(), p4_products.len());
        
        // For each planet type, test a sampling of products from each tier
        for planet_type in &planet_types {
            let mut p1_success = 0;
            let mut p2_success = 0;
            let mut p4_success = 0;
            
            // Test a sample of P1 products (up to 5)
            for p1_product in p1_products.iter().take(5) {
                let configs = factory_planet(&repo, *planet_type, &p1_product.name);
                if !configs.is_empty() {
                    p1_success += 1;
                }
            }
            
            // Test a sample of P2 products (up to 5)
            for p2_product in p2_products.iter().take(5) {
                let configs = factory_planet(&repo, *planet_type, &p2_product.name);
                if !configs.is_empty() {
                    p2_success += 1;
                }
            }
            
            // Test a sample of P4 products (up to 3)
            for p4_product in p4_products.iter().take(3) {
                let configs = factory_planet(&repo, *planet_type, &p4_product.name);
                if !configs.is_empty() {
                    p4_success += 1;
                }
            }
            
            println!("Planet type {:?}: P1={}/{}, P2={}/{}, P4={}/{}", 
                     planet_type, 
                     p1_success, std::cmp::min(5, p1_products.len()),
                     p2_success, std::cmp::min(5, p2_products.len()),
                     p4_success, std::cmp::min(3, p4_products.len()));
        }
        
        // Verify factory_planet behaves the same as find_valid_factory_configurations
        // Test with a sample of products of different tiers
        let sample_products = vec![
            &p1_products[0].name,
            &p2_products[0].name,
            &p4_products[0].name,
        ];
        
        let sample_planet_type = PlanetType::Barren;
        for product_name in sample_products {
            let configs1 = factory_planet(&repo, sample_planet_type, product_name);
            let configs2 = find_valid_factory_configurations(&repo, sample_planet_type, product_name);
            
            assert_eq!(configs1.len(), configs2.len(), 
                      "factory_planet and find_valid_factory_configurations should behave the same for {}", 
                      product_name);
            
            // Also verify the configurations themselves match
            if !configs1.is_empty() {
                for (c1, c2) in configs1.iter().zip(configs2.iter()) {
                    assert_eq!(c1.start_tier, c2.start_tier);
                    assert_eq!(c1.end_tier, c2.end_tier);
                    assert_eq!(c1.outputs, c2.outputs);
                    // The order of inputs might differ, so just check lengths
                    assert_eq!(c1.imported_inputs.len(), c2.imported_inputs.len());
                    assert_eq!(c1.mined_inputs.len(), c2.mined_inputs.len());
                }
            }
        }
    }
}
