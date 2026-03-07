use serde_json::Value;

/// Generate a post info HTML page from the raw API detail JSON.
/// All file links use local relative paths (same folder as the HTML).
pub fn generate_post_html(json: &Value) -> String {
    let post = json.get("post").unwrap_or(json);
    let top_attachments = json.get("attachments").and_then(|v| v.as_array());
    let previews = json.get("previews").and_then(|v| v.as_array());
    let videos = json.get("videos").and_then(|v| v.as_array());

    let title = post_str(post, "title").unwrap_or("제목 없음");
    let id = post_str(post, "id").unwrap_or("");
    let service = post_str(post, "service").unwrap_or("");
    let user = post_str(post, "user").unwrap_or("");
    let published = post_str(post, "published").unwrap_or("");
    let added = post_str(post, "added").unwrap_or("");
    let edited = post_str(post, "edited");
    let content = post_str(post, "content").unwrap_or("");
    let shared_file = post.get("shared_file").and_then(|v| v.as_bool()).unwrap_or(false);
    let prev_id = post_str(post, "prev");
    let next_id = post_str(post, "next");
    let embed = post.get("embed").and_then(|v| v.as_object());
    let tags = post.get("tags").and_then(|v| v.as_array());
    let file = post.get("file").and_then(|v| v.as_object());

    let kemono_url = format!("https://kemono.cr/{}/user/{}/post/{}", service, user, id);

    // --- Master thumbnail ---
    let master_thumb_name = previews
        .and_then(|p| p.first())
        .and_then(|prv| prv.get("name").and_then(|v| v.as_str()))
        .unwrap_or("");

    let thumb_html = if !master_thumb_name.is_empty() {
        format!(
            r#"<div class="hero-thumb"><img src="{}" alt="썸네일" loading="lazy"></div>"#,
            esc(master_thumb_name)
        )
    } else {
        r#"<div class="hero-thumb hero-thumb--empty"><i data-lucide="image-off"></i></div>"#.to_string()
    };

    // --- Content ---
    let content_html = if content.is_empty() {
        r#"<span class="empty">내용 없음</span>"#.to_string()
    } else {
        auto_link_urls(&esc(content)).replace('\n', "<br>\n")
    };

    // --- Attachments ---
    let mut att_rows = String::new();
    let att_count;
    if let Some(atts) = top_attachments {
        att_count = atts.len();
        for (i, att) in atts.iter().enumerate() {
            let name = att.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let ext = att.get("extension").and_then(|v| v.as_str()).unwrap_or("");
            let icon = ext_icon(ext);
            att_rows.push_str(&format!(
                r#"            <tr>
              <td class="att-idx">{}</td>
              <td><i data-lucide="{}"></i> <a href="{}" title="{}">{}</a></td>
              <td><code>{}</code></td>
            </tr>
"#,
                i + 1, icon, esc(name), esc(name), esc(name), esc(ext),
            ));
        }
    } else {
        att_count = 0;
    }

    // --- Previews (skip first = master thumb) ---
    let mut preview_figures = String::new();
    if let Some(prvs) = previews {
        for (i, prv) in prvs.iter().enumerate() {
            if i == 0 { continue; }
            let name = prv.get("name").and_then(|v| v.as_str()).unwrap_or("");
            preview_figures.push_str(&format!(
                r#"          <figure><img src="{}" alt="{}" loading="lazy"><figcaption>{}</figcaption></figure>
"#,
                esc(name), esc(name), esc(name),
            ));
        }
    }

    // --- Videos ---
    let mut video_items = String::new();
    let vid_count;
    if let Some(vids) = videos {
        vid_count = vids.len();
        for vid in vids {
            let name = vid.get("name").and_then(|v| v.as_str()).unwrap_or("");
            video_items.push_str(&format!(
                r#"            <li><i data-lucide="play-circle"></i> <a href="{}">{}</a></li>
"#,
                esc(name), esc(name),
            ));
        }
    } else {
        vid_count = 0;
    }

    // --- Embed ---
    let embed_section = match embed {
        Some(emb) if !emb.is_empty() => {
            let mut rows = String::new();
            for (k, v) in emb {
                let val_str = match v {
                    Value::String(s) => s.clone(),
                    other => other.to_string(),
                };
                rows.push_str(&format!(
                    "            <dt>{}</dt><dd>{}</dd>\n",
                    esc(k), esc(&val_str)
                ));
            }
            format!(
                r#"      <section class="card">
        <h2><i data-lucide="code-2"></i> 임베드</h2>
        <dl class="info-grid">{}</dl>
      </section>
"#,
                rows
            )
        }
        _ => String::new(),
    };

    // --- Tags ---
    let tags_section = match tags {
        Some(tag_list) if !tag_list.is_empty() => {
            let mut items = String::new();
            for tag in tag_list {
                let t = tag.as_str().unwrap_or("");
                items.push_str(&format!(
                    "            <span class=\"tag\">{}</span>\n",
                    esc(t)
                ));
            }
            format!(
                r#"      <section class="card">
        <h2><i data-lucide="tags"></i> 태그</h2>
        <div class="tag-list">{}</div>
      </section>
"#,
                items
            )
        }
        _ => String::new(),
    };

    // --- Main file ---
    let file_section = match file {
        Some(f) => {
            let fname = f.get("name").and_then(|v| v.as_str()).unwrap_or("");
            format!(
                r#"      <div class="main-file"><i data-lucide="file-down"></i> <strong>대표 파일:</strong> <a href="{}">{}</a></div>
"#,
                esc(fname), esc(fname)
            )
        }
        None => String::new(),
    };

    // --- Navigation ---
    let mut nav_links = format!(
        r#"          <a href="{}" target="_blank" rel="noopener" class="nav-btn"><i data-lucide="external-link"></i> Kemono에서 보기</a>
"#,
        kemono_url
    );
    if let Some(prev) = prev_id {
        let prev_url = format!("https://kemono.cr/{}/user/{}/post/{}", service, user, prev);
        nav_links.push_str(&format!(
            r#"          <a href="{}" target="_blank" rel="noopener" class="nav-btn"><i data-lucide="arrow-left"></i> 이전 ({})</a>
"#,
            prev_url, prev
        ));
    }
    if let Some(next) = next_id {
        let next_url = format!("https://kemono.cr/{}/user/{}/post/{}", service, user, next);
        nav_links.push_str(&format!(
            r#"          <a href="{}" target="_blank" rel="noopener" class="nav-btn">다음 ({}) <i data-lucide="arrow-right"></i></a>
"#,
            next_url, next
        ));
    }

    // --- Edited row ---
    let edited_row = match edited {
        Some(ed) => format!(
            r#"
              <dt><i data-lucide="pencil"></i> 수정일</dt><dd>{}</dd>"#,
            esc(ed)
        ),
        None => String::new(),
    };

    format!(
        r##"<!DOCTYPE html>
<html lang="ko">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>{title} — {id}</title>
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&family=Noto+Sans+KR:wght@400;500;700&display=swap" rel="stylesheet">
  <script src="https://unpkg.com/lucide@latest"></script>
  <style>
    :root {{
      --bg: #0f0f1a;
      --surface: #1a1a2e;
      --surface2: #1f1f35;
      --text: #e2e2ef;
      --text-dim: #8888a8;
      --accent: #6c5ce7;
      --accent-soft: rgba(108,92,231,0.15);
      --link: #74b9ff;
      --link-hover: #a29bfe;
      --border: #2d2d4a;
      --success: #00cec9;
      --radius: 12px;
      --shadow: 0 2px 12px rgba(0,0,0,0.3);
    }}
    * {{ margin: 0; padding: 0; box-sizing: border-box; }}
    body {{
      font-family: 'Inter', 'Noto Sans KR', -apple-system, BlinkMacSystemFont, sans-serif;
      background: var(--bg);
      color: var(--text);
      line-height: 1.7;
      min-height: 100vh;
    }}
    .container {{ max-width: 1000px; margin: 0 auto; padding: 2rem 1.5rem; }}
    .header {{
      background: linear-gradient(135deg, var(--surface) 0%, var(--surface2) 100%);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 1.75rem;
      margin-bottom: 1.5rem;
      box-shadow: var(--shadow);
    }}
    .header h1 {{
      font-size: 1.6rem;
      font-weight: 700;
      color: #fff;
      margin-bottom: 0.25rem;
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }}
    .header h1 .badge {{
      font-size: 0.7rem;
      font-weight: 500;
      background: var(--accent);
      color: #fff;
      padding: 0.15rem 0.5rem;
      border-radius: 999px;
      text-transform: uppercase;
      letter-spacing: 0.05em;
    }}
    .hero {{
      display: grid;
      grid-template-columns: 1fr 280px;
      gap: 1.5rem;
      align-items: start;
    }}
    @media (max-width: 700px) {{
      .hero {{ grid-template-columns: 1fr; }}
      .hero-thumb {{ order: -1; }}
    }}
    .hero-thumb {{
      border-radius: var(--radius);
      overflow: hidden;
      border: 1px solid var(--border);
      box-shadow: var(--shadow);
      background: var(--surface2);
    }}
    .hero-thumb img {{
      width: 100%;
      display: block;
      object-fit: cover;
      max-height: 320px;
    }}
    .hero-thumb--empty {{
      display: flex;
      align-items: center;
      justify-content: center;
      min-height: 180px;
      color: var(--text-dim);
    }}
    .info-grid {{
      display: grid;
      grid-template-columns: auto 1fr;
      gap: 0.4rem 1rem;
    }}
    .info-grid dt {{
      font-weight: 500;
      color: var(--text-dim);
      font-size: 0.85rem;
      display: flex;
      align-items: center;
      gap: 0.35rem;
      white-space: nowrap;
    }}
    .info-grid dd {{
      font-size: 0.9rem;
      word-break: break-all;
    }}
    .info-grid dd a {{ color: var(--link); }}
    .card {{
      background: var(--surface);
      border: 1px solid var(--border);
      border-radius: var(--radius);
      padding: 1.25rem 1.5rem;
      margin-bottom: 1.25rem;
      box-shadow: var(--shadow);
    }}
    .card h2 {{
      font-size: 1rem;
      font-weight: 600;
      color: var(--accent);
      margin-bottom: 1rem;
      display: flex;
      align-items: center;
      gap: 0.4rem;
      padding-bottom: 0.6rem;
      border-bottom: 1px solid var(--border);
    }}
    .card h2 .count {{
      font-size: 0.75rem;
      font-weight: 500;
      background: var(--accent-soft);
      color: var(--accent);
      padding: 0.1rem 0.45rem;
      border-radius: 999px;
      margin-left: 0.25rem;
    }}
    .content-body {{
      white-space: pre-wrap;
      word-wrap: break-word;
      font-size: 0.92rem;
      line-height: 1.8;
    }}
    .content-body a {{ color: var(--link); }}
    .content-body a:hover {{ color: var(--link-hover); text-decoration: underline; }}
    .empty {{ color: var(--text-dim); font-style: italic; }}
    .main-file {{
      display: flex;
      align-items: center;
      gap: 0.5rem;
      background: var(--accent-soft);
      border: 1px solid var(--accent);
      border-radius: 8px;
      padding: 0.6rem 1rem;
      margin-bottom: 1.25rem;
      font-size: 0.9rem;
    }}
    .main-file a {{ color: var(--link); }}
    .att-table {{ width: 100%; border-collapse: collapse; font-size: 0.85rem; }}
    .att-table th {{
      text-align: left;
      padding: 0.6rem 0.75rem;
      background: var(--surface2);
      color: var(--text-dim);
      font-weight: 600;
      font-size: 0.8rem;
      text-transform: uppercase;
      letter-spacing: 0.04em;
      border-bottom: 2px solid var(--border);
    }}
    .att-table td {{
      padding: 0.55rem 0.75rem;
      border-bottom: 1px solid var(--border);
      vertical-align: middle;
    }}
    .att-table tr:hover td {{ background: var(--accent-soft); }}
    .att-table a {{ color: var(--link); }}
    .att-table a:hover {{ color: var(--link-hover); text-decoration: underline; }}
    .att-table code {{ font-size: 0.78rem; color: var(--text-dim); background: var(--surface2); padding: 0.1rem 0.35rem; border-radius: 4px; }}
    .att-idx {{ color: var(--text-dim); font-size: 0.8rem; text-align: center; width: 2rem; }}
    .preview-grid {{
      display: grid;
      grid-template-columns: repeat(auto-fill, minmax(160px, 1fr));
      gap: 0.75rem;
    }}
    .preview-grid figure {{ margin: 0; }}
    .preview-grid img {{
      width: 100%;
      border-radius: 8px;
      display: block;
      transition: transform 0.2s, box-shadow 0.2s;
      cursor: pointer;
      border: 1px solid var(--border);
    }}
    .preview-grid img:hover {{
      transform: scale(1.03);
      box-shadow: 0 4px 20px rgba(108,92,231,0.3);
    }}
    .preview-grid figcaption {{
      font-size: 0.7rem;
      color: var(--text-dim);
      margin-top: 0.3rem;
      overflow: hidden;
      text-overflow: ellipsis;
      white-space: nowrap;
    }}
    .video-list {{ list-style: none; }}
    .video-list li {{
      padding: 0.45rem 0;
      display: flex;
      align-items: center;
      gap: 0.4rem;
      font-size: 0.9rem;
    }}
    .video-list li + li {{ border-top: 1px solid var(--border); }}
    .video-list a {{ color: var(--link); }}
    .video-list a:hover {{ color: var(--link-hover); text-decoration: underline; }}
    .tag-list {{ display: flex; gap: 0.4rem; flex-wrap: wrap; }}
    .tag {{
      background: var(--accent-soft);
      color: var(--accent);
      padding: 0.2rem 0.65rem;
      border-radius: 999px;
      font-size: 0.8rem;
      font-weight: 500;
      border: 1px solid rgba(108,92,231,0.3);
    }}
    .nav-bar {{
      display: flex;
      gap: 0.75rem;
      flex-wrap: wrap;
      margin-top: 1.5rem;
      padding-top: 1.25rem;
      border-top: 1px solid var(--border);
    }}
    .nav-btn {{
      display: inline-flex;
      align-items: center;
      gap: 0.35rem;
      padding: 0.5rem 1rem;
      border-radius: 8px;
      font-size: 0.85rem;
      font-weight: 500;
      color: var(--text);
      background: var(--surface);
      border: 1px solid var(--border);
      text-decoration: none;
      transition: all 0.15s;
    }}
    .nav-btn:hover {{
      background: var(--accent-soft);
      border-color: var(--accent);
      color: #fff;
    }}
    .footer {{
      margin-top: 2rem;
      padding: 1rem 0;
      border-top: 1px solid var(--border);
      font-size: 0.78rem;
      color: var(--text-dim);
      text-align: center;
    }}
    .footer a {{ color: var(--text-dim); }}
    .footer a:hover {{ color: var(--link); }}
    [data-lucide] {{ width: 16px; height: 16px; stroke-width: 2; vertical-align: -2px; }}
  </style>
</head>
<body>
  <div class="container">

    <div class="header">
      <h1>{title} <span class="badge">{service}</span></h1>
    </div>

    <div class="hero">
      <div class="card" style="margin-bottom:0">
        <h2><i data-lucide="info"></i> 게시글 정보</h2>
        <dl class="info-grid">
          <dt><i data-lucide="hash"></i> 게시글 ID</dt><dd>{id}</dd>
          <dt><i data-lucide="server"></i> 서비스</dt><dd>{service}</dd>
          <dt><i data-lucide="user"></i> 크리에이터 ID</dt><dd>{user}</dd>
          <dt><i data-lucide="calendar"></i> 게시일</dt><dd>{published}</dd>
          <dt><i data-lucide="download-cloud"></i> 수집일</dt><dd>{added}</dd>{edited_row}
          <dt><i data-lucide="share-2"></i> 공유 파일</dt><dd>{shared_file}</dd>
        </dl>
      </div>
      {thumb_html}
    </div>

    <div style="height:1.25rem"></div>

    <section class="card">
      <h2><i data-lucide="file-text"></i> 본문</h2>
      <div class="content-body">{content_html}</div>
    </section>

{file_section}
{embed_section}
{tags_section}
    <section class="card">
      <h2><i data-lucide="paperclip"></i> 첨부파일 <span class="count">{att_count}</span></h2>
      <table class="att-table">
        <thead>
          <tr><th>#</th><th>파일명</th><th>형식</th></tr>
        </thead>
        <tbody>
{att_rows}        </tbody>
      </table>
    </section>

    <section class="card">
      <h2><i data-lucide="images"></i> 미리보기</h2>
      <div class="preview-grid">
{preview_figures}      </div>
    </section>

    <section class="card">
      <h2><i data-lucide="film"></i> 동영상 <span class="count">{vid_count}</span></h2>
      <ul class="video-list">
{video_items}      </ul>
    </section>

    <nav class="nav-bar">
{nav_links}    </nav>

    <div class="footer">
      Kemono-Tools로 생성됨 &bull; 원본: <a href="{kemono_url}" target="_blank" rel="noopener">{kemono_url}</a>
    </div>

  </div>
  <script>lucide.createIcons();</script>
</body>
</html>"##,
        title = esc(title),
        id = esc(id),
        service = esc(service),
        user = esc(user),
        published = esc(published),
        added = esc(added),
        edited_row = edited_row,
        shared_file = if shared_file { "예" } else { "아니오" },
        thumb_html = thumb_html,
        content_html = content_html,
        file_section = file_section,
        embed_section = embed_section,
        tags_section = tags_section,
        att_rows = att_rows,
        att_count = att_count,
        preview_figures = preview_figures,
        video_items = video_items,
        vid_count = vid_count,
        nav_links = nav_links,
        kemono_url = kemono_url,
    )
}

// ── Helpers ──

fn post_str<'a>(post: &'a Value, key: &str) -> Option<&'a str> {
    post.get(key).and_then(|v| v.as_str())
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
     .replace('<', "&lt;")
     .replace('>', "&gt;")
     .replace('"', "&quot;")
}

fn ext_icon(ext: &str) -> &'static str {
    match ext {
        ".mp4" | ".webm" | ".mkv" => "film",
        ".png" | ".jpg" | ".jpeg" | ".gif" | ".webp" => "image",
        ".zip" | ".rar" | ".7z" => "archive",
        _ => "file",
    }
}

fn auto_link_urls(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    for line in text.split("<br>") {
        if !result.is_empty() {
            result.push_str("<br>");
        }
        for (i, word) in line.split_whitespace().enumerate() {
            if i > 0 { result.push(' '); }
            if word.starts_with("http://") || word.starts_with("https://") {
                result.push_str(&format!(
                    r#"<a href="{}" target="_blank" rel="noopener">{}</a>"#,
                    word, word
                ));
            } else {
                result.push_str(word);
            }
        }
    }
    result
}

/// Sanitize a string for use as a filesystem directory name.
/// Replaces characters that are invalid on Windows/Linux.
pub fn sanitize_folder_name(name: &str) -> String {
    let sanitized: String = name.chars().map(|c| {
        match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            _ => c,
        }
    }).collect();
    // Trim trailing dots/spaces (Windows restriction)
    sanitized.trim_end_matches(|c: char| c == '.' || c == ' ').to_string()
}
