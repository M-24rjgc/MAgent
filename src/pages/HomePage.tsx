import React, { useState, useEffect } from 'react';
import { Link } from 'react-router-dom';
import { 
  Lightbulb, 
  Brain, 
  TrendingUp, 
  Clock, 
  FolderOpen,
  Plus,
  BarChart3,
  Users
} from 'lucide-react';
import { OptimizationSession } from '../types';

const HomePage: React.FC = () => {
  const [recentSessions, setRecentSessions] = useState<OptimizationSession[]>([]);
  const [stats, setStats] = useState({
    totalProjects: 0,
    completedSessions: 0,
    avgOptimizationTime: 0,
    successRate: 0
  });
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    loadDashboardData();
  }, []);

  const loadDashboardData = async () => {
    try {
      setLoading(true);
      // TODO: 实现后端 API 调用
      // const sessions = await invoke<OptimizationSession[]>('get_recent_sessions');
      // const statsData = await invoke<any>('get_dashboard_stats');
      
      // 模拟数据
      setTimeout(() => {
        setRecentSessions([]);
        setStats({
          totalProjects: 12,
          completedSessions: 45,
          avgOptimizationTime: 8.5,
          successRate: 92
        });
        setLoading(false);
      }, 1000);
    } catch (error) {
      console.error('Failed to load dashboard data:', error);
      setLoading(false);
    }
  };

  const quickActionCards = [
    {
      title: '新建想法',
      description: '输入您的创意，开始优化之旅',
      icon: Lightbulb,
      to: '/idea-input',
      color: 'from-blue-500 to-blue-600',
      textColor: 'text-blue-600'
    },
    {
      title: '智能工作区',
      description: '查看 Agent 协作优化过程',
      icon: Brain,
      to: '/workspace',
      color: 'from-purple-500 to-purple-600',
      textColor: 'text-purple-600'
    },
    {
      title: '项目管理',
      description: '管理所有项目和历史记录',
      icon: FolderOpen,
      to: '/projects',
      color: 'from-green-500 to-green-600',
      textColor: 'text-green-600'
    }
  ];

  const StatCard: React.FC<{
    title: string;
    value: string | number;
    subtitle: string;
    icon: React.ComponentType<any>;
    trend?: string;
  }> = ({ title, value, subtitle, icon: Icon, trend }) => (
    <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm font-medium text-gray-600 dark:text-gray-400">{title}</p>
          <p className="text-2xl font-bold text-gray-900 dark:text-white mt-2">{value}</p>
          <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">{subtitle}</p>
        </div>
        <div className="w-12 h-12 bg-gray-50 dark:bg-gray-700 rounded-lg flex items-center justify-center">
          <Icon className="w-6 h-6 text-gray-600 dark:text-gray-300" />
        </div>
      </div>
      {trend && (
        <div className="mt-4 flex items-center text-sm text-green-600 dark:text-green-400">
          <TrendingUp className="w-4 h-4 mr-1" />
          {trend}
        </div>
      )}
    </div>
  );

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
      </div>
    );
  }

  return (
    <div className="space-y-6">
      {/* 页面标题 */}
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">
          多智能体概念优化引擎
        </h1>
        <p className="text-gray-600 dark:text-gray-400 mt-2">
          欢迎回来！让 AI 团队协作优化您的创意想法
        </p>
      </div>

      {/* 快速操作卡片 */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        {quickActionCards.map((card, index) => {
          const Icon = card.icon;
          return (
            <Link
              key={index}
              to={card.to}
              className="group block"
            >
              <div className="bg-white dark:bg-gray-800 rounded-xl p-6 shadow-sm border border-gray-200 dark:border-gray-700 hover:shadow-md transition-all duration-200 group-hover:scale-105">
                <div className="flex items-center mb-4">
                  <div className={`w-12 h-12 bg-gradient-to-r ${card.color} rounded-lg flex items-center justify-center`}>
                    <Icon className="w-6 h-6 text-white" />
                  </div>
                  <div className="ml-4">
                    <h3 className={`text-lg font-semibold ${card.textColor} dark:text-white`}>
                      {card.title}
                    </h3>
                  </div>
                </div>
                <p className="text-gray-600 dark:text-gray-400 text-sm">
                  {card.description}
                </p>
                <div className="mt-4 flex items-center text-sm text-blue-600 dark:text-blue-400">
                  <span>开始使用</span>
                  <Plus className="w-4 h-4 ml-1 group-hover:translate-x-1 transition-transform" />
                </div>
              </div>
            </Link>
          );
        })}
      </div>

      {/* 统计数据 */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6 mb-8">
        <StatCard
          title="总项目数"
          value={stats.totalProjects}
          subtitle="个活跃项目"
          icon={FolderOpen}
          trend="+2 本月"
        />
        <StatCard
          title="完成会话"
          value={stats.completedSessions}
          subtitle="次优化完成"
          icon={BarChart3}
          trend="+15% 提升"
        />
        <StatCard
          title="平均耗时"
          value={`${stats.avgOptimizationTime}分钟`}
          subtitle="每次优化"
          icon={Clock}
        />
        <StatCard
          title="成功率"
          value={`${stats.successRate}%`}
          subtitle="优化成功率"
          icon={TrendingUp}
          trend="+3% 提升"
        />
      </div>

      {/* 最近会话 */}
      <div className="bg-white dark:bg-gray-800 rounded-xl shadow-sm border border-gray-200 dark:border-gray-700">
        <div className="p-6 border-b border-gray-200 dark:border-gray-700">
          <div className="flex items-center justify-between">
            <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
              最近的优化会话
            </h2>
            <Link 
              to="/projects"
              className="text-sm text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors"
            >
              查看全部
            </Link>
          </div>
        </div>
        
        {recentSessions.length === 0 ? (
          <div className="p-12 text-center">
            <Users className="w-12 h-12 text-gray-400 mx-auto mb-4" />
            <h3 className="text-lg font-medium text-gray-900 dark:text-white mb-2">
              还没有优化会话
            </h3>
            <p className="text-gray-500 dark:text-gray-400 mb-4">
              创建您的第一个想法优化项目开始使用
            </p>
            <Link
              to="/idea-input"
              className="inline-flex items-center px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
            >
              <Plus className="w-4 h-4 mr-2" />
              开始新项目
            </Link>
          </div>
        ) : (
          <div className="divide-y divide-gray-200 dark:divide-gray-700">
            {recentSessions.map((session) => (
              <div key={session.id} className="p-6 hover:bg-gray-50 dark:hover:bg-gray-700/50 transition-colors">
                <div className="flex items-center justify-between">
                  <div className="flex-1">
                    <h3 className="font-medium text-gray-900 dark:text-white">
                      {session.idea_seed.raw_text.substring(0, 60)}...
                    </h3>
                    <p className="text-sm text-gray-500 dark:text-gray-400 mt-1">
                      状态: {session.current_state} • 创建于 {new Date(session.created_at).toLocaleDateString()}
                    </p>
                  </div>
                  <Link
                    to={`/workspace?session=${session.id}`}
                    className="ml-4 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 transition-colors"
                  >
                    继续
                  </Link>
                </div>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export default HomePage;
