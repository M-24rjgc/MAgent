# MAgent - 多智能体概念优化引擎

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Version](https://img.shields.io/badge/version-0.1.0-green.svg)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey.svg)

一个基于多智能体协作的概念优化引擎，使用 Rust + Tauri 2.x 构建的跨平台桌面应用程序。

## ✨ 核心特性

- 🤖 **6 个 AI 智能代理协作系统**
- 🎯 **多维度概念优化流程**
- 🌐 **支持多个 AI 模型提供商**
- 💻 **现代化 React 界面**
- ⚡ **高性能 Rust 后端**
- 🔒 **本地数据存储**

## 🏗️ 技术架构

### 前端

- **框架**: React 18.3 + TypeScript
- **样式**: Tailwind CSS
- **路由**: React Router
- **构建**: Vite

### 后端

- **框架**: Tauri 2.x
- **语言**: Rust
- **数据库**: SQLite
- **异步**: Tokio

### AI 集成

- OpenAI (GPT-4, GPT-3.5)
- Anthropic Claude
- DeepSeek

## 🧠 智能代理系统

### 6 个核心代理

1. **ClarifierAgent** - 想法澄清代理

   - 分析输入的概念
   - 生成澄清问题
   - 识别概念边界

2. **InnovatorAgent** - 创新建议代理

   - 多维度创新分析
   - 生成改进建议
   - 探索新的可能性

3. **CriticAgent** - 批判分析代理

   - 识别潜在问题
   - 提供建设性批评
   - 分析可行性

4. **SynthesizerAgent** - 综合整理代理

   - 整合各方观点
   - 生成统一方案
   - 平衡不同需求

5. **VerifierAgent** - 验证确认代理

   - 验证方案可行性
   - 检查逻辑一致性
   - 确认实施步骤

6. **SummarizerAgent** - 总结输出代理
   - 生成最终报告
   - 提炼核心要点
   - 制定行动计划

## 🚀 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- Tauri CLI

### 安装依赖

```bash
# 安装前端依赖
npm install

# 安装Tauri CLI
npm install -g @tauri-apps/cli
```

### 配置 AI API

复制配置模板并填入你的 API 密钥：

```bash
cp config.example.toml config.toml
```

编辑 `config.toml` 文件，填入真实的 API 密钥：

```toml
[ai]
openai_api_key = "your-openai-api-key"
claude_api_key = "your-claude-api-key"
deepseek_api_key = "your-deepseek-api-key"
```

### 运行开发环境

```bash
# 启动开发服务器（前端 + 后端）
npm run tauri:dev

# 或者分别启动
npm run dev:frontend  # 仅前端
cargo run --manifest-path src-tauri/Cargo.toml  # 仅后端
```

### 构建生产版本

```bash
# 构建桌面应用
npm run tauri:build
```

## 📱 界面预览

### 7 个核心页面

1. **首页** - 项目概览和快速访问
2. **想法输入** - 概念输入和初始化
3. **讨论澄清** - AI 澄清对话界面
4. **质疑批判** - 批判性分析界面
5. **工作台** - 多代理协作工作区
6. **项目管理** - 项目和历史管理
7. **设置** - 配置和个性化选项

## 🔧 项目结构

```
magent/
├── src/                    # React前端源码
│   ├── components/         # UI组件
│   ├── pages/             # 页面组件
│   └── types/             # TypeScript类型定义
├── src-tauri/             # Rust后端源码
│   ├── src/
│   │   ├── agents/        # AI代理实现
│   │   ├── models/        # AI模型管理
│   │   ├── storage/       # 数据存储
│   │   └── core/          # 核心逻辑
│   └── Cargo.toml
├── ui-prototype/          # 静态原型页面
└── docs/                  # 项目文档
```

## 📋 开发状态

- ✅ 前端界面 (100%)
- ✅ AI 模型集成 (100%)
- ✅ 智能代理系统 (85%)
- ✅ 数据存储 (90%)
- ✅ 基础功能 (90%)

## 🤝 贡献指南

欢迎贡献代码！请遵循以下流程：

1. Fork 本仓库
2. 创建功能分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🔗 相关链接

- [Tauri 官网](https://tauri.app/)
- [React 官网](https://reactjs.org/)
- [Rust 官网](https://www.rust-lang.org/)

## 📞 联系方式

如有问题或建议，请创建 Issue 或联系项目维护者。

---

**注意**: 本项目需要配置 AI API 密钥才能正常使用所有功能。请确保在 `config.toml` 中配置了有效的 API 密钥。
