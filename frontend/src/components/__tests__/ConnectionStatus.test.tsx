import { render, screen } from "@testing-library/react";
import { ConnectionStatus } from "../ConnectionStatus";
import { WebSocketConnection } from "../../types/energy-trading";

describe("ConnectionStatus", () => {
  it("should display connected status", () => {
    const connection: WebSocketConnection = {
      isConnected: true,
      lastMessage: null,
      error: null,
      reconnectAttempts: 0,
    };

    render(<ConnectionStatus connection={connection} />);

    expect(screen.getByText("🟢")).toBeInTheDocument();
    expect(screen.getByText("Connected")).toBeInTheDocument();
  });

  it("should display error status", () => {
    const connection: WebSocketConnection = {
      isConnected: false,
      lastMessage: null,
      error: "Connection failed",
      reconnectAttempts: 0,
    };

    render(<ConnectionStatus connection={connection} />);

    expect(screen.getByText("🔴")).toBeInTheDocument();
    expect(screen.getByText("Error")).toBeInTheDocument();
    expect(screen.getByText("Connection failed")).toBeInTheDocument();
  });

  it("should display reconnecting status", () => {
    const connection: WebSocketConnection = {
      isConnected: false,
      lastMessage: null,
      error: null,
      reconnectAttempts: 3,
    };

    render(<ConnectionStatus connection={connection} />);

    expect(screen.getByText("🟡")).toBeInTheDocument();
    expect(screen.getByText("Reconnecting (3)")).toBeInTheDocument();
  });

  it("should display connecting status", () => {
    const connection: WebSocketConnection = {
      isConnected: false,
      lastMessage: null,
      error: null,
      reconnectAttempts: 0,
    };

    render(<ConnectionStatus connection={connection} />);

    expect(screen.getByText("🟡")).toBeInTheDocument();
    expect(screen.getByText("Connecting...")).toBeInTheDocument();
  });

  it("should apply correct styling for connected status", () => {
    const connection: WebSocketConnection = {
      isConnected: true,
      lastMessage: null,
      error: null,
      reconnectAttempts: 0,
    };

    const { container } = render(<ConnectionStatus connection={connection} />);
    const statusText = container.querySelector(".text-green-600");

    expect(statusText).toBeInTheDocument();
  });

  it("should apply correct styling for error status", () => {
    const connection: WebSocketConnection = {
      isConnected: false,
      lastMessage: null,
      error: "Test error",
      reconnectAttempts: 0,
    };

    const { container } = render(<ConnectionStatus connection={connection} />);
    const statusText = container.querySelector(".text-red-600");

    expect(statusText).toBeInTheDocument();
  });

  it("should apply correct styling for reconnecting status", () => {
    const connection: WebSocketConnection = {
      isConnected: false,
      lastMessage: null,
      error: null,
      reconnectAttempts: 2,
    };

    const { container } = render(<ConnectionStatus connection={connection} />);
    const statusText = container.querySelector(".text-yellow-600");

    expect(statusText).toBeInTheDocument();
  });
});
