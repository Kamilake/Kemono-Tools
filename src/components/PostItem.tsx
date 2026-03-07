import type { Post } from "../types";

interface Props {
  post: Post;
  onDownload: (post: Post) => void;
  downloading: boolean;
}

export function PostItem({ post, onDownload, downloading }: Props) {
  const fileCount =
    post.attachments.length + (post.file ? 1 : 0);

  return (
    <div className="post-item">
      <div className="post-info">
        <h3 className="post-title">{post.title || `Post #${post.id}`}</h3>
        <div className="post-meta">
          <span className="post-id">ID: {post.id}</span>
          {post.published && (
            <span className="post-date">
              {new Date(post.published).toLocaleDateString("ko-KR")}
            </span>
          )}
          <span className="post-files">📎 {fileCount}개 파일</span>
        </div>
        {post.substring && (
          <p className="post-excerpt">
            {post.substring.slice(0, 120)}
            {post.substring.length > 120 ? "..." : ""}
          </p>
        )}
      </div>
      <button
        className="download-btn"
        onClick={() => onDownload(post)}
        disabled={downloading || fileCount === 0}
      >
        {downloading ? "⏳ 대기중..." : `⬇️ 전체 다운로드 (${fileCount})`}
      </button>
    </div>
  );
}
