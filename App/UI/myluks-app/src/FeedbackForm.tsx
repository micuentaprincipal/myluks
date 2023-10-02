import './FeedbackForm.css';
import React, { useState, useContext } from 'react';
import { AuthContext } from './App';

const MAX_FEEDBACK_LENGTH = 1500;  // Establecer un límite de 1500 caracteres para el feedback

interface FormState {
  subject: string;
  feedback: string;
  email: string;
  isSubmitting: boolean;
  errorMessage: string | null;
  successMessage: string | null;
}

const FeedbackForm: React.FC = () => {
  const { isAuthenticated, email: userEmail } = useContext(AuthContext);

  const [formState, setFormState] = useState<FormState>({
    subject: '',
    feedback: '',
    email: isAuthenticated ? userEmail : '',
    isSubmitting: false,
    errorMessage: null,
    successMessage: null,
  });

  const isValidEmail = (email: string) => /\S+@\S+\.\S+/.test(email);

  const handleValidation = () => {
    if (!formState.feedback.trim() || formState.feedback.length > MAX_FEEDBACK_LENGTH) {
      setFormState(prev => ({ ...prev, errorMessage: 'Por favor, ingresa tus comentarios correctamente antes de enviar.' }));
      return false;
    }

    if (formState.email && !isValidEmail(formState.email)) {
      setFormState(prev => ({ ...prev, errorMessage: 'Por favor, ingresa un correo electrónico válido.' }));
      return false;
    }

    return true;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setFormState(prev => ({ ...prev, isSubmitting: true, errorMessage: null, successMessage: null }));

    if (!handleValidation()) {
      setFormState(prev => ({ ...prev, isSubmitting: false }));
      return;
    }

    console.log({ feedback: formState.feedback, email: formState.email });

    setFormState({
      subject: '',
      feedback: '',
      email: isAuthenticated ? userEmail : '',
      isSubmitting: false,
      errorMessage: null,
      successMessage: '¡Gracias por tus comentarios!',
    });
  };

  return (
    <div className="feedback-section">
      {/* ... */}
      <form onSubmit={handleSubmit}>
        {/* Nuevo campo de asunto */}
        <div className="feedback-input">
          <label htmlFor="subject-input">
            Asunto:
          </label>
          <input 
            id="subject-input"
            type="text" 
            value={formState.subject} 
            onChange={(e) => setFormState({ ...formState, subject: e.target.value })}
            placeholder="Tema de tus comentarios"
            maxLength={100}
          />
        </div>
        <div className="feedback-input">
          <label htmlFor="feedback-textarea">
            Comentarios:
          </label>
          <textarea 
            id="feedback-textarea"
            value={formState.feedback} 
            onChange={(e) => setFormState({ ...formState, feedback: e.target.value })} 
            placeholder="Escribe tus comentarios aquí..."
            maxLength={MAX_FEEDBACK_LENGTH}
            required
          />
          <small>{formState.feedback.length}/{MAX_FEEDBACK_LENGTH}</small>
        </div>
        <div className="feedback-input">
          <label htmlFor="email-input">
            Correo Electrónico (opcional):
          </label>
          <input 
            id="email-input"
            type="email" 
            value={formState.email} 
            onChange={(e) => setFormState({ ...formState, email: e.target.value })}
            placeholder="Tu correo electrónico"
            readOnly={isAuthenticated}
          />
          {!isAuthenticated && <small className="email-feedback">
            {formState.email && !isValidEmail(formState.email) ? "Email inválido" : ""}
          </small>}
        </div>
        <button type="submit" disabled={formState.isSubmitting}>
          {formState.isSubmitting ? 'Enviando...' : 'Enviar'}
        </button>
        {formState.isSubmitting && <div className="loader"></div>}
      </form>
    </div>
  );
}

export default FeedbackForm;
