import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Login from "./Login";
import "./styles.css";
import { invoke } from "@tauri-apps/api";

const token = localStorage.getItem("token");
const isLoggedIn = token != null;

if (isLoggedIn) { // TODO リロードする度に実行してしまうのでなんとかする
  const instance = localStorage.getItem("instance");
  invoke("set_token", { token });
  invoke("set_instance", { instance });
}

ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
).render(
  <React.StrictMode>
    {isLoggedIn ? <App /> : <Login />}
  </React.StrictMode>
);
