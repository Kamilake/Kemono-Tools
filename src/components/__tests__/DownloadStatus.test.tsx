import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { DownloadStatus } from "../DownloadStatus";
import type { DownloadProgress } from "../../types";

describe("DownloadStatus", () => {
  it("renders nothing when no downloads", () => {
    const { container } = render(<DownloadStatus downloads={new Map()} />);
    expect(container.firstChild).toBeNull();
  });

  it("shows downloading item with progress", () => {
    const downloads = new Map<string, DownloadProgress>([
      ["dl1", { id: "dl1", downloaded: 50, total: 100, status: "downloading", file_name: "video.mp4" }],
    ]);
    render(<DownloadStatus downloads={downloads} />);
    expect(screen.getByText("video.mp4")).toBeInTheDocument();
    expect(screen.getByText("50%")).toBeInTheDocument();
  });

  it("shows completed and failed counts", () => {
    const downloads = new Map<string, DownloadProgress>([
      ["dl1", { id: "dl1", downloaded: 100, total: 100, status: "completed", file_name: "a.mp4" }],
      ["dl2", { id: "dl2", downloaded: 0, total: 100, status: "failed", file_name: "b.mp4" }],
      ["dl3", { id: "dl3", downloaded: 0, total: 0, status: "pending", file_name: "c.mp4" }],
    ]);
    render(<DownloadStatus downloads={downloads} />);
    expect(screen.getByText("✅ 1")).toBeInTheDocument();
    expect(screen.getByText("❌ 1")).toBeInTheDocument();
    expect(screen.getByText("⏳ 1")).toBeInTheDocument();
  });
});
