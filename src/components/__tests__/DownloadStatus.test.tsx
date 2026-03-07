import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { PostItem } from "../PostItem";
import type { Post, DownloadProgress } from "../../types";

const mockPost: Post = {
  id: "983680",
  user: "73695",
  service: "fantia",
  title: "테스트 게시글",
  published: "2022-06-15T12:00:00",
  attachments: [
    { name: "file1.mp4", path: "/aa/bb/hash.mp4", server: "https://n1.kemono.cr" },
    { name: "file2.png", path: "/cc/dd/hash.png", server: "https://n2.kemono.cr" },
  ],
};

const defaultProps = {
  post: mockPost,
  onDownload: () => {},
  onCancel: () => {},
  onRetry: () => {},
  downloading: false,
  downloads: [] as DownloadProgress[],
};

describe("PostItem download progress", () => {
  it("shows download button with file count when idle", () => {
    render(<PostItem {...defaultProps} />);
    expect(screen.getByText(/다운로드 \(2\)/)).toBeInTheDocument();
  });

  it("shows progress percentage when downloading", () => {
    const downloads: DownloadProgress[] = [
      { id: "983680_file1.mp4_url1", downloaded: 50, total: 100, status: "downloading", file_name: "file1.mp4", attempt: 1, max_retries: 25, retry_secs: 0 },
      { id: "983680_file2.png_url2", downloaded: 0, total: 100, status: "pending", file_name: "file2.png", attempt: 0, max_retries: 25, retry_secs: 0 },
    ];
    render(<PostItem {...defaultProps} downloading={true} downloads={downloads} />);
    // 0 completed out of 2 files = 0%
    expect(screen.getByText("0/2 (0%)")).toBeInTheDocument();
  });

  it("shows completed state", () => {
    const downloads: DownloadProgress[] = [
      { id: "983680_file1.mp4_url1", downloaded: 100, total: 100, status: "completed", file_name: "file1.mp4", attempt: 1, max_retries: 25, retry_secs: 0 },
      { id: "983680_file2.png_url2", downloaded: 100, total: 100, status: "completed", file_name: "file2.png", attempt: 1, max_retries: 25, retry_secs: 0 },
    ];
    render(<PostItem {...defaultProps} downloads={downloads} />);
    expect(screen.getByText("✅ 완료")).toBeInTheDocument();
  });

  it("shows failed state with retry button", () => {
    const downloads: DownloadProgress[] = [
      { id: "983680_file1.mp4_url1", downloaded: 0, total: 100, status: "failed", file_name: "file1.mp4", attempt: 25, max_retries: 25, retry_secs: 0, error: "Connection reset" },
    ];
    render(<PostItem {...defaultProps} downloads={downloads} />);
    expect(screen.getByText("❌ 실패")).toBeInTheDocument();
    expect(screen.getByTitle("실패한 파일 재시도")).toBeInTheDocument();
  });

  it("shows cancel button when active", () => {
    const downloads: DownloadProgress[] = [
      { id: "983680_file1.mp4_url1", downloaded: 30, total: 100, status: "downloading", file_name: "file1.mp4", attempt: 1, max_retries: 25, retry_secs: 0 },
    ];
    render(<PostItem {...defaultProps} downloading={true} downloads={downloads} />);
    expect(screen.getByTitle("다운로드 취소")).toBeInTheDocument();
  });

  it("shows inline file list with per-file status", () => {
    const downloads: DownloadProgress[] = [
      { id: "983680_file1.mp4_url1", downloaded: 100, total: 100, status: "completed", file_name: "file1.mp4", attempt: 1, max_retries: 25, retry_secs: 0 },
      { id: "983680_file2.png_url2", downloaded: 30, total: 100, status: "downloading", file_name: "file2.png", attempt: 1, max_retries: 25, retry_secs: 0 },
    ];
    render(<PostItem {...defaultProps} downloading={true} downloads={downloads} />);
    expect(screen.getByText("file1.mp4")).toBeInTheDocument();
    expect(screen.getByText("file2.png")).toBeInTheDocument();
  });
});
