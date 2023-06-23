import RenderNote from "./RenderNote";
import { useEffect, useState } from "react";
import { Note } from "../interfaces/note";
import { invoke } from "@tauri-apps/api";

const untilDate = localStorage.getItem("untilDate");

function Timeline() {
  const [notes, setNotes] = useState<Note[]>([]);

  useEffect(() => {
    const fetchNotes = async () => {
      const initialNotes = await invoke<Note[]>("fetch_notes", {
        untilDate,
      });
      setNotes(initialNotes);
    };

    fetchNotes();
  }, []);

  async function loadMoreNotesUp() {
    if (!notes.length) return;

    const newNotes = await invoke<Note[]>("fetch_notes", {
      sinceId: notes[0].id,
    });
    setNotes([...newNotes, ...notes]);
  }

  async function loadMoreNotesDown() {
    if (!notes.length) return;

    const newNotes = await invoke<Note[]>("fetch_notes", {
      untilId: notes[notes.length - 1].id,
    });
    setNotes([...notes, ...newNotes]);
  }

  useEffect(() => {
    function handleScroll() {
      const isAtBottom =
        window.innerHeight + window.scrollY >=
        document.body.offsetHeight;
      if (isAtBottom) {
        loadMoreNotesDown();
      }
    }

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
      {untilDate && (
        <div>
          <button
            onClick={closeTimeMachine}
            className="fixed z-10 left-1 top-1"
          >
            ✖
          </button>
          <button onClick={loadMoreNotesUp}>もっと見る</button>
        </div>
      )}
      <div className="list-none">
        {notes.map((note) => (
          <li key={note.id}>
            <RenderNote note={note} />
          </li>
        ))}
        <button onClick={loadMoreNotesDown}>もっと見る</button>
      </div>
    </div>
  );
}

export default Timeline;
