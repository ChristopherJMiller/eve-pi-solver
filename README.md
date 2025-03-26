# EVE Online Planetary Industry (PI) Solver

This is a Prolog-based solver for planning Planetary Industry (PI) operations in EVE Online. The solver takes a target product (P3 or P4) and generates a production plan across multiple planets and characters.

## Overview

Planetary Industry in EVE Online is a system where players can extract resources from planets and process them into higher-tier products. These products range from P0 (raw materials) to P4 (advanced commodities).

The production chain is as follows:

- P0: Raw resources extracted directly from planets
- P1: Basic processed materials made from P0
- P2: Refined commodities made from two P1 inputs
- P3: Specialized commodities made from two P2 inputs
- P4: Advanced commodities made from three P3 inputs

Different planet types contain different resources, and each product tier requires specific building setups.

## Features

- Analyzes production requirements for P3 and P4 products
- Generates optimal planet assignments based on available planets and character skills
- Accounts for planet types and their available resources
- Calculates building requirements for different production setups
- Balances production across multiple characters
- Outputs production plans in JSON format

## Usage

### Prerequisites

Ensure you have SWI-Prolog installed. You can download it from [SWI-Prolog website](https://www.swi-prolog.org/download/stable).

### Configuration

1. Configure your available planets in `planets.json`
2. Configure your characters in `characters.json`

### Running the Solver

#### Using the shell script (recommended)

```bash
# Solve for the default product (recursive_computing_module)
./run.sh

# Solve for a specific product
./run.sh broadcast_node
```

If you have `jq` installed, the script will also show a summary of the generated plan.

#### Using Prolog directly

```bash
# Solve for the default product
swipl -q -l run_solver.pl

# Solve for a specific product
swipl -q -l run_solver.pl -t main -- broadcast_node
```

#### Available products

P4 Products:

- broadcast_node
- integrity_response_drones
- nano_factory
- organic_mortar_applicators
- recursive_computing_module
- self_harmonizing_power_core
- sterile_conduit
- wetware_mainframe

P3 Products (examples):

- synthetic_synapses
- robotics
- supercomputers
- guidance_systems

## Input Files

### planets.json

Contains information about available planets:

```json
[
  {
    "id": "planet1",
    "type": "barren",
    "resources": ["base_metals", "noble_metals"]
  },
  {
    "id": "planet2",
    "type": "temperate",
    "resources": ["aqueous_liquids", "carbon_compounds"]
  }
]
```

- `id`: A unique identifier for the planet
- `type`: The planet type (barren, gas, ice, lava, oceanic, plasma, storm, temperate)
- `resources`: Array of resources available on the planet

### characters.json

Contains information about characters:

```json
[
  {
    "name": "Character1",
    "planets": 5,
    "skills": {
      "command_center_upgrades": 4
    }
  }
]
```

- `name`: Character name
- `planets`: Maximum number of planets the character can use
- `skills`: Character skills, including command_center_upgrades level

## Output

The solver generates a production plan in `output.json` with the following structure:

```json
{
  "plan": [
    {
      "character": "Character1",
      "planet": "planet1",
      "type": "barren",
      "import": ["product1", "product2"],
      "mine": ["resource1"],
      "output": "final_product"
    }
  ]
}
```

## Codebase Structure

The solver is modular and consists of the following components:

- `pi_solver.pl`: Core solver logic for generating production plans
- `pi_factory.pl`: Factory configurations for different production scenarios
- `pi_planet_data.pl`: Planet types and resource definitions
- `pi_product_data.pl`: Product definitions and production chains
- `run_solver.pl`: Main entry point with command-line argument handling
- `run.sh`: Shell script wrapper for easy execution

## Implementation Details

The solver uses several strategies:

1. For P4 production, it identifies the required P3 inputs and plans their production
2. For P3 production, it identifies P2 inputs and plans their production
3. For P2 production, it either:
   - Finds planets that can extract both required P0 resources directly (P0->P2)
   - Finds separate planets for P1 production and a factory planet for P2 production

The solver accounts for character skills and available planet count, ensuring the production plan is feasible with the given constraints.

## License

This project is open source and available for educational purposes for EVE Online players.

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests to improve the solver.
