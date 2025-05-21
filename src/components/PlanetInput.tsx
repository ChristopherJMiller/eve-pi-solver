import { useState, useEffect } from "react";
import type { Planet } from "../utils/prologService";
import { PrologService } from "../utils/prologService";

interface PlanetInputProps {
  onAddPlanet: (planet: Planet) => void;
  planets: Planet[];
}

export default function PlanetInput({
  onAddPlanet,
  planets,
}: PlanetInputProps) {
  const [id, setId] = useState("");
  const [type, setType] = useState("");
  const [resources, setResources] = useState<string[]>([]);
  const [availableTypes, setAvailableTypes] = useState<string[]>([]);
  const [selectedResources, setSelectedResources] = useState<string[]>([]);
  const [availableResources, setAvailableResources] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  // Load planet types from Prolog
  useEffect(() => {
    const loadPlanetTypes = async () => {
      try {
        const prologService = PrologService.getInstance();
        const types = await prologService.getPlanetTypes();
        setAvailableTypes(types);
        setIsLoading(false);
      } catch (error) {
        console.error("Failed to load planet types:", error);
        setIsLoading(false);
      }
    };

    loadPlanetTypes();
  }, []);

  // When planet type changes, update available resources
  useEffect(() => {
    if (!type) return;

    // In a real app, we would query Prolog for this
    // For now, we'll use a simplified list based on the example planets.json
    const resourceMap: Record<string, string[]> = {
      barren: [
        "base_metals",
        "heavy_metals",
        "noble_metals",
        "chiral_structures",
      ],
      lava: ["base_metals", "felsic_magma", "heavy_metals"],
      temperate: [
        "aqueous_liquids",
        "carbon_compounds",
        "complex_organisms",
        "microorganisms",
        "autotrophs",
      ],
      gas: [
        "carbon_compounds",
        "ionic_solutions",
        "noble_gas",
        "reactive_gas",
        "suspended_plasma",
      ],
      oceanic: ["aqueous_liquids", "microorganisms", "planktic_colonies"],
      ice: ["noble_gas", "non_cs_crystals"],
      plasma: [
        "base_metals",
        "heavy_metals",
        "noble_metals",
        "non_cs_crystals",
        "suspended_plasma",
        "chiral_structures",
      ],
      storm: ["ionic_solutions", "reactive_gas", "suspended_plasma"],
    };

    setAvailableResources(resourceMap[type] || []);
    setSelectedResources([]);
  }, [type]);

  const handleResourceToggle = (resource: string) => {
    if (selectedResources.includes(resource)) {
      setSelectedResources(selectedResources.filter((r) => r !== resource));
    } else {
      setSelectedResources([...selectedResources, resource]);
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const planet: Planet = {
      id,
      type,
      resources: selectedResources,
    };

    onAddPlanet(planet);
    setId("");
    setType("");
    setSelectedResources([]);
  };

  if (isLoading) {
    return (
      <div className="bg-slate-800 rounded-lg border border-slate-700 shadow-md p-5 flex justify-center items-center h-full">
        <div className="text-slate-300">Loading planet data...</div>
      </div>
    );
  }

  return (
    <div className="bg-slate-800 rounded-lg border border-slate-700 shadow-md p-5">
      <h2 className="text-xl font-semibold text-blue-400 mb-4">Add Planet</h2>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label
            htmlFor="planetId"
            className="block text-sm font-medium text-slate-300 mb-1"
          >
            Planet ID
          </label>
          <input
            id="planetId"
            type="text"
            className="bg-slate-700 border border-slate-600 text-white rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full"
            value={id}
            onChange={(e) => setId(e.target.value)}
            required
          />
        </div>

        <div>
          <label
            htmlFor="planetType"
            className="block text-sm font-medium text-slate-300 mb-1"
          >
            Planet Type
          </label>
          <select
            id="planetType"
            className="bg-slate-700 border border-slate-600 text-white rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full"
            value={type}
            onChange={(e) => setType(e.target.value)}
            required
          >
            <option value="">Select planet type</option>
            {availableTypes.map((planetType) => (
              <option key={planetType} value={planetType}>
                {planetType.charAt(0).toUpperCase() + planetType.slice(1)}
              </option>
            ))}
          </select>
        </div>

        {type && (
          <div>
            <label className="block text-sm font-medium text-slate-300 mb-2">
              Available Resources
            </label>
            <div className="grid grid-cols-2 md:grid-cols-3 gap-2">
              {availableResources.map((resource) => (
                <div key={resource} className="flex items-center">
                  <input
                    type="checkbox"
                    id={`resource-${resource}`}
                    checked={selectedResources.includes(resource)}
                    onChange={() => handleResourceToggle(resource)}
                    className="form-checkbox h-4 w-4 text-blue-500 focus:ring-blue-500 rounded mr-2 bg-slate-700 border-slate-600"
                  />
                  <label
                    htmlFor={`resource-${resource}`}
                    className="text-sm text-slate-300"
                  >
                    {resource.replace(/_/g, " ")}
                  </label>
                </div>
              ))}
            </div>
          </div>
        )}

        <button
          type="submit"
          className="bg-blue-600 hover:bg-blue-500 text-white font-medium py-2 px-4 rounded-md transition-colors duration-200 w-full"
          disabled={!id || !type || selectedResources.length === 0}
        >
          Add Planet
        </button>
      </form>

      {planets.length > 0 && (
        <div className="mt-6">
          <h3 className="text-lg font-semibold text-blue-400 mb-2">Planets</h3>
          <div className="space-y-2">
            {planets.map((planet, index) => (
              <div key={index} className="bg-slate-700 p-3 rounded-md">
                <div className="flex justify-between items-center">
                  <div>
                    <p className="font-medium text-white">{planet.id}</p>
                    <p className="text-sm text-slate-300">
                      Type: {planet.type}
                    </p>
                  </div>
                </div>
                <div className="mt-2">
                  <p className="text-xs text-slate-400">Resources:</p>
                  <div className="flex flex-wrap gap-1 mt-1">
                    {planet.resources.map((resource, idx) => (
                      <span
                        key={idx}
                        className="text-xs bg-blue-900/40 text-blue-300 px-2 py-1 rounded-md"
                      >
                        {resource}
                      </span>
                    ))}
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
