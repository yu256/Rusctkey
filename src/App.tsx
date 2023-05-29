import Timeline from "./components/Timeline";
import Menu from "./components/BottomMenu";

function App() {
  // function logout() {
  //   localStorage.removeItem("token");
  //   localStorage.removeItem("instance");
  //   location.reload();
  // }

  return (
    <div>
      {/* <button onClick={logout}>ログアウト</button> */}
      <Timeline />
      <Menu />
    </div>
  );
}

export default App;
