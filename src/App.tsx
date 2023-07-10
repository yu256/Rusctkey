import Timeline from "./components/Timeline";
import Menu from "./components/AppMenu";
import { readTextFile, BaseDirectory } from "@tauri-apps/api/fs";

const url = await readTextFile("instance", {
  dir: BaseDirectory.AppLocalData,
});
const token = await readTextFile("i", {
  dir: BaseDirectory.AppLocalData,
});

export const ws = new WebSocket(`wss://${url}/streaming?i=${token}`);

ws.onopen = () => {
  const message = {
    type: "connect",
    body: {
      channel: "homeTimeline",
      id: "1",
    },
  };

  ws.send(JSON.stringify(message));
};

function App() {
  return (
    <>
      <Timeline />
      <Menu />
    </>
  );
}

export default App;
