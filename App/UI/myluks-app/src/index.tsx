// Bibliotecas externas
import React from 'react';
import ReactDOM from 'react-dom';

// Componentes  
import App from './App';

// Estilos
import './index.css';

/**
 * El punto de entrada principal de la aplicación React.
 * Renderiza el componente App dentro del elemento con el ID 'root'.
 */
ReactDOM.render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
  document.getElementById('root')
);
