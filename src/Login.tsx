import { invoke } from "@tauri-apps/api";

function Login() {
  let token: string;
  let instance: string;

  return (
    <>
      <form
        onSubmit={async () => {
          if (await invoke("set_credentials", { instance, token }))
            location.reload();
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
    </>
  );
}

export default Login;
