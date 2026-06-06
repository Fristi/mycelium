import React from "react";
import ReactDOM from "react-dom/client";
import { Auth0Provider } from "@auth0/auth0-react";
import App from "./App.tsx";
import "./styles/global.css";
import { createHashRouter, createRoutesFromElements, Route, RouterProvider } from "react-router-dom";
import { AuthContext } from "./AuthContext.tsx";
import { PlantView } from "./pages/PlantView.tsx";
import { PlantList } from "./pages/PlantList.tsx";
import { PlantEdit } from "./pages/PlantEdit.tsx";
import { PlantAdd, PlantProvisioning } from "./pages/PlantAdd.tsx";

const callbackUri = "http://localhost:1420";
const cacheLocation = "localstorage";

const router = createHashRouter(
  createRoutesFromElements(
    <Route
      path="/"
      element={<App />}
      // errorElement={<ErrorPage />}
    >
      <Route>
        <Route index element={<PlantList />} />
        <Route path="plant-add" element={<PlantAdd />} />
        <Route path="plant-add/:deviceId" element={<PlantProvisioning />} />
        <Route path="plants/:plantId/edit" element={<PlantEdit />} />
        <Route path="plants/:plantId" element={<PlantView />} />
      </Route>
    </Route>,
  ),
);

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Auth0Provider
      domain="mycelium-greens.eu.auth0.com"
      clientId="TTqNjNFpS7J158xPzznXSAMK302F6Amc"
      useRefreshTokens={true}
      useRefreshTokensFallback={false}
      cacheLocation={cacheLocation}
      authorizationParams={{
        redirect_uri: callbackUri,
        audience: "https://mycelium-greens.eu.auth0.com/api/v2/",
        scope: "read:current_user update:current_user_metadata",
      }}
    >
      <AuthContext>
        <RouterProvider router={router} />
      </AuthContext>
    </Auth0Provider>
  </React.StrictMode>,
);
