import { invoke } from "@tauri-apps/api";

function Login() {
  let token: string;
  let instance: string;

  async function set() {
    if (await invoke("set_credentials", { instance, token }))
      location.reload();
  }

  return (
    <div>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          if (instance && token) set();
        }}
      >
        <input
          onChange={(e) => (token = e.currentTarget.value)}
          placeholder="トークン"
        />
        <input
          onChange={(e) => (instance = e.currentTarget.value)}
          placeholder="インスタンスのURL"
        />
        <button type="submit">保存</button>
      </form>
    </div>
  );
}

export default Login;
