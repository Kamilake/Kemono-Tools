import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { Skeleton } from "../Skeleton";

describe("Skeleton", () => {
  it("renders default 5 skeleton items", () => {
    const { container } = render(<Skeleton />);
    const items = container.querySelectorAll(".skeleton-item");
    expect(items).toHaveLength(5);
  });

  it("renders custom count", () => {
    const { container } = render(<Skeleton count={3} />);
    const items = container.querySelectorAll(".skeleton-item");
    expect(items).toHaveLength(3);
  });

  it("each item has title, meta, bar elements", () => {
    const { container } = render(<Skeleton count={1} />);
    expect(container.querySelector(".skeleton-title")).toBeInTheDocument();
    expect(container.querySelector(".skeleton-meta")).toBeInTheDocument();
    expect(container.querySelector(".skeleton-bar")).toBeInTheDocument();
  });
});
