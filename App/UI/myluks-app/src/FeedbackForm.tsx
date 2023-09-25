import './FeedbackForm.css';
import React, { useState } from 'react';

// Definición de la interfaz para el estado del formulario
interface FormState {
  feedback: string;
  email: string;
  message: string;
  isSubmitting: boolean;
}

const FeedbackForm: React.FC = () => {
  const [formState, setFormState] = useState<FormState>({
    feedback: '',
    email: '',
    message: '',
    isSubmitting: false,
  });

  // Función para manejar el envío del formulario
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setFormState({ ...formState, isSubmitting: true });

    // Validación del feedback
    if (!formState.feedback.trim()) {
      setFormState({ ...formState, message: 'Por favor, ingresa tus comentarios antes de enviar.', isSubmitting: false });
      return;
    }

    // Validación del correo electrónico
    if (formState.email && !formState.email.match(/\S+@\S+\.\S+/)) {
      setFormState({ ...formState, message: 'Por favor, ingresa un correo electrónico válido.', isSubmitting: false });
      return;
    }

    // Aquí puedes manejar el envío real del feedback, por ejemplo, enviándolo a una API.
    console.log({ feedback: formState.feedback, email: formState.email });

    // Reiniciar el formulario
    setFormState({
      feedback: '',
      email: '',
      message: '¡Gracias por tus comentarios!',
      isSubmitting: false
    });
  }

  return (
    <div className="feedback-section">
      <h2>Envíanos tus comentarios</h2>
      {formState.message && <p>{formState.message}</p>}
      <form onSubmit={handleSubmit}>
        <div className="feedback-input">
          <label>
            Comentarios:
            <textarea 
              value={formState.feedback} 
              onChange={(e) => setFormState({ ...formState, feedback: e.target.value })} 
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
              value={formState.email} 
              onChange={(e) => setFormState({ ...formState, email: e.target.value })}
              placeholder="Tu correo electrónico"
            />
          </label>
        </div>
        <button type="submit" disabled={formState.isSubmitting}>
          {formState.isSubmitting ? 'Enviando...' : 'Enviar'}
        </button>
      </form>
    </div>
  );
}

export default FeedbackForm;