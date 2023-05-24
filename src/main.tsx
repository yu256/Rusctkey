import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Login from "./Login";
import "./styles.scss";

const isLoggedIn = localStorage.getItem("token") != null;

ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
).render(
  <React.StrictMode>
    {isLoggedIn ? <App /> : <Login />}
  </React.StrictMode>
);
