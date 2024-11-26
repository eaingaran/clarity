import ReactGA from "react-ga4";
import { v4 as uuidv4 } from "uuid";

const GA_MEASUREMENT_ID = "G-4D04KFX7QJ";

export const trackEvent = (category: string, action: string, label: string) => {
  ReactGA.event({ category, action, label });
};

const initializeAnalytics = () => {
  // Generate a unique client ID or retrieve it from localStorage
  const clientId = localStorage.getItem("clientId") || uuidv4();
  localStorage.setItem("clientId", clientId);

  ReactGA.initialize(GA_MEASUREMENT_ID, {
    gaOptions: {
      clientId: clientId,
    },
  });
};

export default initializeAnalytics;
