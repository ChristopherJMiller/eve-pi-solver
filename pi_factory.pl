:- module(pi_factory, [
    factory_planet/6
]).

:- use_module(pi_product_data).
:- use_module(pi_planet_data).

% Create a list of products at a specific tier
% Used to validate product tiers in manufacturing chains
tiered_product_list(Tier, []).
tiered_product_list(Tier, [Product|Rest]) :-
    \+ member(Product, Rest),
    product(Product, Tier, _),
    tiered_product_list(Tier, Rest).

% Verify all outputs can be manufactured from available inputs
% Ensures the manufacturing chain is valid
manufacturing_list(_, [], AllInputs).
manufacturing_list(Tier, [Output|OutputRest], AllInputs) :-
    product(Output, Tier, InputList),
    subset(InputList, AllInputs),
    manufacturing_list(_, OutputRest, AllInputs).

% Products that require special mining (P4 products requiring P0 resources)
requires_p4_mined(nano_factory).
requires_p4_mined(organic_mortar_applicators).
requires_p4_mined(sterile_conduit).

% Define factory configurations for P4 production without mining requirements
% factory_type(TierStart, TierEnd, [Imported Inputs], [Mined Inputs], [Outputs])
factory_type(p2, p4, ImportedInputs, [], Output) :-
    \+ requires_p4_mined(Output),
    product(Output, p4, P3Manufactured),
    manufacturing_list(p3, P3Manufactured, ImportedInputs).

% Define factory configurations for P4 production with mining requirements
% Handles special cases where P4 products need direct access to P0 resources
factory_type(p2, p4, ImportedInputsMiningExcluded, MinedInput, Output) :-
    requires_p4_mined(Output),
    product(Output, p4, Manfacturing),
    manufacturing_list(p3, Manfacturing, ImportedInputs),
    !,
    member(P1Product, Manfacturing),
    product(P1Product, p1, [MinedInputItem|_]),
    length(MinedInput, 1),
    MinedInput = [MinedInputItem],
    product(MinedInputItem, p0, _),
    !,
    delete(ImportedInputs, MinedInputItem, ImportedInputsMiningExcluded).

% Define factory configuration for P0 to P2 direct production
% Used when a single planet can extract and process up to P2
factory_type(p0, p2, [], MinedInputs, Output) :-
    product(Output, p2, P1Manufactured),
    !,
    factory_type(p0, p1, [], MinedInputs, P1Manufactured),
    tiered_product_list(p0, MinedInputs).

% Define factory configuration for P1 to P2 production (factory planet)
% Base case: no outputs
factory_type(p1, p2, _, [], []).
% Recursive case: process each P2 output
factory_type(p1, p2, Imports, [], [Output|OutputRest]) :-
    product(Output, p2, P1Subset),
    subset(P1Subset, Imports),
    !,
    factory_type(p1, p2, Imports, [], OutputRest).

% Define factory configuration for P0 to P1 direct production
% Matches each P0 input with corresponding P1 output
factory_type(p0, p1, [], [MinedInput|MinedInputsRest], [Output|OutputsRest]) :-
    length([MinedInput|MinedInputsRest], InputsLength),
    length([Output|OutputsRest], OutputsLength),
    InputsLength =:= OutputsLength,
    !,
    product(MinedInput, p0, _),
    product(Output, p1, [MinedInput]),
    factory_type(p0, p1, [], MinedInputsRest, OutputsRest).

% Base case for factory types
factory_type(_, _, [], [], []).

% Verify a planet can extract all required mined inputs
% Used to match planet types with resource requirements
valid_planet_for_mining(_, []).
valid_planet_for_mining(Planet, [MinedInput|RestMinedInputs]) :-
    product(MinedInput, p0, _),
    planet_resource(MinedInput, PlanetTypes),
    member(Planet, PlanetTypes),
    planet_type(Planet),
    valid_planet_for_mining(Planet, RestMinedInputs).

% Main predicate to determine if a planet can support a specific factory setup
% Combines factory type configuration with planet resource validation
factory_planet(Planet, StartTier, EndTier, ImportedInputs, MinedInputs, Output) :-
    factory_type(StartTier, EndTier, ImportedInputs, MinedInputs, Output),
    tiered_product_list(p0, MinedInputs),
    valid_planet_for_mining(Planet, MinedInputs).
