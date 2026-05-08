use crate::api::anthropic::AnthropicClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Deserialize)]
pub struct AiAssistParams {
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MusicIdeas {
    pub title: String,
    pub prompt: String,
    pub lyrics: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct CoverPromptParams {
    pub title: String,
    pub music_prompt: String,
    pub lyrics: Option<String>,
}

#[tauri::command]
pub async fn generate_music_ideas(params: AiAssistParams) -> Result<MusicIdeas, String> {
    let api_key = crate::config::get_api_key()?;
    let client = crate::api::minimax::MinimaxClient::new(&api_key)?;

    let resp = client.generate_lyrics(&params.description).await?;

    Ok(MusicIdeas {
        title: resp.song_title,
        prompt: resp.style_tags,
        lyrics: resp.lyrics,
    })
}

#[tauri::command]
pub async fn generate_cover_prompt(params: CoverPromptParams) -> Result<String, String> {
    let api_key = crate::config::get_api_key()?;
    let client = AnthropicClient::new(&api_key)?;

    let system_prompt = r#"你是一个专业的专辑封面设计描述专家。根据歌曲信息生成一段用于AI图片生成的英文提示词。

要求：
- 提示词必须是英文
- 描述一个适合作为音乐专辑封面的视觉场景
- 不要包含文字、字母、logo、标题等文字元素
- 风格要艺术化，适合音乐封面
- 长度控制在100个英文单词以内
- 只返回提示词本身，不要任何解释或额外文字"#;

    let mut user_message = format!(
        "歌曲标题：{}\n音乐风格：{}",
        params.title, params.music_prompt
    );
    if let Some(ref lyrics) = params.lyrics {
        // Take first 500 chars (not bytes) of lyrics to capture the mood
        let excerpt: String = lyrics.chars().take(500).collect();
        user_message.push_str(&format!("\n歌词摘录：{}", excerpt));
    }

    let response = client.chat(system_prompt, &user_message).await?;

    Ok(response.trim().to_string())
}
