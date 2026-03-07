export function Skeleton({ count = 5 }: { count?: number }) {
  return (
    <div className="skeleton-list">
      {Array.from({ length: count }).map((_, i) => (
        <div key={i} className="skeleton-item">
          <div className="skeleton-title" />
          <div className="skeleton-meta" />
          <div className="skeleton-bar" />
        </div>
      ))}
    </div>
  );
}
