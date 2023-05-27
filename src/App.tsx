import GetNote from "./components/GetNote";
import PostNote from "./components/PostNote";

function App() {
  function logout() {
    localStorage.removeItem("token");
    localStorage.removeItem("instance");
    location.reload();
  }

  return (
    <div>
      <button onClick={logout}>ログアウト</button>
      <GetNote />
      <PostNote />
    </div>
  );
}

export default App;
