import originalAxios from "axios";

const baseURL = "https://api.clarity.aingaran.dev";

const axios = originalAxios.create({
  baseURL: baseURL,
});

export default axios;
