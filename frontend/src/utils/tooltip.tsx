import { useTheme } from "@mui/material/styles";

export const tooltipComponentsProps = () => {
  const theme = useTheme();
  return {
    tooltip: {
      sx: {
        bgcolor:
          theme.palette.mode === "dark" ? "common.white" : "common.black",
        "& .MuiTooltip-arrow": {
          color:
            theme.palette.mode === "dark" ? "common.white" : "common.black",
        },
        color: theme.palette.mode === "dark" ? "common.black" : "common.white",
        opacity: "80%",
        borderRadius: 3,
      },
    },
  };
};
