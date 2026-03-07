import { useState } from "react";

interface Props {
  onSearch: (creatorId: string) => void;
  loading: boolean;
}

export function UserSearch({ onSearch, loading }: Props) {
  const [userId, setUserId] = useState("");

  const handleSubmit = () => {
    const trimmed = userId.trim();
    if (trimmed) {
      onSearch(trimmed);
    }
  };

  return (
    <div className="user-search">
      <input
        type="text"
        className="user-search-input"
        placeholder="유저 ID 입력..."
        value={userId}
        onChange={(e) => setUserId(e.target.value)}
        onKeyDown={(e) => {
          if (e.key === "Enter") handleSubmit();
        }}
        disabled={loading}
      />
      <button
        className="search-btn"
        onClick={handleSubmit}
        disabled={loading || !userId.trim()}
        title="검색"
      >
        🔍
      </button>
    </div>
  );
}
