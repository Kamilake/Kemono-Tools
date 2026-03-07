import { render, screen, fireEvent } from "@testing-library/react";
import { describe, it, expect, vi } from "vitest";
import { UserSearch } from "../UserSearch";

describe("UserSearch", () => {
  it("renders input and search button", () => {
    render(<UserSearch onSearch={() => {}} loading={false} />);
    expect(screen.getByPlaceholderText("유저 ID 입력...")).toBeInTheDocument();
    expect(screen.getByTitle("검색")).toBeInTheDocument();
  });

  it("calls onSearch when Enter is pressed", () => {
    const onSearch = vi.fn();
    render(<UserSearch onSearch={onSearch} loading={false} />);
    const input = screen.getByPlaceholderText("유저 ID 입력...");
    fireEvent.change(input, { target: { value: "73695" } });
    fireEvent.keyDown(input, { key: "Enter" });
    expect(onSearch).toHaveBeenCalledWith("73695");
  });

  it("calls onSearch when button is clicked", () => {
    const onSearch = vi.fn();
    render(<UserSearch onSearch={onSearch} loading={false} />);
    fireEvent.change(screen.getByPlaceholderText("유저 ID 입력..."), {
      target: { value: "73695" },
    });
    fireEvent.click(screen.getByTitle("검색"));
    expect(onSearch).toHaveBeenCalledWith("73695");
  });

  it("does not call onSearch with empty input", () => {
    const onSearch = vi.fn();
    render(<UserSearch onSearch={onSearch} loading={false} />);
    fireEvent.click(screen.getByTitle("검색"));
    expect(onSearch).not.toHaveBeenCalled();
  });

  it("disables input and button when loading", () => {
    render(<UserSearch onSearch={() => {}} loading={true} />);
    expect(screen.getByPlaceholderText("유저 ID 입력...")).toBeDisabled();
    expect(screen.getByTitle("검색")).toBeDisabled();
  });
});
