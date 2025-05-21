import { useState, useEffect } from "react";
import { PrologService } from "./utils/prologService";
import type { Character, Planet, ProductionPlan } from "./utils/prologService";
import Header from "./components/Header";
import CharacterInput from "./components/CharacterInput";
import PlanetInput from "./components/PlanetInput";
import ProductSelector from "./components/ProductSelector";
import ResultDisplay from "./components/ResultDisplay";

function App() {
  const [characters, setCharacters] = useState<Character[]>([]);
  const [planets, setPlanets] = useState<Planet[]>([]);
  const [selectedProduct, setSelectedProduct] = useState("");
  const [productionPlan, setProductionPlan] = useState<ProductionPlan | null>(
    null
  );
  const [isCalculating, setIsCalculating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [isReady, setIsReady] = useState(false);

  // Initialize Prolog service
  useEffect(() => {
    const initProlog = async () => {
      try {
        const prologService = PrologService.getInstance();
        await prologService.initialize();
        setIsReady(true);
      } catch (error) {
        console.error("Failed to initialize Prolog service:", error);
        setError(
          "Failed to initialize Prolog service. Please refresh the page."
        );
      }
    };

    initProlog();
  }, []);

  const handleAddCharacter = (character: Character) => {
    setCharacters([...characters, character]);
  };

  const handleAddPlanet = (planet: Planet) => {
    setPlanets([...planets, planet]);
  };

  const handleSelectProduct = (product: string) => {
    setSelectedProduct(product);
    // Reset production plan when product changes
    setProductionPlan(null);
  };

  const handleCalculate = async () => {
    if (!selectedProduct || characters.length === 0 || planets.length === 0) {
      setError(
        "Please add at least one character, one planet, and select a product."
      );
      return;
    }

    setError(null);
    setIsCalculating(true);

    try {
      const prologService = PrologService.getInstance();
      const plan = await prologService.calculateProductionPlan(
        characters,
        planets,
        selectedProduct
      );
      setProductionPlan(plan);
    } catch (error) {
      console.error("Error calculating production plan:", error);
      setError(
        "Failed to calculate production plan. Please check your inputs and try again."
      );
    } finally {
      setIsCalculating(false);
    }
  };

  // Sample data for testing
  const loadSampleData = () => {
    const sampleCharacters: Character[] = [
      {
        name: "character_1",
        planets: 5,
        skills: {
          command_center_upgrades: 5,
          interplanetary_consolidation: 5,
          remote_sensing: 5,
          planetary_production: 5,
          planetology: 5,
          advanced_planetology: 5,
        },
      },
      {
        name: "character_2",
        planets: 5,
        skills: {
          command_center_upgrades: 5,
          interplanetary_consolidation: 5,
          remote_sensing: 5,
          planetary_production: 5,
          planetology: 5,
          advanced_planetology: 5,
        },
      },
    ];

    const samplePlanets: Planet[] = [
      {
        id: "planet_1",
        type: "barren",
        resources: [
          "base_metals",
          "heavy_metals",
          "noble_metals",
          "chiral_structures",
        ],
      },
      {
        id: "planet_3",
        type: "temperate",
        resources: [
          "aqueous_liquids",
          "carbon_compounds",
          "complex_organisms",
          "microorganisms",
          "autotrophs",
        ],
      },
      {
        id: "planet_4",
        type: "gas",
        resources: [
          "carbon_compounds",
          "ionic_solutions",
          "noble_gas",
          "reactive_gas",
          "suspended_plasma",
        ],
      },
      {
        id: "planet_5",
        type: "oceanic",
        resources: ["aqueous_liquids", "microorganisms", "planktic_colonies"],
      },
    ];

    setCharacters(sampleCharacters);
    setPlanets(samplePlanets);
  };

  return (
    <div className="min-h-screen bg-slate-900">
      <div className="bg-slate-900/90 min-h-screen pb-12">
        <Header />

        <main className="container mx-auto px-4 max-w-7xl">
          <div className="grid grid-cols-1 lg:grid-cols-12 gap-6">
            <div className="lg:col-span-4">
              <div className="sticky top-6 space-y-6">
                <div className="bg-slate-800 rounded-lg shadow-lg border border-slate-700 p-5">
                  <h2 className="text-xl font-semibold text-blue-400 mb-4 flex items-center">
                    <svg
                      xmlns="http://www.w3.org/2000/svg"
                      className="h-5 w-5 mr-2"
                      fill="none"
                      viewBox="0 0 24 24"
                      stroke="currentColor"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M13 10V3L4 14h7v7l9-11h-7z"
                      />
                    </svg>
                    EVE Online PI Solver
                  </h2>
                  <p className="text-sm mb-4 text-slate-300">
                    This tool helps you optimize your Planetary Industry setup
                    in EVE Online. Add your characters, available planets, and
                    select a target product to generate the most efficient
                    production chain.
                  </p>

                  <div className="space-y-3">
                    <button
                      onClick={handleCalculate}
                      className={`w-full flex items-center justify-center bg-blue-600 hover:bg-blue-500 text-white font-medium py-2 px-4 rounded-md transition-colors duration-200 ${
                        !isReady ||
                        !selectedProduct ||
                        characters.length === 0 ||
                        planets.length === 0 ||
                        isCalculating
                          ? "opacity-50 cursor-not-allowed"
                          : ""
                      }`}
                      disabled={
                        !isReady ||
                        !selectedProduct ||
                        characters.length === 0 ||
                        planets.length === 0 ||
                        isCalculating
                      }
                    >
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        className="h-4 w-4 mr-2"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M9 7h6m0 10v-3m-3 3h.01M9 17h.01M9 14h.01M12 14h.01M15 11h.01M12 11h.01M9 11h.01M7 21h10a2 2 0 002-2V5a2 2 0 00-2-2H7a2 2 0 00-2 2v14a2 2 0 002 2z"
                        />
                      </svg>
                      Calculate Production Plan
                    </button>

                    <button
                      onClick={loadSampleData}
                      className="w-full flex items-center justify-center bg-slate-700 hover:bg-slate-600 text-white font-medium py-2 px-4 rounded-md transition-colors duration-200"
                    >
                      <svg
                        xmlns="http://www.w3.org/2000/svg"
                        className="h-4 w-4 mr-2"
                        fill="none"
                        viewBox="0 0 24 24"
                        stroke="currentColor"
                      >
                        <path
                          strokeLinecap="round"
                          strokeLinejoin="round"
                          strokeWidth={2}
                          d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-4l-4 4m0 0l-4-4m4 4V4"
                        />
                      </svg>
                      Load Sample Data
                    </button>
                  </div>
                </div>

                <ProductSelector
                  onSelectProduct={handleSelectProduct}
                  selectedProduct={selectedProduct}
                />
              </div>
            </div>

            <div className="lg:col-span-8">
              <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
                <CharacterInput
                  onAddCharacter={handleAddCharacter}
                  characters={characters}
                />

                <PlanetInput onAddPlanet={handleAddPlanet} planets={planets} />
              </div>

              <ResultDisplay
                plan={productionPlan}
                isLoading={isCalculating}
                error={error}
              />
            </div>
          </div>
        </main>

        <footer className="mt-16 py-6 border-t border-slate-700">
          <div className="container mx-auto px-4 text-center text-slate-400 text-sm">
            <p>
              EVE Online PI Solver — A tool for optimizing planetary industry
              production
            </p>
            <p className="mt-2">
              EVE Online is a registered trademark of CCP Games.
            </p>
          </div>
        </footer>
      </div>
    </div>
  );
}

export default App;
