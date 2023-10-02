import './App.css';
import React, { lazy, Suspense, useState, createContext, useContext, useEffect } from 'react';
import { BrowserRouter as Router, Route, Link, Routes, useLocation, Navigate } from 'react-router-dom';

const Tutorial = lazy(() => import('./Tutorial'));
const FeedbackForm = lazy(() => import('./FeedbackForm'));

export const AuthContext = createContext<{
  isAuthenticated: boolean;
  email: string;
  toggleAuthentication: () => void;
  setEmail: (email: string) => void;
}>({
  isAuthenticated: false,
  email: '',
  toggleAuthentication: () => {},
  setEmail: () => {},
});

export const ThemeContext = createContext<{ darkMode: boolean; toggleDarkMode: () => void }>({
  darkMode: false,
  toggleDarkMode: () => {},
});

// Crear un simple sistema de notificaciones:
const NotificationContext = createContext<{ message?: string; showMessage: (msg: string) => void }>({
  message: undefined,
  showMessage: () => {},
});

// Componente de notificación
const Notification: React.FC = () => {
  const { message } = useContext(NotificationContext);
  return message ? <div className="notification">{message}</div> : null;
};

interface ActiveLinkProps {
  to: string;
  children: React.ReactNode;
}

const ActiveLink: React.FC<ActiveLinkProps> = ({ to, children }) => {
  const location = useLocation();
  const isActive = location.pathname === to;

  return (
    <Link to={to} className={isActive ? 'active-link' : ''} aria-current={isActive ? 'page' : undefined}>
      {children}
    </Link>
  );
};

interface NavigationProps {
  toggleDarkMode: () => void;
  darkMode: boolean;
}

const Navigation: React.FC<NavigationProps> = ({ toggleDarkMode, darkMode }) => (
  <nav aria-label="Navegación principal">
    <ul role="menu">
      <li role="none"><ActiveLink to="/">Inicio</ActiveLink></li>
      <li role="none"><ActiveLink to="/tutorial">Tutorial</ActiveLink></li>
      <li role="none"><ActiveLink to="/feedback">Feedback</ActiveLink></li>
    </ul>
    <button onClick={toggleDarkMode}>{darkMode ? 'Modo Claro' : 'Modo Oscuro'}</button>
  </nav>
);

interface AuthenticationProps {
  isAuthenticated: boolean;
  toggleAuth: () => void;
}

const Authentication: React.FC<AuthenticationProps> = ({ isAuthenticated, toggleAuth }) => (
  <div>
    {isAuthenticated ? (
      <>
        <p>Estás autenticado. ¡Bienvenido de nuevo!</p>
        <button onClick={toggleAuth}>Cerrar sesión</button>
      </>
    ) : (
      <>
        <p>No estás autenticado. Por favor, inicia sesión para continuar.</p>
        <button onClick={toggleAuth}>Iniciar sesión</button>
      </>
    )}
  </div>
);

interface HomeProps {
  isAuthenticated: boolean;
}

const Home: React.FC<HomeProps> = ({ isAuthenticated }) => (
  <>
    <h2>Bienvenido a MyLuks Wallet</h2>
    <p>{isAuthenticated ? "Gracias por volver." : "Una breve descripción de tu aplicación aquí."}</p>
  </>
);

const NotFound: React.FC = () => <div role="alert">Página no encontrada</div>;

interface ProtectedRouteProps {
  isAuthenticated: boolean;
  children: React.ReactNode;
}

const ProtectedRoute: React.FC<ProtectedRouteProps> = ({ isAuthenticated, children }) => {
  if (!isAuthenticated) {
    return <div>Por favor, inicia sesión para ver esta página.</div>;
  }
  return <>{children}</>;
};

interface ErrorBoundaryProps {
  children: React.ReactNode;
}

interface ErrorBoundaryState {
  hasError: boolean;
  errorInfo: any;
}

class ErrorBoundary extends React.Component<ErrorBoundaryProps, ErrorBoundaryState> {
  constructor(props: ErrorBoundaryProps) {
    super(props);
    this.state = { hasError: false, errorInfo: null };
  }

  static getDerivedStateFromError() {
    return { hasError: true };
  }

  componentDidCatch(error: any, errorInfo: any) {
    console.error("Caught an error:", error, errorInfo);
    this.setState({ errorInfo });
  }

  render() {
    if (this.state.hasError) {
      return <div className="error-message">Ha ocurrido un error. Por favor, intenta recargar la página.</div>;
    }
    return this.props.children;
  }
}

const App: React.FC = () => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const toggleAuthentication = () => setIsAuthenticated(prev => !prev);

  const [email, setEmail] = useState('');  // Nuevo estado para el email

  const [darkMode, setDarkMode] = useState(false);
  const toggleDarkMode = () => setDarkMode(prev => !prev);

  useEffect(() => {
    const themeClass = darkMode ? 'dark-mode' : 'light-mode';
    document.body.className = themeClass;
  }, [darkMode]);

  // Lógica para mostrar notificaciones:
  const [message, setMessage] = useState<string | undefined>();
  const showMessage = (msg: string) => setMessage(msg);

  return (
    <Router>
      <AuthContext.Provider value={{ isAuthenticated, email, toggleAuthentication, setEmail }}>
        <ThemeContext.Provider value={{ darkMode, toggleDarkMode }}>
          {/* Proveer el contexto de notificación */}
          <NotificationContext.Provider value={{ message, showMessage }}>
            <div className="App">
              <header className="App-header">
                <h1>MyLuks Wallet</h1>
                <Navigation toggleDarkMode={toggleDarkMode} darkMode={darkMode} />
              </header>
              <Authentication isAuthenticated={isAuthenticated} toggleAuth={toggleAuthentication} />
              <main role="main">
                <ErrorBoundary>
                  <Suspense fallback={<div>Cargando...</div>}>
                    <Routes>
                      <Route path="/" element={<Home isAuthenticated={isAuthenticated} />} />
                      <Route path="/tutorial" element={<Tutorial />} />
                      <Route path="/feedback" element={<FeedbackForm />} />
                      <Route path="*" element={<NotFound />} />
                    </Routes>
                  </Suspense>
                </ErrorBoundary>
              </main>
              <footer>
                <p>&copy; {new Date().getFullYear()} MyLuks Wallet. Todos los derechos reservados.</p>
              </footer>
              
              {/* Componente de notificación */}
              <Notification />
            </div>
          </NotificationContext.Provider>
        </ThemeContext.Provider>
      </AuthContext.Provider>
    </Router>
  );
};

export default App;
