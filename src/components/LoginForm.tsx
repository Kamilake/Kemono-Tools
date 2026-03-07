import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

interface Props {
  username: string;
  password: string;
  onLogin: (username: string, password: string, session: string) => void;
}

export function LoginForm({ username: initUser, password: initPass, onLogin }: Props) {
  const [username, setUsername] = useState(initUser);
  const [password, setPassword] = useState(initPass);
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const handleLogin = async () => {
    if (!username.trim() || !password.trim()) return;
    setLoading(true);
    setError("");
    try {
      const session = await invoke<string>("login", {
        username: username.trim(),
        password: password.trim(),
      });
      onLogin(username.trim(), password.trim(), session);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="login-form">
      <h3>🔐 로그인</h3>
      <input
        type="text"
        placeholder="아이디"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
        disabled={loading}
      />
      <input
        type="password"
        placeholder="비밀번호"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
        onKeyDown={(e) => {
          if (e.key === "Enter") handleLogin();
        }}
        disabled={loading}
      />
      <button onClick={handleLogin} disabled={loading}>
        {loading ? "로그인 중..." : "로그인"}
      </button>
      {error && <div className="error-msg">{error}</div>}
    </div>
  );
}
