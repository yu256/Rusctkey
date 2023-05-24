import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import GetNote from "./components/GetNote";
import PostNote from "./components/PostNote";
import "./App.scss";

function App() {

  useEffect(() => {
    const token = localStorage.getItem('token');
    const instance = localStorage.getItem('instance');
    invoke("set_token", { token });
    invoke("set_instance", { instance });
  }, []);

  function logout() {
    localStorage.removeItem("token");
    localStorage.removeItem("instance");
    location.reload();
  }

  return (
    <div className="container">
      <h1>Misskey Client</h1>

      <button onClick={logout}>
        ログアウト
      </button>
      <GetNote />
      <PostNote />
    </div>
  );
}

export default App;