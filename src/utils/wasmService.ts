import init, { PiSolver as WasmPiSolver } from '../../pkg/eve_pi.js';

export interface Character {
  name: string;
  planets: number;
  skills: {
    command_center_upgrades: number;
    interplanetary_consolidation: number;
    remote_sensing?: number;
    planetary_production?: number;
    planetology?: number;
    advanced_planetology?: number;
  };
}

export interface Planet {
  id: string;
  type: string;
  resources: string[];
}

export interface ProductionStep {
  character: string;
  planet: string;
  type: string;
  output: string;
  import: string[];
  mine: string[];
}

export interface ProductionPlan {
  plan: ProductionStep[];
}

export class PiSolverService {
  private static instance: PiSolverService | null = null;
  private solver: WasmPiSolver | null = null;
  private initialized = false;
  private initializationPromise: Promise<void> | null = null;

  private constructor() {}

  static getInstance(): PiSolverService {
    if (!PiSolverService.instance) {
      PiSolverService.instance = new PiSolverService();
    }
    return PiSolverService.instance;
  }

  async initialize(): Promise<void> {
    // If already initialized, return immediately
    if (this.initialized) return;

    // If initialization is in progress, wait for it to complete
    if (this.initializationPromise) {
      return this.initializationPromise;
    }

    // Start initialization and store the promise
    this.initializationPromise = this.doInitialize();
    
    try {
      await this.initializationPromise;
    } finally {
      // Clear the promise once initialization is complete (success or failure)
      this.initializationPromise = null;
    }
  }

  private async doInitialize(): Promise<void> {
    try {
      // Initialize the WASM module
      await init();
      
      // Create a new solver instance
      this.solver = new WasmPiSolver();
      
      this.initialized = true;
      console.log('PI Solver service initialized successfully');
    } catch (error) {
      console.error('Failed to initialize PI Solver service:', error);
      throw new Error('Failed to initialize PI Solver service');
    }
  }

  async calculateProductionPlan(
    characters: Character[],
    planets: Planet[],
    targetProduct: string
  ): Promise<ProductionPlan> {
    if (!this.initialized || !this.solver) {
      throw new Error('PI Solver service not initialized');
    }

    try {
      // Convert planets to the format expected by the Rust code
      const planetsForRust = planets.map(planet => ({
        id: planet.id,
        planet_type: this.convertPlanetType(planet.type),
        resources: planet.resources
      }));

      // Pass JavaScript objects directly to WASM functions
      await this.solver.load_characters(characters);
      await this.solver.load_planets(planetsForRust);

      // Solve for the target product
      const plan = await this.solver.solve(targetProduct);

      // The plan is already a JavaScript object, no need for format_production_plan
      return {
        plan: plan.assignments.map((assignment: any) => ({
          character: assignment.character,
          planet: assignment.planet,
          type: assignment.planet_type,
          output: assignment.output,
          import: assignment.imported_inputs || [],
          mine: assignment.mined_inputs || []
        }))
      };
    } catch (error) {
      console.error('Error calculating production plan:', error);
      throw new Error(`Failed to calculate production plan: ${error}`);
    }
  }

  async getProductTiers(): Promise<string[]> {
    // Return the available product tiers
    return ['p0', 'p1', 'p2', 'p3', 'p4'];
  }

  async getProductsByTier(tier: string): Promise<string[]> {
    // This would ideally come from the WASM module, but for now we'll hardcode
    // the products based on the tier
    const products: Record<string, string[]> = {
      p0: [
        'aqueous_liquids', 'autotrophs', 'base_metals', 'carbon_compounds',
        'complex_organisms', 'felsic_magma', 'heavy_metals', 'ionic_solutions',
        'micro_organisms', 'noble_gas', 'noble_metals', 'non_cs_crystals',
        'planktic_colonies', 'reactive_gas', 'suspended_plasma'
      ],
      p1: [
        'bacteria', 'biofuels', 'biomass', 'chiral_structures', 'electrolytes',
        'industrial_fibers', 'oxidizing_compound', 'oxygen', 'plasmoids',
        'precious_metals', 'proteins', 'reactive_metals', 'silicon',
        'toxic_metals', 'water'
      ],
      p2: [
        'biocells', 'construction_blocks', 'consumer_electronics', 'coolant',
        'enriched_uranium', 'fertilizer', 'gen_enhanced_livestock', 'livestock',
        'mechanical_parts', 'microfiber_shielding', 'miniature_electronics',
        'nanites', 'oxides', 'polyaramids', 'polytextiles', 'rocket_fuel',
        'silicate_glass', 'superconductors', 'supertensile_plastics',
        'synthetic_oil', 'test_cultures', 'transmitter', 'viral_agent',
        'water_cooled_cpu'
      ],
      p3: [
        'biotech_research_reports', 'camera_drones', 'condensates',
        'cryoprotectant_solution', 'data_chips', 'gel_matrix_biopaste',
        'guidance_systems', 'hazmat_detection_systems', 'hermetic_membranes',
        'high_tech_transmitters', 'industrial_explosives', 'neocoms',
        'nuclear_reactors', 'planetary_vehicles', 'robotics',
        'smartfab_units', 'supercomputers', 'synthetic_synapses',
        'transcranial_microcontrollers', 'ukomi_superconductors'
      ],
      p4: [
        'broadcast_node', 'integrity_response_drones', 'nano_factory',
        'organic_mortar_applicators', 'recursive_computing_module',
        'self_harmonizing_power_core', 'sterile_conduits', 'wetware_mainframe'
      ]
    };

    return products[tier] || [];
  }

  async getPlanetTypes(): Promise<string[]> {
    return ['barren', 'gas', 'ice', 'lava', 'oceanic', 'plasma', 'storm', 'temperate'];
  }

  private convertPlanetType(type: string): string {
    // Convert from frontend format to Rust enum format
    const typeMap: Record<string, string> = {
      'barren': 'Barren',
      'gas': 'Gas',
      'ice': 'Ice',
      'lava': 'Lava',
      'oceanic': 'Oceanic',
      'plasma': 'Plasma',
      'storm': 'Storm',
      'temperate': 'Temperate'
    };
    return typeMap[type] || type;
  }
}

// Export with a clear name that doesn't conflict
export const PiSolverClient = PiSolverService; 