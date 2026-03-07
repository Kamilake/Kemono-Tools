import type { DownloadProgress } from "../types";

interface Props {
  downloads: Map<string, DownloadProgress>;
}

export function DownloadStatus({ downloads }: Props) {
  const items = Array.from(downloads.values());
  if (items.length === 0) return null;

  const active = items.filter((d) => d.status === "downloading");
  const pending = items.filter((d) => d.status === "pending");
  const completed = items.filter((d) => d.status === "completed");
  const failed = items.filter((d) => d.status === "failed");
  const retrying = items.filter((d) => d.status === "retrying");

  return (
    <div className="download-status">
      <h3 className="download-status-title">
        다운로드 상태
        <span className="download-counts">
          {active.length > 0 && <span className="count-active">⬇️ {active.length}</span>}
          {pending.length > 0 && <span className="count-pending">⏳ {pending.length}</span>}
          {completed.length > 0 && <span className="count-done">✅ {completed.length}</span>}
          {retrying.length > 0 && <span className="count-retry">🔄 {retrying.length}</span>}
          {failed.length > 0 && <span className="count-fail">❌ {failed.length}</span>}
        </span>
      </h3>
      <div className="download-items">
        {items.map((dl) => (
          <div key={dl.id} className={`download-item status-${dl.status}`}>
            <span className="dl-name" title={dl.file_name}>
              {dl.file_name.length > 40
                ? dl.file_name.slice(0, 37) + "..."
                : dl.file_name}
            </span>
            {(dl.status === "downloading" || dl.status === "pending") && (
              <div className="progress-bar">
                <div
                  className="progress-fill"
                  style={{
                    width: `${dl.total > 0 ? Math.round((dl.downloaded / dl.total) * 100) : 0}%`,
                  }}
                />
                <span className="progress-text">
                  {dl.status === "pending"
                    ? "대기중 0%"
                    : dl.total > 0
                      ? `${Math.round((dl.downloaded / dl.total) * 100)}%`
                      : "0%"}
                </span>
              </div>
            )}
            {dl.status === "retrying" && (
              <span className="dl-status-label dl-retrying">
                🔄 재시도 {dl.attempt}/{dl.max_retries} ({dl.retry_secs}초 후)
                {dl.error && <span className="dl-error" title={dl.error}> - {dl.error.slice(0, 60)}</span>}
              </span>
            )}
            {dl.status !== "downloading" && dl.status !== "retrying" && dl.status !== "pending" && (
              <span className="dl-status-label">
                {dl.status === "completed" && "완료"}
                {dl.status === "failed" && (
                  <span title={dl.error || ""}>
                    실패 ({dl.max_retries}회 시도){dl.error ? `: ${dl.error.slice(0, 80)}` : ""}
                  </span>
                )}
              </span>
            )}
          </div>
        ))}
      </div>
    </div>
  );
}
