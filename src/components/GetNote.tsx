import { Note } from "../interfaces/note";
import { invoke } from "@tauri-apps/api";
import { useState } from "react";
import RenderNote from "./RenderNote";

function GetNote() {
  const [noteId, setNoteId] = useState("");
  const [note, getNote] = useState<Note>();

  async function get() {
    getNote(await invoke<Note>("get_note", { noteId }));
  }

  return (
    <div>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          get();
        }}
      >
        <input
          onChange={(e) => setNoteId(e.currentTarget.value)}
          placeholder="ノートのURL"
        />
        <button type="submit">Get</button>
      </form>
      {note && <RenderNote note={note} />}
    </div>
  );
}

export default GetNote;
