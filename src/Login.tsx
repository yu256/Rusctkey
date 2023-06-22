import { invoke } from "@tauri-apps/api";

function Login() {
  let token: string;
  let instance: string;

  async function set() {
    (await invoke("set_credentials", { instance, token }))
      ? location.reload()
      : console.log("URLかトークンが正しくありません。");
  }

  return (
    <div>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          instance && token
            ? set()
            : console.log("値がセットされていません。"); // TODO ポップアップさせる
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
