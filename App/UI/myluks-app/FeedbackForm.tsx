import './FeedbackForm.css';

import React, { useState } from 'react';

const FeedbackForm: React.FC = () => {
  const [feedback, setFeedback] = useState('');
  const [email, setEmail] = useState('');
  const [message, setMessage] = useState('');
  const [isSubmitting, setIsSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsSubmitting(true);

    if (!feedback.trim()) {
      setMessage('Por favor, ingresa tus comentarios antes de enviar.');
      setIsSubmitting(false);
      return;
    }

    if (email && !email.match(/\S+@\S+\.\S+/)) {
      setMessage('Por favor, ingresa un correo electrónico válido.');
      setIsSubmitting(false);
      return;
    }

    // Aquí puedes manejar el envío del feedback, por ejemplo, enviándolo a una API.
    console.log({ feedback, email });

    setFeedback('');
    setEmail('');
    setMessage('¡Gracias por tus comentarios!');
    setIsSubmitting(false);
  }

  return (
    <div className="feedback-section">
      <h2>Envíanos tus comentarios</h2>
      {message && <p>{message}</p>}
      <form onSubmit={handleSubmit}>
        <div className="feedback-input">
          <label>
            Comentarios:
            <textarea 
              value={feedback} 
              onChange={(e) => setFeedback(e.target.value)} 
              placeholder="Escribe tus comentarios aquí..."
              required
            />
          </label>
        </div>
        <div className="feedback-input">
          <label>
            Correo Electrónico (opcional):
            <input 
              type="email" 
              value={email} 
              onChange={(e) => setEmail(e.target.value)}
              placeholder="Tu correo electrónico"
            />
          </label>
        </div>
        <button type="submit" disabled={isSubmitting}>
          {isSubmitting ? 'Enviando...' : 'Enviar'}
        </button>
      </form>
    </div>
  );
}

export default FeedbackForm;