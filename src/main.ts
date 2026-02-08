import { invoke } from "@tauri-apps/api/core";

window.addEventListener("DOMContentLoaded", () => {
  const textarea = document.querySelector("textarea") as HTMLTextAreaElement;

  let lastState: string | null = null; // 用于存储替换前的文本快照

  textarea.addEventListener("keyup", async (e: KeyboardEvent) => {
    if (e.key === "Enter" && e.ctrlKey) {
      e.preventDefault();

      const content = textarea.value;
      const triggerRegex = /(.*?)\s*--(m|tr|py|cpp)$/;
      const match = content.match(triggerRegex);

      if (!match) return;
      lastState = content; // 处理之前保存当前完整内容快照

      const term = match[1].trim();
      const mode = match[2];
      textarea.disabled = true;

      let word: string;
      switch (mode) {
        case "m":
          word = "Parsing";
          break;
        case "tr":
          word = "Translating";
          break;
        default:
          word = "Processing";
          break;
      }

      textarea.value = content.replace(
        triggerRegex,
        `${word} "${term}"...`
      );

      try {
        const result = await invoke<string>("call_deepseek", { term, mode });
        textarea.value = content.replace(
          triggerRegex,
          `${result}`
        );
      } catch (error) {
        textarea.value = content.replace(
          triggerRegex,
          `Error: ${String(error)}`
        );
      } finally {
        textarea.disabled = false;
        textarea.focus();
      }
    }

    // 快照撤销/切换
    if (e.key == "h" && e.ctrlKey) {
      e.preventDefault();
      if (lastState != null) {
        const currentState = textarea.value;
        textarea.value = lastState;
        lastState = currentState; // 更新本次快照为刚刚处理结果，实现类似切换效果
      } 
    }
  });
});
