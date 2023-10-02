type BaseTutorialStep = {
  id: string;
  title: string;
  content: string;
  createdAt: Date;
  lastUpdated: Date;
  completed?: boolean;
};

interface ImageTutorialStep extends BaseTutorialStep {
  imageUrl: string;
  action: string;
}

interface LinkTutorialStep extends BaseTutorialStep {
  relatedLinks: string[];
}

type TutorialStep = BaseTutorialStep | ImageTutorialStep | LinkTutorialStep;

const ERROR_MESSAGES = {
  INVALID_DATA: (id: string) => `Invalid data for step ID: ${id}`,
  INVALID_IMAGE_URL: (id: string) => `Invalid imageUrl for step ID: ${id}`,
  INVALID_RELATED_LINK: (id: string) => `Invalid relatedLink for step ID: ${id}`,
  INVALID_DATE_RANGE: (id: string) => `Invalid date range for step ID: ${id}. 'lastUpdated' cannot be before 'createdAt'.`,
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

const Validator = {
  isValidData(step: TutorialStep): boolean {
    return !(!step.id || !step.title || !step.content || !step.createdAt || !step.lastUpdated);
  },
  isValidImageURL(url: string): boolean {
    return isValidURL(url);
  },
  areValidRelatedLinks(links: string[]): boolean {
    return links.every(link => isValidURL(link));
  },
  isValidDateRange(step: TutorialStep): boolean {
    return step.createdAt <= step.lastUpdated;
  },
};

const isImageTutorialStep = (step: TutorialStep): step is ImageTutorialStep => {
  return 'imageUrl' in step;
};

const isLinkTutorialStep = (step: TutorialStep): step is LinkTutorialStep => {
  return 'relatedLinks' in step;
};

const logError = (message: string) => {
  // TODO: Consider integrating an external logger if necessary.
  throw new Error(message);
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
    if (!Validator.isValidData(step)) {
      logError(ERROR_MESSAGES.INVALID_DATA(step.id));
    }
    if (!Validator.isValidDateRange(step)) {
      logError(ERROR_MESSAGES.INVALID_DATE_RANGE(step.id));
    }
    if (isImageTutorialStep(step) && !Validator.isValidImageURL(step.imageUrl)) {
      logError(ERROR_MESSAGES.INVALID_IMAGE_URL(step.id));
    }
    if (isLinkTutorialStep(step) && !Validator.areValidRelatedLinks(step.relatedLinks)) {
      logError(ERROR_MESSAGES.INVALID_RELATED_LINK(step.id));
    }
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

const markStepAsCompleted = (data: TutorialStep[], stepId: string): TutorialStep[] => {
  return data.map(step => {
    if (step.id === stepId) {
      return {
        ...step,
        completed: true
      };
    }
    return step;
  });
};

const filterBySearchTerm = (data: TutorialStep[], searchTerm: string): TutorialStep[] => {
  const lowerCaseSearchTerm = searchTerm.toLowerCase();
  return data.filter(step => 
    step.title.toLowerCase().includes(lowerCaseSearchTerm) || 
    step.content.toLowerCase().includes(lowerCaseSearchTerm)
  );
};

const sortDataByLastUpdated = (data: TutorialStep[]): TutorialStep[] => {
  return [...data].sort((a, b) => b.lastUpdated.getTime() - a.lastUpdated.getTime());
};

// Caché para los datos
let cachedData: TutorialStep[] | null = null;

const TutorialData = (): TutorialStep[] => {
  if (cachedData) return cachedData;

  const data: TutorialStep[] = [
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
  ];

  const sortedData = [...data].sort((a, b) => a.createdAt.getTime() - b.createdAt.getTime());

  validateTutorialData(sortedData);

  cachedData = sortedData; // Cache the data
  return sortedData;
};

export default TutorialData;
export { markStepAsCompleted, filterBySearchTerm, sortDataByLastUpdated };
