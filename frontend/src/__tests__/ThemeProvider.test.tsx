import React from "react";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { ThemeProvider, useTheme } from "../contexts/ThemeContext";

// Mock localStorage
const localStorageMock = {
  getItem: jest.fn(),
  setItem: jest.fn(),
  removeItem: jest.fn(),
  clear: jest.fn(),
};
Object.defineProperty(window, "localStorage", {
  value: localStorageMock,
});

// Mock matchMedia for system preference detection
Object.defineProperty(window, "matchMedia", {
  writable: true,
  value: jest.fn().mockImplementation((query) => ({
    matches: query === "(prefers-color-scheme: dark)",
    media: query,
    onchange: null,
    addListener: jest.fn(),
    removeListener: jest.fn(),
    addEventListener: jest.fn(),
    removeEventListener: jest.fn(),
    dispatchEvent: jest.fn(),
  })),
});

// Test component to access theme context
const TestComponent = () => {
  const { theme, toggleTheme, setTheme } = useTheme();
  return (
    <div>
      <div data-testid="current-theme">{theme}</div>
      <button data-testid="toggle-theme" onClick={toggleTheme}>
        Toggle Theme
      </button>
      <button data-testid="set-light" onClick={() => setTheme("light")}>
        Set Light
      </button>
      <button data-testid="set-dark" onClick={() => setTheme("dark")}>
        Set Dark
      </button>
      <button data-testid="set-system" onClick={() => setTheme("system")}>
        Set System
      </button>
    </div>
  );
};

describe("ThemeProvider", () => {
  beforeEach(() => {
    localStorageMock.getItem.mockClear();
    localStorageMock.setItem.mockClear();
  });

  it("should initialize with system preference when no saved theme", () => {
    localStorageMock.getItem.mockReturnValue(null);

    render(
      <ThemeProvider>
        <TestComponent />
      </ThemeProvider>
    );

    expect(screen.getByTestId("current-theme")).toHaveTextContent("system");
  });

  it("should initialize with saved theme from localStorage", () => {
    localStorageMock.getItem.mockReturnValue("light");

    render(
      <ThemeProvider>
        <TestComponent />
      </ThemeProvider>
    );

    expect(screen.getByTestId("current-theme")).toHaveTextContent("light");
    expect(localStorageMock.getItem).toHaveBeenCalledWith(
      "energy-trading-theme"
    );
  });

  it("should toggle between light and dark themes", async () => {
    localStorageMock.getItem.mockReturnValue("light");

    render(
      <ThemeProvider>
        <TestComponent />
      </ThemeProvider>
    );

    expect(screen.getByTestId("current-theme")).toHaveTextContent("light");

    fireEvent.click(screen.getByTestId("toggle-theme"));

    await waitFor(() => {
      expect(screen.getByTestId("current-theme")).toHaveTextContent("dark");
    });

    expect(localStorageMock.setItem).toHaveBeenCalledWith(
      "energy-trading-theme",
      "dark"
    );
  });

  it("should set specific theme", async () => {
    localStorageMock.getItem.mockReturnValue("system");

    render(
      <ThemeProvider>
        <TestComponent />
      </ThemeProvider>
    );

    fireEvent.click(screen.getByTestId("set-light"));

    await waitFor(() => {
      expect(screen.getByTestId("current-theme")).toHaveTextContent("light");
    });

    expect(localStorageMock.setItem).toHaveBeenCalledWith(
      "energy-trading-theme",
      "light"
    );
  });

  it("should handle system theme preference changes", async () => {
    localStorageMock.getItem.mockReturnValue("system");

    render(
      <ThemeProvider>
        <TestComponent />
      </ThemeProvider>
    );

    expect(screen.getByTestId("current-theme")).toHaveTextContent("system");

    // System theme should resolve to dark based on our mock
    expect(document.documentElement).toHaveClass("dark");
  });

  it("should apply correct CSS classes to document element", async () => {
    localStorageMock.getItem.mockReturnValue("dark");

    render(
      <ThemeProvider>
        <TestComponent />
      </ThemeProvider>
    );

    expect(document.documentElement).toHaveClass("dark");

    fireEvent.click(screen.getByTestId("set-light"));

    await waitFor(() => {
      expect(document.documentElement).not.toHaveClass("dark");
    });
  });
});
