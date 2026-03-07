export interface Attachment {
  server?: string;
  name: string;
  path: string;
  extension?: string;
}

export interface FileInfo {
  name: string;
  path: string;
  server?: string;
}

export interface Post {
  id: string;
  user: string;
  service: string;
  title: string;
  substring?: string;
  published?: string;
  file?: FileInfo;
  attachments: Attachment[];
}

export interface DownloadState {
  url: string;
  path: string;
  downloaded: number;
  total: number;
  status: "pending" | "downloading" | "completed" | "failed" | "paused";
  post_id: string;
  file_name: string;
}

export interface Settings {
  server: string;
  service: string;
  session: string;
  download_path: string;
  username: string;
  password: string;
  downloads: Record<string, DownloadState>;
}

export interface DownloadProgress {
  id: string;
  downloaded: number;
  total: number;
  status: string;
  file_name: string;
  attempt: number;
  max_retries: number;
  retry_secs: number;
  error?: string;
}

export const SERVICES = [
  "fanbox",
  "fantia",
  "patreon",
  "pixiv",
  "gumroad",
  "subscribestar",
  "dlsite",
  "discord",
  "boosty",
  "afdian",
] as const;

export type ServiceName = (typeof SERVICES)[number];
