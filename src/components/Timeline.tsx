import RenderNote from "./RenderNote";
import { useEffect, useState } from "react";
import { Note } from "../interfaces/note";
import { invoke } from "@tauri-apps/api";

const until_date = localStorage.getItem("untilDate");

function Timeline() {
  const [notes, setNotes] = useState<Note[]>([]);

  useEffect(() => {
    const fetchNotes = async () => {
      const initialNotes = await invoke<Note[]>("fetch_notes", {
        untilDate: until_date != null ? Number(until_date) : null,
      });
      setNotes(initialNotes);
    };

    fetchNotes();
  }, []);

  const loadMoreNotes = async () => {
    if (notes.length === 0) return;

    const lastNoteId = notes[notes.length - 1].id;
    const newNotes = await invoke<Note[]>("fetch_notes", {
      id: lastNoteId,
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

  function closeTimeMachine() {
    localStorage.removeItem("untilDate");
    location.reload();
  }

  return (
    <div>
      {until_date && (
        <div>
          <button onClick={closeTimeMachine} className="fixed z-10">
            ✖
          </button>
        </div>
      )}
      <div className="list-none">
        {notes.map((note) => (
          <li key={note.id}>
            <RenderNote note={note} />
          </li>
        ))}
        <button onClick={loadMoreNotes}>もっと見る</button>
      </div>
    </div>
  );
}

export default Timeline;
