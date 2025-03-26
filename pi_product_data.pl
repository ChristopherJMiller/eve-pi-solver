:- module(pi_product_data, [
    product_tier/1,
    product/3,
    product_tier_of/2,
    previous_tier/2
]).

:- use_module(pi_planet_data).

% Define the product tiers in planetary industry
% From P0 (raw materials) to P4 (advanced commodities)
product_tier(p0).
product_tier(p1).
product_tier(p2).
product_tier(p3).
product_tier(p4).

% Define the tier progression relationship for the production chain
% previous_tier(FromTier, ToTier) - the relationship between tiers
previous_tier(p0, p1).
previous_tier(p1, p2).
previous_tier(p2, p3).
previous_tier(p3, p4).

% P0 Products (base resources)
% product(Name, Tier, [Ingredients])
% P0 products have no ingredients as they are extracted directly
product(aqueous_liquids, p0, []).
product(autotrophs, p0, []).
product(base_metals, p0, []).
product(carbon_compounds, p0, []).
product(complex_organisms, p0, []).
product(felsic_magma, p0, []).
product(heavy_metals, p0, []).
product(ionic_solutions, p0, []).
product(micro_organisms, p0, []).
product(noble_gas, p0, []).
product(noble_metals, p0, []).
product(non_cs_crystals, p0, []).
product(planktic_colonies, p0, []).
product(reactive_gas, p0, []).
product(suspended_plasma, p0, []).

% P1 Products (basic processed materials)
% Each P1 product requires one P0 input
product(bacteria, p1, [micro_organisms]).
product(biofuels, p1, [carbon_compounds]).
product(biomass, p1, [planktic_colonies]).
product(chiral_structures, p1, [non_cs_crystals]).
product(electrolytes, p1, [ionic_solutions]).
product(industrial_fibers, p1, [autotrophs]).
product(oxidizing_compound, p1, [reactive_gas]).
product(oxygen, p1, [noble_gas]).
product(plasmoids, p1, [suspended_plasma]).
product(precious_metals, p1, [noble_metals]).
product(proteins, p1, [complex_organisms]).
product(reactive_metals, p1, [base_metals]).
product(silicon, p1, [felsic_magma]).
product(toxic_metals, p1, [heavy_metals]).
product(water, p1, [aqueous_liquids]).

% P2 Products (refined commodities)
% Each P2 product requires two P1 inputs
product(biocells, p2, [precious_metals, biofuels]).
product(construction_blocks, p2, [toxic_metals, reactive_metals]).
product(consumer_electronics, p2, [chiral_structures, toxic_metals]).
product(coolant, p2, [water, electrolytes]).
product(enriched_uranium, p2, [toxic_metals, precious_metals]).
product(fertilizer, p2, [proteins, bacteria]).
product(gen_enhanced_livestock, p2, [biomass, proteins]).
product(livestock, p2, [biofuels, proteins]).
product(mechanical_parts, p2, [precious_metals, reactive_metals]).
product(microfiber_shielding, p2, [silicon, industrial_fibers]).
product(miniature_electronics, p2, [silicon, chiral_structures]).
product(nanites, p2, [reactive_metals, bacteria]).
product(oxides, p2, [oxygen, oxidizing_compound]).
product(polyaramids, p2, [industrial_fibers, oxidizing_compound]).
product(polytextiles, p2, [industrial_fibers, biofuels]).
product(rocket_fuel, p2, [electrolytes, plasmoids]).
product(silicate_glass, p2, [silicon, oxidizing_compound]).
product(superconductors, p2, [water, plasmoids]).
product(supertensile_plastics, p2, [biomass, oxygen]).
product(synthetic_oil, p2, [oxygen, electrolytes]).
product(test_cultures, p2, [water, bacteria]).
product(transmitter, p2, [chiral_structures, plasmoids]).
product(viral_agent, p2, [biomass, bacteria]).
product(water_cooled_cpu, p2, [water, reactive_metals]).

% P3 Products (specialized commodities)
% Most P3 products require multiple P2 inputs
product(biotech_research_reports, p3, [nanites, livestock, construction_blocks]).
product(camera_drones, p3, [silicate_glass, rocket_fuel]).
product(condensates, p3, [oxides, coolant]).
product(cryoprotectant_solution, p3, [test_cultures, synthetic_oil, fertilizer]).
product(data_chips, p3, [supertensile_plastics, microfiber_shielding]).
product(gel_matrix_biopaste, p3, [oxides, biocells, superconductors]).
product(guidance_systems, p3, [water_cooled_cpu, transmitter]).
product(hazmat_detection_systems, p3, [polytextiles, viral_agent, transmitter]).
product(hermetic_membranes, p3, [polyaramids, gen_enhanced_livestock]).
product(high_tech_transmitters, p3, [polyaramids, transmitter]).
product(industrial_explosives, p3, [fertilizer, polytextiles]).
product(neocoms, p3, [biocells, silicate_glass]).
product(nuclear_reactors, p3, [microfiber_shielding, enriched_uranium]).
product(planetary_vehicles, p3, [supertensile_plastics, mechanical_parts, miniature_electronics]).
product(robotics, p3, [mechanical_parts, consumer_electronics]).
product(smartfab_units, p3, [construction_blocks, miniature_electronics]).
product(supercomputers, p3, [water_cooled_cpu, coolant, consumer_electronics]).
product(synthetic_synapses, p3, [supertensile_plastics, test_cultures]).
product(transcranial_microcontrollers, p3, [biocells, nanites]).
product(ukomi_super_conductors, p3, [synthetic_oil, superconductors]).
product(vaccines, p3, [livestock, viral_agent]).

% P4 Products (advanced commodities)
% Each P4 product requires three P3 inputs
product(broadcast_node, p4, [neocoms, data_chips, high_tech_transmitters]).
product(integrity_response_drones, p4, [gel_matrix_biopaste, hazmat_detection_systems, planetary_vehicles]).
product(nano_factory, p4, [industrial_explosives, ukomi_super_conductors, reactive_metals]).
product(organic_mortar_applicators, p4, [condensates, robotics, bacteria]).
product(recursive_computing_module, p4, [synthetic_synapses, guidance_systems, transcranial_microcontrollers]).
product(self_harmonizing_power_core, p4, [camera_drones, nuclear_reactors, hermetic_membranes]).
product(sterile_conduit, p4, [smartfab_units, vaccines, water]).
product(wetware_mainframe, p4, [supercomputers, biotech_research_reports, cryoprotectant_solution]).

% Get the tier of a product
% Identifies which production tier a product belongs to
product_tier_of(Product, p0) :-
    resource(Product, _).
product_tier_of(Product, Tier) :-
    product(Product, Tier, _). 