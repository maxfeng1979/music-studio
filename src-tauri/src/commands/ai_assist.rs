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
    let client = AnthropicClient::new(&api_key)?;

    let system_prompt = r#"你是一个专业的音乐创作助手。用户会描述他们想要的音乐，你需要生成：
1. 歌曲标题（title）
2. 音乐风格描述（prompt）- 用于音乐生成API的风格输入，用逗号分隔的关键词，200字符以内
3. 歌词（lyrics）- 使用结构标签格式

歌词必须使用以下标签来标记歌曲结构：
[Intro], [Verse], [Pre Chorus], [Chorus], [Interlude], [Bridge], [Outro], [Post Chorus], [Transition], [Break], [Hook], [Build Up], [Inst], [Solo]

规则：
- 歌词总长度不超过3000字符
- prompt使用逗号分隔的音乐风格关键词，例如："独立民谣,忧郁,内省,渴望,独自漫步,咖啡馆"
- 如果用户描述的是纯音乐/器乐，歌词部分留空
- 歌词要有完整结构：至少包含[Intro]、[Verse]、[Chorus]
- 歌词语言跟用户描述的语言一致

你必须严格按以下JSON格式返回，不要包含任何其他文字：
{"title": "歌曲标题", "prompt": "风格关键词", "lyrics": "带标签的歌词"}"#;

    let user_message = format!("用户想要的音乐：{}", params.description);

    let response = client.chat(system_prompt, &user_message).await?;

    // Parse JSON from response - handle potential markdown code blocks
    let json_str = response
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let ideas: MusicIdeas = serde_json::from_str(json_str)
        .map_err(|e| format!("Failed to parse AI response: {}. Response: {}", e, json_str))?;

    Ok(ideas)
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
        // Take first 500 chars of lyrics to capture the mood
        let excerpt = if lyrics.len() > 500 { &lyrics[..500] } else { lyrics };
        user_message.push_str(&format!("\n歌词摘录：{}", excerpt));
    }

    let response = client.chat(system_prompt, &user_message).await?;

    Ok(response.trim().to_string())
}
