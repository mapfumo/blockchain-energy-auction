import React from "react";
import { render, screen, fireEvent } from "@testing-library/react";
import { ThemeProvider } from "../contexts/ThemeContext";
import { ThemeToggle } from "../components/ThemeToggle";

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

const renderWithTheme = (component: React.ReactElement) => {
  return render(<ThemeProvider>{component}</ThemeProvider>);
};

describe("ThemeToggle", () => {
  beforeEach(() => {
    localStorageMock.getItem.mockClear();
    localStorageMock.setItem.mockClear();
  });

  it("should render with light theme icon initially", () => {
    localStorageMock.getItem.mockReturnValue("light");

    renderWithTheme(<ThemeToggle />);

    const button = screen.getByRole("button");
    expect(button).toBeInTheDocument();
    expect(button).toHaveAttribute("title", "Current theme: Light");
    expect(button).toHaveAttribute(
      "aria-label",
      "Toggle theme. Current: Light"
    );
  });

  it("should render with dark theme icon when dark theme is active", () => {
    localStorageMock.getItem.mockReturnValue("dark");

    renderWithTheme(<ThemeToggle />);

    const button = screen.getByRole("button");
    expect(button).toHaveAttribute("title", "Current theme: Dark");
    expect(button).toHaveAttribute("aria-label", "Toggle theme. Current: Dark");
  });

  it("should render with system theme icon when system theme is active", () => {
    localStorageMock.getItem.mockReturnValue("system");

    renderWithTheme(<ThemeToggle />);

    const button = screen.getByRole("button");
    expect(button).toHaveAttribute("title", "Current theme: System (dark)");
    expect(button).toHaveAttribute(
      "aria-label",
      "Toggle theme. Current: System (dark)"
    );
  });

  it("should toggle theme when clicked", () => {
    localStorageMock.getItem.mockReturnValue("light");

    renderWithTheme(<ThemeToggle />);

    const button = screen.getByRole("button");
    fireEvent.click(button);

    expect(localStorageMock.setItem).toHaveBeenCalledWith(
      "energy-trading-theme",
      "dark"
    );
  });

  it("should cycle through themes: light -> dark -> system -> light", () => {
    localStorageMock.getItem.mockReturnValue("light");

    renderWithTheme(<ThemeToggle />);

    const button = screen.getByRole("button");

    // First click: light -> dark
    fireEvent.click(button);
    expect(localStorageMock.setItem).toHaveBeenCalledWith(
      "energy-trading-theme",
      "dark"
    );

    // Second click: dark -> system
    fireEvent.click(button);
    expect(localStorageMock.setItem).toHaveBeenCalledWith(
      "energy-trading-theme",
      "system"
    );

    // Third click: system -> light
    fireEvent.click(button);
    expect(localStorageMock.setItem).toHaveBeenCalledWith(
      "energy-trading-theme",
      "light"
    );
  });

  it("should apply custom className", () => {
    localStorageMock.getItem.mockReturnValue("light");

    renderWithTheme(<ThemeToggle className="custom-class" />);

    const button = screen.getByRole("button");
    expect(button).toHaveClass("custom-class");
  });

  it("should have proper accessibility attributes", () => {
    localStorageMock.getItem.mockReturnValue("light");

    renderWithTheme(<ThemeToggle />);

    const button = screen.getByRole("button");
    expect(button).toHaveAttribute("aria-label");
    expect(button).toHaveAttribute("title");
    // Button type is not explicitly set, which is fine for accessibility
  });
});
