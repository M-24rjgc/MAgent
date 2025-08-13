import React, { useState } from 'react';
import { 
  ArrowLeft, Plus, Search, Filter, Clock, Users, 
  CheckCircle, AlertCircle, Pause, TrendingUp, 
  Calendar, Tag, Star, MoreHorizontal
} from 'lucide-react';
import { useNavigate } from 'react-router-dom';

interface Project {
  id: string;
  title: string;
  description: string;
  status: 'active' | 'paused' | 'completed' | 'draft';
  priority: 'high' | 'medium' | 'low';
  progress: number;
  team: string;
  createdAt: Date;
  dueDate?: Date;
  tags: string[];
  collaborators: number;
  iterations: number;
}

type FilterType = 'all' | 'active' | 'paused' | 'completed' | 'draft';
type SortType = 'recent' | 'priority' | 'progress' | 'name';

export const ProjectsPage: React.FC = () => {
  const navigate = useNavigate();
  const [searchTerm, setSearchTerm] = useState('');
  const [activeFilter, setActiveFilter] = useState<FilterType>('all');
  const [sortBy, setSortBy] = useState<SortType>('recent');
  const [viewMode, setViewMode] = useState<'grid' | 'list'>('grid');

  // 模拟项目数据
  const projects: Project[] = [
    {
      id: '1',
      title: '在线教育平台',
      description: '构建一个基于AI的个性化在线学习平台，专注于职业技能培训',
      status: 'active',
      priority: 'high',
      progress: 75,
      team: 'AI研发团队',
      createdAt: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000),
      dueDate: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000),
      tags: ['AI', '教育', '平台'],
      collaborators: 5,
      iterations: 12
    },
    {
      id: '2',
      title: '智能客服系统',
      description: '开发多模态智能客服机器人，支持文本、语音和图像交互',
      status: 'active',
      priority: 'medium',
      progress: 45,
      team: '产品团队',
      createdAt: new Date(Date.now() - 14 * 24 * 60 * 60 * 1000),
      dueDate: new Date(Date.now() + 45 * 24 * 60 * 60 * 1000),
      tags: ['AI', '客服', '多模态'],
      collaborators: 3,
      iterations: 8
    },
    {
      id: '3',
      title: '区块链投票系统',
      description: '基于区块链技术的透明化投票平台，确保投票过程的公正性',
      status: 'paused',
      priority: 'low',
      progress: 20,
      team: '工程团队',
      createdAt: new Date(Date.now() - 21 * 24 * 60 * 60 * 1000),
      tags: ['区块链', '投票', '去中心化'],
      collaborators: 4,
      iterations: 3
    },
    {
      id: '4',
      title: '健康管理应用',
      description: '个人健康数据追踪和AI健康建议系统',
      status: 'completed',
      priority: 'medium',
      progress: 100,
      team: 'AI研发团队',
      createdAt: new Date(Date.now() - 90 * 24 * 60 * 60 * 1000),
      dueDate: new Date(Date.now() - 7 * 24 * 60 * 60 * 1000),
      tags: ['健康', 'AI', '移动应用'],
      collaborators: 6,
      iterations: 18
    },
    {
      id: '5',
      title: '智能家居控制器',
      description: '统一的智能家居设备管理和自动化控制系统',
      status: 'draft',
      priority: 'low',
      progress: 5,
      team: '工程团队',
      createdAt: new Date(Date.now() - 3 * 24 * 60 * 60 * 1000),
      tags: ['IoT', '智能家居', '自动化'],
      collaborators: 2,
      iterations: 1
    }
  ];

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'active': return 'bg-green-100 text-green-800';
      case 'paused': return 'bg-yellow-100 text-yellow-800';
      case 'completed': return 'bg-blue-100 text-blue-800';
      case 'draft': return 'bg-gray-100 text-gray-800';
      default: return 'bg-gray-100 text-gray-800';
    }
  };

  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'active': return <CheckCircle className="w-4 h-4" />;
      case 'paused': return <Pause className="w-4 h-4" />;
      case 'completed': return <CheckCircle className="w-4 h-4" />;
      case 'draft': return <AlertCircle className="w-4 h-4" />;
      default: return <AlertCircle className="w-4 h-4" />;
    }
  };

  const getPriorityColor = (priority: string) => {
    switch (priority) {
      case 'high': return 'text-red-600 bg-red-50';
      case 'medium': return 'text-yellow-600 bg-yellow-50';
      case 'low': return 'text-green-600 bg-green-50';
      default: return 'text-gray-600 bg-gray-50';
    }
  };

  const getStatusText = (status: string) => {
    switch (status) {
      case 'active': return '进行中';
      case 'paused': return '已暂停';
      case 'completed': return '已完成';
      case 'draft': return '草稿';
      default: return status;
    }
  };

  const getPriorityText = (priority: string) => {
    switch (priority) {
      case 'high': return '高优先级';
      case 'medium': return '中优先级';
      case 'low': return '低优先级';
      default: return priority;
    }
  };

  const filteredProjects = projects.filter(project => {
    const matchesSearch = project.title.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         project.description.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         project.tags.some(tag => tag.toLowerCase().includes(searchTerm.toLowerCase()));
    
    const matchesFilter = activeFilter === 'all' || project.status === activeFilter;
    
    return matchesSearch && matchesFilter;
  });

  const getFilterCount = (filter: FilterType) => {
    if (filter === 'all') return projects.length;
    return projects.filter(p => p.status === filter).length;
  };

  const formatDate = (date: Date) => {
    return date.toLocaleDateString('zh-CN', { 
      year: 'numeric', 
      month: 'short', 
      day: 'numeric' 
    });
  };

  const ProjectCard: React.FC<{ project: Project }> = ({ project }) => (
    <div className="bg-white rounded-xl shadow-sm border border-gray-200 p-6 hover:shadow-lg transition-all duration-300 hover:-translate-y-1">
      <div className="flex items-start justify-between mb-4">
        <div className="flex-1">
          <div className="flex items-center space-x-2 mb-2">
            <h3 className="text-lg font-semibold text-gray-900">{project.title}</h3>
            <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${getPriorityColor(project.priority)}`}>
              {getPriorityText(project.priority)}
            </span>
          </div>
          <p className="text-gray-600 text-sm leading-relaxed mb-3">{project.description}</p>
        </div>
        <button className="p-2 hover:bg-gray-100 rounded-lg transition-colors">
          <MoreHorizontal className="w-4 h-4 text-gray-500" />
        </button>
      </div>

      {/* 进度条 */}
      <div className="mb-4">
        <div className="flex justify-between text-sm mb-2">
          <span className="text-gray-600">项目进度</span>
          <span className="font-medium text-gray-900">{project.progress}%</span>
        </div>
        <div className="h-2 bg-gray-200 rounded-full overflow-hidden">
          <div 
            className="h-full bg-gradient-to-r from-blue-500 to-purple-600 rounded-full transition-all duration-500"
            style={{ width: `${project.progress}%` }}
          />
        </div>
      </div>

      {/* 标签 */}
      <div className="flex flex-wrap gap-2 mb-4">
        {project.tags.map((tag, index) => (
          <span 
            key={index}
            className="inline-flex items-center px-2 py-1 rounded-md text-xs font-medium bg-gray-100 text-gray-700"
          >
            <Tag className="w-3 h-3 mr-1" />
            {tag}
          </span>
        ))}
      </div>

      {/* 底部信息 */}
      <div className="flex items-center justify-between text-sm">
        <div className="flex items-center space-x-4">
          <div className="flex items-center space-x-1 text-gray-600">
            <Users className="w-4 h-4" />
            <span>{project.collaborators}</span>
          </div>
          <div className="flex items-center space-x-1 text-gray-600">
            <TrendingUp className="w-4 h-4" />
            <span>{project.iterations}轮</span>
          </div>
          <div className="flex items-center space-x-1 text-gray-600">
            <Calendar className="w-4 h-4" />
            <span>{formatDate(project.createdAt)}</span>
          </div>
        </div>
        <div className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${getStatusColor(project.status)}`}>
          {getStatusIcon(project.status)}
          <span className="ml-1">{getStatusText(project.status)}</span>
        </div>
      </div>
    </div>
  );

  return (
    <div className="min-h-screen bg-gray-50">
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
            <h1 className="text-xl font-semibold text-gray-900">项目管理</h1>
          </div>
          <div className="flex items-center space-x-4">
            <button className="px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-lg hover:opacity-90 transition-opacity flex items-center space-x-2">
              <Plus className="w-4 h-4" />
              <span>新建项目</span>
            </button>
          </div>
        </div>
      </nav>

      <div className="flex">
        {/* 左侧筛选面板 */}
        <div className="w-80 bg-white border-r border-gray-200 p-6">
          <div className="space-y-6">
            {/* 搜索 */}
            <div>
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 w-4 h-4 text-gray-400" />
                <input 
                  type="text" 
                  placeholder="搜索项目..." 
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                />
              </div>
            </div>

            {/* 状态筛选 */}
            <div>
              <h3 className="text-sm font-medium text-gray-900 mb-3">项目状态</h3>
              <div className="space-y-2">
                {([
                  { key: 'all', label: '全部项目', color: 'bg-blue-500' },
                  { key: 'active', label: '进行中', color: 'bg-green-500' },
                  { key: 'paused', label: '暂停', color: 'bg-yellow-500' },
                  { key: 'completed', label: '已完成', color: 'bg-gray-500' },
                  { key: 'draft', label: '草稿', color: 'bg-gray-400' }
                ] as const).map((filter) => (
                  <button
                    key={filter.key}
                    onClick={() => setActiveFilter(filter.key)}
                    className={`w-full text-left px-3 py-2 rounded-lg text-sm transition-colors flex items-center justify-between ${
                      activeFilter === filter.key 
                        ? 'bg-gradient-to-r from-blue-600 to-purple-600 text-white' 
                        : 'hover:bg-gray-100 text-gray-700'
                    }`}
                  >
                    <div className="flex items-center">
                      <span className={`w-2 h-2 rounded-full mr-2 ${filter.color}`}></span>
                      {filter.label}
                    </div>
                    <span className="text-xs">({getFilterCount(filter.key)})</span>
                  </button>
                ))}
              </div>
            </div>

            {/* 排序 */}
            <div>
              <h3 className="text-sm font-medium text-gray-900 mb-3">排序方式</h3>
              <select 
                value={sortBy}
                onChange={(e) => setSortBy(e.target.value as SortType)}
                className="w-full p-2 border border-gray-300 rounded-lg text-sm focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
              >
                <option value="recent">最近创建</option>
                <option value="priority">优先级</option>
                <option value="progress">进度</option>
                <option value="name">名称</option>
              </select>
            </div>

            {/* 视图模式 */}
            <div>
              <h3 className="text-sm font-medium text-gray-900 mb-3">视图模式</h3>
              <div className="flex space-x-2">
                <button
                  onClick={() => setViewMode('grid')}
                  className={`flex-1 px-3 py-2 rounded-lg text-xs font-medium transition-colors ${
                    viewMode === 'grid' 
                      ? 'bg-blue-100 text-blue-700' 
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  网格视图
                </button>
                <button
                  onClick={() => setViewMode('list')}
                  className={`flex-1 px-3 py-2 rounded-lg text-xs font-medium transition-colors ${
                    viewMode === 'list' 
                      ? 'bg-blue-100 text-blue-700' 
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  列表视图
                </button>
              </div>
            </div>
          </div>
        </div>

        {/* 右侧项目列表 */}
        <div className="flex-1 p-8">
          <div className="max-w-6xl mx-auto">
            {/* 头部统计 */}
            <div className="grid grid-cols-4 gap-6 mb-8">
              <div className="bg-white rounded-xl p-6 border border-gray-200">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-sm text-gray-600">总项目</p>
                    <p className="text-2xl font-bold text-gray-900">{projects.length}</p>
                  </div>
                  <div className="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center">
                    <Star className="w-6 h-6 text-blue-600" />
                  </div>
                </div>
              </div>
              <div className="bg-white rounded-xl p-6 border border-gray-200">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-sm text-gray-600">进行中</p>
                    <p className="text-2xl font-bold text-green-600">{getFilterCount('active')}</p>
                  </div>
                  <div className="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center">
                    <TrendingUp className="w-6 h-6 text-green-600" />
                  </div>
                </div>
              </div>
              <div className="bg-white rounded-xl p-6 border border-gray-200">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-sm text-gray-600">已完成</p>
                    <p className="text-2xl font-bold text-blue-600">{getFilterCount('completed')}</p>
                  </div>
                  <div className="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center">
                    <CheckCircle className="w-6 h-6 text-blue-600" />
                  </div>
                </div>
              </div>
              <div className="bg-white rounded-xl p-6 border border-gray-200">
                <div className="flex items-center justify-between">
                  <div>
                    <p className="text-sm text-gray-600">本月新增</p>
                    <p className="text-2xl font-bold text-purple-600">3</p>
                  </div>
                  <div className="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center">
                    <Plus className="w-6 h-6 text-purple-600" />
                  </div>
                </div>
              </div>
            </div>

            {/* 工具栏 */}
            <div className="flex items-center justify-between mb-6">
              <div className="flex items-center space-x-4">
                <h2 className="text-lg font-semibold text-gray-900">
                  {activeFilter === 'all' ? '全部项目' : getStatusText(activeFilter)}
                </h2>
                <span className="text-sm text-gray-500">
                  共 {filteredProjects.length} 个项目
                </span>
              </div>
              <div className="flex items-center space-x-2">
                <button className="px-3 py-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-lg transition-colors flex items-center space-x-2">
                  <Filter className="w-4 h-4" />
                  <span className="text-sm">筛选</span>
                </button>
                <button className="px-3 py-2 text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-lg transition-colors flex items-center space-x-2">
                  <Clock className="w-4 h-4" />
                  <span className="text-sm">时间线</span>
                </button>
              </div>
            </div>

            {/* 项目网格 */}
            {viewMode === 'grid' ? (
              <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {filteredProjects.map((project) => (
                  <ProjectCard key={project.id} project={project} />
                ))}
              </div>
            ) : (
              // 列表视图 - 简化实现
              <div className="space-y-4">
                {filteredProjects.map((project) => (
                  <div key={project.id} className="bg-white rounded-lg border border-gray-200 p-4">
                    <div className="flex items-center justify-between">
                      <div className="flex-1">
                        <h3 className="font-semibold text-gray-900">{project.title}</h3>
                        <p className="text-sm text-gray-600 mt-1">{project.description}</p>
                      </div>
                      <div className="flex items-center space-x-4">
                        <span className="text-sm text-gray-500">{project.progress}%</span>
                        <span className={`inline-flex items-center px-2 py-1 rounded-full text-xs font-medium ${getStatusColor(project.status)}`}>
                          {getStatusText(project.status)}
                        </span>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            )}

            {/* 空状态 */}
            {filteredProjects.length === 0 && (
              <div className="text-center py-12">
                <div className="w-16 h-16 mx-auto mb-4 bg-gray-100 rounded-full flex items-center justify-center">
                  <Search className="w-8 h-8 text-gray-400" />
                </div>
                <h3 className="text-lg font-medium text-gray-900 mb-2">没有找到项目</h3>
                <p className="text-gray-600 mb-6">尝试调整搜索条件或创建新项目</p>
                <button className="px-4 py-2 bg-gradient-to-r from-blue-600 to-purple-600 text-white rounded-lg hover:opacity-90 transition-opacity">
                  创建第一个项目
                </button>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
