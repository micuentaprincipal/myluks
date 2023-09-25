type BaseTutorialStep = {
  id: string;
  title: string;
  content: string;
  createdAt: Date;
  lastUpdated: Date;
};

type ImageTutorialStep = BaseTutorialStep & {
  imageUrl: string;
};

type LinkTutorialStep = BaseTutorialStep & {
  relatedLinks: string[];
};

type TutorialStep = BaseTutorialStep | ImageTutorialStep | LinkTutorialStep;

// Función para determinar si un paso es de tipo ImageTutorialStep
const isImageTutorialStep = (step: TutorialStep): step is ImageTutorialStep => {
  return 'imageUrl' in step;
};

// Función para determinar si un paso es de tipo LinkTutorialStep
const isLinkTutorialStep = (step: TutorialStep): step is LinkTutorialStep => {
  return 'relatedLinks' in step;
};

const logError = (message: string) => {
  // Aquí se puede integrar cualquier logger externo si es necesario
  console.error(message);
};

const transformData = (data: TutorialStep[]): TutorialStep[] => {
  // Esta función puede ser útil si necesitas hacer alguna transformación adicional en el futuro
  return data;
};

const getRecentlyUpdated = (data: TutorialStep[], days: number): TutorialStep[] => {
  const cutoffDate = new Date();
  cutoffDate.setDate(cutoffDate.getDate() - days);

  return data.filter(step => step.lastUpdated >= cutoffDate);
};

// Función principal de validación
const validateTutorialData = (data: TutorialStep[]) => {
  data.forEach(step => {
    validateRequiredFields(step);

    if (isImageTutorialStep(step)) {
      validateImageURL(step);
    } else if (isLinkTutorialStep(step)) {
      validateRelatedLinks(step);
    }

    validateDates(step);
  });
};

// Validación de campos requeridos
const validateRequiredFields = (step: TutorialStep) => {
  if (!step.id || !step.title || !step.content || !step.createdAt || !step.lastUpdated) {
    logError(`Invalid data for step ID: ${step.id}`); // Usamos el logger
  }
};

// Validación de URL de imagen
const validateImageURL = (step: ImageTutorialStep) => {
  if (!isValidURL(step.imageUrl)) {
    logError(`Invalid imageUrl for step ID: ${step.id}`); // Usamos el logger
  }
};

// Validación de enlaces relacionados
const validateRelatedLinks = (step: LinkTutorialStep) => {
  step.relatedLinks.forEach(link => {
    if (!isValidURL(link)) {
      logError(`Invalid relatedLink for step ID: ${step.id}`); // Usamos el logger
    }
  });
};

// Validación de fechas
const validateDates = (step: TutorialStep) => {
  if (step.createdAt > step.lastUpdated) {
    logError(`Invalid date range for step ID: ${step.id}. 'lastUpdated' cannot be before 'createdAt'.`); // Usamos el logger
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

export default TutorialData;  