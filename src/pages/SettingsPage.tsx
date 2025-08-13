import React, { useState } from 'react';
import { 
  ArrowLeft, Save, Settings, Brain, Workflow, Users, 
  Shield, Plug, Bell, Database, Moon, Sun,
  Key, Download, Upload, AlertTriangle,
  CheckCircle, Info
} from 'lucide-react';
import { useNavigate } from 'react-router-dom';

type SettingSection = 'general' | 'ai' | 'workflow' | 'team' | 'security' | 'integration' | 'notification' | 'backup';

interface AIProvider {
  id: string;
  name: string;
  status: 'connected' | 'disconnected' | 'error';
  apiKey?: string;
  models: string[];
}

export const SettingsPage: React.FC = () => {
  const navigate = useNavigate();
  const [activeSection, setActiveSection] = useState<SettingSection>('general');
  const [isDarkMode, setIsDarkMode] = useState(false);
  const [language, setLanguage] = useState('zh-CN');
  const [autoSave, setAutoSave] = useState(true);
  const [showSaveNotification, setShowSaveNotification] = useState(false);

  // AI 提供商配置
  const [aiProviders] = useState<AIProvider[]>([
    {
      id: 'openai',
      name: 'OpenAI GPT',
      status: 'connected',
      models: ['GPT-4', 'GPT-3.5-turbo', 'GPT-4-turbo']
    },
    {
      id: 'anthropic',
      name: 'Anthropic Claude',
      status: 'connected',
      models: ['Claude-3-Opus', 'Claude-3-Sonnet', 'Claude-3-Haiku']
    },
    {
      id: 'google',
      name: 'Google Gemini',
      status: 'disconnected',
      models: ['Gemini-Pro', 'Gemini-Ultra']
    },
    {
      id: 'deepseek',
      name: 'DeepSeek',
      status: 'error',
      models: ['DeepSeek-V2', 'DeepSeek-Coder']
    }
  ]);

  const handleSave = () => {
    // 模拟保存操作
    setShowSaveNotification(true);
    setTimeout(() => setShowSaveNotification(false), 3000);
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'connected': return <CheckCircle className="w-4 h-4 text-green-600" />;
      case 'disconnected': return <AlertTriangle className="w-4 h-4 text-gray-400" />;
      case 'error': return <AlertTriangle className="w-4 h-4 text-red-600" />;
      default: return <Info className="w-4 h-4 text-gray-400" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'connected': return 'bg-green-100 text-green-800';
      case 'disconnected': return 'bg-gray-100 text-gray-800';
      case 'error': return 'bg-red-100 text-red-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getStatusText = (status: string) => {
    switch (status) {
      case 'connected': return '已连接';
      case 'disconnected': return '未连接';
      case 'error': return '连接错误';
      default: return '未知';
    }
  };

  const sections = [
    { id: 'general' as const, name: '常规设置', icon: Settings },
    { id: 'ai' as const, name: 'AI 配置', icon: Brain },
    { id: 'workflow' as const, name: '工作流设置', icon: Workflow },
    { id: 'team' as const, name: '团队管理', icon: Users },
    { id: 'security' as const, name: '安全设置', icon: Shield },
    { id: 'integration' as const, name: '集成配置', icon: Plug },
    { id: 'notification' as const, name: '通知设置', icon: Bell },
    { id: 'backup' as const, name: '备份恢复', icon: Database }
  ];

  const ToggleSwitch: React.FC<{ checked: boolean; onChange: (checked: boolean) => void }> = ({ checked, onChange }) => (
    <button
      onClick={() => onChange(!checked)}
      className={`relative w-12 h-6 rounded-full transition-colors ${
        checked ? 'bg-gradient-to-r from-blue-600 to-purple-600' : 'bg-gray-300'
      }`}
    >
      <div
        className={`absolute top-1 w-4 h-4 bg-white rounded-full transition-transform ${
          checked ? 'translate-x-7' : 'translate-x-1'
        }`}
      />
    </button>
  );

  const SettingItem: React.FC<{
    title: string;
    description: string;
    children: React.ReactNode;
  }> = ({ title, description, children }) => (
    <div className="p-4 rounded-lg border border-gray-200 hover:bg-gray-50 transition-colors">
      <div className="flex items-center justify-between">
        <div className="flex-1">
          <h3 className="font-medium text-gray-900">{title}</h3>
          <p className="text-sm text-gray-600 mt-1">{description}</p>
        </div>
        <div className="ml-4">
          {children}
        </div>
      </div>
    </div>
  );

  const renderSectionContent = () => {
    switch (activeSection) {
      case 'general':
        return (
          <div className="space-y-6">
            <SettingItem
              title="界面语言"
              description="选择系统界面显示语言"
            >
              <select 
                value={language}
                onChange={(e) => setLanguage(e.target.value)}
                className="p-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              >
                <option value="zh-CN">简体中文</option>
                <option value="en-US">English</option>
                <option value="ja-JP">日本語</option>
              </select>
            </SettingItem>

            <SettingItem
              title="深色模式"
              description="切换到深色主题以减少眼部疲劳"
            >
              <div className="flex items-center space-x-2">
                <Sun className="w-4 h-4 text-gray-500" />
                <ToggleSwitch checked={isDarkMode} onChange={setIsDarkMode} />
                <Moon className="w-4 h-4 text-gray-500" />
              </div>
            </SettingItem>

            <SettingItem
              title="自动保存"
              description="自动保存您的工作进度和设置"
            >
              <ToggleSwitch checked={autoSave} onChange={setAutoSave} />
            </SettingItem>

            <SettingItem
              title="时区设置"
              description="设置您所在的时区"
            >
              <select className="p-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                <option>Asia/Shanghai (UTC+8)</option>
                <option>Asia/Tokyo (UTC+9)</option>
                <option>America/New_York (UTC-5)</option>
                <option>Europe/London (UTC+0)</option>
              </select>
            </SettingItem>
          </div>
        );

      case 'ai':
        return (
          <div className="space-y-6">
            <div className="bg-blue-50 border border-blue-200 rounded-lg p-4">
              <div className="flex items-center space-x-2">
                <Info className="w-5 h-5 text-blue-600" />
                <h3 className="font-medium text-blue-900">AI 模型配置</h3>
              </div>
              <p className="text-sm text-blue-700 mt-1">
                配置您要使用的 AI 模型提供商。确保您拥有相应的 API 密钥。
              </p>
            </div>

            <div className="space-y-4">
              {aiProviders.map((provider) => (
                <div key={provider.id} className="border border-gray-200 rounded-lg p-4">
                  <div className="flex items-center justify-between mb-3">
                    <div className="flex items-center space-x-3">
                      <div className="w-10 h-10 bg-gray-100 rounded-lg flex items-center justify-center">
                        <Brain className="w-5 h-5 text-gray-600" />
                      </div>
                      <div>
                        <h3 className="font-medium text-gray-900">{provider.name}</h3>
                        <div className="flex items-center space-x-2">
                          {getStatusIcon(provider.status)}
                          <span className={`text-xs px-2 py-1 rounded-full ${getStatusColor(provider.status)}`}>
                            {getStatusText(provider.status)}
                          </span>
                        </div>
                      </div>
                    </div>
                    <button className="px-3 py-1 text-sm border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors">
                      配置
                    </button>
                  </div>

                  {provider.status === 'connected' && (
                    <div className="space-y-2">
                      <p className="text-sm text-gray-600">可用模型:</p>
                      <div className="flex flex-wrap gap-2">
                        {provider.models.map((model, index) => (
                          <span 
                            key={index}
                            className="inline-flex items-center px-2 py-1 rounded-md text-xs font-medium bg-green-100 text-green-800"
                          >
                            {model}
                          </span>
                        ))}
                      </div>
                    </div>
                  )}

                  {provider.status === 'error' && (
                    <div className="bg-red-50 border border-red-200 rounded-md p-3">
                      <p className="text-sm text-red-700">
                        连接失败: API 密钥无效或网络错误
                      </p>
                    </div>
                  )}
                </div>
              ))}
            </div>

            <div className="bg-gray-50 rounded-lg p-4">
              <h3 className="font-medium text-gray-900 mb-3">模型使用策略</h3>
              <div className="space-y-3">
                <SettingItem
                  title="自动降级"
                  description="当首选模型不可用时自动切换到备用模型"
                >
                  <ToggleSwitch checked={true} onChange={() => {}} />
                </SettingItem>
                <SettingItem
                  title="成本优化"
                  description="优先使用成本较低的模型"
                >
                  <ToggleSwitch checked={false} onChange={() => {}} />
                </SettingItem>
              </div>
            </div>
          </div>
        );

      case 'workflow':
        return (
          <div className="space-y-6">
            <SettingItem
              title="最大迭代轮数"
              description="设置智能体协作讨论的最大轮数"
            >
              <input 
                type="number" 
                defaultValue="20"
                className="w-20 p-2 border border-gray-300 rounded-lg text-sm text-center focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              />
            </SettingItem>

            <SettingItem
              title="收敛阈值"
              description="智能体意见收敛度达到此值时停止讨论"
            >
              <div className="flex items-center space-x-2">
                <input 
                  type="range" 
                  min="50" 
                  max="95" 
                  defaultValue="80"
                  className="w-32"
                />
                <span className="text-sm text-gray-600">80%</span>
              </div>
            </SettingItem>

            <SettingItem
              title="并行处理"
              description="允许多个智能体同时处理任务"
            >
              <ToggleSwitch checked={true} onChange={() => {}} />
            </SettingItem>

            <SettingItem
              title="实时更新"
              description="实时显示智能体处理进度"
            >
              <ToggleSwitch checked={true} onChange={() => {}} />
            </SettingItem>
          </div>
        );

      case 'security':
        return (
          <div className="space-y-6">
            <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4">
              <div className="flex items-center space-x-2">
                <Shield className="w-5 h-5 text-yellow-600" />
                <h3 className="font-medium text-yellow-900">安全提醒</h3>
              </div>
              <p className="text-sm text-yellow-700 mt-1">
                请妥善保管您的 API 密钥，不要分享给他人。
              </p>
            </div>

            <SettingItem
              title="API 密钥加密"
              description="在本地存储中加密保存 API 密钥"
            >
              <ToggleSwitch checked={true} onChange={() => {}} />
            </SettingItem>

            <SettingItem
              title="访问日志"
              description="记录所有 API 调用和访问日志"
            >
              <ToggleSwitch checked={true} onChange={() => {}} />
            </SettingItem>

            <SettingItem
              title="安全连接"
              description="强制使用 HTTPS 连接"
            >
              <ToggleSwitch checked={true} onChange={() => {}} />
            </SettingItem>

            <div className="border border-gray-200 rounded-lg p-4">
              <h3 className="font-medium text-gray-900 mb-3">密钥管理</h3>
              <div className="space-y-3">
                <button className="w-full flex items-center justify-center space-x-2 px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors">
                  <Key className="w-4 h-4" />
                  <span>管理 API 密钥</span>
                </button>
                <button className="w-full flex items-center justify-center space-x-2 px-4 py-2 border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors">
                  <Download className="w-4 h-4" />
                  <span>导出密钥备份</span>
                </button>
              </div>
            </div>
          </div>
        );

      case 'backup':
        return (
          <div className="space-y-6">
            <SettingItem
              title="自动备份"
              description="定期自动备份您的项目和设置"
            >
              <ToggleSwitch checked={true} onChange={() => {}} />
            </SettingItem>

            <SettingItem
              title="备份频率"
              description="设置自动备份的频率"
            >
              <select className="p-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                <option>每天</option>
                <option>每周</option>
                <option>每月</option>
              </select>
            </SettingItem>

            <div className="border border-gray-200 rounded-lg p-4">
              <h3 className="font-medium text-gray-900 mb-3">备份操作</h3>
              <div className="grid grid-cols-2 gap-3">
                <button className="flex items-center justify-center space-x-2 px-4 py-2 bg-blue-50 text-blue-700 rounded-lg hover:bg-blue-100 transition-colors">
                  <Download className="w-4 h-4" />
                  <span>立即备份</span>
                </button>
                <button className="flex items-center justify-center space-x-2 px-4 py-2 bg-green-50 text-green-700 rounded-lg hover:bg-green-100 transition-colors">
                  <Upload className="w-4 h-4" />
                  <span>恢复备份</span>
                </button>
              </div>
            </div>

            <div className="bg-gray-50 rounded-lg p-4">
              <h3 className="font-medium text-gray-900 mb-3">最近备份</h3>
              <div className="space-y-2">
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-600">2024-12-19 14:30</span>
                  <span className="text-green-600">成功</span>
                </div>
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-600">2024-12-18 14:30</span>
                  <span className="text-green-600">成功</span>
                </div>
                <div className="flex items-center justify-between text-sm">
                  <span className="text-gray-600">2024-12-17 14:30</span>
                  <span className="text-green-600">成功</span>
                </div>
              </div>
            </div>
          </div>
        );

      default:
        return (
          <div className="text-center py-12">
            <div className="w-16 h-16 mx-auto mb-4 bg-gray-100 rounded-full flex items-center justify-center">
              <Settings className="w-8 h-8 text-gray-400" />
            </div>
            <h3 className="text-lg font-medium text-gray-900 mb-2">此功能正在开发中</h3>
            <p className="text-gray-600">敬请期待后续版本更新</p>
          </div>
        );
    }
  };

  return (
    <div className="min-h-screen bg-gray-50">
      {/* 成功通知 */}
      {showSaveNotification && (
        <div className="fixed top-4 right-4 z-50 bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded-lg shadow-lg flex items-center space-x-2">
          <CheckCircle className="w-5 h-5" />
          <span>设置已保存</span>
        </div>
      )}

      {/* 顶部导航 */}
      <nav className="bg-white border-b border-gray-200 px-6 py-4">
        <div className="flex items-center justify-between">
          <div className="flex items-center space-x-4">
            <button 
              onClick={() => navigate('/')}
              className="flex items-center text-gray-600 hover:text-gray-900 transition-colors"
            >
              <ArrowLeft className="w-5 h-5 mr-2" />
              返回主页
            </button>
            <div className="h-6 w-px bg-gray-300"></div>
            <h1 className="text-xl font-semibold text-gray-900">系统设置</h1>
          </div>
          <div className="flex items-center space-x-4">
            <button 
              onClick={handleSave}
              className="px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-lg hover:opacity-90 transition-opacity flex items-center space-x-2"
            >
              <Save className="w-4 h-4" />
              <span>保存设置</span>
            </button>
          </div>
        </div>
      </nav>

      <div className="flex">
        {/* 左侧导航 */}
        <div className="w-80 bg-white border-r border-gray-200 p-6">
          <div className="space-y-2">
            {sections.map((section) => {
              const Icon = section.icon;
              return (
                <button
                  key={section.id}
                  onClick={() => setActiveSection(section.id)}
                  className={`w-full text-left px-4 py-3 rounded-lg flex items-center space-x-3 transition-colors ${
                    activeSection === section.id
                      ? 'bg-gradient-to-r from-blue-600 to-purple-600 text-white'
                      : 'text-gray-700 hover:bg-gray-100'
                  }`}
                >
                  <Icon className="w-5 h-5" />
                  <span>{section.name}</span>
                </button>
              );
            })}
          </div>
        </div>

        {/* 右侧设置内容 */}
        <div className="flex-1 p-8 overflow-y-auto">
          <div className="max-w-4xl mx-auto">
            <div className="bg-white rounded-xl shadow-lg border border-gray-200">
              <div className="p-6 border-b border-gray-200">
                <h2 className="text-xl font-semibold text-gray-900">
                  {sections.find(s => s.id === activeSection)?.name}
                </h2>
                <p className="text-gray-600 mt-1">
                  {activeSection === 'general' && '管理系统的基本配置和偏好设置'}
                  {activeSection === 'ai' && '配置 AI 模型提供商和相关参数'}
                  {activeSection === 'workflow' && '自定义智能体工作流程和规则'}
                  {activeSection === 'team' && '管理团队成员和权限设置'}
                  {activeSection === 'security' && '配置安全策略和访问控制'}
                  {activeSection === 'integration' && '管理第三方服务集成'}
                  {activeSection === 'notification' && '设置通知偏好和提醒规则'}
                  {activeSection === 'backup' && '配置数据备份和恢复选项'}
                </p>
              </div>
              <div className="p-6">
                {renderSectionContent()}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
