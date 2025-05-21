import { useState, useEffect } from "react";
import { PrologService } from "../utils/prologService";

interface ProductSelectorProps {
  onSelectProduct: (product: string) => void;
  selectedProduct: string;
}

export default function ProductSelector({
  onSelectProduct,
  selectedProduct,
}: ProductSelectorProps) {
  const [productTiers, setProductTiers] = useState<string[]>([]);
  const [selectedTier, setSelectedTier] = useState<string>("");
  const [products, setProducts] = useState<string[]>([]);
  const [isLoading, setIsLoading] = useState(true);

  // Load product tiers
  useEffect(() => {
    const loadProductTiers = async () => {
      try {
        const prologService = PrologService.getInstance();
        const tiers = await prologService.getProductTiers();
        setProductTiers(tiers);
        setIsLoading(false);
      } catch (error) {
        console.error("Failed to load product tiers:", error);
        setIsLoading(false);
      }
    };

    loadProductTiers();
  }, []);

  // Load products for selected tier
  useEffect(() => {
    if (!selectedTier) return;

    const loadProducts = async () => {
      try {
        setIsLoading(true);
        const prologService = PrologService.getInstance();
        const products = await prologService.getProductsByTier(selectedTier);
        setProducts(products);
        setIsLoading(false);
      } catch (error) {
        console.error(
          `Failed to load products for tier ${selectedTier}:`,
          error
        );
        setIsLoading(false);
      }
    };

    loadProducts();
  }, [selectedTier]);

  // Format tier name for display
  const formatTierName = (tier: string) => {
    switch (tier) {
      case "p0":
        return "P0 - Raw Materials";
      case "p1":
        return "P1 - Basic Processed Materials";
      case "p2":
        return "P2 - Refined Commodities";
      case "p3":
        return "P3 - Specialized Commodities";
      case "p4":
        return "P4 - Advanced Commodities";
      default:
        return tier.toUpperCase();
    }
  };

  // Format product name for display
  const formatProductName = (product: string) => {
    return product
      .split("_")
      .map((word) => word.charAt(0).toUpperCase() + word.slice(1))
      .join(" ");
  };

  return (
    <div className="bg-slate-800 rounded-lg border border-slate-700 shadow-md p-5">
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
            d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
          />
        </svg>
        Select Target Product
      </h2>

      <div className="space-y-5">
        <div>
          <label
            htmlFor="productTier"
            className="block text-sm font-medium text-slate-300 mb-1"
          >
            Product Tier
          </label>
          <div className="relative">
            <select
              id="productTier"
              className="bg-slate-700 border border-slate-600 text-white rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full appearance-none"
              value={selectedTier}
              onChange={(e) => setSelectedTier(e.target.value)}
              disabled={isLoading || productTiers.length === 0}
            >
              <option value="">Select product tier</option>
              {productTiers.map((tier) => (
                <option key={tier} value={tier}>
                  {formatTierName(tier)}
                </option>
              ))}
            </select>
            <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-blue-400">
              <svg
                className="h-4 w-4"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M19 9l-7 7-7-7"
                />
              </svg>
            </div>
          </div>
          {isLoading && selectedTier && (
            <p className="text-xs text-blue-400 mt-1">Loading products...</p>
          )}
        </div>

        {selectedTier && (
          <div>
            <label
              htmlFor="product"
              className="block text-sm font-medium text-slate-300 mb-1"
            >
              Target Product
            </label>
            <div className="relative">
              <select
                id="product"
                className="bg-slate-700 border border-slate-600 text-white rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent w-full appearance-none"
                value={selectedProduct}
                onChange={(e) => onSelectProduct(e.target.value)}
                disabled={isLoading || products.length === 0}
              >
                <option value="">Select product</option>
                {products.map((product) => (
                  <option key={product} value={product}>
                    {formatProductName(product)}
                  </option>
                ))}
              </select>
              <div className="pointer-events-none absolute inset-y-0 right-0 flex items-center px-2 text-blue-400">
                <svg
                  className="h-4 w-4"
                  fill="none"
                  viewBox="0 0 24 24"
                  stroke="currentColor"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M19 9l-7 7-7-7"
                  />
                </svg>
              </div>
            </div>
          </div>
        )}

        {selectedProduct && (
          <div className="mt-4 p-4 bg-blue-900/20 rounded-lg border border-blue-500/30 shadow-inner">
            <div className="flex items-center justify-center">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                className="h-5 w-5 text-blue-400 mr-2"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
              <p className="text-center text-blue-300 font-medium">
                {formatProductName(selectedProduct)}
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
