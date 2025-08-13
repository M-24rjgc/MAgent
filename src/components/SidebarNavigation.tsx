import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { 
  Home, 
  Lightbulb, 
  MessageSquare, 
  Users, 
  FolderOpen, 
  Settings,
  Brain
} from 'lucide-react';

interface NavItem {
  path: string;
  label: string;
  icon: React.ComponentType<any>;
  description: string;
}

const navItems: NavItem[] = [
  {
    path: '/',
    label: '首页',
    icon: Home,
    description: '概览和快速操作'
  },
  {
    path: '/idea-input',
    label: '想法输入',
    icon: Lightbulb,
    description: '输入和完善您的创意'
  },
  {
    path: '/questioning',
    label: '智能问答',
    icon: MessageSquare,
    description: 'AI 澄清和深入挖掘'
  },
  {
    path: '/workspace',
    label: '工作区',
    icon: Brain,
    description: 'Agent 协作优化过程'
  },
  {
    path: '/discussion',
    label: '协作讨论',
    icon: Users,
    description: '想法迭代和反馈'
  },
  {
    path: '/projects',
    label: '项目管理',
    icon: FolderOpen,
    description: '管理所有项目和会话'
  },
  {
    path: '/settings',
    label: '设置',
    icon: Settings,
    description: '系统配置和偏好'
  }
];

interface SidebarNavigationProps {
  isCollapsed?: boolean;
  onToggleCollapse?: () => void;
}

const SidebarNavigation: React.FC<SidebarNavigationProps> = ({ 
  isCollapsed = false,
  onToggleCollapse 
}) => {
  const location = useLocation();

  return (
    <div className={`
      bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-700
      transition-all duration-300 ease-in-out
      ${isCollapsed ? 'w-16' : 'w-64'}
      flex flex-col h-full
    `}>
      {/* Logo 区域 */}
      <div className="p-4 border-b border-gray-200 dark:border-gray-700">
        <div className="flex items-center space-x-3">
          <div className="w-8 h-8 bg-gradient-to-br from-blue-500 to-purple-600 rounded-lg flex items-center justify-center">
            <Brain className="w-5 h-5 text-white" />
          </div>
          {!isCollapsed && (
            <div>
              <h1 className="text-lg font-bold text-gray-900 dark:text-white">
                MAgent
              </h1>
              <p className="text-xs text-gray-500 dark:text-gray-400">
                多智能体概念优化引擎
              </p>
            </div>
          )}
        </div>
      </div>

      {/* 导航菜单 */}
      <nav className="flex-1 px-2 py-4 space-y-1">
        {navItems.map((item) => {
          const Icon = item.icon;
          const isActive = location.pathname === item.path;
          
          return (
            <Link
              key={item.path}
              to={item.path}
              className={`
                group flex items-center px-3 py-2 text-sm font-medium rounded-lg
                transition-all duration-200
                ${isActive
                  ? 'bg-blue-50 dark:bg-blue-900/20 text-blue-700 dark:text-blue-300 border-r-2 border-blue-500'
                  : 'text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-800 hover:text-gray-900 dark:hover:text-white'
                }
              `}
              title={isCollapsed ? item.label : undefined}
            >
              <Icon className={`
                flex-shrink-0 w-5 h-5
                ${isActive ? 'text-blue-600 dark:text-blue-400' : 'text-gray-400 group-hover:text-gray-500'}
                ${!isCollapsed ? 'mr-3' : ''}
              `} />
              
              {!isCollapsed && (
                <div className="flex-1 min-w-0">
                  <div className="font-medium">{item.label}</div>
                  <div className="text-xs text-gray-500 dark:text-gray-400 truncate">
                    {item.description}
                  </div>
                </div>
              )}
            </Link>
          );
        })}
      </nav>

      {/* 折叠按钮 */}
      {onToggleCollapse && (
        <div className="p-2 border-t border-gray-200 dark:border-gray-700">
          <button
            onClick={onToggleCollapse}
            className="w-full p-2 text-gray-500 hover:text-gray-700 dark:hover:text-gray-300 transition-colors"
            title={isCollapsed ? '展开侧边栏' : '折叠侧边栏'}
          >
            <div className={`transform transition-transform ${isCollapsed ? 'rotate-180' : ''}`}>
              ◀
            </div>
          </button>
        </div>
      )}
    </div>
  );
};

export default SidebarNavigation;
