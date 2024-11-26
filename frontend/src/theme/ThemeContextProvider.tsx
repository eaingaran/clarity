import {
  StyledEngineProvider,
  ThemeProvider,
  createTheme,
} from "@mui/material/styles";
import { ReactNode, createContext, useEffect, useMemo, useState } from "react";
import useMediaQuery from "@mui/material/useMediaQuery";

type ThemeContextType = {
  switchColorMode: () => void;
};

type ThemeProviderProps = {
  children: ReactNode;
};

export const ThemeContext = createContext<ThemeContextType>({
  switchColorMode: () => {},
});

export function ThemeContextProvider({ children }: ThemeProviderProps) {
  const prefersDarkMode = useMediaQuery("(prefers-color-scheme: dark)");
  const last_used_theme = localStorage.getItem("theme");
  const [mode, setMode] = useState<"light" | "dark">(
    last_used_theme === null
      ? prefersDarkMode
        ? "dark"
        : "light"
      : last_used_theme === "dark"
      ? "dark"
      : "light"
  );

  useEffect(() => {
    localStorage.setItem("theme", mode);
  }, [mode]);

  const switchColorMode = () => {
    setMode((prevMode) => (prevMode === "light" ? "dark" : "light"));
  };

  const theme = useMemo(
    () =>
      createTheme({
        palette: {
          mode,
        },
      }),
    [mode]
  );

  return (
    <StyledEngineProvider injectFirst>
      <ThemeContext.Provider value={{ switchColorMode }}>
        <ThemeProvider theme={theme}>{children}</ThemeProvider>
      </ThemeContext.Provider>
    </StyledEngineProvider>
  );
}
