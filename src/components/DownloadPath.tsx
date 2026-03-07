interface Props {
  value: string;
  onChange: (path: string) => void;
}

export function DownloadPath({ value, onChange }: Props) {
  return (
    <div className="download-path">
      <label className="download-path-label">📁 다운로드 경로</label>
      <input
        type="text"
        className="download-path-input"
        value={value}
        onChange={(e) => onChange(e.target.value)}
        placeholder="다운로드 경로..."
      />
    </div>
  );
}
