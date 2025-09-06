import React from "react";
import { ThemeToggle } from "../components/ThemeToggle";
import { useTheme } from "../contexts/ThemeContext";

const ThemeTestPage: React.FC = () => {
  const { theme, resolvedTheme } = useTheme();

  return (
    <div className="min-h-screen bg-white dark:bg-gray-900 p-8">
      <div className="max-w-4xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-gray-100 mb-8">
          Theme System Test
        </h1>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-8">
          <div className="card">
            <div className="card-content">
              <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-4">
                Theme Information
              </h2>
              <div className="space-y-2">
                <p className="text-gray-600 dark:text-gray-400">
                  <span className="font-medium">Current Theme:</span> {theme}
                </p>
                <p className="text-gray-600 dark:text-gray-400">
                  <span className="font-medium">Resolved Theme:</span>{" "}
                  {resolvedTheme}
                </p>
                <p className="text-gray-600 dark:text-gray-400">
                  <span className="font-medium">System Preference:</span>{" "}
                  {typeof window !== "undefined" && window.matchMedia
                    ? window.matchMedia("(prefers-color-scheme: dark)").matches
                      ? "dark"
                      : "light"
                    : "unknown"}
                </p>
              </div>
            </div>
          </div>

          <div className="card">
            <div className="card-content">
              <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-4">
                Theme Toggle
              </h2>
              <div className="space-y-4">
                <ThemeToggle />
                <p className="text-sm text-gray-600 dark:text-gray-400">
                  Click the button above to cycle through themes: Light → Dark →
                  System → Light
                </p>
              </div>
            </div>
          </div>

          <div className="card">
            <div className="card-content">
              <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-4">
                Color Palette Test
              </h2>
              <div className="grid grid-cols-2 gap-4">
                <div className="p-4 bg-blue-100 dark:bg-blue-900 rounded">
                  <p className="text-blue-800 dark:text-blue-200">Primary</p>
                </div>
                <div className="p-4 bg-green-100 dark:bg-green-900 rounded">
                  <p className="text-green-800 dark:text-green-200">Success</p>
                </div>
                <div className="p-4 bg-yellow-100 dark:bg-yellow-900 rounded">
                  <p className="text-yellow-800 dark:text-yellow-200">
                    Warning
                  </p>
                </div>
                <div className="p-4 bg-red-100 dark:bg-red-900 rounded">
                  <p className="text-red-800 dark:text-red-200">Danger</p>
                </div>
              </div>
            </div>
          </div>

          <div className="card">
            <div className="card-content">
              <h2 className="text-xl font-semibold text-gray-900 dark:text-gray-100 mb-4">
                Component Examples
              </h2>
              <div className="space-y-4">
                <button className="btn btn-primary px-4 py-2">
                  Primary Button
                </button>
                <button className="btn btn-secondary px-4 py-2">
                  Secondary Button
                </button>
                <button className="btn btn-outline px-4 py-2">
                  Outline Button
                </button>
                <button className="btn btn-ghost px-4 py-2">
                  Ghost Button
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ThemeTestPage;
