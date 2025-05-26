import { useRef, useEffect } from "react";
import type { ProductionPlan, ProductionStep } from "../utils/wasmService";

interface ResultDisplayProps {
  plan: ProductionPlan | null;
  isLoading: boolean;
  error: string | null;
}

export default function ResultDisplay({
  plan,
  isLoading,
  error,
}: ResultDisplayProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  // Format resource name for display
  const formatName = (name: string) => {
    return name
      .split("_")
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(" ");
  };

  // Draw production flowchart
  useEffect(() => {
    if (!plan || !canvasRef.current) return;

    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    // Reset canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // We'll implement a simple visualization in a future iteration
    // For now, we'll just have a placeholder
  }, [plan]);

  if (isLoading) {
    return (
      <div className="bg-slate-800 rounded-lg border border-slate-700 shadow-md p-6">
        <div className="flex flex-col items-center justify-center p-8">
          <div className="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500 mb-4"></div>
          <p className="text-blue-400 font-medium">
            Calculating optimal production plan...
          </p>
          <p className="text-slate-400 text-sm mt-2">This may take a moment</p>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="bg-slate-800 rounded-lg border border-slate-700 shadow-md p-6">
        <div className="p-6 text-center">
          <div className="flex items-center justify-center mb-3">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-10 w-10 text-red-500"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
          </div>
          <h3 className="text-xl font-semibold text-red-500 mb-2">Error</h3>
          <p className="text-slate-300">{error}</p>
        </div>
      </div>
    );
  }

  if (!plan || plan.plan.length === 0) {
    return (
      <div className="bg-slate-800 rounded-lg border border-slate-700 shadow-md p-6">
        <div className="p-6 text-center">
          <div className="flex items-center justify-center mb-3">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              className="h-10 w-10 text-blue-400"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
              />
            </svg>
          </div>
          <h3 className="text-xl font-semibold text-blue-400 mb-2">
            No Results Yet
          </h3>
          <p className="text-slate-300">
            Configure your characters, planets, and target product, then
            calculate a production plan.
          </p>
        </div>
      </div>
    );
  }

  // Group steps by planet
  const stepsByPlanet: Record<string, ProductionStep[]> = {};
  plan.plan.forEach((step) => {
    if (!stepsByPlanet[step.planet]) {
      stepsByPlanet[step.planet] = [];
    }
    stepsByPlanet[step.planet].push(step);
  });

  return (
    <div className="bg-slate-800 rounded-lg border border-slate-700 shadow-md p-6">
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
            d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
          />
        </svg>
        Production Plan Results
      </h2>

      <div className="mb-6">
        <canvas
          ref={canvasRef}
          width={800}
          height={400}
          className="w-full h-64 bg-slate-900 border border-slate-600 rounded-lg shadow-inner"
        />
      </div>

      {/* Planet-based view */}
      <div className="space-y-6">
        {Object.entries(stepsByPlanet).map(([planetId, steps]) => (
          <div
            key={planetId}
            className="border border-slate-600 rounded-lg overflow-hidden shadow-md bg-slate-700/30"
          >
            <div className="bg-slate-700 p-3 border-b border-slate-600">
              <h3 className="font-medium text-white flex items-center">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-4 w-4 mr-2 text-blue-400"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
                Planet: {planetId} ({steps[0].type})
              </h3>
              <p className="text-sm text-slate-300 mt-1 flex items-center">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  className="h-3 w-3 mr-1 text-slate-400"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                  />
                </svg>
                Operated by: {steps[0].character}
              </p>
            </div>

            <div className="p-4 space-y-4">
              {steps.map((step, idx) => (
                <div
                  key={idx}
                  className="bg-slate-700/70 p-4 rounded-lg border border-slate-600 shadow-sm"
                >
                  <div className="font-medium text-blue-400 mb-3 flex items-center">
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
                        d="M7 12l3-3 3 3 4-4M8 21l4-4 4 4M3 4h18M4 4h16v12a1 1 0 01-1 1H5a1 1 0 01-1-1V4z"
                      />
                    </svg>
                    Producing: {formatName(step.output)}
                  </div>

                  {step.mine.length > 0 && (
                    <div className="mb-3">
                      <div className="text-sm text-slate-300 mb-1.5 flex items-center">
                        <svg
                          xmlns="http://www.w3.org/2000/svg"
                          className="h-3 w-3 mr-1"
                          fill="none"
                          viewBox="0 0 24 24"
                          stroke="currentColor"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z"
                          />
                        </svg>
                        Mining:
                      </div>
                      <div className="flex flex-wrap gap-1.5">
                        {step.mine.map((resource, i) => (
                          <span
                            key={i}
                            className="text-xs bg-blue-900/30 text-blue-300 px-2 py-1 rounded-md border border-blue-500/20"
                          >
                            {formatName(resource)}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}

                  {step.import.length > 0 && (
                    <div>
                      <div className="text-sm text-slate-300 mb-1.5 flex items-center">
                        <svg
                          xmlns="http://www.w3.org/2000/svg"
                          className="h-3 w-3 mr-1"
                          fill="none"
                          viewBox="0 0 24 24"
                          stroke="currentColor"
                        >
                          <path
                            strokeLinecap="round"
                            strokeLinejoin="round"
                            strokeWidth={2}
                            d="M4 16v1a3 3 0 003 3h10a3 3 0 003-3v-1m-4-8l-4-4m0 0L8 8m4-4v12"
                          />
                        </svg>
                        Importing:
                      </div>
                      <div className="flex flex-wrap gap-1.5">
                        {step.import.map((resource, i) => (
                          <span
                            key={i}
                            className="text-xs bg-amber-900/30 text-amber-300 px-2 py-1 rounded-md border border-amber-500/20"
                          >
                            {formatName(resource)}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>

      {/* JSON output */}
      <div className="mt-8 pt-4 border-t border-slate-700">
        <h3 className="text-base font-semibold text-blue-400 mb-2 flex items-center">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            className="h-4 w-4 mr-1"
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"
            />
          </svg>
          Raw JSON Output
        </h3>
        <pre className="bg-slate-900 p-4 rounded-lg text-xs font-mono overflow-auto max-h-60 text-slate-300 border border-slate-700">
          {JSON.stringify(plan, null, 2)}
        </pre>
      </div>
    </div>
  );
}
