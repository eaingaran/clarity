import { Box } from "@mui/material";
import Footer from "components/common/Footer";
import Header from "components/common/Header";
import Routes from "Routes";
import { useEffect } from "react";
import { useLocation } from "react-router-dom";
import initializeAnalytics from "utils/analytics";
import ReactGA from "react-ga4";

const App = () => {
  const location = useLocation();

  useEffect(() => {
    initializeAnalytics();
  }, []);

  useEffect(() => {
    ReactGA.send({ hitType: "pageview", page: location.pathname });
  }, [location]);

  return (
    <>
      <Box
        sx={{
          bgcolor: "background.default",
          display: "flex",
          height: "100dvh",
          width: "100dvw",
          flexDirection: "column",
          transition: "all 0.1s ease-in-out",
        }}
      >
        <Header />
        <Box
          component="main"
          sx={{
            bgcolor: "background.default",
            display: "flex",
            flex: 1,
            width: "100dvw",
            position: "relative",
            flexDirection: "column",
            alignItems: "center",
            overflow: "auto",
            transition: "all 0.1s ease-in-out",
          }}
        >
          <Routes />
        </Box>
        <Footer />
      </Box>
    </>
  );
};

export default App;
