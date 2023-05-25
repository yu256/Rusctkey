import { invoke } from "@tauri-apps/api";
import { useState } from "react";

function PostNote() {
  const [text, setNote] = useState("");

  async function post() {
    const form = document.getElementById("form") as HTMLInputElement;
    (await invoke("post", { text }))
      ? (form.value = "")
      : console.log("投稿失敗");
  }

  function postFile() {
    invoke("upload_file");
  }

  return (
    <div className="container">
      <form
        onSubmit={(e) => {
          e.preventDefault();
          post();
        }}
      >
        <input
          id="form"
          onChange={(e) => setNote(e.currentTarget.value)}
          placeholder="ノート内容"
        />
        <button type="submit">投稿</button>
      </form>
      <button onClick={postFile}>画像をアップロード</button>
    </div>
  );
}

export default PostNote;
