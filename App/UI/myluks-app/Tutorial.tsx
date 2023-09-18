import './Tutorial.css';

import React, { useState, memo } from 'react';
import TutorialData from './TutorialData';

interface TutorialItemProps {
  id: string; // Añadido id a las propiedades
  title: string;
  content: string;
}

const TutorialItem: React.FC<TutorialItemProps> = memo(({ id, title, content }) => {
  const [isOpen, setIsOpen] = useState(false);

  const headerId = `header-${id}`;
  const contentId = `content-${id}`;

  return (
    <div className="tutorial-item">
      <button 
        id={headerId}
        className="tutorial-header" 
        onClick={() => setIsOpen(prev => !prev)}
        aria-expanded={isOpen}
        aria-controls={contentId}
      >
        {title} <span>{isOpen ? '-' : '+'}</span>
      </button>
      <div 
        id={contentId}
        className={`tutorial-content ${isOpen ? 'open' : ''}`}
        aria-labelledby={headerId} // Asocia el contenido con el botón
      >
        <p>{content}</p>
      </div>
    </div>
  );
});

const Tutorial: React.FC = () => {
  const data = TutorialData();

  return (
    <div className="tutorial-section">
      <h2>Tutorial</h2>
      {data.map(({ id, title, content }) => (
        <TutorialItem key={id} id={id} title={title} content={content} /> // Pasando id como prop
      ))}
    </div>
  );
}

export default Tutorial;