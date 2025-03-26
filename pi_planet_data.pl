:- module(pi_planet_data, [
    planet_type/1,
    planet_resource/2
]).

% Planet types available in EVE Online
% Used to validate planet assignments in the solver
planet_type(barren).
planet_type(gas).
planet_type(ice).
planet_type(lava).
planet_type(oceanic).
planet_type(plasma).
planet_type(storm).
planet_type(temperate).

% P0 Resources and the planets they can be found on
% planet_resource(Name, PlanetTypes)
% Maps each raw resource to the list of planet types where it can be extracted
planet_resource(aqueous_liquids, [oceanic, temperate]).
planet_resource(autotrophs, [temperate]).
planet_resource(base_metals, [barren, lava, plasma]).
planet_resource(carbon_compounds, [gas, temperate]).
planet_resource(complex_organisms, [temperate]).
planet_resource(felsic_magma, [lava]).
planet_resource(heavy_metals, [barren, lava, plasma]).
planet_resource(ionic_solutions, [gas, storm]).
planet_resource(micro_organisms, [oceanic, temperate]).
planet_resource(noble_gas, [gas, ice]).
planet_resource(noble_metals, [barren, plasma]).
planet_resource(non_cs_crystals, [ice, plasma]).
planet_resource(planktic_colonies, [oceanic]).
planet_resource(reactive_gas, [gas, storm]).
planet_resource(suspended_plasma, [gas, plasma, storm]). 