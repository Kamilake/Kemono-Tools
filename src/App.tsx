import { useState, useCallback, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { ServiceSelector } from "./components/ServiceSelector";
import { UserSearch } from "./components/UserSearch";
import { PostList } from "./components/PostList";
import { DownloadPath } from "./components/DownloadPath";
import { DownloadStatus } from "./components/DownloadStatus";
import { LoginForm } from "./components/LoginForm";
import { useSettings } from "./hooks/useSettings";
import { usePosts } from "./hooks/usePosts";
import type { Post, DownloadProgress, ServiceName } from "./types";

export default function App() {
  const { settings, updateSettings, loaded } = useSettings();
  const { posts, loading, error, fetchPosts } = usePosts();
  const [downloadingPosts, setDownloadingPosts] = useState<Set<string>>(new Set());
  const [downloads, setDownloads] = useState<Map<string, DownloadProgress>>(new Map());
  const [needsLogin, setNeedsLogin] = useState(false);
  const [currentCreatorId, setCurrentCreatorId] = useState("");

  // Listen for download progress events
  useEffect(() => {
    const unlisten = listen<DownloadProgress>("download-progress", (event) => {
      setDownloads((prev) => {
        const next = new Map(prev);
        next.set(event.payload.id, event.payload);
        return next;
      });

      if (event.payload.status === "completed" || event.payload.status === "failed") {
        // Check if all downloads for a post are done
        setTimeout(() => {
          setDownloads((current) => {
            const postDownloads = Array.from(current.values());
            const byPost = new Map<string, DownloadProgress[]>();
            for (const dl of postDownloads) {
              const postId = dl.id.split("_")[0];
              if (!byPost.has(postId)) byPost.set(postId, []);
              byPost.get(postId)!.push(dl);
            }
            for (const [postId, dls] of byPost) {
              if (dls.every((d) => d.status === "completed" || d.status === "failed")) {
                setDownloadingPosts((prev) => {
                  const next = new Set(prev);
                  next.delete(postId);
                  return next;
                });
              }
            }
            return current;
          });
        }, 500);
      }
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  // Check if session exists
  useEffect(() => {
    if (loaded && !settings.session) {
      setNeedsLogin(true);
    }
  }, [loaded, settings.session]);

  const handleSearch = useCallback(
    (creatorId: string) => {
      setCurrentCreatorId(creatorId);
      fetchPosts(settings.service, creatorId);
    },
    [settings.service, fetchPosts]
  );

  const handleDownload = useCallback(
    async (post: Post) => {
      setDownloadingPosts((prev) => new Set(prev).add(post.id));
      try {
        await invoke("download_post_files", {
          service: settings.service,
          creatorId: currentCreatorId,
          postId: post.id,
        });
      } catch (e) {
        console.error("Download failed:", e);
        setDownloadingPosts((prev) => {
          const next = new Set(prev);
          next.delete(post.id);
          return next;
        });
      }
    },
    [settings.service, currentCreatorId]
  );

  const handleLogin = useCallback(
    async (username: string, password: string, session: string) => {
      await updateSettings({ username, password, session });
      setNeedsLogin(false);
    },
    [updateSettings]
  );

  if (!loaded) {
    return <div className="empty-msg">로딩 중...</div>;
  }

  if (needsLogin) {
    return (
      <div>
        <div className="app-header">
          <h1>Kemono Tools</h1>
        </div>
        <LoginForm
          username={settings.username}
          password={settings.password}
          onLogin={handleLogin}
        />
      </div>
    );
  }

  return (
    <div>
      <div className="app-header">
        <h1>Kemono Tools</h1>
      </div>

      <div className="controls">
        <ServiceSelector
          value={settings.service as ServiceName}
          onChange={(service) => updateSettings({ service })}
        />
        <UserSearch onSearch={handleSearch} loading={loading} />
      </div>

      <DownloadPath
        value={settings.download_path}
        onChange={(download_path) => updateSettings({ download_path })}
      />

      <button
        className="debug-btn"
        style={{ margin: "4px 0", padding: "4px 8px", fontSize: "12px", opacity: 0.7 }}
        onClick={async () => {
          try {
            const info = await invoke<string>("debug_download_path");
            alert(info);
          } catch (e) {
            alert("Error: " + e);
          }
        }}
      >
        🔍 다운로드 경로 확인
      </button>

      <PostList
        posts={posts}
        loading={loading}
        error={error}
        onDownload={handleDownload}
        downloadingPosts={downloadingPosts}
      />

      <DownloadStatus downloads={downloads} />
    </div>
  );
}
