import { invoke } from "@tauri-apps/api/core";

window.addEventListener("DOMContentLoaded", () => {
  const textarea = document.querySelector("textarea") as HTMLTextAreaElement;

  textarea.addEventListener("keyup", async (e: KeyboardEvent) => {
    if (e.key === "Enter" && e.ctrlKey) {
      e.preventDefault();

      const content = textarea.value.trim();
      const triggerRegex = /(.*?)\s*--(m|tr|py|cpp)$/;
      const match = content.match(triggerRegex);
      if (!match) return;

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
  });
});
