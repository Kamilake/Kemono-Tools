import type { Post, DownloadProgress } from "../types";

interface Props {
  post: Post;
  onDownload: (post: Post) => void;
  onCancel: (postId: string) => void;
  onRetry: (post: Post) => void;
  downloading: boolean;
  downloads: DownloadProgress[];
}

function formatSize(bytes: number): string {
  if (bytes === 0) return "";
  if (bytes < 1024) return `${bytes}B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)}KB`;
  if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)}MB`;
  return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)}GB`;
}

export function PostItem({ post, onDownload, onCancel, onRetry, downloading, downloads }: Props) {
  const fileCount = post.attachments.length + (post.file ? 1 : 0);

  // Compute aggregate progress: completed files / total files
  const hasDownloads = downloads.length > 0;
  const completedCount = downloads.filter((d) => d.status === "completed").length;
  const totalFiles = hasDownloads ? downloads.length : fileCount;
  const overallPercent = totalFiles > 0 ? Math.round((completedCount / totalFiles) * 100) : 0;

  const allDone = hasDownloads && downloads.every((d) => d.status === "completed");
  const hasFailed = downloads.some((d) => d.status === "failed");
  const isActive = downloading || downloads.some((d) => d.status === "downloading" || d.status === "pending" || d.status === "retrying");

  // Button state
  let btnLabel: string;
  let btnClass = "download-btn";
  if (allDone) {
    btnLabel = "✅ 완료";
    btnClass += " download-btn--completed";
  } else if (hasFailed && !isActive) {
    btnLabel = "❌ 실패";
    btnClass += " download-btn--failed";
  } else if (isActive) {
    btnLabel = `${completedCount}/${totalFiles} (${overallPercent}%)`;
    btnClass += " download-btn--active";
  } else {
    btnLabel = `⬇️ 다운로드 (${fileCount})`;
  }

  const handleClick = () => {
    if (allDone || isActive) return;
    if (hasFailed) {
      onRetry(post);
    } else {
      onDownload(post);
    }
  };

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

      <div className="post-actions">
        <button
          className={btnClass}
          onClick={handleClick}
          disabled={fileCount === 0 || allDone || (isActive && !hasFailed)}
          style={isActive && hasDownloads ? {
            background: `linear-gradient(to right, var(--accent) ${overallPercent}%, var(--surface2) ${overallPercent}%)`,
          } : undefined}
        >
          {btnLabel}
        </button>
        {isActive && (
          <button className="cancel-btn" onClick={() => onCancel(post.id)} title="다운로드 취소">
            ✕
          </button>
        )}
        {hasFailed && !isActive && (
          <button className="retry-btn" onClick={() => onRetry(post)} title="실패한 파일 재시도">
            🔄
          </button>
        )}
      </div>

      {hasDownloads && (
        <div className="post-downloads">
          {downloads.map((dl) => (
            <div key={dl.id} className={`post-dl-item post-dl--${dl.status}`}>
              <span className="post-dl-name" title={dl.file_name}>
                {dl.file_name.length > 50 ? dl.file_name.slice(0, 47) + "..." : dl.file_name}
              </span>
              <span className="post-dl-status">
                {dl.status === "downloading" && (
                  <>
                    <span className="post-dl-bar">
                      <span
                        className="post-dl-bar-fill"
                        style={{ width: `${dl.total > 0 ? Math.round((dl.downloaded / dl.total) * 100) : 0}%` }}
                      />
                    </span>
                    <span className="post-dl-pct">
                      {dl.total > 0 ? `${Math.round((dl.downloaded / dl.total) * 100)}%` : "0%"}
                      {dl.total > 0 && <span className="post-dl-size"> {formatSize(dl.downloaded)}/{formatSize(dl.total)}</span>}
                    </span>
                  </>
                )}
                {dl.status === "pending" && <span className="post-dl-pending">대기중</span>}
                {dl.status === "completed" && <span className="post-dl-done">✅</span>}
                {dl.status === "failed" && (
                  <span className="post-dl-fail" title={dl.error || ""}>
                    ❌ {dl.error ? dl.error.slice(0, 40) : "실패"}
                  </span>
                )}
                {dl.status === "retrying" && (
                  <span className="post-dl-retry">
                    🔄 {dl.retry_secs}초 후 재시도 ({dl.attempt}/{dl.max_retries})
                  </span>
                )}
              </span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
