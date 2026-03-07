import type { Post } from "../types";
import { PostItem } from "./PostItem";
import { Skeleton } from "./Skeleton";

interface Props {
  posts: Post[];
  loading: boolean;
  error: string | null;
  onDownload: (post: Post) => void;
  downloadingPosts: Set<string>;
}

export function PostList({ posts, loading, error, onDownload, downloadingPosts }: Props) {
  if (loading) {
    return <Skeleton count={6} />;
  }

  if (error) {
    return <div className="error-msg">❌ {error}</div>;
  }

  if (posts.length === 0) {
    return <div className="empty-msg">게시글이 없습니다. 유저 ID를 입력하고 검색하세요.</div>;
  }

  return (
    <div className="post-list">
      <div className="post-count">{posts.length}개 게시글</div>
      {posts.map((post) => (
        <PostItem
          key={post.id}
          post={post}
          onDownload={onDownload}
          downloading={downloadingPosts.has(post.id)}
        />
      ))}
    </div>
  );
}
