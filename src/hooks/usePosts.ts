import { useState, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { Post } from "../types";

export function usePosts() {
  const [posts, setPosts] = useState<Post[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const fetchPosts = useCallback(async (service: string, creatorId: string) => {
    setLoading(true);
    setError(null);
    setPosts([]);
    try {
      const result = await invoke<Post[]>("get_posts", {
        service,
        creatorId,
      });
      setPosts(result);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }, []);

  const fetchPost = useCallback(async (service: string, creatorId: string, postId: string) => {
    return invoke<Post>("get_post", { service, creatorId, postId });
  }, []);

  return { posts, loading, error, fetchPosts, fetchPost };
}
