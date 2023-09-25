import './Tutorial.css';
import React, { useState, memo, Fragment } from 'react';
import TutorialData from './TutorialData';

interface TutorialItemProps {
  id: string;
  title: string;
  content: string;
  imageUrl?: string;
}

const TutorialItem: React.FC<TutorialItemProps> = memo(({ id, title, content, imageUrl }) => {
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
        {title} <span>{isOpen ? '-' : '+'}</span>
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
    </Fragment>
  );
});

const Tutorial: React.FC = () => {
  const data = TutorialData();

  return (
    <div className="tutorial-section" role="tablist">
      <h2>Tutorial</h2>
      {data.map((tutorialStep) => (
        <TutorialItem key={tutorialStep.id} {...tutorialStep} />
      ))}
    </div>
  );
}

export default Tutorial;