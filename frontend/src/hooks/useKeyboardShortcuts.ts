import { useEffect } from "react";

interface KeyboardShortcuts {
  onRefresh?: () => void;
  onToggleTheme?: () => void;
  onFocusSearch?: () => void;
  onEscape?: () => void;
}

export const useKeyboardShortcuts = ({
  onRefresh,
  onToggleTheme,
  onFocusSearch,
  onEscape,
}: KeyboardShortcuts) => {
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      // Ignore if user is typing in an input field
      if (
        event.target instanceof HTMLInputElement ||
        event.target instanceof HTMLTextAreaElement ||
        event.target instanceof HTMLSelectElement
      ) {
        return;
      }

      // Ctrl/Cmd + R: Refresh data
      if ((event.ctrlKey || event.metaKey) && event.key === "r") {
        event.preventDefault();
        onRefresh?.();
      }

      // Ctrl/Cmd + Shift + T: Toggle theme
      if (
        (event.ctrlKey || event.metaKey) &&
        event.shiftKey &&
        event.key === "T"
      ) {
        event.preventDefault();
        onToggleTheme?.();
      }

      // Ctrl/Cmd + K: Focus search
      if ((event.ctrlKey || event.metaKey) && event.key === "k") {
        event.preventDefault();
        onFocusSearch?.();
      }

      // Escape: Close modals/clear focus
      if (event.key === "Escape") {
        onEscape?.();
      }
    };

    document.addEventListener("keydown", handleKeyDown);
    return () => document.removeEventListener("keydown", handleKeyDown);
  }, [onRefresh, onToggleTheme, onFocusSearch, onEscape]);
};
