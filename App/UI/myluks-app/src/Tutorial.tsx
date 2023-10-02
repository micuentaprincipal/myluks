import './Tutorial.css';
import React, { useState, memo, Fragment, useEffect, useContext } from 'react';
import TutorialData from './TutorialData';
import { AuthContext } from './App';  // <-- Importamos el contexto de autenticación

interface TutorialItemProps {
  id: string;
  title: string;
  content: string;
  imageUrl?: string;
  createdAt: Date;
  isCompleted?: boolean;
  toggleCompleted?: (id: string) => void;
}

const TutorialItem: React.FC<TutorialItemProps> = memo(({ id, title, content, imageUrl, isCompleted, toggleCompleted }) => {
  const [isOpen, setIsOpen] = useState(false);
  const headerId = `header-${id}`;
  const contentId = `content-${id}`;

  return (
    <Fragment>
      <button 
        id={headerId}
        className="tutorial-header" 
        onClick={() => setIsOpen(prev => !prev)}
        onKeyPress={(event) => {
          if (event.key === "Enter" || event.key === " ") {
            setIsOpen(prev => !prev);
          }
        }}
        aria-expanded={isOpen}
        aria-controls={contentId}
        role="tab"
      >
        {title} <span>{isOpen ? '-' : '+'}</span> {isCompleted && <span>(Completado)</span>}
      </button>
      <div 
        id={contentId}
        className={`tutorial-content ${isOpen ? 'open' : ''}`}
        aria-labelledby={headerId}
        role="tabpanel"
        style={{ transition: 'max-height 0.5s ease-in-out' }}
      >
        {imageUrl && <img src={imageUrl} alt={title} className="tutorial-image" loading="lazy" />}
        <p>{content}</p>
      </div>
      <button onClick={() => toggleCompleted && toggleCompleted(id)}>
        {isCompleted ? 'Desmarcar como completado' : 'Marcar como completado'}
      </button>
    </Fragment>
  );
});

const Tutorial: React.FC = () => {
  const [data, setData] = useState(TutorialData());
  const [searchTerm, setSearchTerm] = useState('');
  const [filterDate, setFilterDate] = useState<Date | null>(null);
  const [completedSteps, setCompletedSteps] = useState<string[]>([]);
  const { isAuthenticated } = useContext(AuthContext);  // <-- Usamos el contexto de autenticación

  // Mostrar un mensaje si el usuario no está autenticado
  if (!isAuthenticated) {
    return <div>Por favor, inicia sesión para acceder al tutorial.</div>;
  }

  useEffect(() => {
    let filteredData = TutorialData();

    if (searchTerm) {
      filteredData = filteredData.filter(item => item.title.toLowerCase().includes(searchTerm.toLowerCase()));
    }

    if (filterDate) {
      filteredData = filteredData.filter(item => item.createdAt <= filterDate);
    }

    setData(filteredData);
  }, [searchTerm, filterDate]);

  const toggleCompleted = (id: string) => {
    setCompletedSteps(prev => {
      if (prev.includes(id)) {
        return prev.filter(stepId => stepId !== id);
      } else {
        return [...prev, id];
      }
    });
  };

  return (
    <div className="tutorial-section" role="tablist">
      <h2>Tutorial</h2>

      {/* Añadimos un aria-label para mejorar la accesibilidad */}
      <input
        type="text"
        placeholder="Buscar por título..."
        onChange={e => setSearchTerm(e.target.value)}
        value={searchTerm}
        aria-label="Buscar tutorial por título"
      />

      <input
        type="date"
        onChange={e => setFilterDate(new Date(e.target.value))}
        aria-label="Filtrar tutorial por fecha"
      />

      {data.map((tutorialStep) => (
        <TutorialItem 
          key={tutorialStep.id} 
          {...tutorialStep} 
          isCompleted={completedSteps.includes(tutorialStep.id)}
          toggleCompleted={toggleCompleted}
        />
      ))}
    </div>
  );
}

export default Tutorial;
