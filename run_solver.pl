% Main entry point for the PI solver
% Only import pi_solver - it will handle importing all other modules
:- use_module(pi_solver).

% Main run predicate to solve for a product and write results to output.json
run(TargetProduct) :-
    format('Solving production plan for ~w...~n', [TargetProduct]),
    write_production_plan_to_file(TargetProduct, 'planets.json', 'characters.json', 'output.json', Status),
    (Status = success ->
        format('Success! Production plan written to output.json~n')
    ;   
        format('Error generating production plan~n')
    ).

% Allow running with command-line argument
% Usage: swipl -q -l run_solver.pl -t main -- product_name
:- initialization(main, main).

main(Argv) :-
    % Check if a product name was provided as an argument
    (Argv = [ProductArg|_] ->
        atom_string(Product, ProductArg),
        run(Product)
    ;
        % Default product if none specified
        format('No product specified, using default: recursive_computing_module~n'),
        run(recursive_computing_module)
    ),
    halt.
main(_) :-
    write('Error running solver.'), nl,
    halt(1). 