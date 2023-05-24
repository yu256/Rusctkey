import { useState } from "react";

function Login() {
    const [token, setToken] = useState("");
    const [instance, setInstance] = useState("");

    function set() {
        localStorage.setItem("token", token);
        localStorage.setItem("instance", instance);
        location.reload();
    }

    return (
      <div className="container">
        <h1>Misskey Client</h1>

        <form
            onSubmit={(e) => {
                e.preventDefault();
                instance && token
                    ? set()
                    : console.log("値がセットされていません。"); // TODO ポップアップさせる
            }}
        >
        <input
            onChange={(e) => setToken(e.currentTarget.value)}
            placeholder="トークン"
        />
        <input
            onChange={(e) => setInstance(e.currentTarget.value)}
            placeholder="インスタンスのURL"
        />
        <button type="submit">保存</button>
        </form>
      </div>
    );
}

export default Login