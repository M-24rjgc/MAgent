import React, { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import { 
  Lightbulb, 
  FileText, 
  Link as LinkIcon, 
  Upload,
  Send,
  Sparkles,
  AlertCircle
} from 'lucide-react';
import { IdeaSeed } from '../types';

const IdeaInputPage: React.FC = () => {
  const navigate = useNavigate();
  const [inputType, setInputType] = useState<'text' | 'file' | 'url'>('text');
  const [ideaText, setIdeaText] = useState('');
  const [contextHints, setContextHints] = useState<string[]>(['']);
  const [domain, setDomain] = useState('');
  const [files, setFiles] = useState<FileList | null>(null);
  const [url, setUrl] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');

  const addContextHint = () => {
    setContextHints([...contextHints, '']);
  };

  const updateContextHint = (index: number, value: string) => {
    const newHints = [...contextHints];
    newHints[index] = value;
    setContextHints(newHints);
  };

  const removeContextHint = (index: number) => {
    if (contextHints.length > 1) {
      setContextHints(contextHints.filter((_, i) => i !== index));
    }
  };

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setFiles(e.target.files);
    }
  };

  const extractTextFromFiles = async (): Promise<string> => {
    if (!files || files.length === 0) return '';
    
    // TODO: 实现文件内容提取
    // 这里应该调用 Tauri 后端来处理文件读取
    const file = files[0];
    return `从文件 "${file.name}" 提取的内容...`;
  };

  const extractTextFromUrl = async (): Promise<string> => {
    if (!url.trim()) return '';
    
    // TODO: 实现 URL 内容抓取
    // 这里应该调用 Tauri 后端来处理 URL 内容抓取
    return `从 URL "${url}" 抓取的内容...`;
  };

  const validateInput = (): boolean => {
    setError('');

    if (inputType === 'text' && !ideaText.trim()) {
      setError('请输入您的想法内容');
      return false;
    }

    if (inputType === 'file' && (!files || files.length === 0)) {
      setError('请选择要上传的文件');
      return false;
    }

    if (inputType === 'url' && !url.trim()) {
      setError('请输入有效的 URL 地址');
      return false;
    }

    return true;
  };

  const startOptimization = async () => {
    if (!validateInput()) return;

    setLoading(true);
    setError('');

    try {
      let finalText = ideaText;

      // 根据输入类型处理内容
      if (inputType === 'file') {
        finalText = await extractTextFromFiles();
      } else if (inputType === 'url') {
        finalText = await extractTextFromUrl();
      }

      // 过滤空的上下文提示
      const validHints = contextHints.filter(hint => hint.trim());

      const seed: IdeaSeed = {
        raw_text: finalText,
        context_hints: validHints,
        domain: domain.trim() || undefined
      };

      // 调用后端开始优化
      const sessionId = await invoke<string>('start_concept_optimization', { seed });
      
      // 导航到工作区页面
      navigate(`/workspace?session=${sessionId}`);
      
    } catch (err) {
      console.error('Failed to start optimization:', err);
      setError('启动优化失败，请检查输入内容并重试');
    } finally {
      setLoading(false);
    }
  };

  const inputTypeButtons = [
    { type: 'text' as const, label: '文本输入', icon: FileText, description: '直接输入文本内容' },
    { type: 'file' as const, label: '文件上传', icon: Upload, description: '上传文档文件' },
    { type: 'url' as const, label: 'URL 链接', icon: LinkIcon, description: '从网页抓取内容' }
  ];

  return (
    <div className="max-w-4xl mx-auto space-y-6">
      {/* 页面标题 */}
      <div className="text-center mb-8">
        <div className="flex items-center justify-center mb-4">
          <div className="w-16 h-16 bg-gradient-to-br from-blue-500 to-purple-600 rounded-2xl flex items-center justify-center">
            <Lightbulb className="w-8 h-8 text-white" />
          </div>
        </div>
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          想法输入
        </h1>
        <p className="text-gray-600 dark:text-gray-400">
          输入您的创意想法，让 AI 团队协作优化它
        </p>
      </div>

      {/* 输入类型选择 */}
      <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          选择输入方式
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {inputTypeButtons.map((button) => {
            const Icon = button.icon;
            const isActive = inputType === button.type;
            
            return (
              <button
                key={button.type}
                onClick={() => setInputType(button.type)}
                className={`
                  p-4 rounded-lg border-2 transition-all duration-200 text-left
                  ${isActive
                    ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20'
                    : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'
                  }
                `}
              >
                <div className="flex items-center mb-2">
                  <Icon className={`w-5 h-5 mr-2 ${isActive ? 'text-blue-600' : 'text-gray-600 dark:text-gray-400'}`} />
                  <span className={`font-medium ${isActive ? 'text-blue-600 dark:text-blue-400' : 'text-gray-900 dark:text-white'}`}>
                    {button.label}
                  </span>
                </div>
                <p className="text-sm text-gray-500 dark:text-gray-400">
                  {button.description}
                </p>
              </button>
            );
          })}
        </div>
      </div>

      {/* 内容输入区域 */}
      <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          内容输入
        </h2>

        {inputType === 'text' && (
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              想法描述
            </label>
            <textarea
              value={ideaText}
              onChange={(e) => setIdeaText(e.target.value)}
              placeholder="请详细描述您的想法、概念或项目..."
              rows={8}
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg resize-none focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white"
            />
            <div className="mt-2 text-sm text-gray-500 dark:text-gray-400">
              {ideaText.length} 字符 • 建议至少 50 字符以获得更好的优化效果
            </div>
          </div>
        )}

        {inputType === 'file' && (
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              上传文件
            </label>
            <div className="border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg p-6">
              <input
                type="file"
                onChange={handleFileChange}
                accept=".txt,.md,.pdf,.doc,.docx"
                className="hidden"
                id="file-upload"
              />
              <label
                htmlFor="file-upload"
                className="cursor-pointer flex flex-col items-center"
              >
                <Upload className="w-12 h-12 text-gray-400 mb-2" />
                <span className="text-sm font-medium text-gray-900 dark:text-white">
                  点击上传文件
                </span>
                <span className="text-xs text-gray-500 dark:text-gray-400 mt-1">
                  支持 TXT, MD, PDF, DOC, DOCX 格式
                </span>
              </label>
              {files && files.length > 0 && (
                <div className="mt-4 text-sm text-green-600 dark:text-green-400">
                  已选择: {files[0].name}
                </div>
              )}
            </div>
          </div>
        )}

        {inputType === 'url' && (
          <div>
            <label className="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-2">
              URL 地址
            </label>
            <input
              type="url"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              placeholder="https://example.com/article"
              className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white"
            />
            <div className="mt-2 text-sm text-gray-500 dark:text-gray-400">
              将自动抓取网页内容进行分析
            </div>
          </div>
        )}
      </div>

      {/* 上下文提示 */}
      <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          上下文提示 (可选)
        </h2>
        <p className="text-sm text-gray-500 dark:text-gray-400 mb-4">
          提供额外的背景信息、约束条件或目标，帮助 AI 更好地理解您的需求
        </p>
        
        <div className="space-y-3">
          {contextHints.map((hint, index) => (
            <div key={index} className="flex space-x-2">
              <input
                type="text"
                value={hint}
                onChange={(e) => updateContextHint(index, e.target.value)}
                placeholder={`提示 ${index + 1}: 例如 "面向年轻用户" 或 "预算有限"`}
                className="flex-1 px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white"
              />
              {contextHints.length > 1 && (
                <button
                  onClick={() => removeContextHint(index)}
                  className="px-3 py-2 text-red-600 hover:text-red-700 transition-colors"
                >
                  删除
                </button>
              )}
            </div>
          ))}
          
          <button
            onClick={addContextHint}
            className="text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 text-sm transition-colors"
          >
            + 添加更多提示
          </button>
        </div>
      </div>

      {/* 领域设置 */}
      <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          领域设置 (可选)
        </h2>
        <input
          type="text"
          value={domain}
          onChange={(e) => setDomain(e.target.value)}
          placeholder="例如: 科技创新、商业模式、产品设计..."
          className="w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent dark:bg-gray-700 dark:text-white"
        />
        <div className="mt-2 text-sm text-gray-500 dark:text-gray-400">
          指定领域可以让 AI 提供更专业的建议
        </div>
      </div>

      {/* 错误提示 */}
      {error && (
        <div className="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
          <div className="flex items-center">
            <AlertCircle className="w-5 h-5 text-red-600 dark:text-red-400 mr-2" />
            <span className="text-red-700 dark:text-red-300">{error}</span>
          </div>
        </div>
      )}

      {/* 开始优化按钮 */}
      <div className="flex justify-center">
        <button
          onClick={startOptimization}
          disabled={loading}
          className={`
            flex items-center px-8 py-3 rounded-lg font-medium transition-all duration-200
            ${loading
              ? 'bg-gray-400 cursor-not-allowed'
              : 'bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 transform hover:scale-105'
            }
            text-white shadow-lg
          `}
        >
          {loading ? (
            <>
              <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white mr-2"></div>
              正在启动优化...
            </>
          ) : (
            <>
              <Sparkles className="w-5 h-5 mr-2" />
              开始 AI 优化
              <Send className="w-5 h-5 ml-2" />
            </>
          )}
        </button>
      </div>
    </div>
  );
};

export default IdeaInputPage;
