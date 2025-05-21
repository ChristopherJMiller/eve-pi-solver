import { useState } from "react";
import type { Character } from "../utils/prologService";

interface CharacterInputProps {
  onAddCharacter: (character: Character) => void;
  characters: Character[];
}

export default function CharacterInput({
  onAddCharacter,
  characters,
}: CharacterInputProps) {
  const [name, setName] = useState("");
  const [planets, setPlanets] = useState(5);
  const [commandCenterUpgrades, setCommandCenterUpgrades] = useState(5);
  const [interplanetaryConsolidation, setInterplanetaryConsolidation] =
    useState(5);
  const [remoteSensing, setRemoteSensing] = useState(5);
  const [planetaryProduction, setPlanetaryProduction] = useState(5);
  const [planetology, setPlanetology] = useState(5);
  const [advancedPlanetology, setAdvancedPlanetology] = useState(5);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    const character: Character = {
      name,
      planets,
      skills: {
        command_center_upgrades: commandCenterUpgrades,
        interplanetary_consolidation: interplanetaryConsolidation,
        remote_sensing: remoteSensing,
        planetary_production: planetaryProduction,
        planetology: planetology,
        advanced_planetology: advancedPlanetology,
      },
    };

    onAddCharacter(character);
    setName("");
  };

  return (
    <div className="bg-slate-800 rounded-lg border border-slate-700 shadow-md p-5">
      <h2 className="text-xl font-semibold text-blue-400 mb-4">
        Add Character
      </h2>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label
            htmlFor="name"
            className="block text-sm font-medium text-slate-300 mb-1"
          >
            Character Name
          </label>
          <input
            id="name"
            type="text"
            className="bg-slate-700 border border-slate-600 text-white rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full"
            value={name}
            onChange={(e) => setName(e.target.value)}
            required
          />
        </div>

        <div>
          <label
            htmlFor="planets"
            className="block text-sm font-medium text-slate-300 mb-1"
          >
            Number of Planets
          </label>
          <input
            id="planets"
            type="number"
            className="bg-slate-700 border border-slate-600 text-white rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full"
            value={planets}
            onChange={(e) => setPlanets(Number(e.target.value))}
            min={1}
            max={6}
            required
          />
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label
              htmlFor="commandCenterUpgrades"
              className="block text-sm font-medium text-slate-300 mb-1"
            >
              Command Center Upgrades
            </label>
            <input
              id="commandCenterUpgrades"
              type="number"
              className="bg-slate-700 border border-slate-600 text-white rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full"
              value={commandCenterUpgrades}
              onChange={(e) => setCommandCenterUpgrades(Number(e.target.value))}
              min={0}
              max={5}
              required
            />
          </div>

          <div>
            <label
              htmlFor="interplanetaryConsolidation"
              className="block text-sm font-medium text-slate-300 mb-1"
            >
              Interplanetary Consolidation
            </label>
            <input
              id="interplanetaryConsolidation"
              type="number"
              className="bg-slate-700 border border-slate-600 text-white rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full"
              value={interplanetaryConsolidation}
              onChange={(e) =>
                setInterplanetaryConsolidation(Number(e.target.value))
              }
              min={0}
              max={5}
              required
            />
          </div>

          <div>
            <label
              htmlFor="remoteSensing"
              className="block text-sm font-medium text-slate-300 mb-1"
            >
              Remote Sensing
            </label>
            <input
              id="remoteSensing"
              type="number"
              className="bg-slate-700 border border-slate-600 text-white rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full"
              value={remoteSensing}
              onChange={(e) => setRemoteSensing(Number(e.target.value))}
              min={0}
              max={5}
              required
            />
          </div>

          <div>
            <label
              htmlFor="planetaryProduction"
              className="block text-sm font-medium text-slate-300 mb-1"
            >
              Planetary Production
            </label>
            <input
              id="planetaryProduction"
              type="number"
              className="bg-slate-700 border border-slate-600 text-white rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full"
              value={planetaryProduction}
              onChange={(e) => setPlanetaryProduction(Number(e.target.value))}
              min={0}
              max={5}
              required
            />
          </div>

          <div>
            <label
              htmlFor="planetology"
              className="block text-sm font-medium text-slate-300 mb-1"
            >
              Planetology
            </label>
            <input
              id="planetology"
              type="number"
              className="bg-slate-700 border border-slate-600 text-white rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full"
              value={planetology}
              onChange={(e) => setPlanetology(Number(e.target.value))}
              min={0}
              max={5}
              required
            />
          </div>

          <div>
            <label
              htmlFor="advancedPlanetology"
              className="block text-sm font-medium text-slate-300 mb-1"
            >
              Advanced Planetology
            </label>
            <input
              id="advancedPlanetology"
              type="number"
              className="bg-slate-700 border border-slate-600 text-white rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full"
              value={advancedPlanetology}
              onChange={(e) => setAdvancedPlanetology(Number(e.target.value))}
              min={0}
              max={5}
              required
            />
          </div>
        </div>

        <button
          type="submit"
          className="bg-blue-600 hover:bg-blue-500 text-white font-medium py-2 px-4 rounded-md transition-colors duration-200 w-full"
        >
          Add Character
        </button>
      </form>

      {characters.length > 0 && (
        <div className="mt-6">
          <h3 className="text-lg font-semibold text-blue-400 mb-2">
            Characters
          </h3>
          <div className="space-y-2">
            {characters.map((char, index) => (
              <div
                key={index}
                className="bg-slate-700 p-3 rounded-md flex justify-between items-center"
              >
                <div>
                  <p className="font-medium text-white">{char.name}</p>
                  <p className="text-sm text-slate-300">
                    Planets: {char.planets}
                  </p>
                </div>
                <div className="text-xs text-slate-400">
                  <p>CCU: {char.skills.command_center_upgrades}</p>
                  <p>IC: {char.skills.interplanetary_consolidation}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}
