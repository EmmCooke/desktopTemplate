import { describe, it, expect } from "vitest";
import { render, screen } from "@/test/test-utils";
import { RootLayout } from "./root-layout";

describe("RootLayout", () => {
  it("renders the app title in the header", () => {
    render(<RootLayout />);
    expect(screen.getByText("Desktop Template")).toBeInTheDocument();
  });
});
