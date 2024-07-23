import XmlReader from './pages/XmlReader';
import { HashRouter as Router, Route, Routes } from 'react-router-dom';
import Layout from './components/Layout';
import Home from './pages/Home';
import Settings from './pages/Settings';

const App = () => {
  return (
    <Router>
      <Layout>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/xmlReader" element={<XmlReader />} />
          <Route path="/settings" element={<Settings />} />
        </Routes>
      </Layout>
    </Router>

  );
};

export default App;