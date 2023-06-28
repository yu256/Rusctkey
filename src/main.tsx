import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Login from "./Login";
import "./styles.css";
import { invoke } from "@tauri-apps/api";

ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
).render((await invoke("check_is_logged_in")) ? <App /> : <Login />);
