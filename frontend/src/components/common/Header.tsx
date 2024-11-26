import {
  DarkModeOutlined as DarkModeOutlinedIcon,
  LightModeOutlined as LightModeOutlinedIcon,
} from "@mui/icons-material";
import { Box, Typography, Grid, Tooltip } from "@mui/material";
import { tooltipComponentsProps } from "utils/tooltip";
import { useTheme } from "@mui/material/styles";
import { useContext } from "react";
import { ThemeContext } from "theme";

const Header = () => {
  const tooltipProps = tooltipComponentsProps();
  const theme = useTheme();
  const { switchColorMode } = useContext(ThemeContext);

  return (
    <>
      <Box
        sx={{
          top: 0,
          left: 0,
          position: "fixed",
          bgcolor: "background.paper",
          paddingTop: 1.5,
          paddingBottom: 3,
          width: "100vw",
          height: "25px",
          zIndex: 1200,
        }}
      >
        <Grid
          container
          alignItems="center"
          justifyContent="space-between"
          paddingLeft={2.5}
          paddingRight={2.5}
        >
          <Typography variant="h4" color="primary" sx={{ fontWeight: "bold" }}>
            Clarity
          </Typography>
          <Tooltip
            describeChild
            title={
              theme.palette.mode === "dark"
                ? "Switch to Light Mode"
                : "Switch to Dark Mode"
            }
            arrow
            placement="top"
            componentsProps={tooltipProps}
          >
            {theme.palette.mode === "dark" ? (
              <LightModeOutlinedIcon
                sx={{ color: "text.primary" }}
                onClick={switchColorMode}
              />
            ) : (
              <DarkModeOutlinedIcon
                sx={{ color: "text.primary" }}
                onClick={switchColorMode}
              />
            )}
          </Tooltip>
        </Grid>
      </Box>
    </>
  );
};

export default Header;
