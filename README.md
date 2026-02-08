# PromptSticker

PromptSticker 是一个专为 AI 提示词（Prompt）创作流程设计的极简置顶记事本，
让你在写 Prompt 时专注、屏息、并能随时调用 AI 辅助能力，提升 Prompt 迭代效率。
（PromptSticker is a minimalist, top-mounted notepad designed specifically for the AI-powered prompt creation workflow.
It allows you to focus and concentrate while writing prompts, and provides AI assistance at any time, improving the efficiency of prompt iteration.）


### 特性（Features）
* **心流体验（Flow Experience）**：极简界面，无标题栏干扰。（Minimalist interface, no title bar distractions.）
* **瞬间隐身（Instant Hiding）**：按下 `F2` 全局隐藏/显示，保护隐私，无缝切换。（Press `F2` to globally hide/show, protecting privacy and seamlessly switching.）
* **置顶透明（Top-Mounted Transparency）**：磨砂玻璃质感，永远浮在浏览器上方，边看 AI 输出边优化。（Frosted glass texture, always floating above the browser, optimized while viewing AI output.）
* **原生性能（Native Performance）**：基于 Tauri 2.0 + Rust 开发，内存占用极低。（Developed based on Tauri 2.0 + Rust, with extremely low memory usage.）
* **即时翻译（Instant Translation）**：通过简洁指令完成中英文互译，无需离开当前上下文。（Translate between Chinese and English via concise commands without leaving the current context.）
* **概念速释（Concept Clarification）**：快速获取技术概念的直观解释，专注理解而非搜索。（Get concise explanations of technical concepts, focusing on understanding rather than searching.）
* **代码速查（Code & API Lookup）**：查询 Python / C++ 中最常用的类或函数，参数与示例一目了然。（Look up commonly used classes or functions in Python/C++, with parameters and minimal examples at a glance.）


### 快捷键（Keyboard Shortcuts）
* `F2`：全局隐藏/显示窗口（Globally hide/show window）
* 在文本末尾添加指令后缀，然后按下 `Ctrl + Enter` 触发 AI 操作：
  * `--tr`：中英文瞬间互译（Instant Translation）
  * `--m`：技术概念解释（Concept Clarification）
  * `--py` / `--cpp`：查询常用类/函数用法（Code & API Lookup）
* `鼠标拖拽`：通过顶部区域自由移动（Move freely via the top area）
* `窗口大小调节`：可以自动调节窗口大小（Automatically resizes the window）
*  `Ctrl + h` 一键切换原文 / AI 结果，边获取信息，边保持写作连贯性（Switch between the original text and AI-processed results without interrupting the current writing context）
* `额外功能持续优化中`（`Continuously Optimizing Additional Features`）


### 开发与构建（Development and Build）
1. 安装 Rust 与 Node.js（Install Rust and Node.js）
2. `npm install`
3. `npm run tauri dev`


### 使用示例（Usage Examples）
* 翻译这段话 --tr
* 怎么实现堆排序 --m
* priority queue in C++ --cpp
* list comprehension in Python --py


### 预览 (Preview)

#### 1. 沉浸式创作 (Writing Mode)
极简的记事本界面，让你进入心流状态。
<p align="center">
  <img src="https://github.com/user-attachments/assets/e175eae9-4d1c-42d4-9742-46a8ded2cc7a" width="80%">
</p>


#### 2. 透明置顶交互 (Interactive Mode)
半透明磨砂质感，配合 AI 输出实时优化 Prompt。
<p align="center">
  <img src="https://github.com/user-attachments/assets/e9db1482-4a80-49f1-98f6-097106fb20bc" width="80%">
</p>


#### 3. 即时翻译（Instant Translation）
中英文实时互译
<p align="center">
  <img src="https://github.com/user-attachments/assets/0d8bacb2-ee8c-4d5d-870c-a37e016c8b28" width="80%">
</p>


#### 4. 概念速释（Concept Clarification）
快速获取技术概念的直观解释，专注理解而非搜索
<p align="center">
  <img src="https://github.com/user-attachments/assets/134e1d5b-ae85-4e97-b9fe-896264c6a5aa" width="80%">
</p>


#### 5. 代码速查（Code & API Lookup）
查询实现需求最常用的类或函数，参数与示例一目了然
<p align="center">
  <img src="https://github.com/user-attachments/assets/af643fb2-f83a-4344-a9e2-cf078668979b" width="80%">
</p>


### 未来规划（Roadmap）
* 加入本地离线模型支持
* 支持更多语言命令（如 Go / Rust / js）
* 自定义指令扩展
* 自动提示建议规则（Auto-suggest）


## 支持与反馈（Support & Feedback）
欢迎提交 Issue、讨论、建议或贡献代码。


## 作者（Author）
**PromptSticker**  
Developed & maintained by **Divocle**.

GitHub: https://github.com/Divocle


## License
MIT License © 2026–present Divocle





