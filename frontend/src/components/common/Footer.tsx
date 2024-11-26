import { Box, Typography, Grid } from "@mui/material";

const Footer = () => {
  return (
    <>
      <Box
        sx={{
          bottom: 0,
          left: 0,
          position: "fixed",
          justifyContent: "space-between",
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
          justifyContent="center"
          alignItems="center"
          paddingLeft={5}
          paddingRight={5}
        >
          <Typography color="primary">© 2024 Aingaran Elango</Typography>
        </Grid>
      </Box>
    </>
  );
};

export default Footer;
