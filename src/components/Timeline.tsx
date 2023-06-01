import RenderNote from "./RenderNote";
import { useEffect, useState } from "react";
import { Note } from "../interfaces/note";
import { invoke } from "@tauri-apps/api";

function Timeline() {
  const [notes, setNotes] = useState<Note[]>([]);

  useEffect(() => {
    const fetchNotes = async () => {
      const initialNotes = await invoke<Note[]>("fetch_notes");
      setNotes(initialNotes);
    };

    fetchNotes();
  }, []);

  const loadMoreNotes = async () => {
    if (notes.length === 0) return;

    const lastNoteId = notes[notes.length - 1].id;
    const newNotes = await invoke<Note[]>("fetch_notes", {
      until_id: lastNoteId,
    });
    setNotes([...notes, ...newNotes]);
  };

  useEffect(() => {
    const handleScroll = () => {
      const isAtBottom =
        window.innerHeight + window.scrollY >=
        document.body.offsetHeight;
      if (isAtBottom) {
        loadMoreNotes();
      }
    };

    window.addEventListener("scroll", handleScroll);
    return () => {
      window.removeEventListener("scroll", handleScroll);
    };
  }, [notes]);

  return (
    <div className="list-none">
      {notes.map((note) => (
        <li key={note.id}>
          <RenderNote note={note} />
        </li>
      ))}
      <button onClick={loadMoreNotes}>もっと見る</button>
    </div>
  );
}

export default Timeline;
