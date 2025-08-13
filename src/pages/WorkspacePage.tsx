import React, { useState, useEffect } from 'react';
import { useSearchParams } from 'react-router-dom';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { 
  Brain, 
  Clock, 
  CheckCircle, 
  AlertCircle, 
  Play, 
  Pause,
  RotateCcw,
  Activity,
  Users,
  MessageSquare,
  FileText,
  TrendingUp,
  Shield
} from 'lucide-react';
import { OptimizationSession, AgentState, AgentStatus } from '../types';

const WorkspacePage: React.FC = () => {
  const [searchParams] = useSearchParams();
  const sessionId = searchParams.get('session');
  
  const [session, setSession] = useState<OptimizationSession | null>(null);
  const [agentStates, setAgentStates] = useState<AgentState[]>([
    { name: 'Clarifier', status: 'pending', progress: 0, message: '等待启动...' },
    { name: 'Innovator', status: 'pending', progress: 0, message: '等待启动...' },
    { name: 'Critic', status: 'pending', progress: 0, message: '等待启动...' },
    { name: 'Synthesizer', status: 'pending', progress: 0, message: '等待启动...' },
    { name: 'Verifier', status: 'pending', progress: 0, message: '等待启动...' },
    { name: 'Summarizer', status: 'pending', progress: 0, message: '等待启动...' }
  ]);
  const [isProcessing, setIsProcessing] = useState(false);
  const [logs, setLogs] = useState<string[]>([]);

  useEffect(() => {
    if (sessionId) {
      loadSession(sessionId);
    }
    
    // 监听后端事件
    const unlisten = listen<any>("agent-update", (event) => {
      handleAgentUpdate(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, [sessionId]);

  const loadSession = async (id: string) => {
    try {
      // TODO: 实现后端 API 调用
      // const sessionData = await invoke<OptimizationSession>('get_session', { sessionId: id });
      // setSession(sessionData);
      
      // 模拟数据
      const mockSession: OptimizationSession = {
        id,
        idea_seed: {
          raw_text: "一个基于AI的智能学习平台，能够根据学生的学习风格和进度自动调整教学内容",
          context_hints: ["面向大学生", "在线教育", "个性化学习"],
          domain: "教育科技"
        },
        current_state: "processing",
        created_at: new Date().toISOString(),
        updated_at: new Date().toISOString()
      };
      
      setSession(mockSession);
    } catch (error) {
      console.error('Failed to load session:', error);
    }
  };

  const handleAgentUpdate = (payload: any) => {
    console.log('Agent update:', payload);
    setLogs(prev => [...prev, `${new Date().toLocaleTimeString()}: ${JSON.stringify(payload)}`]);
    
    // 更新对应 Agent 的状态
    setAgentStates(prev => prev.map(agent => 
      agent.name === payload.agent 
        ? { ...agent, status: payload.status, progress: payload.progress, message: payload.message }
        : agent
    ));
  };

  const startOptimization = async () => {
    if (!session) return;
    
    setIsProcessing(true);
    
    try {
      await invoke('start_agent_workflow', { sessionId: session.id });
      setLogs(prev => [...prev, `${new Date().toLocaleTimeString()}: 开始 Agent 工作流`]);
    } catch (error) {
      console.error('Failed to start optimization:', error);
      setLogs(prev => [...prev, `${new Date().toLocaleTimeString()}: 启动失败 - ${error}`]);
    }
  };

  const pauseOptimization = async () => {
    try {
      await invoke('pause_agent_workflow', { sessionId: session?.id });
      setIsProcessing(false);
      setLogs(prev => [...prev, `${new Date().toLocaleTimeString()}: 暂停工作流`]);
    } catch (error) {
      console.error('Failed to pause optimization:', error);
    }
  };

  const resetOptimization = async () => {
    try {
      await invoke('reset_agent_workflow', { sessionId: session?.id });
      setIsProcessing(false);
      setAgentStates(prev => prev.map(agent => ({ 
        ...agent, 
        status: 'pending', 
        progress: 0, 
        message: '等待启动...' 
      })));
      setLogs([]);
    } catch (error) {
      console.error('Failed to reset optimization:', error);
    }
  };

  const getStatusIcon = (status: AgentStatus) => {
    switch (status) {
      case 'running':
        return <Activity className="w-5 h-5 text-blue-500 animate-pulse" />;
      case 'completed':
        return <CheckCircle className="w-5 h-5 text-green-500" />;
      case 'error':
        return <AlertCircle className="w-5 h-5 text-red-500" />;
      default:
        return <Clock className="w-5 h-5 text-gray-400" />;
    }
  };

  const getAgentIcon = (agentName: string) => {
    switch (agentName) {
      case 'Clarifier':
        return <MessageSquare className="w-6 h-6" />;
      case 'Innovator':
        return <TrendingUp className="w-6 h-6" />;
      case 'Critic':
        return <AlertCircle className="w-6 h-6" />;
      case 'Synthesizer':
        return <Users className="w-6 h-6" />;
      case 'Verifier':
        return <Shield className="w-6 h-6" />;
      case 'Summarizer':
        return <FileText className="w-6 h-6" />;
      default:
        return <Brain className="w-6 h-6" />;
    }
  };

  const getAgentDescription = (agentName: string) => {
    switch (agentName) {
      case 'Clarifier':
        return '澄清想法，提取关键信息槽位';
      case 'Innovator':
        return '生成创新建议和改进方案';
      case 'Critic':
        return '批评分析，识别问题和风险';
      case 'Synthesizer':
        return '综合优化，生成迭代版本';
      case 'Verifier':
        return '验证可行性和合规性';
      case 'Summarizer':
        return '生成最终报告和建议';
      default:
        return 'AI 智能体';
    }
  };

  if (!session) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-center">
          <Brain className="w-16 h-16 text-gray-400 mx-auto mb-4" />
          <h2 className="text-xl font-semibold text-gray-700 mb-2">未找到会话</h2>
          <p className="text-gray-500">请从想法输入页面开始创建新的优化会话</p>
        </div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* 页面标题和控制 */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
            AI 工作区
          </h1>
          <p className="text-gray-600 dark:text-gray-400 mt-1">
            多智能体协作优化进行中
          </p>
        </div>
        
        <div className="flex space-x-3">
          <button
            onClick={startOptimization}
            disabled={isProcessing}
            className="flex items-center px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <Play className="w-4 h-4 mr-2" />
            开始
          </button>
          
          <button
            onClick={pauseOptimization}
            disabled={!isProcessing}
            className="flex items-center px-4 py-2 bg-yellow-600 text-white rounded-lg hover:bg-yellow-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
          >
            <Pause className="w-4 h-4 mr-2" />
            暂停
          </button>
          
          <button
            onClick={resetOptimization}
            className="flex items-center px-4 py-2 bg-gray-600 text-white rounded-lg hover:bg-gray-700 transition-colors"
          >
            <RotateCcw className="w-4 h-4 mr-2" />
            重置
          </button>
        </div>
      </div>

      {/* 会话信息 */}
      <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          当前会话信息
        </h2>
        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="text-sm font-medium text-gray-500 dark:text-gray-400">想法描述</label>
            <p className="text-gray-900 dark:text-white mt-1">{session.idea_seed.raw_text}</p>
          </div>
          <div>
            <label className="text-sm font-medium text-gray-500 dark:text-gray-400">领域</label>
            <p className="text-gray-900 dark:text-white mt-1">{session.idea_seed.domain || '未指定'}</p>
          </div>
          {session.idea_seed.context_hints.length > 0 && (
            <div className="md:col-span-2">
              <label className="text-sm font-medium text-gray-500 dark:text-gray-400">上下文提示</label>
              <div className="flex flex-wrap gap-2 mt-1">
                {session.idea_seed.context_hints.map((hint, index) => (
                  <span
                    key={index}
                    className="px-2 py-1 bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300 text-sm rounded-md"
                  >
                    {hint}
                  </span>
                ))}
              </div>
            </div>
          )}
        </div>
      </div>

      {/* Agent 状态面板 */}
      <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-6">
          Agent 执行状态
        </h2>
        
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          {agentStates.map((agent) => (
            <div
              key={agent.name}
              className={`
                p-4 rounded-lg border-2 transition-all duration-200
                ${agent.status === 'running' ? 'border-blue-500 bg-blue-50 dark:bg-blue-900/20' : ''}
                ${agent.status === 'completed' ? 'border-green-500 bg-green-50 dark:bg-green-900/20' : ''}
                ${agent.status === 'error' ? 'border-red-500 bg-red-50 dark:bg-red-900/20' : ''}
                ${agent.status === 'pending' ? 'border-gray-200 dark:border-gray-700' : ''}
              `}
            >
              <div className="flex items-center justify-between mb-3">
                <div className="flex items-center space-x-2">
                  <div className={`
                    p-2 rounded-lg
                    ${agent.status === 'running' ? 'bg-blue-500 text-white' : ''}
                    ${agent.status === 'completed' ? 'bg-green-500 text-white' : ''}
                    ${agent.status === 'error' ? 'bg-red-500 text-white' : ''}
                    ${agent.status === 'pending' ? 'bg-gray-100 dark:bg-gray-700 text-gray-600 dark:text-gray-300' : ''}
                  `}>
                    {getAgentIcon(agent.name)}
                  </div>
                  <div>
                    <h3 className="font-medium text-gray-900 dark:text-white">
                      {agent.name}
                    </h3>
                    <p className="text-xs text-gray-500 dark:text-gray-400">
                      {getAgentDescription(agent.name)}
                    </p>
                  </div>
                </div>
                {getStatusIcon(agent.status)}
              </div>
              
              <div className="mb-2">
                <div className="flex justify-between items-center mb-1">
                  <span className="text-sm text-gray-600 dark:text-gray-400">进度</span>
                  <span className="text-sm font-medium text-gray-900 dark:text-white">
                    {agent.progress}%
                  </span>
                </div>
                <div className="w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2">
                  <div
                    className={`
                      h-2 rounded-full transition-all duration-300
                      ${agent.status === 'running' ? 'bg-blue-500' : ''}
                      ${agent.status === 'completed' ? 'bg-green-500' : ''}
                      ${agent.status === 'error' ? 'bg-red-500' : ''}
                      ${agent.status === 'pending' ? 'bg-gray-300 dark:bg-gray-600' : ''}
                    `}
                    style={{ width: `${agent.progress}%` }}
                  ></div>
                </div>
              </div>
              
              <p className="text-sm text-gray-600 dark:text-gray-400">
                {agent.message}
              </p>
              
              {agent.startTime && (
                <div className="mt-2 text-xs text-gray-500 dark:text-gray-400">
                  开始时间: {new Date(agent.startTime).toLocaleTimeString()}
                </div>
              )}
            </div>
          ))}
        </div>
      </div>

      {/* 实时日志 */}
      <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
        <h2 className="text-lg font-semibold text-gray-900 dark:text-white mb-4">
          执行日志
        </h2>
        
        <div className="bg-gray-50 dark:bg-gray-900 rounded-lg p-4 h-64 overflow-y-auto font-mono text-sm">
          {logs.length === 0 ? (
            <div className="flex items-center justify-center h-full text-gray-500 dark:text-gray-400">
              暂无日志信息
            </div>
          ) : (
            <div className="space-y-1">
              {logs.map((log, index) => (
                <div key={index} className="text-gray-700 dark:text-gray-300">
                  {log}
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};

export default WorkspacePage;
