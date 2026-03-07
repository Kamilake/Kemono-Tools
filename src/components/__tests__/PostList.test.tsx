import { render, screen } from "@testing-library/react";
import { describe, it, expect } from "vitest";
import { PostList } from "../PostList";
import type { Post } from "../../types";

const mockPosts: Post[] = [
  {
    id: "983680",
    user: "73695",
    service: "fantia",
    title: "테스트 게시글",
    published: "2022-06-15T12:00:00",
    attachments: [
      { name: "file1.mp4", path: "/aa/bb/hash.mp4", server: "https://n1.kemono.cr" },
      { name: "file2.png", path: "/cc/dd/hash.png", server: "https://n2.kemono.cr" },
    ],
  },
  {
    id: "123456",
    user: "73695",
    service: "fantia",
    title: "두번째 게시글",
    published: "2022-07-01T12:00:00",
    attachments: [],
  },
];

const defaultProps = {
  onDownload: () => {},
  onCancelDownload: () => {},
  onRetryDownload: () => {},
  downloadingPosts: new Set<string>(),
  downloadsByPost: new Map(),
};

describe("PostList", () => {
  it("shows skeleton when loading", () => {
    const { container } = render(
      <PostList posts={[]} loading={true} error={null} {...defaultProps} />
    );
    expect(container.querySelector(".skeleton-list")).toBeInTheDocument();
  });

  it("shows error message", () => {
    render(
      <PostList posts={[]} loading={false} error="Something went wrong" {...defaultProps} />
    );
    expect(screen.getByText(/Something went wrong/)).toBeInTheDocument();
  });

  it("shows empty message when no posts", () => {
    render(
      <PostList posts={[]} loading={false} error={null} {...defaultProps} />
    );
    expect(screen.getByText(/게시글이 없습니다/)).toBeInTheDocument();
  });

  it("renders post items", () => {
    render(
      <PostList posts={mockPosts} loading={false} error={null} {...defaultProps} />
    );
    expect(screen.getByText("테스트 게시글")).toBeInTheDocument();
    expect(screen.getByText("두번째 게시글")).toBeInTheDocument();
    expect(screen.getByText("2개 게시글")).toBeInTheDocument();
  });

  it("shows file count per post", () => {
    render(
      <PostList posts={mockPosts} loading={false} error={null} {...defaultProps} />
    );
    expect(screen.getByText("📎 2개 파일")).toBeInTheDocument();
    expect(screen.getByText("📎 0개 파일")).toBeInTheDocument();
  });
});
