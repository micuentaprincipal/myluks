type TutorialStep = {
  id: string;
  title: string;
  content: string;
  imageUrl?: string; // URL de una imagen relacionada con el paso.
  relatedLinks?: string[]; // Enlaces a recursos adicionales o documentación.
  action?: string; // Una acción recomendada, como "Crear una billetera ahora".
  createdAt: Date; // Fecha de creación del tutorial.
  lastUpdated: Date; // Fecha de la última actualización.
};

const TutorialData = (): TutorialStep[] => {
  const data = [
    {
      id: 'step1',
      title: "Paso 1: Crear una Billetera",
      content: "Instrucciones sobre cómo crear una billetera en MyLuks.",
      imageUrl: "https://example.com/wallet-image.jpg",
      action: "Crear billetera",
      createdAt: new Date("2023-01-01"),
      lastUpdated: new Date("2023-01-10"),
    },
    {
      id: 'step2',
      title: "Paso 2: Seguridad",
      content: "Importancia de mantener tus claves privadas seguras.",
      relatedLinks: [
        "https://security-resources.com/tips",
        "https://crypto-security.com/guide"
      ],
      createdAt: new Date("2023-02-01"),
      lastUpdated: new Date("2023-02-15"),
    },
    // ... otros pasos ...
  ];

  const sortedData = data.sort((a, b) => a.createdAt.getTime() - b.createdAt.getTime());
  validateTutorialData(sortedData);
  return sortedData;
}

// Función principal de validación
const validateTutorialData = (data: TutorialStep[]) => {
  data.forEach(step => {
    validateRequiredFields(step);
    validateImageURL(step);
    validateRelatedLinks(step);
    validateDates(step);
  });
};

// Validación de campos requeridos
const validateRequiredFields = (step: TutorialStep) => {
  if (!step.id || !step.title || !step.content || !step.createdAt || !step.lastUpdated) {
    console.error(`Invalid data for step ID: ${step.id}`);
  }
};

// Validación de URL de imagen
const validateImageURL = (step: TutorialStep) => {
  if (step.imageUrl && !isValidURL(step.imageUrl)) {
    console.error(`Invalid imageUrl for step ID: ${step.id}`);
  }
};

// Validación de enlaces relacionados
const validateRelatedLinks = (step: TutorialStep) => {
  if (step.relatedLinks) {
    step.relatedLinks.forEach(link => {
      if (!isValidURL(link)) {
        console.error(`Invalid relatedLink for step ID: ${step.id}`);
      }
    });
  }
};

// Validación de fechas
const validateDates = (step: TutorialStep) => {
  if (step.createdAt > step.lastUpdated) {
    console.error(`Invalid date range for step ID: ${step.id}. 'lastUpdated' cannot be before 'createdAt'.`);
  }
};

// Función para comprobar si una URL es válida
const isValidURL = (url: string): boolean => {
  try {
    new URL(url);
    return true;
  } catch (_) {
    return false;
  }
}

export default TutorialData;  