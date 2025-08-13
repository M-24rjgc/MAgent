import React, { useState, useEffect } from 'react';
import { 
  Users, Search, Shield, AlertTriangle, TrendingUp, CheckCircle, 
  MessageCircle, ArrowLeft, Play, Pause, RotateCcw
} from 'lucide-react';
import { useNavigate } from 'react-router-dom';

interface Agent {
  id: string;
  name: string;
  role: string;
  status: 'online' | 'offline' | 'speaking';
  icon: React.ReactNode;
  model?: string;
  contributions: number;
  task?: string;
}

interface Message {
  id: string;
  agentId: string;
  agentName: string;
  content: string;
  timestamp: Date;
  type: 'message' | 'proposal' | 'criticism';
}

interface Proposal {
  id: string;
  title: string;
  description: string;
  support: number;
  agentId: string;
  timestamp: Date;
}

export const DiscussionPage: React.FC = () => {
  const navigate = useNavigate();
  const [messages, setMessages] = useState<Message[]>([]);
  const [proposals, setProposals] = useState<Proposal[]>([]);
  const [isDiscussionActive, setIsDiscussionActive] = useState(true);
  const [convergenceScore] = useState(75);

  const agents: Agent[] = [
    {
      id: 'search',
      name: '搜索专家',
      role: '联网搜索智能体',
      status: 'speaking',
      icon: <Search className="w-5 h-5 text-blue-600" />,
      model: '秘塔搜索',
      contributions: 12,
      task: '搜索中...'
    },
    {
      id: 'critic',
      name: '批判专家',
      role: '反驳型智能体',
      status: 'online',
      icon: <Shield className="w-5 h-5 text-red-600" />,
      model: 'GPT-4',
      contributions: 8,
      task: '活跃'
    },
    {
      id: 'innovator',
      name: '创新专家',
      role: '创意生成智能体',
      status: 'online',
      icon: <TrendingUp className="w-5 h-5 text-green-600" />,
      model: 'Claude-3',
      contributions: 15,
      task: '思考中'
    },
    {
      id: 'verifier',
      name: '验证专家',
      role: '事实核查智能体',
      status: 'online',
      icon: <CheckCircle className="w-5 h-5 text-purple-600" />,
      model: 'GPT-4',
      contributions: 6,
      task: '验证中'
    },
    {
      id: 'synthesizer',
      name: '综合专家',
      role: '意见归纳智能体',
      status: 'online',
      icon: <MessageCircle className="w-5 h-5 text-indigo-600" />,
      model: 'Claude-3',
      contributions: 10,
      task: '归纳中'
    }
  ];

  // 模拟初始消息
  useEffect(() => {
    const initialMessages: Message[] = [
      {
        id: '1',
        agentId: 'search',
        agentName: '搜索专家',
        content: '我找到了关于在线教育平台的最新市场数据。根据2024年的研究，中国在线教育市场规模已达到4500亿元，同比增长18%。用户对个性化学习和AI辅助功能的需求快速增长。',
        timestamp: new Date(Date.now() - 300000),
        type: 'message'
      },
      {
        id: '2',
        agentId: 'critic',
        agentName: '批判专家',
        content: '我对这个想法有一些担忧。虽然市场需求很大，但竞争异常激烈。腾讯课堂、网易云课堂等巨头已经占据了主要市场份额。新进入者需要面对巨大的获客成本和技术投入。',
        timestamp: new Date(Date.now() - 240000),
        type: 'criticism'
      },
      {
        id: '3',
        agentId: 'innovator',
        agentName: '创新专家',
        content: '我们可以考虑差异化策略：1) 专注于职业技能培训细分市场 2) 采用AI驱动的个性化学习路径 3) 建立师资众包平台 4) 与企业合作提供内训服务。这样可以避开巨头的正面竞争。',
        timestamp: new Date(Date.now() - 180000),
        type: 'proposal'
      }
    ];

    const initialProposals: Proposal[] = [
      {
        id: '1',
        title: '专注职业技能培训',
        description: '避开K12和高等教育红海，专注于职业技能提升',
        support: 85,
        agentId: 'innovator',
        timestamp: new Date(Date.now() - 120000)
      },
      {
        id: '2',
        title: 'AI个性化学习引擎',
        description: '基于学习行为数据构建智能推荐系统',
        support: 92,
        agentId: 'innovator',
        timestamp: new Date(Date.now() - 60000)
      }
    ];

    setMessages(initialMessages);
    setProposals(initialProposals);
  }, []);

  const getAgentStatusColor = (status: string) => {
    switch (status) {
      case 'online': return 'bg-green-500';
      case 'speaking': return 'bg-blue-500 animate-pulse';
      case 'offline': return 'bg-gray-400';
      default: return 'bg-gray-400';
    }
  };

  const getAgentCardClass = (status: string) => {
    let baseClass = 'border-2 rounded-xl p-4 transition-all duration-300 cursor-pointer relative overflow-hidden';
    
    switch (status) {
      case 'speaking':
        return `${baseClass} border-green-400 bg-gradient-to-br from-green-50 to-green-100 shadow-lg animate-pulse`;
      case 'online':
        return `${baseClass} border-blue-300 bg-gradient-to-br from-blue-50 to-blue-100 hover:border-blue-400 hover:shadow-lg`;
      case 'offline':
        return `${baseClass} border-gray-200 bg-gray-50`;
      default:
        return `${baseClass} border-gray-200`;
    }
  };

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString('zh-CN', { 
      hour: '2-digit', 
      minute: '2-digit' 
    });
  };

  const getMessageTypeColor = (type: string) => {
    switch (type) {
      case 'proposal': return 'border-l-4 border-green-500 bg-green-50';
      case 'criticism': return 'border-l-4 border-red-500 bg-red-50';
      default: return 'border-l-4 border-blue-500 bg-blue-50';
    }
  };

  return (
    <div className="h-screen bg-gray-50 overflow-hidden">
      {/* 返回导航 */}
      <div className="fixed top-4 left-4 z-50">
        <button 
          onClick={() => navigate('/questioning')}
          className="bg-white rounded-lg shadow-lg border border-gray-200 px-4 py-2 flex items-center space-x-2 hover:bg-gray-50 transition-colors"
        >
          <ArrowLeft className="w-4 h-4" />
          <span className="text-sm font-medium">返回智能反问</span>
        </button>
      </div>

      <div className="h-full flex">
        {/* 左侧智能体列表 */}
        <div className="w-80 bg-white border-r border-gray-200 flex flex-col">
          <div className="p-6 border-b border-gray-200">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-lg font-semibold text-gray-900">智能体团队</h2>
              <div className="flex items-center space-x-2">
                <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
                  {agents.length}个智能体
                </span>
              </div>
            </div>
            <div className="flex items-center space-x-2 text-sm text-gray-600">
              <Users className="w-4 h-4" />
              <span>多角度协作讨论</span>
            </div>
          </div>

          <div className="flex-1 overflow-y-auto p-4 space-y-3">
            {agents.map((agent) => (
              <div key={agent.id} className={getAgentCardClass(agent.status)}>
                <div className={`absolute top-4 right-4 w-2 h-2 rounded-full ${getAgentStatusColor(agent.status)}`}></div>
                
                <div className="flex items-center space-x-3 mb-3">
                  <div className="w-10 h-10 bg-gray-100 rounded-lg flex items-center justify-center">
                    {agent.icon}
                  </div>
                  <div>
                    <h3 className="font-semibold text-gray-900">{agent.name}</h3>
                    <p className="text-xs text-gray-500">{agent.role}</p>
                  </div>
                </div>
                
                <div className="space-y-2 text-sm">
                  {agent.model && (
                    <div className="flex justify-between">
                      <span className="text-gray-600">模型</span>
                      <span className="text-gray-900">{agent.model}</span>
                    </div>
                  )}
                  <div className="flex justify-between">
                    <span className="text-gray-600">状态</span>
                    <span className={`font-medium ${
                      agent.status === 'speaking' ? 'text-green-600' : 
                      agent.status === 'online' ? 'text-blue-600' : 'text-gray-500'
                    }`}>
                      {agent.task || agent.status}
                    </span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-600">贡献</span>
                    <span className="text-gray-900">{agent.contributions}条洞察</span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* 中间讨论区域 */}
        <div className="flex-1 flex flex-col">
          {/* 讨论控制栏 */}
          <div className="bg-white border-b border-gray-200 p-4">
            <div className="flex items-center justify-between">
              <div className="flex items-center space-x-4">
                <h2 className="text-lg font-semibold text-gray-900">多角度协作讨论</h2>
                <div className="flex items-center space-x-2">
                  <div className="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                  <span className="text-sm text-gray-600">实时讨论中</span>
                </div>
              </div>
              <div className="flex items-center space-x-2">
                <button 
                  onClick={() => setIsDiscussionActive(!isDiscussionActive)}
                  className={`px-3 py-2 rounded-lg flex items-center space-x-2 transition-colors ${
                    isDiscussionActive 
                      ? 'bg-red-100 text-red-700 hover:bg-red-200' 
                      : 'bg-green-100 text-green-700 hover:bg-green-200'
                  }`}
                >
                  {isDiscussionActive ? <Pause className="w-4 h-4" /> : <Play className="w-4 h-4" />}
                  <span className="text-sm font-medium">
                    {isDiscussionActive ? '暂停讨论' : '继续讨论'}
                  </span>
                </button>
                <button className="px-3 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 transition-colors flex items-center space-x-2">
                  <RotateCcw className="w-4 h-4" />
                  <span className="text-sm font-medium">重新开始</span>
                </button>
              </div>
            </div>
          </div>

          {/* 消息时间线 */}
          <div className="flex-1 overflow-y-auto p-6">
            <div className="max-w-4xl mx-auto">
              <div className="space-y-6">
                {messages.map((message, index) => (
                  <div key={message.id} className="relative">
                    {/* 时间线 */}
                    {index > 0 && (
                      <div className="absolute left-8 -top-3 w-px h-6 bg-gray-200"></div>
                    )}
                    
                    <div className="flex items-start space-x-4">
                      {/* 头像 */}
                      <div className="flex-shrink-0">
                        <div className="w-8 h-8 bg-blue-100 rounded-full flex items-center justify-center border-2 border-white shadow-sm">
                          <div className="w-4 h-4 bg-blue-600 rounded-full"></div>
                        </div>
                      </div>
                      
                      {/* 消息内容 */}
                      <div className="flex-1">
                        <div className={`p-4 rounded-lg ${getMessageTypeColor(message.type)}`}>
                          <div className="flex items-center justify-between mb-2">
                            <div className="flex items-center space-x-2">
                              <span className="font-semibold text-gray-900">{message.agentName}</span>
                              <span className="text-xs text-gray-500">{formatTime(message.timestamp)}</span>
                            </div>
                            <div className="flex items-center space-x-1">
                              {message.type === 'proposal' && <TrendingUp className="w-4 h-4 text-green-600" />}
                              {message.type === 'criticism' && <AlertTriangle className="w-4 h-4 text-red-600" />}
                              {message.type === 'message' && <MessageCircle className="w-4 h-4 text-blue-600" />}
                            </div>
                          </div>
                          <p className="text-gray-800 leading-relaxed">{message.content}</p>
                        </div>
                      </div>
                    </div>
                  </div>
                ))}
                
                {/* 正在输入指示器 */}
                {isDiscussionActive && (
                  <div className="flex items-start space-x-4">
                    <div className="flex-shrink-0">
                      <div className="w-8 h-8 bg-gray-100 rounded-full flex items-center justify-center border-2 border-white shadow-sm">
                        <div className="w-4 h-4 bg-gray-400 rounded-full animate-pulse"></div>
                      </div>
                    </div>
                    <div className="flex-1">
                      <div className="bg-white border border-gray-200 rounded-lg p-4">
                        <div className="flex items-center space-x-2 mb-2">
                          <span className="font-semibold text-gray-700">验证专家</span>
                          <span className="text-xs text-gray-500">正在输入...</span>
                        </div>
                        <div className="flex space-x-1">
                          <div className="w-2 h-2 bg-blue-600 rounded-full animate-bounce"></div>
                          <div className="w-2 h-2 bg-blue-600 rounded-full animate-bounce" style={{ animationDelay: '0.1s' }}></div>
                          <div className="w-2 h-2 bg-blue-600 rounded-full animate-bounce" style={{ animationDelay: '0.2s' }}></div>
                        </div>
                      </div>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>

        {/* 右侧提案面板 */}
        <div className="w-80 bg-white border-l border-gray-200 flex flex-col">
          <div className="p-6 border-b border-gray-200">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-lg font-semibold text-gray-900">当前提案</h2>
              <span className="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800">
                {proposals.length}个提案
              </span>
            </div>
            
            {/* 收敛度指示器 */}
            <div className="flex items-center space-x-4">
              <div className="flex-shrink-0">
                <div className="relative w-16 h-16">
                  <svg className="w-16 h-16 transform -rotate-90">
                    <circle cx="32" cy="32" r="28" fill="none" stroke="#e5e7eb" strokeWidth="4" />
                    <circle 
                      cx="32" 
                      cy="32" 
                      r="28" 
                      fill="none" 
                      stroke="#3b82f6" 
                      strokeWidth="4" 
                      strokeLinecap="round"
                      strokeDasharray={`${2 * Math.PI * 28}`}
                      strokeDashoffset={`${2 * Math.PI * 28 * (1 - convergenceScore / 100)}`}
                      className="transition-all duration-500"
                    />
                  </svg>
                  <div className="absolute inset-0 flex items-center justify-center">
                    <span className="text-sm font-bold text-gray-900">{convergenceScore}%</span>
                  </div>
                </div>
              </div>
              <div>
                <p className="text-sm font-medium text-gray-900">收敛度</p>
                <p className="text-xs text-gray-600">智能体意见一致性</p>
              </div>
            </div>
          </div>

          <div className="flex-1 overflow-y-auto p-4 space-y-3">
            {proposals.map((proposal) => (
              <div key={proposal.id} className="bg-white border border-gray-200 rounded-lg p-4 hover:shadow-lg transition-shadow">
                <div className="flex items-center justify-between mb-2">
                  <h3 className="font-semibold text-gray-900 text-sm">{proposal.title}</h3>
                  <span className="text-xs text-gray-500">{formatTime(proposal.timestamp)}</span>
                </div>
                <p className="text-sm text-gray-600 mb-3">{proposal.description}</p>
                
                {/* 支持度条 */}
                <div className="space-y-2">
                  <div className="flex justify-between text-xs">
                    <span className="text-gray-600">支持度</span>
                    <span className="font-medium text-gray-900">{proposal.support}%</span>
                  </div>
                  <div className="h-2 bg-gray-200 rounded-full overflow-hidden">
                    <div 
                      className="h-full bg-gradient-to-r from-green-500 to-green-600 rounded-full transition-all duration-500"
                      style={{ width: `${proposal.support}%` }}
                    ></div>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};
