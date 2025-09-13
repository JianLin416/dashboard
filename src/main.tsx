import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { BrowserRouter, Route, Routes } from "react-router";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <BrowserRouter>
        <React.StrictMode>
            <Routes>
                <Route path="/" element={<App />} />
            </Routes>
        </React.StrictMode>
    </BrowserRouter>
);
