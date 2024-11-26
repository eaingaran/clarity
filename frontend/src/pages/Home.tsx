import { useState } from "react";
import {
  Typography,
  Box,
  TextField,
  FormGroup,
  FormControlLabel,
  Switch,
  Autocomplete,
  Button,
  Grid,
  Divider,
  Container,
  ToggleButton,
  ToggleButtonGroup,
  ImageList,
  ImageListItem,
} from "@mui/material";
import axios from "utils/axios";
import ls from "localstorage-ttl";

interface ScraperResponseData {
  url: string;
  include_images: boolean;
  status: string;
  images: string[];
  content: string;
}
interface SummaryResponseData {
  url: string;
  model: string;
  status: string;
  title: string;
  short_summary: string;
  long_summary: string;
  duration: number;
}

const Home = () => {
  const [url, setUrl] = useState("");
  const [selectedModel, setSelectedModel] = useState({
    label: "Llama 3.2",
    id: "llama3.2",
  });
  const [includeImages, setIncludeImages] = useState(false);
  const [scraperResult, setScraperResult] =
    useState<ScraperResponseData | null>(null);
  const [summaryResult, setSummaryResult] =
    useState<SummaryResponseData | null>(null);
  const [summarizing, setSummarizing] = useState(false);
  const [viewMode, setViewMode] = useState<"short" | "long" | "images">(
    "short",
  );
  const [useCache, setUseCache] = useState(true);

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    await fetchData();
  };

  const handleUrlChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setUrl(event.target.value);
  };

  const handleModelChange = (
    _: React.SyntheticEvent<Element, Event>,
    newValue: any,
  ) => {
    setSelectedModel(newValue);
  };

  const handleIncludeImagesChange = (
    event: React.ChangeEvent<HTMLInputElement>,
  ) => {
    setIncludeImages(event.target.checked);
  };

  const handleUseCacheChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    setUseCache(event.target.checked);
  };

  const isValidUrl = (urlString: string) => {
    try {
      new URL(urlString);
      return true;
    } catch (error) {
      return false;
    }
  };

  const fetchData = async () => {
    if (!url || !isValidUrl(url)) {
      return null;
    }
    try {
      setSummarizing(true);
      const scraperResponse = await axios.post("/api/scrape", {
        url,
        include_images: includeImages,
      });
      console.log(scraperResponse.data);
      const scraperResponseData: ScraperResponseData = scraperResponse.data;
      setScraperResult(scraperResponseData);

      if (!scraperResponseData || !scraperResponseData.content) {
        console.log("Issue with the scraper response:", scraperResponseData);
        setSummarizing(false);
        return null;
      }

      const cached_value = ls.get<SummaryResponseData>(url + selectedModel.id);
      if (cached_value && useCache) {
        console.log("using cached value");
        console.log(cached_value);
        setSummaryResult(cached_value);
      } else {
        const summaryResponse = await axios.post(
          "https://summary.aingaran.dev/api/summarise",
          {
            url,
            model: selectedModel.id,
            content: scraperResponseData.content,
          },
        );
        console.log(summaryResponse.data);
        const summaryResponseData: SummaryResponseData = summaryResponse.data;
        setSummaryResult(summaryResponseData);
        ls.set<SummaryResponseData>(
          url + selectedModel.id,
          summaryResponseData,
          3600000,
        );
      }
    } catch (error) {
      console.error("Error fetching data:", error);
    } finally {
      setSummarizing(false);
    }
  };

  return (
    <>
      <Container maxWidth="md" sx={{ mt: 10 }}>
        <Box
          component="form"
          onSubmit={handleSubmit}
          sx={{
            display: "flex",
            flexDirection: "column",
            flex: 1,
            alignItems: "center",
            justifyContent: "center",
            gap: 2,
            padding: 2,
          }}
        >
          <Grid container spacing={2} alignItems="center">
            <Grid item xs={12} md={9}>
              <TextField
                id="url"
                label="URL"
                value={url}
                fullWidth
                onChange={handleUrlChange}
              />
            </Grid>
            <Grid item xs={12} md={3}>
              <Button
                type="submit"
                variant="contained"
                fullWidth
                disabled={!url || !isValidUrl(url) || summarizing}
              >
                Submit
              </Button>
            </Grid>
          </Grid>

          <Grid container spacing={2} alignItems="center">
            <Grid item xs={12} md={6}>
              <Autocomplete
                disablePortal
                isOptionEqualToValue={(option, value) => option.id === value.id}
                options={[
                  { label: "Llama 3.2", id: "llama3.2" },
                  { label: "Mistral", id: "mistral" },
                ]}
                defaultValue={selectedModel}
                value={selectedModel}
                onChange={handleModelChange}
                id="model"
                sx={{ width: "auto" }}
                renderInput={(params) => (
                  <TextField {...params} label="Model" />
                )}
              />
            </Grid>
            <Grid item xs={12} md={3}>
              <FormGroup>
                <FormControlLabel
                  control={
                    <Switch
                      id="showImages"
                      checked={includeImages}
                      onChange={handleIncludeImagesChange}
                    />
                  }
                  label="Show Images"
                />
              </FormGroup>
            </Grid>
            <Grid item xs={12} md={3}>
              <FormGroup>
                <FormControlLabel
                  control={
                    <Switch
                      id="useCache"
                      checked={useCache}
                      onChange={handleUseCacheChange}
                    />
                  }
                  label="Use cache"
                />
              </FormGroup>
            </Grid>
          </Grid>

          <Divider sx={{ width: "80%", margin: "20px 0" }} />

          {summaryResult ? (
            <Box sx={{ textAlign: "center" }}>
              {" "}
              {/* Center align the content */}
              <Typography variant="h6" align="center">
                {summaryResult.title}
              </Typography>
              <Divider sx={{ width: "80%", margin: "20px auto" }} />{" "}
              {/* Center the divider */}
              <ToggleButtonGroup
                exclusive
                value={viewMode}
                onChange={(_, newValue) => newValue && setViewMode(newValue)}
                sx={{ my: 2 }}
              >
                <ToggleButton value="short">Short Summary</ToggleButton>
                <ToggleButton value="long">Long Summary</ToggleButton>
                <ToggleButton value="images">Images</ToggleButton>
              </ToggleButtonGroup>
              {viewMode === "short" && (
                <Typography variant="body2" paragraph>
                  {summaryResult.short_summary}
                </Typography>
              )}
              {viewMode === "long" && (
                <Typography variant="body1" paragraph>
                  {summaryResult.long_summary}
                </Typography>
              )}
              {viewMode === "images" &&
                (scraperResult?.include_images ? (
                  <ImageList cols={3} gap={8}>
                    {scraperResult.images.map((imageSrc) => (
                      <ImageListItem key={imageSrc}>
                        <img src={imageSrc} alt="Article" loading="lazy" />
                      </ImageListItem>
                    ))}
                  </ImageList>
                ) : (
                  <Typography variant="body2">
                    Enable "Include Images" to see images from the article.
                  </Typography>
                ))}
            </Box>
          ) : (
            <Typography>{""}</Typography>
          )}
        </Box>
      </Container>
    </>
  );
};

export default Home;
