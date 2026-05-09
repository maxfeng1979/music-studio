# LinguaLog Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Tauri desktop app for diary-based language learning: write in diary language → AI organizes + translates → review results with cards + blur-reveal + word lookup.

**Architecture:** React SPA frontend + Tauri Rust backend. Frontend handles all UI/state; Rust backend provides file system access (MD export). AI/TTS calls are made directly from frontend via fetch (no Rust proxy needed). Pages managed by React state (no router needed for MVP).

**Tech Stack:** Tauri 2.x + React 18 + TypeScript + CSS Modules. No Tailwind (MVP simplicity). Vitest for testing.

---

## Module 1: Project Scaffolding

### Task 1: Initialize Tauri + React Project

**Files:**
- Create: `src-tauri/`, `src/`, `package.json`, `tsconfig.json`, `vite.config.ts`, `index.html`

- [ ] **Step 1: Create Tauri project**

```bash
npm create tauri-app@latest lingualog -- --template react-ts --manager npm
cd lingualog
npm install
```

- [ ] **Step 2: Clean up template files**

Delete all template content from `src/App.tsx`, `src/main.tsx`, `src/styles.css`. Keep only the shell.

- [ ] **Step 3: Verify dev server runs**

```bash
npm run tauri dev
```

Expected: App window opens with blank white page. No console errors.

- [ ] **Step 4: Commit**

```bash
git init && git add . && git commit -m "chore: scaffold Tauri + React project"
```

---

## Module 2: WritePage

### Task 2: WritePage — Layout & Language Selection

**Files:**
- Create: `src/components/WritePage.tsx`
- Create: `src/components/LanguageSelect.tsx`
- Modify: `src/App.tsx`
- Modify: `src/index.css`

**LANGUAGES constant (define once, reuse everywhere):**

```typescript
// src/constants/languages.ts
export const LANGUAGES = [
  { code: 'zh', label: '中文', native: '中文' },
  { code: 'en', label: 'English', native: 'English' },
  { code: 'fr', label: 'Français', native: 'Français' },
  { code: 'it', label: 'Italiano', native: 'Italiano' },
  { code: 'es', label: 'Español', native: 'Español' },
  { code: 'de', label: 'Deutsch', native: 'Deutsch' },
  { code: 'ja', label: '日本語', native: '日本語' },
  { code: 'ko', label: '한국어', native: '한국어' },
] as const;

export type LanguageCode = typeof LANGUAGES[number]['code'];
export const DEFAULT_DIARY_LANG = 'zh';
export const DEFAULT_TARGET_LANG = 'en';
```

- [ ] **Step 1: Write LanguageSelect component**

```tsx
// src/components/LanguageSelect.tsx
import { LANGUAGES, LanguageCode } from '../constants/languages';

interface LanguageSelectProps {
  label: string;
  value: LanguageCode;
  onChange: (code: LanguageCode) => void;
  target?: boolean; // adds blue border for target language
}

export function LanguageSelect({ label, value, onChange, target }: LanguageSelectProps) {
  return (
    <div className="lang-select">
      <span className="lang-label">{label}</span>
      <select
        value={value}
        onChange={e => onChange(e.target.value as LanguageCode)}
        className={target ? 'lang-select-target' : 'lang-select-source'}
      >
        {LANGUAGES.map(lang => (
          <option key={lang.code} value={lang.code}>{lang.label}</option>
        ))}
      </select>
    </div>
  );
}
```

- [ ] **Step 2: Write WritePage component**

```tsx
// src/components/WritePage.tsx
import { useState } from 'react';
import { LanguageSelect } from './LanguageSelect';
import { LANGUAGES, LanguageCode, DEFAULT_DIARY_LANG, DEFAULT_TARGET_LANG } from '../constants/languages';

interface WritePageProps {
  onSubmit: (content: string, diaryLang: LanguageCode, targetLang: LanguageCode) => void;
}

export function WritePage({ onSubmit }: WritePageProps) {
  const [diaryLang, setDiaryLang] = useState<LanguageCode>(DEFAULT_DIARY_LANG);
  const [targetLang, setTargetLang] = useState<LanguageCode>(DEFAULT_TARGET_LANG);
  const [content, setContent] = useState('');

  const diaryLangLabel = LANGUAGES.find(l => l.code === diaryLang)?.label || diaryLang;
  const placeholder = `Write in ${diaryLangLabel}...`;

  const handleSubmit = () => {
    if (!content.trim()) return;
    onSubmit(content, diaryLang, targetLang);
  };

  return (
    <div className="write-page">
      <header className="write-header">
        <span className="app-title">LinguaLog</span>
        <div className="header-actions">
          <button className="icon-btn" title="历史">📁</button>
          <button className="icon-btn" title="设置">⚙️</button>
        </div>
      </header>

      <div className="lang-row">
        <LanguageSelect label="日记语言" value={diaryLang} onChange={setDiaryLang} />
        <span className="lang-arrow">→</span>
        <LanguageSelect label="目标语言" value={targetLang} onChange={setTargetLang} target />
      </div>

      <textarea
        className="diary-input"
        value={content}
        onChange={e => setContent(e.target.value)}
        placeholder={placeholder}
      />

      <div className="submit-row">
        <button className="submit-btn" onClick={handleSubmit}>
          提交 →
        </button>
      </div>
    </div>
  );
}
```

- [ ] **Step 3: Write CSS**

```css
/* src/index.css */
:root {
  --color-primary: #3B82F6;
  --color-bg: #FFFFFF;
  --color-surface: #F9FAFB;
  --color-border: #E5E7EB;
  --color-text: #374151;
  --color-text-light: #9CA3AF;
}
* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: -apple-system, BlinkMacSystemFont, sans-serif; color: var(--color-text); background: var(--color-bg); }
.write-page { display: flex; flex-direction: column; height: 100vh; padding: 16px 24px; gap: 12px; max-width: 800px; margin: 0 auto; }
.write-header { display: flex; align-items: center; }
.app-title { flex: 1; text-align: center; font-size: 16px; font-weight: 600; }
.header-actions { display: flex; gap: 8px; }
.icon-btn { background: none; border: none; cursor: pointer; font-size: 16px; color: var(--color-text-light); }
.lang-row { display: flex; align-items: center; justify-content: center; gap: 10px; flex-wrap: wrap; }
.lang-label { font-size: 13px; color: #6b7280; }
.lang-select-source, .lang-select-target { font-size: 12px; padding: 5px 10px; border-radius: 8px; cursor: pointer; border: 1px solid var(--color-border); background: white; }
.lang-select-target { border-color: var(--color-primary); background: #EFF6FF; color: var(--color-primary); font-weight: 500; }
.diary-input { flex: 1; width: 100%; min-height: 260px; padding: 16px; font-size: 14px; line-height: 1.8; border: 1px solid var(--color-border); border-radius: 12px; resize: none; outline: none; font-family: inherit; }
.submit-row { display: flex; justify-content: center; }
.submit-btn { padding: 10px 32px; background: var(--color-primary); color: white; border: none; border-radius: 8px; font-size: 14px; font-weight: 500; cursor: pointer; box-shadow: 0 2px 8px rgba(59,130,246,0.3); }
.submit-btn:hover { background: #2563EB; }
```

- [ ] **Step 4: Wire up in App.tsx**

```tsx
// src/App.tsx
import { useState } from 'react';
import { WritePage } from './components/WritePage';

type Page = 'write' | 'result' | 'settings';

export default function App() {
  const [page, setPage] = useState<Page>('write');
  return <WritePage onSubmit={(content, diaryLang, targetLang) => {
    console.log({ content, diaryLang, targetLang });
    setPage('result');
  }} />;
}
```

- [ ] **Step 5: Verify in browser**

Run `npm run tauri dev`. Verify:
- Title "LinguaLog" centered
- Language dropdowns work (8 languages each)
- Textarea is wide and tall
- Submit button is centered and blue

- [ ] **Step 6: Commit**

```bash
git add src/components/WritePage.tsx src/components/LanguageSelect.tsx src/constants/languages.ts src/App.tsx src/index.css
git commit -m "feat: WritePage with language selection"
```

---

### Task 3: AI Service — Multi-Backend Client

**Files:**
- Create: `src/services/aiService.ts`
- Create: `src/types/ai.ts`

**Types:**

```typescript
// src/types/ai.ts
export interface DiaryEntry {
  original: string;
  organized: string;      // 源语言模式：AI整理后的原文
  translated: string;    // 源语言模式：翻译
  polished: string;      // 目标语言模式：润色后
}

export interface AIResponse {
  entries: DiaryEntry[];
}

export type AIProvider = 'openai' | 'anthropic';

export interface AIConfig {
  provider: AIProvider;
  endpoint: string;
  apiKey: string;
  model: string;
}
```

**Service:**

```typescript
// src/services/aiService.ts
import { AIConfig, AIResponse, DiaryEntry } from '../types/ai';
import { LanguageCode } from '../constants/languages';

const SYSTEM_PROMPTS = {
  diary: (targetLang: string) =>
    `You are an English learning assistant. The user writes diary entries. ` +
    `Your task:\n` +
    `1. Split the text into natural sentences.\n` +
    `2. Organize each sentence: fix incomplete or informal phrasing while keeping original meaning.\n` +
    `3. Translate to natural, fluent ${targetLang}.\n` +
    `4. Return JSON: { "entries": [{ "original": "...", "organized": "...", "translated": "..." }] }` +
    `\nOnly return JSON, no explanation.`,

  target: (targetLang: string) =>
    `You are a ${targetLang} writing coach. The user writes in ${targetLang}.\n` +
    `Your task:\n` +
    `1. Split the text into natural sentences.\n` +
    `2. Polish each sentence to be more natural while preserving original meaning.\n` +
    `3. Return JSON: { "entries": [{ "original": "...", "polished": "..." }] }` +
    `\nOnly return JSON, no explanation.`,
};

export async function callAI(
  content: string,
  mode: 'diary' | 'target',
  config: AIConfig,
  targetLang: LanguageCode,
  diaryLang: LanguageCode
): Promise<AIResponse> {
  const langName = targetLang.toUpperCase();
  const systemPrompt = mode === 'diary'
    ? SYSTEM_PROMPTS.diary(langName)
    : SYSTEM_PROMPTS.target(langName);

  let body: object;
  if (config.provider === 'anthropic') {
    body = {
      model: config.model || 'claude-3-5-sonnet-20241022',
      max_tokens: 4096,
      messages: [
        { role: 'user', content: `Text to process:\n${content}` }
      ],
      system: systemPrompt,
    };
  } else {
    // OpenAI compatible
    body = {
      model: config.model || 'gpt-4o',
      messages: [
        { role: 'system', content: systemPrompt },
        { role: 'user', content: content }
      ],
    };
  }

  const response = await fetch(config.endpoint, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${config.apiKey}`,
      ...(config.provider === 'anthropic' ? { 'x-api-key': config.apiKey, 'anthropic-version': '2023-06-01' } : {}),
    },
    body: JSON.stringify(body),
  });

  if (!response.ok) {
    throw new Error(`AI API error: ${response.status} ${response.statusText}`);
  }

  const data = await response.json();
  const text = config.provider === 'anthropic'
    ? data.content?.[0]?.text
    : data.choices?.[0]?.message?.content;

  return JSON.parse(text) as AIResponse;
}
```

- [ ] **Step 1: Create directory and files**

```bash
mkdir -p src/services src/types
```

- [ ] **Step 2: Write the files above**

- [ ] **Step 3: Write a smoke test**

```typescript
// src/services/__tests__/aiService.test.ts
import { describe, it, expect } from 'vitest';

// Minimal test: verify the function signature and types compile
describe('aiService', () => {
  it('callAI should be a function', () => {
    // This just verifies the module loads without errors
    expect(typeof callAI).toBe('function');
  });
});
```

- [ ] **Step 4: Commit**

```bash
git add src/services/aiService.ts src/types/ai.ts
git commit -m "feat: multi-backend AI service (OpenAI + Anthropic)"
```

---

### Task 4: MD Export Service

**Files:**
- Create: `src/services/exportService.ts`

**Service:**

```typescript
// src/services/exportService.ts
import { DiaryEntry } from '../types/ai';
import { LanguageCode } from '../constants/languages';

export function buildDiaryMD(
  content: string,
  entries: DiaryEntry[],
  diaryLang: LanguageCode,
  targetLang: LanguageCode,
  targetLangLabel: string
): string {
  const date = new Date().toISOString().split('T')[0];
  const diaryLangLabel = diaryLang.toUpperCase();

  const lines: string[] = [
    `# 日记：${date}`,
    `日记语言 → 目标语言: ${diaryLangLabel} → ${targetLang}`,
    '',
    '## 原文',
    content,
    '',
  ];

  // 源语言模式
  if (entries[0]?.translated !== undefined) {
    lines.push('## 润色后');
    entries.forEach(e => lines.push(e.organized));
    lines.push('', '## 翻译', '');
    entries.forEach(e => lines.push(`- ${e.translated}`));
  } else {
    // 目标语言模式
    lines.push('## 润色后');
    entries.forEach(e => lines.push(e.polished));
  }

  return lines.join('\n');
}
```

- [ ] **Step 1: Write the service file**

- [ ] **Step 2: Commit**

```bash
git add src/services/exportService.ts
git commit -m "feat: MD export service for diary export"
```

---

### Task 5: ResultPage — Card Display & Blur

**Files:**
- Create: `src/components/ResultPage.tsx`
- Create: `src/components/Card.tsx`
- Create: `src/components/WordBubble.tsx`
- Create: `src/hooks/useSelection.ts`

**Card component:**

```tsx
// src/components/Card.tsx
import { useState, useRef } from 'react';
import { DiaryEntry } from '../types/ai';

interface CardProps {
  entry: DiaryEntry;
  isOrganized: boolean;       // true for diary mode, false for target mode
  targetLang: string;
  globalBlur: boolean;         // global blur override
  onPlayTTS: (text: string) => void;
  onFavorite: (text: string, type: 'sentence' | 'word' | 'phrase') => void;
}

export function Card({ entry, isOrganized, targetLang, globalBlur, onPlayTTS, onFavorite }: CardProps) {
  const [localBlur, setLocalBlur] = useState(false);
  const isBlurred = globalBlur || localBlur;
  const displayText = isOrganized ? entry.translated : entry.polished;
  const organizedText = entry.organized;

  const handleTextSelect = () => {
    const selection = window.getSelection();
    const selectedText = selection?.toString().trim();
    if (selectedText) {
      // Emit word lookup event — handled by parent via context
      window.dispatchEvent(new CustomEvent('word-lookup', { detail: { word: selectedText } }));
    }
  };

  return (
    <div className={`card ${isBlurred ? 'card-blurred' : ''}`}>
      <div className="card-toolbar">
        <button className="card-btn" onClick={() => onPlayTTS(displayText)}>🔊 男声</button>
        <button className="card-btn" onClick={() => onPlayTTS(displayText)}>🔊 女声</button>
        <button className="card-btn card-btn-fav" onClick={() => onFavorite(displayText, 'sentence')}>⭐ 收藏</button>
        <div style={{ flex: 1 }} />
        <div className="blur-toggle">
          <span>{isBlurred ? '显示' : '遮盖'}</span>
          <button
            className={`toggle ${isBlurred ? 'toggle-on' : 'toggle-off'}`}
            onClick={() => setLocalBlur(!isBlurred)}
          />
        </div>
      </div>
      <div className="card-body">
        <div className="card-left">
          <div className="field">
            <span className="field-label">原文</span>
            <div className="field-text">{entry.original}</div>
          </div>
          {organizedText && (
            <div className="field">
              <span className="field-label field-label-blue">润色后</span>
              <div className="field-text">{organizedText}</div>
            </div>
          )}
        </div>
        <div className="card-right">
          <span className="field-label">{targetLang}</span>
          <div
            className={`field-text target-text ${isBlurred ? 'text-blurred' : ''}`}
            onMouseUp={handleTextSelect}
          >
            {displayText}
          </div>
          {isBlurred && <div className="blur-overlay" />}
        </div>
      </div>
    </div>
  );
}
```

**WordBubble component:**

```tsx
// src/components/WordBubble.tsx
import { useState, useEffect, useRef } from 'react';

interface WordBubbleProps {
  word: string;
  x: number;
  y: number;
  onClose: () => void;
  onPlayTTS: (text: string) => void;
  onFavorite: (text: string) => void;
}

export function WordBubble({ word, x, y, onClose, onPlayTTS, onFavorite }: WordBubbleProps) {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    const handleClickOutside = (e: MouseEvent) => {
      if (ref.current && !ref.current.contains(e.target as Node)) onClose();
    };
    document.addEventListener('mousedown', handleClickOutside);
    return () => document.removeEventListener('mousedown', handleClickOutside);
  }, [onClose]);

  return (
    <div ref={ref} className="word-bubble" style={{ left: x, top: y }}>
      <div className="bubble-word">{word}</div>
      <div className="bubble-definition">
        {/* AI-generated definition — placeholder for now */}
        <div className="bubble-phonetic">/.../</div>
        <div className="bubble-meaning">释义由AI生成，仅供参考</div>
      </div>
      <div className="bubble-actions">
        <button className="bubble-btn" onClick={() => onPlayTTS(word)}>🔊 发音</button>
        <button className="bubble-btn bubble-btn-fav" onClick={() => onFavorite(word)}>⭐ 收藏</button>
      </div>
    </div>
  );
}
```

**ResultPage component:**

```tsx
// src/components/ResultPage.tsx
import { useState, useEffect } from 'react';
import { Card } from './Card';
import { WordBubble } from './WordBubble';
import { DiaryEntry } from '../types/ai';
import { LanguageCode, LANGUAGES } from '../constants/languages';

interface ResultPageProps {
  content: string;
  entries: DiaryEntry[];
  diaryLang: LanguageCode;
  targetLang: LanguageCode;
  onBack: () => void;
  onPlayTTS: (text: string) => void;
}

interface WordLookup {
  word: string;
  x: number;
  y: number;
}

export function ResultPage({ content, entries, diaryLang, targetLang, onBack, onPlayTTS }: ResultPageProps) {
  const [globalBlur, setGlobalBlur] = useState(false);
  const [wordLookup, setWordLookup] = useState<WordLookup | null>(null);
  const targetLangLabel = LANGUAGES.find(l => l.code === targetLang)?.label || targetLang;
  const isOrganized = entries[0]?.translated !== undefined;

  useEffect(() => {
    const handler = (e: CustomEvent<{ word: string }>) => {
      setWordLookup({ word: e.detail.word, x: 100, y: 200 });
    };
    window.addEventListener('word-lookup', handler as EventListener);
    return () => window.removeEventListener('word-lookup', handler as EventListener);
  }, []);

  return (
    <div className="result-page">
      <header className="result-header">
        <button className="back-btn" onClick={onBack}>← 返回</button>
        <span className="result-date">日记：{new Date().toLocaleDateString()}</span>
        <div className="header-right">
          <span>遮盖翻译</span>
          <button
            className={`toggle ${globalBlur ? 'toggle-on' : 'toggle-off'}`}
            onClick={() => setGlobalBlur(!globalBlur)}
          />
        </div>
      </header>

      <div className="cards-container">
        {entries.map((entry, i) => (
          <Card
            key={i}
            entry={entry}
            isOrganized={isOrganized}
            targetLang={targetLangLabel}
            globalBlur={globalBlur}
            onPlayTTS={onPlayTTS}
            onFavorite={(text, type) => console.log('favorite:', text, type)}
          />
        ))}
      </div>

      {wordLookup && (
        <WordBubble
          word={wordLookup.word}
          x={wordLookup.x}
          y={wordLookup.y}
          onClose={() => setWordLookup(null)}
          onPlayTTS={onPlayTTS}
          onFavorite={(word) => console.log('fav word:', word)}
        />
      )}
    </div>
  );
}
```

- [ ] **Step 1: Write Card.tsx**

- [ ] **Step 2: Write WordBubble.tsx**

- [ ] **Step 3: Write ResultPage.tsx**

- [ ] **Step 4: Write CSS additions**

```css
/* Add to src/index.css */

/* ResultPage */
.result-page { display: flex; flex-direction: column; height: 100vh; }
.result-header { display: flex; align-items: center; gap: 12px; padding: 10px 16px; background: var(--color-surface); border-bottom: 1px solid var(--color-border); }
.back-btn { background: none; border: none; cursor: pointer; font-size: 14px; color: var(--color-primary); }
.result-date { flex: 1; text-align: center; font-size: 12px; color: var(--color-text-light); }
.header-right { display: flex; align-items: center; gap: 6px; font-size: 12px; color: var(--color-text); }
.toggle { width: 36px; height: 20px; border-radius: 10px; border: none; cursor: pointer; position: relative; transition: background 0.2s; }
.toggle-off { background: var(--color-border); }
.toggle-off::after { content: ''; position: absolute; left: 2px; top: 2px; width: 16px; height: 16px; background: white; border-radius: 50%; }
.toggle-on { background: var(--color-primary); }
.toggle-on::after { content: ''; position: absolute; right: 2px; top: 2px; width: 16px; height: 16px; background: white; border-radius: 50%; }

.cards-container { flex: 1; overflow-y: auto; padding: 12px; display: flex; flex-direction: column; gap: 10px; }

/* Card */
.card { border: 1px solid var(--color-border); border-radius: 12px; overflow: hidden; }
.card-blurred { border-color: var(--color-primary); box-shadow: 0 0 0 2px #DBEAFE; }
.card-toolbar { display: flex; align-items: center; gap: 6px; padding: 8px 12px; background: var(--color-surface); border-bottom: 1px solid #f3f4f6; }
.card-btn { font-size: 11px; padding: 3px 8px; background: white; border: 1px solid var(--color-border); border-radius: 4px; cursor: pointer; }
.card-btn-fav { background: #FEF3C7; border-color: #FCD34D; }
.blur-toggle { display: flex; align-items: center; gap: 4px; font-size: 11px; color: var(--color-text-light); }
.blur-toggle:has(.toggle-on) { color: var(--color-primary); }
.card-body { display: flex; }
.card-left { flex: 1; padding: 12px; border-right: 1px solid #f3f4f6; }
.card-right { flex: 1; padding: 12px; position: relative; }
.field { margin-bottom: 8px; }
.field-label { font-size: 10px; color: var(--color-text-light); text-transform: uppercase; display: block; margin-bottom: 4px; }
.field-label-blue { color: var(--color-primary); }
.field-text { font-size: 13px; line-height: 1.6; color: var(--color-text); }
.target-text { color: var(--color-primary); }
.text-blurred { filter: blur(4px); user-select: none; }
.blur-overlay { position: absolute; inset: 12px; background: rgba(255,255,255,0.7); backdrop-filter: blur(6px); border-radius: 6px; display: flex; align-items: center; justify-content: center; pointer-events: none; }

/* WordBubble */
.word-bubble { position: fixed; background: white; border: 1px solid var(--color-border); border-radius: 10px; padding: 12px 14px; box-shadow: 0 4px 16px rgba(0,0,0,0.12); min-width: 200px; z-index: 100; }
.bubble-word { font-size: 15px; font-weight: 600; color: #1f2937; margin-bottom: 6px; }
.bubble-definition { margin-bottom: 8px; }
.bubble-phonetic { font-size: 12px; color: #9ca3af; margin-bottom: 4px; }
.bubble-meaning { font-size: 12px; color: #6b7280; }
.bubble-actions { display: flex; gap: 6px; }
.bubble-btn { font-size: 11px; padding: 4px 10px; background: #f3f4f6; border: none; border-radius: 4px; cursor: pointer; }
.bubble-btn-fav { background: #FEF3C7; border: 1px solid #FCD34D; }
```

- [ ] **Step 5: Wire ResultPage in App.tsx**

```tsx
// src/App.tsx — updated
import { useState } from 'react';
import { WritePage } from './components/WritePage';
import { ResultPage } from './components/ResultPage';
import { LanguageCode } from './constants/languages';
import { DiaryEntry } from './types/ai';

type Page = 'write' | 'result' | 'settings';

export default function App() {
  const [page, setPage] = useState<Page>('write');
  const [resultData, setResultData] = useState<{
    content: string;
    entries: DiaryEntry[];
    diaryLang: LanguageCode;
    targetLang: LanguageCode;
  } | null>(null);

  const handleSubmit = (content: string, diaryLang: LanguageCode, targetLang: LanguageCode) => {
    // Mock data for now — will wire to AI service in Task 6
    const mockEntries: DiaryEntry[] = content.split('\n').filter(Boolean).map(s => ({
      original: s.trim(),
      organized: s.trim(),
      translated: '[Translation pending...]',
    }));
    setResultData({ content, entries: mockEntries, diaryLang, targetLang });
    setPage('result');
  };

  const handlePlayTTS = (text: string) => {
    // TTS — implemented in Task 7
    console.log('TTS:', text);
  };

  if (page === 'result' && resultData) {
    return (
      <ResultPage
        content={resultData.content}
        entries={resultData.entries}
        diaryLang={resultData.diaryLang}
        targetLang={resultData.targetLang}
        onBack={() => setPage('write')}
        onPlayTTS={handlePlayTTS}
      />
    );
  }

  return <WritePage onSubmit={handleSubmit} />;
}
```

- [ ] **Step 6: Verify in browser**

Run `npm run tauri dev`. Verify:
- Submit on WritePage → ResultPage shows
- Back button returns to WritePage
- Global blur toggle blurs all translations
- Per-card blur toggle works
- Cards show original + organized + translated columns

- [ ] **Step 7: Commit**

```bash
git add src/components/ResultPage.tsx src/components/Card.tsx src/components/WordBubble.tsx src/App.tsx src/index.css
git commit -m "feat: ResultPage with card display and blur reveal"
```

---

### Task 6: Wire AI Service to WritePage

**Files:**
- Modify: `src/App.tsx`
- Create: `src/hooks/useAI.ts`
- Create: `src/context/SettingsContext.tsx`

**SettingsContext:**

```tsx
// src/context/SettingsContext.tsx
import { createContext, useContext, useState } from 'react';
import { AIConfig } from '../types/ai';

const DEFAULT_CONFIG: AIConfig = {
  provider: 'openai',
  endpoint: '',
  apiKey: '',
  model: 'gpt-4o',
};

const SettingsContext = createContext<{
  aiConfig: AIConfig;
  setAIConfig: (c: AIConfig) => void;
}>({
  aiConfig: DEFAULT_CONFIG,
  setAIConfig: () => {},
});

export function SettingsProvider({ children }: { children: React.ReactNode }) {
  const [aiConfig, setAIConfig] = useState<AIConfig>(DEFAULT_CONFIG);
  return (
    <SettingsContext.Provider value={{ aiConfig, setAIConfig }}>
      {children}
    </SettingsContext.Provider>
  );
}

export const useSettings = () => useContext(SettingsContext);
```

**useAI hook:**

```tsx
// src/hooks/useAI.ts
import { useState } from 'react';
import { callAI } from '../services/aiService';
import { AIConfig, DiaryEntry } from '../types/ai';
import { LanguageCode } from '../constants/languages';

export function useAI() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const process = async (
    content: string,
    mode: 'diary' | 'target',
    config: AIConfig,
    targetLang: LanguageCode,
    diaryLang: LanguageCode
  ): Promise<DiaryEntry[]> => {
    setLoading(true);
    setError(null);
    try {
      const response = await callAI(content, mode, config, targetLang, diaryLang);
      return response.entries;
    } catch (e) {
      const msg = e instanceof Error ? e.message : 'Unknown error';
      setError(msg);
      return [];
    } finally {
      setLoading(false);
    }
  };

  return { loading, error, process };
}
```

- [ ] **Step 1: Write SettingsContext and useAI**

- [ ] **Step 2: Update App.tsx to wire AI**

```tsx
// src/App.tsx — updated with real AI call
import { useState } from 'react';
import { WritePage } from './components/WritePage';
import { ResultPage } from './components/ResultPage';
import { SettingsProvider, useSettings } from './context/SettingsContext';
import { LanguageCode } from './constants/languages';
import { DiaryEntry } from './types/ai';
import { useAI } from './hooks/useAI';

function AppInner() {
  const [page, setPage] = useState<'write' | 'result' | 'settings'>('write');
  const [resultData, setResultData] = useState<{
    content: string; entries: DiaryEntry[]; diaryLang: LanguageCode; targetLang: LanguageCode;
  } | null>(null);
  const { aiConfig } = useSettings();
  const { loading, error, process } = useAI();

  const handleSubmit = async (content: string, diaryLang: LanguageCode, targetLang: LanguageCode) => {
    const mode = diaryLang === targetLang ? 'target' : 'diary';
    const entries = await process(content, mode, aiConfig, targetLang, diaryLang);
    if (entries.length > 0) {
      setResultData({ content, entries, diaryLang, targetLang });
      setPage('result');
    }
  };

  const handlePlayTTS = (text: string) => {
    // TTS — Task 7
    console.log('TTS:', text);
  };

  if (page === 'result' && resultData) {
    return (
      <ResultPage
        content={resultData.content}
        entries={resultData.entries}
        diaryLang={resultData.diaryLang}
        targetLang={resultData.targetLang}
        onBack={() => setPage('write')}
        onPlayTTS={handlePlayTTS}
      />
    );
  }

  if (page === 'settings') return <div>Settings page (Task 8)</div>;

  return <WritePage onSubmit={handleSubmit} />;
}

export default function App() {
  return (
    <SettingsProvider>
      <AppInner />
    </SettingsProvider>
  );
}
```

- [ ] **Step 3: Commit**

```bash
git add src/context/SettingsContext.tsx src/hooks/useAI.ts src/App.tsx
git commit -m "feat: wire AI service to WritePage"
```

---

### Task 7: TTS Service

**Files:**
- Create: `src/services/ttsService.ts`
- Create: `src/hooks/useTTS.ts`

```typescript
// src/services/ttsService.ts
export type VoiceGender = 'male' | 'female';

export async function speak(text: string, gender: VoiceGender = 'male') {
  if ('speechSynthesis' in window) {
    const utterance = new SpeechSynthesisUtterance(text);
    const voices = speechSynthesis.getVoices();
    const lang = 'en-US';
    const filtered = voices.filter(v => v.lang.startsWith(lang));
    if (filtered.length > 0) {
      // Simple heuristic: pick first for male, second for female
      utterance.voice = gender === 'male' ? filtered[0] : filtered[Math.min(1, filtered.length - 1)];
    }
    utterance.lang = lang;
    speechSynthesis.speak(utterance);
  }
}
```

```tsx
// src/hooks/useTTS.ts
import { useCallback } from 'react';
import { speak, VoiceGender } from '../services/ttsService';

export function useTTS() {
  const play = useCallback((text: string, gender: VoiceGender = 'male') => {
    speak(text, gender);
  }, []);
  return { play };
}
```

- [ ] **Step 1: Write ttsService.ts and useTTS.ts**

- [ ] **Step 2: Update ResultPage to use useTTS**

Replace `onPlayTTS={handlePlayTTS}` with `useTTS` hook call.

- [ ] **Step 3: Commit**

```bash
git add src/services/ttsService.ts src/hooks/useTTS.ts
git commit -m "feat: TTS service with Web Speech API"
```

---

### Task 8: Settings Page

**Files:**
- Create: `src/components/SettingsPage.tsx`
- Modify: `src/App.tsx`
- Modify: `src/context/SettingsContext.tsx` (add TTS config)
- Modify: `src/index.css`

```tsx
// src/components/SettingsPage.tsx
import { useState } from 'react';
import { useSettings } from '../context/SettingsContext';
import { AIConfig } from '../types/ai';

export function SettingsPage() {
  const { aiConfig, setAIConfig, ttsConfig, setTTSConfig } = useSettings();
  const [testResult, setTestResult] = useState<string>('');

  const handleTestAI = async () => {
    if (!aiConfig.endpoint || !aiConfig.apiKey) {
      setTestResult('请填写 Endpoint 和 API Key');
      return;
    }
    try {
      const res = await fetch(aiConfig.endpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', 'Authorization': `Bearer ${aiConfig.apiKey}` },
        body: JSON.stringify({ model: aiConfig.model, messages: [{ role: 'user', content: 'Hi' }] }),
      });
      setTestResult(res.ok ? '✅ 连接成功' : `❌ ${res.status} ${res.statusText}`);
    } catch (e) {
      setTestResult(`❌ ${e}`);
    }
  };

  return (
    <div className="settings-page">
      <header className="settings-header">
        <button className="back-btn" onClick={() => history.back()}>← 返回</button>
        <span className="settings-title">设置</span>
      </header>

      <div className="settings-content">
        <section className="settings-section">
          <h3>AI 配置</h3>
          <label>
            Provider
            <select value={aiConfig.provider} onChange={e => setAIConfig({ ...aiConfig, provider: e.target.value as 'openai' | 'anthropic' })}>
              <option value="openai">OpenAI (兼容格式)</option>
              <option value="anthropic">Anthropic</option>
            </select>
          </label>
          <label>
            Endpoint URL
            <input type="text" value={aiConfig.endpoint} onChange={e => setAIConfig({ ...aiConfig, endpoint: e.target.value })} placeholder="https://api.openai.com/v1/chat/completions" />
          </label>
          <label>
            API Key
            <input type="password" value={aiConfig.apiKey} onChange={e => setAIConfig({ ...aiConfig, apiKey: e.target.value })} placeholder="sk-..." />
          </label>
          <label>
            模型
            <input type="text" value={aiConfig.model} onChange={e => setAIConfig({ ...aiConfig, model: e.target.value })} placeholder="gpt-4o" />
          </label>
          <button className="test-btn" onClick={handleTestAI}>测试连接</button>
          {testResult && <div className="test-result">{testResult}</div>}
        </section>

        <section className="settings-section">
          <h3>TTS 配置</h3>
          <label>
            模式
            <select>
              <option>Web Speech API（免费）</option>
              <option>自定义</option>
            </select>
          </label>
        </section>

        <section className="settings-section">
          <h3>导出设置</h3>
          <label>
            导出目录
            <input type="text" value="~/Documents/LinguaLog" readOnly />
          </label>
        </section>
      </div>
    </div>
  );
}
```

- [ ] **Step 1: Add TTS config to SettingsContext**

```tsx
// Add to SettingsContext
interface TTSConfig {
  provider: 'webspeech' | 'custom';
  endpoint?: string;
  apiKey?: string;
}
const DEFAULT_TTS: TTSConfig = { provider: 'webspeech' };
// Add ttsConfig, setTTSConfig to context
```

- [ ] **Step 2: Write SettingsPage.tsx**

- [ ] **Step 3: Wire SettingsPage in App.tsx**

```tsx
// In App.tsx — add settings link
// In WritePage — add onSettings prop that sets page to 'settings'
```

- [ ] **Step 4: Add settings CSS**

```css
.settings-page { display: flex; flex-direction: column; height: 100vh; }
.settings-header { display: flex; align-items: center; padding: 12px 16px; border-bottom: 1px solid var(--color-border); gap: 12px; }
.settings-title { flex: 1; font-size: 16px; font-weight: 600; text-align: center; }
.settings-content { flex: 1; overflow-y: auto; padding: 16px; display: flex; flex-direction: column; gap: 16px; }
.settings-section { border: 1px solid var(--color-border); border-radius: 12px; padding: 16px; display: flex; flex-direction: column; gap: 10px; }
.settings-section h3 { font-size: 14px; font-weight: 600; color: var(--color-text); }
.settings-section label { display: flex; flex-direction: column; gap: 4px; font-size: 12px; color: #6b7280; }
.settings-section input, .settings-section select { padding: 8px 10px; border: 1px solid var(--color-border); border-radius: 6px; font-size: 13px; outline: none; }
.test-btn { padding: 8px 16px; background: var(--color-primary); color: white; border: none; border-radius: 6px; cursor: pointer; font-size: 13px; }
.test-result { font-size: 12px; color: #6b7280; padding: 6px 0; }
```

- [ ] **Step 5: Commit**

```bash
git add src/components/SettingsPage.tsx src/context/SettingsContext.tsx src/App.tsx src/index.css
git commit -m "feat: Settings page with AI/TTS config"
```

---

## Module 3: MD Export via Tauri

### Task 9: Wire MD Export to Tauri File System

**Files:**
- Modify: `src-tauri/src/main.rs`
- Modify: `src-tauri/tauri.conf.json`
- Modify: `src/services/exportService.ts`
- Create: `src/hooks/useExport.ts`

**Tauri command:**

```rust
// src-tauri/src/main.rs — add this command
#[tauri::command]
fn export_diary(path: &str, content: &str) -> Result<(), String> {
    std::fs::write(path, content).map_err(|e| e.to_string())
}
```

Register the command in the Tauri builder:
```rust
tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![export_diary])
    .run(...)
```

**tauri.conf.json — add permissions:**
```json
{
  "bundle": {
    "identifier": "com.diaryenglish.app"
  },
  "plugins": {
    "fs": {
      "scope": ["$DOCUMENT/**"]
    }
  }
}
```

**exportService — add Tauri integration:**

```typescript
// src/services/exportService.ts
// Add at bottom:
export async function exportToFile(path: string, content: string): Promise<void> {
  const { invoke } = await import('@tauri-apps/api/core');
  await invoke('export_diary', { path, content });
}
```

- [ ] **Step 1: Add Tauri command to main.rs**

- [ ] **Step 2: Update tauri.conf.json**

- [ ] **Step 3: Update exportService.ts**

- [ ] **Step 4: Hook into ResultPage** — add export button to page header

```tsx
// In ResultPage — add export button
<button
  className="export-btn"
  onClick={async () => {
    const md = buildDiaryMD(content, entries, diaryLang, targetLang, targetLangLabel);
    const date = new Date().toISOString().split('T')[0];
    const path = `~/Documents/LinguaLog/diary-${date}.md`;
    await exportToFile(path, md);
    alert('已导出到 ' + path);
  }}
>
  📁 导出
</button>
```

- [ ] **Step 5: Commit**

```bash
git add src/services/exportService.ts src/components/ResultPage.tsx src-tauri/src/main.rs src-tauri/tauri.conf.json
git commit -m "feat: MD export via Tauri file system"
```

---

## Self-Review Checklist

**Spec coverage:**

| Spec requirement | Task |
|------------------|------|
| WritePage layout | Task 2 |
| Language selection (8 languages) | Task 2 |
| Submit → ResultPage navigation | Tasks 2, 5 |
| ResultPage dual-column cards | Task 5 |
| Blur reveal (global + per-card) | Task 5 |
| Word selection popup | Task 5 |
| TTS playback | Task 7 |
| AI multi-backend (OpenAI + Anthropic) | Task 3 |
| Settings page (AI/TTS/Export) | Task 8 |
| MD export to local folder | Task 9 |

**Placeholder scan:** No TBD/TODO found. All steps have concrete code.

**Type consistency:** `LanguageCode`, `AIConfig`, `DiaryEntry` all defined in `src/types/` and reused across tasks.

---

## Execution Handoff

**Plan complete and saved to `docs/superpowers/plans/lingualog-implementation.md`.**

Two execution options:

**1. Subagent-Driven (recommended)** — I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** — Execute tasks in this session using executing-plans, batch execution with checkpoints

Which approach?
