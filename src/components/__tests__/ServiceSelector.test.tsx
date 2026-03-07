import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { ServiceSelector } from "../ServiceSelector";
import { SERVICES } from "../../types";

describe("ServiceSelector", () => {
  it("renders all service options", () => {
    render(<ServiceSelector value="fantia" onChange={() => {}} />);
    const select = screen.getByRole("combobox");
    expect(select).toBeInTheDocument();
    const options = screen.getAllByRole("option");
    expect(options).toHaveLength(SERVICES.length);
  });

  it("shows selected value", () => {
    render(<ServiceSelector value="pixiv" onChange={() => {}} />);
    const select = screen.getByRole("combobox") as HTMLSelectElement;
    expect(select.value).toBe("pixiv");
  });

  it("calls onChange when selection changes", () => {
    const onChange = vi.fn();
    render(<ServiceSelector value="fantia" onChange={onChange} />);
    fireEvent.change(screen.getByRole("combobox"), { target: { value: "patreon" } });
    expect(onChange).toHaveBeenCalledWith("patreon");
  });
});
