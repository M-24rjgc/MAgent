// 核心数据类型定义
export interface IdeaSeed {
  raw_text: string;
  context_hints: string[];
  domain?: string;
}

export interface Clarification {
  question: string;
  answer?: string;
  slot_name: string;
  importance: number;
  suggested_answers?: string[];
}

export interface InnovationDelta {
  category: string;
  suggestion: string;
  rationale: string;
  impact_score: number;
  implementation_difficulty: number;
}

export interface CriticismPoint {
  category: string;
  issue: string;
  severity: number;
  evidence: string;
  suggested_fix?: string;
}

export interface IterationVersion {
  version_number: number;
  enhanced_idea: string;
  key_changes: string[];
  confidence_score: number;
  timestamp: string;
}

export interface VerificationReport {
  logical_consistency: number;
  factual_accuracy: number;
  feasibility: number;
  risk_assessment: number;
  overall_score: number;
  critical_issues: string[];
  recommendations: string[];
}

export interface Summary {
  executive_overview: string;
  key_strengths: string[];
  main_concerns: string[];
  final_recommendation: string;
  confidence_level: number;
  next_steps: string[];
}

export interface OptimizationSession {
  id: string;
  idea_seed: IdeaSeed;
  current_state: string;
  clarifications?: Clarification[];
  innovations?: InnovationDelta[];
  criticisms?: CriticismPoint[];
  iterations?: IterationVersion[];
  verification?: VerificationReport;
  summary?: Summary;
  created_at: string;
  updated_at: string;
}

// Agent 状态类型
export type AgentStatus = 'pending' | 'running' | 'completed' | 'error';

export interface AgentState {
  name: string;
  status: AgentStatus;
  progress: number;
  message?: string;
  startTime?: string;
  endTime?: string;
}

// UI 状态类型
export interface AppState {
  currentSession?: OptimizationSession;
  agentStates: AgentState[];
  isProcessing: boolean;
  error?: string;
}

// 项目管理类型
export interface Project {
  id: string;
  name: string;
  description: string;
  status: 'draft' | 'processing' | 'completed' | 'archived';
  sessions: OptimizationSession[];
  created_at: string;
  updated_at: string;
  tags: string[];
}

// 设置配置类型
export interface AppSettings {
  theme: 'light' | 'dark' | 'system';
  language: 'zh-CN' | 'en-US';
  aiModel: string;
  autoSave: boolean;
  notifications: boolean;
}
