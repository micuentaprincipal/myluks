import './App.css';

import React, { lazy, Suspense } from 'react';
import { BrowserRouter as Router, Route, Link, Routes, Outlet, useLocation } from 'react-router-dom';
import FeedbackForm from './FeedbackForm';

const Tutorial = lazy(() => import('./Tutorial')); // Lazy Loading for Tutorial

// Componente para determinar si un enlace es activo
const ActiveLink: React.FC<{ to: string, children: React.ReactNode }> = ({ to, children }) => {
  const location = useLocation();
  const isActive = location.pathname === to ? 'active-link' : '';

  return <Link to={to} className={isActive} aria-current={isActive ? 'page' : undefined}>{children}</Link>;
}

// Componente de Navegación
const Navigation: React.FC = () => {
  return (
    <nav aria-label="Navegación principal">
      <ul>
        <li><ActiveLink to="/">Inicio</ActiveLink></li>
        <li><ActiveLink to="/tutorial">Tutorial</ActiveLink></li>
        <li><ActiveLink to="/feedback">Feedback</ActiveLink></li>
      </ul>
    </nav>
  );
}

const Home: React.FC = () => {
  return (
    <div>
      <h2>Bienvenido a MyLuks Wallet</h2>
      <p>Una breve descripción de tu aplicación aquí.</p>
    </div>
  );
}

const NotFound: React.FC = () => {
  return <div>Página no encontrada</div>;
}

function App() {
  return (
    <Router>
      <div className="App">
        <header className="App-header">
          <h1>MyLuks Wallet</h1>
          <Navigation />
        </header>
        <main>
          <Suspense fallback={<div>Cargando...</div>}>
            <Routes>
              <Route path="/" element={<Home />} />
              <Route path="/tutorial" element={<Tutorial />} />
              <Route path="/feedback" element={<FeedbackForm />} />
              <Route path="*" element={<NotFound />} />
            </Routes>
          </Suspense>
        </main>
        <footer>
          <p>&copy; 2023 MyLuks Wallet. Todos los derechos reservados.</p>
        </footer>
      </div>
    </Router>
  );
}

export default App;