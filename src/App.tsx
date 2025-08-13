import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Layout from './components/Layout';
import HomePage from './pages/HomePage';
import IdeaInputPage from './pages/IdeaInputPage';
import WorkspacePage from './pages/WorkspacePage';
import QuestioningPage from './pages/QuestioningPage';
import { DiscussionPage } from './pages/DiscussionPage';
import { ProjectsPage } from './pages/ProjectsPage';
import { SettingsPage } from './pages/SettingsPage';
import './App.css';

function App() {
  return (
    <Router>
      <div className="min-h-screen bg-gray-50 dark:bg-gray-900">
        <Layout>
          <Routes>
            <Route path="/" element={<HomePage />} />
            <Route path="/idea-input" element={<IdeaInputPage />} />
            <Route path="/workspace" element={<WorkspacePage />} />
            <Route path="/questioning" element={<QuestioningPage />} />
            <Route path="/discussion" element={<DiscussionPage />} />
            <Route path="/projects" element={<ProjectsPage />} />
            <Route path="/settings" element={<SettingsPage />} />
          </Routes>
        </Layout>
      </div>
    </Router>
  );
}

export default App;
