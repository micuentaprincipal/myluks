import React from 'react';
import { render, screen } from '@testing-library/react';
import App from './App';

// Comprobación de que el título principal se renderiza
test('renders MyLuks Wallet title', () => {
  render(<App />);
  const titleElement = screen.getByText(/MyLuks Wallet/i);
  expect(titleElement).toBeInTheDocument();
});

// Comprobación de que los enlaces de navegación se renderizan
test('renders navigation links', () => {
  render(<App />);
  
  const homeLink = screen.getByText(/Inicio/i);
  expect(homeLink).toBeInTheDocument();

  const tutorialLink = screen.getByText(/Tutorial/i);
  expect(tutorialLink).toBeInTheDocument();

  const feedbackLink = screen.getByText(/Feedback/i);
  expect(feedbackLink).toBeInTheDocument();
});

// Comprobación de que el pie de página se renderiza con el texto de derechos de autor
test('renders footer with copyright text', () => {
  render(<App />);
  
  const footerText = screen.getByText(/© 2023 MyLuks Wallet. Todos los derechos reservados./i);
  expect(footerText).toBeInTheDocument();
});