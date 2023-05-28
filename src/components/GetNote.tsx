import { Note } from "../interfaces/note";
import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import RenderNote from "./RenderNote";

function GetNote() {
  let noteId: string;
  const [note, getNote] = useState<Note>();

  return (
    <div>
      <form
        onSubmit={async (e) => {
          e.preventDefault();
          getNote(await invoke<Note>("get_note", { noteId }));
        }}
      >
        <input
          onChange={(e) => noteId = e.currentTarget.value}
          placeholder="ノートのURL"
        />
        <button type="submit">Get</button>
      </form>
      {note && <RenderNote note={note} />}
    </div>
  );
}

export default GetNote;
